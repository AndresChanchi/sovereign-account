---
id: docs-index
title: Sovereign Account
type: overview
status: stable
---

# Sovereign Account

A universal primitive for **sovereign identity, authority, and execution**.

Sovereign Account defines a formal model that lets an application determine:

1. Which sovereign `Identity` is exercising authority.
2. What authority exists for that identity.
3. Which subject or mechanism may exercise that authority.
4. Under what conditions it may be exercised.
5. Which action the consuming context requests.
6. Whether that exercise is authorized.
7. How an authorized decision is materialized.

The domain is **implementation-agnostic**. It does not depend on Ethereum, EVM, Stylus, Solidity, a specific curve, or a specific account-abstraction standard.

---

## Domains

| Domain | Status |
|--------|--------|
| [Account](domain/account/index.md) | Stable |

---

## Key Ideas

- **Identity is sovereign continuity**, not a credential.
- **Account is an operational component**, not a blockchain address.
- **Authority is derived**, contextual, and monotonic downward.
- **Delegation never transfers sovereignty**.
- **Proof and Authorization are different layers**.
- **Execution materializes a decision already validated**.

---

## Structure

- **Ubiquitous Language** — the conceptual vocabulary.
- **Formal Laws** — the invariants (`D1`–`D6`).
- **Formal Verification** — Dafny sources.

Start with [the Account domain](domain/account/index.md).

---

## Machine-readable

- [`llms.txt`](llms.txt) — LLM-oriented index.
- [`_data/manifest.json`](../dist/_data/manifest.json) — build output (generated).

