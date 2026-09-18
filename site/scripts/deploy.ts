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

function parseArgs(): { network: Network; dryRun: boolean } {
  const args = process.argv.slice(2);
  let network: Network = 'devnet';
  let dryRun = false;

  for (const arg of args) {
    if (arg === '--mainnet' || arg === '--network=mainnet') network = 'mainnet';
    if (arg === '--devnet' || arg === '--testnet' || arg === '--network=devnet') network = 'devnet';
    if (arg === '--dry-run') dryRun = true;
  }

  return { network, dryRun };
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

async function readPreviousManifest(): Promise<Record<string, unknown> | null> {
  try {
    return JSON.parse(await readFile(MANIFEST_FILE, 'utf-8'));
  } catch {
    return null;
  }
}

const FUND_HEADROOM = ethers.parseEther('0.0005');

/**
 * Attempts to fund the Irys account with progressive fee multipliers.
 * Arbitrum Sepolia baseFee fluctuates per block; the SDK computes maxFeePerGas
 * without headroom, so a race is possible. Each retry increases the multiplier.
 */
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
      // Unknown error, don't retry
      throw err;
    }
  }

  throw new Error(
    `Funding failed after ${multipliers.length} attempts. Last error: ${lastError instanceof Error ? lastError.message : String(lastError)}`,
  );
}

async function main() {
  const { network, dryRun } = parseArgs();
  const { pk, rpc } = loadEnv(network);

  const wallet = new ethers.Wallet(pk);
  const size = await dirSize(DIST);

  console.log(`\n─── Irys Deploy ───`);
  console.log(`  Network     ${network}`);
  console.log(`  Wallet      ${wallet.address}`);
  console.log(`  RPC         ${rpc}`);
  console.log(`  Dist size   ${fmt(size)}`);

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

  console.log(`\n  Wallet ETH  ${formatEth(walletAtomic)}`);
  console.log(`  Irys credit ${formatEth(irysAtomic)}`);
  console.log(`  Est. cost   ${formatEth(costAtomic)}`);

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

  console.log(`\n  Uploading dist/...`);
  const start = Date.now();
  const receipt = await irys.uploadFolder(DIST, {
    indexFile: 'index.html',
    batchSize: 50,
  });
  const elapsed = ((Date.now() - start) / 1000).toFixed(1);

  const manifestId = receipt.id;
  const gateway = `https://gateway.irys.xyz/${manifestId}/`;

  console.log(`\n  Uploaded in ${elapsed}s`);
  console.log(`  Manifest ID  ${manifestId}`);
  console.log(`  Gateway      ${gateway}`);
  console.log(`\n  Verify:`);
  console.log(`    curl -sI ${gateway}`);
  console.log(`    curl -s  ${gateway}llms.txt | head`);

  const previous = (await readPreviousManifest()) as { history?: unknown[] } | null;
  const history = previous?.history ?? [];
  history.push({
    network,
    manifestId,
    gateway,
    timestamp: new Date().toISOString(),
    sizeBytes: size,
  });

  await writeFile(
    MANIFEST_FILE,
    JSON.stringify({ latest: { network, manifestId, gateway }, history }, null, 2) + '\n',
    'utf-8',
  );

  console.log(`\n  Manifest recorded in ${MANIFEST_FILE}`);
}

main().catch((err) => {
  console.error(`\n❌ ${err.message ?? err}`);
  process.exit(1);
});
