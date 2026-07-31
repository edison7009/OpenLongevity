# Open Longevity — portable Codex project memory

Updated: 2026-08-01

This file preserves the decisions and working context needed to continue the
project on another machine. It contains no API keys, private user parameters,
or temporary deployment credentials.

## Product identity

- Product name: **Open Longevity** (always include the space in visible text).
- Chinese product name: **科学延寿**. Use **延寿** consistently in Chinese
  product, website, documentation, and starter-library copy; keep the previous
  product-domain term out of new copy.
- Version: **0.0.10**.
- Goal: a productized, local-first scientific-longevity desktop application for
  Windows, macOS, and Linux—not a personal wrapper around the developer's notes.
- Product origin: inspired by the developer's `C:\Life extension` notes, but the
  shipped application and its data library must be completely independent of
  that folder.
- Default library locations are documented in `README.md`; users may explicitly
  choose another directory later.
- Initial languages: Simplified Chinese and English.
- Starter knowledge is product-neutral and must not contain the developer's
  personal health parameters.

## Core product model

The application combines:

1. Knowledge reading and internal note links.
2. AI-assisted collection: users paste a URL or material into the conversation
   and ask AI to structure and save it; there is no separate quick-capture
   navigation item.
3. AI-assisted longevity planning grounded in the local knowledge library.

The default starter library currently contains **88 Chinese documents plus 88
English companion documents**. Run `npm run library:check` to verify the pairs.

Primary knowledge categories:

- Longevity strategies
- People / public cases
- Longevity stories and anecdotes

The application must open reference websites in the user's system browser.
Article keywords may link internally to other knowledge pages, and content pages
use a minimal icon-only back action beside the category label.

## Strategy priorities

The home page uses an editable T1–T5 priority map. The order is a starting
reference based on public protocols (including Bryan Johnson) and evidence
maturity; it is not a universal medical ranking. Users can reorder it or ask AI
to help.

Current default examples:

- T1: strength training, aerobic exercise, high-quality/healthy diet
- T2: creatine, soluble dietary fiber, Omega-3
- T3: vitamin D3, magnesium, vitamin C
- T4: CoQ10, NAD+, spermidine
- T5: ergothioneine, PQQ, Ca-AKG

Use **NAD+** as the visible umbrella term rather than “NMN / NR”.
The library also includes healthy diet, Yamanaka factors, and mouse longevity
gene-editing material. Do not restore the removed Lü Liangwei-specific content
to the product template.

## Desktop experience decisions

- Visible desktop slogan remains:
  **由 AI 和科学来驱动，你的延寿计划**
- Left navigation label is **首页 / Home**.
- Left navigation is one fixed hierarchical tree: top-level categories expand
  to second-level notes. People and longevity anecdotes are expandable too.
- Avoid a visible sidebar scrollbar, but keep mouse-wheel scrolling.
- The left, center, and right panes are resizable. While dragging, only the
  divider being manipulated is highlighted; the opposite divider stays idle.
- Top title/drag bar uses a restrained blue–green gradient. It should not repeat
  the logo, product name, or slogan.
- Windows/Linux use custom window controls; macOS uses native traffic-light
  controls on the left.
- The desktop app is single-instance. Launching it again must restore and focus
  the existing main window instead of opening another process/window.
- Settings is a gear button near the window controls, not a permanent sidebar
  item.
- Remove redundant headers, helper labels, dark duplicate divider lines,
  “30 秒结论”, model IDs in the chat box, local-context labels, and knowledge
  context cards.
- Keep the chat composer in a persistent bottom layout row. The content above
  it scrolls independently so the final message is never covered and the
  composer never disappears while scrolling.
- Keep the composer compact. Below it, show only a single-line reminder that
  public, local, and AI-generated content must be independently verified; do
  not place consent controls there or in the right rail.
- On startup, show a non-dismissible global open-source software use-boundary
  dialog until the user accepts it on seven consecutive distinct local calendar
  days. Same-day relaunches do not advance progress; missing a day or declining
  resets it, and declining closes the app. After day seven the dialog no longer
  appears. Persist progress locally under `openlongevity:disclaimer-progress:v2`.
  Frame this dialog around the MIT-licensed software boundary: code and
  information organization are provided as is, users select their models and
  sources, verify outputs, and decide how to use them. Do not characterize the
  project as a medical-liability actor or use injury/death language.
- Chat uses a minimal two-sided conversation layout: user messages are compact
  bubbles aligned to the right, while Open Longevity answers remain readable,
  unframed content aligned to the left. Do not show participant names or avatars;
  message position already communicates the speaker.
- Opening AI chat or switching conversations must position the message area at
  the bottom before paint, with no visible scroll animation. New output follows
  only while the user remains near the bottom; reading older messages must not
  pull the user back down.
- Provider reasoning/thinking content stays internal to the model session. Never
  emit it into the chat UI or persist it in UI messages; only the final answer
  is user-visible. When loading conversations, remove reasoning-detail markup
  written by versions affected by the v0.0.6 streaming bug.
- The chat composer shows context usage as a percentage with a themed
  hover/focus explanation. Near the 1,000,000-byte app context budget, older
  conversation is compacted locally while about 800 KB of recent messages
  remains verbatim, up to 150 KB of compacted history is retained, and the
  complete conversation stays on disk. The actual provider limit may be lower.
  AI chat exposes New chat actions in three places: the conversation header,
  beside AI chat in the fixed left navigation, and in the fixed right-rail
  header. On non-AI pages, the right-rail action returns to AI chat instead.
  Selecting a saved conversation is read-only and must not move it to the top of
  history; only new conversation content changes its recency. Each history item
  ends with its actual last-updated date and time.
- Avoid hover tooltips and decorative hover motion throughout the app.
- Use generous, older-adult-friendly typography, especially in the center
  reading area.
- Tier-list items are plain large text sized to their content. Do not render
  them as bordered buttons, add arrows, or force equal widths.
- The right pane stacks two persistent sections: **Favorites** on top and
  **My Plan** shortcuts (supplements, exercise, diet, daily routine, health log) below; when a note
  is open its sources appear as a third section. The old header star toggle is
  gone (favorites are always visible). On first launch the favorites are seeded
  once with Bryan Johnson (flag `openlongevity:favorites-seeded:v1`); a user's
  later edits are never overwritten.
- The **My Plan** rail has five sections: supplements (补剂计划), exercise (运动计划), diet (饮食计划), daily routine (作息计划), and health log (健康记录). Clicking a section opens its own **note page** — `plans/supplements.md`, `plans/exercise.md`, `plans/diet.md`, `plans/daily-routine.md` — rendered like any other library note (new page, back navigation); the health log opens the per-day editor page. The AI maintains the four plan pages via the `update_plan` tool (standard format: goals, current status, concrete arrangements, review notes).
- AI tools follow the "everything is a note" model: `save_note` (new notes in inbox/dossiers/cases/stories), `update_note` (edit any note by relative path, optionally writing `sources` into frontmatter), `update_plan` (the four plan pages), and `update_tier` (reassign an item's T1–T5 tier in `catalog/strategies.csv`, which drives the home strategy map; `pending` hides an item). The frontend reloads the library after every agent run so edits appear immediately.
- AI settings expose OpenAI and Anthropic wire protocols. Each protocol keeps
  its own API URL, model, and API key, and switching protocols must never
  overwrite the other protocol's values. New fields start empty; service URLs
  and model names appear only as visibly marked `e.g.` placeholders.
- AI provider settings, including API keys, persist as plaintext JSON in the
  current user's app-data directory (`OpenLongevity/config.json`). They must
  never be written into the repository or knowledge library.
- Modal headers share one standard component: a fixed 44 x 44 icon tile aligned
  to the vertical center of an `OPEN LONGEVITY` eyebrow plus main title block.
  Settings and capture dialogs use the same header and add a close action on the
  right; do not maintain separate modal-title alignment rules.
- Local knowledge retrieval uses an in-process Rust knowledge map: cached
  language-aware Markdown parsing, weighted title/path/heading/body matching,
  one-hop Markdown-link graph expansion, and relevant excerpt selection.
  Automatic grounding and the `search_library` tool share this retriever. It
  uses no embedding API, external service, vector database, or indexing tokens.
- Evidence-oriented questions automatically use an app-managed live research
  layer: the configured model produces a concise English biomedical query that
  is instructed to exclude personal identifiers and measurements,
  then the backend searches PubMed, ClinicalTrials.gov, and bioRxiv (through
  Europe PMC). Answers receive a deterministic source list and must distinguish
  peer-reviewed papers, trial registrations/results, and preprints.
- The app silently checks the latest GitHub release. When an update exists, a
  small teal update control appears beside the sidebar product name. Windows
  downloads the published NSIS installer with circular progress and launches
  it; unsupported platforms or failed installs fall back to the product
  website.
- A future night mode is desirable but not yet a release blocker.

## Website

- Website source: `website/`
- Primary hosted URL: `https://openlongevity.life/` (Cloudflare Pages).
- The website is build-free HTML/CSS/JavaScript. For Cloudflare Pages, leave
  the framework preset, build command, and root directory empty; set only the
  build output directory to `website`. GitHub Pages publishes that same folder
  from the main repository workflow as a mirror at
  `https://edison7009.github.io/OpenLongevity/`.
- `website/version.json` participates in `npm run release:check` and must match
  all desktop version fields. `website/_worker.js` exposes release-aware
  version and download routes, and the install scripts fall back to GitHub.
- The desktop app and website are separate products in the same repository.
  The former `OpenLongevity-website` repository remains as a migration backup.
- Visual direction: dark teal, Renaissance scientific engraving, warm paper,
  copper/brass lines, and a full-bleed Tree of Life hero image. It should feel
  optimistic, healthy, literary, and scientific—not dense or
  trypophobia-inducing.
- Keep the hero image clean. The six content sections below it use a restrained
  static analog-film treatment: one shared lightweight WebP grain texture,
  section-specific exposure direction, muted teal/copper light leaks, and soft
  edge fading. The overlays sit behind content, never animate, and must preserve
  text contrast on desktop and mobile.
- Website hero:
  - Chinese: **让 AI 与科学，照亮你的生命之树**
  - English: **Let AI and science illuminate your Tree of Life**
- Website hero lead is one unified subtitle paragraph at one visual level:
  **富豪花费百万美元借助科技延寿，而 Open Longevity 希望把生命之光同样带给普通家庭；以
  Bryan Johnson 公开的延寿计划为蓝本，融入 AI 与科学依据，让普通人也能拥有富豪级的延寿策略。**
  Do not render the Bryan Johnson sentence as a smaller note.
- The hero installer uses text-only platform tabs with no underline indicator.
  Its Chinese primary actions are **安装 Open Longevity** and **在 GitHub 上点星**;
  English keeps **Install Open Longevity** and **Star on GitHub**. Align the
  install label and release version by their text baselines.
  Its macOS fallback note is the compact inline command: `macOS 首次需在「终端」
  xattr -cr '/Applications/Open Longevity.app'`. Keep this installation area
  comfortably readable: 12px platform tabs, 13px desktop command text, and a
  14px macOS note; mobile may ellipsize the visible command while copying the
  complete value.
- The scrolling T1–T5 strip is bilingual; strategy names are the same enlarged
  size as the T1–T5 labels.
- Open manifesto section:
  - **科学延寿，不应该是富豪专属。**
  - Three principles: 开放知识 / 可验证证据 / 开源工具.
- Product-section title:
  - Chinese: **AI + 科学的时代**
  - English: **The age of AI + science**

## Architecture and important paths

- Desktop frontend: React + TypeScript + Vite under `src/`.
- Desktop shell/backend: Tauri 2 + Rust under `src-tauri/`.
- Product starter library: `starter-knowledge/`.
- Desktop visual assets: `public/`, `src-tauri/icons/`, and `设计思路/`.
- Architecture notes: `docs/ARCHITECTURE.md`.
- Bilingual-library rules: `docs/BILINGUAL_LIBRARY.md`.
- Cross-platform release workflow: `.github/workflows/release.yml`.
- Website: build-free static HTML/CSS/JavaScript under `website/`.

## Restore development dependencies on a new machine

Prerequisites:

- Node.js and npm
- Rust toolchain
- Tauri platform prerequisites for the operating system

Desktop:

```powershell
cd C:\OpenLongevity
npm install
npm run library:check
npm run typecheck
npm run tauri:dev
```

Website:

```powershell
cd C:\OpenLongevity\website
npx serve .
```

Recreating desktop `node_modules/`, `dist/`, or `src-tauri/target/` is expected.
These are deliberately excluded from portable copies because they are generated
and machine-specific. The website itself has no generated build directory.

## Recommended continuation

1. Continue refining the desktop experience and verify pane resizing, native
   title-bar behavior, keyboard navigation, and readable scaling.
2. Add and test night mode.
3. Finish product-grade AI provider configuration and knowledge-grounded tool
   calls.
4. Review the bilingual starter library for scientific sourcing and product
   neutrality.
5. Test the published `v0.0.10` installers on real Windows, macOS, and Linux
   machines. Future production releases should add Windows and Apple code
   signing when certificates are available.
