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
