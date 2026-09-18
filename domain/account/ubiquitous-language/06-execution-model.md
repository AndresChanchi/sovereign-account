---
id: account-ul-execution-model
title: Execution Model
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "06"
type: concept
status: stable
section: "30-43"
value-objects: [DomainAction, ExecutionRequest, ExecutionTarget, ExecutionContext, ExecutionConstraints]
depends-on:
  - account-ul-foundations
  - account-ul-authorization
  - account-ul-derived-authority
  - account-ul-state-and-policy
related:
  - account-ul-reference
formal-laws:
  - D1.46
  - D1.47
  - D1.48
  - D1.49
  - D6.1
  - D6.2
  - D6.3
  - D6.4
  - D6.5
  - D6.6
  - D6.7
  - D6.8
  - D6.9
  - D6.10
  - D6.11
  - D6.12
  - D6.13
verification:
  dafny:
    - domain/account/formal/execution/DomainAction.dfy
    - domain/account/formal/execution/ExecutionRequest.dfy
    - domain/account/formal/execution/ExecutionContext.dfy
    - domain/account/formal/execution/ExecutionConstraints.dfy
    - domain/account/formal/execution/ExecutionSemantics.dfy
---

# Execution Model

> Original DDD sections: §30–§43.

This chapter defines how an authorized decision is materialized, the components that orchestrate the process, and the boundary between the domain and infrastructure.

---

## Domain Action

A **Domain Action** represents a concrete intent of the consuming context.

Examples:

```text
UploadPhoto
DeletePhoto
TransferFunds
ApprovePayroll
CreateInvoice
ShareAlbum
```

The domain does not define:

- the catalog;
- the meaning;
- the lifecycle;
- the internal identity;
- the business rules.

`DomainAction` is an **external Value Object**.

The domain only needs a sufficient representation to determine whether authority exists to execute the action.

---

## Execution Request

An **Execution Request** represents a request to materialize a `DomainAction`.

It may contain:

- `DomainAction`;
- `Authorization`;
- `RequestedAuthority`;
- `ExecutionTarget`;
- `ExecutionConstraints`;
- context necessary for the `Runtime`.

`ExecutionRequest` is a **Value Object / request value**.

It does not represent a materialized `Execution`.

---

## Execution Target

An **Execution Target** identifies the technical destination on which the execution will be materialized.

It may represent:

- a contract;
- a resource;
- a service;
- an infrastructure;
- another compatible destination.

It does not define `Authority`.

The difference is:

```text
Capability Scope
    =
where authority exists

Execution Target
    =
where execution is materialized
```

---

## Execution Context

An **Execution Context** represents the complete and validated decision required for an execution to be materialized.

It may incorporate the results of:

- `Authorization`;
- `EffectiveAuthority`;
- `AuthorizationState`;
- `PolicyEffect` instances;
- `Restriction` instances;
- `ExecutionConstraints`;
- `DomainAction`;
- `ExecutionTarget`;
- other relevant conditions.

A valid `ExecutionContext` means:

> **the required authority and necessary conditions have been evaluated, and execution may proceed under that context.**

`ExecutionContext` is a **Value Object / complete execution decision**.

The `Execution Engine` receives only valid contexts.

It does not re-decide `Authority`.

---

## Execution Constraints

**Execution Constraints** represent conditions that must be satisfied to materialize an `ExecutionContext`.

They may include:

- limits;
- temporality;
- atomicity;
- operational limits;
- materialization conditions;
- other recognized restrictions.

`Execution Constraints` do not create `Authority`.

They only condition the materialization of an authorized decision.

---

## Runtime

The **Runtime** is the transient component that **orchestrates the distributed evaluation and operational materialization** of a request.

The `Runtime`:

1. receives an `ExecutionRequest`;
2. coordinates retrieval of the relevant `AuthorizationState`;
3. coordinates `Authorization` validation;
4. coordinates `EffectiveAuthority` determination;
5. coordinates the application of relevant `PolicyEffect` and `Restriction` instances;
6. verifies `ExecutionConstraints`;
7. builds the `ExecutionContext`;
8. delivers only valid contexts to the `Execution Engine`.

### Important

The `Runtime` **is not the owner of all authority rules**.

Evaluation may be distributed across bounded contexts or specialized modules, for example:

```text
Authentication / Credential Verification
Authorization
Recovery
Access
Registry
Economics
Execution Gateway
```

The `Runtime` coordinates those results.

Therefore:

```text
Runtime
    =
Authority / Execution Orchestration
```

not:

```text
Runtime
    =
owner of every authorization rule
```

---

## Execution Engine

The **Execution Engine** receives a valid `ExecutionContext` and transforms that decision into materializable operations.

Its conceptual input is:

```text
Execution Context
```

It does not:

- interpret `Credential` instances;
- decide `Authority`;
- create `Authorization`;
- define `DomainAction` instances;
- redefine `Policy`;
- implement cryptographic rules.

Its responsibility is:

> **to materialize a decision already taken.**

It may produce one or multiple operations when the infrastructure supports:

- batching;
- multicall;
- atomicity;
- other equivalent mechanisms.

---

## Execution

An **Execution** represents the concrete process/materialization of an authorized action.

It is not currently an Entity of the `Account` domain.

The architecture distinguishes:

```text
Execution Request
        ↓
Execution Context
        ↓
Runtime orchestration
        ↓
Execution Engine
        ↓
Adapter
        ↓
physical execution
```

An execution may consist of:

```text
one operation
```

or:

```text
multiple operations
```

The infrastructure determines how the following are guaranteed:

- atomicity;
- integrity;
- replay protection;
- constraint satisfaction.

### Lifecycle

The `Runtime` may have an operational lifecycle:

```text
Validation
→ PreFlight
→ Accounting
→ Dispatch
→ Settlement
```

and results such as:

```text
Completed
Reverted
Aborted
Expired
Cancelled
Failed
```

but these represent **the lifecycle of the operational workflow**, not a persistent `Execution` Entity of the `Account` domain.

Therefore:

```text
Execution
    ≠
Execution Entity
```

and no `ExecutionId` is introduced.

---

## Adapter

An **Adapter** materializes an `Execution` on a concrete infrastructure.

Examples:

- EIP-7702;
- ERC-4337;
- RIP-7560;
- future forms of account abstraction;
- other compatible infrastructures.

The `Adapter` transforms:

```text
Domain Execution Semantics
        ↓
Infrastructure primitives
```

and not the other way around.

### Principle

> **The domain defines what a valid execution means; the `Adapter` defines how to materialize it on a concrete infrastructure.**

---

## Blockchain

The **Blockchain** is a fundamental part of the context.

It provides the shared environment where `Account` instances can:

- exercise authority;
- produce verifiable changes;
- materialize executions.

The domain does not abstract the existence of a blockchain.

It abstracts the differences between concrete infrastructures used to operate on it.

```text
Blockchain
    ≠
Ethereum
    ≠
EVM
    ≠
Arbitrum
    ≠
Stylus
    ≠
EIP-7702
    ≠
ERC-4337
```

---

## Chain and Execution Environment

The `Chain` attribute in the `Authorization` Context identifies the semantic execution environment for which the authorization was issued.

It forms part of the semantic value of `Authorization`.

The **Execution Environment** is a separate concept: it is the concrete environment where a decision is materialized. It is not part of the `ExecutionContext` Value Object.

The relationship between them is expressed as a compatibility relation:

```text
compatible: Chain × ExecutionEnvironment → bool
```

An `ExecutionContext` may be intrinsically valid while remaining incompatible with a specific `ExecutionEnvironment`. Compatibility is a **materialization condition**, not a validity condition.

The formal treatment of this separation is defined in [Formal Laws — D6](../formal-laws/D6-authorization-and-environment.md).

---

## Gas Payment

**Gas Payment** represents the provision of economic resources necessary to materialize an `Execution`.

`Gas Payment` is independent of `Authority`.

An entity may pay for an `Execution` without acquiring authority over it.

```text
Authority
    ≠
Gas Payment
```

Paying does not grant authorization.

---

## Execution Sponsor

An **Execution Sponsor** provides resources to pay for an `Execution` on behalf of another subject.

The `Sponsor` does not automatically acquire `Authority`.

Concrete mechanisms may include:

- relayers;
- paymasters;
- sponsored accounts;
- native mechanisms;
- others.

Sponsorship belongs to infrastructure/economics.

---

## Authentication

**Authentication** is the process through which evidence is obtained that a `Credential` corresponds to the mechanism or subject that intends to exercise it.

`Authentication` is not equivalent to `Authorization`.

```text
Authentication
    =
who / what produced the evidence

Authorization
    =
what authority may be exercised
```

A correct authentication does not automatically grant a `Capability`.

---

## Cross-References

- [Foundations](01-foundations.md) — `Account` as the operational component.
- [Authorization](03-authorization.md) — how `Authorization` participates in `ExecutionContext`.
- [Derived Authority](04-derived-authority.md) — `EffectiveAuthority` as an input to `ExecutionContext`.
- [State and Policy](05-state-and-policy.md) — state transitions produced by `PolicyEffect`.
- [Formal Laws — D6](../formal-laws/D6-authorization-and-environment.md) — Chain semantics and environment compatibility.
