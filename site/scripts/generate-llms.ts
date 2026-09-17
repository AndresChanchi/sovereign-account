import { readFile, writeFile, stat } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const DIST = join(__dirname, '..', 'dist');
const MANIFEST = join(DIST, '_data', 'manifest.json');

interface Doc {
  id: string;
  slug: string;
  chapter: string;
  path: string;
  title: string;
  type: string;
  metadata: Record<string, unknown>;
}

interface Manifest {
  generated: string;
  content: number;
  indexes: number;
  total: number;
  docs: Doc[];
  indexes_docs: Doc[];
}

async function exists(p: string): Promise<boolean> {
  try {
    await stat(p);
    return true;
  } catch {
    return false;
  }
}

async function main() {
  if (!(await exists(MANIFEST))) {
    console.warn('[generate-llms] manifest.json not found — skipping');
    return;
  }

  const manifest: Manifest = JSON.parse(await readFile(MANIFEST, 'utf-8'));
  const { docs, indexes_docs } = manifest;

  const ul = docs.filter((d) => d.chapter === 'ubiquitous-language');
  const laws = docs.filter((d) => d.chapter === 'formal-laws');

  let out = `# Sovereign Account\n`;
  out += `> A universal primitive for sovereign identity, authority, and execution.\n\n`;
  out += `Each entry points to the Markdown source. Structured metadata is available under \`_data/\`.\n\n`;

  if (ul.length) {
    out += `## Ubiquitous Language\n`;
    for (const d of ul) {
      out += `- [${d.title}](./${d.path}.md)\n`;
    }
    out += `\n`;
  }

  if (laws.length) {
    out += `## Formal Laws\n`;
    for (const d of laws) {
      out += `- [${d.title}](./${d.path}.md)\n`;
    }
    out += `\n`;
  }

  if (indexes_docs.length) {
    out += `## Indexes\n`;
    for (const d of indexes_docs) {
      out += `- [${d.title}](./${d.path}.md)\n`;
    }
    out += `\n`;
  }

  out += `## Machine-readable\n`;
  out += `- [Manifest](./_data/manifest.json)\n`;
  out += `- Per-document JSON: \`./_data/<chapter>/<slug>.json\`\n`;

  await writeFile(join(DIST, 'llms.txt'), out, 'utf-8');
  console.log(
    `[generate-llms] llms.txt written (${docs.length} content + ${indexes_docs.length} index)`,
  );
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
