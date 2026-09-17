---
id: account-law-D5
title: D5 — Restriction Association
domain: sovereign-account
chapter: formal-laws
type: law-set
status: stable
law-id: D5
laws:
  - D5.1
  - D5.2
  - D5.3
  - D5.4
  - D5.5
depends-on:
  - account-ul-derived-authority
  - account-ul-state-and-policy
related:
  - account-law-D1
  - account-law-D2
formal-laws:
  - D1.44
  - D1.45
  - D2.1
  - D2.4
  - D2.6
verification:
  dafny:
    - domain/account/formal/authority/AuthorizationState.dfy
    - domain/account/formal/foundation/Restriction.dfy
    - domain/account/formal/policy/PolicyEffect.dfy
---

# D5 — Restriction Association

## Context

The domain defines `Restriction` as a **Value Object** that limits the conditions under which a `Capability` may be exercised.

`Restriction` does not contain a reference to the `Capability` or `Scope` to which it applies. It is an autonomous semantic value.

The effect `ModifyRestriction(Capability, optional Scope, Restriction)` (defined in [D2](D2-policy-effect-and-state.md)) replaces the `Restriction` associated with a `(Capability, Scope)` pair by a new `Restriction`.

However, the representation of `AuthorizationState` does not inherently define how a `Restriction` is associated with the `Capability` and `Scope` it constrains:

```text
AuthorizationState
    restrictions : set<Restriction>
```

Such a representation does not allow:

- Determining which `Restriction` corresponds to a given `Capability`/`Scope`.
- Replacing an existing `Restriction` without ambiguity.
- Verifying that there are no orphaned or inconsistent `Restriction` instances.

The purpose of this law set is to formalize the semantic relationship between `Restriction`, `Capability`, and `Scope` inside `AuthorizationState`, establishing an explicit association that permits modeling the application of `ModifyRestriction` correctly.

---

## Domain Decisions

1. **`AuthorizationState` maintains an explicit association between `(Capability, Scope)` and `Restriction`.**  
   Each `Restriction` is bound to a `Capability` and a specific `Scope` (the latter may be optional).  
   This association is what allows `ModifyRestriction` to be applied as a replacement for a previous `Restriction`.

2. **`Restriction` remains a Value Object without its own identity.**  
   It does not contain `Capability` or `Scope`. Its semantic identity is determined only by its value.  
   The association with `Capability`/`Scope` resides in `AuthorizationState`, not in `Restriction`.

3. **The association is functional: for each `(Capability, Scope)` there is at most one `Restriction`.**  
   Duplicate `Restriction` instances for the same target are not permitted.  
   If no `Restriction` exists for a target, it is considered that the `Capability` has no additional restrictions (e.g. "no limit").

4. **The `Scope` in `ModifyRestriction` may be optional.**  
   - If `Scope` is present, the `Restriction` is associated with the `(Capability, Scope)` pair.
   - If `Scope` is absent, the `Restriction` is associated with the `Capability` without an additional `Scope` (that is, the `Restriction` applies to the `Capability` in its general domain).

5. **Modifying a `Restriction` replaces the previous `Restriction` for that target.**  
   `ModifyRestriction(Capability, Scope, NewRestriction)`:
   - If `(Capability, Scope) → OldRestriction` exists, it is replaced by `NewRestriction`.
   - If no such association exists, the new association is added.
   - `NewRestriction` is not permitted to be semantically invalid or to violate domain invariants.

6. **`EffectiveAuthority` must consider the `Restriction` associated with the corresponding `Capability` and `Scope`.**  
   When computing `EffectiveAuthority`, the authority of a `Capability` is intersected with the `Restriction` that corresponds to it, if one exists.

---

## Association Laws

### D5.1 Restriction–Capability–Scope Association

`AuthorizationState` contains a mapping:

```text
RestrictionMap: (Capability × OptionalScope) → Restriction
```

**Law D5.1.1**  
For each `(Capability, Scope)` there is at most one `Restriction`.

**Law D5.1.2**  
If a `Capability` has no associated `Restriction` in a `Scope`, it is considered that there is no additional restriction (equivalent to "no restriction").

---

## Modification Laws

### D5.2 Application of ModifyRestriction

The operation `ModifyRestriction(cap, scope, newRestriction)` over `AuthorizationState` is defined as:

**Law D5.2.1**  
If `scope` is present:
- It replaces or adds the association `(cap, scope) → newRestriction`.

**Law D5.2.2**  
If `scope` is absent:
- It replaces or adds the association `(cap, null) → newRestriction` (general `Scope`).

**Law D5.2.3**  
The operation is atomic: either the entire state is updated, or no change occurs.

---

## Integrity Laws

### D5.3 Association Integrity

**Law D5.3.1**  
There cannot be two `Restriction` instances for the same `(Capability, Scope)`.

**Law D5.3.2**  
If a `Capability` is removed from `AuthorizationState` (for example, because its authority is revoked), all its associated `Restriction` instances must also be removed.

**Law D5.3.3**  
A `Restriction` cannot exist in `AuthorizationState` without being associated with an existing `Capability`.

---

## Authority Laws

### D5.4 Effect on EffectiveAuthority

When computing `EffectiveAuthority`:

**Law D5.4.1**  
For each `Capability` considered exercisable, the `Restriction` associated with `(Capability, Scope)` is applied, if it exists.

**Law D5.4.2**  
The `Restriction` is applied as an intersection with the derived authority: the `Capability` is exercisable only to the extent the `Restriction` permits.

**Law D5.4.3**  
If no `Restriction` exists for the `Capability`/`Scope`, authority is not restricted by this mechanism.

---

## Consistency Laws

### D5.5 Consistency with Entity Lifecycles

**Law D5.5.1**  
The association `(Capability, Scope) → Restriction` is independent of the Entities (`Credential`, `Session`, `Delegation`).  
`Restriction` instances are part of the operational state of the `Account` and persist while the `Capability` remains recognized.

**Law D5.5.2**  
Removing a `Credential` does not automatically remove `Capability` restrictions; these are managed by `AuthorizationState` transitions (e.g. `DisableCapability`, `ModifyRestriction`).

---

## Conceptual Model of D5

```text
                     AUTHORIZATION STATE
                            │
            ┌───────────────┼───────────────┐
            │               │               │
    Capabilities        Credentials      RestrictionMap
            │               │               │
      (EntityId)      (EntityId)          │
            │               │               │
    CapabilityKind   CredentialStatus    (Capability, Scope)
            │               │               │
         Scope              ...          Restriction
            │               │               │
            └───────────────┼───────────────┘
                            │
                            ▼
                   EFFECTIVE AUTHORITY
              (with Restrictions applied)
```

The key distinction:

```text
Restriction
    = Value Object without Capability/Scope

RestrictionMap
    = Association binding each Capability/Scope
      to its corresponding Restriction

AuthorizationState
    = Contains Capabilities, Credentials, and RestrictionMap
```

---

## Invariant Principles of D5

1. **Each `(Capability, Scope)` has at most one `Restriction`.**  
   This guarantees uniqueness and prevents conflicts.

2. **Every `Restriction` is associated with an existing `Capability`.**  
   There are no `Restriction` instances without a `Capability`.

3. **Modifying a `Restriction` is atomic and replaces the previous one.**  
   No accumulation of `Restriction` instances over the same target occurs.

4. **`EffectiveAuthority` always applies the `Restriction` corresponding to the `Scope`.**  
   If it exists, it limits authority; if not, no additional restriction applies.

5. **The association is independent of `Credential`, `Session`, and `Delegation`.**  
   It is an attribute of the `Capability` in the authorization state.

---

## Relationship with Other Domain Modules

- **`AuthorizationState`**: Contains `RestrictionMap` as part of its structure.
- **`PolicyEffect`**: `ModifyRestriction` acts on `RestrictionMap`.
- **`EffectiveAuthority`**: Consults `RestrictionMap` when computing effective authority.
- **`PolicyConsumption`**: Orchestrates the atomic application of `ModifyRestriction` along with other effects.
- **`AccountTransitions`**: May include transitions that affect `RestrictionMap` (e.g. `DisableCapability` may remove the associated `Restriction`).

---

## Cross-References

- [Derived Authority](../ubiquitous-language/04-derived-authority.md) — `Restriction` and `EffectiveAuthority`.
- [State and Policy](../ubiquitous-language/05-state-and-policy.md) — `AuthorizationState` and `PolicyEffect`.
- [Formal Laws — D1](D1-delegation-and-authority.md) — restriction laws (`D1.44`, `D1.45`).
- [Formal Laws — D2](D2-policy-effect-and-state.md) — `ModifyRestriction` applicability and atomicity.
