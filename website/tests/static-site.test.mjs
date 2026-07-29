import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import test from "node:test";

test("builds a portable GitHub Pages site", async () => {
  const html = await readFile(
    new URL("../dist/index.html", import.meta.url),
    "utf8",
  );

  assert.match(html, /<title>Open Longevity/);
  assert.match(html, /OpenLongevity\/assets\//);
  assert.match(html, /OpenLongevity\/open-longevity-logo\.png/);
  await access(new URL("../dist/product-ui/home-en.png", import.meta.url));
  await access(new URL("../dist/product-ui/settings-zh.png", import.meta.url));
});
