import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import test from "node:test";

test("builds a portable root-hosted site", async () => {
  const html = await readFile(
    new URL("../dist/index.html", import.meta.url),
    "utf8",
  );

  assert.match(html, /<title>Open Longevity/);
  assert.match(html, /src="\/assets\//);
  assert.match(html, /href="\/open-longevity-logo\.png/);
  assert.doesNotMatch(html, /\/src\/main\.tsx/);
  assert.doesNotMatch(html, /%BASE_URL%/);
  await access(new URL("../dist/product-ui/home-en.png", import.meta.url));
  await access(new URL("../dist/product-ui/settings-zh.png", import.meta.url));
  await access(new URL("../dist/install.txt", import.meta.url));
  await access(
    new URL("../dist/fonts/Newsreader-Variable-Latin.woff2", import.meta.url),
  );

  const assetsDirectory = new URL("../dist/assets/", import.meta.url);
  const { readdir } = await import("node:fs/promises");
  const scripts = (await readdir(assetsDirectory)).filter((file) =>
    file.endsWith(".js"),
  );
  const javascript = (
    await Promise.all(
      scripts.map((file) =>
        readFile(new URL(file, assetsDirectory), "utf8"),
      ),
    )
  ).join("\n");

  assert.match(javascript, /github\.com\/edison7009\/OpenLongevity/);
  assert.match(javascript, /\/releases\/latest/);
  assert.match(javascript, /install\.txt/);
});
