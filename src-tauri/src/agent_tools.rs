// Agent Tools — domain-specific tools for Open Longevity.
// save_note, search_library, read_note operate on the local knowledge library.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

use crate::knowledge_map;
use crate::llm_stream::ToolDef;

// ── Tool result ──

pub struct ToolResult {
    pub success: bool,
    pub output: String,
}

// ── Tool definitions sent to the LLM ──

pub fn get_tool_definitions() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "save_note".into(),
            description: "Save a structured Markdown note to the user's local knowledge library. \
                Use this whenever the user wants to record, save, or remember information — \
                a summary, a finding, a plan, a comparison, a protocol, or any note. \
                The note is saved as a .md file in the library. \
                IMPORTANT: you MUST include both a non-empty 'title' and a non-empty 'content' \
                string in the arguments; calls with missing or empty arguments are rejected. \
                Choose category: 'inbox' for general notes, 'dossiers' for strategy/compound notes, \
                'cases' for person/protocol notes, 'stories' for anecdote/observation notes."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "A concise title for the note (used as filename stem)"
                    },
                    "content": {
                        "type": "string",
                        "description": "The note body in clean Markdown"
                    },
                    "category": {
                        "type": "string",
                        "enum": ["inbox", "dossiers", "cases", "stories"],
                        "description": "Which library folder to save into. Default: inbox"
                    }
                },
                "required": ["title", "content"]
            }),
        },
        ToolDef {
            name: "update_plan".into(),
            description: "Update one of the user's personal plan pages in the knowledge library. \
                Use this whenever the user asks to organize, update, or write their exercise, \
                supplement, diet, or daily-routine plan. Each module maps to its own page: \
                'exercise' -> plans/exercise.md, 'supplements' -> plans/supplements.md, \
                'diet' -> plans/diet.md, 'daily_routine' -> plans/daily-routine.md. \
                You MUST provide a non-empty 'module' and the full Markdown 'content' for the page. \
                Follow the page's standard format: goals, current status, concrete arrangements \
                (sets/reps or dose/frequency), and review notes. For the 'supplements' module, \
                include a '品牌' section with per-supplement top pick and alternatives (quality \
                tiers P1–P3 from the library's products/ brand-tier notes; tiers reflect quality \
                and transparency, not efficacy). Write the complete page, not just a diff."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "module": {
                        "type": "string",
                        "enum": ["exercise", "supplements", "diet", "daily_routine"],
                        "description": "Which plan page to update"
                    },
                    "content": {
                        "type": "string",
                        "description": "The full Markdown content for the plan page"
                    }
                },
                "required": ["module", "content"]
            }),
        },
        ToolDef {
            name: "update_note".into(),
            description: "Update an existing note in the knowledge library by its relative path \
                (e.g. 'dossiers/creatine.md', 'plans/exercise.md', 'cases/bryan-johnson-daily.md'). \
                Use this to edit any note, including its frontmatter and sources. Provide the full \
                new file content. Optionally provide 'sources' as a list of URLs; they are written \
                into the note's frontmatter when the content has none."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path of the note file, e.g. 'dossiers/creatine.md'"
                    },
                    "content": {
                        "type": "string",
                        "description": "The full new Markdown content of the note"
                    },
                    "sources": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Optional source URLs to record in the note's frontmatter"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        ToolDef {
            name: "update_tier".into(),
            description: "Adjust the T1–T5 tier of an item on the user's home strategy map \
                (catalog/strategies.csv). 'name' may be the item id, Chinese name, or English name \
                (e.g. 'creatine' or '肌酸' or 'Creatine'). 'tier' is one of T1, T2, T3, T4, T5, or \
                'pending' to hide the item from the home map. The home page reflects the change \
                after the library reloads."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Item id, Chinese name, or English name"
                    },
                    "tier": {
                        "type": "string",
                        "enum": ["T1", "T2", "T3", "T4", "T5", "pending"]
                    }
                },
                "required": ["name", "tier"]
            }),
        },
        ToolDef {
            name: "search_library".into(),
            description: "Search the user's local knowledge library by keyword. \
                Returns a list of matching note paths with title and a short snippet. \
                Use this to find relevant notes before answering questions or before reading a note."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Keywords to search for (space-separated terms)"
                    }
                },
                "required": ["query"]
            }),
        },
        ToolDef {
            name: "read_note".into(),
            description: "Read the full content of a note from the knowledge library. \
                Provide the relative path (e.g. 'dossiers/nmn.md'). \
                Use this after search_library to get the full text of a relevant note."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to the note file (e.g. 'dossiers/nmn.md')"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDef {
            name: "suggest_memory".into(),
            description: "Suggest one to three long-term memory candidates for the user to confirm. \
                Use only for durable user-stated goals, preferences, constraints, corrections, profile facts, or health context that should help future conversations. \
                This tool does not save memory; it only asks the frontend to show confirmation cards.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "items": {
                        "type": "array",
                        "maxItems": 3,
                        "items": {
                            "type": "object",
                            "properties": {
                                "kind": {
                                    "type": "string",
                                    "enum": ["goal", "preference", "constraint", "profile", "correction", "health_context"]
                                },
                                "content": {
                                    "type": "string",
                                    "description": "A concise user-confirmable fact, 240 characters or fewer."
                                }
                            },
                            "required": ["kind", "content"]
                        }
                    }
                },
                "required": ["items"]
            }),
        },
    ]
}

// ── Tool execution ──

pub async fn execute_tool(
    name: &str,
    args: &Value,
    knowledge_root: &Path,
    locale: &str,
) -> ToolResult {
    match name {
        "save_note" => exec_save_note(args, knowledge_root, locale),
        "update_plan" => exec_update_plan(args, knowledge_root, locale),
        "update_note" => exec_update_note(args, knowledge_root, locale),
        "update_tier" => exec_update_tier(args, knowledge_root, locale),
        "search_library" => exec_search_library(args, knowledge_root, locale),
        "read_note" => exec_read_note(args, knowledge_root),
        "suggest_memory" => ToolResult {
            success: true,
            output: "Memory suggestion sent for user confirmation.".into(),
        },
        _ => ToolResult {
            success: false,
            output: format!("Unknown tool: {name}"),
        },
    }
}

// ── save_note ──

fn exec_save_note(args: &Value, root: &Path, _locale: &str) -> ToolResult {
    if !args.is_object() {
        return ToolResult {
            success: false,
            output: "Invalid save_note arguments: expected a JSON object with a non-empty 'title' \
                and a non-empty 'content' string. Retry with complete arguments."
                .into(),
        };
    }
    let title = match args.pointer("/title").and_then(Value::as_str) {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'title' — save_note requires a non-empty 'title' and a \
                    non-empty 'content' string (category is optional). Retry with complete arguments."
                    .into(),
            }
        }
    };
    let content = match args.pointer("/content").and_then(Value::as_str) {
        Some(c) if !c.trim().is_empty() => c.trim(),
        _ => {
            return ToolResult {
                success: false,
                output:
                    "Missing or empty 'content' — save_note requires a non-empty 'title' and a \
                    non-empty 'content' string. Retry with complete arguments."
                        .into(),
            }
        }
    };
    let category = args
        .pointer("/category")
        .and_then(Value::as_str)
        .unwrap_or("inbox");

    let valid_categories = ["inbox", "dossiers", "cases", "stories"];
    if !valid_categories.contains(&category) {
        return ToolResult {
            success: false,
            output: format!(
                "Invalid category '{category}'. Must be one of: {}",
                valid_categories.join(", ")
            ),
        };
    }

    // Sanitize filename
    let safe_name = sanitize_filename(title);
    let filename = format!("{safe_name}.md");
    let dir = root.join(category);

    if let Err(e) = fs::create_dir_all(&dir) {
        return ToolResult {
            success: false,
            output: format!("Cannot create directory {}: {e}", dir.display()),
        };
    }

    let file_path = dir.join(&filename);

    // Build the note with frontmatter
    let note = format!(
        "---\ntitle: {title}\nsource: ai-agent\ncreated: {}\n---\n\n# {title}\n\n{content}\n",
        chrono::Local::now().format("%Y-%m-%d")
    );

    // Truncate if too large
    let note = if note.len() > 120_000 {
        let mut truncated = note.chars().take(119_900).collect::<String>();
        truncated.push_str("\n\n…[truncated]");
        truncated
    } else {
        note
    };

    match fs::write(&file_path, &note) {
        Ok(_) => {
            let relative = file_path
                .strip_prefix(root)
                .unwrap_or(&file_path)
                .to_string_lossy()
                .replace('\\', "/");
            ToolResult {
                success: true,
                output: format!(
                    "Saved note to {relative} ({note_len} chars)",
                    note_len = note.chars().count()
                ),
            }
        }
        Err(e) => ToolResult {
            success: false,
            output: format!("Failed to write {}: {e}", file_path.display()),
        },
    }
}

// ── update_plan ──

fn exec_update_plan(args: &Value, root: &Path, _locale: &str) -> ToolResult {
    if !args.is_object() {
        return ToolResult {
            success: false,
            output: "Invalid update_plan arguments: expected a JSON object with a non-empty \
                'module' and a non-empty 'content' string. Retry with complete arguments."
                .into(),
        };
    }
    let module = match args.pointer("/module").and_then(Value::as_str) {
        Some(m) if !m.trim().is_empty() => m.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'module' — update_plan requires 'module' to be one of: \
                    exercise, supplements, diet, daily_routine. Retry with complete arguments."
                    .into(),
            }
        }
    };
    let content = match args.pointer("/content").and_then(Value::as_str) {
        Some(c) if !c.trim().is_empty() => c.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'content' — update_plan requires the full Markdown \
                    content for the plan page. Retry with complete arguments."
                    .into(),
            }
        }
    };
    let filename = match module {
        "exercise" => "exercise.md",
        "supplements" => "supplements.md",
        "diet" => "diet.md",
        "daily_routine" => "daily-routine.md",
        _ => {
            return ToolResult {
                success: false,
                output: format!(
                    "Invalid module '{module}' — must be one of: exercise, supplements, diet, \
                     daily_routine."
                ),
            }
        }
    };

    let dir = root.join("plans");
    if let Err(e) = fs::create_dir_all(&dir) {
        return ToolResult {
            success: false,
            output: format!("Cannot create directory {}: {e}", dir.display()),
        };
    }
    let file_path = dir.join(filename);
    match fs::write(&file_path, format!("{content}\n")) {
        Ok(_) => ToolResult {
            success: true,
            output: format!(
                "Updated plans/{filename} ({chars} chars)",
                chars = content.chars().count()
            ),
        },
        Err(e) => ToolResult {
            success: false,
            output: format!("Failed to write {}: {e}", file_path.display()),
        },
    }
}

// ── update_note ──

fn exec_update_note(args: &Value, root: &Path, _locale: &str) -> ToolResult {
    if !args.is_object() {
        return ToolResult {
            success: false,
            output: "Invalid update_note arguments: expected a JSON object with a non-empty \
                'path' and a non-empty 'content' string. Retry with complete arguments."
                .into(),
        };
    }
    let path = match args.pointer("/path").and_then(Value::as_str) {
        Some(p) if !p.trim().is_empty() => p.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'path' — update_note requires a relative Markdown path \
                    inside the library (e.g. 'dossiers/creatine.md'). Retry with complete arguments."
                    .into(),
            }
        }
    };
    let clean = path.replace('\\', "/");
    if clean.starts_with('/') || clean.contains("..") || !clean.ends_with(".md") {
        return ToolResult {
            success: false,
            output: "Invalid 'path' — must be a relative Markdown path inside the library \
                (e.g. 'dossiers/creatine.md'). Retry with a valid path."
                .into(),
        };
    }
    let content = match args.pointer("/content").and_then(Value::as_str) {
        Some(c) if !c.trim().is_empty() => c.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'content' — update_note requires the full new Markdown \
                    content. Retry with complete arguments."
                    .into(),
            }
        }
    };

    let mut body = content.to_string();
    if let Some(sources) = args.pointer("/sources").and_then(Value::as_array) {
        let urls: Vec<&str> = sources.iter().filter_map(Value::as_str).collect();
        if !urls.is_empty() && !body.trim_start().starts_with("---") {
            let list = urls
                .iter()
                .map(|url| format!("  - {url}"))
                .collect::<Vec<_>>()
                .join("\n");
            body = format!("---\nsources:\n{list}\n---\n\n{body}");
        }
    }

    let file_path = root.join(&clean);
    if let Some(parent) = file_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return ToolResult {
                success: false,
                output: format!("Cannot create directory {}: {e}", parent.display()),
            };
        }
    }
    match fs::write(&file_path, format!("{body}\n")) {
        Ok(_) => ToolResult {
            success: true,
            output: format!(
                "Updated {clean} ({chars} chars)",
                chars = body.chars().count()
            ),
        },
        Err(e) => ToolResult {
            success: false,
            output: format!("Failed to write {}: {e}", file_path.display()),
        },
    }
}

// ── update_tier ──

fn split_csv_line(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                out.push(cur.trim().to_string());
                cur = String::new();
            }
            _ => cur.push(ch),
        }
    }
    out.push(cur.trim().to_string());
    out
}

fn exec_update_tier(args: &Value, root: &Path, _locale: &str) -> ToolResult {
    if !args.is_object() {
        return ToolResult {
            success: false,
            output: "Invalid update_tier arguments: expected a JSON object with a non-empty \
                'name' and 'tier'. Retry with complete arguments."
                .into(),
        };
    }
    let name = match args.pointer("/name").and_then(Value::as_str) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => {
            return ToolResult {
                success: false,
                output:
                    "Missing or empty 'name' — update_tier requires the item id, Chinese name, \
                    or English name. Retry with complete arguments."
                        .into(),
            }
        }
    };
    let tier = match args.pointer("/tier").and_then(Value::as_str) {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'tier' — update_tier requires T1, T2, T3, T4, T5, or \
                    'pending'. Retry with complete arguments."
                    .into(),
            }
        }
    };
    const VALID_TIERS: [&str; 6] = ["T1", "T2", "T3", "T4", "T5", "pending"];
    if !VALID_TIERS.contains(&tier) {
        return ToolResult {
            success: false,
            output: format!("Invalid tier '{tier}' — must be one of: T1, T2, T3, T4, T5, pending."),
        };
    }

    let strategies_path = root.join("catalog").join("strategies.csv");
    let catalog_path = if strategies_path.is_file() {
        strategies_path
    } else {
        root.join("catalog").join("supplements.csv")
    };
    let Ok(existing) = fs::read_to_string(&catalog_path) else {
        return ToolResult {
            success: false,
            output: format!("Cannot read strategy catalog at {}", catalog_path.display()),
        };
    };
    let mut lines: Vec<String> = existing.lines().map(str::to_string).collect();
    if lines.len() < 2 {
        return ToolResult {
            success: false,
            output: "Strategy catalog has no items to update.".into(),
        };
    }

    let mut found = false;
    for line in lines.iter_mut().skip(1) {
        let mut fields = split_csv_line(line);
        if fields.len() < 7 {
            continue;
        }
        let matches = fields[0].eq_ignore_ascii_case(name)
            || fields[1] == name
            || fields[2].eq_ignore_ascii_case(name);
        if matches {
            fields[6] = tier.to_string();
            *line = fields.join(",");
            found = true;
            break;
        }
    }
    if !found {
        let names: Vec<String> = lines
            .iter()
            .skip(1)
            .filter_map(|line| {
                let fields = split_csv_line(line);
                (fields.len() >= 3)
                    .then(|| format!("{} / {} / {}", fields[0], fields[1], fields[2]))
            })
            .take(24)
            .collect();
        return ToolResult {
            success: false,
            output: format!(
                "Could not find '{name}' in the strategy catalog. Available items: {}",
                names.join(", ")
            ),
        };
    }

    match fs::write(&catalog_path, format!("{}\n", lines.join("\n"))) {
        Ok(_) => ToolResult {
            success: true,
            output: format!("Updated tier of '{name}' to {tier} in catalog/strategies.csv"),
        },
        Err(e) => ToolResult {
            success: false,
            output: format!("Failed to write {}: {e}", catalog_path.display()),
        },
    }
}

// ── search_library ──

fn exec_search_library(args: &Value, root: &Path, locale: &str) -> ToolResult {
    if !args.is_object() {
        return ToolResult {
            success: false,
            output: "Invalid search_library arguments: expected a JSON object with a non-empty \
                'query' string. Retry with complete arguments."
                .into(),
        };
    }
    let query = match args.pointer("/query").and_then(Value::as_str) {
        Some(q) if !q.trim().is_empty() => q.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'query' — search_library requires a non-empty 'query' \
                    string. Retry with complete arguments."
                    .into(),
            }
        }
    };

    let results = knowledge_map::search_library(root, query, locale, 10);

    if results.is_empty() {
        return ToolResult {
            success: true,
            output: "No matching notes found.".into(),
        };
    }

    let mut output = String::new();
    for hit in &results {
        let relation = if hit.via_graph { " · linked note" } else { "" };
        output.push_str(&format!(
            "- [{}] {} (score {}{relation})\n  {}\n",
            hit.path, hit.title, hit.score, hit.snippet
        ));
    }
    ToolResult {
        success: true,
        output,
    }
}

// ── read_note ──

fn exec_read_note(args: &Value, root: &Path) -> ToolResult {
    if !args.is_object() {
        return ToolResult {
            success: false,
            output: "Invalid read_note arguments: expected a JSON object with a non-empty 'path' \
                string (e.g. 'dossiers/nmn.md'). Retry with complete arguments."
                .into(),
        };
    }
    let path = match args.pointer("/path").and_then(Value::as_str) {
        Some(p) if !p.trim().is_empty() => p.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'path' — read_note requires a non-empty 'path' string \
                    (e.g. 'dossiers/nmn.md'). Retry with complete arguments."
                    .into(),
            }
        }
    };

    // Security: prevent path traversal
    let clean = path.replace('\\', "/");
    if clean.contains("..") {
        return ToolResult {
            success: false,
            output: "Path must not contain '..'".into(),
        };
    }

    let full_path = root.join(&clean);
    match fs::read_to_string(&full_path) {
        Ok(content) => {
            let truncated = if content.len() > 50_000 {
                let mut t = content.chars().take(49_900).collect::<String>();
                t.push_str("\n\n…[truncated]");
                t
            } else {
                content
            };
            ToolResult {
                success: true,
                output: truncated,
            }
        }
        Err(e) => ToolResult {
            success: false,
            output: format!("Cannot read {path}: {e}"),
        },
    }
}

// ── Helpers ──

fn sanitize_filename(title: &str) -> String {
    let mut name = title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_lowercase()
        .replace(' ', "-");

    // Remove consecutive dashes
    while name.contains("--") {
        name = name.replace("--", "-");
    }
    name = name.trim_matches('-').to_string();

    if name.is_empty() {
        name = "untitled".into();
    }
    if name.len() > 80 {
        name = name.chars().take(80).collect();
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_note_rejects_empty_arguments_with_guidance() {
        let result = exec_save_note(&json!({}), Path::new("unused"), "zh");
        assert!(!result.success);
        assert!(result.output.contains("Missing or empty 'title'"));
        assert!(result.output.contains("Retry"));
    }

    #[test]
    fn save_note_rejects_non_object_arguments() {
        let result = exec_save_note(&json!([]), Path::new("unused"), "zh");
        assert!(!result.success);
        assert!(result.output.contains("expected a JSON object"));
    }

    #[test]
    fn save_note_reports_missing_content_after_title() {
        let result = exec_save_note(&json!({"title": "健身计划"}), Path::new("unused"), "zh");
        assert!(!result.success);
        assert!(result.output.contains("Missing or empty 'content'"));
    }

    #[test]
    fn save_note_writes_note_successfully() {
        let dir = std::env::temp_dir().join(format!("ol-save-note-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let result = exec_save_note(
            &json!({
                "title": "健身计划",
                "content": "# 健身计划\n\n内容",
                "category": "inbox"
            }),
            &dir,
            "zh",
        );
        assert!(result.success, "{}", result.output);
        assert!(dir.join("inbox").join("健身计划.md").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_library_rejects_missing_query_with_guidance() {
        let result = exec_search_library(&json!({}), Path::new("unused"), "zh");
        assert!(!result.success);
        assert!(result.output.contains("Missing or empty 'query'"));
        assert!(result.output.contains("Retry"));
    }

    #[test]
    fn update_plan_writes_module_page() {
        let dir = std::env::temp_dir().join(format!("ol-update-plan-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let result = exec_update_plan(
            &json!({"module": "exercise", "content": "# 运动计划\n\n每天深蹲 100 个"}),
            &dir,
            "zh",
        );
        assert!(result.success, "{}", result.output);
        let body = fs::read_to_string(dir.join("plans").join("exercise.md")).unwrap();
        assert!(body.contains("每天深蹲 100 个"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn update_plan_rejects_invalid_module_and_empty_content() {
        let dir =
            std::env::temp_dir().join(format!("ol-update-plan-test-{}-b", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let bad = exec_update_plan(&json!({"module": "unknown", "content": "x"}), &dir, "zh");
        assert!(!bad.success);
        assert!(bad.output.contains("Invalid module"));
        let empty = exec_update_plan(&json!({"module": "exercise", "content": ""}), &dir, "zh");
        assert!(!empty.success);
        assert!(empty.output.contains("Missing or empty 'content'"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn update_note_writes_relative_path_with_sources() {
        let dir = std::env::temp_dir().join(format!("ol-update-note-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let result = exec_update_note(
            &json!({
                "path": "dossiers/creatine.md",
                "content": "# 肌酸\n\n每日 5g",
                "sources": ["https://example.com/creatine"]
            }),
            &dir,
            "zh",
        );
        assert!(result.success, "{}", result.output);
        let body = fs::read_to_string(dir.join("dossiers").join("creatine.md")).unwrap();
        assert!(body.contains("sources:"));
        assert!(body.contains("https://example.com/creatine"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn update_note_rejects_path_traversal() {
        let dir =
            std::env::temp_dir().join(format!("ol-update-note-test-{}-b", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let result = exec_update_note(&json!({"path": "../escape.md", "content": "x"}), &dir, "zh");
        assert!(!result.success);
        assert!(result.output.contains("Invalid 'path'"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn update_tier_reassigns_catalog_row() {
        let dir = std::env::temp_dir().join(format!("ol-update-tier-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("catalog")).unwrap();
        let csv = "id,name_zh,name_en,category,bryan_status,evidence_status,tier,review_priority,notes\ncreatine,肌酸,Creatine,运动营养,example,starter,T2,review,notes here\n";
        fs::write(dir.join("catalog").join("strategies.csv"), csv).unwrap();
        let result = exec_update_tier(&json!({"name": "肌酸", "tier": "T1"}), &dir, "zh");
        assert!(result.success, "{}", result.output);
        let updated = fs::read_to_string(dir.join("catalog").join("strategies.csv")).unwrap();
        assert!(updated.contains(",T1,review,"));
        let missing = exec_update_tier(&json!({"name": "不存在", "tier": "T2"}), &dir, "zh");
        assert!(!missing.success);
        assert!(missing.output.contains("Could not find"));
        let _ = fs::remove_dir_all(&dir);
    }
}
