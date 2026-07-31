use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const MEMORY_CONTEXT_MAX_BYTES: usize = 12_000;
const VALID_KINDS: &[&str] = &[
    "goal",
    "preference",
    "constraint",
    "profile",
    "correction",
    "health_context",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemorySuggestion {
    pub id: String,
    pub kind: String,
    pub content: String,
    pub source_conversation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryItem {
    pub id: String,
    pub kind: String,
    pub content: String,
    pub source_conversation_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStore {
    pub version: u32,
    pub items: Vec<MemoryItem>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self {
            version: 1,
            items: Vec::new(),
        }
    }
}

fn memory_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("OpenLongevity")
        .join("memory")
}

fn memory_file() -> PathBuf {
    memory_dir().join("memory.json")
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn write_json_atomic(store: &MemoryStore) -> Result<(), String> {
    fs::create_dir_all(memory_dir())
        .map_err(|error| format!("Cannot create memory dir: {error}"))?;
    let path = memory_file();
    let temp = path.with_extension("json.tmp");
    let json = serde_json::to_string_pretty(store)
        .map_err(|error| format!("Cannot serialize memory: {error}"))?;
    fs::write(&temp, json).map_err(|error| format!("Cannot write {}: {error}", temp.display()))?;
    if path.exists() {
        fs::remove_file(&path)
            .map_err(|error| format!("Cannot replace {}: {error}", path.display()))?;
    }
    fs::rename(&temp, &path).map_err(|error| format!("Cannot move memory file: {error}"))
}

fn load_store() -> MemoryStore {
    fs::read_to_string(memory_file())
        .ok()
        .and_then(|json| serde_json::from_str::<MemoryStore>(&json).ok())
        .unwrap_or_default()
}

fn validate_memory(kind: &str, content: &str) -> Result<(), String> {
    if !VALID_KINDS.contains(&kind) {
        return Err(format!("Invalid memory kind: {kind}"));
    }
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err("Memory content is required".to_string());
    }
    if trimmed.chars().count() > 240 {
        return Err("Memory content must be 240 characters or fewer".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn confirm_memory_suggestion(suggestion: MemorySuggestion) -> Result<MemoryItem, String> {
    validate_memory(&suggestion.kind, &suggestion.content)?;
    let mut store = load_store();
    let normalized = suggestion.content.trim().to_string();
    if let Some(existing) = store.items.iter().find(|item| {
        item.kind == suggestion.kind && item.content.trim().eq_ignore_ascii_case(&normalized)
    }) {
        return Ok(existing.clone());
    }

    let timestamp = now_ms();
    let item = MemoryItem {
        id: if suggestion.id.trim().is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            suggestion.id
        },
        kind: suggestion.kind,
        content: normalized,
        source_conversation_id: suggestion.source_conversation_id,
        created_at: timestamp,
        updated_at: timestamp,
    };
    store.items.push(item.clone());
    write_json_atomic(&store)?;
    Ok(item)
}

pub fn build_memory_context(locale: &str) -> String {
    let store = load_store();
    if store.items.is_empty() {
        return String::new();
    }

    let heading = if locale == "en" {
        "## CONFIRMED LONG-TERM MEMORY\nThese are user-confirmed facts. Use them as lightweight context; do not treat them as medical orders."
    } else {
        "## 已确认长期记忆\n以下是用户确认过的短事实。将其作为轻量背景，不要把它们当作医疗指令。"
    };
    let mut output = String::from("\n\n");
    output.push_str(heading);
    for item in store.items.iter().rev() {
        let line = format!("\n- [{}] {}", item.kind, item.content.trim());
        if output.len() + line.len() > MEMORY_CONTEXT_MAX_BYTES {
            break;
        }
        output.push_str(&line);
    }
    output
}

pub fn parse_memory_suggestions(args: &str, conversation_id: &str) -> Vec<MemorySuggestion> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(args) else {
        return Vec::new();
    };
    let values = value
        .get("items")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_else(|| vec![value]);

    values
        .into_iter()
        .filter_map(|item| {
            let kind = item.get("kind")?.as_str()?.trim().to_string();
            let content = item.get("content")?.as_str()?.trim().to_string();
            if validate_memory(&kind, &content).is_err() {
                return None;
            }
            Some(MemorySuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                kind,
                content,
                source_conversation_id: conversation_id.to_string(),
            })
        })
        .take(3)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_memory() {
        assert!(validate_memory("goal", "Improve sleep").is_ok());
        assert!(validate_memory("unknown", "Improve sleep").is_err());
        assert!(validate_memory("goal", "").is_err());
    }

    #[test]
    fn parses_array_suggestions() {
        let parsed = parse_memory_suggestions(
            r#"{"items":[{"kind":"goal","content":"Improve sleep"},{"kind":"preference","content":"Prefers concise answers"}]}"#,
            "abc",
        );
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].source_conversation_id, "abc");
    }
}
