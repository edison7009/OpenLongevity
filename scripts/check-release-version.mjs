import { readFileSync } from 'node:fs';

const packageJson = JSON.parse(readFileSync(new URL('../package.json', import.meta.url), 'utf8'));
const tauriConfig = JSON.parse(
  readFileSync(new URL('../src-tauri/tauri.conf.json', import.meta.url), 'utf8'),
);
const cargoManifest = readFileSync(
  new URL('../src-tauri/Cargo.toml', import.meta.url),
  'utf8',
);
const cargoVersion = cargoManifest.match(
  /^\s*version\s*=\s*"([^"]+)"\s*$/m,
)?.[1];

const versions = {
  'package.json': packageJson.version,
  'src-tauri/tauri.conf.json': tauriConfig.version,
  'src-tauri/Cargo.toml': cargoVersion,
};
const uniqueVersions = new Set(Object.values(versions));

if (uniqueVersions.size !== 1 || uniqueVersions.has(undefined)) {
  console.error('Release versions do not match:', versions);
  process.exit(1);
}

const version = packageJson.version;
const tag = process.env.GITHUB_REF_NAME;
if (tag?.startsWith('v') && tag.slice(1) !== version) {
  console.error(`Release tag ${tag} does not match application version ${version}.`);
  process.exit(1);
}

console.log(`Release version ${version} is consistent.`);
