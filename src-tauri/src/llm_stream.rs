// LLM Client — OpenAI and Anthropic protocols with SSE streaming.
// Ported from EchoBird's services/llm_client.rs and adapted to OpenLongevity.

use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use tokio::sync::mpsc;

/// Maximum quiet time between SSE chunks before declaring the upstream stalled.
const SSE_CHUNK_TIMEOUT: Duration = Duration::from_secs(300);

/// Number of attempts (including the first) for retryable upstream errors.
const MAX_RETRY_ATTEMPTS: u32 = 3;

// ── Public Types ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking {
        thinking: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        signature: String,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

/// Events emitted during streaming.
#[derive(Debug, Clone)]
pub enum LlmEvent {
    TextDelta(String),
    Thinking(String),
    ThinkingSignature(String),
    ToolCallStart { id: String, name: String },
    ToolCallDelta { id: String, args_chunk: String },
    ToolCallEnd { id: String },
    Done { stop_reason: String },
    Error(String),
}

// ── Client ──

#[derive(Clone)]
pub struct LlmClient {
    config: LlmConfig,
    http: reqwest::Client,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Result<Self, String> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {e}"))?;
        Ok(Self { config, http })
    }

    #[allow(dead_code)]
    pub fn provider(&self) -> LlmProvider {
        self.config.provider
    }

    /// Stream a chat completion. Returns a channel receiver of LlmEvent.
    pub async fn chat_stream(
        &self,
        messages: &[Message],
        tools: &[ToolDef],
        system_prompt: &str,
    ) -> Result<mpsc::Receiver<LlmEvent>, String> {
        match self.config.provider {
            LlmProvider::OpenAI => {
                self.chat_stream_openai(messages, tools, system_prompt)
                    .await
            }
            LlmProvider::Anthropic => {
                self.chat_stream_anthropic(messages, tools, system_prompt)
                    .await
            }
        }
    }

    // ── OpenAI ──

    async fn chat_stream_openai(
        &self,
        messages: &[Message],
        tools: &[ToolDef],
        system_prompt: &str,
    ) -> Result<mpsc::Receiver<LlmEvent>, String> {
        let url = openai_endpoint(&self.config.base_url);

        let tools_json: Vec<Value> = tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            })
            .collect();

        let mut msgs = vec![json!({"role": "system", "content": system_prompt})];
        for m in messages {
            msgs.push(message_to_openai_json(m));
        }

        let mut body = json!({
            "model": self.config.model,
            "messages": msgs,
            "stream": true,
        });
        if !tools_json.is_empty() {
            body["tools"] = json!(tools_json);
            body["tool_choice"] = json!("auto");
        }

        let auth_value = HeaderValue::from_str(&format!("Bearer {}", self.config.api_key))
            .map_err(|e| format!("Invalid API key: {e}"))?;

        let http = self.http.clone();
        let url_owned = url.clone();
        let body_owned = body.clone();
        let make_request = move || -> Result<reqwest::RequestBuilder, String> {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(AUTHORIZATION, auth_value.clone());
            headers.insert(
                "HTTP-Referer",
                HeaderValue::from_static("https://openlongevity.science"),
            );
            headers.insert("X-Title", HeaderValue::from_static("Open Longevity"));
            Ok(http.post(&url_owned).headers(headers).json(&body_owned))
        };

        let OpenedStream { mut es, pending } =
            open_with_retry(make_request, MAX_RETRY_ATTEMPTS).await?;

        let (tx, rx) = mpsc::channel::<LlmEvent>(128);
        tokio::spawn(async move {
            // index -> (id, name, args)
            let mut tool_acc: std::collections::HashMap<i64, (String, String, String)> =
                std::collections::HashMap::new();
            let mut events: std::collections::VecDeque<Result<Event, reqwest_eventsource::Error>> =
                pending.into_iter().map(Ok).collect();

            loop {
                let event = if let Some(buffered) = events.pop_front() {
                    Some(buffered)
                } else {
                    match tokio::time::timeout(SSE_CHUNK_TIMEOUT, es.next()).await {
                        Ok(maybe_event) => maybe_event,
                        Err(_) => {
                            let _ = tx
                                .send(LlmEvent::Error(format!(
                                    "Upstream stalled (no data for {}s)",
                                    SSE_CHUNK_TIMEOUT.as_secs()
                                )))
                                .await;
                            break;
                        }
                    }
                };
                let Some(event) = event else { break };
                match event {
                    Ok(Event::Message(msg)) => {
                        if msg.data == "[DONE]" {
                            let _ = tx
                                .send(LlmEvent::Done {
                                    stop_reason: "stop".into(),
                                })
                                .await;
                            break;
                        }
                        let Ok(chunk) = serde_json::from_str::<Value>(&msg.data) else {
                            continue;
                        };

                        let delta = match chunk.pointer("/choices/0/delta") {
                            Some(d) => d,
                            None => {
                                if let Some(reason) = chunk
                                    .pointer("/choices/0/finish_reason")
                                    .and_then(Value::as_str)
                                {
                                    let _ = tx
                                        .send(LlmEvent::Done {
                                            stop_reason: reason.into(),
                                        })
                                        .await;
                                    break;
                                }
                                continue;
                            }
                        };

                        if let Some(content) = delta.get("content").and_then(Value::as_str) {
                            if !content.is_empty() {
                                let _ = tx.send(LlmEvent::TextDelta(content.to_string())).await;
                            }
                        }
                        if let Some(reasoning) =
                            delta.get("reasoning_content").and_then(Value::as_str)
                        {
                            if !reasoning.is_empty() {
                                let _ = tx.send(LlmEvent::Thinking(reasoning.to_string())).await;
                            }
                        }
                        if let Some(tool_calls) = delta.get("tool_calls").and_then(Value::as_array)
                        {
                            for tc in tool_calls {
                                let idx = tc.get("index").and_then(Value::as_i64).unwrap_or(0);
                                let is_new = !tool_acc.contains_key(&idx);
                                let entry = tool_acc.entry(idx).or_insert((
                                    String::new(),
                                    String::new(),
                                    String::new(),
                                ));
                                if let Some(id) = tc.get("id").and_then(Value::as_str) {
                                    if entry.0.is_empty() {
                                        entry.0 = id.to_string();
                                    }
                                }
                                if let Some(func) = tc.get("function") {
                                    if let Some(name) = func.get("name").and_then(Value::as_str) {
                                        if entry.1.is_empty() {
                                            entry.1 = name.to_string();
                                        }
                                    }
                                    if let Some(args) =
                                        func.get("arguments").and_then(Value::as_str)
                                    {
                                        entry.2.push_str(args);
                                    }
                                }
                                if is_new && !entry.0.is_empty() && !entry.1.is_empty() {
                                    let _ = tx
                                        .send(LlmEvent::ToolCallStart {
                                            id: entry.0.clone(),
                                            name: entry.1.clone(),
                                        })
                                        .await;
                                }
                            }
                        }
                        if let Some(reason) = chunk
                            .pointer("/choices/0/finish_reason")
                            .and_then(Value::as_str)
                        {
                            for (_, (id, _, _)) in tool_acc.drain() {
                                let _ = tx.send(LlmEvent::ToolCallEnd { id }).await;
                            }
                            let _ = tx
                                .send(LlmEvent::Done {
                                    stop_reason: reason.into(),
                                })
                                .await;
                            break;
                        }
                    }
                    Ok(Event::Open) => {}
                    Err(e) => {
                        let msg = match e {
                            reqwest_eventsource::Error::InvalidStatusCode(status, response) => {
                                let body = response.text().await.unwrap_or_default();
                                extract_upstream_error_message(status.as_u16(), &body)
                            }
                            reqwest_eventsource::Error::Transport(t) => {
                                format!("Connection error: {t}")
                            }
                            other => format!("Stream error: {other}"),
                        };
                        let _ = tx.send(LlmEvent::Error(msg)).await;
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }

    // ── Anthropic ──

    async fn chat_stream_anthropic(
        &self,
        messages: &[Message],
        tools: &[ToolDef],
        system_prompt: &str,
    ) -> Result<mpsc::Receiver<LlmEvent>, String> {
        let url = anthropic_endpoint(&self.config.base_url);

        let normalized = normalize_anthropic_messages(messages);
        let msgs: Vec<Value> = normalized.iter().map(message_to_anthropic_json).collect();

        let tools_json: Vec<Value> = tools
            .iter()
            .map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.parameters,
                })
            })
            .collect();

        let mut body = json!({
            "model": self.config.model,
            "system": system_prompt,
            "messages": msgs,
            "max_tokens": 4096,
            "stream": true,
        });
        if !tools_json.is_empty() {
            body["tools"] = json!(tools_json);
        }

        let api_key_value = HeaderValue::from_str(&self.config.api_key)
            .map_err(|e| format!("Invalid API key: {e}"))?;

        let http = self.http.clone();
        let url_owned = url.clone();
        let body_owned = body.clone();
        let make_request = move || -> Result<reqwest::RequestBuilder, String> {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert("x-api-key", api_key_value.clone());
            headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
            headers.insert(
                "HTTP-Referer",
                HeaderValue::from_static("https://openlongevity.science"),
            );
            headers.insert("X-Title", HeaderValue::from_static("Open Longevity"));
            Ok(http.post(&url_owned).headers(headers).json(&body_owned))
        };

        let OpenedStream { mut es, pending } = open_with_retry(make_request, 1).await?;

        let (tx, rx) = mpsc::channel::<LlmEvent>(128);
        tokio::spawn(async move {
            let mut current_tool_id = String::new();
            let mut done_sent = false;
            let mut events: std::collections::VecDeque<Result<Event, reqwest_eventsource::Error>> =
                pending.into_iter().map(Ok).collect();

            loop {
                let event = if let Some(buffered) = events.pop_front() {
                    Some(buffered)
                } else {
                    match tokio::time::timeout(SSE_CHUNK_TIMEOUT, es.next()).await {
                        Ok(maybe_event) => maybe_event,
                        Err(_) => {
                            let _ = tx
                                .send(LlmEvent::Error(format!(
                                    "Upstream stalled (no data for {}s)",
                                    SSE_CHUNK_TIMEOUT.as_secs()
                                )))
                                .await;
                            break;
                        }
                    }
                };
                let Some(event) = event else { break };
                match event {
                    Ok(Event::Message(msg)) => {
                        let Ok(data) = serde_json::from_str::<Value>(&msg.data) else {
                            continue;
                        };
                        match msg.event.as_str() {
                            "content_block_start" => {
                                if let Some(block) = data.get("content_block") {
                                    if block.get("type").and_then(Value::as_str) == Some("tool_use")
                                    {
                                        current_tool_id = block
                                            .get("id")
                                            .and_then(Value::as_str)
                                            .unwrap_or("")
                                            .to_string();
                                        let name = block
                                            .get("name")
                                            .and_then(Value::as_str)
                                            .unwrap_or("")
                                            .to_string();
                                        let _ = tx
                                            .send(LlmEvent::ToolCallStart {
                                                id: current_tool_id.clone(),
                                                name,
                                            })
                                            .await;
                                    }
                                }
                            }
                            "content_block_delta" => {
                                if let Some(delta) = data.get("delta") {
                                    match delta.get("type").and_then(Value::as_str) {
                                        Some("text_delta") => {
                                            if let Some(text) =
                                                delta.get("text").and_then(Value::as_str)
                                            {
                                                let _ = tx
                                                    .send(LlmEvent::TextDelta(text.to_string()))
                                                    .await;
                                            }
                                        }
                                        Some("input_json_delta") => {
                                            if let Some(json_str) =
                                                delta.get("partial_json").and_then(Value::as_str)
                                            {
                                                let _ = tx
                                                    .send(LlmEvent::ToolCallDelta {
                                                        id: current_tool_id.clone(),
                                                        args_chunk: json_str.to_string(),
                                                    })
                                                    .await;
                                            }
                                        }
                                        Some("thinking_delta") => {
                                            if let Some(thinking) =
                                                delta.get("thinking").and_then(Value::as_str)
                                            {
                                                let _ = tx
                                                    .send(LlmEvent::Thinking(thinking.to_string()))
                                                    .await;
                                            }
                                        }
                                        Some("signature_delta") => {
                                            if let Some(sig) =
                                                delta.get("signature").and_then(Value::as_str)
                                            {
                                                let _ = tx
                                                    .send(LlmEvent::ThinkingSignature(
                                                        sig.to_string(),
                                                    ))
                                                    .await;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            "content_block_stop" => {
                                if !current_tool_id.is_empty() {
                                    let _ = tx
                                        .send(LlmEvent::ToolCallEnd {
                                            id: current_tool_id.clone(),
                                        })
                                        .await;
                                    current_tool_id.clear();
                                }
                            }
                            "message_delta" => {
                                if let Some(reason) =
                                    data.pointer("/delta/stop_reason").and_then(Value::as_str)
                                {
                                    let _ = tx
                                        .send(LlmEvent::Done {
                                            stop_reason: reason.to_string(),
                                        })
                                        .await;
                                    done_sent = true;
                                }
                            }
                            "message_stop" => {
                                if !done_sent {
                                    let _ = tx
                                        .send(LlmEvent::Done {
                                            stop_reason: "end_turn".into(),
                                        })
                                        .await;
                                }
                                break;
                            }
                            "error" => {
                                let err_msg = data
                                    .pointer("/error/message")
                                    .and_then(Value::as_str)
                                    .unwrap_or("Unknown Anthropic error");
                                let _ = tx.send(LlmEvent::Error(err_msg.to_string())).await;
                                break;
                            }
                            _ => {}
                        }
                    }
                    Ok(Event::Open) => {}
                    Err(e) => {
                        let msg = match e {
                            reqwest_eventsource::Error::InvalidStatusCode(status, response) => {
                                let body = response.text().await.unwrap_or_default();
                                extract_upstream_error_message(status.as_u16(), &body)
                            }
                            reqwest_eventsource::Error::Transport(t) => {
                                format!("Connection error: {t}")
                            }
                            other => format!("Stream error: {other}"),
                        };
                        let _ = tx.send(LlmEvent::Error(msg)).await;
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }
}

// ── Helpers ──

fn openai_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/chat/completions")
    }
}

fn anthropic_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/messages") {
        trimmed.to_string()
    } else if trimmed.ends_with("/v1") {
        format!("{trimmed}/messages")
    } else {
        format!("{trimmed}/v1/messages")
    }
}

fn message_to_openai_json(m: &Message) -> Value {
    match &m.content {
        MessageContent::Text(text) => json!({"role": m.role, "content": text}),
        MessageContent::Blocks(blocks) => {
            if (m.role == "tool" || m.role == "user") && blocks.len() == 1 {
                if let Some(ContentBlock::ToolResult {
                    tool_use_id,
                    content,
                }) = blocks.first()
                {
                    return json!({
                        "role": "tool",
                        "tool_call_id": tool_use_id,
                        "content": content,
                    });
                }
            }
            if m.role == "assistant" {
                let mut tool_calls = Vec::new();
                let mut text_parts = Vec::new();
                let mut reasoning_parts = Vec::new();
                for block in blocks {
                    match block {
                        ContentBlock::Text { text } => text_parts.push(text.clone()),
                        ContentBlock::Thinking { thinking, .. } => {
                            reasoning_parts.push(thinking.clone())
                        }
                        ContentBlock::ToolUse { id, name, input } => {
                            tool_calls.push(json!({
                                "id": id,
                                "type": "function",
                                "function": {"name": name, "arguments": input.to_string()},
                            }));
                        }
                        ContentBlock::ToolResult { .. } => {}
                    }
                }
                let mut msg = json!({"role": "assistant"});
                msg["content"] = json!(text_parts.join(""));
                if !reasoning_parts.is_empty() {
                    msg["reasoning_content"] = json!(reasoning_parts.join(""));
                }
                if !tool_calls.is_empty() {
                    msg["tool_calls"] = json!(tool_calls);
                }
                return msg;
            }
            json!({"role": m.role, "content": m.content.clone()})
        }
    }
}

fn message_to_anthropic_json(m: &Message) -> Value {
    match &m.content {
        MessageContent::Text(text) => json!({"role": m.role, "content": text}),
        MessageContent::Blocks(blocks) => {
            let content: Vec<Value> = blocks
                .iter()
                .map(|b| match b {
                    ContentBlock::Text { text } => json!({"type": "text", "text": text}),
                    ContentBlock::Thinking { thinking, signature } => {
                        let mut block = json!({"type": "thinking", "thinking": thinking});
                        if !signature.is_empty() {
                            block["signature"] = json!(signature);
                        }
                        block
                    }
                    ContentBlock::ToolUse { id, name, input } => {
                        json!({"type": "tool_use", "id": id, "name": name, "input": input})
                    }
                    ContentBlock::ToolResult { tool_use_id, content } => {
                        json!({"type": "tool_result", "tool_use_id": tool_use_id, "content": content})
                    }
                })
                .collect();
            json!({"role": m.role, "content": content})
        }
    }
}

/// Merge consecutive pure-tool_result user messages into one. Anthropic
/// requires all tool_results for a turn in the immediately-following user
/// message; splitting them produces a 400 about unmatched ids.
fn normalize_anthropic_messages(messages: &[Message]) -> Vec<Message> {
    fn is_pure_tool_result_user(m: &Message) -> bool {
        if m.role != "user" {
            return false;
        }
        match &m.content {
            MessageContent::Blocks(blocks) => {
                !blocks.is_empty()
                    && blocks
                        .iter()
                        .all(|b| matches!(b, ContentBlock::ToolResult { .. }))
            }
            _ => false,
        }
    }

    let mut out: Vec<Message> = Vec::with_capacity(messages.len());
    let mut i = 0;
    while i < messages.len() {
        if is_pure_tool_result_user(&messages[i]) {
            let mut merged: Vec<ContentBlock> = Vec::new();
            while i < messages.len() && is_pure_tool_result_user(&messages[i]) {
                if let MessageContent::Blocks(blocks) = &messages[i].content {
                    merged.extend(blocks.iter().cloned());
                }
                i += 1;
            }
            out.push(Message {
                role: "user".into(),
                content: MessageContent::Blocks(merged),
            });
        } else {
            out.push(messages[i].clone());
            i += 1;
        }
    }
    out
}

/// Status codes worth retrying. 408 deliberately excluded — a streaming
/// request that already timed out is unlikely to recover on retry.
fn is_retryable_status(status: u16) -> bool {
    status == 429 || (500..600).contains(&status)
}

/// Honor `Retry-After: <seconds>`, capped at 30s.
fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let raw = headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    let secs: u64 = raw.trim().parse().ok()?;
    Some(Duration::from_secs(secs.min(30)))
}

/// Exponential backoff for attempt N (0-indexed). 500ms × 3^N.
fn backoff_for_attempt(attempt: u32) -> Duration {
    let base_ms: u64 = 500;
    let factor: u64 = 3u64.saturating_pow(attempt);
    Duration::from_millis(base_ms.saturating_mul(factor))
}

/// Extract a user-friendly error message from an upstream error response.
fn extract_upstream_error_message(status: u16, body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return format!("Upstream returned HTTP {status}");
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
        if let Some(msg) = parsed
            .get("error")
            .and_then(|e| e.get("message"))
            .and_then(Value::as_str)
        {
            return msg.to_string();
        }
        if let Some(msg) = parsed.get("error").and_then(Value::as_str) {
            return msg.to_string();
        }
        if let Some(msg) = parsed.get("message").and_then(Value::as_str) {
            return msg.to_string();
        }
    }
    if looks_like_html(trimmed) {
        if let Some(title) = extract_html_title(trimmed) {
            return title;
        }
        return format!("Upstream returned HTTP {status}");
    }
    let truncated: String = trimmed.chars().take(200).collect();
    if truncated.is_empty() {
        format!("Upstream returned HTTP {status}")
    } else {
        truncated
    }
}

fn looks_like_html(body: &str) -> bool {
    let head: String = body
        .trim_start()
        .chars()
        .take(64)
        .collect::<String>()
        .to_ascii_lowercase();
    head.starts_with("<!doctype html")
        || head.starts_with("<html")
        || head.starts_with("<head")
        || head.starts_with("<body")
        || head.starts_with("<center>")
}

fn extract_html_title(body: &str) -> Option<String> {
    let lower = body.to_ascii_lowercase();
    let start_tag = lower.find("<title")?;
    let after_open = body[start_tag..].find('>').map(|i| start_tag + i + 1)?;
    let end_tag = lower[after_open..].find("</title>")?;
    let title = body[after_open..after_open + end_tag].trim();
    if title.is_empty() {
        None
    } else {
        Some(title.to_string())
    }
}

/// Connection-time outcome from `open_with_retry`.
struct OpenedStream {
    es: EventSource,
    pending: Vec<Event>,
}

/// Open an EventSource with transient-error retry. Returns when the server
/// emits `Open` (healthy stream) or a `Message` directly (buffered into
/// `pending`), or when a non-retryable error fires / attempts are exhausted.
async fn open_with_retry<F>(make_request: F, max_attempts: u32) -> Result<OpenedStream, String>
where
    F: Fn() -> Result<reqwest::RequestBuilder, String>,
{
    let mut last_err = String::from("All retry attempts failed");
    for attempt in 0..max_attempts {
        let request = make_request()?;
        let mut es = match EventSource::new(request) {
            Ok(es) => es,
            Err(e) => return Err(format!("Could not build request: {e}")),
        };

        match es.next().await {
            Some(Ok(Event::Open)) => {
                return Ok(OpenedStream {
                    es,
                    pending: Vec::new(),
                });
            }
            Some(Ok(ev @ Event::Message(_))) => {
                return Ok(OpenedStream {
                    es,
                    pending: vec![ev],
                });
            }
            Some(Err(reqwest_eventsource::Error::InvalidStatusCode(status, response))) => {
                let retry_after = parse_retry_after(response.headers());
                let body = response.text().await.unwrap_or_default();
                let message = extract_upstream_error_message(status.as_u16(), &body);
                if is_retryable_status(status.as_u16()) && attempt + 1 < max_attempts {
                    let wait = retry_after.unwrap_or_else(|| backoff_for_attempt(attempt));
                    log::info!(
                        "[LLM] Upstream {} (attempt {}/{}), retrying in {:?}: {}",
                        status,
                        attempt + 1,
                        max_attempts,
                        wait,
                        message
                    );
                    tokio::time::sleep(wait).await;
                    last_err = message;
                    continue;
                }
                return Err(message);
            }
            Some(Err(reqwest_eventsource::Error::Transport(t))) => {
                if attempt + 1 < max_attempts {
                    let wait = backoff_for_attempt(attempt);
                    log::info!(
                        "[LLM] Transport error (attempt {}/{}), retrying in {:?}: {}",
                        attempt + 1,
                        max_attempts,
                        wait,
                        t
                    );
                    tokio::time::sleep(wait).await;
                    last_err = format!("Connection error: {t}");
                    continue;
                }
                return Err(format!("Connection error: {t}"));
            }
            Some(Err(other)) => {
                return Err(format!("SSE setup error: {other}"));
            }
            None => {
                if attempt + 1 < max_attempts {
                    tokio::time::sleep(backoff_for_attempt(attempt)).await;
                    last_err = "Upstream closed connection immediately".to_string();
                    continue;
                }
                return Err("Upstream closed connection immediately".to_string());
            }
        }
    }
    Err(last_err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openai_endpoint_appends_chat_completions() {
        assert_eq!(
            openai_endpoint("https://api.openai.com/v1"),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(
            openai_endpoint("https://api.openai.com/v1/"),
            "https://api.openai.com/v1/chat/completions"
        );
    }

    #[test]
    fn openai_endpoint_keeps_existing_chat_completions() {
        assert_eq!(
            openai_endpoint("https://api.openai.com/v1/chat/completions"),
            "https://api.openai.com/v1/chat/completions"
        );
    }

    #[test]
    fn anthropic_endpoint_appends_v1_messages() {
        assert_eq!(
            anthropic_endpoint("https://api.anthropic.com"),
            "https://api.anthropic.com/v1/messages"
        );
    }

    #[test]
    fn anthropic_endpoint_appends_messages_to_v1() {
        assert_eq!(
            anthropic_endpoint("https://api.anthropic.com/v1"),
            "https://api.anthropic.com/v1/messages"
        );
    }

    #[test]
    fn anthropic_endpoint_keeps_messages_suffix() {
        assert_eq!(
            anthropic_endpoint("https://api.anthropic.com/v1/messages"),
            "https://api.anthropic.com/v1/messages"
        );
    }

    #[test]
    fn normalize_merges_consecutive_tool_result_users() {
        let input = vec![
            Message {
                role: "user".into(),
                content: MessageContent::Text("go".into()),
            },
            Message {
                role: "assistant".into(),
                content: MessageContent::Blocks(vec![
                    ContentBlock::ToolUse {
                        id: "c1".into(),
                        name: "t".into(),
                        input: json!({}),
                    },
                    ContentBlock::ToolUse {
                        id: "c2".into(),
                        name: "t".into(),
                        input: json!({}),
                    },
                ]),
            },
            Message {
                role: "user".into(),
                content: MessageContent::Blocks(vec![ContentBlock::ToolResult {
                    tool_use_id: "c1".into(),
                    content: "r1".into(),
                }]),
            },
            Message {
                role: "user".into(),
                content: MessageContent::Blocks(vec![ContentBlock::ToolResult {
                    tool_use_id: "c2".into(),
                    content: "r2".into(),
                }]),
            },
        ];
        let out = normalize_anthropic_messages(&input);
        assert_eq!(out.len(), 3, "user, assistant, merged-user");
        if let MessageContent::Blocks(blocks) = &out[2].content {
            assert_eq!(blocks.len(), 2, "two merged tool_results");
        } else {
            panic!("expected Blocks");
        }
    }

    #[test]
    fn retryable_includes_429_and_5xx() {
        assert!(is_retryable_status(429));
        assert!(is_retryable_status(500));
        assert!(is_retryable_status(503));
        assert!(!is_retryable_status(200));
        assert!(!is_retryable_status(400));
        assert!(!is_retryable_status(401));
    }

    #[test]
    fn backoff_grows_exponentially() {
        assert_eq!(backoff_for_attempt(0), Duration::from_millis(500));
        assert_eq!(backoff_for_attempt(1), Duration::from_millis(1500));
        assert_eq!(backoff_for_attempt(2), Duration::from_millis(4500));
    }
}
