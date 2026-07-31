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

The primary site is deployed by Cloudflare Pages. Use these Git integration
settings:

| Setting | Value |
| --- | --- |
| Production branch | `main` |
| Root directory | `website` |
| Build command | `npm run build` |
| Build output directory | `dist` |

The checked-in `wrangler.toml` also declares `./dist` as the Pages output
directory so the deployment target stays versioned with the website.

`website/` is the Vite project root and contains TypeScript source. It must not
be selected as the build output directory; only `website/dist/` contains the
browser-ready static site.

Primary URL:
[openlongevity.life](https://openlongevity.life/)

GitHub Pages mirror:
[edison7009.github.io/OpenLongevity](https://edison7009.github.io/OpenLongevity/)
