---
id: account-formal-laws-index
title: Formal Laws
domain: sovereign-account
type: index
status: stable
---

# Formal Laws

The formal invariants of the Account domain, grouped as `D1`–`D6`.

Each file is a self-contained set of laws. Identifiers (`D1.1`, `D1.2`, …) are stable and referenced from the vocabulary and from the Dafny sources.

---

## Laws

| ID | Title | Scope |
|----|-------|-------|
| [D1](D1-delegation-and-authority.md) | Delegation and Authority | Sovereignty, delegation, derived authority, provenance, monotonicity. |
| [D2](D2-policy-effect-and-state.md) | Policy Effect and State | Policy effect algebra, ordered sequences, atomic consumption, contradiction. |
| [D3](D3-entity-identity-and-recognition.md) | Entity Identity and Recognition | Identity immutability, recognition persistence, per-entity lifecycles. |
| [D4](D4-proof-and-authorization-equality.md) | Proof and Authorization Equality | Semantic value of Authorization, exclusion of Proof, Replay Protection. |
| [D5](D5-restriction-association.md) | Restriction Association | Restriction–Capability–Scope mapping, `ModifyRestriction` semantics. |
| [D6](D6-authorization-and-environment.md) | Authorization and Environment | Chain as semantic context, environment-independent acceptance, compatibility at materialization. |

---

## Independence

Each law set is independent of:

- Dafny
- Rust
- EVM
- blockchain addresses
- cryptographic primitives
- storage representations
- implementation-specific data structures

The implementation **must demonstrate** these laws, not redefine them.

---

## Rule

When Dafny finds an ambiguity:

```text
Dafny ambiguity
      ↓
DDD clarification
      ↓
formal law
      ↓
proof
```

Never resolve an ambiguity by introducing a convenient structure in Dafny.

---

## Cross-Reference

Laws reference the vocabulary via `ref:` and cite the original section numbers:

- §1–§6, §44 — Foundations
- §7–§14 — Authority Model
- §15–§19, §42 — Authorization
- §20–§24, §45 — Derived Authority
- §25–§29 — State and Policy
- §30–§43 — Execution Model
- §46–§53 — Reference

---

## Module Diagrams

Per-module component diagrams, rendered from `architecture/workspace.dsl`.

### Foundation

<!-- architecture:start Foundation -->

```mermaid
%% kipio-diagram: Foundation
graph TB
  linkStyle default fill:#ffffff

  subgraph diagram ["Component View: Sovereign Account - Foundation"]
    style diagram fill:#ffffff,stroke:#ffffff

    subgraph 10 ["Sovereign Account"]
      style 10 fill:#ffffff,stroke:#151c29,color:#151c29

      subgraph 19 ["Foundation"]
        style 19 fill:#ffffff,stroke:#0a101d,color:#0a101d

        20["<div style='font-weight: bold'>Identity</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Sovereign continuity. Exactly<br />one per Account (D1.1).</div>"]
        style 20 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        21["<div style='font-weight: bold'>Subject</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Semantic actor. Value Object.</div>"]
        style 21 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        22["<div style='font-weight: bold'>Capability</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Faculty by value. No<br />CapabilityId (D1.7).</div>"]
        style 22 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        23["<div style='font-weight: bold'>CapabilityKind</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Semantic class of a<br />Capability. Value Object.</div>"]
        style 23 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        24["<div style='font-weight: bold'>Delegate</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Special CapabilityKind.<br />Authorizes the act of<br />delegating (D1.17).</div>"]
        style 24 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        25["<div style='font-weight: bold'>Scope</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Domain of exercise.<br />Restricts, never expands<br />(D1.42).</div>"]
        style 25 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        26["<div style='font-weight: bold'>Restriction</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Limit on conditions.<br />Non-expansive (D1.44).</div>"]
        style 26 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        27["<div style='font-weight: bold'>Chain</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Semantic execution<br />environment. Part of<br />Authorization Context (D6.2).</div>"]
        style 27 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        28["<div style='font-weight: bold'>DomainAction</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>External intent. Belongs to<br />the consumer context.</div>"]
        style 28 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        29["<div style='font-weight: bold'>ExecutionTarget</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Technical destination.<br />Distinct from Capability<br />Scope (D1.42).</div>"]
        style 29 fill:#1e293b,stroke:#151c29,color:#e2e8f0
      end

    end

  end
```

<!-- architecture:end -->

### Authority

<!-- architecture:start Authority -->

```mermaid
%% kipio-diagram: Authority
graph TB
  linkStyle default fill:#ffffff

  subgraph diagram ["Component View: Sovereign Account - Authority"]
    style diagram fill:#ffffff,stroke:#ffffff

    subgraph 10 ["Sovereign Account"]
      style 10 fill:#ffffff,stroke:#151c29,color:#151c29

      subgraph 30 ["Authority"]
        style 30 fill:#ffffff,stroke:#0a101d,color:#0a101d

        31["<div style='font-weight: bold'>Credential</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Recognized source of<br />evidence. Entity, independent<br />of Identity (D1.9).</div>"]
        style 31 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        32["<div style='font-weight: bold'>CredentialAuthority</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Authority per Account. Value<br />Object (D1.9).</div>"]
        style 32 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        33["<div style='font-weight: bold'>Session</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Temporary derived authority.<br />Bounded by Credential<br />Authority (D1.11).</div>"]
        style 33 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        34["<div style='font-weight: bold'>Delegation</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Derived authority to a<br />delegatee. Bounded by Source<br />(D1.21).</div>"]
        style 34 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        35["<div style='font-weight: bold'>EffectiveAuthority</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Contextual, derived, by<br />value. Context is input, not<br />stored (D1.29).</div>"]
        style 35 fill:#1e293b,stroke:#151c29,color:#e2e8f0
      end

    end

  end
```

<!-- architecture:end -->

### Authorization

<!-- architecture:start Authorization -->

```mermaid
%% kipio-diagram: Authorization
graph TB
  linkStyle default fill:#ffffff

  subgraph diagram ["Component View: Sovereign Account - Authorization"]
    style diagram fill:#ffffff,stroke:#ffffff

    subgraph 10 ["Sovereign Account"]
      style 10 fill:#ffffff,stroke:#151c29,color:#151c29

      subgraph 36 ["Authorization"]
        style 36 fill:#ffffff,stroke:#0a101d,color:#0a101d

        37["<div style='font-weight: bold'>Authorization</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Value Object with Context.<br />Equality ignores Proof<br />(D4.3).</div>"]
        style 37 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        38["<div style='font-weight: bold'>RequestedAuthority</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Authority requested. Value<br />Object, grants nothing<br />(D1.32).</div>"]
        style 38 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        39["<div style='font-weight: bold'>AuthorizationValidation</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Domain-level acceptance.<br />Considers Credential, Proof,<br />Effective Authority, Replay<br />(D4.2).</div>"]
        style 39 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        40["<div style='font-weight: bold'>Replay</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Replay protection. Part of<br />Authorization semantic value<br />(D4.5).</div>"]
        style 40 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        41["<div style='font-weight: bold'>AuthorizationAcceptanceTransition</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Accepted state transition.<br />Preserves domain invariants<br />(D1.38).</div>"]
        style 41 fill:#1e293b,stroke:#151c29,color:#e2e8f0
      end

    end

  end
```

<!-- architecture:end -->

### Policy

<!-- architecture:start Policy -->

```mermaid
%% kipio-diagram: Policy
graph TB
  linkStyle default fill:#ffffff

  subgraph diagram ["Component View: Sovereign Account - Policy"]
    style diagram fill:#ffffff,stroke:#ffffff

    subgraph 10 ["Sovereign Account"]
      style 10 fill:#ffffff,stroke:#151c29,color:#151c29

      subgraph 42 ["Policy"]
        style 42 fill:#ffffff,stroke:#0a101d,color:#0a101d

        43["<div style='font-weight: bold'>Policy</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>External recognized decision.<br />No identity inside Account<br />(D1.35).</div>"]
        style 43 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        44["<div style='font-weight: bold'>PolicyEffect</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Semantic change descriptor.<br />Value Object, closed algebra<br />(D2.1).</div>"]
        style 44 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        45["<div style='font-weight: bold'>PolicyConsumption</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Atomic ordered application.<br />All effects or none (D2.3).</div>"]
        style 45 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        46["<div style='font-weight: bold'>AuthorizationStateTransition</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Valid state transformation.<br />Preserves invariants (D1.38).</div>"]
        style 46 fill:#1e293b,stroke:#151c29,color:#e2e8f0
      end

    end

  end
```

<!-- architecture:end -->

### Execution

<!-- architecture:start Execution -->

```mermaid
%% kipio-diagram: Execution
graph TB
  linkStyle default fill:#ffffff

  subgraph diagram ["Component View: Sovereign Account - Execution"]
    style diagram fill:#ffffff,stroke:#ffffff

    subgraph 10 ["Sovereign Account"]
      style 10 fill:#ffffff,stroke:#151c29,color:#151c29

      subgraph 47 ["Execution"]
        style 47 fill:#ffffff,stroke:#0a101d,color:#0a101d

        48["<div style='font-weight: bold'>ExecutionRequest</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Request to materialize. Value<br />Object.</div>"]
        style 48 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        49["<div style='font-weight: bold'>ExecutionContext</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Complete validated decision.<br />Carries Chain but not<br />Environment (D6.5).</div>"]
        style 49 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        50["<div style='font-weight: bold'>ExecutionConstraints</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Materialization conditions.<br />Do not create Authority<br />(D1.46).</div>"]
        style 50 fill:#1e293b,stroke:#151c29,color:#e2e8f0
        51["<div style='font-weight: bold'>ExecutionSemantics</div><div style='font-size: 70%; margin-top: 0px'>[Component]</div><div style='font-size: 80%; margin-top:10px'>Dafny laws for execution and<br />chain/environment<br />compatibility (D6).</div>"]
        style 51 fill:#1e293b,stroke:#151c29,color:#e2e8f0
      end

    end

  end
```

<!-- architecture:end -->
