import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import test from "node:test";

const website = new URL("../", import.meta.url);

test("website is directly deployable without a build step", async () => {
  const html = await readFile(new URL("index.html", website), "utf8");
  const javascript = await readFile(new URL("js/main.js", website), "utf8");
  const stylesheet = await readFile(new URL("css/style.css", website), "utf8");
  const version = JSON.parse(await readFile(new URL("version.json", website), "utf8"));

  assert.match(html, /<title>Open Longevity/);
  assert.match(html, /src="\.\/js\/main\.js"/);
  assert.match(html, /href="\.\/css\/style\.css"/);
  assert.doesNotMatch(html, /\/src\/main\.tsx|%BASE_URL%/);
  assert.doesNotMatch(javascript, /\bReact\b|createRoot|import\.meta/);
  assert.doesNotMatch(stylesheet, /@import\s+["']tailwindcss/);
  assert.match(stylesheet, /grid-template-columns:\s*minmax\(0, 1\.18fr\)\s+minmax\(0, 0\.82fr\)/);
  assert.match(stylesheet, /\.release-version\s*\{[^}]*font-size:\s*17px/s);
  assert.doesNotMatch(html, /[↗☆]/);
  assert.doesNotMatch(stylesheet, /\.platform-switch button::after/);
  assert.match(stylesheet, /\.platform-note code\s*\{[^}]*display:\s*inline/s);
  assert.match(html, /data-i18n="heroDetail"/);
  assert.match(javascript, /富豪花费百万美元享受科技带来的长寿/);
  assert.match(javascript, /heroDetail:/);
  assert.match(javascript, /macOS 首次需在「终端」/);
  assert.match(html, /xattr -cr '\/Applications\/Open Longevity\.app'/);
  assert.match(javascript, /navigator\.userAgentData\?\.platform/);
  assert.match(javascript, /note\.hidden = note\.dataset\.platformNote !== platform/);
  assert.match(version.version, /^\d+\.\d+\.\d+$/);
  assert.equal((html.match(/<main\b/g) ?? []).length, 1);
  assert.equal((html.match(/<h1\b/g) ?? []).length, 1);

  for (const path of [
    "_worker.js",
    "install.ps1",
    "install.sh",
    "open-longevity-logo.png",
    "tree-of-life-engraving.png",
    "product-ui/home-en.png",
    "product-ui/settings-zh.png",
    "fonts/Newsreader-Variable-Latin.woff2",
  ]) {
    await access(new URL(path, website));
  }
});

test("install and update routes use published release assets", async () => {
  const worker = await readFile(new URL("_worker.js", website), "utf8");
  const windows = await readFile(new URL("install.ps1", website), "utf8");
  const unix = await readFile(new URL("install.sh", website), "utf8");

  assert.match(worker, /releases\/latest/);
  assert.match(worker, /_Windows_x64-setup\.exe/);
  assert.match(worker, /_macOS_arm64\.dmg/);
  assert.match(worker, /_Linux_x64\.AppImage/);
  assert.match(windows, /openlongevity\.life\/version\.json\?platform=windows/);
  assert.match(unix, /openlongevity\.life/);
});

test("Cloudflare worker resolves versions, downloads, and static files", async () => {
  const originalFetch = globalThis.fetch;
  const release = {
    tag_name: "v1.2.3",
    assets: [
      {
        name: "Open.Longevity_1.2.3_Windows_x64-setup.exe",
        browser_download_url: "https://github.com/example/windows.exe",
      },
    ],
  };

  globalThis.fetch = async (input) => {
    const url = String(input);
    if (url.includes("api.github.com")) return Response.json(release);
    if (url.includes("windows.exe")) {
      return new Response("installer", {
        headers: { "Content-Length": "9" },
      });
    }
    throw new Error(`Unexpected fetch: ${url}`);
  };

  try {
    const worker = (await import(`../_worker.js?test=${Date.now()}`)).default;
    const environment = {
      ASSETS: {
        fetch: async () => new Response("static-file"),
      },
    };

    const versionResponse = await worker.fetch(
      new Request("https://openlongevity.life/version.json?platform=windows"),
      environment,
    );
    assert.deepEqual(await versionResponse.json(), { version: "1.2.3" });

    const downloadResponse = await worker.fetch(
      new Request("https://openlongevity.life/download/windows"),
      environment,
    );
    assert.equal(await downloadResponse.text(), "installer");
    assert.equal(
      downloadResponse.headers.get("Content-Disposition"),
      'attachment; filename="Open.Longevity_1.2.3_Windows_x64-setup.exe"',
    );

    const staticResponse = await worker.fetch(
      new Request("https://openlongevity.life/index.html"),
      environment,
    );
    assert.equal(await staticResponse.text(), "static-file");
  } finally {
    globalThis.fetch = originalFetch;
  }
});
