# Open Longevity website

Public website for [Open Longevity](https://github.com/edison7009/OpenLongevity),
an open-source, local-first longevity knowledge application powered by AI and
scientific evidence.

## Development

Requires Node.js 22 or newer.

```bash
npm install
npm run dev
```

## Verification

```bash
npm test
```

The production build is a static Vite site in `dist/`.

## Deployment

Changes under `website/` on the main repository's `main` branch are deployed
automatically by the root `.github/workflows/website-pages.yml` workflow.

Public URL:
[edison7009.github.io/OpenLongevity](https://edison7009.github.io/OpenLongevity/)
