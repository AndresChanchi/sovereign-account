#!/usr/bin/env bun
import { Uploader } from '@irys/upload';
import { Arbitrum } from '@irys/upload-ethereum';

const TIMESTAMP = new Date().toISOString();
const V1 = `mutability test v1 — ${TIMESTAMP}\n`;
const V2 = `mutability test v2 — ${TIMESTAMP}\n`;

async function fetchMutable(rootId: string, attempts = 15): Promise<string | null> {
  const url = `https://gateway.irys.xyz/mutable/${rootId}`;
  for (let i = 0; i < attempts; i++) {
    try {
      const res = await fetch(url, { redirect: 'follow' });
      if (res.ok) {
        const text = await res.text();
        if (text.includes('mutability test')) return text;
      }
    } catch {
      // ignore and retry
    }
    await new Promise((r) => setTimeout(r, 2000));
  }
  return null;
}

async function main() {
  const pk = process.env.PRIVATE_KEY_DEVNET;
  const rpc = process.env.RPC_URL_DEVNET;
  if (!pk || !rpc) throw new Error('Missing PRIVATE_KEY_DEVNET or RPC_URL_DEVNET');

  const irys = await Uploader(Arbitrum).withWallet(pk).withRpc(rpc).devnet();

  console.log('\n─── Mutability Test ───');

  console.log('\n  [1/3] Uploading V1 as root...');
  const r1 = await irys.upload(V1, {
    tags: [{ name: 'Content-Type', value: 'text/plain' }],
  });
  const rootId = r1.id;
  console.log(`  Root TX: ${rootId}`);

  console.log('\n  [2/3] Uploading V2 tagged Root-TX...');
  const r2 = await irys.upload(V2, {
    tags: [
      { name: 'Content-Type', value: 'text/plain' },
      { name: 'Root-TX', value: rootId },
    ],
  });
  console.log(`  Update TX: ${r2.id}`);

  console.log('\n  [3/3] Fetching mutable URL...');
  const served = await fetchMutable(rootId);
  if (!served) {
    console.log(`  FAIL: mutable URL did not serve any content within timeout`);
    console.log(`  Check manually: https://gateway.irys.xyz/mutable/${rootId}`);
    process.exit(1);
  }

  console.log(`  Served content:\n    ${served.trim()}`);
  if (served === V2) {
    console.log('\n  PASS: mutable URL serves the latest version');
  } else if (served === V1) {
    console.log('\n  PARTIAL: mutable URL serves V1 (indexing may take longer)');
    console.log(`  Recheck: https://gateway.irys.xyz/mutable/${rootId}`);
  } else {
    console.log('\n  UNEXPECTED: served content does not match V1 or V2');
    process.exit(1);
  }
}

main().catch((err) => {
  console.error(`\n  ERROR: ${err instanceof Error ? err.message : String(err)}`);
  process.exit(1);
});
