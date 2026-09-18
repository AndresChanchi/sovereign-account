#!/usr/bin/env bun
import { Uploader } from '@irys/upload';
import { Arbitrum } from '@irys/upload-ethereum';
import { ethers } from 'ethers';
import { readdir, stat, writeFile, readFile } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(__dirname, '..');
const DIST = join(PROJECT_ROOT, 'dist');
const MANIFEST_FILE = join(PROJECT_ROOT, 'deploy-manifest.json');

type Network = 'devnet' | 'mainnet';

interface DeployOptions {
  network: Network;
  dryRun: boolean;
  mutable: boolean;
}

interface ManifestEntry {
  network: Network;
  manifestId: string;
  gateway: string;
  folderRootId?: string;
  redirectRootId?: string;
  redirectGateway?: string;
  timestamp: string;
  sizeBytes: number;
}

interface ManifestFile {
  latest: ManifestEntry | null;
  history: ManifestEntry[];
}

function parseArgs(): DeployOptions {
  const args = process.argv.slice(2);
  let network: Network = 'devnet';
  let dryRun = false;
  let mutable = false;

  for (const arg of args) {
    if (arg === '--mainnet' || arg === '--network=mainnet') network = 'mainnet';
    if (arg === '--devnet' || arg === '--testnet' || arg === '--network=devnet') network = 'devnet';
    if (arg === '--dry-run') dryRun = true;
    if (arg === '--mutable') mutable = true;
  }

  return { network, dryRun, mutable };
}

function loadEnv(network: Network) {
  const suffix = network.toUpperCase();
  const pk = process.env[`PRIVATE_KEY_${suffix}`];
  const rpc = process.env[`RPC_URL_${suffix}`];

  if (!pk) throw new Error(`Missing PRIVATE_KEY_${suffix} in environment`);
  if (!rpc) throw new Error(`Missing RPC_URL_${suffix} in environment`);

  return { pk, rpc };
}

function toBigInt(value: unknown): bigint {
  if (value == null) throw new Error('toBigInt: null/undefined value');
  if (typeof value === 'bigint') return value;
  if (typeof value === 'string') return BigInt(value);
  if (typeof value === 'number') return BigInt(Math.trunc(value));
  return BigInt((value as { toString(): string }).toString());
}

function formatEth(value: bigint): string {
  return ethers.formatEther(value);
}

async function dirSize(dir: string): Promise<number> {
  const entries = await readdir(dir, { withFileTypes: true });
  let total = 0;
  for (const e of entries) {
    const full = join(dir, e.name);
    if (e.isDirectory()) total += await dirSize(full);
    else total += (await stat(full)).size;
  }
  return total;
}

function fmt(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 ** 2) return `${(bytes / 1024).toFixed(1)} KiB`;
  if (bytes < 1024 ** 3) return `${(bytes / 1024 ** 2).toFixed(2)} MiB`;
  return `${(bytes / 1024 ** 3).toFixed(2)} GiB`;
}

async function readManifest(): Promise<ManifestFile> {
  try {
    const raw = await readFile(MANIFEST_FILE, 'utf-8');
    return JSON.parse(raw) as ManifestFile;
  } catch {
    return { latest: null, history: [] };
  }
}

async function writeManifest(manifest: ManifestFile): Promise<void> {
  await writeFile(MANIFEST_FILE, JSON.stringify(manifest, null, 2) + '\n', 'utf-8');
}

const FUND_HEADROOM = ethers.parseEther('0.0005');

async function fundWithRetry(
  irys: Awaited<ReturnType<ReturnType<typeof Uploader>['withWallet']>>,
  amount: bigint,
  multipliers: number[] = [1.5, 2.0, 3.0],
): Promise<{ id: string }> {
  let lastError: unknown = null;

  for (const multiplier of multipliers) {
    try {
      console.log(`  Attempting fund with fee multiplier ${multiplier}...`);
      const receipt = await irys.fund(
        amount as unknown as Parameters<typeof irys.fund>[0],
        multiplier,
      );
      return receipt;
    } catch (err) {
      lastError = err;
      const msg = err instanceof Error ? err.message : String(err);
      if (msg.includes('max fee per gas less than block base fee')) {
        console.log(`  Race detected at multiplier ${multiplier}, retrying...`);
        continue;
      }
      throw err;
    }
  }

  throw new Error(
    `Funding failed after ${multipliers.length} attempts. Last error: ${
      lastError instanceof Error ? lastError.message : String(lastError)
    }`,
  );
}

function buildRedirectHtml(manifestId: string): string {
  const target = `https://gateway.irys.xyz/${manifestId}/`;
  const jsonTarget = JSON.stringify(target);

  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Redirecting — Sovereign Account</title>
<link rel="canonical" href="${target}">
<style>
  html { color-scheme: light dark; }
  body {
    font-family: ui-sans-serif, system-ui, -apple-system, sans-serif;
    display: flex; align-items: center; justify-content: center;
    min-height: 100vh; margin: 0; padding: 2rem;
    background: #020617; color: #f1f5f9;
    text-align: center;
  }
  a { color: #10b981; }
  p { max-width: 32rem; line-height: 1.6; }
  code { font-family: ui-monospace, monospace; font-size: 0.875em; }
</style>
<script>
  window.location.replace(${jsonTarget});
</script>
<noscript>
  <meta http-equiv="refresh" content="0; url=${target}">
</noscript>
</head>
<body>
<p>Redirecting to the latest version…<br><a href="${target}"><code>${target}</code></a></p>
</body>
</html>`;
}

async function main() {
  const { network, dryRun, mutable } = parseArgs();
  const { pk, rpc } = loadEnv(network);

  const wallet = new ethers.Wallet(pk);
  const size = await dirSize(DIST);
  const previous = await readManifest();

  const prevEntry =
    previous.latest?.network === network ? previous.latest : null;
  const folderRootId = prevEntry?.folderRootId;
  const redirectRootId = prevEntry?.redirectRootId;
  const willCreateFolderRoot = mutable && !folderRootId;
  const willCreateRedirectRoot = mutable && !redirectRootId;

  console.log(`\n─── Irys Deploy ───`);
  console.log(`  Network      ${network}`);
  console.log(`  Wallet       ${wallet.address}`);
  console.log(`  RPC          ${rpc}`);
  console.log(`  Dist size    ${fmt(size)}`);
  console.log(`  Mode         ${mutable ? 'mutable' : 'immutable'}`);
  if (mutable && folderRootId) {
    console.log(`  Folder root  ${folderRootId}`);
  }
  if (mutable && redirectRootId) {
    console.log(`  Redirect root ${redirectRootId}`);
  }

  const builder = Uploader(Arbitrum).withWallet(pk).withRpc(rpc);
  const irys = network === 'devnet' ? await builder.devnet() : await builder;

  const provider = new ethers.JsonRpcProvider(rpc);
  const [walletBal, irysBal, price] = await Promise.all([
    provider.getBalance(wallet.address),
    irys.getBalance(),
    irys.getPrice(size),
  ]);

  const walletAtomic = toBigInt(walletBal);
  const irysAtomic = toBigInt(irysBal);
  const costAtomic = toBigInt(price);

  console.log(`\n  Wallet ETH   ${formatEth(walletAtomic)}`);
  console.log(`  Irys credit  ${formatEth(irysAtomic)}`);
  console.log(`  Est. cost    ${formatEth(costAtomic)}`);

  if (dryRun) {
    console.log(`\n[dry-run] Aborting before upload.`);
    return;
  }

  if (irysAtomic < costAtomic) {
    const gap = costAtomic - irysAtomic;
    const fundAmount = gap + FUND_HEADROOM;

    console.log(`\n  Funding Irys account with ${formatEth(fundAmount)} ETH...`);
    const fundReceipt = await fundWithRetry(irys, fundAmount);
    console.log(`  Funded: ${fundReceipt.id}`);

    process.stdout.write('  Waiting for credit');
    while (toBigInt(await irys.getBalance()) < costAtomic) {
      process.stdout.write('.');
      await new Promise((r) => setTimeout(r, 2000));
    }
    console.log();
  }

  // ── 1. Upload the folder ──
  const folderManifestTags =
    mutable && folderRootId ? [{ name: 'Root-TX', value: folderRootId }] : [];

  console.log(`\n  Uploading dist/...`);
  const start = Date.now();
  const folderReceipt = await irys.uploadFolder(DIST, {
    indexFile: 'index.html',
    batchSize: 50,
    ...(folderManifestTags.length > 0 ? { manifestTags: folderManifestTags } : {}),
  });
  const elapsed = ((Date.now() - start) / 1000).toFixed(1);

  const manifestId = folderReceipt.id;
  const gateway = `https://gateway.irys.xyz/${manifestId}/`;

  console.log(`  Uploaded in ${elapsed}s`);
  console.log(`  Manifest ID  ${manifestId}`);
  console.log(`  Gateway      ${gateway}`);

  // ── 2. Upload the redirect (mutable only) ──
  let newFolderRootId: string | undefined;
  let newRedirectRootId: string | undefined;
  let redirectGateway: string | undefined;

  if (mutable) {
    newFolderRootId = folderRootId ?? manifestId;

    const redirectHtml = buildRedirectHtml(manifestId);
    const redirectTags = [
      { name: 'Content-Type', value: 'text/html' },
      ...(redirectRootId ? [{ name: 'Root-TX', value: redirectRootId }] : []),
    ];

    console.log(`\n  Uploading redirect...`);
    const redirectReceipt = await irys.upload(redirectHtml, { tags: redirectTags });

    newRedirectRootId = redirectRootId ?? redirectReceipt.id;
    redirectGateway = `https://gateway.irys.xyz/mutable/${newRedirectRootId}/`;

    console.log(`  Redirect TX  ${redirectReceipt.id}`);
    console.log(
      `  Redirect root ${newRedirectRootId}${willCreateRedirectRoot ? ' (created)' : ''}`,
    );
    console.log(`  Redirect URL ${redirectGateway}`);
  }

  // ── 3. Persist manifest ──
  const entry: ManifestEntry = {
    network,
    manifestId,
    gateway,
    ...(mutable
      ? {
          folderRootId: newFolderRootId,
          redirectRootId: newRedirectRootId,
          redirectGateway,
        }
      : {}),
    timestamp: new Date().toISOString(),
    sizeBytes: size,
  };

  previous.history.push(entry);
  previous.latest = entry;
  await writeManifest(previous);

  console.log(`\n  Manifest recorded in ${MANIFEST_FILE}`);

  if (mutable) {
    console.log(`\n  Public URL (stable):`);
    console.log(`    ${redirectGateway}`);
    console.log(`  Content URL (per deploy):`);
    console.log(`    ${gateway}`);
    console.log(`\n  Next update:`);
    console.log(`    bun run deploy:${network}:mutable`);
  }
}

main().catch((err) => {
  console.error(`\n❌ ${err.message ?? err}`);
  process.exit(1);
});
