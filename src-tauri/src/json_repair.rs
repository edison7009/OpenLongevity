// JSON repair for LLM tool-call arguments.
//
// Ported from EchoBird's services/json_repair.rs. Layered repair:
// Layer 1: direct parse (fast path).
// Layer 2: structural repair (fences, commas, quotes, escapes).
// Layer 3: key-value scrape as last resort.
// On total failure the original string is returned so the tool surfaces the
// real parse error rather than a misleading stub.

/// Normalize tool-call arguments into valid JSON. Returns the original string
/// on total failure so the tool's own parse error reaches the model.
pub fn repair_tool_args(_tool_name: &str, args: &str) -> String {
    if serde_json::from_str::<serde_json::Value>(args).is_ok() {
        return args.to_string();
    }
    let repaired = repair_json(args);
    if serde_json::from_str::<serde_json::Value>(&repaired).is_ok() {
        return repaired;
    }
    let extracted = extract_json_fields(args);
    if let Some(obj) = extracted.as_object() {
        if !obj.is_empty() {
            if let Ok(s) = serde_json::to_string(&extracted) {
                return s;
            }
        }
    }
    args.to_string()
}

/// Repair common LLM-JSON malformations. Order matters: escape-fix before
/// fence-strip; unquoted-key insertion before trailing-comma cleanup.
pub fn repair_json(s: &str) -> String {
    let mut result = s.to_string();

    // Fix invalid backslash escapes (e.g. `\.` → `\\.`).
    let valid_escapes = ['\\', '"', '/', 'n', 'r', 't', 'b', 'f', 'u'];
    let chars: Vec<char> = result.chars().collect();
    let mut fixed = String::with_capacity(result.len() + 16);
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            let next = chars[i + 1];
            if valid_escapes.contains(&next) {
                fixed.push('\\');
                fixed.push(next);
            } else {
                fixed.push('\\');
                fixed.push('\\');
                fixed.push(next);
            }
            i += 2;
        } else {
            fixed.push(chars[i]);
            i += 1;
        }
    }
    result = fixed;

    // Strip markdown fences.
    result = result.trim().to_string();
    if let Some(rest) = result.strip_prefix("```json") {
        result = rest.trim().to_string();
    } else if let Some(rest) = result.strip_prefix("```") {
        result = rest.trim().to_string();
    }
    if let Some(rest) = result.strip_suffix("```") {
        result = rest.trim().to_string();
    }

    // Single-quote → double-quote only when no double quotes are present.
    if !result.contains('"') && result.contains('\'') {
        result = result.replace('\'', "\"");
    }

    // Insert missing commas between `"a": "x" "b": "y"`.
    let chars: Vec<char> = result.chars().collect();
    let mut insertions: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '"' {
            let mut k = i + 1;
            while k < chars.len() && chars[k] != '"' {
                if chars[k] == '\\' {
                    k += 1;
                }
                k += 1;
            }
            if k + 1 < chars.len() {
                let mut q = k + 1;
                while q < chars.len() && chars[q].is_whitespace() {
                    q += 1;
                }
                if q < chars.len() && chars[q] == '"' {
                    let mut r = q + 1;
                    while r < chars.len() && chars[r] != '"' {
                        if chars[r] == '\\' {
                            r += 1;
                        }
                        r += 1;
                    }
                    if r + 1 < chars.len() {
                        let mut t = r + 1;
                        while t < chars.len() && chars[t].is_whitespace() {
                            t += 1;
                        }
                        if t < chars.len() && chars[t] == ':' {
                            insertions.push(k + 1);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    let mut chars = chars;
    for pos in insertions.into_iter().rev() {
        chars.insert(pos, ',');
    }
    result = chars.into_iter().collect();

    // Quote bare keys: {path: "src"} → {"path": "src"}.
    let rchars: Vec<char> = result.chars().collect();
    let mut fixed = String::with_capacity(result.len() + 16);
    let mut ri = 0;
    while ri < rchars.len() {
        if rchars[ri] == '{' || rchars[ri] == ',' {
            fixed.push(rchars[ri]);
            ri += 1;
            while ri < rchars.len() && rchars[ri].is_whitespace() {
                fixed.push(rchars[ri]);
                ri += 1;
            }
            if ri < rchars.len() && rchars[ri].is_alphabetic() {
                let key_start = ri;
                while ri < rchars.len() && (rchars[ri].is_alphanumeric() || rchars[ri] == '_') {
                    ri += 1;
                }
                let mut ki = ri;
                while ki < rchars.len() && rchars[ki].is_whitespace() {
                    ki += 1;
                }
                if ki < rchars.len() && rchars[ki] == ':' {
                    fixed.push('"');
                    for c in &rchars[key_start..ri] {
                        fixed.push(*c);
                    }
                    fixed.push('"');
                } else {
                    for c in &rchars[key_start..ri] {
                        fixed.push(*c);
                    }
                }
            }
        } else {
            fixed.push(rchars[ri]);
            ri += 1;
        }
    }
    result = fixed;

    // Drop trailing commas.
    loop {
        let before = result.clone();
        result = result.replace(",}", "}").replace(",]", "]");
        if result == before {
            break;
        }
    }

    // Wrap if neither object nor array.
    if !result.starts_with('{') && !result.starts_with('[') {
        result = format!("{{{}}}", result);
    }

    // Pad missing closing braces.
    let open = result.chars().filter(|c| *c == '{').count();
    let close = result.chars().filter(|c| *c == '}').count();
    for _ in 0..open.saturating_sub(close) {
        result.push('}');
    }

    result
}

/// Last-resort key-value scraper. Finds every `"key": value` pair by string
/// matching, ignoring outer JSON structure.
pub fn extract_json_fields(s: &str) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let key = if chars[i] == '"' {
            let start = i + 1;
            i = start;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            let k: String = chars[start..i.min(len)].iter().collect();
            if i < len {
                i += 1;
            }
            k
        } else if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            chars[start..i].iter().collect()
        } else {
            i += 1;
            continue;
        };

        while i < len && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= len || chars[i] != ':' {
            continue;
        }
        i += 1;
        while i < len && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= len {
            break;
        }

        if chars[i] == '"' {
            let start = i + 1;
            i = start;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            let raw: String = chars[start..i.min(len)].iter().collect();
            let val = raw
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            map.insert(key, serde_json::json!(val));
            if i < len {
                i += 1;
            }
        } else if chars[i] == 't' || chars[i] == 'f' {
            let start = i;
            while i < len && chars[i].is_alphabetic() {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            match word.as_str() {
                "true" => {
                    map.insert(key, serde_json::json!(true));
                }
                "false" => {
                    map.insert(key, serde_json::json!(false));
                }
                _ => {
                    map.insert(key, serde_json::json!(word));
                }
            }
        } else if chars[i].is_ascii_digit() || chars[i] == '-' {
            let start = i;
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '-') {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            if let Ok(n) = num_str.parse::<i64>() {
                map.insert(key, serde_json::json!(n));
            } else if let Ok(f) = num_str.parse::<f64>() {
                map.insert(key, serde_json::json!(f));
            }
        } else {
            let start = i;
            while i < len && !matches!(chars[i], ',' | '}' | ']') {
                i += 1;
            }
            let val: String = chars[start..i]
                .iter()
                .collect::<String>()
                .trim()
                .to_string();
            if !val.is_empty() {
                map.insert(key, serde_json::json!(val));
            }
        }
    }

    serde_json::Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> serde_json::Value {
        serde_json::from_str(s).expect("expected valid JSON")
    }

    #[test]
    fn passes_valid_json_through_unchanged() {
        let input = r#"{"file_path":"/tmp/a.rs","content":"x"}"#;
        assert_eq!(repair_tool_args("file_write", input), input);
    }

    #[test]
    fn fixes_trailing_comma() {
        let v = parse(&repair_json(r#"{"key": "value",}"#));
        assert_eq!(v["key"], "value");
    }

    #[test]
    fn fixes_single_quotes() {
        let v = parse(&repair_json("{'key': 'value'}"));
        assert_eq!(v["key"], "value");
    }

    #[test]
    fn fixes_unquoted_keys() {
        let v = parse(&repair_json(r#"{path: "src/main.rs"}"#));
        assert_eq!(v["path"], "src/main.rs");
    }

    #[test]
    fn fixes_invalid_backslash_dot_in_regex() {
        let v = parse(&repair_json(r#"{"pattern":"app\.rs"}"#));
        assert!(v["pattern"].as_str().unwrap().contains('.'));
    }

    #[test]
    fn fixes_markdown_json_fence() {
        let v = parse(&repair_json("```json\n{\"key\": \"value\"}\n```"));
        assert_eq!(v["key"], "value");
    }

    #[test]
    fn fixes_markdown_bare_fence() {
        let v = parse(&repair_json("```\n{\"key\": \"value\"}\n```"));
        assert_eq!(v["key"], "value");
    }

    #[test]
    fn fixes_missing_comma_between_fields() {
        let repaired = repair_json(r#"{"path": "src" "depth": 2}"#);
        let v = parse(&repaired);
        assert_eq!(v["path"], "src");
        assert_eq!(v["depth"], 2);
    }

    #[test]
    fn extract_fields_basic() {
        let v = extract_json_fields(r#"{"file_path": "/src/main.rs", "pattern": "hello"}"#);
        assert_eq!(v["file_path"], "/src/main.rs");
        assert_eq!(v["pattern"], "hello");
    }

    #[test]
    fn extract_fields_booleans_and_numbers() {
        let v = extract_json_fields(r#"{"recursive": true, "depth": 3}"#);
        assert_eq!(v["recursive"], true);
        assert_eq!(v["depth"], 3);
    }

    #[test]
    fn keeps_empty_object_untouched() {
        assert_eq!(repair_tool_args("file_write", "{}"), "{}");
    }

    #[test]
    fn returns_original_when_unsalvageable() {
        assert_eq!(repair_tool_args("file_write", "!!!"), "!!!");
    }

    #[test]
    fn full_chain_recovers_fence_wrapped_json() {
        let input = "```json\n{\"file_path\":\"/tmp/a.rs\",\"content\":\"x\"}\n```";
        let v = parse(&repair_tool_args("file_write", input));
        assert_eq!(v["file_path"], "/tmp/a.rs");
    }
}
