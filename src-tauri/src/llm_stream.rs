// Streaming LLM client with OpenAI-compatible tool-call support.
// Parses SSE chunks for text deltas and tool-call deltas.

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::mpsc;

// ── Public types ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone)]
pub enum LlmEvent {
    TextDelta(String),
    ToolCallStart { id: String, name: String },
    ToolCallDelta { id: String, args_chunk: String },
    ToolCallEnd { id: String },
    Done,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct ToolCallAccumulator {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

// ── Streaming chat completion ──

pub async fn chat_stream(
    api_key: &str,
    base_url: &str,
    model: &str,
    messages: &[Value],
    tools: &[ToolDef],
) -> Result<mpsc::Receiver<LlmEvent>, String> {
    let (tx, rx) = mpsc::channel::<LlmEvent>(128);

    let endpoint = chat_endpoint(base_url);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

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

    let mut body = json!({
        "model": model,
        "messages": messages,
        "stream": true,
    });
    if !tools_json.is_empty() {
        body["tools"] = json!(tools_json);
    }

    let response = client
        .post(&endpoint)
        .bearer_auth(api_key)
        .header("HTTP-Referer", "https://openlongevity.science")
        .header("X-Title", "Open Longevity")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Provider request failed: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let err_body = response.text().await.unwrap_or_default();
        let msg = serde_json::from_str::<Value>(&err_body)
            .ok()
            .and_then(|v| v.pointer("/error/message").and_then(Value::as_str).map(String::from))
            .unwrap_or_else(|| err_body.chars().take(300).collect());
        return Err(format!("Provider error {status}: {msg}"));
    }

    tokio::spawn(async move {
        let mut stream = response.bytes_stream();
        let mut buf = String::new();
        // Accumulate tool calls by index
        let mut tool_acc: std::collections::HashMap<usize, ToolCallAccumulator> =
            std::collections::HashMap::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(bytes) => bytes,
                Err(e) => {
                    let _ = tx.send(LlmEvent::Error(format!("Stream error: {e}"))).await;
                    return;
                }
            };
            buf.push_str(&String::from_utf8_lossy(&chunk));

            // Process complete SSE events (separated by \n\n)
            while let Some(pos) = buf.find("\n\n") {
                let event_str = buf[..pos].to_string();
                buf = buf[pos + 2..].to_string();

                for line in event_str.lines() {
                    let line = line.trim();
                    if !line.starts_with("data: ") {
                        continue;
                    }
                    let data = &line[6..];
                    if data == "[DONE]" {
                        // Flush any accumulated tool calls
                        for idx in sorted_keys(&tool_acc) {
                            if let Some(acc) = tool_acc.remove(&idx) {
                                let _ = tx
                                    .send(LlmEvent::ToolCallEnd { id: acc.id.clone() })
                                    .await;
                            }
                        }
                        let _ = tx.send(LlmEvent::Done).await;
                        return;
                    }

                    let parsed: Value = match serde_json::from_str(data) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    let choice = match parsed.pointer("/choices/0") {
                        Some(c) => c,
                        None => continue,
                    };

                    // Text delta
                    if let Some(content) = choice
                        .pointer("/delta/content")
                        .and_then(Value::as_str)
                    {
                        if !content.is_empty() {
                            let _ = tx.send(LlmEvent::TextDelta(content.to_string())).await;
                        }
                    }

                    // Tool call deltas
                    if let Some(tool_calls) = choice
                        .pointer("/delta/tool_calls")
                        .and_then(Value::as_array)
                    {
                        for tc in tool_calls {
                            let idx = tc
                                .pointer("/index")
                                .and_then(Value::as_u64)
                                .unwrap_or(0) as usize;
                            let acc = tool_acc.entry(idx).or_insert_with(|| {
                                let id = tc
                                    .pointer("/id")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string();
                                let name = tc
                                    .pointer("/function/name")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string();
                                if !id.is_empty() && !name.is_empty() {
                                    // Defer the start event until we have both id and name
                                    // We'll send it on the first delta or at flush time
                                }
                                ToolCallAccumulator {
                                    id,
                                    name,
                                    arguments: String::new(),
                                }
                            });

                            // Update id/name if they appear in this chunk
                            if let Some(id) = tc.pointer("/id").and_then(Value::as_str) {
                                if acc.id.is_empty() {
                                    acc.id = id.to_string();
                                }
                            }
                            if let Some(name) =
                                tc.pointer("/function/name").and_then(Value::as_str)
                            {
                                if acc.name.is_empty() {
                                    acc.name = name.to_string();
                                    // Now we have both — emit start
                                    let _ = tx
                                        .send(LlmEvent::ToolCallStart {
                                            id: acc.id.clone(),
                                            name: acc.name.clone(),
                                        })
                                        .await;
                                }
                            }
                            if let Some(args) = tc
                                .pointer("/function/arguments")
                                .and_then(Value::as_str)
                            {
                                if !args.is_empty() {
                                    acc.arguments.push_str(args);
                                    let _ = tx
                                        .send(LlmEvent::ToolCallDelta {
                                            id: acc.id.clone(),
                                            args_chunk: args.to_string(),
                                        })
                                        .await;
                                }
                            }
                        }
                    }

                    // Check finish_reason
                    if let Some(reason) = choice
                        .pointer("/finish_reason")
                        .and_then(Value::as_str)
                    {
                        if reason == "tool_calls" {
                            // Flush accumulated tool calls
                            for idx in sorted_keys(&tool_acc) {
                                if let Some(acc) = tool_acc.remove(&idx) {
                                    let _ = tx
                                        .send(LlmEvent::ToolCallEnd { id: acc.id.clone() })
                                        .await;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Stream ended without [DONE] — flush remaining
        for idx in sorted_keys(&tool_acc) {
            if let Some(acc) = tool_acc.remove(&idx) {
                let _ = tx.send(LlmEvent::ToolCallEnd { id: acc.id.clone() }).await;
            }
        }
        let _ = tx.send(LlmEvent::Done).await;
    });

    Ok(rx)
}

/// Non-streaming chat completion (fallback for models that don't support streaming).
pub async fn chat_sync(
    api_key: &str,
    base_url: &str,
    model: &str,
    messages: &[Value],
    tools: &[ToolDef],
) -> Result<(String, Vec<ToolCallAccumulator>), String> {
    let endpoint = chat_endpoint(base_url);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

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

    let mut body = json!({
        "model": model,
        "messages": messages,
    });
    if !tools_json.is_empty() {
        body["tools"] = json!(tools_json);
    }

    let response = client
        .post(&endpoint)
        .bearer_auth(api_key)
        .header("HTTP-Referer", "https://openlongevity.science")
        .header("X-Title", "Open Longevity")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Provider request failed: {e}"))?;

    let status = response.status();
    let payload: Value = response
        .json()
        .await
        .map_err(|e| format!("Invalid JSON from provider: {e}"))?;

    if !status.is_success() {
        let msg = payload
            .pointer("/error/message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown provider error");
        return Err(format!("Provider error {status}: {msg}"));
    }

    let choice = payload
        .pointer("/choices/0")
        .ok_or("No choices in response")?;

    let text = choice
        .pointer("/message/content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let mut tool_calls = Vec::new();
    if let Some(tcs) = choice.pointer("/message/tool_calls").and_then(Value::as_array) {
        for tc in tcs {
            let id = tc.pointer("/id").and_then(Value::as_str).unwrap_or("").to_string();
            let name = tc
                .pointer("/function/name")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let arguments = tc
                .pointer("/function/arguments")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            tool_calls.push(ToolCallAccumulator { id, name, arguments });
        }
    }

    Ok((text, tool_calls))
}

fn chat_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/chat/completions")
    }
}

fn sorted_keys(map: &std::collections::HashMap<usize, ToolCallAccumulator>) -> Vec<usize> {
    let mut keys: Vec<usize> = map.keys().copied().collect();
    keys.sort();
    keys
}