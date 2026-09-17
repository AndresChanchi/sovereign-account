---
id: docs-readme
title: Documentation Guide
type: guide
status: stable
---

# Documentation Guide

This directory contains the source of truth for the **Sovereign Account** project.

Everything here is authored in Markdown and compiled into a static, decentralized-ready artifact.

---

## Philosophy

The documentation is written **once** and compiled into multiple representations:

- **HTML** — for humans.
- **JSON** — for machines.
- **`llms.txt`** — for LLMs.
- **MCP endpoints** — for agents (future).

The build pipeline is described in `guides/build-pipeline.md`.

---

## Structure

```text
docs/
├── README.md                    # This file
├── index.md                     # Landing page
├── llms.txt                     # Root index for LLMs
├── glossary.md                  # Global glossary
│
├── domain/                      # Bounded contexts
│   ├── index.md
│   └── account/                 # Sovereign Account domain
│       ├── index.md
│       ├── README.md
│       ├── ubiquitous-language/ # Concepts, laws, model
│       ├── formal-laws/         # D1–D6 formal specifications
│       └── formal/              # Dafny source references
│
├── architecture/                # (future) C4 + Structurizr
├── decisions/                   # (future) ADRs
└── guides/                      # (future) How-to guides
```

---

## Frontmatter

Every document includes YAML frontmatter:

```yaml
---
id: account-foundations              # Stable identifier
title: Foundations                   # Human-readable title
domain: sovereign-account            # Bounded context
chapter: ubiquitous-language         # Top-level grouping
chapter-number: "01"                 # Ordering within chapter
type: concept                        # concept | law-set | overview | index | guide
status: stable                       # draft | review | stable | deprecated
section: "1-6, 44"                   # Original DDD section numbers
entities: [Identity, Account]        # Domain entities referenced
value-objects: [Subject]             # Value Objects referenced
depends-on: []                       # IDs of prerequisite documents
related: []                          # IDs of related documents
formal-laws: []                      # Dx.y references
verification:
  dafny: []                          # Paths to .dfy sources
---
```

Frontmatter is what makes the corpus **machine-readable**. It powers:

- Navigation generation
- `llms.txt` generation
- Cross-reference resolution
- The graph of dependencies

---

## Cross-references

Never link by file path. Use `ref:` syntax:

```markdown
See [Credential Authority](ref:account-authority-credential-authority).
```

The build pipeline resolves `ref:` to the correct URL for each representation (HTML, JSON, LLM).

This lets us move files without breaking links, and serve the same content across different roots.

---

## Writing Rules

1. **No references to "debt", "correction", or "previous version".** Documents describe the domain as it is.
2. **No infrastructure leaks.** No Ethereum, EVM, Stylus, Solidity, or blockchain-specific terms in the domain layer.
3. **No personal notes.** Documents are written for any reader (human or machine).
4. **Preserve precision.** Formal laws are never paraphrased.
5. **One concept per section.** Prefer adding a section over mixing topics.

---

## Adding a New Document

1. Choose the correct `chapter/` and `chapter-number`.
2. Assign a stable `id` (kebab-case, prefixed by domain).
3. Fill frontmatter (`depends-on`, `related`, `formal-laws`).
4. Write H1 + content.
5. Add a `ref:` from the chapter `index.md`.

---

## Building

```bash
docs build
```

Produces `dist/` with:

- Static HTML
- Structured JSON (`_data/`)
- `llms.txt`
- Copied assets

See `guides/build-pipeline.md` for the full pipeline.
