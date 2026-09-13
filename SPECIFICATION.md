# Arcstone Execution Boundary: Experimental Strategic & Technical Specification

---

**Project Identity:** Arcstone Execution Boundary
**Repository Identity:** `arcstone-mcp-sidecar`
**Status:** Experimental / Downstream / Non-Canonical / Pre-Implementation
**Version:** 0.1 Experimental Definition — Blind Architecture Review Integrated
**Upstream State:** Arcstone Continuity Core — Frozen; Arcstone Path A Ingress Lab Run 001 / Run 002 — Complete and Frozen
**Initial Transport:** Local CLI / stdio
**Future Candidate Distribution:** MCP / JSON-RPC wrapper, standalone GitHub repository, optional package distribution after experimental validation

---

## 1. Executive Summary & Experimental Thesis

### 1.1 Operational Thesis

Probabilistic or otherwise nondeterministic systems may propose actions. A protected real-world side effect should occur only when an independent deterministic execution boundary possesses valid explicit authorization for that exact proposed operation.

The Arcstone Execution Boundary is a downstream experimental system intended to test whether deterministic authorization can mediate irreversible machine actions without granting the upstream producer authority over authorization, actuation, or the protected resource.

The system is designed for machine-first operation. Humans remain responsible for governance, scientific interpretation, experimental setup, and architectural change, but do not need to participate in each execution decision.

### 1.2 Primary Research Question

**Can a deterministic downstream execution boundary prevent an untrusted or nondeterministic producer from causing a protected side effect unless an explicit, boundary-controlled authorization condition has been satisfied?**

### 1.3 Primary Experimental Invariants

```text
I1 — Non-Authorization Safety

NO VALID AUTHORIZATION
        ⇒
ZERO PROTECTED ACTUATION
```

```text
I2 — Single-Use Authority

ONE VALID SINGLE-USE AUTHORIZATION
        ⇒
AT MOST ONE PROTECTED ACTUATION
```

```text
I3 — Exclusive Actuation Authority

UNTRUSTED PRODUCER
        ⇏
PROTECTED RESOURCE

EXECUTION BOUNDARY / ACTUATOR
        ⇒
SOLE VALID MUTATION PATH
```

These are target invariants to be tested. They are not treated as established guarantees before successful adversarial validation.

### 1.4 Scope Boundary

This repository does **not** redefine, extend, or reinterpret the Arcstone Continuity Core.

This repository does **not** promote the Path A Ingress Lab into an operational dependency.

This repository is a sibling downstream investigation that inherits constraints and evidence from prior work while introducing new execution-boundary hypotheses of its own.

The initial experiment does **not** claim:

- production-grade authorization security;
- complete AI or autonomous-agent governance;
- model alignment or AI safety;
- complete execution-membrane behavior;
- cryptographic capability security;
- MCP protocol correctness;
- WASM isolation guarantees;
- general filesystem, database, cloud, or payment authorization;
- exactly-once real-world execution;
- universal host or kernel security;
- validation of the broader Arcstone architecture;
- equivalence between controlled Path A elapsed fixtures and real-world latency.

---

## 2. Upstream Dependency and Authority Model

### 2.1 Frozen Upstream Components

The following upstream surfaces are frozen for this experiment:

1. **Arcstone Continuity Core** — existing deterministic primitive and Path A reference surface.
2. **Arcstone Path A Ingress Lab** — completed producer-independence and exact-replay evidence, including frozen Run 001 and Run 002.

No changes to either upstream repository are required or permitted by this experiment.

### 2.2 Repository Topology

```text
                 Arcstone Continuity Core
                        FROZEN
                       /      \
                      /        \
                     ▼          ▼
       Path A Ingress Lab    Execution Boundary
          COMPLETE/FROZEN      NEXT EXPERIMENT
```

The Execution Boundary does not depend on the Ingress Lab at runtime.

The two downstream repositories ask different questions:

```text
Ingress Lab:
Can arbitrary external producers reach an unchanged deterministic
predicate reproducibly without producer provenance becoming an input?

Execution Boundary:
Can deterministic authorization prevent unauthorized protected action?
```

### 2.3 Authority Direction

```text
UPSTREAM AUTHORITY
        ↓
DOWNSTREAM REALIZATION
        ↓
EXPERIMENTAL EVIDENCE
```

Experimental evidence may inform future research, but downstream observations do not alter upstream semantics.

No downstream component may claim canonical authority over upstream behavior.

### 2.4 Evaluation Is Not Authorization

A favorable deterministic-core result is not permission to perform a protected side effect.

```text
DETERMINISTIC EVALUATION
        �
AUTHORIZATION
        �
SUCCESSFUL ACTUATION
```

The Execution Boundary owns its downstream authorization semantics. It must not retrofit them into the frozen core.

---

## 3. Trust Boundary

### 3.1 Untrusted / Nondeterministic Producer Domain

Examples include:

- AI models;
- autonomous agents;
- scripts;
- workflow engines;
- human-operated tools;
- robot controllers;
- cloud processes;
- other machine systems.

The producer may propose an operation but must not possess direct authority to:

- create trusted authorization;
- modify trusted authorization;
- consume trusted authorization;
- invoke the protected actuator outside the gate;
- mutate the protected resource directly.

Producer identity and provenance are evidence metadata unless a future experiment explicitly admits them as policy inputs.

### 3.2 Trusted Experimental Issuer

The initial experiment includes a minimal trusted issuer fixture.

Its only role is to create an explicit authorization record before execution.

The issuer is not:

- an identity provider;
- a certificate authority;
- a network service;
- a policy engine;
- an AI trust service;
- a production authorization platform.

Issuance is an explicit experimental act. Merely requesting an operation must never create authority.

### 3.3 Deterministic Execution Gate

The gate evaluates explicit request data against boundary-controlled authorization state and emits:

```text
ALLOW
```

or:

```text
DENY(reason)
```

The gate must not depend implicitly on model identity, provenance, wall-clock timing, host load, or a favorable upstream result.

### 3.4 Protected Actuator

The protected actuator is the only component permitted to perform the protected side effect.

The producer must not be able to bypass the gate and reach the protected mutation path directly.

### 3.5 Protected Resource

The initial protected resource is exactly one OS-visible file or file-like target under authority-controlled access.

The initial operation is:

```text
WRITE_PROTECTED_FILE
```

The experiment must define resource identity unambiguously. Path aliases, relative traversal, symlinks, or alternate names must not silently create a second route to the same or another protected resource.

For v0.1, resource identity should be frozen as one predeclared protected target under the experiment's control.

---

## 4. Initial Experimental Architecture

```text
                  TRUSTED EXPERIMENT SETUP
                           │
                         ISSUER
                           │
                           ▼
                  AUTHORIZATION STATE
                           │
                           ▼
UNTRUSTED              DETERMINISTIC
PRODUCER ────────────────► GATE
                              │
                       ATOMIC CLAIM /
                         CONSUME
                              │
                              ▼
                          ACTUATOR
                              │
                              ▼
                     PROTECTED RESOURCE
```

The initial experiment is local-first and does not require MCP, network transport, package distribution, containers, or WASM.

---

## 5. Orthogonal State Model

The first experiment must not collapse authorization lifecycle, decision, actuation, and observed effect into one grand state machine.

### 5.1 Authorization Lifecycle

```text
ABSENT
INVALID
ISSUED
CONSUMED
```

### 5.2 Request Decision

```text
ALLOW
DENY(reason)
```

### 5.3 Actuation Outcome

```text
NOT_ATTEMPTED
SUCCEEDED
FAILED
```

### 5.4 Observed Effect

For the initial experiment:

```text
effect_generation = 0 | 1
```

or an equivalent exact pre/post byte observation.

### 5.5 Required Separation

Examples:

```text
DENY
authorization = ISSUED
actuation = NOT_ATTEMPTED
effect = unchanged
```

```text
ALLOW
authorization = CONSUMED
actuation = SUCCEEDED
effect_generation = 1
```

```text
ALLOW
authorization = CONSUMED
actuation = FAILED
effect = unchanged
```

The last case is valid and important:

```text
AUTHORIZED
        �
ACTUATION SUCCEEDED
```

---

## 6. Authorization Representation

### 6.1 Minimum v0.1 Record

The first experiment requires only enough trusted state to define one exact authorized actuation.

A minimal record is:

```text
authorization_id
action
resource_id
payload_binding
state
```

Where:

```text
state = ISSUED | CONSUMED
```

Authorization must bind at minimum to:

- exact action;
- exact protected resource identity;
- exact payload bytes or a deterministic digest of those bytes;
- unique authorization identity.

Producer identity must not supply authority.

### 6.2 Payload Binding

For very small fixed experimental payloads, exact byte equality is sufficient.

A digest such as SHA-256 may be used for evidence integrity or for convenient payload binding when useful, but cryptographic hashing is not itself treated as authorization.

### 6.3 Single-Use Requirement

The first experiment intentionally defines authorization as permission for exactly one occurrence of one explicitly bound actuation.

Therefore authorization must be single-use.

A reusable record would test a standing capability rather than one bounded actuation.

### 6.4 Consumption Semantics

v0.1 uses **at-most-once authorization semantics**.

The authorization must be atomically claimed/marked `CONSUMED` immediately before the actuator operation is issued.

```text
ISSUED
   ↓
ATOMIC CLAIM / CONSUME
   ↓
CONSUMED
   ↓
ACTUATION ATTEMPT
   ├── SUCCEEDED
   └── FAILED
```

If actuation fails after consumption:

```text
authorization = CONSUMED
actuation = FAILED
effect = unchanged
```

No automatic retry is implied.

Exactly-once real-world execution is explicitly deferred.

### 6.5 Concurrency Requirement

Two concurrent attempts using one `ISSUED` authorization must not both obtain execution authority.

The authorization claim must therefore be atomic within the local experimental environment.

A simple single-host filesystem primitive such as exclusive create, rename, or equivalent atomic state transition is sufficient if it can be demonstrated and replayed.

No distributed lock service is required.

---

## 7. Deterministic Evaluation and Downstream Authorization

### 7.1 Preserve Upstream Semantics

The Execution Boundary must not introduce a new Arcstone lattice by renaming or conflating upstream states.

If the frozen deterministic core is invoked, its exact result must be preserved separately as evidence.

The core result is not authorization.

### 7.2 Core Runtime Status

The frozen deterministic core is **optional at runtime** for the minimum authorization experiment.

If used, it may only contribute an inspectable deterministic evaluation record.

The authorization experiment must remain capable of proving:

```text
favorable core result + no authorization
        ⇒
DENY
```

and:

```text
valid authorization + core omitted
        ⇒
ALLOW
```

for the defined v0.1 policy.

These tests demonstrate experimentally that core evaluation is not hidden authorization.

### 7.3 Minimal v0.1 Authorization Decision

```text
authorization state = ISSUED
AND
action binding exact
AND
resource binding exact
AND
payload binding exact
AND
atomic claim succeeds
        ↓
ALLOW
```

Otherwise:

```text
DENY(reason)
```

### 7.4 No New Execution Lattice

The initial experiment does not define or claim a new canonical lattice such as:

```text
FAIL ≻ FREEZE ≻ PWC ≻ REFUSAL ≻ PASS
```

No complex state lattice is necessary for this research question.

---

## 8. Timing Semantics

Timing domains remain explicitly separated.

```text
Path A controlled elapsed fixture
        �
sidecar execution duration
        �
network latency
        �
model generation latency
        �
actuator completion time
```

The initial experiment must not feed LLM generation duration, host process duration, wall-clock execution time, or network latency into the existing Path A elapsed input unless a future investigation explicitly establishes that correspondence.

Clocks and timestamps may be recorded as evidence metadata. They are not authorization inputs in v0.1.

---

## 9. Initial Protected Actuator

### 9.1 Operation

```text
WRITE_PROTECTED_FILE
```

### 9.2 Target Property

The producer cannot directly perform the protected mutation through the experimental path.

Only the protected actuator may write the protected target.

### 9.3 Observable Outcomes

Rejected:

```text
decision = DENY
actuation = NOT_ATTEMPTED
effect_generation = 0
protected bytes unchanged
```

Successful authorized actuation:

```text
decision = ALLOW
authorization_state = CONSUMED
actuation = SUCCEEDED
effect_generation = 1
written bytes = exact authorized payload
```

Authorized but failed actuation:

```text
decision = ALLOW
authorization_state = CONSUMED
actuation = FAILED
effect_generation = 0
```

Replay:

```text
decision = DENY(CONSUMED)
additional_effects = 0
```

---

## 10. Adversarial Experimental Matrix

### T0 — Valid Exact Authorization

Matching `ISSUED` authorization, exact action/resource/payload.

Expected:

```text
ALLOW
CONSUMED
SUCCEEDED
exactly one protected effect
```

### T1 — No Authorization

Expected:

```text
DENY(ABSENT)
zero protected effects
```

### T2 — Invalid / Invented Authorization

Expected:

```text
DENY(INVALID)
zero protected effects
```

### T3 — Payload Mutation

One or more payload bytes differ from the authorized binding.

Expected:

```text
DENY(BIND)
zero protected effects
```

### T4 — Resource Substitution

Authorization for resource A is presented for resource B or an unauthorized alias.

Expected:

```text
DENY(BIND)
zero protected effects
```

### T5 — Action Substitution

Authorization for one action is presented for another action.

Expected:

```text
DENY(BIND)
zero protected effects
```

### T6 — Replay After Consumption

Re-submit T0 after authorization is consumed.

Expected:

```text
DENY(CONSUMED)
zero additional effects
```

### T7 — Direct Protected-Resource Bypass

Producer attempts to modify the protected resource directly.

Expected:

```text
OS / authority denial
zero protected effects
```

If this succeeds, the architecture fails immediately.

### T8 — Direct Actuator Bypass

Producer attempts to invoke the actuator without a valid gate decision.

Expected:

```text
denied or unreachable
zero protected effects
```

If this succeeds, the gate is advisory and the architecture fails.

### T9 — Forged Issuance

Producer attempts to create or modify trusted authorization state.

Expected:

```text
OS / authority denial
zero protected effects
```

If the producer can mint a record treated as `ISSUED`, the experiment fails.

### T10 — Stale Issued Authorization Never Submitted

Authorization exists but no valid request reaches the gate.

Expected:

```text
no actuation
effect unchanged
```

This proves authorization existence is not itself actuation.

### T11 — Producer Identity / Provenance Change

Identical explicit request and authorization state; producer label changes.

Expected:

```text
same decision
```

Identity is not authority.

### T12 — Favorable Core Evaluation Without Authorization

If the frozen core is invoked and produces a favorable result but no trusted authorization exists:

Expected:

```text
DENY(ABSENT)
zero protected effects
```

This proves deterministic evaluation is not authorization.

### T13 — Valid Authorization With Core Omitted

For the minimum v0.1 authorization policy, valid authorization exists and the frozen core is not invoked.

Expected:

```text
ALLOW
```

subject to exact action/resource/payload binding.

This proves the core is not hidden authorization.

### T14 — Malformed Request

Expected:

```text
DENY(MALFORMED)
zero protected effects
```

### T15 — Repeated Invalid Attempts

Repeated mismatching requests target one valid `ISSUED` authorization.

Expected:

```text
all DENY
authorization remains ISSUED
zero protected effects
```

A later exact request may still use the authorization once.

### T16 — Authorized Request With Induced Actuator Failure

Expected:

```text
ALLOW
authorization = CONSUMED
actuation = FAILED
effect unchanged
```

This proves authorization and actuation outcome remain distinct.

### T17 — Concurrent Double Submit

Two simultaneous requests compete for one `ISSUED` authorization.

Expected:

```text
exactly one successful authorization claim
at most one actuator invocation
effect_generation <= 1
```

If both produce effects, single-use semantics fail.

---

## 11. Evidence Requirements

Each frozen experimental run should preserve enough information for independent inspection without reproducing the original AI or producer.

Recommended layout:

```text
evidence/
└── run-001/
    ├── specification.md
    ├── environment.txt
    ├── authority-state.txt
    ├── request.bin
    ├── authorization-before.json
    ├── authorization-after.json
    ├── decision.json
    ├── actuator-result.json
    ├── protected-before.bin
    ├── protected-after.bin
    ├── observations.jsonl
    ├── SHA256SUMS.txt
    └── summary.json
```

Evidence should record at minimum:

- run identifier;
- frozen experiment version;
- environment and toolchain versions;
- relevant OS/filesystem assumptions;
- producer, issuer, gate, and actuator principal/permission configuration;
- protected resource ownership and permissions;
- raw serialized request bytes;
- authorization state before and after;
- action, resource, and payload binding;
- gate decision and reason code;
- whether the frozen core was invoked;
- if invoked, exact core inputs, explicit parameters, and exact result;
- authorization claim/consumption result;
- actuator outcome: `NOT_ATTEMPTED`, `SUCCEEDED`, or `FAILED`;
- actuator invocation count;
- protected resource bytes before and after;
- effect generation or equivalent effect count;
- resulting artifact digest;
- expected outcome;
- observed outcome;
- PASS / FAIL;
- timestamps only as metadata;
- full SHA-256 manifest.

Evidence directories become immutable once frozen.

A new experimental condition requires a new run identifier.

Expected results should be frozen before the final evidence run to reduce retrospective reinterpretation.

---

## 12. Proposed Initial Repository Structure

```text
arcstone-mcp-sidecar/
├── Cargo.toml
├── README.md
├── AGENTS.md
├── STATUS.md
├── EXPERIMENT.md
├── src/
│   ├── lib.rs
│   ├── authorization.rs
│   ├── issuer.rs
│   ├── request.rs
│   ├── gate.rs
│   ├── actuator.rs
│   └── evidence.rs
├── src/bin/
│   └── arcstone-exec.rs
├── tests/
│   ├── authorization_matrix.rs
│   ├── actuator_boundary.rs
│   ├── core_separation.rs
│   ├── concurrency.rs
│   └── replay.rs
└── evidence/
```

The initial implementation may use native Rust and local process boundaries.

The repository name may remain `arcstone-mcp-sidecar`, but MCP is not required for the first experiment and must not define the scientific claim.

No internal component should be named `arcstone-core`.

If a kernel-specific name becomes necessary, use a downstream-specific name such as `arcstone-execution-kernel` or keep the implementation internal to the repository.

---

## 13. Phase-Gated Development Program

### Phase 0 — Specification Definition

Complete before repository implementation:

- define frozen upstream relationships;
- define trust boundary;
- define issuer role;
- define authorization semantics;
- define consumption semantics;
- define protected resource identity;
- define one protected actuator;
- define adversarial matrix;
- define evidence schema;
- define PASS / FAIL criteria;
- define non-claims.

### Phase 1 — Downstream Admission Audit

Verify:

- no Continuity Core modification;
- no Continuity Core semantic reinterpretation;
- no Ingress Lab modification;
- sibling downstream placement;
- evaluation is not authorization;
- authorization is not actuation;
- producer identity is not authority;
- timing domains remain separate;
- issuer state is producer-inaccessible;
- actuator and resource are producer-inaccessible;
- no canonical authority is claimed.

The experiment definition should be frozen only after this audit passes.

### Phase 2 — Minimal Native Experiment

Implement only:

- local CLI / stdio;
- explicit request serialization;
- trusted issuer fixture;
- boundary-owned authorization state;
- deterministic gate;
- atomic single-use claim / consume;
- one `WRITE_PROTECTED_FILE` actuator;
- protected resource isolation;
- evidence capture.

Explicitly exclude:

- MCP server;
- NPM publication;
- WASM;
- HMAC / signatures / PKI;
- generalized policy compiler;
- multiple actuator adapters;
- network services;
- production deployment;
- exactly-once delivery machinery.

### Phase 3 — Adversarial Validation

Run T0–T17.

The experiment passes only if all frozen expected outcomes are observed, especially:

```text
unauthorized conditions
        ⇒
zero protected effects
```

```text
one valid single-use authorization
        ⇒
at most one protected effect
```

```text
direct producer bypass
        ⇒
denied
```

```text
producer authorization forgery
        ⇒
denied
```

```text
double-submit race
        ⇒
at most one successful claim / effect
```

### Phase 4 — Evidence Freeze

After successful validation:

- preserve exact evidence;
- generate SHA-256 manifest;
- record environment and authority configuration;
- record commit;
- rerun full verification;
- freeze the evidence directory;
- prohibit mutation of the frozen run.

### Phase 5 — Capability Hardening

Only after the minimal boundary is validated, investigate:

- cryptographic authorization objects;
- HMAC or asymmetric capability grants;
- nonce / replay mechanisms beyond local consumed state;
- expiry;
- key custody;
- issuer / verifier separation;
- stronger resource identity mechanisms.

### Phase 6 — Isolation

Investigate whether additional isolation materially improves the verified boundary:

- WASM;
- native sandboxing;
- process isolation;
- restricted OS capabilities;
- containers or VMs if justified by a new threat model.

### Phase 7 — Machine Protocol

After authorization semantics stabilize:

- MCP;
- JSON-RPC;
- structured refusal / retry semantics;
- machine-readable capability descriptions.

Protocol wrappers must preserve the underlying execution-boundary semantics.

### Phase 8 — Distribution

Only after experimental evidence and interface stabilization:

- standalone public release;
- package distribution;
- optional NPM package;
- broader actuator adapters;
- documented integration surface.

---

## 14. Deferred Candidate Mechanisms

The following ideas remain potentially valuable but are explicitly deferred until justified by evidence:

- HMAC, signatures, PKI, KMS, capability certificates;
- MCP;
- WASM;
- containers / VMs as the primary isolation story;
- network APIs, brokers, queues, or remote RPC;
- static 4096-byte execution Control Frame;
- decoupled payload streaming;
- WASM linear-memory staging;
- in-sandbox payload hashing;
- sliding-window rate counters;
- runtime execution-budget clamping;
- new execution-state lattices;
- post-write verification state;
- YAML / policy-language compilation;
- generalized tool-schema mediation;
- agent frameworks or model-specific integrations;
- identity providers and model trust scores;
- standing capabilities;
- delegation or multi-party authorization;
- exactly-once distributed actuation;
- NPM package distribution.

These are candidates for later experiments, not inherited guarantees.

---

## 15. Experimental Success Criteria

The v0.1 experiment is successful only if all of the following are demonstrated with preserved evidence:

1. The producer cannot perform the protected mutation directly.
2. The producer cannot invoke the actuator around the gate.
3. The producer cannot create or modify trusted authorization state.
4. Invalid, absent, malformed, mismatched, or reused authorization produces zero protected effect.
5. A valid single-use authorization produces at most one protected effect.
6. The written payload exactly matches the authorized payload.
7. Resource substitution and action substitution are rejected.
8. Authorization replay is rejected.
9. Concurrent double-submit produces at most one successful claim and effect.
10. Producer provenance changes do not affect the decision unless provenance is explicitly admitted as a policy input.
11. Favorable core evaluation without authorization remains non-executable.
12. Valid authorization can be evaluated without treating the core as hidden authority.
13. Authorization state and actuator outcome remain distinguishable.
14. Actuator failure does not masquerade as authorization failure or success.
15. Timing semantics are not silently reinterpreted.
16. Evidence can be replayed and independently inspected.
17. The frozen upstream repositories remain unchanged.
18. No result is promoted into a broader architectural guarantee beyond the experiment's scope.

---

## 16. Failure / Stop Conditions

Implementation must stop and the specification must be reopened if any of the following occurs:

- the experiment requires modifying the frozen Continuity Core;
- the experiment requires modifying frozen Ingress Lab evidence;
- downstream authorization depends on an unstated upstream reinterpretation;
- the producer can mutate the protected resource directly;
- the producer can invoke the actuator without the gate;
- the producer can create a record the gate accepts as trusted authorization;
- payload mutation, resource substitution, or action substitution still actuates;
- replay causes a second effect;
- two racers consume one authorization and produce multiple effects;
- gate authorization depends on producer identity, wall-clock timing, host load, or core verdict without explicit experimental admission;
- authorization and actuation outcome cannot be distinguished;
- resource identity is too ambiguous to define a falsifiable binding;
- evidence cannot establish whether a protected mutation occurred;
- implementation complexity begins requiring MCP, WASM, cryptography, distributed locking, or package distribution merely to prove the initial invariant.

A failed condition must not be repaired by redefining PASS.

It must trigger a new downstream question or architecture revision.

---

## 17. Multi-Principal Value Model

| Principal | Experimental Value |
|---|---|
| Humans | Define and govern bounded execution conditions without manually approving each machine action. |
| AI systems | Receive structured machine-readable execution outcomes without gaining authority over the deterministic boundary. |
| Autonomous agents | Propose, receive denial/authorization outcomes, and re-plan without directly controlling protected actuation. |
| Machines / cloud processes | Use explicit deterministic execution conditions and replayable evidence around protected mutations. |

These are intended system properties, not validated production claims.

---

## 18. Bounded Claim if v0.1 Passes

A successful v0.1 experiment would support approximately this claim:

> **Under the frozen experimental environment and authority configuration, an untrusted producer could not cause the defined protected OS-visible side effect through the tested routes without matching boundary-controlled authorization; one valid authorization produced at most one protected actuation attempt, and authorization, actuation, and observed effect remained separately inspectable.**

It would **not** establish:

- cryptographic capability security;
- arbitrary-code containment;
- remote attacker resistance;
- universal sandbox security;
- kernel security;
- AI alignment;
- safe autonomous agents in general;
- race-free distributed execution;
- exactly-once delivery;
- production authorization;
- physical safety.

---

## 19. Working Principle

> **Evidence before expansion. Preserve the core. Test the boundary. Freeze completed evidence.**

The Arcstone Execution Boundary should advance only when a concrete execution-boundary question requires the next mechanism.

The first target is not a complete agent platform.

The first target is one deterministic authorization boundary blocking one protected irreversible action.
