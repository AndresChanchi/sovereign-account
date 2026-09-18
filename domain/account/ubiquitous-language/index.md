---
id: account-ubiquitous-language-index
title: Ubiquitous Language
domain: sovereign-account
type: index
status: stable
---

# Ubiquitous Language

The vocabulary of the Account domain.

---

## Chapters

| # | Chapter | Original sections | Description |
| --- | --- | --- | --- |
| 01 | Foundations | §1–§6, §44 | Purpose, external context, Subject, Identity, Account, sovereignty. |
| 02 | Authority Model | §7–§14 | Authority, Capability, Scope, Credential, Credential Authority. |
| 03 | Authorization | §15–§19, §42 | Proof, Verifier, Requested Authority, Authorization, Validation, Replay Protection. |
| 04 | Derived Authority | §20–§24, §45 | Session, Delegation, Delegatee, Restriction, Effective Authority. |
| 05 | State and Policy | §25–§29 | Authorization State, Transitions, Policy, Policy Effect, Policy Consumption. |
| 06 | Execution Model | §30–§43 | Domain Action, Execution Request/Target/Context/Constraints, Runtime, Engine, Adapter. |
| 07 | Reference | §46–§53 | Classification, identifiers, stress test, principles, consolidated model. |

---

## Reading Order

The chapters are ordered by **conceptual dependency**:

```text
01 Foundations
      ↓
02 Authority Model
      ↓
03 Authorization
      ↓
04 Derived Authority
      ↓
05 State and Policy
      ↓
06 Execution Model
      ↓
07 Reference
```

Every chapter may be read independently, but later chapters assume the concepts of earlier ones.

---

## Cross-Reference

- [Formal Laws](../formal-laws/index.md) — the invariants these chapters describe.
- [Glossary](../../../glossary.md) — flat index of all concepts.

---

## Architecture Diagrams

Interactive C4 diagrams, rendered from `architecture/workspace.dsl`.

### System Context

<!-- architecture:start Context -->

```mermaid
%% kipio-diagram: Context
graph LR
  linkStyle default fill:#ffffff

  subgraph diagram ["System Context View: Sovereign Account"]
    style diagram fill:#ffffff,stroke:#ffffff

    1["<div style='font-weight: bold'>User</div><div style='font-size: 70%; margin-top: 0px'>[Person]</div><div style='font-size: 80%; margin-top:10px'>A sovereign identity holder,<br />typically abstracted from<br />blockchain details.</div>"]
    style 1 fill:#10b981,stroke:#0b815a,color:#020617
    10["<div style='font-weight: bold'>Sovereign Account</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Sovereign identity, authority<br />derivation, and execution<br />decision model.</div>"]
    style 10 fill:#1e293b,stroke:#151c29,color:#f1f5f9
    2["<div style='font-weight: bold'>Developer</div><div style='font-size: 70%; margin-top: 0px'>[Person]</div><div style='font-size: 80%; margin-top:10px'>Builds applications on top of<br />Sovereign Account.</div>"]
    style 2 fill:#10b981,stroke:#0b815a,color:#020617
    3["<div style='font-weight: bold'>Agent</div><div style='font-size: 70%; margin-top: 0px'>[Person]</div><div style='font-size: 80%; margin-top:10px'>An automated service or<br />device operating on behalf of<br />a Subject.</div>"]
    style 3 fill:#10b981,stroke:#0b815a,color:#020617
    4["<div style='font-weight: bold'>DeFi Protocol</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Finance use case.</div>"]
    style 4 fill:#1e293b,stroke:#151c29,color:#f1f5f9
    5["<div style='font-weight: bold'>Recovery Context</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Guardian-based recovery.<br />Produces<br />RecoveryPolicyRequest.</div>"]
    style 5 fill:#1e293b,stroke:#151c29,color:#f1f5f9
    6["<div style='font-weight: bold'>Economics Context</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Subsidies and sponsorship.</div>"]
    style 6 fill:#1e293b,stroke:#151c29,color:#f1f5f9
    7["<div style='font-weight: bold'>Registry Context</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Content metadata binding.</div>"]
    style 7 fill:#1e293b,stroke:#151c29,color:#f1f5f9
    8["<div style='font-weight: bold'>Irys</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Permanent decentralized<br />storage.</div>"]
    style 8 fill:#1e293b,stroke:#151c29,color:#f1f5f9
    9["<div style='font-weight: bold'>Blockchain</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Arbitrum, EVM-compatible.</div>"]
    style 9 fill:#1e293b,stroke:#151c29,color:#f1f5f9

    1-. "<div>Exercises authority through</div><div style='font-size: 70%'></div>" .->10
    2-. "<div>Builds on top of</div><div style='font-size: 70%'></div>" .->10
    3-. "<div>Operates on behalf of a<br />Subject</div><div style='font-size: 70%'></div>" .->10
    4-. "<div>Requests authorization for</div><div style='font-size: 70%'></div>" .->10
    5-. "<div>Submits RecoveryPolicyRequest<br />to</div><div style='font-size: 70%'></div>" .->10
    6-. "<div>Sponsors execution via</div><div style='font-size: 70%'></div>" .->10
    7-. "<div>Binds content metadata<br />through</div><div style='font-size: 70%'></div>" .->10
    10-. "<div>Publishes documentation to</div><div style='font-size: 70%'></div>" .->8
    10-. "<div>Materializes executions on</div><div style='font-size: 70%'></div>" .->9

  end
```

<!-- architecture:end -->

### Containers

<!-- architecture:start Containers -->

```mermaid
%% kipio-diagram: Containers
graph TB
  linkStyle default fill:#ffffff

  subgraph diagram ["Container View: Sovereign Account"]
    style diagram fill:#ffffff,stroke:#ffffff

    9["<div style='font-weight: bold'>Blockchain</div><div style='font-size: 70%; margin-top: 0px'>[Software System]</div><div style='font-size: 80%; margin-top:10px'>Arbitrum, EVM-compatible.</div>"]
    style 9 fill:#1e293b,stroke:#151c29,color:#f1f5f9

    subgraph 10 ["Sovereign Account"]
      style 10 fill:#ffffff,stroke:#151c29,color:#151c29

      11["<div style='font-weight: bold'>Runtime</div><div style='font-size: 70%; margin-top: 0px'>[Container: Rust / Stylus]</div><div style='font-size: 80%; margin-top:10px'>Coordinates distributed<br />authority evaluation and<br />execution orchestration.<br />Never owns authorization<br />rules (D1.37).</div>"]
      style 11 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      12["<div style='font-weight: bold'>Execution Engine</div><div style='font-size: 70%; margin-top: 0px'>[Container: Rust / Stylus]</div><div style='font-size: 80%; margin-top:10px'>Materializes valid Execution<br />Contexts. Does not decide<br />authority (D1.47).</div>"]
      style 12 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      13["<div style='font-weight: bold'>Adapter</div><div style='font-size: 70%; margin-top: 0px'>[Container: Rust / Stylus]</div><div style='font-size: 80%; margin-top:10px'>Materializes executions on<br />concrete infrastructure<br />(EIP-7702, ERC-4337, future<br />AA).</div>"]
      style 13 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      14["<div style='font-weight: bold'>Authorization State</div><div style='font-size: 70%; margin-top: 0px'>[Container: Rust / Stylus]</div><div style='font-size: 80%; margin-top:10px'>Operational authority state.<br />Operative-historical record<br />of recognized entities<br />(D3.2).</div>"]
      style 14 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      19["<div style='font-weight: bold'>Foundation</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>Domain primitives. Source of<br />truth for entity and value<br />object definitions.</div>"]
      style 19 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      30["<div style='font-weight: bold'>Authority</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>Credential, Session,<br />Delegation, Effective<br />Authority. Source of derived<br />authority.</div>"]
      style 30 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      36["<div style='font-weight: bold'>Authorization</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>Request, validation, replay,<br />acceptance transition.</div>"]
      style 36 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      42["<div style='font-weight: bold'>Policy</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>External decisions recognized<br />by Account.</div>"]
      style 42 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      47["<div style='font-weight: bold'>Execution</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>Execution semantics. Defines<br />what a valid execution means.</div>"]
      style 47 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      52["<div style='font-weight: bold'>Laws</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>Formal laws D1–D6 encoded as<br />lemmas. Proven by Dafny.</div>"]
      style 52 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      61["<div style='font-weight: bold'>Proofs</div><div style='font-size: 70%; margin-top: 0px'>[Container: Dafny]</div><div style='font-size: 80%; margin-top:10px'>Machine-checked verification<br />proofs.</div>"]
      style 61 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
      64["<div style='font-weight: bold'>Infrastructure</div><div style='font-size: 70%; margin-top: 0px'>[Container: Rust / Stylus]</div><div style='font-size: 80%; margin-top:10px'>Transversal concepts that<br />cross domain boundaries but<br />do not own authority<br />semantics.</div>"]
      style 64 fill:#0f172a,stroke:#0a101d,color:#e2e8f0
    end

    11-. "<div>Reads Authorization State</div><div style='font-size: 70%'></div>" .->14
    11-. "<div>Coordinates validation</div><div style='font-size: 70%'></div>" .->36
    11-. "<div>Coordinates Effective<br />Authority</div><div style='font-size: 70%'></div>" .->30
    11-. "<div>Coordinates Policy Effects</div><div style='font-size: 70%'></div>" .->42
    11-. "<div>Builds Execution Context</div><div style='font-size: 70%'></div>" .->47
    11-. "<div>Coordinates Proof<br />verification</div><div style='font-size: 70%'></div>" .->64
    12-. "<div>Consumes Execution Context</div><div style='font-size: 70%'></div>" .->47
    12-. "<div>Delegates materialization</div><div style='font-size: 70%'></div>" .->13
    13-. "<div>Materializes on</div><div style='font-size: 70%'></div>" .->9

  end
```

<!-- architecture:end -->

