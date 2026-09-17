---
id: account-law-D3
title: D3 — Entity Identity and Recognition
domain: sovereign-account
chapter: formal-laws
type: law-set
status: stable
law-id: D3
laws:
  - D3.1
  - D3.2
  - D3.3
  - D3.4
  - D3.5
  - D3.6
  - D3.7
  - D3.8
depends-on:
  - account-ul-state-and-policy
  - account-ul-derived-authority
related:
  - account-law-D1
  - account-law-D2
  - account-law-D4
formal-laws:
  - D1.13
  - D1.38
  - D1.39
  - D3.5
  - D3.6
verification:
  dafny:
    - domain/account/formal/authority/AuthorizationState.dfy
    - domain/account/formal/authority/Credential.dfy
    - domain/account/formal/authority/Session.dfy
    - domain/account/formal/authority/Delegation.dfy
    - domain/account/formal/account/AccountTransitions.dfy
---

# D3 — Entity Identity and Recognition

## Context

In the domain, `Credential`, `Session`, and `Delegation` are **Entities**: they possess a stable individual identity (`CredentialId`, `SessionId`, `DelegationId`) that remains invariable throughout their entire lifecycle.

The formalization of the domain distinguishes two conceptual properties that, although related, are independent:

1. **Entity identity** — `EntityId(updated) == EntityId(original)`.
2. **Recognition in the state** — `EntityId ∈ AuthorizationState` after a transition.

An Entity may preserve its identity without being recognized in the state (for example, if it was never registered), or it may be recognized but with a state that makes it unusable (revoked or temporarily invalid).

The purpose of this law set is to formalize the separation between Entity identity, recognition in `AuthorizationState`, and effective usability, establishing that **a revoked or temporarily invalid Entity remains represented in `AuthorizationState`**, so that the state distinguishes between "never recognized" and "recognized and subsequently disabled".

---

## Domain Decisions

1. **`AuthorizationState` is an operational-historical record.**  
   It preserves the representation of all Entities that have been recognized by the `Account`, even after their state changes (for example, after a revocation).  
   This allows distinguishing:
   - `never recognized`
   - `recognized and currently usable`
   - `recognized but currently unusable` (by revocation or temporary invalidity)

2. **The identity of an Entity is immutable.**  
   The `Id` of a `Credential`, `Session`, or `Delegation` never changes during its lifecycle.  
   Transitions modify the **state** (e.g. `Active → Revoked`), but not the `Id`.

3. **Each Entity has its own lifecycle, defined by the domain.**  
   - **`Credential`**: `Active ↔ Suspended → Revoked` (the domain contemplates `suspended` and `reactivated`).  
   - **`Session`**: `Active → Revoked`; additionally, it has a temporal validity interval (`validFrom`, `validUntil`) that determines its usability over time.  
   - **`Delegation`**: `Active → Revoked`; it also has a temporal validity interval.  
   **There is no universal `EntityStatus`.** Each Entity uses its own state type.

4. **Expiration is not a persistent state.**  
   For `Session` and `Delegation`, the "expired" condition is derived from their validity interval and the evaluation time. A `Session` or `Delegation` with `status = Active` but with `now > validUntil` is **temporarily invalid**, but its persistent state remains `Active`. Temporal invalidity does not change the state of the Entity; it only affects its contribution to `EffectiveAuthority`.

5. **Revocation of a `Credential` affects `Session` instances, not `Delegation` instances by structural dependency.**  
   A `Session` explicitly depends on a `Credential` (it has a `CredentialId`). Therefore, if the `Credential` is revoked, the `Session` stops being usable (its source of authority is invalid).  
   A `Delegation`, in contrast, depends on a source `Identity` (`SourceIdentityId`), not on a specific `Credential`. The invalidity of a `Credential` may affect a `Delegation` **only if the delegated authority came from that `Credential`** (by provenance), but it is not a universal structural dependency. Therefore, **a general rule that invalidates all `Delegation` instances upon revoking a `Credential` is not imposed**.

6. **State transitions (e.g. revocation) do not remove the Entity from the state.**  
   Revocation changes the state of the Entity (e.g. from `Active` to `Revoked`), but the Entity remains recognized in `AuthorizationState`. This preserves the history and allows distinguishing "never recognized" from "recognized and revoked".

7. **Transition laws must prove two separate properties:**
   - **Entity identity preservation**: `EntityId(new) == EntityId(old)`.
   - **State membership preservation**: `EntityId ∈ AuthorizationState(new)`.

8. **`AuthorizationState` does not contain mutator functions.**  
   Transitions (revocation, creation, etc.) are the responsibility of `AccountTransitions` or `PolicyConsumption`. `AuthorizationState` is a data structure with invariants, not a module that mutates its own state.

---

## Identity Laws

### D3.1 Identity Immutability

For every transition that modifies an Entity `E` (`Credential`, `Session`, or `Delegation`):

**Law D3.1.1**  
`EntityId(E') == EntityId(E)`

**Law D3.1.2**  
The identifier of an Entity is never reassigned to another Entity.

---

## State Membership Laws

### D3.2 State Membership Persistence

**Law D3.2.1**  
If an Entity `E` is recognized in `AuthorizationState` in a state `S`, then after any valid transition affecting `E`, `EntityId(E)` continues to belong to the set of Entities recognized in `S'`.

**Law D3.2.2**  
Revocation (or any state change) does not remove the Entity from the state. It only changes its `status` or other relevant attributes.

### D3.4 Entity Identity vs State Membership (Separate Laws)

For any transition `T` that updates an Entity `E`:

**Law D3.4.1 — Identity preservation**  
`EntityId(E_after) == EntityId(E_before)`

**Law D3.4.2 — State membership preservation**  
`EntityId(E_after) ∈ AuthorizationState(after)`  
if and only if `EntityId(E_before) ∈ AuthorizationState(before)`

That is, state membership is preserved across the transition, independently of the state change or temporal validity.

---

## Lifecycle Laws

### D3.3 Specific Lifecycles (Non-Universal)

Each Entity possesses its own state type and transition rules:

#### `Credential`
- States: `Active`, `Suspended`, `Revoked`.
- Valid transitions:
  - `Active ↔ Suspended` (possible reactivation).
  - `Active → Revoked` (terminal).
  - `Suspended → Revoked` (terminal).
  - `Revoked → Active` and `Revoked → Suspended` are not permitted.

#### `Session`
- States: `Active`, `Revoked`.
- Additionally, it possesses `validFrom` and `validUntil` (validity interval).
- The condition `now < validFrom` or `now > validUntil` makes the `Session` **temporarily invalid**, but its persistent state remains `Active`.
- Valid transitions:
  - `Active → Revoked` (terminal).
  - `Revoked → Active` is not permitted.

#### `Delegation`
- States: `Active`, `Revoked`.
- Additionally, it possesses `validFrom` and `validUntil` (validity interval).
- The condition `now < validFrom` or `now > validUntil` makes the `Delegation` **temporarily invalid**, but its persistent state remains `Active`.
- Valid transitions:
  - `Active → Revoked` (terminal).
  - `Revoked → Active` is not permitted.

**Law D3.3.1**  
There is no universal `EntityStatus`. Each Entity uses its own state type.

**Law D3.3.2**  
`Expiration` (temporal) is not a persistent state. It is a condition derived from the validity interval and the evaluation time.

---

## Usability Laws

### D3.5 Usability and Contribution to EffectiveAuthority

The usability of an Entity to contribute to `EffectiveAuthority` depends on:

- Its **persistent state** (e.g. `Active` vs `Revoked`).
- Its **temporal validity** (if applicable).
- For `Session`: whether its source `Credential` is usable.
- For `Delegation`: whether the delegated authority may be exercised by the source (evaluated by provenance).

**Law D3.5.1**  
An Entity may only contribute to `EffectiveAuthority` if:
- Its persistent state is `Active` (or its equivalent, according to the lifecycle).
- It satisfies temporal conditions (e.g. `validFrom <= now <= validUntil`).
- For `Session`: its source `Credential` is usable.
- For `Delegation`: the delegated authority is exercisable by the source in the current context.

**Law D3.5.2**  
A revoked Entity (`Revoked`) never contributes to `EffectiveAuthority`, independently of its temporal validity.

**Law D3.5.3**  
An Entity with `Active` state but temporarily invalid (e.g. expired) does not contribute to `EffectiveAuthority`; its contribution is `∅` at that moment.

---

## Propagation Laws

### D3.6 Invalidity Propagation from `Credential` Revocation

**Law D3.6.1**  
If a `Credential` is revoked, every `Session` that depends on it (has its `CredentialId`) stops being usable. Its persistent state may remain `Active`, but its contribution to `EffectiveAuthority` is `∅` because its source of authority has been invalidated.

**Law D3.6.2**  
Revoking a `Credential` **does not automatically invalidate all `Delegation` instances** in the system. A `Delegation` may be affected only if the delegated authority came from that `Credential` (by provenance), but no universal structural dependency exists. Evaluation of `Delegation` usability must consider the provenance of its authority and whether that authority remains valid in the current context.

**Law D3.6.3**  
It is not necessary to explicitly revoke each `Session` or `Delegation` upon revoking a `Credential`. Invalidity propagates through the `EffectiveAuthority` rules (per [D1](D1-delegation-and-authority.md) and D3.5), not through state changes of the dependent Entities.

---

## Transition Laws

### D3.7 Valid Transitions and Invariant Preservation

**Law D3.7.1**  
Any transition that modifies the state of an Entity must preserve:
- Entity identity.
- State membership.
- Integrity invariants (e.g. a `Credential` cannot be in two states simultaneously).

**Law D3.7.2**  
A transition that attempts to remove an Entity from the state (remove its representation) is not valid.  
The domain does not admit deletion; only state changes.

---

## Responsibility Laws

### D3.8 Separation of Responsibilities

**Law D3.8.1**  
`AuthorizationState` is a data structure that maintains the recognition state and attributes of Entities. It does not contain mutator functions.

**Law D3.8.2**  
State transitions (e.g. `revokeCredential`, `createSession`) are the responsibility of `AccountTransitions` or `PolicyConsumption`. These functions take an `AuthorizationState` and produce a new valid state, verifying the invariants.

---

## Conceptual Model of D3

```text
                     AUTHORIZATION STATE
                            │
            ┌───────────────┼───────────────┐
            │               │               │
    Credentials        Sessions         Delegations
            │               │               │
      (EntityId)      (EntityId)       (EntityId)
            │               │               │
    CredentialStatus  SessionStatus   DelegationStatus
    (Active|Susp..)   (Active|Revoked) (Active|Revoked)
            │               │               │
            │         validFrom/Until   validFrom/Until
            │               │               │
            └───────────────┼───────────────┘
                            │
                            ▼
                   USABILITY EVALUATION
              (persistent state + temporal
               + dependencies + provenance)
                            │
                            ▼
                   EFFECTIVE AUTHORITY
```

The key distinction:

```text
Entity identity
    = stable, independent of state

Entity recognition in state
    = persistent throughout the lifecycle

Entity usability
    = determined by persistent state,
      temporal validity, dependencies, and provenance
```

---

## Invariant Principles of D3

1. **The identifier of an Entity is immutable.**  
   `Id` is what defines the Entity throughout its lifecycle.

2. **State membership is permanent once the Entity has been recognized.**  
   Entities are not removed from the state; only their internal state changes.

3. **Each Entity has its own lifecycle; there is no universal `EntityStatus`.**  
   `Credential`, `Session`, and `Delegation` have distinct state sets, defined by the domain.

4. **Expiration is a temporal condition, not a persistent state.**  
   Entities may be temporarily invalid without changing their persistent state.

5. **Revoking a `Credential` invalidates the `Session` instances that depend on it, but not `Delegation` instances by default.**  
   `Delegation` instances are evaluated according to their source and provenance.

6. **State transitions must preserve identity and state membership.**  
   These are transversal invariants.

7. **`AuthorizationState` does not mutate its own state; transitions are external and produce new valid states.**

---

## Relationship with Other Domain Modules

- **`AuthorizationState`**: Data structure that stores Entities with their states and temporal attributes. It does not contain mutation logic.
- **`EffectiveAuthority`**: Computes the current authority by filtering Entities by usability (state, temporality, dependencies, and provenance).
- **`AccountTransitions`**: Defines valid state transitions (e.g. revoke `Credential`, create `Session`). Takes a state and returns a new state or error.
- **`PolicyConsumption`**: May include effects such as `RevokeCredential(id)`, which triggers a state transition in `AccountTransitions`.
- **`Session`**: Explicitly depends on a `Credential`.
- **`Delegation`**: Depends on a source `Identity`; its validity is evaluated by provenance.

---

## Cross-References

- [Foundations](../ubiquitous-language/01-foundations.md) — `Identity` and `Account`.
- [Authority Model](../ubiquitous-language/02-authority-model.md) — `Credential` as an Entity.
- [Derived Authority](../ubiquitous-language/04-derived-authority.md) — `Session`, `Delegation`, and `EffectiveAuthority`.
- [State and Policy](../ubiquitous-language/05-state-and-policy.md) — `AuthorizationState` and its transitions.
- [Formal Laws — D1](D1-delegation-and-authority.md) — authority invariants and revocation laws.
- [Formal Laws — D2](D2-policy-effect-and-state.md) — `PolicyEffect` atomicity and state transitions.
