# Sovereign Account — Documentation Site

Static site generator for the Sovereign Account documentation. Compiles Markdown sources under `../domain/account/` into HTML, JSON, and LLM-oriented artifacts, ready to publish to any static host (Irys, GitHub Pages, Cloudflare Pages, Netlify).

## Stack

- **Astro 7** — static site generation with Content Collections and Zod validation
- **Tailwind CSS v4** — styling, dark mode, responsive layout
- **Bun** — package manager and script runner
- **Irys** — permanent storage target (devnet for tests, L1 for production)

## Layout

```text
site/
├── astro.config.mjs           # Astro, Tailwind, markdown processor
├── package.json               # Scripts and dependencies
├── public/                    # Favicon and raw .md copies (generated)
├── scripts/
│   ├── sync-content.ts        # Copies docs/, rewrites links, lowercases filenames
│   ├── validate-frontmatter.ts
│   ├── generate-data.ts       # Emits dist/_data/*.json
│   ├── generate-llms.ts       # Emits dist/llms.txt
│   ├── balance.ts             # Irys wallet balance check
│   └── deploy.ts              # Uploads dist/ to Irys
└── src/
    ├── content.config.ts      # Zod schemas for frontmatter
    ├── components/            # Sidebar, Toc, ThemeToggle, PageActions
    ├── layouts/DocLayout.astro
    ├── pages/
    │   ├── index.astro
    │   └── [...slug].astro
    └── styles/globals.css
```

## Development

```sh
bun install
bun run dev
```

`dev` runs `sync-content.ts` first, then starts the Astro dev server at `http://localhost:4321`. Edits to `../domain/account/*.md` require restarting the dev server (the sync is a one-shot copy).

## Build

```sh
bun run build
```

The build pipeline:

1. `sync-content.ts` copies `../domain/account/` into `src/content/docs/` and `public/`, rewrites all `.md` links to absolute lowercase routes, and lowercases filenames.
2. `validate-frontmatter.ts` rejects any `.md` missing required frontmatter.
3. `astro build` generates HTML into `dist/`.
4. `generate-data.ts` emits one JSON per document at `dist/_data/<chapter>/<slug>.json`, plus `dist/_data/manifest.json`.
5. `generate-llms.ts` emits `dist/llms.txt` indexing all content docs.

Final output:

```text
dist/
├── index.html
├── ubiquitous-language/<slug>/index.html   # rendered HTML
├── ubiquitous-language/<slug>.md           # raw markdown for LLMs
├── formal-laws/<slug>/index.html
├── formal-laws/<slug>.md
├── _data/<chapter>/<slug>.json             # structured frontmatter
├── _data/manifest.json
├── assets/                                 # CSS bundle
└── llms.txt
```

## Deployment

Copy `.env.example` to `.env` and fill in your keys:

```sh
cp .env.example .env
```

```env
PRIVATE_KEY_DEVNET=0x...
RPC_URL_DEVNET=https://sepolia-rollup.arbitrum.io/rpc
PRIVATE_KEY_MAINNET=0x...
RPC_URL_MAINNET=https://arb1.arbitrum.io/rpc
```

Commands:

```sh
bun run deploy:balance   # prints wallet ETH and Irys credit
bun run deploy:devnet    # uploads to Irys devnet
bun run deploy:mainnet   # uploads to Irys L1
```

The deploy script writes `deploy-manifest.json` locally with the last `manifestId` and a history. That file is gitignored.

## Deployment Playbook

Everything below is a pitfall we actually hit while getting this site to render correctly on Irys's CDN. Most apply to any static host without server-side directory resolution (Irys, IPFS gateways, some CDN configurations). Read this before modifying the pipeline.

### 1. Astro 5 removed `entry.render()`

**Symptom:** `TypeError: entry.render is not a function` at build.

**Cause:** Astro 4 exposed `render` as a method on the entry. Astro 5 moved it to a top-level function.

**Fix:**

```ts
import { getCollection, render } from 'astro:content';
const { Content, headings } = await render(entry);
```

### 2. Astro 7 defaults to Sätteri, not unified

**Symptom:** After adding `remarkPlugins` to `markdown`, the build fails with "unified processor is no longer installed by default".

**Cause:** Astro 7 switched the default Markdown processor to Sätteri. Remark plugins require the `unified` processor, no longer bundled.

**Fix:**

```sh
bun add @astrojs/markdown-remark
```

```ts
import { unified } from '@astrojs/markdown-remark';

export default defineConfig({
  markdown: {
    processor: unified({ remarkPlugins: [ /* ... */ ] }),
  },
});
```

### 3. Astro ignores `_`-prefixed directories in `pages/`

**Symptom:** An endpoint at `src/pages/_data/[...slug].json.ts` never runs.

**Cause:** Astro treats any file or folder starting with `_` inside `pages/` as private and skips it. It is a Jekyll-era convention.

**Fix:** Generate those files outside Astro's routing. `generate-data.ts` writes directly to `dist/_data/` after `astro build`.

### 4. `<base href="/">` breaks fragment links

**Symptom:** Clicking "On this page" navigates to `https://site/#anchor` instead of `https://site/page/#anchor`.

**Cause:** A `<base>` tag causes the browser to resolve all relative URLs, including pure fragments, against the base URL rather than the current document.

**Fix:** Do not use `<base>`. Make every link absolute. The markdown link rewriter handles this at sync time.

### 5. `/` and `/_astro/` are unreliable on static CDNs

**Symptom:** CSS 404s on the Irys CDN with `Unable to find specified path in manifest`.

**Cause:** Some CDNs treat `_`-prefixed paths as private. Others require explicit trailing slashes for directory resolution.

**Fix:**

```ts
// astro.config.mjs
build: {
  format: 'directory',
  assets: 'assets',   // emits to /assets/ instead of /_astro/
},
```

Every internal link must end with a trailing slash. The CDN does not resolve `/formal-laws/d1-delegation-and-authority` to `/formal-laws/d1-delegation-and-authority/index.html`. It only serves `/formal-laws/d1-delegation-and-authority/`.

### 6. Rewrite markdown links once, at sync time

**Symptom:** Cross-references in the rendered HTML point to `/page.md` or resolve incorrectly on the CDN.

**Cause:** Astro's markdown link handling behaves differently in dev versus build, and differently across hosts.

**Fix:** Rewrite all markdown links to absolute, lowercase, trailing-slash routes **before** Astro sees them. This is done in `scripts/sync-content.ts`:

```text
[Foundations](01-foundations.md)
  → [Foundations](/ubiquitous-language/01-foundations/)
```

Do not attempt this with a remark plugin. The plugin's `file.path` varies between environments and the rewriting silently skips files.

### 7. Astro lowercases content IDs, so raw file names must match

**Symptom:** "View raw" links work for `ubiquitous-language/01-foundations.md` but 404 for `formal-laws/D1-delegation-and-authority.md`.

**Cause:** Astro slugifies content IDs to lowercase (`entry.id === 'd1-delegation-and-authority'`), but the raw `.md` copies kept their original case. The link pointed to the lowercase version, which did not exist.

**Fix:** Lowercase every filename when copying to `public/`. Same in `sync-content.ts`.

### 8. Zod schema silently drops entries

**Symptom:** `index.md` files exist but no `/formal-laws/index.html` is generated. Build reports 14 pages when you expect 16.

**Cause:** The Zod schema required `chapter` and `chapter-number` on every entry. `index.md` files lack those fields and are silently filtered out.

**Fix:** Make the fields optional and enforce them with `.refine()` only on non-index entries:

```ts
.refine(
  (data) => data.type === 'index' || (data.chapter && data['chapter-number']),
  { message: 'chapter and chapter-number are required for non-index entries' },
)
```

And in `[...slug].astro`, map the index entry to its chapter root:

```ts
const isIndexEntry = entry.id === 'index';
const slug = isIndexEntry ? collection : `${collection}/${entry.id}`;
```

### 9. Cache by path, not by manifest

**Symptom:** After deploying a new manifest, the CDN serves stale HTML.

**Cause:** The Irys CDN (`*.datasprite-cdn.com`) caches responses by URL path, not by manifest ID. When you redeploy, the path is the same and the old HTML is served from cache.

**Fix:** Rebuild clean so asset hashes change. The new hash invalidates the CSS reference and forces the CDN to fetch fresh HTML. If it persists, open in incognito or append a cache-busting query parameter.

### 10. Irys gas race on Arbitrum Sepolia

**Symptom:** `max fee per gas less than block base fee` when funding the Irys account.

**Cause:** `irys.fund()` computes `maxFeePerGas` from the current `baseFee` without headroom. Arbitrum Sepolia recalculates `baseFee` per block, and by the time the transaction reaches the mempool the base fee has risen.

**Fix:** Pass a fee multiplier to `fund()` and retry on race errors:

```ts
for (const multiplier of [1.5, 2.0, 3.0]) {
  try {
    return await irys.fund(amount, multiplier);
  } catch (err) {
    if (String(err).includes('max fee per gas')) continue;
    throw err;
  }
}
```

### 11. BigNumber.js vs bigint in the Irys SDK

**Symptom:** `price.sub is not a function` when comparing balances.

**Cause:** The Irys SDK returns values as `BigNumber`, `bigint`, `string`, or `number` depending on the version and call. Arithmetic methods are inconsistent.

**Fix:** Normalize every value to `bigint` on receipt:

```ts
function toBigInt(value: unknown): bigint {
  if (typeof value === 'bigint') return value;
  if (typeof value === 'string') return BigInt(value);
  if (typeof value === 'number') return BigInt(Math.trunc(value));
  return BigInt(value.toString());
}
```

### 12. Rebuild clean before any deploy

**Symptom:** Deploy uploads stale content.

**Cause:** Astro's build cache (`.astro/`) or leftover `dist/` from a previous build.

**Fix:**

```sh
rm -rf dist .astro src/content/docs public/ubiquitous-language public/formal-laws
bun run build
bun run deploy:devnet
```

## Pre-Deploy Checklist

Run these before `bun run deploy:devnet` or `deploy:mainnet`:

```sh
cd site
rm -rf dist .astro src/content/docs public/ubiquitous-language public/formal-laws
bun run build

grep -c 'base-tag' dist/formal-laws/d1-delegation-and-authority/index.html       # expect 0
grep -o 'href="/[^"]*css"' dist/formal-laws/d1-delegation-and-authority/index.html   # expect /assets/...
ls dist/formal-laws/index.html dist/ubiquitous-language/index.html               # both must exist
grep -o 'href="/formal-laws/[^"]*"' dist/ubiquitous-language/07-reference/index.html  # all lowercase

bun run deploy:balance
```

After deploy, verify on the gateway:

- `https://gateway.irys.xyz/<manifestId>/` loads with styles.
- `https://gateway.irys.xyz/<manifestId>/formal-laws/` loads the Formal Laws landing.
- `https://gateway.irys.xyz/<manifestId>/formal-laws/d1-delegation-and-authority.md` returns markdown.
- Clicking a sidebar link navigates to the correct path.
- "View raw" opens the markdown.
- "On this page" anchors jump within the current page.
