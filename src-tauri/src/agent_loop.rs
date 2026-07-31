// Agent Loop — ReAct loop for Open Longevity.
// Ported from EchoBird's services/agent_loop.rs mechanics, keeping Open
// Longevity's domain (local knowledge library tools + longevity prompt).
// Streams text + tool calls to the frontend via Tauri events.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::agent_tools;
use crate::conversations;
use crate::json_repair;
use crate::llm_stream::{
    self, ContentBlock, LlmClient, LlmConfig, LlmEvent, LlmProvider, Message, MessageContent,
};
use crate::memory;

// ── Constants ──

const MAX_TOOL_LOOPS: usize = 150;
const MAX_CONTEXT_BYTES: usize = 300_000;
const MAX_SSE_RETRIES: u32 = 3;
const FIRST_TOKEN_TIMEOUT_SECS: u64 = 60;
const INTER_TOKEN_TIMEOUT_SECS: u64 = 120;
const MAX_WAIT_WARNINGS: u32 = 2;
const MAX_OUTPUT_BYTES: usize = 8_000;
const LOOP_REPEAT_THRESHOLD: usize = 3;
const RECENT_CALLS_CAPACITY: usize = 8;

// ── Events emitted to frontend ──

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    #[serde(rename = "text_delta")]
    TextDelta {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        text: String,
    },
    #[serde(rename = "thinking")]
    Thinking {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        text: String,
    },
    #[serde(rename = "tool_call_start")]
    ToolCallStart {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        id: String,
        name: String,
    },
    #[serde(rename = "tool_call_args")]
    ToolCallArgs {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        id: String,
        args: String,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        id: String,
        output: String,
        success: bool,
    },
    #[serde(rename = "memory_suggestion")]
    MemorySuggestion {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        suggestion: memory::MemorySuggestion,
    },
    #[serde(rename = "done")]
    Done {
        #[serde(rename = "conversationId")]
        conversation_id: String,
    },
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        message: String,
    },
    #[serde(rename = "state")]
    StateChange {
        #[serde(rename = "conversationId")]
        conversation_id: String,
        state: String,
    },
}

// ── Request from frontend ──

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRequest {
    pub conversation_id: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub message: String,
    pub locale: String,
    pub knowledge_root: String,
    #[serde(default)]
    pub context_paths: Vec<String>,
    #[serde(default)]
    pub history: Vec<HistoryLine>,
    /// Which wire protocol the provider speaks: "openai" or "anthropic".
    #[serde(default)]
    pub provider: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryLine {
    pub role: String,
    pub content: String,
}

// ── Session state ──

pub struct AgentSession {
    pub messages: Vec<Message>,
    pub running: bool,
    pub cancel_token: CancellationToken,
    /// Ring buffer of recent tool-call hashes for loop detection.
    pub recent_calls: std::collections::VecDeque<u64>,
}

impl Default for AgentSession {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentSession {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            running: false,
            cancel_token: CancellationToken::new(),
            recent_calls: std::collections::VecDeque::with_capacity(RECENT_CALLS_CAPACITY),
        }
    }

    pub fn cancel(&mut self) {
        self.cancel_token.cancel();
        self.running = false;
    }

    pub fn prepare_run(&mut self) {
        if self.cancel_token.is_cancelled() {
            self.cancel_token = CancellationToken::new();
        }
        self.running = true;
        self.recent_calls.clear();
    }

    /// Record a tool call and return Some(reason) if it has now repeated
    /// `LOOP_REPEAT_THRESHOLD` times in the recent window.
    pub fn record_call_and_detect_loop(&mut self, hash: u64) -> Option<String> {
        let prior_count = self.recent_calls.iter().filter(|&&h| h == hash).count();
        if self.recent_calls.len() >= RECENT_CALLS_CAPACITY {
            self.recent_calls.pop_front();
        }
        self.recent_calls.push_back(hash);
        if prior_count + 1 >= LOOP_REPEAT_THRESHOLD {
            Some(format!(
                "Loop detected: this exact tool call has now run {} times without progress. \
                 Change approach or ask the user.",
                prior_count + 1
            ))
        } else {
            None
        }
    }
}

pub type SharedSessionMap = Arc<Mutex<HashMap<String, AgentSession>>>;

pub fn create_session_map() -> SharedSessionMap {
    Arc::new(Mutex::new(HashMap::new()))
}

// ── Session persistence ──

fn sessions_dir() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("OpenLongevity")
        .join("sessions")
}

fn session_file() -> std::path::PathBuf {
    sessions_dir().join("session.json")
}

pub fn clear_session_from_disk() {
    let path = session_file();
    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }
}

// ── User profile bootstrap (kept from original) ──

fn build_user_profile_context(knowledge_root: &std::path::Path, locale: &str) -> String {
    let personal_paths = [
        "profile/about-me.md",
        "plans/current-protocol.md",
        "records/lab-results.md",
        "records/diet-log.md",
        "records/training-log.md",
    ];

    let mut sections = Vec::new();
    for relative in &personal_paths {
        let base_path = knowledge_root.join(relative);
        let localized = if locale == "en" {
            let stem = base_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let ext = base_path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let parent = base_path.parent().unwrap_or_else(|| knowledge_root);
            parent.join(format!("{stem}.en.{ext}"))
        } else {
            base_path.clone()
        };
        let path = if localized.is_file() {
            localized
        } else if base_path.is_file() {
            base_path
        } else {
            continue;
        };
        if let Ok(content) = std::fs::read_to_string(&path) {
            if !content.trim().is_empty() {
                let truncated = if content.len() > 6000 {
                    let mut t: String = content.chars().take(5900).collect();
                    t.push_str("\n…[truncated]");
                    t
                } else {
                    content
                };
                sections.push(format!("--- {relative} ---\n{truncated}"));
            }
        }
    }

    if sections.is_empty() {
        return String::new();
    }
    format!(
        "\n\n## USER PROFILE (always available — do not ask the user to repeat this)\n{}",
        sections.join("\n\n")
    )
}

// ── System prompt ──

fn build_system_prompt(locale: &str, user_profile: &str, memory_context: &str) -> String {
    let language_rule = if locale == "en" {
        "Reply in English."
    } else {
        "使用简体中文回答。"
    };
    format!(
        "You are Open Longevity, a local-first scientific longevity assistant with tool-calling ability. \
         You can save notes to the user's knowledge library, search existing notes, read note content, and suggest long-term memory candidates. \
         When the user wants to record, save, or remember something, use the save_note tool — do NOT just \
         tell them to do it manually. Use suggest_memory only for durable user-confirmed goals, preferences, constraints, corrections, profile facts, or health context worth reusing in future conversations. \
         suggest_memory only proposes a memory candidate; the user must confirm before it is saved. \
         When answering questions about the library, use search_library and read_note to ground your answers in actual notes. \
         The user's local notes are your primary memory. Cite the note path in parentheses when a \
         statement depends on it. Clearly separate the user's personal protocol from general information. \
         Never invent a study, measurement, dose, or source. Preserve concise safety boundaries for \
         medication interactions, allergies, pregnancy, and organ impairment when relevant. \
         Do not diagnose or prescribe. {language_rule}{user_profile}{memory_context}"
    )
}

// ── Loop detection ──

fn loop_args_hash(tool_name: &str, args: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    tool_name.hash(&mut h);
    let canon = serde_json::from_str::<Value>(args)
        .ok()
        .and_then(|v| serde_json::to_string(&v).ok())
        .unwrap_or_else(|| args.to_string());
    canon.hash(&mut h);
    h.finish()
}

// ── LLM server-down detection ──

fn is_llm_server_down(err: &str) -> bool {
    let lower = err.to_lowercase();
    lower.contains("connection refused")
        || lower.contains("os error 111")
        || lower.contains("os error 61")
        || lower.contains("os error 10061")
        || lower.contains("no connection could be made")
        || lower.contains("failed to connect")
        || lower.contains("tcp connect error")
}

fn format_server_down_error(err: &str) -> String {
    log::error!("[AgentLoop] LLM server is down: {err}");
    "⚠️ The model server is unreachable (connection refused). Check your API URL and key, then try again."
        .to_string()
}

// ── Tool classification ──

fn is_shared_tool(name: &str) -> bool {
    matches!(name, "search_library" | "read_note")
}

// ── Output truncation ──

fn truncate_output(output: &str) -> String {
    if output.len() > MAX_OUTPUT_BYTES {
        let mut t: String = output.chars().take(MAX_OUTPUT_BYTES - 20).collect();
        t.push_str("\n…[truncated]");
        t
    } else {
        output.to_string()
    }
}

// ── Message builders ──

fn build_assistant_message(
    text: &str,
    thinking: &str,
    signature: &str,
    tool_calls: &[llm_stream::ToolCall],
) -> Message {
    let has_thinking = !thinking.is_empty();
    if tool_calls.is_empty() && !has_thinking {
        return Message {
            role: "assistant".into(),
            content: MessageContent::Text(text.to_string()),
        };
    }
    let mut blocks: Vec<ContentBlock> = Vec::new();
    if has_thinking {
        blocks.push(ContentBlock::Thinking {
            thinking: thinking.to_string(),
            signature: signature.to_string(),
        });
    }
    if !text.is_empty() {
        blocks.push(ContentBlock::Text {
            text: text.to_string(),
        });
    }
    for tc in tool_calls {
        let input: Value =
            serde_json::from_str(&tc.arguments).unwrap_or(Value::Object(Default::default()));
        blocks.push(ContentBlock::ToolUse {
            id: tc.id.clone(),
            name: tc.name.clone(),
            input,
        });
    }
    Message {
        role: "assistant".into(),
        content: MessageContent::Blocks(blocks),
    }
}

/// Strip tool_result blocks whose matching tool_use was truncated away.
fn ensure_tool_results_paired(messages: &mut Vec<Message>) {
    use std::collections::HashSet;
    let mut present_tool_use_ids: HashSet<String> = HashSet::new();
    for msg in messages.iter() {
        if let MessageContent::Blocks(blocks) = &msg.content {
            for block in blocks {
                if let ContentBlock::ToolUse { id, .. } = block {
                    present_tool_use_ids.insert(id.clone());
                }
            }
        }
    }

    let mut dropped = 0usize;
    for msg in messages.iter_mut() {
        if let MessageContent::Blocks(blocks) = &mut msg.content {
            let before = blocks.len();
            blocks.retain(|block| match block {
                ContentBlock::ToolResult { tool_use_id, .. } => {
                    present_tool_use_ids.contains(tool_use_id)
                }
                _ => true,
            });
            dropped += before - blocks.len();
        }
    }
    messages.retain(|msg| match &msg.content {
        MessageContent::Text(s) => !s.is_empty(),
        MessageContent::Blocks(blocks) => !blocks.is_empty(),
    });

    if dropped > 0 {
        log::info!(
            "[AgentLoop] ensure_tool_results_paired: dropped {} orphan tool_result block(s)",
            dropped
        );
    }
}

// ── Event emitters ──

fn emit_event(app: &AppHandle, event: AgentEvent) {
    if let Err(e) = app.emit("agent_event", &event) {
        log::error!("[AgentLoop] emit failed: {e}");
    }
}

fn emit_text(app: &AppHandle, conversation_id: &str, text: String) {
    emit_event(
        app,
        AgentEvent::TextDelta {
            conversation_id: conversation_id.to_string(),
            text,
        },
    );
}

fn emit_thinking(app: &AppHandle, conversation_id: &str, text: String) {
    emit_event(
        app,
        AgentEvent::Thinking {
            conversation_id: conversation_id.to_string(),
            text,
        },
    );
}

fn emit_tool_start(app: &AppHandle, conversation_id: &str, id: String, name: String) {
    emit_event(
        app,
        AgentEvent::ToolCallStart {
            conversation_id: conversation_id.to_string(),
            id,
            name,
        },
    );
}

fn emit_tool_args(app: &AppHandle, conversation_id: &str, id: String, args: String) {
    emit_event(
        app,
        AgentEvent::ToolCallArgs {
            conversation_id: conversation_id.to_string(),
            id,
            args,
        },
    );
}

fn emit_tool_result(
    app: &AppHandle,
    conversation_id: &str,
    id: String,
    output: String,
    success: bool,
) {
    emit_event(
        app,
        AgentEvent::ToolResult {
            conversation_id: conversation_id.to_string(),
            id,
            output,
            success,
        },
    );
}

fn emit_memory_suggestion(
    app: &AppHandle,
    conversation_id: &str,
    suggestion: memory::MemorySuggestion,
) {
    emit_event(
        app,
        AgentEvent::MemorySuggestion {
            conversation_id: conversation_id.to_string(),
            suggestion,
        },
    );
}

fn emit_done(app: &AppHandle, conversation_id: &str) {
    emit_event(
        app,
        AgentEvent::Done {
            conversation_id: conversation_id.to_string(),
        },
    );
}

fn emit_error(app: &AppHandle, conversation_id: &str, message: String) {
    emit_event(
        app,
        AgentEvent::Error {
            conversation_id: conversation_id.to_string(),
            message,
        },
    );
}

fn emit_state(app: &AppHandle, conversation_id: &str, state: &str) {
    emit_event(
        app,
        AgentEvent::StateChange {
            conversation_id: conversation_id.to_string(),
            state: state.to_string(),
        },
    );
}

async fn push_tool_result(
    session_map: &SharedSessionMap,
    conversation_id: &str,
    tool_use_id: &str,
    content: &str,
) {
    let mut map = session_map.lock().await;
    if let Some(sess) = map.get_mut(conversation_id) {
        sess.messages.push(Message {
            role: "tool".into(),
            content: MessageContent::Blocks(vec![ContentBlock::ToolResult {
                tool_use_id: tool_use_id.into(),
                content: content.into(),
            }]),
        });
    }
}

// ── Main agent loop ──

pub async fn run_agent(
    app: AppHandle,
    request: AgentRequest,
    session_map: SharedSessionMap,
    research_context: Option<String>,
) -> Result<(), String> {
    let knowledge_root = std::path::PathBuf::from(&request.knowledge_root);

    let provider = match request.provider.to_lowercase().as_str() {
        "anthropic" => LlmProvider::Anthropic,
        _ => LlmProvider::OpenAI,
    };
    let client = LlmClient::new(LlmConfig {
        provider,
        base_url: request.base_url.clone(),
        api_key: request.api_key.clone(),
        model: request.model.clone(),
    })?;

    let tools = agent_tools::get_tool_definitions();
    let user_profile = build_user_profile_context(&knowledge_root, &request.locale);
    let memory_context = memory::build_memory_context(&request.locale);
    let system_prompt = build_system_prompt(&request.locale, &user_profile, &memory_context);
    let conversation_id = request.conversation_id.clone();

    // Load persisted session + append the user message.
    {
        let mut map = session_map.lock().await;
        let sess = map
            .entry(conversation_id.clone())
            .or_insert_with(AgentSession::new);
        if sess.messages.is_empty() {
            sess.messages = conversations::load_llm_messages(&conversation_id);
        }
        // If the persisted session is empty, seed from frontend-provided history.
        if sess.messages.is_empty() && !request.history.is_empty() {
            for line in request
                .history
                .iter()
                .rev()
                .take(8)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
            {
                if matches!(line.role.as_str(), "user" | "assistant") {
                    sess.messages.push(Message {
                        role: line.role.clone(),
                        content: MessageContent::Text(line.content.clone()),
                    });
                }
            }
        }
        sess.prepare_run();
        let mut user_content = request.message.clone();
        if let Some(ref rc) = research_context {
            user_content.push_str(rc);
        }
        sess.messages.push(Message {
            role: "user".into(),
            content: MessageContent::Text(user_content),
        });
        sess.cancel_token = CancellationToken::new();
    }

    let cancel_token = {
        let map = session_map.lock().await;
        map.get(&conversation_id)
            .map(|s| s.cancel_token.clone())
            .unwrap_or_else(|| CancellationToken::new())
    };

    emit_state(&app, &conversation_id, "processing");

    let mut loop_count = 0usize;
    let mut sse_retry_count = 0u32;

    loop {
        loop_count += 1;
        if loop_count > MAX_TOOL_LOOPS {
            emit_error(
                &app,
                &conversation_id,
                format!("Reached maximum tool-call rounds ({MAX_TOOL_LOOPS})"),
            );
            break;
        }

        if cancel_token.is_cancelled() {
            emit_error(&app, &conversation_id, "Cancelled by user".into());
            break;
        }

        // Snapshot messages with byte-budget truncation + orphan cleanup.
        let messages: Vec<Message> = {
            let map = session_map.lock().await;
            let all: Vec<Message> = map
                .get(&conversation_id)
                .map(|s| s.messages.clone())
                .unwrap_or_default();
            let mut budget = MAX_CONTEXT_BYTES;
            let mut kept: Vec<&Message> = Vec::new();
            for m in all.iter().rev() {
                let sz = serde_json::to_string(m).map(|s| s.len()).unwrap_or(256);
                if sz > budget {
                    break;
                }
                budget -= sz;
                kept.push(m);
            }
            kept.reverse();
            let mut owned: Vec<Message> = kept.into_iter().cloned().collect();
            ensure_tool_results_paired(&mut owned);
            owned
        };

        let mut rx = match client.chat_stream(&messages, &tools, &system_prompt).await {
            Ok(rx) => rx,
            Err(ref e) if is_llm_server_down(e) => {
                emit_error(&app, &conversation_id, format_server_down_error(e));
                break;
            }
            Err(e) => {
                if sse_retry_count < MAX_SSE_RETRIES {
                    sse_retry_count += 1;
                    log::warn!(
                        "[AgentLoop] chat_stream failed, retrying ({}/{}): {}",
                        sse_retry_count,
                        MAX_SSE_RETRIES,
                        e
                    );
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    loop_count -= 1;
                    continue;
                }
                emit_error(&app, &conversation_id, e);
                break;
            }
        };

        // Consume the stream with per-event timeout.
        let mut text_accum = String::new();
        let mut thinking_accum = String::new();
        let mut thinking_sig = String::new();
        let mut tool_args_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut tool_calls: Vec<llm_stream::ToolCall> = Vec::new();
        let mut stop_reason = String::new();
        let mut had_error = false;
        let mut sse_error_msg = String::new();
        let mut received_any_token = false;
        let mut wait_warnings = 0u32;

        loop {
            let timeout_secs = if received_any_token {
                INTER_TOKEN_TIMEOUT_SECS
            } else {
                FIRST_TOKEN_TIMEOUT_SECS
            };
            let recv_result = tokio::select! {
                result = tokio::time::timeout(Duration::from_secs(timeout_secs), rx.recv()) => result,
                _ = cancel_token.cancelled() => {
                    emit_error(&app, &conversation_id,"Cancelled by user".into());
                    had_error = true;
                    sse_error_msg.clear();
                    break;
                }
            };
            match recv_result {
                Err(_elapsed) => {
                    wait_warnings += 1;
                    if wait_warnings <= MAX_WAIT_WARNINGS {
                        let hint = if request.locale == "en" {
                            format!(
                                "\n⏳ Still waiting for model response... ({}/{})\n",
                                wait_warnings, MAX_WAIT_WARNINGS
                            )
                        } else {
                            format!(
                                "\n⏳ 仍在等待模型响应... ({}/{})\n",
                                wait_warnings, MAX_WAIT_WARNINGS
                            )
                        };
                        emit_text(&app, &conversation_id, hint);
                        continue;
                    }
                    let timeout_msg = if received_any_token {
                        format!(
                            "⚠️ Model stopped responding (no data for {}s).",
                            INTER_TOKEN_TIMEOUT_SECS
                        )
                    } else {
                        format!(
                            "⚠️ LLM did not respond within {}s.",
                            FIRST_TOKEN_TIMEOUT_SECS * (MAX_WAIT_WARNINGS as u64 + 1)
                        )
                    };
                    emit_error(&app, &conversation_id, timeout_msg);
                    had_error = true;
                    sse_error_msg.clear();
                    break;
                }
                Ok(None) => break,
                Ok(Some(event)) => match event {
                    LlmEvent::TextDelta(text) => {
                        received_any_token = true;
                        text_accum.push_str(&text);
                        emit_text(&app, &conversation_id, text);
                    }
                    LlmEvent::Thinking(text) => {
                        received_any_token = true;
                        thinking_accum.push_str(&text);
                        emit_thinking(&app, &conversation_id, text);
                    }
                    LlmEvent::ThinkingSignature(sig) => {
                        thinking_sig = sig;
                    }
                    LlmEvent::ToolCallStart { id, name } => {
                        received_any_token = true;
                        tool_args_map.insert(id.clone(), String::new());
                        tool_calls.push(llm_stream::ToolCall {
                            id: id.clone(),
                            name: name.clone(),
                            arguments: String::new(),
                        });
                        emit_tool_start(&app, &conversation_id, id, name);
                    }
                    LlmEvent::ToolCallDelta { id, args_chunk } => {
                        if let Some(args) = tool_args_map.get_mut(&id) {
                            args.push_str(&args_chunk);
                        }
                        emit_tool_args(&app, &conversation_id, id, args_chunk);
                    }
                    LlmEvent::ToolCallEnd { id } => {
                        let final_args = tool_args_map.remove(&id).unwrap_or_default();
                        if let Some(tc) = tool_calls.iter_mut().find(|t| t.id == id) {
                            // Repair common LLM JSON malformations before execution.
                            tc.arguments = json_repair::repair_tool_args(&tc.name, &final_args);
                        }
                    }
                    LlmEvent::Done {
                        stop_reason: reason,
                    } => {
                        stop_reason = reason;
                        break;
                    }
                    LlmEvent::Error(msg) => {
                        if is_llm_server_down(&msg) {
                            emit_error(&app, &conversation_id, format_server_down_error(&msg));
                            had_error = true;
                            sse_error_msg.clear();
                            break;
                        }
                        sse_error_msg = msg;
                        had_error = true;
                        break;
                    }
                },
            }
        }

        if had_error {
            if !sse_error_msg.is_empty() && sse_retry_count < MAX_SSE_RETRIES {
                sse_retry_count += 1;
                log::warn!(
                    "[AgentLoop] SSE stream error, retrying ({}/{}): {}",
                    sse_retry_count,
                    MAX_SSE_RETRIES,
                    sse_error_msg
                );
                // Preserve partial text so the retry doesn't lose it.
                if !text_accum.is_empty() && tool_calls.is_empty() {
                    let mut map = session_map.lock().await;
                    if let Some(sess) = map.get_mut(&conversation_id) {
                        sess.messages.push(Message {
                            role: "assistant".into(),
                            content: MessageContent::Text(text_accum.clone()),
                        });
                    }
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
                loop_count -= 1;
                continue;
            }
            // Remove the user message that caused the error from history.
            let mut map = session_map.lock().await;
            if let Some(sess) = map.get_mut(&conversation_id) {
                if sess.messages.last().map(|m| m.role.as_str()) == Some("user") {
                    sess.messages.pop();
                }
            }
            break;
        }

        // Reset SSE retry budget on success.
        sse_retry_count = 0;

        // Store the assistant message.
        {
            let mut map = session_map.lock().await;
            if let Some(sess) = map.get_mut(&conversation_id) {
                sess.messages.push(build_assistant_message(
                    &text_accum,
                    &thinking_accum,
                    &thinking_sig,
                    &tool_calls,
                ));
            }
        }

        // If no tool calls, the turn is done.
        if tool_calls.is_empty() {
            log::info!("[AgentLoop] LLM finished with no tool calls (reason: {stop_reason})");
            break;
        }

        // Deduplicate within the batch: skip calls whose (name, args) hash
        // already appeared earlier in this batch. The LLM occasionally
        // emits the same call 3-4 times in one turn as a "triple-tap" quirk.
        // Duplicate ids still get a synthetic result so tool_use ↔ tool_result
        // pairing stays intact.
        let mut seen_hashes: HashMap<u64, String> = HashMap::new();
        let mut deduped: Vec<(usize, Option<llm_stream::ToolCall>)> =
            Vec::with_capacity(tool_calls.len());
        for (i, tc) in tool_calls.iter().enumerate() {
            let h = loop_args_hash(&tc.name, &tc.arguments);
            if let Some(first_id) = seen_hashes.get(&h) {
                let msg =
                    format!("Skipped — identical call already executed as tool_use {first_id}");
                emit_tool_result(&app, &conversation_id, tc.id.clone(), msg.clone(), true);
                push_tool_result(&session_map, &conversation_id, &tc.id, &msg).await;
                deduped.push((i, None));
            } else {
                seen_hashes.insert(h, tc.id.clone());
                deduped.push((i, Some(tc.clone())));
            }
        }

        // Execute tool calls (duplicates already handled above).
        emit_state(&app, &conversation_id, "executing");
        let unique_calls: Vec<&llm_stream::ToolCall> =
            deduped.iter().filter_map(|(_, opt)| opt.as_ref()).collect();
        let all_shared = unique_calls.iter().all(|tc| is_shared_tool(&tc.name));
        if all_shared && unique_calls.len() > 1 {
            let mut handles = Vec::with_capacity(unique_calls.len());
            for tc in &unique_calls {
                let name = tc.name.clone();
                let args = tc.arguments.clone();
                let kr = knowledge_root.clone();
                let loc = request.locale.clone();
                handles.push(tokio::spawn(async move {
                    let parsed: Value = serde_json::from_str(&args).unwrap_or(json!({}));
                    agent_tools::execute_tool(&name, &parsed, &kr, &loc).await
                }));
            }
            let results = futures_util::future::join_all(handles).await;
            for (tc, result) in unique_calls.iter().zip(results) {
                let result = result.unwrap_or_else(|e| agent_tools::ToolResult {
                    success: false,
                    output: format!("Tool task panicked: {e}"),
                });
                if tc.name == "suggest_memory" && result.success {
                    for suggestion in
                        memory::parse_memory_suggestions(&tc.arguments, &conversation_id)
                    {
                        emit_memory_suggestion(&app, &conversation_id, suggestion);
                    }
                }
                let preview = truncate_output(&result.output);
                emit_tool_result(
                    &app,
                    &conversation_id,
                    tc.id.clone(),
                    preview,
                    result.success,
                );
                push_tool_result(&session_map, &conversation_id, &tc.id, &result.output).await;
            }
        } else {
            for tc in &unique_calls {
                if cancel_token.is_cancelled() {
                    break;
                }
                let parsed: Value = serde_json::from_str(&tc.arguments).unwrap_or(json!({}));
                let result =
                    agent_tools::execute_tool(&tc.name, &parsed, &knowledge_root, &request.locale)
                        .await;
                if tc.name == "suggest_memory" && result.success {
                    for suggestion in
                        memory::parse_memory_suggestions(&tc.arguments, &conversation_id)
                    {
                        emit_memory_suggestion(&app, &conversation_id, suggestion);
                    }
                }
                let preview = truncate_output(&result.output);
                emit_tool_result(
                    &app,
                    &conversation_id,
                    tc.id.clone(),
                    preview,
                    result.success,
                );
                push_tool_result(&session_map, &conversation_id, &tc.id, &result.output).await;
            }
        }

        // Loop detection: only track unique calls (across iterations).
        {
            let mut map = session_map.lock().await;
            if let Some(sess) = map.get_mut(&conversation_id) {
                for h in seen_hashes.keys() {
                    if let Some(reason) = sess.record_call_and_detect_loop(*h) {
                        log::warn!("[AgentLoop] Loop guard tripped on hash {h}: {reason}");
                    }
                }
            }
        }

        emit_state(&app, &conversation_id, "processing");
    }

    // Persist session + finalize.
    {
        let mut map = session_map.lock().await;
        if let Some(sess) = map.get_mut(&conversation_id) {
            sess.running = false;
            if let Err(error) = conversations::save_llm_messages(&conversation_id, &sess.messages) {
                log::error!(
                    "[AgentSession] Failed to save conversation {conversation_id}: {error}"
                );
            }
        }
    }
    emit_done(&app, &conversation_id);
    emit_state(&app, &conversation_id, "idle");

    Ok(())
}
