---
id: account-index
title: Account Domain
domain: sovereign-account
type: overview
status: stable
---
 # Account Domain

 The **Account** domain defines a universal primitive for **sovereign identity, authority, and execution**.

 Its purpose is to let an application determine:

 1. Which sovereign `Identity` is exercising authority.
2. What authority exists for that identity.
3. Which subject or mechanism may exercise that authority.
4. Under what conditions.
5. Which action the consuming context requests.
6. Whether the exercise is authorized.
7. How an authorized decision is materialized.

 The domain **does not define the meaning of business actions**. The same model applies to persons, organizations, agents, services, devices, and other subjects.

---

 ## Structure

 | Section | Description |
| --- | --- |
| Ubiquitous Language | The domain vocabulary: concepts, relationships, and model. |
| Formal Laws | The invariants `D1`–`D6` that the implementation must preserve. |
| Formal Verification | Dafny sources and verification status. |

---

 ## Reading Order

 Recommended path for a first read:

 1. Foundations — purpose, Subject, Identity, Account.
2. Authority Model — Authority, Capability, Credential.
3. Authorization — Authorization, Proof, Validation.
4. Derived Authority — Session, Delegation, Effective Authority.
5. State and Policy — Authorization State, Policy Effects.
6. Execution Model — Runtime, Execution Context, Adapter.
7. Reference — classification, principles, consolidated model.

 Formal laws can be read independently of the vocabulary, but assume it.

---

 ## Core Separations

```
External Identity ≠ Subject ≠ Identity ≠ Account ≠ Blockchain Address
```

```
Capability ≠ Credential Authority ≠ Effective Authority ≠ Authorization ≠ Execution
```

```
Chain (semantic) ≠ Execution Environment (infrastructure)
```

---

 ## Machine-readable

 - `llms.txt` — root LLM index (account points here).
