# Open Longevity — portable Codex project memory

Updated: 2026-07-30

This file preserves the decisions and working context needed to continue the
project on another machine. It contains no API keys, private user parameters,
or temporary deployment credentials.

## Product identity

- Product name: **Open Longevity** (always include the space in visible text).
- Chinese product name: **科学长寿**.
- Version: **0.0.2**.
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

The default starter library currently contains **84 Chinese documents plus 84
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
  **由 AI 和科学来驱动，你的长寿计划**
- Left navigation label is **首页 / Home**.
- Left navigation is one fixed hierarchical tree: top-level categories expand
  to second-level notes. People and longevity anecdotes are expandable too.
- Avoid a visible sidebar scrollbar, but keep mouse-wheel scrolling.
- The left, center, and right panes are resizable.
- Top title/drag bar uses a restrained blue–green gradient. It should not repeat
  the logo, product name, or slogan.
- Windows/Linux use custom window controls; macOS uses native traffic-light
  controls on the left.
- Settings is a gear button near the window controls, not a permanent sidebar
  item.
- Remove redundant headers, helper labels, dark duplicate divider lines,
  “30 秒结论”, model IDs in the chat box, local-context labels, and knowledge
  context cards.
- Avoid hover tooltips and decorative hover motion throughout the app.
- Use generous, older-adult-friendly typography, especially in the center
  reading area.
- Tier-list items are plain large text sized to their content. Do not render
  them as bordered buttons, add arrows, or force equal widths.
- The right pane is for **My Plan** shortcuts (supplements, exercise, diet,
  etc.) and a switchable favorites list. Content such as Bryan Johnson can be
  favorited.
- AI settings support OpenAI-compatible providers and custom endpoints. Current
  visible defaults discussed for the UI are `gpt-5.5`,
  `deepseek-v4-pro`, and `kimi-k3`; Chinese “Custom” is **自定义** and examples
  should be visibly marked `e.g.`.
- API keys stay in memory and are not written to disk.
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
- Current hosted URL:
  `https://edison7009.github.io/OpenLongevity/`
- The website is a static Vite/React site deployed publicly through GitHub
  Pages from the main repository workflow.
- The desktop app and website are separate products in the same repository.
  The former `OpenLongevity-website` repository remains as a migration backup.
- Visual direction: dark teal, Renaissance scientific engraving, warm paper,
  copper/brass lines, and a full-bleed Tree of Life hero image. It should feel
  optimistic, healthy, literary, and scientific—not dense or
  trypophobia-inducing.
- Website hero:
  - Chinese: **让 AI 与科学，照亮你的生命之树**
  - English: **Let AI and science illuminate your Tree of Life**
- Website body positions Open Longevity as using Bryan Johnson's longevity plan
  as a starting blueprint, then adding AI and scientific evidence so ordinary
  people can access strategies once reserved for the wealthy.
- The scrolling T1–T5 strip is bilingual; strategy names are the same enlarged
  size as the T1–T5 labels.
- Open manifesto section:
  - **科学长寿，不应该是富豪专属。**
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
- Website: static Vite/React in `website/app/`; public assets in
  `website/public/`.

## Restore development dependencies on a new machine

Prerequisites:

- Node.js and npm
- Rust toolchain
- Tauri platform prerequisites for the operating system
- Website requires Node.js 22.13 or newer

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
npm install
npm run build
npm run dev
```

Recreating `node_modules/`, `dist/`, or `src-tauri/target/` is expected. These
are deliberately excluded from portable copies because they are generated and
machine-specific.

## Recommended continuation

1. Continue refining the desktop experience and verify pane resizing, native
   title-bar behavior, keyboard navigation, and readable scaling.
2. Add and test night mode.
3. Finish product-grade AI provider configuration and knowledge-grounded tool
   calls.
4. Review the bilingual starter library for scientific sourcing and product
   neutrality.
5. Test the published `v0.0.1` installers on real Windows, macOS, and Linux
   machines. Future production releases should add Windows and Apple code
   signing when certificates are available.
