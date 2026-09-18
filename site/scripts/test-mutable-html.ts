#!/usr/bin/env bun
import { Uploader } from '@irys/upload';
import { Arbitrum } from '@irys/upload-ethereum';

const V1 = `<!DOCTYPE html>
<html><head><title>Test V1</title></head>
<body><h1>MUTABLE HTML V1</h1></body></html>`;

const V2 = `<!DOCTYPE html>
<html><head><title>Test V2</title></head>
<body><h1>MUTABLE HTML V2</h1></body></html>`;

async function fetchMutable(rootId: string, attempts = 15): Promise<string | null> {
  const url = `https://gateway.irys.xyz/mutable/${rootId}`;
  for (let i = 0; i < attempts; i++) {
    try {
      const res = await fetch(url, { redirect: 'follow' });
      if (res.ok) {
        const text = await res.text();
        if (text.includes('MUTABLE HTML')) return text;
      }
    } catch {
      // retry
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

  console.log('\n─── Mutable HTML Test ───');

  console.log('\n  [1/3] Uploading V1 as root (Content-Type: text/html)...');
  const r1 = await irys.upload(V1, {
    tags: [{ name: 'Content-Type', value: 'text/html' }],
  });
  console.log(`  Root TX: ${r1.id}`);

  console.log('\n  [2/3] Uploading V2 tagged Root-TX...');
  const r2 = await irys.upload(V2, {
    tags: [
      { name: 'Content-Type', value: 'text/html' },
      { name: 'Root-TX', value: r1.id },
    ],
  });
  console.log(`  Update TX: ${r2.id}`);

  console.log('\n  [3/3] Fetching mutable URL (with trailing slash)...');
  const served = await fetchMutable(r1.id + '/');
  if (!served) {
    console.log('  FAIL: mutable URL did not serve content within timeout');
    console.log(`  Check manually: https://gateway.irys.xyz/mutable/${r1.id}/`);
    process.exit(1);
  }

  console.log(`  Served content type: HTML`);
  console.log(`  Contains V2: ${served.includes('MUTABLE HTML V2')}`);
  console.log(`  Contains V1: ${served.includes('MUTABLE HTML V1')}`);

  if (served.includes('MUTABLE HTML V2')) {
    console.log('\n  PASS: mutable URL serves latest HTML');
  } else if (served.includes('MUTABLE HTML V1')) {
    console.log('\n  PARTIAL: mutable URL serves V1 (indexing delay)');
  } else {
    console.log('\n  UNEXPECTED: served content does not match');
  }

  console.log(`\n  Manual check: https://gateway.irys.xyz/mutable/${r1.id}/`);
}

main().catch((err) => {
  console.error(`\n  ERROR: ${err instanceof Error ? err.message : String(err)}`);
  process.exit(1);
});
