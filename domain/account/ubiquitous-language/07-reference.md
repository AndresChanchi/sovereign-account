---
id: account-ul-reference
title: Reference
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "07"
type: reference
status: stable
section: "46-53"
depends-on:
  - account-ul-foundations
  - account-ul-authority-model
  - account-ul-authorization
  - account-ul-derived-authority
  - account-ul-state-and-policy
  - account-ul-execution-model
related:
  - account-formal-laws-index
formal-laws: []
verification:
  dafny: []
---

# Reference

> Original DDD sections: §46–§53.

This chapter consolidates the classification, identifier semantics, architectural principles, and the formalization rule of the Account domain.

---

## Entity and Value Object Classification

### Entities

```text
Identity
Account
Credential
Session
Delegation
````

 These entities have their own individual identity and lifecycle continuity.

 Their identifiers are:

```
IdentityId
AccountId
CredentialId
SessionId
DelegationId
```

 ### Value Objects

```
Subject
Capability
CapabilityKind
Scope
Restriction
CredentialAuthority
EffectiveAuthority
RequestedAuthority
Authorization
Policy
PolicyEffect
PolicyConsumption
DomainAction
ExecutionRequest
ExecutionContext
ExecutionConstraints
Timestamp
```

 ### Operational / Infrastructure Concepts

```
Proof
Verifier
Runtime
Execution Engine
Execution
Adapter
Blockchain
Gas Payment
Execution Sponsor
Authentication
```

 These must not be artificially assigned Entity identity within the `Account` domain.

---

 ## Identifier Semantics

 An **Identifier** represents a stable reference used to distinguish an Entity whose individual identity is part of its semantics.

 The current identifiers are:

```
Identity       → IdentityId
Account        → AccountId
Credential     → CredentialId
Session        → SessionId
Delegation     → DelegationId
```

 No identifiers are assigned to:

```
Capability
PolicyEffect
Authorization
ExecutionContext
DomainAction
Restriction
Scope
```

 because their meaning is determined by their content.

 ### Identifier ≠ representation

 An Identifier is not automatically:

 - a hash;
- a nonce;
- an address;
- a `B256`;
- a private key;
- a public key;
- a classification.

 ### Size

 The domain does not currently establish:

```
|Identifier| = 32 bytes
```

 The physical representation may be defined by infrastructure without necessarily changing the semantics of the Identifier.

 ### Uniqueness

 Each Entity must be distinguishable within the identity scope required by its lifecycle.

 The concrete way to guarantee that uniqueness belongs to the design of the state and corresponding infrastructure.

 ### Generation

 The domain does not require a single generation mechanism.

 It may be:

 - deterministic;
- random;
- derived;
- assigned;
- external;
- another compatible mechanism.

---

 ## Universal Account Stress Test

 Every new abstraction must be justified by a real and recurring problem.

 Before introducing a new Entity, Value Object, relationship, or rule, the following must be verified:

 ### 1\. Real problem

 What concrete and recurring problem does it solve?

 ### 2\. Reuse

 Does it appear in more than one application or context?

 ### 3\. Application independence

 Can it exist without knowing the specific business model of an application?

 ### 4\. Infrastructure independence

 Can it be expressed without depending on a concrete implementation?

 ### 5\. Composition

 Can it be solved by composing existing concepts?

 ### 6\. Semantics

 Does it represent a reality of the domain or an implementation need?

 ### 7\. Reuse across Subjects

 Can it be used with different types of `Subject`?

 ### 8\. Identity Requirement

 Does it need its own individual identity, or is its meaning fully determined by its values?

 The absence of a clear identity need must prevent introducing an artificial Identifier.

---

 ## Architectural Principles

 1. **`Subject`, `Identity`, `Account`, and Blockchain Address are different concepts.**
2. **External Identity and Authentication Mechanism are not automatically an `Identity`.**
3. **`Subject` represents the semantic actor.**
4. **`Identity` represents sovereign continuity.**
5. **An `Identity` may control multiple `Account` instances.**
6. **Each `Account` has exactly one sovereign `Identity`.**
7. **Authorizing another `Identity` over an `Account` does not create co-sovereignty.**
8. **`Account` is an Entity distinct from a Blockchain Address.**
9. **`Capability` is a Value Object.**
10. **`Capability` expresses a faculty, not an individual historical grant.**
11. **`Capability` does not need a `CapabilityId`.**
12. **`CapabilityKind` and `Scope` are Value Objects.**
13. **`Credential` is an Entity independent of `Identity`.**
14. **A `Credential` may be recognized by one or more `Account` instances.**
15. **`Credential` recognition is specific to each `Account`.**
16. **`CredentialAuthority` is a Value Object that describes the authority a `Credential` may attempt to exercise on an `Account`.**
17. **A `Credential` does not acquire sovereignty by being recognized by an `Account`.**
18. **A `Session` is a temporary Entity derived from a `Credential`.**
19. **`SessionAuthority` may never exceed `CredentialAuthority`.**
20. **`Delegation` is an Entity with its own lifecycle.**
21. **`DelegatedAuthority` may never exceed the `DelegatableAuthority` of the source.**
22. **`Restriction` is a Value Object that limits existing authority.**
23. **`EffectiveAuthority` is contextual, derived, and by value.**
24. **`Authorization` is a Value Object.**
25. **`Authorization` does not need an `AuthorizationId`.**
26. **`RequestedAuthority` is a Value Object.**
27. **`Proof Verification` and `Authorization Validation` are different responsibilities.**
28. **A valid `Proof` does not imply a valid `Authorization`.**
29. **`Policy` is an external decision recognized by an `Account` and does not need its own identity inside the `Account`.**
30. **`RecoveryPolicyRequest` and other historical entities belong to their producing bounded contexts.**
31. **`PolicyEffect` is a Value Object.**
32. **A `Policy` may produce one or more `PolicyEffect` instances.**
33. **`Policy Consumption` produces recognized changes on `AuthorizationState`.**
34. **`AuthorizationState` contains operational relationships between `Credential`, `Account`, `Session`, `Delegation`, `Capability`, and `PolicyEffect`.**
35. **`Authorization State Transition` represents valid changes of that state.**
36. **`DomainAction` belongs to the consuming bounded context.**
37. **`ExecutionRequest` is a Value Object.**
38. **`ExecutionContext` is a Value Object and represents a complete validated decision.**
39. **`Execution Constraints` do not create `Authority`.**
40. **`Runtime` coordinates the distributed evaluation of authority and the orchestration of execution.**
41. **`Runtime` is not the owner of all authorization rules.**
42. **`Execution Engine` materializes valid `ExecutionContext` instances and does not decide `Authority`.**
43. **`Execution` is an operational materialization process, not an Entity of the `Account` domain.**
44. **No `ExecutionId` is introduced.**
45. **An `Execution` may produce multiple operations when the infrastructure supports batching, multicall, or atomicity.**
46. **`Adapter` instances materialize the domain semantics on concrete infrastructures.**
47. **Blockchain is a fundamental part of the context.**
48. **Authentication, Authorization, and Gas Payment are different responsibilities.**
49. **`CapabilityScope` and `ExecutionTarget` are different concepts.**
50. **Metadata does not by itself modify authority.**
51. **Identifiers exist because certain Entities need individual identity.**
52. **Not every domain concept needs an Identifier.**
53. **An Identifier is not automatically a hash, nonce, address, `B256`, or cryptographic key.**
54. **The domain does not currently fix a concrete Identifier size.**
55. **`Replay Protection` is a transversal property and does not convert `Authorization` into an Entity.**
56. **A new abstraction must be justified by a real and recurring need.**
57. **Formal verification must be derived from the meaning of the domain and must not use Rust/EVM details to redefine the domain.**

---

 ## Consolidated Conceptual Model

```
                         EXTERNAL WORLD
                               │
                 ┌─────────────┴─────────────┐
                 │                           │
          External Identity          Authentication
          email / phone / EOA         mechanisms
                 │                           │
                 └─────────────┬─────────────┘
                               │
                         authentication
                           evidence
                               │
                               ▼
                           CREDENTIAL
                               │
                    recognized by Account
                               │
                               ▼
                           IDENTITY
                               │
                  sovereign ownership
                               │
                  ┌────────────┼────────────┐
                  │            │            │
               Account A    Account B    Account C
                  │            │            │
                  └────────────┼────────────┘
                               │
                     AUTHORIZATION STATE
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
   CAPABILITIES            CREDENTIALS            POLICIES
        │                      │                      │
        │              CREDENTIAL AUTHORITY       POLICY EFFECTS
        │                      │                      │
        │                   SESSIONS                 │
        │                      │                      │
        │                 DELEGATIONS                │
        │                      │                      │
        └──────────────────────┼──────────────────────┘
                               │
                         RESTRICTIONS
                               │
                               ▼
                     EFFECTIVE AUTHORITY
                               │
                    ┌──────────┴──────────┐
                    │                     │
              AUTHORIZATION         DOMAIN ACTION
                    │                     │
                    └──────────┬──────────┘
                               │
                      EXECUTION REQUEST
                               │
                               ▼
                            RUNTIME
                               │
                  distributed authority
                     / orchestration
                               │
                               ▼
                     EXECUTION CONTEXT
                               │
                               ▼
                      EXECUTION ENGINE
                               │
                               ▼
                            ADAPTER
                               │
                               ▼
                          BLOCKCHAIN
```

 Transversal infrastructure:

```
Proof
Verifier
Authentication mechanisms
Replay Protection
Gas Payment
Execution Sponsor
Privacy mechanisms
Blockchain Address representations
```

---

 ## Fundamental Distinction

```
SUBJECT
    =
SEMANTIC ACTOR

IDENTITY
    =
SOVEREIGN CONTINUITY

ACCOUNT
    =
OPERATIONAL COMPONENT THROUGH WHICH
AN IDENTITY EXERCISES AUTHORITY ON BLOCKCHAIN

CAPABILITY
    =
WHAT AUTHORITY EXISTS

CREDENTIAL
    =
RECOGNIZED SOURCE FOR PRODUCING
AUTHORIZATION EVIDENCE

CREDENTIAL AUTHORITY
    =
WHAT A CREDENTIAL MAY ATTEMPT TO EXERCISE
ON AN ACCOUNT

AUTHORIZATION
    =
WHAT AUTHORITY IS REQUESTED / EVIDENCED

RESTRICTION / SCOPE / SESSION / DELEGATION
    =
UNDER WHAT CONDITIONS

EFFECTIVE AUTHORITY
    =
WHAT MAY ACTUALLY BE EXERCISED IN CONTEXT

AUTHORIZATION STATE
    =
OPERATIVE AUTHORITY STATE

RUNTIME
    =
DISTRIBUTED AUTHORITY / EXECUTION ORCHESTRATION

EXECUTION CONTEXT
    =
COMPLETE VALID EXECUTION DECISION

EXECUTION ENGINE
    =
EXECUTION MATERIALIZATION

EXECUTION
    =
OPERATIONAL MATERIALIZATION PROCESS

ADAPTER
    =
INFRASTRUCTURE-SPECIFIC MATERIALIZATION
```

 And the fundamental separations:

```
External Identity
    ≠
Subject
    ≠
Identity
    ≠
Account
    ≠
Blockchain Address
```

 and:

```
Capability
    ≠
Credential Authority
    ≠
Effective Authority
    ≠
Authorization
    ≠
Execution
```

---

 ## Closed Domain Decisions

 The following decisions about the nature of the concepts are established:

```
Subject
    → Value Object

Identity
    → Entity

Account
    → Entity

Capability
    → Value Object

CapabilityKind
    → Value Object (with the semantic exception of `Delegate`, which is a special CapabilityKind of the domain)

Scope
    → Value Object

Restriction
    → Value Object

Credential
    → Entity

CredentialAuthority
    → Value Object

Session
    → Entity

Delegation
    → Entity

EffectiveAuthority
    → Value Object

RequestedAuthority
    → Value Object

Authorization
    → Value Object

Policy
    → recognized external decision/value

PolicyEffect
    → Value Object

PolicyConsumption
    → Value Object / transition value

DomainAction
    → external Value Object

ExecutionRequest
    → Value Object

ExecutionContext
    → Value Object

ExecutionConstraints
    → Value Object

Execution
    → operational process, not Entity
```

 And the fundamental cardinalities:

```
Identity 1
    └── 0..N Accounts

Account 1
    └── exactly 1 sovereign Identity

Credential 1
    └── 0..N recognized Accounts

Subject 1
    └── 0..N Identities
```

---

 ## Rule for the Formalization

 The formal priority is:

```
DDD semantic invariant
        ↓
formal domain model
        ↓
formal law
        ↓
proof
        ↓
Rust / blockchain representation
```

 Never:

```
Rust / EVM convenience
        ↓
formal type
        ↓
DDD retrofitted afterwards
```

 When the formalization finds an ambiguity:

```
formalization ambiguity
      ↓
DDD clarification
      ↓
formal law
      ↓
proof
```

 An ambiguity of the domain must not be resolved simply by introducing a convenient structure in the formal model.

---

 ## Cross-References

 - Foundations — purpose, `Subject`, `Identity`, `Account`.
- Authority Model — `Capability`, `Credential`, `CredentialAuthority`.
- Authorization — `Authorization`, `Proof`, `Validation`.
- Derived Authority — `Session`, `Delegation`, `EffectiveAuthority`.
- State and Policy — `AuthorizationState`, `PolicyEffect`, `PolicyConsumption`.
- Execution Model — `Runtime`, `ExecutionContext`, `Adapter`.
- Formal Laws — `D1`–`D6`.
