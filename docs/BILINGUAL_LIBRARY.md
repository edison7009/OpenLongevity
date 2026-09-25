# Bilingual starter library

Open Longevity keeps one logical knowledge library rather than two independent copies.

## File pairing

- The canonical Chinese starter article remains `name.md`.
- Its maintained English companion is `name.en.md`.
- Both files keep the same logical `id`.
- English companions include:

```yaml
locale: en
translation_of: path/to/name.md
```

The application selects `name.en.md` when the interface language is English. If the companion is
missing, it falls back to `name.md` instead of hiding the article.

## Original-language evidence

Standing user preference: translate the presentation for non-expert readers,
not the source evidence. This applies to papers and all other reference materials.
Keep reader-facing explanations short, conversational, and accurate; summarize
the practical relevance and main limitation without reproducing a technical paper.

Reader-facing strategies and guides remain bilingual. `papers/` contains evidence
records in the source language, without translated companions or translated paper
abstracts. Each record declares `content_type: evidence`, `source_type`,
`source_language`, and `last_checked`, and preserves its original title and source
URL (plus DOI/PMID when available). Trial registrations and company reports are
explicitly distinguished from peer-reviewed papers.

Evidence stays out of the main article list. Readers reach it through links in
strategies and guides; local AI retrieval can use the same records in either UI
language. The bilingual checker validates this narrowly scoped exception.

## User-authored notes

User-created Markdown is not duplicated automatically. A note without a companion remains
available in its original language under either interface language. This avoids machine
translation silently changing personal records.

## Translation rules

- Preserve URLs, citations, identifiers, tables, numbers, units, evidence grades, and safety
  boundaries.
- Do not strengthen causal language or add claims absent from the source.
- Keep product names, study names, and scientific terminology traceable to the original.
- Update both members of a maintained pair when starter content changes.

## Retrieval

AI retrieval prefers the companion matching the current interface language. User-authored notes
and selected page context remain eligible regardless of language.
