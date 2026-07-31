const REPOSITORY = "edison7009/OpenLongevity";

const platformMatchers = {
  windows: (name) => name.endsWith("_Windows_x64-setup.exe"),
  "windows-msi": (name) => name.endsWith("_Windows_x64.msi"),
  "macos-arm": (name) => name.endsWith("_macOS_arm64.dmg"),
  "macos-intel": (name) => name.endsWith("_macOS_x64.dmg"),
  "linux-deb": (name) => name.endsWith("_Linux_x64.deb"),
  "linux-rpm": (name) => name.endsWith("_Linux_x64.rpm"),
  "linux-appimage": (name) => name.endsWith("_Linux_x64.AppImage"),
};

async function latestAssets() {
  const response = await fetch(`https://api.github.com/repos/${REPOSITORY}/releases/latest`, {
    headers: {
      Accept: "application/vnd.github+json",
      "User-Agent": "Open-Longevity-Website",
    },
    cf: { cacheEverything: true, cacheTtl: 300 },
  });
  if (!response.ok) throw new Error(`GitHub API ${response.status}`);

  const release = await response.json();
  const assets = {};
  for (const [platform, matches] of Object.entries(platformMatchers)) {
    const asset = release.assets.find((candidate) => matches(candidate.name));
    if (asset) {
      assets[platform] = {
        name: asset.name,
        url: asset.browser_download_url,
        version: release.tag_name.replace(/^v/i, ""),
      };
    }
  }
  return assets;
}

export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    if (url.pathname === "/version.json") {
      try {
        const assets = await latestAssets();
        const platform = url.searchParams.get("platform");
        const version = platform ? assets[platform]?.version ?? "" : assets.windows?.version ?? "";
        return Response.json({ version }, { headers: { "Cache-Control": "no-store" } });
      } catch (error) {
        return Response.json({ version: "", error: error.message }, { status: 502, headers: { "Cache-Control": "no-store" } });
      }
    }

    const download = url.pathname.match(/^\/download\/([a-z0-9-]+)$/);
    if (download) {
      const platform = download[1];
      if (!platformMatchers[platform]) return new Response("Unknown platform", { status: 400 });
      try {
        const asset = (await latestAssets())[platform];
        if (!asset) return new Response("Installer is not available yet", { status: 404 });
        const response = await fetch(asset.url, { headers: { "User-Agent": "Open-Longevity-Website" } });
        if (!response.ok) return new Response("Unable to download installer", { status: 502 });
        return new Response(response.body, {
          headers: {
            "Content-Type": "application/octet-stream",
            "Content-Disposition": `attachment; filename="${asset.name}"`,
            "Content-Length": response.headers.get("Content-Length") ?? "",
            "Cache-Control": "no-store",
            "X-Open-Longevity-Version": asset.version,
          },
        });
      } catch (error) {
        return new Response(`Unable to resolve installer: ${error.message}`, { status: 502 });
      }
    }

    return env.ASSETS.fetch(request);
  },
};
