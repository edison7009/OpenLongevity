# Open Longevity agent notes

Before making product or design changes, read `CODEX_HANDOFF.md`. It is the
portable project memory for continuing development on another machine.

Project rules:

- Treat the desktop app and `website/` as separate products sharing one brand,
  while keeping both in this repository.
- The desktop app must use its own product library. Never bind it to
  `C:\Life extension` or any developer-specific directory.
- Preserve the bilingual starter library and run `npm run library:check` after
  changing it.
- Keep user data local by default. Do not persist API keys.
- Preserve the restrained teal/green visual language, large readable type, and
  low-chrome desktop UI. Avoid hover tooltips, unnecessary borders, redundant
  labels, and web-like decoration inside the desktop app.
- Do not commit generated dependencies or build output (`node_modules/`,
  `dist/`, `src-tauri/target/`).
- `website/` is deployed publicly from this repository with GitHub Pages.
  Preserve its static Vite build and the root Pages workflow when publishing it.
- Commit as `codex` (lowercase, the https://github.com/codex identity) in this
  repository; local `user.name` and `user.email` are already configured for it.
