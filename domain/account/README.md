docs/domain/account/README.md

---

 ## id: account-readme\
 title: Account — Navigation\
 domain: sovereign-account\
 type: guide\
 status: stable

 # Account — Navigation

 Guide to the `domain/account/` tree.

---

 ## Layout

```
domain/account/
├── index.md
├── README.md
│
├── ubiquitous-language/
│   ├── index.md
│   ├── 01-foundations.md
│   ├── 02-authority-model.md
│   ├── 03-authorization.md
│   ├── 04-derived-authority.md
│   ├── 05-state-and-policy.md
│   ├── 06-execution-model.md
│   └── 07-reference.md
│
├── formal-laws/
│   ├── index.md
│   ├── D1-delegation-and-authority.md
│   ├── D2-policy-effect-and-state.md
│   ├── D3-entity-identity-and-recognition.md
│   ├── D4-proof-and-authorization-equality.md
│   ├── D5-restriction-association.md
│   └── D6-authorization-and-environment.md
│
└── formal/
    └── index.md
```

---

 ## Ubiquitous Language ↔ Formal Laws

 Every chapter of the vocabulary has corresponding formal laws:

 | Vocabulary | Formal Laws |
| --- | --- |
| Foundations (§1–§6, §44) | D1.1–D1.4, D3.1–D3.2 |
| Authority Model (§7–§14) | D1.5–D1.10 |
| Authorization (§15–§19, §42) | D4.\* |
| Derived Authority (§20–§24, §45) | D1.11–D1.50, D5.\* |
| State and Policy (§25–§29) | D2._, D5._ |
| Execution Model (§30–§43) | D6.\* |
| Reference (§46–§53) | — |

---

 ## Adding Content

 1. Prefer adding a section to an existing chapter over creating a new chapter.
2. Create a new chapter only when a new bounded subgroup of concepts appears.
3. Update the corresponding `index.md` and `llms.txt`.
4. Cross-link with `ref:` identifiers.
