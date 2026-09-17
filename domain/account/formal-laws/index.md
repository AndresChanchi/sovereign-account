---
id: account-formal-laws-index
title: Formal Laws
domain: sovereign-account
type: index
status: stable
---

# Formal Laws

The formal invariants of the Account domain, grouped as `D1`–`D6`.

Each file is a self-contained set of laws. Identifiers (`D1.1`, `D1.2`, …) are stable and referenced from the vocabulary and from the Dafny sources.

---

## Laws

| ID | Title | Scope |
|----|-------|-------|
| [D1](D1-delegation-and-authority.md) | Delegation and Authority | Sovereignty, delegation, derived authority, provenance, monotonicity. |
| [D2](D2-policy-effect-and-state.md) | Policy Effect and State | Policy effect algebra, ordered sequences, atomic consumption, contradiction. |
| [D3](D3-entity-identity-and-recognition.md) | Entity Identity and Recognition | Identity immutability, recognition persistence, per-entity lifecycles. |
| [D4](D4-proof-and-authorization-equality.md) | Proof and Authorization Equality | Semantic value of Authorization, exclusion of Proof, Replay Protection. |
| [D5](D5-restriction-association.md) | Restriction Association | Restriction–Capability–Scope mapping, `ModifyRestriction` semantics. |
| [D6](D6-authorization-and-environment.md) | Authorization and Environment | Chain as semantic context, environment-independent acceptance, compatibility at materialization. |

---

## Independence

Each law set is independent of:

- Dafny
- Rust
- EVM
- blockchain addresses
- cryptographic primitives
- storage representations
- implementation-specific data structures

The implementation **must demonstrate** these laws, not redefine them.

---

## Rule

When Dafny finds an ambiguity:

```text
Dafny ambiguity
      ↓
DDD clarification
      ↓
formal law
      ↓
proof
```

Never resolve an ambiguity by introducing a convenient structure in Dafny.

---

## Cross-Reference

Laws reference the vocabulary via `ref:` and cite the original section numbers:

- §1–§6, §44 — Foundations
- §7–§14 — Authority Model
- §15–§19, §42 — Authorization
- §20–§24, §45 — Derived Authority
- §25–§29 — State and Policy
- §30–§43 — Execution Model
- §46–§53 — Reference
