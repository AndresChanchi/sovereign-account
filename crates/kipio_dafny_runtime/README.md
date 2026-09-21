# kipio_dafny_runtime

Runtime support for Kipio's Dafny-generated Rust code.

This crate provides the primitives that the Dafny 4.11 Rust backend emits
for the Kipio Account formal domain: `DafnyInt`, `Sequence<T>`, `Map<K,V>`,
`Set<T>`, `Multiset<T>`, `DafnyChar`, and the reference-counted object
machinery used by translated code.

It is not intended to be used directly. It is consumed by
[`kipio_account_generated`](../kipio_account_generated), which contains the
translated domain model, and transitively by any crate that depends on it.

## Origin

The source is emitted by:

```bash
dafny translate rs <anchor>.dfy \
  --output <anchor>.rs \
  --enforce-determinism \
  --include-runtime
```

The production pipeline in `tools/export_rust_production.py` renames the
package from `dafny_runtime` to `kipio_dafny_runtime` and adds the
publication metadata. The rename avoids the crates.io name collision with
the upstream Dafny runtime.

## Usage

Consuming crates alias the package back to `dafny_runtime` via Cargo's
`package` directive:

```toml
[dependencies]
dafny_runtime = { package = "kipio_dafny_runtime", version = "0.1.0" }
```

With that alias in place, the emitted `use dafny_runtime::...` paths
resolve without modification. The generated source does not need to be
post-processed.

## Features

- `small-int` — use `i128` for `DafnyInt` instead of `num::BigInt`.
  Recommended for on-chain targets where `BigInt` allocates excessively.
- `sync` — use `Arc`/`Mutex` instead of `Rc`/`RefCell`, allowing values
  to cross thread boundaries.

## Repository

Source, issues, and release notes:

<https://github.com/AndresChanchi/sovereign-account>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../../LICENSE-MIT))

at your option.
