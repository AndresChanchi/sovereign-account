---
id: account-ul-authority-model
title: Authority Model
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "02"
type: concept
status: stable
section: "7-14"
entities: [Credential]
value-objects: [Authority, Capability, CapabilityKind, Scope, CredentialAuthority]
depends-on:
  - account-ul-foundations
related:
  - account-ul-authorization
  - account-ul-derived-authority
formal-laws:
  - D1.5
  - D1.6
  - D1.7
  - D1.8
  - D1.9
  - D1.10
verification:
  dafny:
    - domain/account/formal/foundation/Capability.dfy
    - domain/account/formal/foundation/Scope.dfy
    - domain/account/formal/authority/Credential.dfy
    - domain/account/formal/authority/CredentialAuthority.dfy
---

# Authority Model

> Original DDD sections: §7–§14.

This chapter defines the concepts that describe **what authority exists**, **who can exercise it**, and **which sources may produce authorization evidence**.

---

## Authority

**Authority** represents the effective faculty of exercising specific `Capability` instances.

`Authority` is not necessarily a persistent Entity.

It is a semantic relation derived from:

- available `Capability` instances;
- `CredentialAuthority`;
- `Session` instances;
- `Delegation` instances;
- `Restriction` instances;
- `PolicyEffect` instances;
- `Scope`;
- temporal conditions;
- context;
- other currently valid conditions.

`Authority` answers:

> **what can be exercised, by whom, and under which conditions.**

`Authority` does not define how cryptographic evidence is produced.

---

## Capability

A **Capability** represents a faculty that may be exercised through an `Identity` or derived authority.

`Capability` expresses:

> **what faculty exists.**

Examples:

```text
Upload
Delete
Share
Transfer
Approve
Delegate
Manage
```

The specific meaning of a `DomainAction` still belongs to the consuming context.

A `Capability` may be limited by:

- `Scope`;
- `Restriction`;
- `Session`;
- `Delegation`;
- temporal conditions;
- context;
- `PolicyEffect`;
- other recognized rules.

### Capability as a Value Object

`Capability` is a **Value Object**.

Two `Capability` instances with exactly the same semantic content represent the same faculty:

```text
Capability A
    ==
Capability B
```

There is no separate individual identity for:

```text
Upload(Album123)
```

Therefore `Capability` **does not need a `CapabilityId`**.

This means:

```text
Capability
    =
faculty

Credential / Delegation / Authorization
    =
mechanisms through which that faculty may be exercised
```

Revoking a `Credential` does not revoke the semantic existence of the `Capability`; it only prevents that `Credential` from continuing to use it.

### Principle

> **A `Capability` is a faculty by value, not an individual historical grant.**

---

## Capability Kind

A **Capability Kind** identifies the semantic class of a `Capability`.

Examples:

```text
Upload
Delete
Share
Transfer
Approve
```

`CapabilityKind`:

- does not grant authority;
- does not represent an execution;
- does not identify a concrete `Capability`.

`CapabilityKind` is a **Value Object**.

Its meaning depends on its values.

### Special CapabilityKind: Delegate

`Delegate` is a **special semantic `CapabilityKind`** of the domain.

Its semantics: **authorizes the act of delegating other Capabilities**.

It is not a consumer-defined `Capability`; it is inherent to the authorization model.

The presence of `Delegate` in the set of `Capability` instances of an `Identity` means that the `Identity` may confer authority to other subjects through a `Delegation`.

**Associated law:**  
For an `Identity` to be the `Source` of a `Delegation`, it must effectively possess the `Delegate` capability over the relevant context (Account, Scope, etc.).

See [Delegation](../formal-laws/D1-delegation-and-authority.md) for the formal treatment.

---

## Capability Scope

The **Capability Scope** determines the set of resources, objects, subjects, or domains over which a `Capability` may be exercised.

`Scope` expresses:

> **over what the faculty may be exercised.**

Example:

```text
Capability:
    Upload

Scope:
    Album #123
```

`Scope` is a **Value Object**.

It must not be confused with `ExecutionTarget`:

```text
Capability Scope
    =
where authority exists

Execution Target
    =
where execution is technically materialized
```

---

## Capability Metadata

**Capability Metadata** contains descriptive information associated with a `Capability`.

It may be used for:

- presentation;
- classification;
- organization;
- discovery;
- UX;
- integration.

Metadata does not modify authority by itself.

It may be:

- public;
- private;
- encrypted;
- protected by zero-knowledge proofs;
- other mechanisms.

The `Account` does not determine the privacy mechanism.

### Invariant

> **Modifying metadata does not modify the authority represented by a `Capability`.**

---

## Credential

A **Credential** is a source recognized by an `Account` through which evidence may be produced to exercise authority.

The `Credential` does not identify the `Identity`. A `Credential` is a recognized source through which authority associated with an `Identity` may be exercised. Therefore, any deterministic mechanism that derives an `Account` must be based on the continuity of the sovereign `Identity`, or on a deterministic resolution toward it, and not directly on a `Credential` that may be replaced during the lifecycle of the `Identity`.

`Credential` is an **Entity**.

A `Credential` has individual continuity throughout its lifecycle:

```text
active
→ suspended
→ reactivated
→ revoked
```

Revoking a `Credential` does not create another `Credential`.

A `Credential` may be based on:

- passkey;
- WebAuthn;
- secp256k1;
- P-256;
- hardware;
- multisig;
- threshold cryptography;
- future mechanisms;
- other mechanisms.

The domain does not identify `Credential` with any of those mechanisms.

### Credential ≠ Identity

A `Credential` does not represent sovereignty.

A `Credential` does not become an `Identity` by producing a `Proof`.

### Credential ≠ Account

A `Credential` is also not an `Account`.

It is a recognized source through which certain capabilities over one or more `Account` instances may be exercised.

---

## Credential ↔ Account Recognition

The relationship between `Credential` and `Account` is **contextual and explicit in `AuthorizationState`**.

A `Credential` may be recognized by one or more `Account` instances.

Therefore:

```text
Credential A
    ├── recognized by Account A
    ├── recognized by Account B
    └── recognized by Account C
```

Recognition is independent for each `Account`.

A `Credential` recognized by an `Account` does not acquire sovereignty over it.

Nor does it imply that the `Account` instances share sovereignty.

The semantic relationship is:

```text
Credential
    │
    ├── Account A → Credential Authority A
    ├── Account B → Credential Authority B
    └── Account C → Credential Authority C
```

Therefore, the same `Credential` may have different authority depending on the `Account` in which it is recognized.

### Credential ↔ Identity

The sovereign `Identity` belongs to the `Account`.

A `Credential` may be used as a mechanism to exercise authority associated with the sovereign `Identity` of the `Account` or derived authority recognized by it.

The `Credential` **does not need to contain the `Identity` as part of its structural identity**.

This preserves the separation:

```text
Identity
    =
sovereign continuity

Credential
    =
recognized authority-exercise source

Account
    =
operational authority state
```

---

## Credential Authority

**Credential Authority** represents the set of `Capability` instances a `Credential` may attempt to exercise within a specific `Account`.

`Credential Authority` is a **Value Object / authority relation value**.

Two `Credential` instances may have the same `CredentialAuthority`:

```text
CredentialAuthority(A)
    ==
CredentialAuthority(B)
```

without being the same `Credential`.

### Invariant

```text
Requested Capability
    ∈
Credential Authority
```

is a necessary condition for a `Credential` to attempt to exercise that `Capability`.

It is not sufficient for the authority to be effective.

`Session` instances, `Delegation` instances, `Restriction` instances, policies, and other conditions may reduce it afterward.

### Principle

> **`CredentialAuthority` describes what a `Credential` may attempt to exercise on an `Account`; it does not grant sovereignty.**

---

## Cross-References

- [Foundations](01-foundations.md) — `Identity`, `Account`, and sovereignty.
- [Authorization](03-authorization.md) — how `Authorization` and `Proof` connect to `Credential`.
- [Derived Authority](04-derived-authority.md) — `Session`, `Delegation`, and `EffectiveAuthority`.
- [Formal Laws — D1](../formal-laws/D1-delegation-and-authority.md) — authority origin and derivation invariants.
