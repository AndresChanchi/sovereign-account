---
id: account-law-D4
title: D4 — Proof and Authorization Equality
domain: sovereign-account
chapter: formal-laws
type: law-set
status: stable
law-id: D4
laws:
  - D4.1
  - D4.2
  - D4.3
  - D4.4
  - D4.5
  - D4.6
  - D4.7
depends-on:
  - account-ul-authorization
related:
  - account-law-D1
  - account-law-D3
formal-laws:
  - D1.32
  - D1.33
  - D1.34
verification:
  dafny:
    - domain/account/formal/authorization/Authorization.dfy
    - domain/account/formal/authorization/AuthorizationValidation.dfy
    - domain/account/formal/authorization/Replay.dfy
---

# D4 — Proof and Authorization Equality

## Domain Decisions

1. **`Proof` is a cryptographic infrastructure concept.**  
   It does not belong to the semantic domain of `Authorization`. Its purpose is to demonstrate that the `Authorization` was produced through the corresponding `Credential`, but it does not define which authorization is requested.

2. **`Proof` equality is irrelevant to the domain.**  
   Two `Proof` instances may be structurally different and still represent valid evidence for the same semantic authorization.  
   The domain does not define `Proof A == Proof B`; that equality is the responsibility of the `Verifier` or the transport layer.

3. **`Proof validity` ≠ `Proof equality` ≠ `Authorization validity`.**  
   - `Proof validity`: cryptographically correct evidence (verifier).  
   - `Proof equality`: structural/infrastructure comparison.  
   - `Authorization validity`: semantic domain + authorization conditions.

4. **`Authorization` is a Value Object whose equality is defined by its semantic content, not by the attached evidence.**  
   The semantic content includes:
   - `Credential` (reference).
   - `RequestedAuthority` (`Capability` instances, `Scope`, `Restriction` instances).
   - Temporal conditions.
   - **`ReplayProtection`** (nonce, sequence, expiration, unique context).
   - **Relevant Context** (defined in the [Authorization](../ubiquitous-language/03-authorization.md) chapter: `AccountId`, `ExecutionTarget`, `DomainAction`, `Scope`, and `Chain`).

   The `Proof` remains outside semantic equality.

5. **`ReplayProtection` is an essential part of the semantic value of `Authorization`.**  
   Two requests with exactly the same semantic content (same `Credential`, same `Capability` instances, same restrictions, same context) but with different `replayKey` represent different authorizations over time.  
   This prevents an attacker from replaying a valid `Authorization` even if they present a cryptographically different `Proof`.

6. **`Authorization` may structurally contain a `Proof`, but that containment does not affect its equality.**  
   It is a semantic container that, for transport/verification reasons, includes evidence, but the evidence does not define the conceptual value.

7. **No artificial predicates such as `SameAuthorization(a,b)` or `SameProof(p,q)` are introduced.**  
   The native equality of the Value Object reflects the semantic equality directly.  
   If the representation in a formal language allows the native equality to express that semantic equality correctly, it is used directly.

---

## Proof Laws

### D4.1 Proof as Infrastructure Concept

**Law D4.1.1**  
`Proof` is an infrastructure concept. Its definition, representation, and equality are external to the `Account` domain.

**Law D4.1.2**  
The domain does not define `Proof equality`. `Proof` equality is the responsibility of the `Verifier` or the transport layer.

### D4.2 Proof Validity vs Authorization Validity

**Law D4.2.1**  
`Proof validity` (cryptographic verification) is a necessary but not sufficient condition for `Authorization validity`.

**Law D4.2.2**  
A valid `Proof` does not imply that the `Authorization` is valid in the domain.  
Domain validity additionally requires that the `RequestedAuthority` is contained within `EffectiveAuthority` and that all other `AuthorizationValidation` conditions hold.

---

## Semantic Value Laws

### D4.3 Authorization Semantic Value

**Law D4.3.1**  
The semantic value of an `Authorization` is defined by the set of domain attributes that constitute its authorized intent:

```text
SemanticValue(Authorization)
    =
( Credential,
  RequestedAuthority,
  Restrictions,
  TemporalConditions,
  ReplayProtection,
  Context )
```

where the `Context` is composed of:

- `AccountId`: the `Account` over which authority is exercised.
- `ExecutionTarget`: the technical destination of the execution.
- `DomainAction`: the business action requested (defined by the consumer).
- `Scope`: the domain of the action.
- `Chain`: identifier of the blockchain environment (if applicable).

These elements form part of the semantic value and therefore affect the equality of `Authorization`.

**Law D4.3.2**  
The `Proof` **does not form part** of the semantic value of `Authorization`.

---

## Equality Laws

### D4.4 Authorization Equality

**Law D4.4.1**  
Two `Authorization` instances are equal if and only if their semantic values are identical:

```text
Authorization A == Authorization B
    ⇔
SemanticValue(A) == SemanticValue(B)
```

**Law D4.4.2**  
Two `Authorization` instances may have different `Proof` instances and still be semantically equal if all their semantic attributes match.

**Law D4.4.3**  
Two `Authorization` instances may have the same `Proof` and be semantically different if their semantic attributes differ.

### D4.6 Structural Containment vs Semantic Equality

**Law D4.6.1**  
An `Authorization` may structurally contain a `Proof` without that containment affecting its semantic equality.

**Law D4.6.2**  
`Authorization` equality in the domain **is not** implemented by comparing the `Proof`; it is implemented by comparing exclusively the semantic attributes defined in Law D4.3.1.

### D4.7 No Artificial Equality Predicates

**Law D4.7.1**  
For the Value Objects of the domain, semantic equality is the native equality of the formal language (e.g. `==` in Dafny) whenever the representation of the type allows that equality to be expressed correctly.

**Law D4.7.2**  
No predicates such as `SameAuthorization(A, B)` or `SameProof(P, Q)` are introduced that duplicate or contradict semantic equality.

---

## Replay Protection Laws

### D4.5 Replay Protection as Semantic Attribute

**Law D4.5.1**  
`ReplayProtection` is a semantic attribute of `Authorization`.  
It includes, at minimum, a mechanism that ensures an `Authorization` cannot be reused outside its original temporal or state context (e.g. nonce, sequence number, expiry, consumption marker).

**Law D4.5.2**  
Two `Authorization` instances with identical semantic attributes except for their `ReplayProtection` represent different authorizations and are not equal under Law D4.4.1.

**Law D4.5.3**  
The `Proof` does not substitute for `ReplayProtection`.  
Even if the `Proof` is different, if the `ReplayProtection` is the same and the rest of the semantic value matches, the `Authorization` is a **replay** and must be rejected by the Replay state.

---

## Conceptual Model of D4

```text
                  AUTHORIZATION
                         │
         ┌───────────────┴───────────────┐
         │                               │
   SEMANTIC CONTENT              CRYPTOGRAPHIC EVIDENCE
         │                               │
         ▼                               ▼
   Credential Reference                Proof
   Requested Authority                 (infrastructure)
   Restrictions
   Temporal Conditions
   Replay Protection
   Context (AccountId, ExecutionTarget,
           DomainAction, Scope, Chain)
         │
         ▼
   SEMANTIC EQUALITY
   (only considers Semantic Content)
```

The key separation:

```text
Authorization semantic value
    = Credential + Requested Authority + Restrictions +
      Temporal Conditions + Replay Protection + Context

Authorization structural content
    = semantic value + Proof (attached for verification)

Authorization equality
    = equality of semantic value (ignores Proof)
```

---

## Invariant Principles of D4

1. **Cryptographic evidence is not the message.**  
   The `Proof` demonstrates that the message was signed by the `Credential`, but does not define the content of the message.

2. **Semantic equality is independent of evidence.**  
   Two authorizations with the same meaning are equal, even if their proofs differ.

3. **`ReplayProtection` is the semantic anchor of temporal uniqueness.**  
   It prevents the system from confusing two identical requests over time.

4. **Infrastructure verifies the `Proof`; the domain validates the `Authorization`.**  
   Each layer has its responsibility. They do not mix.

5. **No artificial `SameX` predicates are created.**  
   Native equality in the formalization (Dafny) reflects the semantic equality of the domain without unnecessary duplication.

---

## Relationship with Other Domain Modules

- **`Authorization`** (Value Object): Defines its equality by semantic value, including the `Context` (`AccountId`, `ExecutionTarget`, `DomainAction`, `Scope`, `Chain`).
- **`Proof`** (infrastructure): External to the domain; it is verified but not compared semantically.
- **`ReplayProtection`** (semantic attribute): Part of `Authorization`; its state is managed in `Replay.dfy` or in the consumption state.
- **`AuthorizationValidation`**: Receives an `Authorization` and a `Proof`.  
  Validates the `Proof` (via `Verifier`) and then validates the semantic `Authorization` (including Replay, `EffectiveAuthority`, and Context compatibility with the state of the `Account`).
- **`Runtime`**: Orchestrates the combined validation.

---

## Cross-References

- [Authorization](../ubiquitous-language/03-authorization.md) — `Authorization`, `Proof`, `RequestedAuthority`, `ReplayProtection`.
- [Authority Model](../ubiquitous-language/02-authority-model.md) — `Credential` and `CredentialAuthority`.
- [Formal Laws — D1](D1-delegation-and-authority.md) — `Proof` does not create authority.
- [Formal Laws — D3](D3-entity-identity-and-recognition.md) — identity immutability and recognition.
