import { readdir, readFile, writeFile, mkdir, rm, stat } from 'node:fs/promises';
import { join, dirname, basename } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(__dirname, '..');
const CONTENT = join(PROJECT_ROOT, 'src', 'content', 'docs', 'domain', 'account');
const DIST = join(PROJECT_ROOT, 'dist');
const DATA_DEST = join(DIST, '_data');

interface Doc {
  id: string;
  slug: string;
  chapter: string;
  path: string;
  title: string;
  type: string;
  metadata: Record<string, unknown>;
}

function parseFrontmatter(raw: string): { data: Record<string, unknown>; body: string } {
  const match = raw.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!match) throw new Error('File has no frontmatter block');
  const data = Bun.YAML.parse(match[1]) as Record<string, unknown>;
  return { data, body: match[2] };
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
  await rm(DATA_DEST, { recursive: true, force: true });

  const chapters = ['ubiquitous-language', 'formal-laws'];
  const contentDocs: Doc[] = [];
  const indexDocs: Doc[] = [];

  for (const chapter of chapters) {
    const chapterDir = join(CONTENT, chapter);
    if (!(await exists(chapterDir))) {
      console.warn(`[generate-data] Missing chapter: ${chapter}`);
      continue;
    }

    const files = await readdir(chapterDir);

    for (const file of files) {
      if (!file.endsWith('.md')) continue;

      const full = join(chapterDir, file);
      const raw = await readFile(full, 'utf-8');
      const { data } = parseFrontmatter(raw);

      const slug = basename(file, '.md');
      const docPath = `${chapter}/${slug}`;
      const type = String(data.type ?? 'unknown');

      const jsonDir = join(DATA_DEST, chapter);
      await mkdir(jsonDir, { recursive: true });

      const doc: Doc = {
        id: String(data.id),
        slug,
        chapter,
        path: docPath,
        title: String(data.title),
        type,
        metadata: data,
      };

      await writeFile(
        join(jsonDir, `${slug}.json`),
        JSON.stringify(doc, null, 2) + '\n',
        'utf-8',
      );

      if (type === 'index') indexDocs.push(doc);
      else contentDocs.push(doc);
    }
  }

  contentDocs.sort((a, b) => a.path.localeCompare(b.path));
  indexDocs.sort((a, b) => a.path.localeCompare(b.path));

  await mkdir(DATA_DEST, { recursive: true });
  await writeFile(
    join(DATA_DEST, 'manifest.json'),
    JSON.stringify(
      {
        generated: new Date().toISOString(),
        content: contentDocs.length,
        indexes: indexDocs.length,
        total: contentDocs.length + indexDocs.length,
        docs: contentDocs,
        indexes_docs: indexDocs,
      },
      null,
      2,
    ) + '\n',
    'utf-8',
  );

  console.log(
    `[generate-data] ${contentDocs.length} content doc(s) + ${indexDocs.length} index doc(s) written`,
  );
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
