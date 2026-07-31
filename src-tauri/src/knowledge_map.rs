use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::UNIX_EPOCH;

const MAX_LIBRARY_FILES: usize = 1_200;
const MAX_SNIPPET_BYTES: usize = 4_800;
const MAX_PERSONAL_NOTE_BYTES: usize = 7_000;

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub score: usize,
    pub path: String,
    pub title: String,
    pub snippet: String,
    pub via_graph: bool,
}

#[derive(Debug)]
struct KnowledgeNote {
    path: String,
    title: String,
    headings: String,
    content: String,
    searchable_content: String,
    links: Vec<String>,
}

#[derive(Debug)]
struct KnowledgeIndex {
    notes: Vec<KnowledgeNote>,
    by_path: HashMap<String, usize>,
    incoming: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct CachedIndex {
    fingerprint: u64,
    index: Arc<KnowledgeIndex>,
}

static INDEX_CACHE: OnceLock<Mutex<HashMap<String, CachedIndex>>> = OnceLock::new();

pub fn search_library(root: &Path, query: &str, locale: &str, limit: usize) -> Vec<SearchHit> {
    let Some(index) = load_index(root, locale) else {
        return Vec::new();
    };
    search_index(&index, query, limit)
}

pub fn retrieve_context(
    root: &Path,
    question: &str,
    selected_paths: &[String],
    locale: &str,
    max_bytes: usize,
) -> String {
    let Some(index) = load_index(root, locale) else {
        return String::new();
    };

    let mut included = HashSet::new();
    let mut sections = Vec::new();
    let personal_paths = [
        "profile/about-me.md",
        "plans/current-protocol.md",
        "records/lab-results.md",
        "records/diet-log.md",
        "records/training-log.md",
    ];

    for requested in personal_paths
        .iter()
        .map(|path| path.to_string())
        .chain(selected_paths.iter().cloned())
    {
        let Some(note_index) = resolve_note_index(&index, &requested) else {
            continue;
        };
        if !included.insert(note_index) {
            continue;
        }
        let note = &index.notes[note_index];
        sections.push(format!(
            "\n\n--- PRIORITY LOCAL NOTE: {} ---\n{}",
            note.path,
            truncate_utf8(&note.content, MAX_PERSONAL_NOTE_BYTES)
        ));
    }

    for hit in search_index(&index, question, 7) {
        let Some(note_index) = resolve_note_index(&index, &hit.path) else {
            continue;
        };
        if !included.insert(note_index) {
            continue;
        }
        let retrieval = if hit.via_graph {
            "knowledge-link neighbor"
        } else {
            "hybrid text match"
        };
        sections.push(format!(
            "\n\n--- RETRIEVED LOCAL NOTE: {} ({retrieval}) ---\n{}",
            hit.path, hit.snippet
        ));
    }

    let mut context = String::from(
        "\n\nLOCAL KNOWLEDGE MAP\n\
         The excerpts below were selected locally from the user's Markdown library. \
         Prefer them over general knowledge when relevant, cite their paths, and do not \
         assume omitted notes are irrelevant.",
    );
    for section in sections {
        if context.len() + section.len() > max_bytes {
            break;
        }
        context.push_str(&section);
    }
    if context.lines().count() <= 4 {
        String::new()
    } else {
        context
    }
}

fn load_index(root: &Path, locale: &str) -> Option<Arc<KnowledgeIndex>> {
    let canonical_root = root.canonicalize().ok()?;
    let paths = collect_logical_markdown_paths(&canonical_root, locale);
    let fingerprint = library_fingerprint(&paths);
    let cache_key = format!("{}::{locale}", canonical_root.to_string_lossy());
    let cache = INDEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    if let Ok(cache_guard) = cache.lock() {
        if let Some(cached) = cache_guard.get(&cache_key) {
            if cached.fingerprint == fingerprint {
                return Some(cached.index.clone());
            }
        }
    }

    let index = Arc::new(build_index(&canonical_root, paths));
    if let Ok(mut cache_guard) = cache.lock() {
        cache_guard.insert(
            cache_key,
            CachedIndex {
                fingerprint,
                index: index.clone(),
            },
        );
    }
    Some(index)
}

fn collect_logical_markdown_paths(root: &Path, locale: &str) -> Vec<PathBuf> {
    fn visit(path: &Path, paths: &mut Vec<PathBuf>) {
        if paths.len() >= MAX_LIBRARY_FILES {
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
    paths.sort();
    paths
        .into_iter()
        .map(|path| localized_note_path(&path, locale))
        .collect()
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

fn english_companion_path(path: &Path) -> PathBuf {
    let stem = path
        .file_stem()
        .map(|value| value.to_string_lossy())
        .unwrap_or_default();
    path.with_file_name(format!("{stem}.en.md"))
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

fn library_fingerprint(paths: &[PathBuf]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for path in paths {
        path.hash(&mut hasher);
        if let Ok(metadata) = fs::metadata(path) {
            metadata.len().hash(&mut hasher);
            metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_nanos())
                .unwrap_or_default()
                .hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn build_index(root: &Path, paths: Vec<PathBuf>) -> KnowledgeIndex {
    let mut notes = Vec::new();
    for path in paths {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let relative = relative_path(root, &path);
        let title = extract_title(&content, &relative);
        let headings = content
            .lines()
            .filter_map(|line| line.trim_start().strip_prefix('#'))
            .map(|line| line.trim_start_matches('#').trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        let links = extract_markdown_links(&relative, &content);
        notes.push(KnowledgeNote {
            path: relative,
            title,
            headings: headings.to_lowercase(),
            searchable_content: content.to_lowercase(),
            content,
            links,
        });
    }

    let mut by_path = HashMap::new();
    for (index, note) in notes.iter().enumerate() {
        by_path.insert(note.path.clone(), index);
        if let Some(base_path) = note.path.strip_suffix(".en.md") {
            by_path.insert(format!("{base_path}.md"), index);
        }
    }

    let mut incoming = vec![Vec::new(); notes.len()];
    for (source_index, note) in notes.iter().enumerate() {
        for target in &note.links {
            if let Some(target_index) = by_path.get(target).copied() {
                incoming[target_index].push(source_index);
            }
        }
    }

    KnowledgeIndex {
        notes,
        by_path,
        incoming,
    }
}

fn search_index(index: &KnowledgeIndex, query: &str, limit: usize) -> Vec<SearchHit> {
    let terms = query_terms(query);
    if terms.is_empty() || limit == 0 {
        return Vec::new();
    }

    let mut base_scores = Vec::new();
    for (note_index, note) in index.notes.iter().enumerate() {
        let score = score_note(note, query, &terms);
        if score > 0 {
            base_scores.push((score, note_index));
        }
    }
    base_scores.sort_by_key(|(score, _)| Reverse(*score));

    let mut combined: HashMap<usize, (usize, bool)> = HashMap::new();
    for (score, note_index) in &base_scores {
        combined.insert(*note_index, (*score, false));
    }

    for (seed_score, seed_index) in base_scores.iter().take(4) {
        let graph_score = (*seed_score / 6).max(1);
        let outgoing = index.notes[*seed_index]
            .links
            .iter()
            .filter_map(|path| index.by_path.get(path).copied());
        let neighbors = outgoing.chain(index.incoming[*seed_index].iter().copied());
        for neighbor in neighbors {
            let entry = combined.entry(neighbor).or_insert((0, true));
            entry.0 = entry.0.saturating_add(graph_score);
            entry.1 = true;
        }
    }

    let mut ranked = combined
        .into_iter()
        .map(|(note_index, (score, via_graph))| (score, note_index, via_graph))
        .collect::<Vec<_>>();
    ranked.sort_by_key(|(score, _, _)| Reverse(*score));
    ranked.truncate(limit);

    ranked
        .into_iter()
        .map(|(score, note_index, via_graph)| {
            let note = &index.notes[note_index];
            SearchHit {
                score,
                path: note.path.clone(),
                title: note.title.clone(),
                snippet: relevant_snippet(&note.content, &terms, MAX_SNIPPET_BYTES),
                via_graph,
            }
        })
        .collect()
}

fn score_note(note: &KnowledgeNote, query: &str, terms: &[String]) -> usize {
    let path = note.path.to_lowercase();
    let title = note.title.to_lowercase();
    let normalized_query = query.trim().to_lowercase();
    let mut score = 0usize;

    if normalized_query.chars().count() >= 3 {
        if title.contains(&normalized_query) {
            score += 120;
        }
        if note.headings.contains(&normalized_query) {
            score += 70;
        }
        if note.searchable_content.contains(&normalized_query) {
            score += 45;
        }
    }

    for term in terms {
        let length_weight = term.chars().count().clamp(1, 4);
        score += occurrence_count(&path, term, 4) * 18 * length_weight;
        score += occurrence_count(&title, term, 4) * 20 * length_weight;
        score += occurrence_count(&note.headings, term, 5) * 9 * length_weight;
        score += occurrence_count(&note.searchable_content, term, 7) * 2 * length_weight;
    }
    score
}

fn query_terms(query: &str) -> Vec<String> {
    const STOP_WORDS: &[&str] = &[
        "about", "and", "are", "can", "could", "for", "from", "how", "should", "the", "this",
        "what", "with", "一下", "个人", "什么", "你的", "可以", "如何", "建议", "我的", "是否",
        "有个", "相关", "能否", "问题",
    ];

    let normalized = query.to_lowercase();
    let mut terms = HashSet::new();
    for token in normalized.split(|character: char| !character.is_alphanumeric()) {
        let token = token.trim();
        if token.chars().count() < 2 || STOP_WORDS.contains(&token) {
            continue;
        }
        if token.chars().any(|character| !character.is_ascii()) {
            let characters = token.chars().collect::<Vec<_>>();
            for width in [2usize, 3] {
                for window in characters.windows(width) {
                    let term = window.iter().collect::<String>();
                    if !STOP_WORDS.contains(&term.as_str()) {
                        terms.insert(term);
                    }
                }
            }
        } else {
            terms.insert(token.to_string());
        }
    }
    let mut terms = terms.into_iter().collect::<Vec<_>>();
    terms.sort();
    terms
}

fn occurrence_count(haystack: &str, needle: &str, cap: usize) -> usize {
    if needle.is_empty() {
        0
    } else {
        haystack.matches(needle).count().min(cap)
    }
}

fn relevant_snippet(content: &str, terms: &[String], max_bytes: usize) -> String {
    let mut blocks = content
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, block)| {
            let trimmed = block.trim();
            if trimmed.is_empty() || trimmed == "---" {
                return None;
            }
            let lower = trimmed.to_lowercase();
            let score = terms
                .iter()
                .map(|term| occurrence_count(&lower, term, 5))
                .sum::<usize>();
            Some((score, index, trimmed))
        })
        .collect::<Vec<_>>();

    blocks.sort_by_key(|(score, _, _)| Reverse(*score));
    let mut selected = blocks
        .iter()
        .filter(|(score, _, _)| *score > 0)
        .take(3)
        .cloned()
        .collect::<Vec<_>>();
    if selected.is_empty() {
        selected.extend(blocks.into_iter().take(2));
    }
    selected.sort_by_key(|(_, index, _)| *index);
    truncate_utf8(
        &selected
            .into_iter()
            .map(|(_, _, block)| block)
            .collect::<Vec<_>>()
            .join("\n\n"),
        max_bytes,
    )
}

fn extract_title(content: &str, path: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            if !title.trim().is_empty() {
                return title.trim().to_string();
            }
        }
        if let Some(title) = trimmed.strip_prefix("title:") {
            if !title.trim().is_empty() {
                return title.trim().trim_matches(['"', '\'']).to_string();
            }
        }
    }
    Path::new(path)
        .file_stem()
        .map(|stem| stem.to_string_lossy().replace(['-', '_'], " "))
        .unwrap_or_else(|| path.to_string())
}

fn extract_markdown_links(source_path: &str, content: &str) -> Vec<String> {
    let mut links = HashSet::new();
    let mut rest = content;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else {
            break;
        };
        if let Some(path) = normalize_link_target(source_path, &rest[..end]) {
            links.insert(path);
        }
        rest = &rest[end + 1..];
    }
    links.into_iter().collect()
}

fn normalize_link_target(source_path: &str, raw_target: &str) -> Option<String> {
    let target = raw_target
        .trim()
        .trim_matches(['<', '>'])
        .split(['#', '?'])
        .next()?
        .replace('\\', "/");
    if target.is_empty()
        || target.contains("://")
        || target.starts_with("mailto:")
        || !target.to_lowercase().ends_with(".md")
    {
        return None;
    }

    let base = Path::new(source_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let joined = base.join(target);
    let mut parts = Vec::new();
    for component in joined.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            Component::ParentDir => {
                parts.pop()?;
            }
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(parts.join("/"))
}

fn resolve_note_index(index: &KnowledgeIndex, requested: &str) -> Option<usize> {
    let normalized = requested
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_string();
    index.by_path.get(&normalized).copied().or_else(|| {
        index
            .by_path
            .get(normalized.trim_end_matches(".en.md"))
            .copied()
    })
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn truncate_utf8(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_string();
    }
    let mut end = max_bytes;
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}\n\n…[excerpt truncated]", &value[..end])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("openlongevity-knowledge-map-{unique}"));
        fs::create_dir_all(root.join("dossiers")).expect("fixture directory should be created");
        fs::create_dir_all(root.join("sources")).expect("fixture directory should be created");
        root
    }

    #[test]
    fn ranks_titles_and_expands_markdown_links() {
        let root = fixture_root();
        fs::write(
            root.join("dossiers/creatine.md"),
            "# 肌酸\n\n肌酸与力量训练证据。\n\n[来源](../sources/creatine-trials.md)",
        )
        .expect("fixture should be written");
        fs::write(
            root.join("sources/creatine-trials.md"),
            "# 肌酸试验\n\n随机试验与安全性来源。",
        )
        .expect("fixture should be written");
        fs::write(
            root.join("dossiers/unrelated.md"),
            "# 其他主题\n\n普通背景内容。肌酸只在尾注出现一次。",
        )
        .expect("fixture should be written");

        let hits = search_library(&root, "肌酸力量训练", "zh", 5);
        assert_eq!(
            hits.first().map(|hit| hit.path.as_str()),
            Some("dossiers/creatine.md")
        );
        assert!(hits
            .iter()
            .any(|hit| hit.path == "sources/creatine-trials.md" && hit.via_graph));

        fs::remove_dir_all(root).expect("fixture should be removed");
    }

    #[test]
    fn context_uses_relevant_excerpts_instead_of_full_notes() {
        let root = fixture_root();
        let filler = "无关开头。".repeat(2_000);
        fs::write(
            root.join("dossiers/vitamin-d.md"),
            format!("# 维生素 D\n\n{filler}\n\n## 自身免疫\n\n维生素 D 自身免疫相关证据摘要。"),
        )
        .expect("fixture should be written");

        let context = retrieve_context(&root, "维生素D与自身免疫", &[], "zh", 20_000);
        assert!(context.contains("自身免疫相关证据摘要"));
        assert!(context.len() < 10_000);

        fs::remove_dir_all(root).expect("fixture should be removed");
    }

    #[test]
    fn starter_library_returns_domain_relevant_notes() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../starter-knowledge");
        let hits = search_library(&root, "维生素 D 自身免疫 VITAL", "zh", 6);
        assert!(hits.iter().any(|hit| {
            hit.path == "dossiers/vitamin-d3.md"
                || hit.path == "papers/vitamin-d-autoimmune-2026-07-21.md"
        }));
        assert!(hits
            .iter()
            .all(|hit| hit.snippet.len() <= MAX_SNIPPET_BYTES + 32));
    }
}
