// Agent Loop — ReAct loop for Open Longevity.
// Streams text + tool calls to the frontend via Tauri events.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

use crate::agent_tools;
use crate::llm_stream::{self, LlmEvent, ToolCallAccumulator};

const MAX_TOOL_LOOPS: usize = 12;

// ── Events emitted to frontend ──

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "tool_call_start")]
    ToolCallStart { id: String, name: String },
    #[serde(rename = "tool_call_args")]
    ToolCallArgs { id: String, args: String },
    #[serde(rename = "tool_result")]
    ToolResult {
        id: String,
        output: String,
        success: bool,
    },
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error { message: String },
}

// ── Request from frontend ──

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRequest {
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
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryLine {
    pub role: String,
    pub content: String,
}

// ── Session state (kept in memory) ──

#[allow(dead_code)] pub struct AgentSession {
    pub messages: Vec<Value>,
}

impl AgentSession {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

// ── Session Persistence ──

fn sessions_dir() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("OpenLongevity")
        .join("sessions")
}

fn session_file() -> std::path::PathBuf {
    sessions_dir().join("default.json")
}

pub fn save_session_to_disk(messages: &[Value]) {
    if let Err(e) = std::fs::create_dir_all(sessions_dir()) {
        log::error!("[AgentSession] Failed to create sessions dir: {e}");
        return;
    }
    let path = session_file();
    match serde_json::to_string_pretty(messages) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                log::error!("[AgentSession] Failed to write session: {e}");
            }
        }
        Err(e) => log::error!("[AgentSession] Failed to serialize session: {e}"),
    }
}

pub fn load_session_from_disk() -> Vec<Value> {
    let path = session_file();
    if !path.exists() {
        return Vec::new();
    }
    match std::fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn clear_session_from_disk() {
    let path = session_file();
    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }
}

// ── User Profile Bootstrap ──

/// Read the user's personal files and build a context block for the system prompt.
/// This is called at the start of each agent run so the AI always knows the user.
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
        // Try localized path first
        let localized = if locale == "en" {
            let stem = base_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            let ext = base_path.extension().unwrap_or_default().to_string_lossy().to_string();
            base_path.with_file_name(format!("{stem}.en.{ext}"))
        } else {
            base_path.clone()
        };

        let path = if localized.is_file() { localized } else if base_path.is_file() { base_path.clone() } else { continue };

        if let Ok(content) = std::fs::read_to_string(&path) {
            if !content.trim().is_empty() {
                let truncated = if content.len() > 6000 {
                    let mut t = content.chars().take(5900).collect::<String>();
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
        "\n\n## USER PROFILE (always available — do not ask the user to repeat this information)\n{}",
        sections.join("\n\n")
    )
}

// ── Main agent loop ──

pub async fn run_agent(
    app: AppHandle,
    request: AgentRequest,
    _session: &mut AgentSession,
    research_context: Option<String>,
) -> Result<(), String> {
    let knowledge_root = std::path::PathBuf::from(&request.knowledge_root);
    let tools = agent_tools::get_tool_definitions();
    let language_rule = if request.locale == "en" {
        "Reply in English."
    } else {
        "使用简体中文回答。"
    };

    // Proactively read user profile and inject into system prompt
    let user_profile = build_user_profile_context(&knowledge_root, &request.locale);

    let system_prompt = format!(
        "You are Open Longevity, a local-first scientific longevity assistant with tool-calling ability. \
         You can save notes to the user's knowledge library, search existing notes, and read note content. \
         When the user wants to record, save, or remember something, use the save_note tool — do NOT just \
         tell them to do it manually. When answering questions about the library, use search_library and \
         read_note to ground your answers in actual notes. \
         The user's local notes are your primary memory. Cite the note path in parentheses when a \
         statement depends on it. Clearly separate the user's personal protocol from general information. \
         Never invent a study, measurement, dose, or source. Preserve concise safety boundaries for \
         medication interactions, allergies, pregnancy, and organ impairment when relevant. \
         Do not diagnose or prescribe. {language_rule}{user_profile}"
    );

    // Load persisted session history (if any)
    let persisted = load_session_from_disk();

    // Build messages for this turn
    let mut messages = vec![json!({ "role": "system", "content": system_prompt })];

    // Add persisted history (skip the system message if present)
    for msg in &persisted {
        let role = msg.pointer("/role").and_then(Value::as_str).unwrap_or("");
        if matches!(role, "user" | "assistant" | "tool") {
            messages.push(msg.clone());
        }
    }

    // Also add frontend-provided history (for non-persistent contexts)
    if persisted.is_empty() {
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
                messages.push(json!({ "role": line.role, "content": line.content }));
            }
        }
    }

    // Build the user message with context
    let mut user_content = request.message.clone();
    if let Some(ref rc) = research_context {
        user_content.push_str(rc);
    }
    messages.push(json!({ "role": "user", "content": user_content }));

    // ReAct loop
    for _loop_idx in 0..MAX_TOOL_LOOPS {
        // Call LLM with streaming
        let mut rx: tokio::sync::mpsc::Receiver<LlmEvent> = match llm_stream::chat_stream(
            &request.api_key,
            &request.base_url,
            &request.model,
            &messages,
            &tools,
        )
        .await
        {
            Ok(rx) => rx,
            Err(e) => {
                // If streaming fails, try non-streaming fallback
                log::warn!("[AgentLoop] Streaming failed, trying sync: {e}");
                match llm_stream::chat_sync(
                    &request.api_key,
                    &request.base_url,
                    &request.model,
                    &messages,
                    &tools,
                )
                .await
                {
                    Ok((text, tool_calls)) => {
                        if !text.is_empty() {
                            let _ = app.emit(
                                "agent_event",
                                AgentEvent::TextDelta { text: text.clone() },
                            );
                        }
                        if tool_calls.is_empty() {
                            let to_save: Vec<Value> = messages.iter().skip(1).cloned().collect();
                            save_session_to_disk(&to_save);
                            let _ = app.emit("agent_event", AgentEvent::Done);
                            return Ok(());
                        }
                        // Process tool calls from sync response
                        process_sync_tool_calls(
                            &app,
                            &mut messages,
                            &text,
                            &tool_calls,
                            &knowledge_root,
                            &request.locale,
                        )
                        .await;
                        continue;
                    }
                    Err(sync_err) => {
                        let _ = app.emit(
                            "agent_event",
                            AgentEvent::Error {
                                message: format!("Stream: {e} | Sync: {sync_err}"),
                            },
                        );
                        let _ = app.emit("agent_event", AgentEvent::Done);
                        return Err(sync_err);
                    }
                }
            }
        };

        // Collect streaming events
        let mut text_buffer = String::new();
        let mut pending_tool_calls: Vec<ToolCallAccumulator> = Vec::new();
        let mut _current_tool_id: Option<String> = None;
        let mut current_tool_name: Option<String> = None;
        let mut current_tool_args = String::new();
        let mut had_tool_calls = false;

        while let Some(event) = rx.recv().await {
            match event {
                LlmEvent::TextDelta(text) => {
                    text_buffer.push_str(&text);
                    let _ = app.emit(
                        "agent_event",
                        AgentEvent::TextDelta { text },
                    );
                }
                LlmEvent::ToolCallStart { id, name } => {
                    had_tool_calls = true;
                    _current_tool_id = Some(id.clone());
                    current_tool_name = Some(name.clone());
                    current_tool_args.clear();
                    let _ = app.emit(
                        "agent_event",
                        AgentEvent::ToolCallStart { id, name },
                    );
                }
                LlmEvent::ToolCallDelta { id, args_chunk } => {
                    current_tool_args.push_str(&args_chunk);
                    let _ = app.emit(
                        "agent_event",
                        AgentEvent::ToolCallArgs {
                            id,
                            args: args_chunk,
                        },
                    );
                }
                LlmEvent::ToolCallEnd { id } => {
                    let name = current_tool_name.take().unwrap_or_default();
                    let args = std::mem::take(&mut current_tool_args);
                    pending_tool_calls.push(ToolCallAccumulator {
                        id,
                        name,
                        arguments: args,
                    });
                    _current_tool_id = None;
                }
                LlmEvent::Done => break,
                LlmEvent::Error(msg) => {
                    let _ = app.emit("agent_event", AgentEvent::Error { message: msg });
                    let _ = app.emit("agent_event", AgentEvent::Done);
                    return Ok(());
                }
            }
        }

        if !had_tool_calls {
            // No tool calls — the LLM is done
            // Save session (exclude system message)
            let to_save: Vec<Value> = messages.iter().skip(1).cloned().collect();
            save_session_to_disk(&to_save);
            let _ = app.emit("agent_event", AgentEvent::Done);
            return Ok(());
        }

        // Add assistant message with tool calls to history
        let tool_calls_json: Vec<Value> = pending_tool_calls
            .iter()
            .map(|tc| {
                json!({
                    "id": tc.id,
                    "type": "function",
                    "function": {
                        "name": tc.name,
                        "arguments": tc.arguments,
                    }
                })
            })
            .collect();

        let assistant_content = if text_buffer.is_empty() {
            Value::Null
        } else {
            Value::String(text_buffer.clone())
        };

        messages.push(json!({
            "role": "assistant",
            "content": assistant_content,
            "tool_calls": tool_calls_json,
        }));

        // Execute each tool call
        for tc in &pending_tool_calls {
            let args: Value = serde_json::from_str(&tc.arguments).unwrap_or(json!({}));
            let result = agent_tools::execute_tool(
                &tc.name,
                &args,
                &knowledge_root,
                &request.locale,
            )
            .await;

            let output_preview = if result.output.len() > 4000 {
                format!(
                    "{}\n…[truncated, {} total chars]",
                    &result.output[..result.output.floor_char_boundary(4000)],
                    result.output.chars().count()
                )
            } else {
                result.output.clone()
            };

            let _ = app.emit(
                "agent_event",
                AgentEvent::ToolResult {
                    id: tc.id.clone(),
                    output: output_preview,
                    success: result.success,
                },
            );

            messages.push(json!({
                "role": "tool",
                "tool_call_id": tc.id,
                "content": result.output,
            }));
        }

        // Save session after tool calls (so partial progress is preserved)
        let to_save: Vec<Value> = messages.iter().skip(1).cloned().collect();
        save_session_to_disk(&to_save);

        // Continue the loop — the LLM will see the tool results
    }

    // Max loops reached
    let _ = app.emit(
        "agent_event",
        AgentEvent::TextDelta {
            text: if request.locale == "en" {
                "\n\n[Maximum tool-call rounds reached]".into()
            } else {
                "\n\n[已达到最大工具调用轮数]".into()
            },
        },
    );
    let _ = app.emit("agent_event", AgentEvent::Done);
    Ok(())
}

async fn process_sync_tool_calls(
    app: &AppHandle,
    messages: &mut Vec<Value>,
    text: &str,
    tool_calls: &[ToolCallAccumulator],
    knowledge_root: &std::path::Path,
    locale: &str,
) {
    let tool_calls_json: Vec<Value> = tool_calls
        .iter()
        .map(|tc| {
            json!({
                "id": tc.id,
                "type": "function",
                "function": {
                    "name": tc.name,
                    "arguments": tc.arguments,
                }
            })
        })
        .collect();

    let assistant_content = if text.is_empty() {
        Value::Null
    } else {
        Value::String(text.to_string())
    };

    messages.push(json!({
        "role": "assistant",
        "content": assistant_content,
        "tool_calls": tool_calls_json,
    }));

    for tc in tool_calls {
        let _ = app.emit(
            "agent_event",
            AgentEvent::ToolCallStart {
                id: tc.id.clone(),
                name: tc.name.clone(),
            },
        );
        let _ = app.emit(
            "agent_event",
            AgentEvent::ToolCallArgs {
                id: tc.id.clone(),
                args: tc.arguments.clone(),
            },
        );

        let args: Value = serde_json::from_str(&tc.arguments).unwrap_or(json!({}));
        let result =
            agent_tools::execute_tool(&tc.name, &args, knowledge_root, locale).await;

        let _ = app.emit(
            "agent_event",
            AgentEvent::ToolResult {
                id: tc.id.clone(),
                output: result.output.clone(),
                success: result.success,
            },
        );

        messages.push(json!({
            "role": "tool",
            "tool_call_id": tc.id,
            "content": result.output,
        }));
    }
}

// Polyfill for Rust < 1.91
#[allow(dead_code)] trait CharBoundaryExt {
    fn floor_char_boundary(&self, index: usize) -> usize;
}

impl CharBoundaryExt for str {
    fn floor_char_boundary(&self, index: usize) -> usize {
        if index >= self.len() {
            return self.len();
        }
        let mut i = index;
        while i > 0 && !self.is_char_boundary(i) {
            i -= 1;
        }
        i
    }
}