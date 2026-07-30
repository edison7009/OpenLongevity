// Agent Tools — domain-specific tools for Open Longevity.
// save_note, search_library, read_note operate on the local knowledge library.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

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
        "search_library" => exec_search_library(args, knowledge_root, locale),
        "read_note" => exec_read_note(args, knowledge_root),
        _ => ToolResult {
            success: false,
            output: format!("Unknown tool: {name}"),
        },
    }
}

// ── save_note ──

fn exec_save_note(args: &Value, root: &Path, _locale: &str) -> ToolResult {
    let title = match args.pointer("/title").and_then(Value::as_str) {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'title'".into(),
            }
        }
    };
    let content = match args.pointer("/content").and_then(Value::as_str) {
        Some(c) if !c.trim().is_empty() => c.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'content'".into(),
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
                output: format!("Saved note to {relative} ({note_len} chars)", note_len = note.chars().count()),
            }
        }
        Err(e) => ToolResult {
            success: false,
            output: format!("Failed to write {}: {e}", file_path.display()),
        },
    }
}

// ── search_library ──

fn exec_search_library(args: &Value, root: &Path, locale: &str) -> ToolResult {
    let query = match args.pointer("/query").and_then(Value::as_str) {
        Some(q) if !q.trim().is_empty() => q.trim().to_lowercase(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'query'".into(),
            }
        }
    };

    let terms: Vec<&str> = query.split_whitespace().collect();
    if terms.is_empty() {
        return ToolResult {
            success: false,
            output: "Empty search query".into(),
        };
    }

    let mut results: Vec<(usize, String, String)> = Vec::new();

    // Walk all .md files in the library
    let entries = match walk_markdown(root, locale) {
        Ok(e) => e,
        Err(e) => {
            return ToolResult {
                success: false,
                output: format!("Cannot scan library: {e}"),
            }
        }
    };

    for (relative, content) in &entries {
        let haystack = format!("{}\n{}", relative.to_lowercase(), content.to_lowercase());
        let score: usize = terms
            .iter()
            .map(|term| haystack.matches(*term).count().min(8))
            .sum();
        if score > 0 {
            // Extract a snippet around the first match
            let snippet = extract_snippet(content, &terms[0], 120);
            results.push((score, relative.clone(), snippet));
        }
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));
    results.truncate(10);

    if results.is_empty() {
        return ToolResult {
            success: true,
            output: "No matching notes found.".into(),
        };
    }

    let mut output = String::new();
    for (score, path, snippet) in &results {
        output.push_str(&format!("- [{path}] (score {score})\n  {snippet}\n"));
    }
    ToolResult {
        success: true,
        output,
    }
}

// ── read_note ──

fn exec_read_note(args: &Value, root: &Path) -> ToolResult {
    let path = match args.pointer("/path").and_then(Value::as_str) {
        Some(p) if !p.trim().is_empty() => p.trim(),
        _ => {
            return ToolResult {
                success: false,
                output: "Missing or empty 'path'".into(),
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

fn walk_markdown(root: &Path, locale: &str) -> Result<Vec<(String, String)>, String> {
    let mut results = Vec::new();
    let _suffix = if locale == "en" { ".en.md" } else { ".md" };

    for category in &["dossiers", "cases", "stories", "inbox", "papers", "sources", "plans", "profile", "records", "catalog"] {
        let dir = root.join(category);
        if !dir.is_dir() {
            continue;
        }
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            // For zh locale, skip .en.md files; for en locale, only include .en.md files
            if locale == "en" {
                if !name.ends_with(".en.md") {
                    continue;
                }
            } else {
                if name.ends_with(".en.md") {
                    continue;
                }
                if !name.ends_with(".md") {
                    continue;
                }
            }
            let relative = format!("{}/{}", category, name);
            if let Ok(content) = fs::read_to_string(&path) {
                results.push((relative, content));
            }
        }
    }
    Ok(results)
}

fn extract_snippet(content: &str, term: &str, radius: usize) -> String {
    let lower = content.to_lowercase();
    if let Some(pos) = lower.find(term) {
        let start = pos.saturating_sub(radius);
        let end = (pos + term.len() + radius).min(content.len());
        // Adjust to char boundaries
        let start = content.floor_char_boundary(start);
        let end = content.ceil_char_boundary(end);
        let snippet = content[start..end].replace('\n', " ");
        format!("…{snippet}…")
    } else {
        // Return first line as fallback
        content
            .lines()
            .find(|l| !l.trim().is_empty() && !l.starts_with('#') && !l.starts_with("---"))
            .unwrap_or("")
            .chars()
            .take(200)
            .collect()
    }
}

// Polyfill for Rust < 1.91
#[allow(dead_code)] trait CharBoundaryExt {
    fn floor_char_boundary(&self, index: usize) -> usize;
    fn ceil_char_boundary(&self, index: usize) -> usize;
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

    fn ceil_char_boundary(&self, index: usize) -> usize {
        if index >= self.len() {
            return self.len();
        }
        let mut i = index;
        while i < self.len() && !self.is_char_boundary(i) {
            i += 1;
        }
        i
    }
}