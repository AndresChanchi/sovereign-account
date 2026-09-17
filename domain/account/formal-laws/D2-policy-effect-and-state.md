---
id: account-law-D2
title: D2 — Policy Effect and State
domain: sovereign-account
chapter: formal-laws
type: law-set
status: stable
law-id: D2
laws:
  - D2.1
  - D2.2
  - D2.3
  - D2.4
  - D2.5
  - D2.6
  - D2.7
  - D2.8
depends-on:
  - account-ul-state-and-policy
  - account-ul-derived-authority
related:
  - account-law-D1
  - account-law-D3
  - account-law-D5
verification:
  dafny:
    - domain/account/formal/policy/Policy.dfy
    - domain/account/formal/policy/PolicyEffect.dfy
    - domain/account/formal/policy/PolicyConsumption.dfy
    - domain/account/formal/laws/PolicyLaws.dfy
---

# D2 — Policy Effect and State

`PolicyEffect` represents a semantic change that a `Policy` produces on `AuthorizationState`.  
It is a **Value Object**: its equality is defined by its type and parameters, and it has no identity of its own.

A `Policy` is an **ordered sequence** of effects that must be consumed atomically, validating all its components beforehand.

---

## Domain Decisions

1. **`PolicyEffect` is a semantic descriptor of an `AuthorizationState` transformation.**  
   Its meaning is fully determined by its type and parameters.

2. **`PolicyEffect` does not know how to mutate the state.**  
   It has no `Apply()` method. The responsibility for applying the effect belongs to the `AuthorizationState` module, which knows its own invariants.  
   This preserves the separation:

   ```text
   PolicyEffect
       =
   what change is requested/recognized

   AuthorizationStateTransition
       =
   valid transformation of state
   ```

3. **A `Policy` is an ordered sequence of effects.**  
   It is not a set, because order matters semantically.  
   For example, `DisableCapability(X)` followed by `EnableCapability(X)` produces a different state than the reverse order.  
   The domain does not assume commutativity without proof.

4. **Policy consumption is atomic.**  
   **All** effects of the sequence are applied, in the given order, or **none**.  
   No intermediate states or partial consumptions are permitted.

5. **Contradictory effects invalidate the entire `Policy`.**  
   If a sequence contains effects that logically contradict each other over the same state (e.g. `DisableCapability(X)` and `EnableCapability(X)`), the transition is invalid and no state change occurs.  
   A `Policy` represents a **coherent decision**, not an imperative program.

6. **Optionality of steps is resolved in the external construction of the `Policy`.**  
   The external context (e.g. `Recovery`, `Economics`, `Governance`) decides which effects to include.  
   The `Runtime` does not decide to skip steps within a consumed `Policy`.  
   Therefore, the domain does not model "optional steps" in consumption; it only models sequences of effects **all of which are mandatory for that decision**.

7. **Independent effects may commute.**  
   This is not assumed, but may be demonstrated as a property.  
   For example, `RevokeCredential(A)` and `RevokeSession(B)` likely commute if they affect different Entities.  
   That commutativity is a **derived theorem**, not a domain assumption.

8. **The effect catalog is semantically closed but extensible by domain decision.**  
   The initial set of constructors covers known cases.  
   If the domain needs new effects in the future, they are added as new constructors, respecting the laws of order, atomicity, and non-contradiction.

9. **Modification of authorization conditions.**  
   The corresponding effect is formalized as `ModifyRestriction(Capability, optional Scope, Restriction)`, since the modifiable conditions in `AuthorizationState` are `Restriction` instances associated with `Capability` and `Scope`.  
   This does not introduce new entities; `Restriction` is already a Value Object defined in the domain.  
   See [Formal Laws — D5](D5-restriction-association.md) for the association semantics.

---

## PolicyEffect Laws

### D2.1 PolicyEffect as Semantic Descriptor

`PolicyEffect` is a Value Object. Its equality is defined by type and parameters.

```text
PolicyEffect
    ::=
    RevokeCredential(CredentialId)
  | RevokeSession(SessionId)
  | RevokeDelegation(DelegationId)
  | DisableCapability(Capability, optional Scope)
  | EnableCapability(Capability, optional Scope)
  | ModifyRestriction(Capability, optional Scope, Restriction)
```

**Law D2.1.1**  
Two `PolicyEffect` instances are equal if and only if they have the same constructor and the same semantic parameters.

**Law D2.1.2**  
`PolicyEffect` does not contain its own identity. There is no `PolicyEffectId`.

---

## Policy Sequence Laws

### D2.2 Policy as Ordered Sequence

A `Policy` is an ordered sequence of zero or more `PolicyEffect` instances.

```text
Policy = ordered sequence of PolicyEffects
```

**Law D2.2.1**  
The order of effects in the sequence is part of the meaning of the `Policy`.  
Two `Policy` instances with the same effects but in a different order are semantically different, unless all their effects are demonstrated to commute.

**Law D2.2.2**  
An empty `Policy` is valid. Its consumption produces no changes in `AuthorizationState`.

---

## Consumption Laws

### D2.3 Atomicity of Consumption

`Policy` consumption is an **atomic transition** over `AuthorizationState`.

**Law D2.3.1**  
For a `Policy` `P = [E1, E2, ..., En]` and a state `S`:

```text
ConsumePolicy(P, S)
    =
    if all effects E1..En are applicable and consistent in S
        then ApplyAll(P, S) -> S'
        else S (no transition)
```

That is, either a new state `S'` is produced, or the state remains unchanged.  
There is no intermediate state.

**Law D2.3.2**  
`ApplyAll` applies effects sequentially in the given order, but validation of all effects occurs **before** modifying the state.

### D2.4 Applicability of Effects

An effect is **applicable** if the current state contains the Entities and conditions the effect requires.

**Law D2.4.1**  
`RevokeCredential(id)` is applicable only if `id` is recognized in `AuthorizationState`.

**Law D2.4.2**  
`DisableCapability(cap, scope)` is applicable only if the `Capability` exists for that `Account` and `Scope`.

**Law D2.4.3**  
`ModifyRestriction(cap, scope, newRestriction)` is applicable only if:
- The `Capability` `cap` exists in `AuthorizationState` for the `Account`.
- If `scope` is present, the `Capability` is associated with that `Scope`.
- The previous `Restriction` (if any) can be replaced by `newRestriction` without violating invariants.
- `newRestriction` is semantically valid (e.g. values within permitted ranges).

### D2.5 Invalid Effect Rejects Entire Policy

**Law D2.5.1**  
If at least one effect in the sequence is not applicable, the entire `Policy` is rejected.  
No effect is applied, not even the applicable ones.

This preserves the atomicity and coherence of the decision.

---

## Consistency Laws

### D2.6 Contradictory Effects

**Law D2.6.1**  
A sequence of effects is **inconsistent** if it contains effects that, applied in the given order, produce a state that violates a fundamental invariant of the domain or contradicts a prior decision within the same sequence.

**Explicit case:**  
`DisableCapability(X)` and `EnableCapability(X)` over the same `Capability` and `Scope` in the same `Policy`, without any intermediate effect modifying the meaning of that `Capability`, constitutes a contradiction.

**Law D2.6.2**  
An inconsistent `Policy` is rejected in its entirety.

### D2.7 Independent Effects May Commute

**Law D2.7.1**  
Two effects `Ea` and `Eb` are independent if they affect disjoint sets of Entities or orthogonal attributes of the state.

**Law D2.7.2**  
For independent effects, the following holds:

```text
Apply(Eb, Apply(Ea, S)) == Apply(Ea, Apply(Eb, S))
```

This property must be demonstrated for each concrete combination of independent effects. It is not a global assumption.

---

## Preservation Laws

### D2.8 Preservation of Domain Invariants

**Law D2.8.1**  
Every `AuthorizationState` transition produced by consuming a valid `Policy` must preserve:

- The unique sovereignty of the `Account` (`SovereignIdentity` remains immutable).
- Authority non-expansion (no effect may create authority that did not exist in legitimate sources).
- Integrity of recognized Entities (their identity remains stable; only their state/lifecycle changes).
- Coherence of delegation and session relationships.

---

## Conceptual Model of D2

```text
            EXTERNAL CONTEXT
                  │
                  │ produce decision
                  ▼
               POLICY
                  │
                  │ recognized by Account
                  ▼
         ORDERED SEQUENCE
         OF POLICY EFFECTS
                  │
                  ▼
         COMPLETE VALIDATION
    (applicability + consistency)
             /          \
           Yes           No
            │             │
            ▼             ▼
     TRANSFORMATION     REJECTION
    (atomic, new        (state
      state S')         unchanged)
```

The key separation:

```text
PolicyEffect
    = descriptor of requested change

AuthorizationStateTransition
    = valid transformation of state

Runtime / PolicyConsumption
    = orchestration of validation and atomic application
```

---

## Invariant Principles of D2

1. **A `Policy` is a decision, not a program.**  
   Its sequence of effects represents a coherent and complete intention.

2. **Atomicity is a fundamental property of `Policy` consumption.**  
   `AuthorizationState` never reflects a partial consumption of a `Policy`.

3. **Contradiction between effects in the same `Policy` invalidates the entire decision.**  
   The domain does not admit ambiguity in recognized external decisions.

4. **Applicability of each effect is a necessary condition for the entire transition.**  
   A single invalid effect prevents the entire transition.

5. **Order is semantic.**  
   The domain does not simplify order to a set without an explicit proof of commutativity.

6. **Entity identity is preserved.**  
   Effects such as `RevokeCredential(id)` refer to the same Entity by its stable `id`.  
   The transition only changes the Entity's state (e.g. from `Active` to `Revoked`), not its identity.

---

## Relationship with Other Domain Modules

- **`AuthorizationState`**: Subject of the transformation. Exposes the functions for validation and effect application.
- **`PolicyConsumption`**: Process that orchestrates validation and atomic application. May be part of the `Runtime` or a specific module.
- **`Recovery`, `Economics`, `Governance`**: External contexts that produce `Policy` instances. They do not need to know the internal mechanism of application; they generate the sequence of effects their decision implies.
- **`Runtime`**: Coordinates reception of the `Policy`, its validation, and its atomic consumption, without skipping steps or modifying the external decision.

---

## Cross-References

- [State and Policy](../ubiquitous-language/05-state-and-policy.md) — `AuthorizationState`, `PolicyEffect`, `PolicyConsumption`.
- [Derived Authority](../ubiquitous-language/04-derived-authority.md) — how `Restriction` participates in authority derivation.
- [Formal Laws — D1](D1-delegation-and-authority.md) — authority invariants.
- [Formal Laws — D3](D3-entity-identity-and-recognition.md) — identity immutability across state transitions.
- [Formal Laws — D5](D5-restriction-association.md) — `ModifyRestriction` target association.
