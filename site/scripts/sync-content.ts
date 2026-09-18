import { rm, mkdir, readdir, readFile, writeFile, stat } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(__dirname, '..');
const REPO_ROOT = join(PROJECT_ROOT, '..');

const SOURCE = join(REPO_ROOT, 'domain', 'account');
const CONTENT_DEST = join(PROJECT_ROOT, 'src', 'content', 'docs', 'domain', 'account');
const PUBLIC_DEST = join(PROJECT_ROOT, 'public');

const CHAPTERS = ['ubiquitous-language', 'formal-laws'] as const;
type Chapter = (typeof CHAPTERS)[number];

async function exists(p: string): Promise<boolean> {
  try {
    await stat(p);
    return true;
  } catch {
    return false;
  }
}

function pageUrl(chapter: Chapter, slug: string): string {
  if (slug === 'index') return `/${chapter}/`;
  return `/${chapter}/${slug}/`;
}

function rewriteLinks(content: string, currentChapter: Chapter, currentSlug: string): string {
  const currentBase = pageUrl(currentChapter, currentSlug);

  // Pass 1: pure fragment links (#anchor) → absolute paths so <base> doesn't break them.
  let out = content.replace(/\]\(#([^)\s]+)\)/g, (_match, anchor: string) => {
    return `](${currentBase}#${anchor})`;
  });

  // Pass 2: .md links (with or without fragment) → absolute lowercase routes.
  out = out.replace(
    /\]\(([^)\s]+?\.md)(#[^)\s]*)?\)/g,
    (match, rawPath: string, hash?: string) => {
      const stripped = rawPath.replace(/\.md$/, '');
      const hashPart = hash ?? '';

      let target: string | null = null;

      if (stripped.startsWith('/')) {
        target = stripped.toLowerCase();
      } else {
        const crossMatch = stripped.match(
          /^(?:\.\.\/|\.\/)?(ubiquitous-language|formal-laws)\/(.+)$/i,
        );
        if (crossMatch) {
          const chapter = crossMatch[1].toLowerCase();
          const slug = crossMatch[2].toLowerCase();
          if (slug === 'index') {
            target = `/${chapter}`;
          } else {
            target = `/${chapter}/${slug}`;
          }
        } else if (!stripped.includes('/')) {
          if (stripped.toLowerCase() === 'index') {
            target = `/${currentChapter}`;
          } else {
            target = `/${currentChapter}/${stripped.toLowerCase()}`;
          }
        }
      }

      if (!target) return match;

      const finalUrl = target.endsWith('/') ? target : `${target}/`;
      return `](${finalUrl}${hashPart})`;
    },
  );

  return out;
}

interface ChapterFile {
  name: string;
  content: string;
}

async function readChapter(chapter: Chapter): Promise<ChapterFile[]> {
  const src = join(SOURCE, chapter);
  const entries = await readdir(src, { withFileTypes: true });
  const files: ChapterFile[] = [];

  for (const entry of entries) {
    if (!entry.isFile()) continue;
    if (!entry.name.endsWith('.md')) continue;

    const slug = entry.name.replace(/\.md$/, '').toLowerCase();
    const raw = await readFile(join(src, entry.name), 'utf-8');
    files.push({
      name: entry.name.toLowerCase(),
      content: rewriteLinks(raw, chapter, slug),
    });
  }

  return files;
}

async function writeChapter(files: ChapterFile[], destDir: string) {
  await mkdir(destDir, { recursive: true });
  for (const file of files) {
    await writeFile(join(destDir, file.name), file.content, 'utf-8');
  }
}

async function main() {
  await rm(CONTENT_DEST, { recursive: true, force: true });

  for (const chapter of CHAPTERS) {
    await rm(join(PUBLIC_DEST, chapter), { recursive: true, force: true });
  }

  for (const chapter of CHAPTERS) {
    const src = join(SOURCE, chapter);
    if (!(await exists(src))) {
      console.warn(`Missing chapter: ${src}`);
      continue;
    }

    const files = await readChapter(chapter);
    await writeChapter(files, join(CONTENT_DEST, chapter));
    await writeChapter(files, join(PUBLIC_DEST, chapter));
  }

  console.log(`Synced ${SOURCE} → ${CONTENT_DEST} (links rewritten, lowercase filenames)`);
  console.log(`Synced markdown → ${PUBLIC_DEST}/{${CHAPTERS.join(',')}}/`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
