workspace "Sovereign Account" "A universal primitive for sovereign identity, authority, and execution." {

    properties {
        "structurizr.introduction" "false"
    }

    !identifiers hierarchical

    model {
        # ── People ──
        user = person "User" "A sovereign identity holder, typically abstracted from blockchain details."
        developer = person "Developer" "Builds applications on top of Sovereign Account."
        agent = person "Agent" "An automated service or device operating on behalf of a Subject."

        # ── External Systems (Consumer Contexts) ──
        defi = softwareSystem "DeFi Protocol" "Finance use case."
        recovery = softwareSystem "Recovery Context" "Guardian-based recovery. Produces RecoveryPolicyRequest."
        economics = softwareSystem "Economics Context" "Subsidies and sponsorship."
        registry = softwareSystem "Registry Context" "Content metadata binding."

        # ── External Infrastructure ──
        irys = softwareSystem "Irys" "Permanent decentralized storage."
        blockchain = softwareSystem "Blockchain" "Arbitrum, EVM-compatible."

        # ── Sovereign Account ──
        account = softwareSystem "Sovereign Account" "Sovereign identity, authority derivation, and execution decision model." {

            # ── Orchestration layer ──
            runtime = container "Runtime" "Coordinates distributed authority evaluation and execution orchestration. Never owns authorization rules (D1.37)." "Rust / Stylus"
            engine = container "Execution Engine" "Materializes valid Execution Contexts. Does not decide authority (D1.47)." "Rust / Stylus"
            adapter = container "Adapter" "Materializes executions on concrete infrastructure (EIP-7702, ERC-4337, future AA)." "Rust / Stylus"

            # ── Authorization State ──
            state = container "Authorization State" "Operational authority state. Operative-historical record of recognized entities (D3.2)." "Rust / Stylus" {
                credentials = component "Credentials" "Entity continuity: Active, Suspended, Revoked."
                sessions = component "Sessions" "Temporal derived authority. Unusable when source credential is invalid (D1.13)."
                delegations = component "Delegations" "Derived authority. Bounded by source delegatable authority (D1.21)."
                restrictionMap = component "RestrictionMap" "Association (Capability, Scope) → Restriction. Functional and unique (D5.1)."
            }

            # ── Foundation ──
            foundation = container "Foundation" "Domain primitives. Source of truth for entity and value object definitions." "Dafny" {
                identity = component "Identity" "Sovereign continuity. Exactly one per Account (D1.1)."
                subject = component "Subject" "Semantic actor. Value Object."
                capability = component "Capability" "Faculty by value. No CapabilityId (D1.7)."
                capabilityKind = component "CapabilityKind" "Semantic class of a Capability. Value Object."
                delegate = component "Delegate" "Special CapabilityKind. Authorizes the act of delegating (D1.17)."
                scope = component "Scope" "Domain of exercise. Restricts, never expands (D1.42)."
                restriction = component "Restriction" "Limit on conditions. Non-expansive (D1.44)."
                chain = component "Chain" "Semantic execution environment. Part of Authorization Context (D6.2)."
                domainAction = component "DomainAction" "External intent. Belongs to the consumer context."
                executionTarget = component "ExecutionTarget" "Technical destination. Distinct from Capability Scope (D1.42)."
            }

            # ── Authority ──
            authority = container "Authority" "Credential, Session, Delegation, Effective Authority. Source of derived authority." "Dafny" {
                credential = component "Credential" "Recognized source of evidence. Entity, independent of Identity (D1.9)."
                credentialAuthority = component "CredentialAuthority" "Authority per Account. Value Object (D1.9)."
                session = component "Session" "Temporary derived authority. Bounded by Credential Authority (D1.11)."
                delegation = component "Delegation" "Derived authority to a delegatee. Bounded by Source (D1.21)."
                effectiveAuthority = component "EffectiveAuthority" "Contextual, derived, by value. Context is input, not stored (D1.29)."
            }

            # ── Authorization ──
            authorization = container "Authorization" "Request, validation, replay, acceptance transition." "Dafny" {
                authorizationRequest = component "Authorization" "Value Object with Context. Equality ignores Proof (D4.3)."
                requestedAuthority = component "RequestedAuthority" "Authority requested. Value Object, grants nothing (D1.32)."
                authorizationValidation = component "AuthorizationValidation" "Domain-level acceptance. Considers Credential, Proof, Effective Authority, Replay (D4.2)."
                replay = component "Replay" "Replay protection. Part of Authorization semantic value (D4.5)."
                acceptanceTransition = component "AuthorizationAcceptanceTransition" "Accepted state transition. Preserves domain invariants (D1.38)."
            }

            # ── Policy ──
            policy = container "Policy" "External decisions recognized by Account." "Dafny" {
                policyModel = component "Policy" "External recognized decision. No identity inside Account (D1.35)."
                policyEffect = component "PolicyEffect" "Semantic change descriptor. Value Object, closed algebra (D2.1)."
                policyConsumption = component "PolicyConsumption" "Atomic ordered application. All effects or none (D2.3)."
                stateTransition = component "AuthorizationStateTransition" "Valid state transformation. Preserves invariants (D1.38)."
            }

            # ── Execution ──
            execution = container "Execution" "Execution semantics. Defines what a valid execution means." "Dafny" {
                executionRequest = component "ExecutionRequest" "Request to materialize. Value Object."
                executionContext = component "ExecutionContext" "Complete validated decision. Carries Chain but not Environment (D6.5)."
                executionConstraints = component "ExecutionConstraints" "Materialization conditions. Do not create Authority (D1.46)."
                executionSemantics = component "ExecutionSemantics" "Dafny laws for execution and chain/environment compatibility (D6)."
            }

            # ── Laws ──
            laws = container "Laws" "Formal laws D1–D6 encoded as lemmas. Proven by Dafny." "Dafny" {
                authorityLaws = component "AuthorityLaws" "D1 — Sovereignty, delegation, provenance, monotonicity."
                authorizationLaws = component "AuthorizationLaws" "D1.32–D1.34, D4 — Proof vs authorization, semantic equality."
                capabilityLaws = component "CapabilityLaws" "D1.7–D1.8 — Capability as by-value faculty."
                delegationLaws = component "DelegationLaws" "D1.14–D1.50 — Root, transitive, restriction, provenance."
                executionLaws = component "ExecutionLaws" "D1.46–D1.48, D6 — Valid execution context, chain compatibility."
                identityLaws = component "IdentityLaws" "D1.1–D1.4 — Exact sovereignty, identity-bound."
                policyLaws = component "PolicyLaws" "D2 — PolicyEffect algebra, atomicity, contradiction."
                sessionLaws = component "SessionLaws" "D1.11–D1.13 — Session derivation, non-amplification."
            }

            # ── Proofs ──
            proofs = container "Proofs" "Machine-checked verification proofs." "Dafny" {
                isolated = component "Isolated Proofs" "Per-module proofs (foundation, authority, authorization, policy, execution)."
                scenarios = component "Scenario Proofs" "End-to-end consumer scenarios (Recovery, DeFi, EIP-7702, Cryptobro, Grandmother, Enterprise, Game, Agent, MultiChain, etc)."
            }

            # ── Cross-cutting infrastructure ──
            infrastructure = container "Infrastructure" "Transversal concepts that cross domain boundaries but do not own authority semantics." "Rust / Stylus" {
                proof = component "Proof" "Cryptographic evidence. Does not create Authority (D1.33)."
                verifier = component "Verifier" "Verifies Proof. Does not determine authority."
                authentication = component "Authentication" "Evidence production. Distinct from Authorization (D1.34)."
                replayProtection = component "Replay Protection" "Transversal property. Not an Entity."
                gasPayment = component "Gas Payment" "Resource provision. Independent of Authority (D1.49)."
                executionSponsor = component "Execution Sponsor" "Provides resources without acquiring authority (D1.49)."
            }
        }

        # ── External relationships ──
        user -> account "Exercises authority through"
        developer -> account "Builds on top of"
        agent -> account "Operates on behalf of a Subject"

        defi -> account "Requests authorization for"
        recovery -> account "Submits RecoveryPolicyRequest to"
        economics -> account "Sponsors execution via"
        registry -> account "Binds content metadata through"

        account -> irys "Publishes documentation to"
        account -> blockchain "Materializes executions on"

        # ── Internal flow (fully-qualified identifiers) ──
        account.runtime -> account.state "Reads Authorization State"
        account.runtime -> account.authorization "Coordinates validation"
        account.runtime -> account.authority "Coordinates Effective Authority"
        account.runtime -> account.policy "Coordinates Policy Effects"
        account.runtime -> account.execution "Builds Execution Context"
        account.runtime -> account.infrastructure "Coordinates Proof verification"

        account.engine -> account.execution "Consumes Execution Context"
        account.engine -> account.adapter "Delegates materialization"
        account.adapter -> blockchain "Materializes on"
    }

    views {
        systemLandscape "Landscape" {
            include *
            autoLayout lr
        }

        systemContext account "Context" {
            include *
            autoLayout lr
        }

        container account "Containers" {
            include *
            autoLayout tb
        }

        component account.foundation "Foundation" {
            include *
            autoLayout tb
        }

        component account.authority "Authority" {
            include *
            autoLayout tb
        }

        component account.authorization "Authorization" {
            include *
            autoLayout tb
        }

        component account.policy "Policy" {
            include *
            autoLayout tb
        }

        component account.execution "Execution" {
            include *
            autoLayout tb
        }

        component account.state "State" {
            include *
            autoLayout tb
        }

        component account.infrastructure "Infrastructure" {
            include *
            autoLayout tb
        }

        styles {
            element "Person" {
                shape Person
                background "#10b981"
                color "#020617"
            }
            element "Software System" {
                background "#1e293b"
                color "#f1f5f9"
            }
            element "Container" {
                background "#0f172a"
                color "#e2e8f0"
            }
            element "Component" {
                background "#1e293b"
                color "#e2e8f0"
            }
        }
    }
}
