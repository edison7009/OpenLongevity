use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

const MAX_NOTE_BYTES: usize = 120_000;
const MAX_CONTEXT_BYTES: usize = 52_000;
const STARTER_PACK_VERSION: &str = "11";
const REMOVED_STARTER_FILES: &[&str] = &[
    "cases/ray-lui.md",
    "research-log/2026-07-21-ray-lui-case.md",
    "sources/ray-lui-sources-2026-07-21.md",
    "sources/user-candidate-list-2026-07-20.md",
    "research-log/2026-07-20-docs-frontend-decision.md",
    "research-log/2026-07-20-knowledge-base-bootstrap.md",
    "research-log/2026-07-20-reader-first-dossier-format.md",
];
const LEGACY_NMN_DOSSIER: &str = r#"---
id: nmn
tier: T4
status: starter
---

# NMN / NR

::: tip 30 秒结论
NMN 与 NR 常围绕 NAD 代谢讨论。整理时要把“提高某个生物标志物”与“改善长期健康结局”明确分开。
:::

## 阅读框架

- 区分 NMN、NR 和其他 NAD 前体；
- 记录研究持续时间与人群；
- 分开整理安全性、药代和临床结局；
- 跟踪正在进行和已经完成的人体试验。
"#;
include!(concat!(env!("OUT_DIR"), "/starter_files.rs"));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Supplement {
    id: String,
    name_zh: String,
    name_en: String,
    category: String,
    tier: String,
    summary: String,
    file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    id: String,
    name: String,
    name_zh: Option<String>,
    summary: String,
    file_path: Option<String>,
    accent: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Story {
    id: String,
    title: String,
    title_en: Option<String>,
    summary: String,
    summary_en: Option<String>,
    file_path: Option<String>,
    accent: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LibrarySnapshot {
    root: String,
    connected: bool,
    supplements: Vec<Supplement>,
    people: Vec<Person>,
    stories: Vec<Story>,
    note_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatLine {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatRequest {
    api_key: String,
    base_url: String,
    model: String,
    question: String,
    locale: String,
    knowledge_root: String,
    #[serde(default)]
    context_paths: Vec<String>,
    #[serde(default)]
    history: Vec<ChatLine>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CaptureRequest {
    knowledge_root: String,
    title: String,
    content: String,
    source_url: Option<String>,
    locale: String,
}

fn default_knowledge_root() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("OpenLongevity")
        .join("library")
}

fn ensure_starter_library(root: &Path) -> Result<(), String> {
    let marker = root.join(".starter-pack-initialized");
    let installed_version = fs::read_to_string(&marker).unwrap_or_default();
    if installed_version.trim() == STARTER_PACK_VERSION {
        return Ok(());
    }

    for directory in [
        "catalog", "dossiers", "cases", "stories", "papers", "sources", "inbox", "profile",
        "plans", "records",
    ] {
        fs::create_dir_all(root.join(directory))
            .map_err(|error| format!("Could not initialize library directory: {error}"))?;
    }

    for (relative_path, content) in STARTER_FILES {
        let path = root.join(relative_path);
        let is_catalog = *relative_path == "catalog/strategies.csv";
        if path.exists() && !is_catalog {
            continue;
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Could not initialize starter content: {error}"))?;
        }
        fs::write(path, content)
            .map_err(|error| format!("Could not write starter content: {error}"))?;
    }

    for (relative_path, _) in STARTER_FILES
        .iter()
        .filter(|(relative_path, _)| relative_path.ends_with(".md"))
    {
        let path = root.join(relative_path);
        if let Ok(existing) = fs::read_to_string(&path) {
            let renamed = existing.replace("OpenLongevity", "Open Longevity");
            if renamed != existing {
                fs::write(&path, renamed)
                    .map_err(|error| format!("Could not migrate starter branding: {error}"))?;
            }
        }
    }

    for relative_path in REMOVED_STARTER_FILES {
        let path = root.join(relative_path);
        if path.is_file() {
            fs::remove_file(path)
                .map_err(|error| format!("Could not remove retired starter content: {error}"))?;
        }
    }

    let nad_dossier = root.join("dossiers/nmn.md");
    if let Ok(existing) = fs::read_to_string(&nad_dossier) {
        let normalized_existing = existing.replace("\r\n", "\n");
        let legacy_comparison = normalized_existing.replace("tier: T3", "tier: T4");
        if legacy_comparison.trim() == LEGACY_NMN_DOSSIER.trim() {
            if let Some((_, updated)) = STARTER_FILES
                .iter()
                .find(|(relative_path, _)| *relative_path == "dossiers/nmn.md")
            {
                let migrated = if normalized_existing.contains("tier: T3") {
                    updated.replace("tier: T4", "tier: T3")
                } else {
                    (*updated).to_string()
                };
                fs::write(&nad_dossier, migrated)
                    .map_err(|error| format!("Could not migrate NAD+ starter dossier: {error}"))?;
            }
        }
    }

    fs::write(marker, STARTER_PACK_VERSION)
        .map_err(|error| format!("Could not finish starter library setup: {error}"))
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn english_companion_path(path: &Path) -> PathBuf {
    let stem = path
        .file_stem()
        .map(|value| value.to_string_lossy())
        .unwrap_or_default();
    let extension = path
        .extension()
        .map(|value| value.to_string_lossy())
        .unwrap_or_else(|| "md".into());
    path.with_file_name(format!("{stem}.en.{extension}"))
}

fn source_path_for_english_companion(path: &Path) -> Option<PathBuf> {
    let stem = path.file_stem()?.to_string_lossy();
    let source_stem = stem.strip_suffix(".en")?;
    let extension = path.extension()?.to_string_lossy();
    Some(path.with_file_name(format!("{source_stem}.{extension}")))
}

fn is_paired_english_companion(path: &Path) -> bool {
    source_path_for_english_companion(path).is_some_and(|source| source.is_file())
}

fn localized_note_path(path: &Path, locale: &str) -> PathBuf {
    if locale == "en" && !is_paired_english_companion(path) {
        let companion = english_companion_path(path);
        if companion.is_file() {
            return companion;
        }
    }
    path.to_path_buf()
}

fn relative_note_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn count_markdown_files(root: &Path) -> usize {
    fn visit(path: &Path, count: &mut usize) {
        let Ok(entries) = fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(&path, count);
            } else if path.extension().is_some_and(|extension| extension == "md")
                && !is_paired_english_companion(&path)
            {
                *count += 1;
            }
        }
    }

    let mut count = 0;
    visit(root, &mut count);
    count
}

fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();

    while let Some(character) = chars.next() {
        match character {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(field.trim().to_string());
                field.clear();
            }
            _ => field.push(character),
        }
    }
    fields.push(field.trim().to_string());
    fields
}

fn clean_markdown_text(value: &str) -> String {
    value
        .replace("**", "")
        .replace('`', "")
        .replace(['\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_summary(markdown: &str) -> String {
    if let Some(start) = markdown.find("::: tip") {
        let after_heading = markdown[start..]
            .find('\n')
            .map(|offset| start + offset + 1)
            .unwrap_or(start);
        if let Some(end_offset) = markdown[after_heading..].find("\n:::") {
            let summary = clean_markdown_text(&markdown[after_heading..after_heading + end_offset]);
            if !summary.is_empty() {
                return truncate_utf8(&summary, 220);
            }
        }
    }

    let mut in_frontmatter = markdown.trim_start().starts_with("---");
    let mut passed_frontmatter = !in_frontmatter;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if in_frontmatter && trimmed == "---" {
            if passed_frontmatter {
                in_frontmatter = false;
            } else {
                passed_frontmatter = true;
            }
            continue;
        }
        if in_frontmatter
            || trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with('|')
        {
            continue;
        }
        let summary = clean_markdown_text(trimmed);
        if summary.len() > 20 {
            return truncate_utf8(&summary, 220);
        }
    }
    String::new()
}

fn extract_frontmatter_value(markdown: &str, key: &str) -> Option<String> {
    let mut lines = markdown.trim_start_matches('\u{feff}').lines();
    if lines.next()?.trim() != "---" {
        return None;
    }

    for line in lines {
        let trimmed = line.trim();
        if trimmed == "---" {
            break;
        }
        let Some((field, value)) = trimmed.split_once(':') else {
            continue;
        };
        if field.trim() != key {
            continue;
        }
        let value = value.trim();
        let unquoted = if value.len() >= 2
            && ((value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\'')))
        {
            &value[1..value.len() - 1]
        } else {
            value
        };
        if !unquoted.is_empty() {
            return Some(unquoted.to_string());
        }
    }
    None
}

fn extract_markdown_title(markdown: &str) -> Option<String> {
    markdown.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|title| !title.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn truncate_utf8(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_string();
    }
    let mut end = max_bytes;
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &value[..end])
}

fn localized_category(category: &str, locale: &str) -> String {
    if locale != "en" {
        return category.to_string();
    }
    match category {
        "运动" => "Exercise",
        "饮食" => "Diet",
        "运动营养" => "Sports nutrition",
        "肠道" => "Gut health",
        "脂肪酸" => "Fatty acids",
        "维生素" => "Vitamins",
        "矿物质" => "Minerals",
        "线粒体" => "Mitochondria",
        "NAD 相关" => "NAD-related",
        "细胞稳态" => "Cellular homeostasis",
        "抗氧化" => "Antioxidants",
        "代谢" => "Metabolic health",
        "前沿生物技术" => "Frontier biotechnology",
        _ => category,
    }
    .to_string()
}

fn load_supplements(root: &Path, locale: &str) -> Vec<Supplement> {
    let strategies_path = root.join("catalog").join("strategies.csv");
    let catalog_path = if strategies_path.is_file() {
        strategies_path
    } else {
        root.join("catalog").join("supplements.csv")
    };
    let Ok(catalog) = fs::read_to_string(catalog_path) else {
        return Vec::new();
    };

    let preferred_order = [
        "strength-training",
        "aerobic-exercise",
        "healthy-diet",
        "quality-protein",
        "creatine",
        "soluble-fiber",
        "omega3",
        "vitamin-d3",
        "magnesium",
        "vitamin-c",
        "coq10",
        "nmn",
        "spermidine",
        "ergothioneine",
        "pqq",
        "ca-akg",
        "partial-reprogramming",
    ];
    let order: HashMap<&str, usize> = preferred_order
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, index))
        .collect();

    let mut supplements = Vec::new();
    for line in catalog.lines().skip(1) {
        let fields = split_csv_line(line);
        if fields.len() < 9 {
            continue;
        }
        let id = fields[0].clone();
        if !order.contains_key(id.as_str()) {
            continue;
        }

        let dossier_path =
            localized_note_path(&root.join("dossiers").join(format!("{id}.md")), locale);
        let dossier_summary = fs::read_to_string(&dossier_path)
            .map(|content| extract_summary(&content))
            .unwrap_or_default();
        let summary = if dossier_summary.is_empty() {
            fields[8].clone()
        } else {
            dossier_summary
        };

        supplements.push(Supplement {
            id,
            name_zh: fields[1].clone(),
            name_en: fields[2].clone(),
            category: localized_category(&fields[3], locale),
            tier: if fields[6].is_empty() {
                "pending".to_string()
            } else {
                fields[6].clone()
            },
            summary,
            file_path: dossier_path
                .exists()
                .then_some(relative_note_path(root, &dossier_path)),
        });
    }

    supplements.sort_by_key(|supplement| {
        order
            .get(supplement.id.as_str())
            .copied()
            .unwrap_or(usize::MAX)
    });
    supplements
}

fn load_people(root: &Path, locale: &str) -> Vec<Person> {
    let specs = [
        (
            "bryan-johnson",
            "Bryan Johnson",
            "布莱恩·约翰逊",
            "bryan-johnson-daily.md",
            "#dce8fb",
        ),
        (
            "peter-attia",
            "Peter Attia",
            "彼得·阿提亚",
            "peter-attia-protocol.md",
            "#e1eee8",
        ),
        (
            "andrew-huberman",
            "Andrew Huberman",
            "安德鲁·休伯曼",
            "andrew-huberman-protocol.md",
            "#eee8da",
        ),
        (
            "chuando-tan",
            "Chuando Tan",
            "陈传多",
            "chuando-tan.md",
            "#eadff1",
        ),
        (
            "edson-brandao",
            "Edson Brandão",
            "埃德森·布兰当",
            "edson-brandao.md",
            "#e4e9f3",
        ),
        (
            "leslie-kenny",
            "Leslie Kenny",
            "莱士里·肯尼",
            "leslie-kenny.md",
            "#dcebec",
        ),
    ];

    specs
        .iter()
        .filter_map(|(id, name, name_zh, filename, accent)| {
            let path = localized_note_path(&root.join("cases").join(filename), locale);
            if !path.exists() {
                return None;
            }
            let summary = fs::read_to_string(&path)
                .map(|content| extract_summary(&content))
                .unwrap_or_default();
            Some(Person {
                id: (*id).to_string(),
                name: (*name).to_string(),
                name_zh: Some((*name_zh).to_string()),
                summary: if summary.is_empty() {
                    "Public protocol and longitudinal case notes.".to_string()
                } else {
                    summary
                },
                file_path: Some(relative_note_path(root, &path)),
                accent: (*accent).to_string(),
            })
        })
        .collect()
}

fn load_stories(root: &Path, locale: &str) -> Vec<Story> {
    let stories_root = root.join("stories");
    let Ok(entries) = fs::read_dir(&stories_root) else {
        return Vec::new();
    };

    let mut paths = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path.extension().is_some_and(|extension| extension == "md")
                && !is_paired_english_companion(path)
        })
        .collect::<Vec<_>>();
    paths.sort();

    let accents = ["#dcefe8", "#e6edf8", "#f1e8dc", "#e8f0df", "#eee5f2"];
    let mut stories = paths
        .into_iter()
        .enumerate()
        .filter_map(|(index, base_path)| {
            let path = localized_note_path(&base_path, locale);
            let metadata = fs::metadata(&path).ok()?;
            if metadata.len() as usize > MAX_NOTE_BYTES {
                return None;
            }
            let markdown = fs::read_to_string(&path).ok()?;
            let file_name = path.file_name()?.to_string_lossy().to_string();
            let file_stem = path.file_stem()?.to_string_lossy().to_string();
            let title = extract_frontmatter_value(&markdown, "title")
                .or_else(|| extract_markdown_title(&markdown))
                .unwrap_or_else(|| file_stem.replace(['-', '_'], " "));
            let summary = extract_frontmatter_value(&markdown, "summary")
                .unwrap_or_else(|| extract_summary(&markdown));

            Some(Story {
                id: extract_frontmatter_value(&markdown, "id").unwrap_or(file_stem),
                title,
                title_en: extract_frontmatter_value(&markdown, "title_en"),
                summary: if summary.is_empty() {
                    "一则来自本地资料库的长寿观察。".to_string()
                } else {
                    summary
                },
                summary_en: extract_frontmatter_value(&markdown, "summary_en"),
                file_path: Some(format!("stories/{file_name}")),
                accent: accents[index % accents.len()].to_string(),
            })
        })
        .collect::<Vec<_>>();
    stories.sort_by(|left, right| left.title.cmp(&right.title));
    stories
}

#[tauri::command]
fn load_library(root: Option<String>, locale: Option<String>) -> Result<LibrarySnapshot, String> {
    let managed_root = default_knowledge_root();
    let locale = locale
        .filter(|value| value == "en")
        .unwrap_or_else(|| "zh".to_string());
    let root = root
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| managed_root.clone());
    if root == managed_root {
        ensure_starter_library(&root)?;
    }
    let connected = root.is_dir();
    let supplements = if connected {
        load_supplements(&root, &locale)
    } else {
        Vec::new()
    };
    let people = if connected {
        load_people(&root, &locale)
    } else {
        Vec::new()
    };
    let stories = if connected {
        load_stories(&root, &locale)
    } else {
        Vec::new()
    };
    let note_count = if connected {
        count_markdown_files(&root)
    } else {
        0
    };

    Ok(LibrarySnapshot {
        root: path_string(&root),
        connected,
        supplements,
        people,
        stories,
        note_count,
    })
}

fn safe_existing_path(root: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("Knowledge directory is unavailable: {error}"))?;
    let candidate = canonical_root.join(relative_path);
    let canonical_candidate = candidate
        .canonicalize()
        .map_err(|error| format!("Note is unavailable: {error}"))?;
    if !canonical_candidate.starts_with(&canonical_root) {
        return Err("Refusing to read outside the selected knowledge directory".to_string());
    }
    Ok(canonical_candidate)
}

#[tauri::command]
fn read_note(root: String, relative_path: String) -> Result<String, String> {
    let path = safe_existing_path(Path::new(&root), &relative_path)?;
    let metadata = fs::metadata(&path).map_err(|error| error.to_string())?;
    if metadata.len() as usize > MAX_NOTE_BYTES {
        return Err("This note is too large to render safely".to_string());
    }
    fs::read_to_string(path).map_err(|error| format!("Could not read note: {error}"))
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && !slug.is_empty() {
            slug.push('-');
            last_was_separator = true;
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "capture".to_string()
    } else {
        slug.to_string()
    }
}

fn yaml_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

#[tauri::command]
fn save_capture(request: CaptureRequest) -> Result<String, String> {
    let root = PathBuf::from(&request.knowledge_root);
    if !root.is_dir() {
        return Err("Choose a valid knowledge directory before saving".to_string());
    }

    let inbox = root.join("inbox");
    fs::create_dir_all(&inbox).map_err(|error| format!("Could not create inbox: {error}"))?;
    let date = Local::now().format("%Y-%m-%d").to_string();
    let base_name = format!("{date}-{}", slugify(&request.title));
    let mut path = inbox.join(format!("{base_name}.md"));
    let mut suffix = 2;
    while path.exists() {
        path = inbox.join(format!("{base_name}-{suffix}.md"));
        suffix += 1;
    }

    let source_line = request
        .source_url
        .as_ref()
        .map(|url| format!("source_url: {}\n", yaml_string(url)))
        .unwrap_or_default();
    let markdown = format!(
        "---\ntitle: {}\ncaptured_at: {}\nlocale: {}\n{}status: inbox\n---\n\n# {}\n\n{}\n",
        yaml_string(&request.title),
        Local::now().to_rfc3339(),
        request.locale,
        source_line,
        request.title,
        request.content.trim()
    );

    fs::write(&path, markdown).map_err(|error| format!("Could not save capture: {error}"))?;
    Ok(path_string(&path))
}

fn collect_markdown_paths(root: &Path, locale: &str) -> Vec<PathBuf> {
    fn visit(path: &Path, paths: &mut Vec<PathBuf>) {
        if paths.len() >= 1_200 {
            return;
        }
        let Ok(entries) = fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit(&path, paths);
            } else if path.extension().is_some_and(|extension| extension == "md")
                && !is_paired_english_companion(&path)
            {
                paths.push(path);
            }
        }
    }

    let mut paths = Vec::new();
    visit(root, &mut paths);
    paths
        .into_iter()
        .map(|path| localized_note_path(&path, locale))
        .collect()
}

fn query_terms(question: &str) -> Vec<String> {
    let normalized = question.to_lowercase();
    let mut terms = HashSet::new();

    for token in normalized.split(|character: char| !character.is_alphanumeric()) {
        let token = token.trim();
        if token.chars().count() >= 2 {
            terms.insert(token.to_string());
            let characters: Vec<char> = token.chars().collect();
            if characters.iter().any(|character| !character.is_ascii()) {
                for pair in characters.windows(2) {
                    terms.insert(pair.iter().collect());
                }
            }
        }
    }
    terms.into_iter().collect()
}

fn retrieve_context(
    root: &Path,
    question: &str,
    selected_paths: &[String],
    locale: &str,
) -> String {
    let canonical_root = match root.canonicalize() {
        Ok(path) => path,
        Err(_) => return String::new(),
    };
    let mut selected = HashSet::new();
    let mut sections: Vec<(String, String)> = Vec::new();

    let personal_paths = [
        "profile/about-me.md",
        "plans/current-protocol.md",
        "records/lab-results.md",
        "records/diet-log.md",
        "records/training-log.md",
    ];
    for relative in personal_paths
        .iter()
        .map(|value| value.to_string())
        .chain(selected_paths.iter().cloned())
    {
        let base_path = canonical_root.join(&relative);
        let localized_path = localized_note_path(&base_path, locale);
        let localized_relative = relative_note_path(&canonical_root, &localized_path);
        if selected.contains(&localized_relative) {
            continue;
        }
        if let Ok(path) = safe_existing_path(&canonical_root, &localized_relative) {
            if let Ok(content) = fs::read_to_string(path) {
                selected.insert(localized_relative.clone());
                sections.push((localized_relative, truncate_utf8(&content, 9_000)));
            }
        }
    }

    let terms = query_terms(question);
    let mut scored: Vec<(usize, String, String)> = Vec::new();
    for path in collect_markdown_paths(&canonical_root, locale) {
        let relative = path
            .strip_prefix(&canonical_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if selected.contains(&relative) {
            continue;
        }
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let haystack = format!("{}\n{}", relative.to_lowercase(), content.to_lowercase());
        let score = terms
            .iter()
            .map(|term| haystack.matches(term).count().min(8))
            .sum::<usize>();
        if score > 0 {
            scored.push((score, relative, truncate_utf8(&content, 8_500)));
        }
    }
    scored.sort_by_key(|(score, _, _)| Reverse(*score));
    sections.extend(
        scored
            .into_iter()
            .take(5)
            .map(|(_, relative, content)| (relative, content)),
    );

    let mut context = String::new();
    for (relative, content) in sections {
        let section = format!("\n\n--- LOCAL NOTE: {relative} ---\n{content}");
        if context.len() + section.len() > MAX_CONTEXT_BYTES {
            break;
        }
        context.push_str(&section);
    }
    context
}

fn chat_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/chat/completions")
    }
}

#[tauri::command]
async fn chat_completion(request: ChatRequest) -> Result<String, String> {
    if request.api_key.trim().is_empty() {
        return Err("An API key is required".to_string());
    }
    if request.base_url.trim().is_empty() || request.model.trim().is_empty() {
        return Err("API URL and model are required".to_string());
    }

    let knowledge_root = PathBuf::from(&request.knowledge_root);
    let context = retrieve_context(
        &knowledge_root,
        &request.question,
        &request.context_paths,
        &request.locale,
    );
    let language_rule = if request.locale == "en" {
        "Reply in English."
    } else {
        "使用简体中文回答。"
    };
    let system_prompt = format!(
        "You are Open Longevity, a local-first scientific longevity assistant. \
         The user's local notes are your primary memory. Use the supplied notes before general knowledge. \
         Cite the local note path in parentheses when a statement depends on it. \
         Clearly separate the user's personal protocol from general information. \
         Never invent a study, measurement, dose, or source. Preserve concise safety boundaries for \
         medication interactions, allergies, pregnancy, and organ impairment when relevant. \
         Do not diagnose or prescribe. {language_rule}"
    );

    let mut messages = vec![json!({ "role": "system", "content": system_prompt })];
    for line in request
        .history
        .into_iter()
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
    let grounded_question = if context.is_empty() {
        request.question
    } else {
        format!(
            "{}\n\nUse the following local context. Do not claim it is exhaustive:{}",
            request.question, context
        )
    };
    messages.push(json!({ "role": "user", "content": grounded_question }));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
        .map_err(|error| format!("Could not create model client: {error}"))?;
    let response = client
        .post(chat_endpoint(&request.base_url))
        .bearer_auth(request.api_key)
        .header("HTTP-Referer", "https://openlongevity.science")
        .header("X-Title", "Open Longevity")
        .json(&json!({
            "model": request.model,
            "messages": messages
        }))
        .send()
        .await
        .map_err(|error| format!("Could not reach the model provider: {error}"))?;

    let status = response.status();
    let payload: Value = response
        .json()
        .await
        .map_err(|error| format!("Provider returned invalid JSON: {error}"))?;
    if !status.is_success() {
        let message = payload
            .pointer("/error/message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown provider error");
        return Err(format!("Provider error {status}: {message}"));
    }

    payload
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| "The provider response did not contain assistant text".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_library,
            read_note,
            save_capture,
            chat_completion
        ])
        .run(tauri::generate_context!())
        .expect("error while running Open Longevity");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_parser_preserves_quoted_commas() {
        let fields = split_csv_line(r#"one,"two, still two",three"#);
        assert_eq!(fields, vec!["one", "two, still two", "three"]);
    }

    #[test]
    fn slug_is_safe_for_capture_filenames() {
        assert_eq!(slugify("Vitamin D / 2026 Update"), "vitamin-d-2026-update");
        assert_eq!(slugify("维生素 D 更新"), "d");
        assert_eq!(slugify("纯中文标题"), "capture");
    }

    #[test]
    fn utf8_truncation_stays_on_character_boundaries() {
        assert_eq!(truncate_utf8("科学长寿", 7), "科学…");
    }

    #[test]
    fn starter_library_is_self_contained() {
        let unique = format!(
            "openlongevity-starter-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after Unix epoch")
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);

        ensure_starter_library(&root).expect("starter library should initialize");
        assert!(root.join("catalog/strategies.csv").is_file());
        assert!(root.join("dossiers/strength-training.md").is_file());
        assert!(root.join("dossiers/healthy-diet.md").is_file());
        assert!(root.join("stories/okinawa-longevity.md").is_file());
        assert_eq!(
            fs::read_to_string(root.join(".starter-pack-initialized"))
                .expect("starter version should be readable"),
            STARTER_PACK_VERSION
        );
        assert!(root.join("profile/about-me.md").is_file());
        let readme = fs::read_to_string(root.join("README.md")).expect("README should be readable");
        assert!(!readme.contains(r"C:\Life extension"));
        let logical_starter_count = STARTER_FILES
            .iter()
            .filter(|(relative_path, _)| {
                relative_path.ends_with(".md") && !relative_path.ends_with(".en.md")
            })
            .count();
        assert_eq!(count_markdown_files(&root), logical_starter_count);
        for (relative_path, _) in STARTER_FILES.iter().filter(|(relative_path, _)| {
            relative_path.ends_with(".md") && !relative_path.ends_with(".en.md")
        }) {
            let companion = relative_path
                .strip_suffix(".md")
                .map(|path| format!("{path}.en.md"))
                .expect("starter Markdown path should have an extension");
            assert!(
                STARTER_FILES
                    .iter()
                    .any(|(candidate, _)| *candidate == companion.as_str()),
                "missing English starter companion for {relative_path}"
            );
            assert!(
                root.join(&companion).is_file(),
                "English starter companion should be installed: {companion}"
            );
        }
        let stories = load_stories(&root, "zh");
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].title, "日本冲绳的长寿文化");
        let supplements = load_supplements(&root, "zh");
        let nad = supplements
            .iter()
            .find(|supplement| supplement.id == "nmn")
            .expect("NAD+ starter entry should exist");
        assert_eq!(nad.name_zh, "NAD+");
        assert!(fs::read_to_string(root.join("dossiers/nmn.md"))
            .expect("NAD+ dossier should be readable")
            .contains("# NAD+"));
        let reprogramming = supplements
            .iter()
            .find(|supplement| supplement.id == "partial-reprogramming")
            .expect("partial reprogramming starter entry should exist");
        assert_eq!(reprogramming.tier, "T5");
        assert!(root.join("dossiers/partial-reprogramming.md").is_file());
        assert!(root.join("papers/existing-evidence-library.md").is_file());
        assert!(root.join("products/index.md").is_file());
        let people = load_people(&root, "zh");
        assert_eq!(people.len(), 6);
        assert!(people.iter().all(|person| person.id != "ray-lui"));
        assert!(!root.join("cases/ray-lui.md").exists());
        let english_supplements = load_supplements(&root, "en");
        assert!(english_supplements
            .iter()
            .find(|supplement| supplement.id == "strength-training")
            .and_then(|supplement| supplement.file_path.as_deref())
            .is_some_and(|path| path == "dossiers/strength-training.en.md"));
        let english_people = load_people(&root, "en");
        assert!(english_people
            .iter()
            .find(|person| person.id == "bryan-johnson")
            .and_then(|person| person.file_path.as_deref())
            .is_some_and(|path| path == "cases/bryan-johnson-daily.en.md"));
        let english_stories = load_stories(&root, "en");
        assert_eq!(english_stories.len(), 1);
        assert_eq!(
            english_stories[0].file_path.as_deref(),
            Some("stories/okinawa-longevity.en.md")
        );
        let english_paths = collect_markdown_paths(&root, "en");
        assert_eq!(english_paths.len(), logical_starter_count);
        assert!(english_paths
            .iter()
            .any(|path| path.ends_with("dossiers/strength-training.en.md")));
        for (_, content) in STARTER_FILES {
            assert!(!content.contains("吕良伟"));
            assert!(!content.contains("肌钙蛋白 I"));
            assert!(!content.contains("桑葚汁 200"));
        }

        fs::write(
            root.join("index.md"),
            "# OpenLongevity 知识库\n\n用户保留的其他内容。",
        )
        .expect("legacy branded starter file should be writable");
        fs::write(root.join(".starter-pack-initialized"), "10")
            .expect("previous starter version should be writable");
        ensure_starter_library(&root).expect("starter branding should migrate");
        let migrated_index =
            fs::read_to_string(root.join("index.md")).expect("migrated index should be readable");
        assert!(migrated_index.contains("# Open Longevity 知识库"));
        assert!(migrated_index.contains("用户保留的其他内容。"));

        fs::write(root.join("cases/ray-lui.md"), "# retired starter case")
            .expect("retired case fixture should be writable");
        fs::write(root.join(".starter-pack-initialized"), "8")
            .expect("legacy starter version should be writable");
        ensure_starter_library(&root).expect("retired starter content should migrate");
        assert!(!root.join("cases/ray-lui.md").exists());

        fs::write(
            root.join("dossiers/nmn.md"),
            LEGACY_NMN_DOSSIER.replace("tier: T4", "tier: T3"),
        )
        .expect("legacy NAD dossier should be writable");
        fs::write(root.join(".starter-pack-initialized"), "6")
            .expect("legacy starter version should be writable");
        ensure_starter_library(&root).expect("legacy NAD dossier should migrate");
        let migrated_nad = fs::read_to_string(root.join("dossiers/nmn.md"))
            .expect("migrated NAD dossier should be readable");
        assert!(migrated_nad.contains("# NAD+"));
        assert!(migrated_nad.contains("NADH"));
        assert!(migrated_nad.contains("tier: T3"));

        fs::write(
            root.join("stories/my-observation.md"),
            "---\ntitle: 我的观察\n---\n\n# 我的观察\n\n这是一篇用户自行添加的长寿轶事文章。",
        )
        .expect("custom story should be writable");
        let stories = load_stories(&root, "zh");
        assert_eq!(stories.len(), 2);
        assert!(stories.iter().any(|story| story.title == "我的观察"));

        fs::remove_dir_all(root).expect("temporary starter library should be removable");
    }
}
