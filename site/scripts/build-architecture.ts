#!/usr/bin/env bun
import { $ } from 'bun';
import { mkdir, cp, rm, stat, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const SITE_ROOT = join(__dirname, '..');
const REPO_ROOT = join(SITE_ROOT, '..');

const WORKSPACE = join(REPO_ROOT, 'architecture', 'workspace.dsl');
const STATIC_OUT = join(REPO_ROOT, 'architecture', 'static');
const MERMAID_OUT = join(REPO_ROOT, 'architecture', 'mermaid');
const PUBLIC_STATIC = join(SITE_ROOT, 'public', 'architecture');
const PUBLIC_MERMAID = join(SITE_ROOT, 'public', 'architecture', 'mermaid');

const STRUCTURIZR_IMAGE = 'structurizr/structurizr:latest';

// Injected into the static export's index.html.
//
// Two responsibilities:
//
// 1. Rewrite absolute paths (/css/, /js/, /img/, /workspace.js) to relative
//    so the viewer works when embedded in an iframe served from a nested
//    route like /formal-laws/d1-.../.
//
// 2. Suppress the introduction modal unconditionally. The workspace-level
//    `structurizr.introduction` property and the `?introduction=false`
//    parameter are documented but do not reliably suppress the modal when
//    the static viewer is embedded in a nested iframe.
//
//    A continuous observer (setInterval + MutationObserver) is intentionally
//    NOT used: the Structurizr viewer mutates the DOM thousands of times per
//    second during layout and animation, so a global observer tanks
//    performance. Instead we run a small burst of timeouts that covers the
//    window in which the intro modal appears (typically <2s after load).
const INJECTED_HEAD_SNIPPET = `
<script>
  (function () {
    document.documentElement.setAttribute('data-introduction-suppressed', 'true');

    function nuke() {
      var nodes = document.querySelectorAll(
        '.modal, .modal-backdrop, [role="dialog"][aria-modal="true"], #introduction'
      );
      for (var i = 0; i < nodes.length; i++) {
        var el = nodes[i];
        el.style.setProperty('display', 'none', 'important');
        el.style.setProperty('visibility', 'hidden', 'important');
        el.style.setProperty('pointer-events', 'none', 'important');
        el.setAttribute('aria-hidden', 'true');
      }
    }

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', nuke);
    } else {
      nuke();
    }

    // A short burst of timeouts to catch the modal on appearance.
    // Total overhead: 7 calls, then silence.
    var delays = [100, 300, 600, 1000, 1800, 2800, 4000];
    for (var i = 0; i < delays.length; i++) {
      setTimeout(nuke, delays[i]);
    }
  })();
</script>
<style>
  html[data-introduction-suppressed="true"] .modal,
  html[data-introduction-suppressed="true"] .modal-backdrop,
  html[data-introduction-suppressed="true"] [role="dialog"][aria-modal="true"] {
    display: none !important;
    visibility: hidden !important;
    pointer-events: none !important;
  }
</style>
`;

async function exists(p: string): Promise<boolean> {
  try {
    await stat(p);
    return true;
  } catch {
    return false;
  }
}

async function exportFormat(format: string, output: string) {
  const uid = process.getuid?.() ?? 0;
  const gid = process.getgid?.() ?? 0;

  await rm(output, { recursive: true, force: true });
  await mkdir(output, { recursive: true });

  const result = await $`docker run --rm \
    --user ${uid}:${gid} \
    -v ${REPO_ROOT}/architecture:/usr/local/structurizr \
    -w /usr/local/structurizr \
    ${STRUCTURIZR_IMAGE} export \
    -workspace workspace.dsl \
    -format ${format} \
    -output ${output.split('/').pop()}`.nothrow();

  if (result.exitCode !== 0) {
    throw new Error(`Structurizr ${format} export failed:\n${result.stderr.toString()}`);
  }
}

function rewriteAbsolutePaths(html: string): string {
  let out = html;
  out = out.replace(/(href|src)="\/((?:css|js|img)\/[^"]+)"/g, '$1="./$2"');
  out = out.replace(/(href|src)="\/workspace\.js"/g, '$1="./workspace.js"');
  return out;
}

async function postProcessStaticExport() {
  const indexPath = join(STATIC_OUT, 'index.html');
  if (!(await exists(indexPath))) return;

  let html = await readFile(indexPath, 'utf-8');

  html = rewriteAbsolutePaths(html);

  if (!html.includes('data-introduction-suppressed')) {
    html = html.replace('</head>', `${INJECTED_HEAD_SNIPPET}</head>`);
  }

  await writeFile(indexPath, html, 'utf-8');
}

async function main() {
  if (!(await exists(WORKSPACE))) {
    console.warn(`[build-architecture] No workspace.dsl at ${WORKSPACE}, skipping`);
    return;
  }

  console.log(`[build-architecture] Exporting diagrams from ${WORKSPACE}`);

  await exportFormat('static', STATIC_OUT);
  if (!(await exists(join(STATIC_OUT, 'index.html')))) {
    throw new Error('[build-architecture] Static export produced no index.html');
  }

  await postProcessStaticExport();

  await exportFormat('mermaid', MERMAID_OUT);
  const mermaidFiles = (await readdir(MERMAID_OUT)).filter((f) => f.endsWith('.mmd'));
  if (mermaidFiles.length === 0) {
    throw new Error('[build-architecture] Mermaid export produced no files');
  }

  await rm(PUBLIC_STATIC, { recursive: true, force: true });
  await mkdir(dirname(PUBLIC_STATIC), { recursive: true });
  await cp(STATIC_OUT, PUBLIC_STATIC, { recursive: true });

  await rm(PUBLIC_MERMAID, { recursive: true, force: true });
  await mkdir(dirname(PUBLIC_MERMAID), { recursive: true });
  await cp(MERMAID_OUT, PUBLIC_MERMAID, { recursive: true });

  console.log(`[build-architecture] Static viewer copied to ${PUBLIC_STATIC}`);
  console.log(`[build-architecture] ${mermaidFiles.length} Mermaid file(s) copied to ${PUBLIC_MERMAID}`);
}

main().catch((err) => {
  console.error(`[build-architecture] ${err.message ?? err}`);
  process.exit(1);
});
