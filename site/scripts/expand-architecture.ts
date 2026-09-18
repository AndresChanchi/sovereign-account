#!/usr/bin/env bun
import { readdir, readFile, writeFile, stat } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const SITE_ROOT = join(__dirname, '..');
const REPO_ROOT = join(SITE_ROOT, '..');

const MERMAID_DIR = join(REPO_ROOT, 'architecture', 'mermaid');

const TARGETS = [
  join(REPO_ROOT, 'domain', 'account', 'ubiquitous-language', 'index.md'),
  join(REPO_ROOT, 'domain', 'account', 'formal-laws', 'index.md'),
];

async function exists(p: string): Promise<boolean> {
  try {
    await stat(p);
    return true;
  } catch {
    return false;
  }
}

async function loadMermaid(diagram: string): Promise<string | null> {
  const file = join(MERMAID_DIR, `structurizr-${diagram}.mmd`);
  if (!(await exists(file))) return null;
  return (await readFile(file, 'utf-8')).trimEnd();
}

function buildBlock(name: string, content: string): string {
  return ['```mermaid', `%% kipio-diagram: ${name}`, content, '```'].join('\n');
}

async function expandFile(file: string): Promise<{ changed: number; missing: string[] }> {
  const raw = await readFile(file, 'utf-8');
  const missing: string[] = [];
  let changed = 0;

  const pattern =
    /<!--\s*architecture:start\s+(\S+)\s*-->([\s\S]*?)<!--\s*architecture:end\s*-->/g;

  const replacements: Array<{ from: string; to: string }> = [];

  let match: RegExpExecArray | null;
  while ((match = pattern.exec(raw)) !== null) {
    const name = match[1];
    const fullMatch = match[0];
    const body = await loadMermaid(name);

    if (!body) {
      missing.push(name);
      continue;
    }

    const replacement =
      `<!-- architecture:start ${name} -->\n\n` +
      buildBlock(name, body) +
      `\n\n<!-- architecture:end -->`;

    replacements.push({ from: fullMatch, to: replacement });
  }

  let out = raw;
  for (const r of replacements) {
    out = out.replace(r.from, r.to);
    changed += 1;
  }

  if (changed > 0) {
    await writeFile(file, out, 'utf-8');
  }

  return { changed, missing };
}

async function main() {
  let total = 0;
  const allMissing: string[] = [];

  for (const file of TARGETS) {
    if (!(await exists(file))) continue;
    const { changed, missing } = await expandFile(file);
    total += changed;
    allMissing.push(...missing);
    console.log(
      `[expand-architecture] ${changed} block(s) in ${file.replace(REPO_ROOT + '/', '')}`,
    );
  }

  if (allMissing.length > 0) {
    console.warn(`[expand-architecture] Missing .mmd for: ${allMissing.join(', ')}`);
  }

  console.log(`[expand-architecture] ${total} block(s) expanded total`);
}

main().catch((err) => {
  console.error(`[expand-architecture] ${err.message ?? err}`);
  process.exit(1);
});
