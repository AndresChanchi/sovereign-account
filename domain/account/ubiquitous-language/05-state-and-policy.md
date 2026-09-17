---
id: account-ul-state-and-policy
title: State and Policy
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "05"
type: concept
status: stable
section: "25-29"
value-objects: [PolicyEffect, PolicyConsumption]
depends-on:
  - account-ul-foundations
  - account-ul-authority-model
  - account-ul-derived-authority
related:
  - account-ul-execution-model
  - account-ul-reference
formal-laws:
  - D1.35
  - D1.36
  - D1.37
  - D1.38
  - D1.39
  - D1.40
  - D1.41
  - D2.1
  - D2.2
  - D2.3
  - D2.4
  - D2.5
  - D2.6
  - D2.7
  - D2.8
  - D5.1
  - D5.2
  - D5.3
  - D5.4
  - D5.5
verification:
  dafny:
    - domain/account/formal/authority/AuthorizationState.dfy
    - domain/account/formal/policy/Policy.dfy
    - domain/account/formal/policy/PolicyEffect.dfy
    - domain/account/formal/policy/PolicyConsumption.dfy
    - domain/account/formal/account/AccountTransitions.dfy
---

 # State and Policy

 > Original DDD sections: §25–§29.

 This chapter defines the **operational authority state** of an `Account` and the mechanisms through which **external decisions** produce valid transitions on that state.

---

 ## Authorization State

 **Authorization State** represents the operational authorization state maintained by an `Account`.

 It may contain:

 - `Credential` instances;
- `Credential` ↔ `Account` recognition;
- `CredentialAuthority` relations;
- `Session` instances;
- `Delegation` instances;
- `Capability` instances;
- `Restriction` associations;
- recognized `PolicyEffect` instances;
- structural relationships necessary to evaluate authority.

 `Authorization State` does not represent an execution.

 `Authorization State` is the state on which `EffectiveAuthority` is resolved.

 ### Important

 `Credential`, `Session`, `Delegation`, `Capability`, etc. may exist as independent concepts, but **the relationships between them belong to `AuthorizationState` when those relationships are part of the operational state of an `Account`**.

 This avoids introducing those relationships artificially inside the Value Objects.

 ### Restriction Association

 `Restriction` is a Value Object that does not contain a reference to the `Capability` or `Scope` to which it applies. That association is part of `AuthorizationState`:

```
RestrictionMap: (Capability × Optional Scope) → Restriction
```

 For the formal treatment, see Formal Laws — D5.

---

 ## Authorization State Transition

 An **Authorization State Transition** represents a valid change in `AuthorizationState`.

 Examples:

 - registering a `Credential`;
- recognizing a `Credential` in an `Account`;
- establishing `CredentialAuthority`;
- modifying `CredentialAuthority`;
- revoking a `Credential`;
- creating a `Session`;
- revoking a `Session`;
- creating a `Delegation`;
- revoking a `Delegation`;
- modifying `Capability` instances;
- applying a `PolicyEffect`.

 These transitions belong to the `Account` domain.

 They do not represent business executions over external resources.

---

 ## Policy

 A **Policy** represents an external decision recognized by an `Account` as capable of producing one or more changes to `AuthorizationState`.

 The `Policy` **does not need its own identity inside the `Account`**.

 In external contexts, an Entity may exist representing the procedure that produced that decision.

 For example:

```
RecoveryPolicyRequest
```

 may be an Entity of a `Recovery` bounded context with:

 - `requestId`;
- lifecycle;
- approval;
- expiration;
- cancellation;
- consumption.

 But:

```
RecoveryPolicyRequest
    ≠
Policy Effect
```

 and:

```
RecoveryPolicyRequest
    ≠
Account Policy Value
```

 The `Account` consumes the recognized decision; it does not need to know the full lifecycle of the producing bounded context.

---

 ## Policy Effect

 A **Policy Effect** represents the semantic change a `Policy` produces on `AuthorizationState`.

 Examples:

```
RevokeCredential(X)
EnableRecovery
DisableCapability(Y)
ModifyAuthorizationCondition(Z)
```

 `PolicyEffect` is a **Value Object**.

 Two distinct `Policy` instances may produce the same effect:

```
PolicyRequest #1
    → RevokeCredential(X)

PolicyRequest #2
    → RevokeCredential(X)
```

 and:

```
RevokeCredential(X)
    ==
RevokeCredential(X)
```

 Historical identity belongs to the external procedure, not to the effect.

 The formal algebra of `PolicyEffect` and its transformation over `AuthorizationState` are defined in Formal Laws — D2.

---

 ## Policy Consumption

 **Policy Consumption** represents the recognition and application of a `Policy` by an `Account`.

 Conceptually:

```
External Policy
      ↓
Policy Recognition
      ↓
Policy Consumption
      ↓
Policy Effect(s)
      ↓
Authorization State Transition
```

 The `Account` does not need to know how the `Policy` was approved.

 Approval belongs to the producing bounded context.

 A `Policy` may produce **one or more `PolicyEffect` instances**.

 The semantics of:

 - atomicity;
- order;
- idempotency;
- duplication;
- partial consumption;

 belong to the formalization of `Policy Consumption` and `Account Transitions`.

 `PolicyConsumption` is a **Value Object / transition value** while no historical lifecycle of its own is discovered inside the `Account`.

 The formal treatment of `Policy Consumption` as an atomic transition is defined in Formal Laws — D2.

---

 ## Cross-References

 - Foundations — `Account` as the operational component.
- Authority Model — `Capability` and `Credential`.
- Derived Authority — `EffectiveAuthority` as derived from state.
- Execution Model — how `AuthorizationState` is consumed by the `Runtime`.
- Formal Laws — D2 — `PolicyEffect` algebra and atomic consumption.
- Formal Laws — D5 — `Restriction` association in `AuthorizationState`.
