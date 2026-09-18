# Sovereign Account — Documentation Site

Static site generator for the Sovereign Account documentation. Compiles Markdown sources under `../domain/account/` into HTML, JSON, and LLM-oriented artifacts, ready to publish to a static host.

## Stack

- **Astro 7** — static site generation with Content Collections and Zod validation
- **Tailwind CSS v4** — styling, dark mode, responsive layout
- **Bun** — package manager and script runner
- **Irys** — permanent storage target (devnet for tests, L1 for production)

## Layout

```text
site/
├── astro.config.mjs
├── package.json
├── public/                    # Favicon and raw .md copies (generated)
├── scripts/
│   ├── sync-content.ts        # Copies docs/, rewrites links, lowercases filenames
│   ├── validate-frontmatter.ts
│   ├── make-relative.ts       # Rewrites absolute paths to relative after astro build
│   ├── generate-data.ts       # Emits dist/_data/*.json
│   ├── generate-llms.ts       # Emits dist/llms.txt
│   ├── balance.ts             # Irys wallet balance check
│   ├── deploy.ts              # Uploads dist/ and the mutable redirect
│   ├── test-mutability.ts     # Verifies Root-TX chain with a small string
│   └── test-mutable-html.ts   # Verifies Content-Type=text/html in mutable chains
└── src/
    ├── content.config.ts
    ├── components/
    ├── layouts/DocLayout.astro
    ├── pages/
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
4. `make-relative.ts` converts absolute asset and route paths to relative paths, so the site works under any origin-isolated host.
5. `generate-data.ts` emits one JSON per document at `dist/_data/<chapter>/<slug>.json`, plus `dist/_data/manifest.json`.
6. `generate-llms.ts` emits `dist/llms.txt` indexing all content docs.

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
bun run deploy:balance         # prints wallet ETH and Irys credit
bun run deploy:devnet          # uploads to Irys devnet (immutable manifest)
bun run deploy:mainnet         # uploads to Irys L1 (immutable manifest)
bun run deploy:devnet:mutable  # uploads + updates the mutable redirect chain
bun run deploy:mainnet:mutable # same, against mainnet
bun run publish:devnet         # build:fresh + deploy:devnet:mutable
bun run publish:mainnet        # build:fresh + deploy:mainnet:mutable
bun run test:mutability        # verifies the Root-TX chain mechanic
```

The deploy script writes `deploy-manifest.json` locally with the last `manifestId`, `folderRootId`, `redirectRootId`, and a history. That file is gitignored.

## Deployment Playbook

Everything below is a pitfall we actually hit while getting this site to render correctly on Irys's CDN. Most apply to any static host without server-side directory resolution.

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

### 4. `<base href="/">` does not work with path-absolute URLs

**Symptom:** After adding `<base>` and rewriting all hrefs to absolute paths (`/assets/...`), the CDN still serves 404s.

**Cause:** Per the HTML spec, `<base href="/foo/">` only affects relative URLs. Path-absolute URLs (`/bar.css`) always resolve against the origin, ignoring `<base>` entirely.

**Fix:** Do not use `<base>`. Instead, rewrite all internal links and asset references to relative paths after `astro build`. See `scripts/make-relative.ts`.

### 5. `/` and `/_astro/` are unreliable on Irys

**Symptom:** CSS 404s on the Irys CDN with `Unable to find specified path in manifest`.

**Cause:** Irys treats `_`-prefixed paths specially and expects `assets/` at the root of the manifest.

**Fix:**

```ts
// astro.config.mjs
build: {
  format: 'directory',
  assets: 'assets',   // emits to /assets/ instead of /_astro/
},
```

Every internal link must end with a trailing slash. The CDN does not resolve `/foo` to `/foo/index.html`. It only serves `/foo/`.

### 6. Rewrite markdown links once, at sync time

**Symptom:** Cross-references in the rendered HTML point to `/page.md` or resolve incorrectly on the CDN.

**Cause:** Astro's markdown link handling behaves differently in dev versus build, and differently across hosts.

**Fix:** Rewrite all markdown links to absolute, lowercase, trailing-slash routes **before** Astro sees them, in `scripts/sync-content.ts`:

```text
[Foundations](01-foundations.md)
  → [Foundations](/ubiquitous-language/01-foundations/)
```

The relative-path pass (`make-relative.ts`) then converts those absolute paths to relative at the HTML level.

### 7. Astro lowercases content IDs, so raw file names must match

**Symptom:** "View raw" links work for `ubiquitous-language/01-foundations.md` but 404 for `formal-laws/D1-delegation-and-authority.md`.

**Cause:** Astro slugifies content IDs to lowercase (`entry.id === 'd1-delegation-and-authority'`), but the raw `.md` copies kept their original case.

**Fix:** Lowercase every filename when copying to `public/`.

### 8. Zod schema silently drops entries

**Symptom:** `index.md` files exist but no `/formal-laws/index.html` is generated.

**Cause:** The Zod schema required `chapter` and `chapter-number` on every entry. `index.md` files lack those fields and are silently filtered out.

**Fix:** Make the fields optional and enforce them with `.refine()` only on non-index entries, and in `[...slug].astro`, map the index entry to its chapter root.

### 9. Cache by path, not by manifest

**Symptom:** After deploying a new manifest, the CDN serves stale HTML.

**Cause:** The Irys CDN (`*.datasprite-cdn.com`) caches responses by URL path, not by manifest ID.

**Fix:** Rebuild clean so asset hashes change. The new hash invalidates the CSS reference and forces the CDN to fetch fresh HTML. If it persists, open in incognito.

### 10. Irys gas race on Arbitrum Sepolia

**Symptom:** `max fee per gas less than block base fee` when funding the Irys account.

**Cause:** `irys.fund()` computes `maxFeePerGas` from the current `baseFee` without headroom.

**Fix:** Pass a fee multiplier to `fund()` and retry on race errors.

### 11. BigNumber.js vs bigint in the Irys SDK

**Symptom:** `price.sub is not a function` when comparing balances.

**Cause:** The Irys SDK returns values as `BigNumber`, `bigint`, `string`, or `number` depending on the version and call.

**Fix:** Normalize every value to `bigint` on receipt via a `toBigInt` helper.

### 12. `/mutable/` does not resolve `indexFile`

**Symptom:** `gateway.irys.xyz/mutable/<folderRootId>/` returns the raw manifest JSON, not the rendered `index.html`.

**Cause:** The `/mutable/:txId` endpoint serves the transaction content as-is. Unlike `gateway.irys.xyz/:manifestId`, it does not resolve `indexFile` for folder manifests. This is documented implicitly: the mutability docs describe `/mutable/` for single-file pointers, and the folder docs only mention `indexFile` for the `/:manifestId` endpoint.

**Fix:** Upload a separate **redirect HTML** as an individual transaction with `Content-Type: text/html`, tagged `Root-TX`. The mutable URL then serves that redirect, which forwards the browser to the current manifest.

See "Mutability and the redirect chain" below.

### 13. Rebuild clean before any deploy

**Symptom:** Deploy uploads stale content, or the site renders without CSS on Irys but works locally.

**Cause:** Running `bun run clean` from inside `dist/` does not clear the directory. `rm -rf dist` resolves to `dist/dist`, which doesn't exist. Astro may also refuse to wipe its own `outDir` when the current working directory is inside it.

**Fix:** Always run build commands from `site/`, never from `site/dist/`. Use `bun run clean:full`, which also wipes `node_modules/.astro` and `node_modules/.vite`.

```sh
cd site
bun run build:fresh
```

## Mutability and the redirect chain

Irys is immutable by design. Every `uploadFolder()` produces a new `manifestId`, and every `manifestId` is a different permanent URL. For a documentation site that changes over time, sharing a link means the link points to a stale version.

Irys supports a **mutable reference** built on the `Root-TX` tag:

```text
Root-TX: <baseTxId>
```

Any subsequent transaction tagged with `Root-TX` joins the chain. The endpoint `gateway.irys.xyz/mutable/<baseTxId>/` always resolves to the most recent transaction in the chain, by millisecond-precise timestamp. All updates must be signed by the same wallet that created the base transaction.

### The folder manifest limitation

`uploadFolder()` accepts an `indexFile` parameter that instructs the gateway to serve a specific file when accessing `gateway.irys.xyz/:manifestId`. **This resolution does not apply to `/mutable/:txId`.** The mutable endpoint treats the transaction as an opaque blob, so a folder manifest returns its JSON descriptor instead of the `index.html`.

### The workaround: a second, separate chain

We maintain two chains on Irys:

1. **Folder chain** (`folderRootId`): built with `uploadFolder()` + `manifestTags: [{ Root-TX: folderRootId }]`. Not directly usable as a public URL, but preserved for the day the gateway supports `indexFile` on mutable endpoints.
2. **Redirect chain** (`redirectRootId`): built with `upload(<html>, { tags: [{ 'Content-Type': 'text/html' }, { 'Root-TX': redirectRootId }] })`. This is the public URL.

Flow:

```text
deploy
  │
  ├── uploadFolder(dist) ──────────────→ manifestId (per-deploy content)
  │
  ├── generate redirect.html ──────────→ points to manifestId
  │
  └── upload(redirect.html, Root-TX) ──→ txId

Public URL: https://gateway.irys.xyz/mutable/<redirectRootId>/
             ↓ serves the latest redirect
             ↓ JS window.location.replace() + <noscript> meta refresh
             https://gateway.irys.xyz/<manifestId>/
```

The redirect uses `window.location.replace()` for an immediate JS redirect, with a `<meta http-equiv="refresh">` inside `<noscript>` as fallback for JS-disabled clients. The page renders a dark placeholder while the redirect fires.

### CDN domains vs gateway

Every Irys upload produces two URLs:

- **Gateway**: `https://gateway.irys.xyz/<id>/`
- **CDN subdomain**: `https://<sandbox>.<node>.datasprite-cdn.com/<id>/`

**Always link to the gateway, not the CDN subdomain.** The Irys documentation explicitly warns:

> Always use `gateway.irys.xyz/<transaction-id>` to link to content. Never link directly to CDN domains — they may be rotated or replaced at any time.

The gateway routes automatically to a healthy CDN domain. Direct CDN subdomain links may break without notice.

### Custom domains

Irys does not currently offer user-configurable custom domains. The domains Irys operates are listed at `irys.xyz/security`:

- `irys.xyz`, `docs.irys.xyz`, `uploader.irys.xyz`, `gateway.irys.xyz`
- `node1.irys.xyz`, `node2.irys.xyz`, `devnet.irys.xyz`
- `datasprite-cdn.com` (CDN infrastructure)

None are assignable to user content. For a branded URL, the mapping has to live outside Irys.

### Mainnet migration

Everything in this section works identically on mainnet. The network flag and RPC endpoint are the only differences, both configured in `.env`. Recommended sequence:

1. `bun run test:mutability` — verify the `Root-TX` chain on devnet with a small string.
2. `bun run test:mutable-html` — verify `Content-Type: text/html` survives the chain.
3. `bun run deploy:devnet:mutable` — first real deploy with the full site on devnet.
4. `bun run deploy:devnet:mutable` again — verify the redirect URL resolves to the new content.
5. `bun run deploy:mainnet:mutable` — first mainnet deploy, creates the production chains.
6. Update any public links to the mainnet redirect URL.

## Pre-Deploy Checklist

Run these before `bun run deploy:devnet` or `bun run deploy:mainnet`:

```sh
cd site
rm -rf dist .astro src/content/docs public/ubiquitous-language public/formal-laws
bun run build

# All checks must pass
grep -c 'base-tag' dist/formal-laws/d1-delegation-and-authority/index.html          # expect 0
grep -o 'href="[^"]*DocLayout[^"]*css"' dist/formal-laws/d1-delegation-and-authority/index.html
# expect href="../../assets/DocLayout.XXXX.css"
ls dist/formal-laws/index.html dist/ubiquitous-language/index.html                    # both must exist
grep -o 'href="[^"]*formal-laws/d1-delegation-and-authority/"' dist/ubiquitous-language/07-reference/index.html
# expect href="../../formal-laws/d1-delegation-and-authority/"

bun run deploy:balance
```

After deploy, verify in an incognito window:

- `https://gateway.irys.xyz/<manifestId>/` — HTML and CSS render.
- `https://gateway.irys.xyz/<manifestId>/formal-laws/` — Formal Laws landing loads.
- `https://gateway.irys.xyz/<manifestId>/formal-laws/d1-delegation-and-authority.md` — raw markdown returns.
- `https://gateway.irys.xyz/mutable/<redirectRootId>/` — redirects to the manifest and renders.
- Sidebar, TOC anchors, "View raw", and "Open in" all work.
