---
id: account-ul-derived-authority
title: Derived Authority
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "04"
type: concept
status: stable
section: "20-24, 45"
entities: [Session, Delegation]
value-objects: [Delegatee, Restriction, EffectiveAuthority]
depends-on:
  - account-ul-foundations
  - account-ul-authority-model
  - account-ul-authorization
related:
  - account-ul-state-and-policy
  - account-ul-execution-model
formal-laws:
  - D1.11
  - D1.12
  - D1.13
  - D1.14
  - D1.15
  - D1.16
  - D1.17
  - D1.18
  - D1.19
  - D1.20
  - D1.21
  - D1.22
  - D1.23
  - D1.24
  - D1.25
  - D1.26
  - D1.27
  - D1.28
  - D1.29
  - D1.30
  - D1.31
  - D1.32
  - D1.33
  - D1.34
  - D1.35
  - D1.36
  - D1.37
  - D1.38
  - D1.39
  - D1.40
  - D1.41
  - D1.42
  - D1.43
  - D1.44
  - D1.45
  - D1.46
  - D1.47
  - D1.48
  - D1.49
  - D1.50
  - D3.5
  - D3.6
verification:
  dafny:
    - domain/account/formal/authority/Session.dfy
    - domain/account/formal/authority/Delegation.dfy
    - domain/account/formal/foundation/Restriction.dfy
    - domain/account/formal/authority/EffectiveAuthority.dfy
---

# Derived Authority

> Original DDD sections: §20–§24, §45.

This chapter defines the mechanisms through which authority is **derived, restricted, and made effective** in a specific context: `Session`, `Delegation`, `Restriction`, and `EffectiveAuthority`.

---

## Session

A **Session** is a temporary authorization derived from a `Credential`.

`Session` is an **Entity**.

It maintains identity during its lifecycle:

```text
active
→ expired

active
→ revoked
```

Expiration or revocation does not create another `Session`.

A `Session` may limit:

- `Capability` instances;
- `Scope`;
- duration;
- frequency;
- value;
- context;
- recipients;
- other conditions.

### Invariant

```text
Session Authority
    ⊆
Credential Authority
```

A `Session` may never expand the authority of its source `Credential`.

### Effect of Credential revocation on Session

If the `Credential` stops being valid (by revocation or another cause), the `Session` **does not change its identity or its persistent state** (e.g. it remains `Active`).

However, it **stops being usable as a source of authority**. Its contribution to `EffectiveAuthority` becomes `∅` while the `Credential` is not valid.

This maintains the distinction between:

- **Entity lifecycle** (identity and persistent state);
- **Effective usability** (capacity to contribute to authority).

### Propagation rule

```text
Credential revoked
    ⇒
Session (even if Active) does not contribute to EffectiveAuthority
```

It is not necessary to change the persistent state of the `Session` to `Revoked` when the `Credential` is revoked; invalidity propagates through the authority rule.

---

## Delegation

A **Delegation** allows deriving authority toward another `Subject` under explicit conditions.

`Delegation` is an **Entity**.

It maintains identity during its lifecycle:

```text
active
→ revoked
```

Revoking a `Delegation` does not create a new `Delegation`.

It does not automatically transfer:

- `Identity`;
- sovereignty;
- ownership of the original `Capability` instances.

### Invariant

```text
Delegated Authority
    ⊆
Delegatable Authority of source
```

Delegated authority may never exceed the authority the source may legitimately delegate.

---

## Delegatee

A **Delegatee** is the `Subject` that receives derived authority through a `Delegation`.

It may represent:

- a person;
- an organization;
- an agent;
- a service;
- a device;
- an `Account`;
- another `Subject`.

`Delegatee` does not mean sovereign owner.

---

## Restriction

A **Restriction** limits the conditions under which a `Capability` may be exercised.

`Restriction` is a **Value Object**.

It does not create new `Capability` instances.

It may limit:

- quantity;
- value;
- frequency;
- time;
- recipient;
- `Scope`;
- context;
- type of operation;
- number of executions.

Example:

```text
Spend
    +
maximum_value = X
    +
valid_until = T
```

Its meaning depends on its values.

### Association with Capability and Scope

A `Restriction` is a pure Value Object. It does not contain a reference to the `Capability` or `Scope` to which it applies. That association is maintained in `AuthorizationState` (see [State and Policy](05-state-and-policy.md) and [Formal Laws — D5](../formal-laws/D5-restriction-association.md)).

---

## Effective Authority

**Effective Authority** represents the authority that may actually be exercised in a specific context.

It is a **derived, contextual, and by-value** representation.

Conceptually:

```text
Account Capabilities
        ∩
Credential Authority
        ∩
Session Authority (only if usable)
        ∩
Delegated Authority (only if usable)
        ∩
Scope Conditions
        ∩
Restrictions
        ∩
Temporal Conditions
        ∩
Applicable Policy Effects
        ↓
Effective Authority
```

### Context is an input, not part of the Value Object

`EffectiveAuthority` is a Value Object whose value is the set of `Capability` instances that may actually be exercised in a given context.

The **context** (`Account`, time, `ExecutionTarget`, etc.) is an **input parameter** for the function that derives `EffectiveAuthority`. It does not form part of the identity of the resulting Value Object.

Therefore:

- Two evaluations of `EffectiveAuthority` with different contexts may produce different values.
- The context is not stored as part of the Value Object.
- The equality of `EffectiveAuthority` is determined exclusively by the set of `Capability` instances that results from the derivation.

**Example:**

- `EffectiveAuthority(I, Account X, Context A) = {Upload, View}`
- `EffectiveAuthority(I, Account X, Context B) = {View}` (because Context B restricts authority)

Both results are distinct Value Objects because their values are different. The context is not part of the value; it is an input.

`EffectiveAuthority` does not need its own identity.

The same `Credential` may produce different `EffectiveAuthority` instances depending on:

- `Account`;
- `Session`;
- `Delegation`;
- `Scope`;
- time;
- `Restriction` instances;
- `AuthorizationState`;
- `PolicyEffect` instances;
- `ExecutionContext`.

---

## Authority Abstraction

**Authority Abstraction** is the ability to represent:

```text
who may exercise
what authority
under what conditions
```

without requiring the conceptual model to know:

- a cryptographic curve;
- a wallet;
- an authentication provider;
- a concrete blockchain;
- an account-abstraction standard;
- an address representation;
- another implementation.

`Authority Abstraction` does not mean removing blockchain from the model.

It means abstracting the concrete implementations through which authority is exercised.

---

## Cross-References

- [Authority Model](02-authority-model.md) — `Capability`, `Credential`, `CredentialAuthority`.
- [Authorization](03-authorization.md) — `RequestedAuthority` and `EffectiveAuthority`.
- [State and Policy](05-state-and-policy.md) — `AuthorizationState`, `PolicyEffect`, `PolicyConsumption`.
- [Formal Laws — D1](../formal-laws/D1-delegation-and-authority.md) — delegation and derived authority invariants.
- [Formal Laws — D5](../formal-laws/D5-restriction-association.md) — `Restriction` association semantics.
