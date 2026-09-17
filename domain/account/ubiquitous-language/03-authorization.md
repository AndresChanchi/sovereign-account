---
id: account-ul-authorization
title: Authorization
domain: sovereign-account
chapter: ubiquitous-language
chapter-number: "03"
type: concept
status: stable
section: "15-19, 42"
value-objects: [Proof, Verifier, RequestedAuthority, Authorization, ReplayProtection]
depends-on:
  - account-ul-foundations
  - account-ul-authority-model
related:
  - account-ul-derived-authority
  - account-ul-execution-model
formal-laws:
  - D1.32
  - D1.33
  - D1.34
  - D4.1
  - D4.2
  - D4.3
  - D4.4
  - D4.5
  - D4.6
  - D4.7
verification:
  dafny:
    - domain/account/formal/authorization/Authorization.dfy
    - domain/account/formal/authorization/AuthorizationValidation.dfy
    - domain/account/formal/authorization/Replay.dfy
---

 # Authorization

 > Original DDD sections: §15–§19, §42.

 This chapter defines how a request to exercise authority is represented, how cryptographic evidence attaches to it, and how its semantic value is distinguished from its evidence.

---

 ## Proof

 A **Proof** is cryptographic evidence used to demonstrate that an `Authorization` was produced through the corresponding `Credential`.

 `Proof` belongs to cryptographic infrastructure.

 It does not define:

 - `Identity`;
- `Capability`;
- `Scope`;
- `Authority`;
- `Authorization` validity.

 ### Principle

 > **Proof validity is not authorization validity.**

---

 ## Verifier

 A **Verifier** verifies a `Proof` using a specific cryptographic mechanism.

 It may use:

 - P-256;
- secp256k1;
- BLS;
- post-quantum cryptography;
- other mechanisms.

 The `Verifier` determines:

 > **whether the evidence satisfies the cryptographic rules of its mechanism.**

 It does not determine:

 > **whether the exercise of authority is permitted.**

---

 ## Requested Authority

 **Requested Authority** represents the authority an `Authorization` attempts to exercise.

 It may include:

 - `Capability` instances;
- `Scope`;
- `Restriction` instances;
- temporal conditions;
- context;
- other relevant conditions.

 `RequestedAuthority` is a **Value Object**.

 It does not grant authority.

 It represents only what an `Authorization` requests to exercise.

---

 ## Authorization

 An **Authorization** represents a verifiable request or evidence of an exercise of authority.

 It contains or references:

 - `Credential`;
- `RequestedAuthority`;
- `Restriction` instances;
- temporal conditions;
- `ReplayProtection`;
- `Proof`;
- **relevant Context (defined below)**.

 `Authorization` is a **Value Object**.

 Two `Authorization` instances are equal when they have the same complete semantic value.

 It does not need an `AuthorizationId`.

 ### Authorization Context

 The **Context** of an `Authorization` is the set of semantic elements that determine the scope and destination of the authorization.

 It forms part of the semantic value of `Authorization` and therefore affects its equality.

 It is composed of:

 - **`AccountId`**: the `Account` over which the authority is exercised.
- **`ExecutionTarget`**: the technical destination of the execution (e.g. contract, resource, service).
- **`DomainAction`**: the business action requested (defined by the consumer).
- **`Scope`**: the domain of the action (may be the same as the `Capability` `Scope` or a more restrictive one).
- **`Chain`**: identifier of the blockchain environment (if applicable), which allows distinguishing between different environments or networks.

 **Principle:**\
 The Context is part of the authorized intent. Two `Authorization` instances with identical attributes but different Context represent different requests and are not equal.

 **Example:**\
 `Authorization A`: `Credential X, RequestedAuthority Upload, Scope Album123, Account A, ExecutionTarget Contract1, DomainAction UploadPhoto`\
 `Authorization B`: same `Credential` and `RequestedAuthority`, but `ExecutionTarget Contract2`. A and B are semantically different.

 ### Main rule

 An `Authorization` may be accepted only when:

```
Requested Authority
    ⊆
Effective Authority
```

 and the other `AuthorizationValidation` conditions are satisfied (including the validity of the Context relative to the state of the `Account`).

 Therefore:

```
Valid Proof
    ≠
Valid Authorization
```

 ### Replay

 `replayKey` or any equivalent mechanism is part of `ReplayProtection`.

 It must not be automatically confused with an Entity identity.

---

 ## Authorization Validation

 **Authorization Validation** determines whether an `Authorization` may be accepted by an `Account` in a given state and context.

 It must consider:

 1. recognized `Credential`;
2. valid `Proof`;
3. compatible `CredentialAuthority`;
4. existing `Capability` instances;
5. `Scope`;
6. `Restriction` instances;
7. `Session` instances;
8. `Delegation` instances;
9. `PolicyEffect` instances;
10. temporality;
11. `ReplayProtection`;
12. **Context** (compatible with the `Account` and the `ExecutionTarget`, among others).

 ### Principle

 > **Proof Verification validates cryptographic evidence; Authorization Validation determines domain authority.**

 Validation of authority may be distributed across different bounded contexts or modules.

---

 ## Replay Protection

 **Replay Protection** ensures that an `Authorization` or `Execution` cannot be reused outside the conditions for which it was created.

 It may depend on:

 - replay keys;
- nonces;
- sequence numbers;
- expiration;
- consumption markers;
- state;
- temporality;
- other mechanisms.

 The domain requires:

 > **A valid `Authorization` in one context must not automatically become a valid `Authorization` in a later or different context when its original conditions no longer hold.**

 `Replay Protection` does not imply the existence of an `Authorization` Entity.

 Its operational state may belong to `AuthorizationState` or to a specific replay state.

---

 ## Cross-References

 - Foundations — `Identity`, `Account`, and sovereignty.
- Authority Model — `Credential` and `CredentialAuthority`.
- Derived Authority — `EffectiveAuthority` and derived mechanisms.
- Execution Model — how an accepted `Authorization` participates in `ExecutionContext`.
- Formal Laws — D4 — semantic value of `Authorization`, exclusion of `Proof`.
