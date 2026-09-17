---
id: domains-index
title: Domains
type: index
status: stable
---

# Domains

Sovereign Account is organized into bounded contexts ("domains"). Each domain is a self-contained conceptual unit with its own vocabulary, laws, and formalizations.

---

## Active Domains

| Domain | Description |
|--------|-------------|
| [Account](account/index.md) | Sovereign identity, authority derivation, and execution decision model. |

## Principles Across Domains

- **Bounded context isolation.** Each domain defines its own vocabulary.
- **No infrastructure leaks.** Domains describe semantics, not implementations.
- **Formal laws are first-class.** Every domain exposes a set of invariants.
- **Cross-domain references** use stable `ref:` identifiers.

