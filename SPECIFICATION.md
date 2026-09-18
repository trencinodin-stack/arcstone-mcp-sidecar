# Arcstone Execution Boundary: Experimental Strategic & Technical Specification

---

**Project Identity:** Arcstone Execution Boundary  
**Repository Identity:** `arcstone-mcp-sidecar`  
**MCP Registry Status:** Active (`io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0`)  
**Status:** Experimental / Downstream / Non-Canonical / Bounded Evidence  
**Version:** 0.1.0 Implementation Baseline  
**Upstream State:** Arcstone Continuity Core — Frozen; Arcstone Path A Ingress Lab Run 001 / Run 002 — Complete and Frozen  
**Interface & Transports:** Local CLI / stdio / Reference MCP Sidecar  

> **Implementation Note:** The v0.1.0 release is officially indexed on the Model Context Protocol (MCP) Registry as `io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0`. It acts as the reference execution boundary sidecar for the Arcstone Security Stack.

---

## 1. Executive Summary & Experimental Thesis

### 1.1 Operational Thesis

Probabilistic or otherwise nondeterministic systems may propose actions. A protected real-world side effect should occur only when an independent deterministic execution boundary possesses valid explicit authorization for that exact proposed operation.

The Arcstone Execution Boundary is a downstream experimental system intended to test whether deterministic authorization can mediate irreversible machine actions without granting the upstream producer authority over authorization, actuation, or the protected resource.

The system is designed for machine-first operation. Humans remain responsible for governance, scientific interpretation, experimental setup, and architectural change, but do not need to participate in each execution decision.

### 1.2 Primary Research Question

**Can a deterministic downstream execution boundary prevent an untrusted or nondeterministic producer from causing a protected side effect unless an explicit, boundary-controlled authorization condition has been satisfied?**

### 1.3 Primary Experimental Invariants

I1 — Non-Authorization Safety:
NO VALID AUTHORIZATION => ZERO PROTECTED ACTUATION

I2 — Single-Use Authority:
ONE VALID SINGLE-USE AUTHORIZATION => AT MOST ONE PROTECTED ACTUATION

I3 — Exclusive Actuation Authority:
UNTRUSTED PRODUCER DOES NOT IMPLY PROTECTED RESOURCE
EXECUTION BOUNDARY / ACTUATOR => SOLE VALID MUTATION PATH

These are target invariants to be tested. They are not treated as established guarantees before successful adversarial validation.

### 1.4 Scope Boundary

This repository does not redefine, extend, or reinterpret the Arcstone Continuity Core.
This repository does not promote the Path A Ingress Lab into an operational dependency.
This repository is a sibling downstream investigation that inherits constraints and evidence from prior work while introducing new execution-boundary hypotheses of its own.

The initial experiment does not claim:
- production-grade authorization security;
- complete AI or autonomous-agent governance;
- model alignment or AI safety;
- complete execution-membrane behavior;
- cryptographic capability security;
- WASM isolation guarantees;
- general filesystem, database, cloud, or payment authorization;
- exactly-once real-world execution;
- universal host or kernel security;
- validation of the broader Arcstone architecture;
- equivalence between controlled Path A elapsed fixtures and real-world latency.

---

## 2. Upstream Dependency and Authority Model

### 2.1 Frozen Upstream Components

1. Arcstone Continuity Core — existing deterministic primitive and Path A reference surface.
2. Arcstone Path A Ingress Lab — completed producer-independence and exact-replay evidence, including frozen Run 001 and Run 002.

No changes to either upstream repository are required or permitted by this experiment.

### 2.2 Repository Topology

Arcstone Continuity Core (FROZEN)
   |
   +---> Path A Ingress Lab (COMPLETE/FROZEN)
   |
   +---> Execution Boundary (MCP SIDECAR / ACTIVE)

The Execution Boundary does not depend on the Ingress Lab at runtime.

### 2.3 Authority Direction

UPSTREAM AUTHORITY -> DOWNSTREAM REALIZATION -> EXPERIMENTAL EVIDENCE

Experimental evidence may inform future research, but downstream observations do not alter upstream semantics.
No downstream component may claim canonical authority over upstream behavior.

### 2.4 Evaluation Is Not Authorization

A favorable deterministic-core result is not permission to perform a protected side effect.

DETERMINISTIC EVALUATION != AUTHORIZATION != SUCCESSFUL ACTUATION

---

## 3. Trust Boundary

### 3.1 Untrusted / Nondeterministic Producer Domain

The producer may propose an operation but must not possess direct authority to:
- create trusted authorization;
- modify trusted authorization;
- consume trusted authorization;
- invoke the protected actuator outside the gate;
- mutate the protected resource directly.

### 3.2 Trusted Experimental Issuer

The initial experiment includes a minimal trusted issuer fixture to create an explicit authorization record before execution.

### 3.3 Deterministic Execution Gate

The gate evaluates explicit request data against boundary-controlled authorization state and emits ALLOW or DENY(reason).

### 3.4 Protected Actuator

The protected actuator is the only component permitted to perform the protected side effect.

### 3.5 Protected Resource

The initial protected resource is exactly one OS-visible target (`EFFECT_LOG` -> `<root>/protected/effect.bin`).

---

## 4. Initial Experimental Architecture

TRUSTED EXPERIMENT SETUP -> ISSUER -> AUTHORIZATION STATE -> GATE -> CONSUME -> ACTUATOR -> PROTECTED RESOURCE

---

## 5. Orthogonal State Model

Authorization Lifecycle: ABSENT | INVALID | ISSUED | CONSUMED
Request Decision: ALLOW | DENY(reason)
Actuation Outcome: NOT_ATTEMPTED | SUCCEEDED | FAILED
Observed Effect: effect_generation = 0 | 1

---

## 6. Phase-Gated Development Program

Phase 0 — Specification Definition
Phase 1 — Downstream Admission Audit
Phase 2 — Minimal Native Experiment
Phase 3 — Adversarial Validation (T0-T17)
Phase 4 — Evidence Freeze (RUN-001)
Phase 5 — Registry Indexing & MCP Baseline (Current Active Stage: io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0)
Phase 6 — Isolation Hardening

---

## 7. Deferred Candidate Mechanisms

Explicitly deferred until justified by evidence:
- HMAC, signatures, PKI, KMS, capability certificates;
- WASM runtimes;
- containers / VMs as primary isolation;
- network APIs, brokers, queues, or remote RPC;
- dynamic policy DSL engines;
- standing capabilities;
- delegation or multi-party authorization;
- exactly-once distributed actuation.

---

## 8. Working Principle

> **Evidence before expansion. Preserve the core. Test the boundary. Freeze completed evidence.**
