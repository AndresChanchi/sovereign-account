---
id: account-law-D6
title: D6 — Authorization and Environment
domain: sovereign-account
chapter: formal-laws
type: law-set
status: stable
law-id: D6
laws:
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
depends-on:
  - account-ul-authorization
  - account-ul-execution-model
related:
  - account-law-D1
  - account-law-D4
formal-laws:
  - D1.32
  - D1.46
  - D1.47
  - D4.3
  - D4.4
verification:
  dafny:
    - domain/account/formal/execution/ExecutionContext.dfy
    - domain/account/formal/execution/ExecutionSemantics.dfy
    - domain/account/formal/authorization/Authorization.dfy
---

# D6 — Authorization and Environment

This law set establishes an explicit architectural decision about the relationship between `Authorization`, `Chain`, and `ExecutionEnvironment`.

Its purpose is to prevent an incorrect interpretation of the formalization in which the absence of an `expectedChain` inside `Authorization Validation` might be considered an omission or a gap.

The architecture defined here is **intentional**.

---

## Independence Laws

### D6.1 Authorization Acceptance Is Independent of the Execution Environment

`Authorization Acceptance` determines whether an `Authorization` may be accepted by an `Account` according to the semantic authorization rules of the domain.

Conceptually:

```text
Authorization
      +
Account
      +
Authorization State
      +
Effective Authority
      +
Authorization Validation conditions
      ↓
Accepted Authorization
```

`Authorization Acceptance` **does not require selecting a concrete `ExecutionEnvironment` in advance**.

Therefore:

```text
Authorization Acceptance
    ≠
Environment-specific Execution Delivery
```

Acceptance answers:

> **Does this `Authorization` satisfy the semantic conditions necessary to be accepted by this `Account`?**

It does not answer:

> **Can this `Authorization` be physically materialized through this concrete execution environment?**

---

## Semantic Chain Laws

### D6.2 Chain Remains Part of Authorization

Independence from the `ExecutionEnvironment` does not remove or weaken the semantics of `Chain`.

`Chain` forms part of the semantic context of `Authorization`.

Therefore:

```text
Authorization A
    Chain = ChainA

Authorization B
    Chain = ChainB
```

are semantically different `Authorization` instances when:

```text
ChainA != ChainB
```

`Chain` participates in the semantic equality of `Authorization`.

Therefore:

```text
Chain
    =
semantic Authorization context
```

and not:

```text
Chain
    =
mere infrastructure metadata
```

### D6.3 Semantic Chain ≠ Selected Execution Environment

The `Chain` contained in an `Authorization` represents the semantic context of that authorized intent.

The `ExecutionEnvironment` represents the concrete environment through which an already-validated decision will be materialized.

They are different concepts:

```text
Authorization.Chain
    =
semantic authorization context

ExecutionEnvironment
    =
selected materialization environment
```

The existence of a `Chain` inside `Authorization` does not imply that `Authorization Validation` must receive an independent parameter such as:

```text
expectedChain : Chain
```

merely to relate the authorization to a selected infrastructure.

That parameter does not form part of the semantics necessary for environment-independent acceptance.

---

## Compatibility Laws

### D6.4 Environment Compatibility Belongs to Materialization

Compatibility between:

```text
Authorization Chain
```

and:

```text
ExecutionEnvironment
```

is evaluated at the execution boundary.

Conceptually:

```text
Authorization
      │
      │ Chain
      ▼
Execution Context
      │
      ▼
Selected Execution Environment
      │
      ▼
Chain × Environment Compatibility
```

The compatibility decision may depend on external infrastructure and on the concrete mechanisms available to materialize the execution.

Therefore, compatibility is expressed as a contextual relation:

```text
CompatibleChainEnvironment
    ⊆
Chain × ExecutionEnvironment
```

The domain does not fix a concrete implementation of that relation.

### D6.5 Intrinsic Validity vs Environment-Specific Delivery

The domain expressly distinguishes:

```text
Intrinsic Execution Validity
```

from:

```text
Environment-specific Delivery Validity
```

An `ExecutionContext` may be intrinsically valid and, at the same time, be incompatible with a concrete `ExecutionEnvironment`.

Formally:

```text
Valid(ExecutionContext)
```

does not imply:

```text
CanEnterEngine(ExecutionContext, every Environment)
```

Entry into a concrete environment additionally requires:

```text
Compatible(
    AuthorizationChain(ExecutionContext),
    SelectedEnvironment
)
```

Therefore:

```text
EngineInputValidity
    =
IntrinsicValidity

EnvironmentSpecificEngineDelivery
    =
IntrinsicValidity
    +
Chain/Environment compatibility
```

---

## Infrastructure Laws

### D6.6 Cross-Chain Infrastructure Is Not Part of Authorization Semantics

The architecture must allow a semantically valid decision to be materialized through different infrastructure mechanisms.

These mechanisms may include, among others:

```text
Account Abstraction
    EIP-7702
    ERC-4337
    future variants

Execution adapters

Cross-chain messaging

Relayers

Bridges

Interoperability protocols

other future mechanisms
```

The domain does not depend on a concrete provider.

For example, interoperability mechanisms may be provided by any of a number of protocols without any of them forming part of the semantic definition of:

```text
Authorization
Account
Identity
EffectiveAuthority
ExecutionContext
```

The architecture must avoid semantic vendor lock-in.

### D6.7 Deployment Environment Does Not Redefine the Domain

A concrete implementation may initially deploy the system on a specific blockchain or environment.

For example:

```text
Current deployment
    =
specific chain
    +
specific execution runtime
```

This deployment decision **does not redefine the semantic universe of `Authorization`**.

Therefore, it must not be inferred that:

```text
Current deployment chain
    =
only valid Authorization chain
```

nor:

```text
Current execution infrastructure
    =
domain-level authorization constraint
```

The current infrastructure may be specific while the semantic model remains prepared for multiple compatible environments.

### D6.8 User-Selected Blockchain Does Not Redefine Authorization Semantics

A user may eventually select a concrete blockchain or infrastructure to materialize an execution.

This does not mean the semantics of `Authorization` must depend on that selection at the moment of `Authorization Acceptance`.

The architecture permits:

```text
User intent
    ↓
Authorization
    ↓
Authorization Acceptance
    ↓
Runtime
    ↓
Infrastructure selection
    ↓
Environment-specific delivery
```

The user may be fully abstracted from the concrete technologies used to materialize the decision.

Likewise, an advanced user may explicitly select specific environments or mechanisms without altering the fundamental semantic rules of authorization.

---

## Formalization Laws

### D6.9 There Is No Debt from the Absence of expectedChain

The absence of:

```text
expectedChain : Chain
```

in `AuthorizationCanBeAccepted` **does not constitute a formalization debt** under this architecture.

The reason is that:

```text
Authorization Acceptance
```

is deliberately independent from:

```text
Selected Execution Environment
```

The formal property:

```text
accepted(Authorization(ChainA))
    ⇒
accepted(Authorization(ChainB))
```

when only `Chain` is substituted and the remaining acceptance conditions are preserved, demonstrates this independence.

It must not be interpreted as:

```text
Chain is irrelevant
```

nor as:

```text
Any Authorization can execute on Any Chain
```

The correct interpretation is:

```text
Chain is semantically relevant,
but Environment selection is a later materialization concern.
```

### D6.10 Formalization Interpretation

The formalization must interpret the following properties jointly:

#### Authorization semantics

```text
Chain
    ∈
Authorization semantic value
```

#### Authorization Acceptance

```text
AuthorizationCanBeAccepted
    does not require
    a selected ExecutionEnvironment
```

#### Execution Context

```text
ExecutionContext
    carries
    Authorization
```

and therefore transports its `Chain`.

#### Execution Environment

```text
ExecutionEnvironment
    ∉
ExecutionContext semantic value
```

#### Environment delivery

```text
Chain × ExecutionEnvironment
    →
compatibility relation
```

#### Execution Engine

```text
Engine delivery
    requires
    intrinsic validity
    +
environment compatibility
```

These properties are complementary.

They must not be fused into a single rule.

---

## Architectural Laws

### D6.11 Architectural Principle

The architecture establishes the following principle:

```text
Semantic authorization
        ↓
Infrastructure-independent acceptance
        ↓
Execution decision
        ↓
Environment selection
        ↓
Infrastructure-specific materialization
```

Therefore:

> **The domain defines the semantics of an authorized decision before selecting the concrete mechanism through which that decision will be materialized.**

This allows the same primitive of identity, authority, and authorization to be used together with different:

- blockchains;
- Account Abstraction mechanisms;
- execution adapters;
- cross-chain protocols;
- relayers;
- sponsorship mechanisms;
- infrastructure providers;

without converting those choices into an artificial part of the semantics of `Authorization`.

### D6.12 Consequence for Formalization

When an adversarial proof demonstrates that:

```text
AuthorizationCanBeAccepted(...)
```

remains true after changing only:

```text
Authorization.Chain
```

the default interpretation must be:

```text
INTENTIONAL ARCHITECTURAL PROPERTY
```

and not:

```text
FORMALIZATION DEBT
```

unless a future explicit domain decision modifies this architecture.

In particular:

```text
CHAIN-INDEPENDENT AUTHORIZATION ACCEPTANCE
    = intentional
```

while:

```text
CHAIN-SENSITIVE ENVIRONMENT DELIVERY
    = required
```

### D6.13 Canonical Separation

The canonical separation of the model is:

```text
Authorization
    =
semantic authorization decision

ExecutionContext
    =
complete validated execution decision

ExecutionEnvironment
    =
selected materialization environment

Adapter / Infrastructure
    =
mechanism used to materialize the decision
```

and:

```text
Authorization Acceptance
    ≠
Environment Compatibility
    ≠
Physical Execution
```

---

## Conceptual Model of D6

```text
                     AUTHORIZATION
                         │
                    Chain = X
                         │
                         ▼
                EXECUTION CONTEXT
                         │
            (VO, no environment, valid by its content)
                         │
                         ▼
           COMPATIBILITY (contextual)
                         │
       compatible(X, Environment) == true
                         │
                         ▼
                RUNTIME
            (verifies compatibility)
                         │
                         ▼
          EXECUTION ENGINE
   (receives context compatible for that environment)
                         │
                         ▼
              INFRASTRUCTURE
   (implements compatibility and materializes)
```

The key separation:

```text
Chain
    = semantic attribute of the domain

ExecutionContext
    = Value Object without environment
    = its validity is independent of the environment

compatible(Chain, Environment)
    = contextual relation

Runtime
    = verifies compatibility before delivering to the Engine

Infrastructure (Adapter)
    = implements compatibility and materializes
```

---

## Invariant Principles of D6

1. **`Chain` is semantic, not infrastructure.**  
   Its meaning belongs to the authorization domain.

2. **`ExecutionContext` does not contain the environment.**  
   Compatibility is an external relation.

3. **Compatibility does not define the validity of the `ExecutionContext`.**  
   Validity is determined by the semantic content of the context.

4. **Compatibility is a materialization condition, not an acceptance condition.**  
   It does not affect `AuthorizationValidation`.

5. **The `Runtime` verifies compatibility before delivering to the Engine.**  
   It is its responsibility to ensure that the context is compatible with the environment.

6. **Infrastructure implements compatibility.**  
   Its concrete definition is external to the domain.

7. **The separation between domain and infrastructure is strict.**  
   No physical dependencies are introduced into the laws of the `Account`.

---

## Relationship with Other Domain Modules

- **`Authorization`**: Contains `Chain` as part of its Context.
- **`ExecutionContext`**: Value Object without environment; its validity is determined by its content, not by compatibility.
- **`Runtime`**: Verifies compatibility before delivering the `ExecutionContext` to the `Execution Engine`.
- **`Execution Engine`**: Receives already-compatible contexts; does not re-validate.
- **`Adapter` / Infrastructure**: Implements `compatible` and materializes the execution.
- **`AuthorizationValidation`**: Is not affected; compatibility is a materialization concern, not an acceptance concern.

---

## Cross-References

- [Authorization](../ubiquitous-language/03-authorization.md) — `Authorization` and its Context.
- [Execution Model](../ubiquitous-language/06-execution-model.md) — `Runtime`, `ExecutionContext`, `Adapter`.
- [Formal Laws — D1](D1-delegation-and-authority.md) — execution requires a valid context.
- [Formal Laws — D4](D4-proof-and-authorization-equality.md) — `Authorization` semantic value.
