import { readFile, readdir } from 'node:fs/promises';
import { join } from 'node:path';
import { load } from 'js-yaml';

const ROOT = join(import.meta.dir, '..', 'src', 'content', 'docs');

// Files that are navigational, not content.
const SKIP = new Set(['README.md']);

async function* walk(dir: string): AsyncGenerator<string> {
  const entries = await readdir(dir, { withFileTypes: true });
  for (const entry of entries) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      yield* walk(full);
    } else if (entry.name.endsWith('.md')) {
      if (SKIP.has(entry.name)) continue;
      yield full;
    }
  }
}

function extractFrontmatter(content: string): string | null {
  const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  return match ? match[1] : null;
}

async function main() {
  let errors = 0;
  let checked = 0;

  for await (const file of walk(ROOT)) {
    checked++;
    const content = await readFile(file, 'utf-8');
    const fm = extractFrontmatter(content);

    if (!fm) {
      console.error(`✗ NO FRONTMATTER: ${file}`);
      errors++;
      continue;
    }

    try {
      load(fm);
    } catch (err) {
      console.error(`✗ BAD YAML: ${file}`);
      console.error(`  ${(err as Error).message.split('\n')[0]}`);
      errors++;
    }
  }

  console.log(`\n${checked} file(s) checked, ${errors} error(s)`);
  if (errors > 0) process.exit(1);
}

main();
