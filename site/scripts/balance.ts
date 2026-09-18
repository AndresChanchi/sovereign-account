#!/usr/bin/env bun
import { Uploader } from '@irys/upload';
import { Arbitrum } from '@irys/upload-ethereum';
import { ethers } from 'ethers';

type Network = 'devnet' | 'mainnet';

function parseArgs(): Network {
  const args = process.argv.slice(2);
  return args.includes('--mainnet') ? 'mainnet' : 'devnet';
}

function toBigInt(value: unknown): bigint {
  if (value == null) throw new Error('toBigInt: null/undefined value');
  if (typeof value === 'bigint') return value;
  if (typeof value === 'string') return BigInt(value);
  if (typeof value === 'number') return BigInt(Math.trunc(value));
  return BigInt((value as { toString(): string }).toString());
}

async function main() {
  const network = parseArgs();
  const suffix = network.toUpperCase();
  const pk = process.env[`PRIVATE_KEY_${suffix}`];
  const rpc = process.env[`RPC_URL_${suffix}`];

  if (!pk || !rpc) {
    throw new Error(`Missing PRIVATE_KEY_${suffix} or RPC_URL_${suffix}`);
  }

  const wallet = new ethers.Wallet(pk);
  const provider = new ethers.JsonRpcProvider(rpc);

  const builder = Uploader(Arbitrum).withWallet(pk).withRpc(rpc);
  const irys = network === 'devnet' ? await builder.devnet() : await builder;

  const [walletBal, irysBal] = await Promise.all([
    provider.getBalance(wallet.address),
    irys.getBalance(),
  ]);

  console.log(`\n─── Balance (${network}) ───`);
  console.log(`  Wallet       ${wallet.address}`);
  console.log(`  Wallet ETH   ${ethers.formatEther(toBigInt(walletBal))}`);
  console.log(`  Irys credit  ${ethers.formatEther(toBigInt(irysBal))}`);
}

main().catch((e) => {
  console.error(e.message ?? e);
  process.exit(1);
});
