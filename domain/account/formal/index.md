---
id: account-formal-index
title: Formal Verification
domain: sovereign-account
type: index
status: stable
---

# Formal Verification

Dafny sources that implement the formal laws of the Account domain.

The `.dfy` files live in the [sovereign-account](https://github.com/AndresChanchi/sovereign-account) repository, under `formal/`. This page is the entry point into those sources.

---

## Module Map

| Module | Purpose | Source |
|--------|---------|--------|
| `foundation/` | Domain primitives: Capability, Identity, Scope, Restriction, Subject, Chain, DomainAction, ExecutionTarget. | [formal/foundation](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/foundation) |
| `authority/` | Credential, CredentialAuthority, Session, Delegation, EffectiveAuthority. | [formal/authority](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/authority) |
| `authorization/` | Authorization, AuthorizationValidation, AuthorizationAcceptanceTransition, Replay. | [formal/authorization](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/authorization) |
| `account/` | Account, AccountTransitions. | [formal/account](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/account) |
| `policy/` | Policy, PolicyEffect, PolicyConsumption, AuthorizationStateTransition. | [formal/policy](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/policy) |
| `execution/` | DomainAction, ExecutionRequest, ExecutionContext, ExecutionConstraints, ExecutionSemantics. | [formal/execution](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/execution) |
| `laws/` | Formal laws D1–D6 encoded as lemmas. | [formal/laws](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/laws) |
| `proofs/isolated/` | Proofs per module, isolated. | [formal/proofs/isolated](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/proofs/isolated) |
| `proofs/scenarios/` | End-to-end proofs per consumer context (Recovery, DeFi, EIP-7702, etc). | [formal/proofs/scenarios](https://github.com/AndresChanchi/sovereign-account/tree/main/formal/proofs/scenarios) |

---

## Running

```bash
dafny verify formal/**/*.dfy


---

## Formal Module Diagrams

### Foundation

<iframe
  src="/architecture/index.html?diagram=Foundation"
  style="width: 100%; height: 500px; border: 0; border-radius: 0.5rem;"
  title="Foundation — Dafny modules"
  loading="lazy"
></iframe>

### Authority

<iframe
  src="/architecture/index.html?diagram=Authority"
  style="width: 100%; height: 500px; border: 0; border-radius: 0.5rem;"
  title="Authority — Dafny modules"
  loading="lazy"
></iframe>
