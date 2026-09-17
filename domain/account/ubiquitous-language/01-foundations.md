---
id: account-ul-foundations
title: Foundations
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "01"
type: concept
status: stable
section: "1-6, 44"
entities: [Identity, Account]
value-objects: [Subject]
depends-on: []
related:
  - account-ul-authority-model
  - account-ul-derived-authority
formal-laws:
  - D1.1
  - D1.2
  - D1.3
  - D1.4
  - D3.1
  - D3.2
verification:
  dafny:
    - domain/account/formal/foundation/Identity.dfy
    - domain/account/formal/foundation/Subject.dfy
    - domain/account/formal/account/Account.dfy
---

# Foundations

> Original DDD sections: §1–§6, §44.

This chapter establishes the primitive concepts of the Account domain: **Subject**, **Identity**, and **Account**, along with the fundamental distinctions between them and the environments they interact with.

---

## Purpose of the domain

The Account domain defines a primitive for **sovereign identity, authority, and execution**.

Its purpose is to let an application determine:

1. Which sovereign `Identity` is exercising authority.
2. What authority exists for that identity.
3. Which subject or mechanism may exercise that authority.
4. Under what conditions it may be exercised.
5. Which action the consuming context requests.
6. Whether the exercise is authorized.
7. How an authorized decision is materialized.

The domain **does not define the meaning of business actions** of the consuming context.

The same model applies to applications working with:

- persons;
- organizations;
- enterprises;
- agents;
- services;
- devices;
- other subjects.

The domain is designed around a shared verifiable environment because the sovereignty represented by `Identity` requires a shared context where states and transitions can be verified without depending exclusively on a central authority.

However, the domain is **not defined by**:

- any specific blockchain;
- any specific virtual machine;
- any specific account-abstraction standard;
- a specific cryptographic curve;
- a specific wallet;
- a specific address representation;
- any other concrete implementation.

---

## External Identity and Authentication Context

An application may rely on identities and authentication mechanisms **external** to the Account domain.

Examples:

```text
email
phone
passkey
WebAuthn credential
OAuth identity
EOA
hardware authenticator
```

These elements belong to the external context and **are not automatically an `Identity`**.

Therefore:

```text
External Identity
    !=
Identity

Authentication Mechanism
    !=
Identity

Blockchain Address
    !=
Identity
```

An external mechanism may produce evidence that is later recognized by a `Credential`.

The conceptual relationship is:

```text
External Identity / Authentication
        ↓
authentication evidence
        ↓
Credential
        ↓
Account recognition
        ↓
authority exercise
        ↓
blockchain execution
```

The loss or replacement of an external authentication mechanism **does not by itself imply the loss of the `Identity`**.

This allows an `Identity` to maintain sovereign continuity even when the mechanisms through which it is exercised change.

---

## Subject

A **Subject** represents the semantic actor to which an `Identity` is attributed.

It may represent:

- a person;
- an organization;
- an enterprise;
- an agent;
- a service;
- a device;
- another actor recognized by the consuming context.

`Subject` is not:

- an `Identity`;
- a `Credential`;
- an `Account`;
- a blockchain address;
- an authentication mechanism.

The distinction is:

```text
Subject
    =
semantic actor

Identity
    =
sovereign continuity within the domain
```

The same `Subject` may be associated with multiple `Identity` instances:

```text
Subject
    ├── Identity A
    ├── Identity B
    └── Identity C
```

This allows, for example, a person to have distinct sovereign identities for different contexts, or an organization to have different sovereign continuities for different functions.

### Cardinality

```text
Subject
    1
    │
    └──── 0..N
           Identities
```

Each `Identity` is attributed to exactly one `Subject`.

Within the domain, `Subject` is a **Value Object / semantic actor descriptor**.

---

## Identity

An **Identity** represents a sovereign continuity recognized by the domain through which a `Subject` may exercise authority.

`Identity` is an **Entity**.

The identity of an `Identity` is independent of:

- Credentials;
- Accounts;
- cryptographic mechanisms;
- blockchain addresses;
- account-abstraction standards;
- technical representations.

An `Identity` may control multiple `Account` instances:

```text
Identity A
    ├── Account A
    ├── Account B
    └── Account C
```

An `Identity` may also use multiple `Credential` instances.

Changing, adding, suspending, revoking, or recovering a `Credential` **does not automatically create a new `Identity`**.

Creating or removing an `Account` also does not automatically create a new `Identity`.

Changing the blockchain representation of an `Account` also does not change the `Identity`.

### Identity Identifier

An `Identity` has its own stable individual identity:

```text
IdentityId
```

### Cardinality

```text
Subject
    1
    │
    └──── 0..N
           Identity
```

---

## Account

An **Account** is the operational component through which an `Identity` exercises authority.

`Account` is an **Entity**.

An `Account` has its own identity:

```text
AccountId
```

`AccountId` is different from a blockchain address.

The `Account` maintains the operational state necessary to determine which exercises of authority may produce a valid execution.

That state may include:

- `Capability` instances;
- `Credential` instances;
- `CredentialAuthority` relations;
- `Session` instances;
- `Delegation` instances;
- recognized `PolicyEffect` instances;
- relationships between these elements.

### Cardinality Identity ↔ Account

The sovereign relationship is:

```text
Identity
    1
    │
    └──── 0..N
           Accounts
```

Each `Account` has **exactly one sovereign `Identity`**.

Therefore:

```text
Account
    └── sovereign Identity = exactly one
```

A second `Identity` may receive authority to operate an `Account` through:

- a `Credential`;
- a `Delegation`;
- a `Session`;
- other recognized mechanisms.

This does not convert the second `Identity` into sovereign of that `Account`.

Therefore:

```text
Identity A ─────► Account X
     sovereign control

Identity B ─────► Account X
     authorized exercise
```

does not mean:

```text
Identity A ──┐
             ├──► Account X
Identity B ──┘
    co-sovereignty
```

The domain **does not permit multiple sovereign `Identity` instances over the same `Account`**.

Creating another `Account` creates another Entity even if it belongs to the same `Identity`.

---

## Account Identity Representation

An `Account` may have a technical representation on a specific blockchain.

For example:

```text
Account
    ↓
technical representation
    ↓
Address
```

Therefore:

```text
Account
    !=
Blockchain Address
```

A blockchain address represents how an `Account` is technically materialized.

It does not represent:

- the `Identity`;
- the sovereignty;
- necessarily the `AccountId`.

Changing between compatible materialization mechanisms does not automatically create another `Account` while the same Entity lifecycle continues.

### Principle

```text
Identity
    ≠
Account
    ≠
Blockchain Address
```

---

## Identity Sovereignty

**Identity Sovereignty** means the sovereign continuity of an `Identity` does not depend on the concrete mechanism through which it is authenticated, exercised, delegated, or materialized.

This implies:

- changing `Credential` instances does not automatically change the `Identity`;
- changing `Account` instances does not automatically change the `Identity`;
- changing the blockchain representation does not automatically change the `Identity`;
- a `Delegation` does not automatically transfer sovereignty;
- a `Sponsor` does not acquire authority by paying;
- a cryptographic mechanism does not define the `Identity`;
- an address does not define the `Identity`.

Sovereignty belongs to `Identity`.

---

## Cross-References

- [Authority Model](02-authority-model.md) — how `Authority` and `Credential` are defined.
- [Derived Authority](04-derived-authority.md) — how `Session` and `Delegation` relate to `Identity`.
- [Formal Laws — D1](../formal-laws/D1-delegation-and-authority.md) — sovereignty invariants.
- [Formal Laws — D3](../formal-laws/D3-entity-identity-and-recognition.md) — identity immutability.
