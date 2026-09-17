import { cp, rm, mkdir, copyFile, readdir } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(__dirname, '..');
const REPO_ROOT = join(PROJECT_ROOT, '..');

const SOURCE = join(REPO_ROOT, 'domain', 'account');
const CONTENT_DEST = join(PROJECT_ROOT, 'src', 'content', 'docs', 'domain', 'account');
const PUBLIC_DEST = join(PROJECT_ROOT, 'public');

async function syncCollections() {
  await rm(CONTENT_DEST, { recursive: true, force: true });
  await mkdir(dirname(CONTENT_DEST), { recursive: true });
  await cp(SOURCE, CONTENT_DEST, { recursive: true });
  console.log(`Synced ${SOURCE} → ${CONTENT_DEST}`);
}

async function syncMarkdown() {
  const chapters = ['ubiquitous-language', 'formal-laws'];

  for (const chapter of chapters) {
    const srcDir = join(SOURCE, chapter);
    const destDir = join(PUBLIC_DEST, chapter);

    await rm(destDir, { recursive: true, force: true });
    await mkdir(destDir, { recursive: true });

    const files = await readdir(srcDir);
    for (const file of files) {
      if (file.endsWith('.md')) {
        await copyFile(join(srcDir, file), join(destDir, file));
      }
    }
  }
  console.log(`Synced markdown → ${PUBLIC_DEST}/{ubiquitous-language,formal-laws}/`);
}

async function main() {
  await syncCollections();
  await syncMarkdown();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
