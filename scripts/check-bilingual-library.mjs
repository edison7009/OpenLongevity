import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const libraryRoot = resolve(fileURLToPath(new URL('../starter-knowledge', import.meta.url)));

function markdownFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) return markdownFiles(path);
    return entry.isFile() && entry.name.endsWith('.md') ? [path] : [];
  });
}

function relativePath(path) {
  return relative(libraryRoot, path).split(sep).join('/');
}

function fieldValue(markdown, field) {
  const match = markdown.match(new RegExp(`^${field}:\\s*(.+?)\\s*$`, 'm'));
  return match?.[1].trim().replace(/^['"]|['"]$/g, '');
}

function markdownLinkUrls(markdown) {
  return new Set(
    [...markdown.matchAll(/\]\((https?:\/\/[^)]+)\)/g)].map((match) => match[1]),
  );
}

function markdownBody(markdown) {
  return markdown.replace(/^\uFEFF?---[ \t]*\r?\n[\s\S]*?\r?\n---[ \t]*(?:\r?\n|$)/, '');
}

const allFiles = markdownFiles(libraryRoot);
const sourceFiles = allFiles.filter((path) => !path.endsWith('.en.md'));
const englishFiles = allFiles.filter((path) => path.endsWith('.en.md'));
const errors = [];

for (const source of sourceFiles) {
  const companion = source.replace(/\.md$/, '.en.md');
  if (!existsSync(companion)) {
    errors.push(`Missing English companion: ${relativePath(source)}`);
    continue;
  }

  const markdown = readFileSync(companion, 'utf8');
  const sourceMarkdown = readFileSync(source, 'utf8');
  if (fieldValue(markdown, 'locale') !== 'en') {
    errors.push(`Missing locale: en: ${relativePath(companion)}`);
  }
  if (fieldValue(markdown, 'translation_of') !== relativePath(source)) {
    errors.push(`Invalid translation_of: ${relativePath(companion)}`);
  }
  if (/\p{Script=Han}/u.test(markdownBody(markdown))) {
    errors.push(`Untranslated Han characters in body: ${relativePath(companion)}`);
  }
  for (const url of markdownLinkUrls(sourceMarkdown)) {
    if (!markdown.includes(url)) {
      errors.push(`Missing source URL in ${relativePath(companion)}: ${url}`);
    }
  }
}

for (const companion of englishFiles) {
  const source = companion.replace(/\.en\.md$/, '.md');
  if (!existsSync(source)) {
    errors.push(`English companion has no source: ${relativePath(companion)}`);
  }
}

if (errors.length) {
  console.error(errors.join('\n'));
  process.exit(1);
}

console.log(
  `Bilingual starter library is complete: ${sourceFiles.length} source files and ${englishFiles.length} English companions.`,
);
