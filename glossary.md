---
id: glossary
title: Glossary
type: reference
status: stable
---

# Glossary

Every concept of the Sovereign Account domain, with its stable identifier and a one-line definition.

For full definitions, follow the `ref:` links.

---

## Entities

| Concept | ID | Definition |
|---------|-----|-----------|
| Identity | `identity` | A sovereign continuity recognized by the protocol. |
| Account | `account` | The operational component through which an Identity exercises authority. |
| Credential | `credential` | A recognized source for producing authorization evidence. |
| Session | `session` | A temporary authorization derived from a Credential. |
| Delegation | `delegation` | An entity that confers derived authority to a delegatee. |

## Value Objects

| Concept | ID | Definition |
|---------|-----|-----------|
| Subject | `subject` | The semantic actor to which an Identity is attributed. |
| Capability | `capability` | A faculty that may be exercised. |
| Capability Kind | `capability-kind` | The semantic class of a Capability. |
| Scope | `scope` | The domain over which a Capability may be exercised. |
| Restriction | `restriction` | A limit on the conditions under which a Capability may be exercised. |
| Credential Authority | `credential-authority` | The set of Capabilities a Credential may attempt to exercise on an Account. |
| Requested Authority | `requested-authority` | The authority an Authorization attempts to exercise. |
| Effective Authority | `effective-authority` | The authority that may actually be exercised in a given context. |
| Authorization | `authorization` | A verifiable request to exercise authority. |
| Policy | `policy` | An external decision recognized by an Account. |
| Policy Effect | `policy-effect` | The semantic change a Policy produces on Authorization State. |
| Policy Consumption | `policy-consumption` | The recognition and application of a Policy. |
| Domain Action | `domain-action` | An intent expressed by the consuming context. |
| Execution Request | `execution-request` | A request to materialize a Domain Action. |
| Execution Target | `execution-target` | The technical destination of an execution. |
| Execution Context | `execution-context` | The complete validated decision required to materialize execution. |
| Execution Constraints | `execution-constraints` | Conditions that must be satisfied to materialize a context. |
| Timestamp | `timestamp` | A temporal marker. |

## Infrastructure Concepts

| Concept | ID | Definition |
|---------|-----|-----------|
| Proof | `proof` | Cryptographic evidence attached to an Authorization. |
| Verifier | `verifier` | A mechanism that verifies a Proof. |
| Runtime | `runtime` | The transient component that orchestrates distributed evaluation. |
| Execution Engine | `execution-engine` | The component that materializes a valid Execution Context. |
| Execution | `execution` | The operational materialization process (not an entity). |
| Adapter | `adapter` | The component that materializes execution on a concrete infrastructure. |
| Blockchain | `blockchain` | The shared environment where authority is exercised. |
| Gas Payment | `gas-payment` | The provision of economic resources to materialize an execution. |
| Execution Sponsor | `execution-sponsor` | A party providing resources to pay for an execution. |
| Authentication | `authentication` | The process of obtaining evidence about a Credential. |

## Formal Laws

| Law | ID | Scope |
|-----|-----|-------|
| Delegation and Authority | `D1` | Sovereignty, delegation, derived authority, provenance. |
| Policy Effect and State | `D2` | Policy Effect algebra, atomicity, contradiction. |
| Entity Identity and Recognition | `D3` | Identity immutability, recognition persistence, lifecycle specificity. |
| Proof and Authorization Equality | `D4` | Semantic value of Authorization, exclusion of Proof. |
| Restriction Association | `D5` | Restriction–Capability–Scope mapping. |
| Authorization and Environment | `D6` | Chain semantics, environment-independent acceptance. |

## See Also

- [Ubiquitous Language Index](domain/account/ubiquitous-language/index.md)
- [Formal Laws Index](domain/account/formal-laws/index.md)
