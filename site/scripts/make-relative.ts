import { readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const DIST = join(import.meta.dir, '..', 'dist');

async function collectHtml(dir: string): Promise<string[]> {
  const entries = await readdir(dir, { withFileTypes: true });
  const out: string[] = [];
  for (const entry of entries) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) out.push(...(await collectHtml(full)));
    else if (entry.name.endsWith('.html')) out.push(full);
  }
  return out;
}

function depthOf(root: string, file: string): number {
  const rel = relative(root, file);
  return rel.split('/').length - 1;
}

function makeRelative(content: string, prefix: string): string {
  let out = content;

  // Assets emitted by Astro (/_astro/ or custom /assets/).
  out = out.replace(/(href|src)="\/(assets|_astro)\//g, `$1="${prefix}$2/`);

  // Chapter routes used by sidebar, TOC, and markdown cross-references.
  out = out.replace(
    /href="\/(ubiquitous-language|formal-laws)\//g,
    `href="${prefix}$1/`,
  );

  // Root link (site logo). Match only when "/" is the entire path.
  out = out.replace(/href="\/(?=["\s>])/g, `href="${prefix}`);

  return out;
}

async function main() {
  const htmlFiles = await collectHtml(DIST);

  for (const file of htmlFiles) {
    const depth = depthOf(DIST, file);
    const prefix = depth === 0 ? './' : '../'.repeat(depth);
    const content = await readFile(file, 'utf-8');
    await writeFile(file, makeRelative(content, prefix), 'utf-8');
  }

  console.log(`[make-relative] rewrote ${htmlFiles.length} HTML file(s)`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
