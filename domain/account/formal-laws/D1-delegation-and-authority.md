---
id: account-law-D1
title: D1 — Delegation and Authority
domain: sovereign-account
chapter: formal-laws
type: law-set
status: stable
law-id: D1
laws:
  - D1.1
  - D1.2
  - D1.3
  - D1.4
  - D1.5
  - D1.6
  - D1.7
  - D1.8
  - D1.9
  - D1.10
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
depends-on:
  - account-ul-foundations
  - account-ul-authority-model
  - account-ul-derived-authority
related:
  - account-law-D2
  - account-law-D3
  - account-law-D4
verification:
  dafny:
    - domain/account/formal/laws/DelegationLaws.dfy
    - domain/account/formal/laws/AuthorityLaws.dfy
    - domain/account/formal/laws/SessionLaws.dfy
    - domain/account/formal/laws/IdentityLaws.dfy
    - domain/account/formal/laws/CapabilityLaws.dfy
    - domain/account/formal/laws/AuthorizationLaws.dfy
    - domain/account/formal/laws/ExecutionLaws.dfy
    - domain/account/formal/laws/PolicyLaws.dfy
---

# D1 — Delegation and Authority

This document constitutes the semantic formalization of the delegation and authority derivation domain of Sovereign Account.

The laws defined here are independent of:

- Dafny;
- Rust;
- EVM;
- blockchain addresses;
- cryptographic primitives;
- storage representations;
- implementation-specific data structures.

The implementation must demonstrate these laws, not redefine them.

---

## Formal Notation

The following conceptual relations are used:

- `Sovereign(A)` — represents the sovereign `Identity` of an `Account`.
- `Authority(I, A, C, context)` — represents the authority an `Identity` may exercise over an `Account` regarding a `Capability` under a given context.
- `EffectiveAuthority(I, A, context)` — represents the effectively exercisable authority.
- `DelegatableAuthority(I, A, context)` — represents the authority the `Identity` may legitimately confer to another subject.
- `DelegatedAuthority(D)` — represents the authority conferred by a `Delegation`.
- `Source(D)` — represents the `Identity` that originates the `Delegation`.
- `Delegatee(D)` — represents the `Subject` that receives the `Delegation`.

These expressions are not presupposed to correspond to additional Entities or Value Objects. They are derived semantic relations.

---

## Sovereignty Laws

### D1.1 Exact Sovereignty

For every `Account`, there exists exactly one sovereign `Identity`.

Formally:

```text
∀ account :
    ∃! identity :
        Sovereign(identity, account)
```

That is:

```text
|SovereignIdentities(account)| = 1
```

An `Account` may not have:

- 0 sovereign identities;
- 2+ sovereign identities;

as a result of authorization relationships.

### D1.2 Sovereignty Is Identity-Bound

Sovereignty belongs to `Identity`, not to:

- `Credential`;
- `Session`;
- `Delegation`;
- `Capability`;
- Blockchain Address;
- `Sponsor`;
- `Proof`.

Therefore:

```text
Sovereign(Account) = Identity
```

and never:

```text
Sovereign(Account) = Credential
Sovereign(Account) = Delegatee
```

### D1.3 Authorization Does Not Create Sovereignty

If an `Identity` receives any authority over an `Account`:

```text
Authorized(B, Account)
```

it does not follow that:

```text
Sovereign(B, Account)
```

Therefore:

```text
Authorized(B, Account) ⇏ Sovereign(B, Account)
```

This includes authority obtained through:

- `Credential`;
- `Session`;
- `Delegation`;
- `PolicyEffect`.

### D1.4 Delegation Does Not Transfer Sovereignty

For every `Delegation` `D`:

```text
Source(D) ≠ sovereign transfer
```

Specifically:

```text
SovereignIdentity(Account(D))
```

remains immutable with respect to the existence of the `Delegation`.

If:

```text
X = SovereignIdentity(Account)
```

and:

```text
D = X → B
```

then:

```text
SovereignIdentity(Account) = X
```

after creating `D`.

---

## Authority Origin Laws

An `Identity` may obtain authority from two semantic sources:

- **sovereign authority**;
- **derived authority**.

Derived authority may proceed from:

- `Credential`;
- `Session`;
- `Delegation`;
- `Policy State`;

according to the relationships recognized by `AuthorizationState`.

### D1.5 Sovereign Authority

The sovereign `Identity` of an `Account` constitutes the root source of authority of that `Account`.

Therefore:

```text
Account
    ↓
Sovereign Identity
    ↓
Account Authorization State
    ↓
Authority
```

Derived authority does not create a new sovereign root.

### D1.6 Derived Authority

Every derived authority must be limited by the authority of its source.

Formally:

```text
DerivedAuthority(child) ⊆ SourceAuthority(source)
```

This law specializes for:

- `Session`;
- `Delegation`.

---

## Capability Laws

### D1.7 Capability Represents Authority

A `Capability` represents a semantic faculty:

```text
Capability = Kind + Scope + relevant semantic restrictions
```

The existence of a `Capability` does not imply that any `Identity` may exercise it.

Therefore:

```text
Capability exists ⇏ Identity may exercise Capability
```

Effective authority depends on `AuthorizationState` and context.

### D1.8 Capability Is Not a Grant

A `Capability` does not represent a historical grant.

Therefore:

```text
Capability ≠ Grant Event
```

The same `Capability` may appear in different authority relationships:

```text
Credential A → Upload(X)
Credential B → Upload(X)
```

without them necessarily being the same historical entity.

---

## Credential Authority Laws

### D1.9 Credential Authority Is Account-Relative

The authority of a `Credential` is not absolute. It must be evaluated with respect to an `Account`:

```text
CredentialAuthority(Credential, Account)
```

Therefore, the same `Credential` may have different authority over different `Account` instances.

### D1.10 Credential Authority Bounds Requested Authority

For a `Credential` to attempt to exercise a `RequestedAuthority`:

```text
RequestedAuthority ⊆ CredentialAuthority
```

must hold as a necessary condition.

Therefore:

```text
RequestedAuthority ⊄ CredentialAuthority ⇒ Authorization invalid
```

But:

```text
RequestedAuthority ⊆ CredentialAuthority
```

is not by itself sufficient to produce `EffectiveAuthority`.

---

## Session Laws

### D1.11 Session Is Derived Authority

A `Session` derives its authority from a `Credential`.

Therefore:

```text
SessionAuthority(S) ⊆ CredentialAuthority(SourceCredential(S))
```

This is a non-expansion law.

### D1.12 Session Cannot Increase Credential Authority

Never:

```text
SessionAuthority ⊃ CredentialAuthority
```

Therefore a `Session` may:

- reduce;
- restrict;
- scope;
- limit;
- expire;

but never:

- invent;
- expand;
- increase;

the authority of its `Credential`.

### D1.13 Credential Revocation and Session Usability

If a `Credential` stops being valid (by revocation or another cause), any `Session` that depends on it (whose authority derives from that `Credential`) stops being usable as a source of authority.

Formally:

```text
CredentialRevoked(C) ⇒ ¬SessionIsUsable(S)
```

for every `Session` `S` that has `C` as `SourceCredential`.

The `Session` **does not change its identity or its persistent state** (e.g. it remains `Active`). However, it **stops contributing to `EffectiveAuthority`**; its contribution becomes `∅` while the `Credential` is not valid.

This maintains the distinction between:

- **Entity lifecycle** (identity and persistent state);
- **Effective usability** (capacity to contribute to authority).

It is not necessary to change the persistent state of the `Session` to `Revoked` when the `Credential` is revoked; invalidity propagates through the authority rule.

---

## Delegation Laws

### D1.14 Every Delegation Has a Source

Every `Delegation` has exactly one source `Identity`:

```text
∀ delegation :
    ∃! identity :
        Source(identity, delegation)
```

The `Source` is the `Identity` that exercises the delegatable authority that originates that `Delegation`.

### D1.15 Root Delegation

When a `Delegation` originates directly from the sovereign authority of an `Account`:

```text
Source(D) = SovereignIdentity(Account(D))
```

Therefore, if `Account X` has sovereign `Identity A`, then `A → B` is a valid root `Delegation` if `A` possesses the necessary authority to delegate.

### D1.16 Delegation Does Not Make the Source Sovereign

If:

```text
B → C
```

then `B` is the source of that `Delegation`, but:

```text
B ≠ SovereignIdentity(Account)
```

unless `B` was already sovereign independently.

That is:

```text
Source(D) ≠ SovereignIdentity(Account)
```

may be perfectly valid. This is precisely what allows transitive delegation without co-sovereignty.

---

## Delegation Authority Laws

### D1.17 Delegation Requires Delegation Authority

Possessing any `Capability` is not sufficient to produce a `Delegation`. Authority to delegate must exist.

`Delegate` is a special semantic `CapabilityKind` of the domain, not defined by the consumer. Its semantics: **authorizes the act of delegating other Capabilities**.

Conceptually:

```text
CanDelegate(I, Account)
```

requires that `I` possesses the `Capability` corresponding to `Delegate` within the applicable context.

Therefore:

```text
Capability(Edit) ⇏ CanDelegate
```

while:

```text
Capability(Delegate)
```

is the faculty that enables the act of delegating.

### D1.18 Delegate Is Itself a Capability

`Delegate` is not a technical property added to a `Capability` (such as `delegatable = true`), nor is `DelegatableCapability` introduced as an independent concept. Instead, `Delegate` is a `Capability`.

Therefore:

```text
CanExercise(Delegate)
```

is the authority necessary to perform the act of delegation.

---

## Delegatable Authority

The authority an `Identity` may delegate is limited by:

- the authority it actually possesses;
- its authority to perform `Delegate`;
- the restrictions the domain applies to the act of delegation.

Therefore:

```text
DelegatableAuthority(I) ⊆ EffectiveAuthority(I)
```

and the existence of `Delegate` is a necessary condition to produce a `Delegation`.

### D1.19 Delegatable Authority Cannot Exceed Effective Authority

Formally:

```text
DelegatableAuthority(I, A, context) ⊆ EffectiveAuthority(I, A, context)
```

Therefore:

> An `Identity` cannot delegate authority that it cannot legitimately exercise.

### D1.20 The Delegate Capability Does Not Mean Everything Is Automatically Delegable

The `Delegate` capability authorizes the act of delegating. It does not mean the source may delegate authority it never received.

Therefore:

```text
Delegate + Edit
```

may permit:

```text
delegate Edit
```

but:

```text
Delegate + Edit
```

does not permit:

```text
delegate Transfer
```

if:

```text
Transfer ∉ EffectiveAuthority(source)
```

---

## Delegated Authority Laws

### D1.21 Delegated Authority Is Bounded by Source

Central law:

```text
DelegatedAuthority(D) ⊆ DelegatableAuthority(Source(D))
```

Therefore, if `B` may delegate `Edit(X)`, then `B → C, Edit(X)` is valid. But `B → C, Transfer(X)` is invalid if `B` cannot delegate `Transfer(X)`.

### D1.22 Delegation May Restrict

The `Delegation` may confer a part of the available authority:

```text
DelegatedAuthority(D) ⊂ DelegatableAuthority(Source(D))
```

This is valid. The `Delegation` does not have to copy all of the source's authority.

Example:

```text
B: Edit, View, Share, Delegate
B → C: View
```

is perfectly valid.

### D1.23 Delegation Cannot Expand

Never:

```text
DelegatedAuthority(D) ⊃ DelegatableAuthority(Source(D))
```

A `Delegation` may only:

- preserve;
- restrict;

authority. Never expand.

---

## Transitive Delegation Laws

### D1.24 Transitive Delegation

If:

```text
A → B
```

and `B` received sufficient authority to delegate, then `B` may produce:

```text
B → C
```

without becoming sovereign of the original `Account`.

Therefore:

```text
A
│
└── delegates → B
                 │
                 └── delegates → C
```

is a chain of derived authority.

### D1.25 Transitive Delegation Is Monotonic Downward

Authority cannot increase along a chain.

For:

```text
D1: A → B
D2: B → C
```

it must hold that:

```text
DelegatedAuthority(D2) ⊆ DelegatedAuthority(D1)
```

if `D2` derives exclusively from the authority received through `D1`.

Therefore:

```text
C authority ⊆ B authority ⊆ A authority
```

with respect to the corresponding delegation chain.

### D1.26 Transitive Delegation Preserves Sovereignty

For:

```text
A → B → C
```

sovereignty remains:

```text
Sovereign(Account) = A
```

even though:

```text
Source(D2) = B
and
Delegatee(D2) = C
```

Therefore:

```text
Source ≠ Sovereign
```

in general.

---

## Delegation Restriction Laws

### D1.27 Delegatee Receives No More Than Explicitly Delegated

The `Delegatee` does not automatically receive all of the source's authority.

If:

```text
A → B {Edit, View}
```

then:

```text
B delegated authority = {Edit, View}
```

not:

```text
B delegated authority = all A authority
```

### D1.28 Restrictions Are Monotonic

If a `Delegation` contains `Restriction` instances:

```text
DelegatedAuthority(D) + Restrictions(D)
```

the resulting authority cannot exceed the original delegated authority.

Formally:

```text
EffectiveAuthority(delegatee) ⊆ DelegatedAuthority(D)
```

before considering later restrictions.

---

## Effective Authority Laws

### D1.29 Effective Authority Is Contextual

There is no single absolute `EffectiveAuthority` for an `Identity`. It must be evaluated with respect to:

- `Identity`;
- `Account`;
- `Context`;
- `Time`;
- `AuthorizationState`.

Therefore:

```text
EffectiveAuthority(I, Account X, Context A)
```

may differ from:

```text
EffectiveAuthority(I, Account X, Context B)
```

The `Context` is an **input parameter** for the function that derives `EffectiveAuthority`. It does not form part of the identity of the resulting Value Object. The equality of `EffectiveAuthority` is determined exclusively by the set of `Capability` instances resulting from the derivation.

### D1.30 Effective Authority Is Derived

`EffectiveAuthority` is not an independent grant. It is derived from the sources and restrictions recognized by `AuthorizationState`.

Conceptually:

```text
Effective Authority =
Available Authority
∩ Credential Authority
∩ Session Authority
∩ Delegated Authority
∩ Scope
∩ Restrictions
∩ Temporal Conditions
∩ Policy Effects
∩ Execution Context
```

The `ExecutionContext` and other conditions are **inputs of the derivation**; they are not part of the resulting value. Not all intersections are necessarily literal in an implementation; they express the semantics of monotonic restriction.

### D1.31 Derived Authority Cannot Increase

Every transformation of derived authority must be monotonic downward.

If:

```text
A₂
```

is authority derived from:

```text
A₁
```

then:

```text
A₂ ⊆ A₁
```

This applies to:

- `Session`;
- `Delegation`;
- `Restriction`;
- `Scope` limitation;
- temporal limitation;

and other mechanisms whose purpose is to derive or restrict authority.

---

## Authorization Laws

### D1.32 Authorization Requires Effective Authority

An `Authorization` may only be accepted if:

```text
RequestedAuthority ⊆ EffectiveAuthority
```

and the other validation conditions are satisfied.

Therefore:

```text
RequestedAuthority ⊄ EffectiveAuthority ⇒ Authorization invalid
```

### D1.33 Proof Does Not Create Authority

A valid `Proof` demonstrates cryptographic evidence. It does not modify:

- `Capability` instances;
- `CredentialAuthority`;
- `SessionAuthority`;
- `DelegatedAuthority`;
- `EffectiveAuthority`.

Therefore:

```text
ValidProof ⇏ AuthorityExists
```

### D1.34 Authentication Does Not Create Authority

Similarly:

```text
AuthenticatedCredential ⇏ UnlimitedAuthority
```

`Authentication` determines the source of evidence. `Authorization` determines what authority may be exercised.

---

## Policy Laws

### D1.35 Policy Effects Modify Authorization State

A recognized `Policy` may produce a `PolicyEffect` that generates a valid `AuthorizationStateTransition`.

Therefore:

```text
Policy
    ↓
Policy Effect
    ↓
Authorization State Transition
```

### D1.36 Policy Cannot Violate Domain Invariants

A `PolicyEffect` cannot produce an `AuthorizationState` that violates the fundamental laws of the domain.

For example, a `Policy` cannot produce:

```text
Account → two sovereign Identities
```

nor:

```text
Delegation → authority greater than source authority
```

`PolicyEffect` instances are subject to the same invariants as any other transition.

---

## Authorization State Laws

### D1.37 Authorization State Is the Operational Authority State

The relationships between:

- `Account`;
- `Credential`;
- `CredentialAuthority`;
- `Session`;
- `Delegation`;
- `Capability`;
- `PolicyEffect`;
- `Restriction`;

when they form part of the operational authority of an `Account`, belong to `AuthorizationState`.

Therefore, `EffectiveAuthority` must be derivable from the relevant state.

### D1.38 State Transition Preservation

Every valid `AuthorizationStateTransition` must preserve the invariants of the domain.

Formally:

```text
ValidState(S)
∧
ValidTransition(S, T)
    ⇒
ValidState(T)
```

This law is especially important for:

- `Credential` registration;
- `Credential` revocation;
- `Session` creation;
- `Session` revocation;
- `Delegation` creation;
- `Delegation` revocation;
- `Capability` changes;
- `PolicyEffect` instances.

---

## Revocation Laws

### D1.39 Revocation Removes Usability, Not Historical Identity

Revoking an Entity with a lifecycle (`Credential`, `Session`, `Delegation`) does not create a new Entity.

For example:

```text
Delegation D: active → revoked
```

remains:

```text
DelegationId = D
```

### D1.40 Revoked Delegation Contributes No Effective Authority

If:

```text
Delegation D = revoked
```

then:

```text
DelegatedAuthority(D)
```

cannot contribute to `EffectiveAuthority`.

Therefore:

```text
Revoked(D) ⇒ EffectiveAuthority derived through D = ∅
```

in the sense of usable authority.

### D1.41 Revocation Is Monotonic for Authority

Revoking a source of authority cannot increase effective authority.

If:

```text
State₁ → revoke(D) → State₂
```

then:

```text
EffectiveAuthority(State₂) ⊆ EffectiveAuthority(State₁)
```

for the affected context.

---

## Scope Laws

### D1.42 Scope Restricts Authority

`Scope` does not create a new `Capability`. It restricts where an existing `Capability` may be exercised.

Therefore:

```text
Capability(Upload) + Scope(Album123)
```

represents:

```text
Upload over Album123
```

and not:

```text
Upload everywhere
```

### D1.43 Scope Cannot Expand Source Authority

A derived `Scope` cannot expand the `Scope` of the source authority.

```text
DerivedScope ⊆ SourceScope
```

when the authority is derived.

---

## Restriction Laws

### D1.44 Restrictions Are Non-Expansive

A `Restriction` may only reduce the conditions of exercise.

Therefore:

```text
Authority after Restriction ⊆ Authority before Restriction
```

### D1.45 Restrictions Compose Monotonically

If an authority is subject to `Restriction` A and then `Restriction` B, then:

```text
Authority(A + B) ⊆ Authority(A)
```

and:

```text
Authority(A + B) ⊆ Authority(B)
```

when both represent restrictions over the same authority.

---

## Execution Laws

### D1.46 Execution Requires a Valid Execution Context

The `Execution Engine` must only receive:

```text
Valid(ExecutionContext)
```

Therefore:

```text
Invalid ExecutionContext ⇒ Execution must not be materialized
```

### D1.47 Execution Engine Does Not Decide Authority

The `Execution Engine` consumes the `ExecutionContext` decision, but does not redefine:

- `EffectiveAuthority`;
- `Authorization`;
- `Capability`;
- `Delegation`;
- `Policy`.

### D1.48 Execution Does Not Create Authority

A successful `Execution` does not by itself produce:

- `Capability`;
- `CredentialAuthority`;
- `Delegation`;
- sovereignty;

unless an explicit domain transition establishes it.

---

## Gas and Sponsorship Laws

### D1.49 Payment Does Not Grant Authority

If:

```text
Sponsor pays Execution
```

it does not follow that:

```text
Sponsor may execute
```

Therefore:

```text
GasPayment ≠ Authority
```

and:

```text
Sponsor ≠ Sovereign
```

---

## Global Monotonic Authority Principle

### D1.50 Authority Non-Expansion

No authority derivation mechanism may create authority greater than the authority it legitimately receives.

For every derived authority:

```text
DerivedAuthority ⊆ SourceAuthority
```

This applies to:

- `Session`;
- `Delegation`;
- `Restriction`;
- `Scope`;
- Policy-derived state;

according to their derivation relationship.

The only way to increase the effective authority of an `Identity` is a legitimate `AuthorizationState` transition that modifies the available authority at its corresponding source.

---

## Delegation Chain Theorem

The previous laws allow expressing the fundamental property of delegation chains.

For a chain:

```text
I₀
 │
 └── D₁ → I₁
             │
             └── D₂ → I₂
                         │
                         └── D₃ → I₃
```

it holds that:

```text
Authority(I₃) ⊆ Authority(I₂) ⊆ Authority(I₁) ⊆ Authority(I₀)
```

with respect to the authority derived by that chain.

And simultaneously:

```text
SovereignIdentity(Account) = I₀
```

if `I₀` is the root source.

Therefore:

```text
I₁ ≠ sovereign
I₂ ≠ sovereign
I₃ ≠ sovereign
```

simply by participating in the chain.

---

## Delegation Cannot Escape Its Origin

A fundamental consequence is:

A `Delegation` cannot semantically escape the authority of its origin.

If:

```text
A → B
```

and:

```text
B → C
```

then `C` cannot obtain:

```text
authority X
```

if `B` did not receive:

```text
authority X
```

from `A` or from another legitimate independent source.

Formally:

```text
X ∈ Authority(C) ⇒ X ∈ Authority(B)
```

for authority derived exclusively from the chain.

And:

```text
X ∈ Authority(B) ⇒ X ∈ Authority(A)
```

if `B` received `X` exclusively through `A`.

---

## Multiple Independent Sources

An `Identity` may receive authority from more than one source.

For example:

```text
A → B : Edit
C → B : Transfer
```

Then:

```text
Authority(B) = AuthorityFrom(A) ∪ AuthorityFrom(C)
```

Therefore, **it is not** formalized that all of `B`'s authority is always a subset of `A`'s authority.

The correct law is:

```text
AuthorityDerivedFrom(A, B) ⊆ Authority(A)
```

Transitivity applies by provenance/source of the authority, not necessarily to all accumulated authority of the `Identity`.

---

## Provenance of Derived Authority

The semantics of derived authority must conceptually preserve its origin.

This does not mean necessarily introducing a new Entity or Value Object. It means the formalization must be able to express:

```text
Authority B
    ├── derived from A
    └── derived from C
```

and:

```text
Delegation B → D
```

may only reuse each part of authority according to the source that legitimately enables it.

It must hold that:

```text
RequestedDelegatedAuthority ⊆ B's DelegatableAuthority
```

and that authority must be backed by legitimate sources.

---

## Formal Definition of Delegation

A `Delegation` is an Entity through which an `Identity` that possesses delegatable authority confers to a `Delegatee` a derived authority, explicitly delimited by `Scope`, `Restriction` instances, and other applicable conditions, without transferring the sovereignty of the source `Account`.

Its fundamental laws are:

1. Exactly one source.
2. Source must possess `Delegate` authority.
3. Source must possess the authority being delegated.
4. `DelegatedAuthority ⊆ DelegatableAuthority(Source)`.
5. `DelegatedAuthority` may be strictly smaller than the source's delegatable authority.
6. `Delegation` cannot increase authority.
7. `Delegatee` does not become sovereign.
8. Transitive delegation is permitted when the delegatee possesses `Delegate` authority.
9. Transitive delegation cannot exceed the authority received through the relevant source chain.
10. Revocation removes the delegation's contribution to `EffectiveAuthority`.
11. `Delegation` does not transfer `Capability` ownership or `Identity` sovereignty.
12. Multiple independent authority sources may coexist.

---

## Summary of D1 Invariants

The domain preserves the following invariants:

**D1.1**  
Every `Account` has exactly one sovereign `Identity`.

**D1.2**  
Authorization of another `Identity` never creates co-sovereignty.

**D1.3**  
Every `Delegation` has exactly one source `Identity`.

**D1.4**  
A root `Delegation` originates from the sovereign `Identity` of the `Account` or from another legitimate authority source recognized by `AuthorizationState`.

**D1.5**  
Creating a `Delegation` never changes `Account` sovereignty.

**D1.6**  
`Delegation` requires authority to exercise `Delegate`.

**D1.7**  
`DelegatedAuthority` must be contained within the source's `DelegatableAuthority`.

**D1.8**  
`DelegatableAuthority` must be contained within the authority legitimately available to the source.

**D1.9**  
`Delegation` may restrict the authority received.

**D1.10**  
`Delegation` may never expand the authority received.

**D1.11**  
A `Session` cannot exceed the authority of its `Credential`.

**D1.12**  
A derived authority cannot exceed its source authority.

**D1.13**  
`Delegation` may be transitive.

**D1.14**  
Transitive delegation cannot exceed the authority received through the corresponding delegation chain.

**D1.15**  
A `Delegatee` does not become sovereign merely by receiving authority.

**D1.16**  
Revocation of a `Delegation` removes its authority contribution without creating a new `Delegation` Entity.

**D1.17**  
Authority from independent sources may be combined.

**D1.18**  
A delegation chain constrains only the authority derived through that chain, not unrelated authority independently obtained by the same `Identity`.

**D1.19**  
`EffectiveAuthority` is contextual and derived. The context is an input parameter, not a stored attribute of the Value Object.

**D1.20**  
`Authorization` is valid only when `RequestedAuthority` is contained within `EffectiveAuthority` and all other `AuthorizationValidation` conditions hold.

---

## Relationship Between DelegatableAuthority and EffectiveAuthority

The correct formulation is:

```text
DelegatableAuthority ⊆ EffectiveAuthority
```

and `Delegate` is a necessary `Capability` to perform the act of delegation.

The formal semantics are:

```text
Effective Authority
        │
        ├── authority I may exercise
        │
        ▼
Delegatable Authority
        │
        ├── authority I may confer
        │
        ▼
Delegated Authority
        │
        └── restricted subset
```

with:

```text
DelegatedAuthority ⊆ DelegatableAuthority ⊆ EffectiveAuthority
```

and additionally:

```text
CanDelegate ⇐ EffectiveAuthority includes Delegate
```

---

## Conceptual Model of D1

```text
                     Identity X
                         │
                         │ sovereign
                         ▼
                      Account X
                         │
                         ▼
                Authorization State
                         │
              ┌──────────┴──────────┐
              │                     │
        Effective Authority      Delegate
              │                     │
              └──────────┬──────────┘
                         ▼
               Delegatable Authority
                         │
                    Delegation
                         │
                         ▼
                     Identity B
                         │
                         ▼
               Delegated Authority
                         │
                         ├── restricted
                         │
                         ▼
               B's Effective Authority
                         │
                    Delegate?
                         │
                         ▼
               B's Delegatable Authority
                         │
                    Delegation
                         │
                         ▼
                     Identity C
```

The two fundamental properties are:

```text
DelegatedAuthority ⊆ DelegatableAuthority(source) ⊆ EffectiveAuthority(source)
```

and:

```text
SovereignIdentity(Account X) = X
```

independently of how many delegations exist.

**Note on the formalization:** `Authority(C) ⊆ Authority(B) ⊆ Authority(A)` is not formalized as a global property of `Identity` instances, since an `Identity` may receive authority from multiple sources. The correct formulation is by chain/provenance:

```text
AuthorityDerivedThrough(D₂) ⊆ AuthorityDerivedThrough(D₁)
```

and only if `D₂` derives from `D₁`. The total authority of an `Identity` may be the union of authority coming from different chains.

---

## Cross-References

- [Foundations](../ubiquitous-language/01-foundations.md) — `Identity`, `Account`, and sovereignty.
- [Authority Model](../ubiquitous-language/02-authority-model.md) — `Capability`, `Credential`, `CredentialAuthority`.
- [Derived Authority](../ubiquitous-language/04-derived-authority.md) — `Session`, `Delegation`, `EffectiveAuthority`.
- [Formal Laws — D2](D2-policy-effect-and-state.md) — `PolicyEffect` algebra.
- [Formal Laws — D3](D3-entity-identity-and-recognition.md) — identity immutability and recognition.
- [Formal Laws — D4](D4-proof-and-authorization-equality.md) — `Authorization` semantic value.
