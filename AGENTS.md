Markdown
# AGENTS.md

This repository is an experimental downstream realization.

## Public research relationship

The public Arcstone research surface consists of three independently scoped repositories:

```text
arcstone-continuity-core
    frozen deterministic Path A reference surface

arcstone-path-a-ingress-lab
    external-producer ingress and replay evidence

arcstone-mcp-sidecar
    Arcstone Execution Boundary — this repository
These repositories form a conceptual research and evidence progression, not a mandatory runtime pipeline.

The Path A Ingress Lab and Execution Boundary are sibling downstream investigations of the frozen Arcstone Continuity Core.

For this repository:

Plaintext
deterministic evaluation
≠ authorization
≠ actuation
≠ observed effect
A favorable Continuity Core result does not create execution authority. The Execution Boundary must preserve its independent authorization lifecycle and must not infer authorization from upstream evaluation.

arcstone-mcp-sidecar is published on the official Model Context Protocol (MCP) Registry (io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0). The underlying implementation exposes the Arcstone Execution Boundary deterministic interface.

Hard boundaries
Do not:

modify or reinterpret the Arcstone Continuity Core;

modify frozen Path A Ingress Lab evidence;

treat a favorable upstream result as authorization;

make producer identity authoritative;

add WASM, cryptographic capability systems, network services, or generalized policy engines to v0.1 without a new research requirement;

collapse authorization, actuation, and observed effect into one state;

weaken single-use claim semantics;

generalize the bounded Windows authority-boundary evidence into a claim of universal OS-level bypass resistance;

treat the completed Windows sub-experiment as completion of the final integrated T0–T17 evidence run or Execution Boundary Run 001.

v0.1 target
One explicit authorization record.

One exact action.

One exact resource identity.

One exact payload binding.

One atomic claim.

One protected-file actuator.

At most one protected actuation attempt.

Resource identity
The only authorized v0.1 resource identifier is:

Plaintext
EFFECT_LOG
It maps internally to:

Plaintext
<root>/protected/effect.bin
The producer never supplies the filesystem target path used by the actuator.

Authorization identity
Authorization IDs are explicit caller-supplied fixtures. Do not generate random authorization IDs in v0.1.

Evidence
Evidence is downstream experimental evidence only. It has no canonical authority over frozen upstream systems.

The bounded Windows authority-boundary experiment is complete and frozen for its tested configuration.

Before making OS-authority claims, read:

Plaintext
docs/WINDOWS-AUTHORITY-BOUNDARY.md
The recorded Windows evidence supports only the tested producer/authority principal separation and ACL configuration. It does not establish general Windows sandbox security, arbitrary hostile-code containment, universal bypass resistance, or production authorization security.

The final integrated T0–T17 evidence run and frozen Execution Boundary Run 001 remain separate evidence milestones until explicitly completed and recorded.


***

### 2. `machine/repo-policy.json`

```json
{
  "schema": "arcstone-execution-boundary/repo-policy-v1",
  "system": "Arcstone Execution Boundary",
  "repository": "arcstone-mcp-sidecar",
  "mcp_registry": {
    "status": "active",
    "reverse_dns": "io.github.trencinodin-stack/arcstone-mcp-sidecar",
    "version": "0.1.0"
  },
  "rules": {
    "no_upstream_mutation": [
      "arcstone-continuity-core",
      "arcstone-path-a-ingress-lab"
    ],
    "no_implicit_authority": true,
    "no_producer_authority": true,
    "requires_new_research_question_before_addition": [
      "wasm",
      "hmac",
      "pki",
      "network-api",
      "policy-dsl",
      "agent-framework"
    ],
    "disallowed_state_collapses": [
      "auth_equals_actuation",
      "evaluation_equals_auth",
      "actuation_equals_observed_effect"
    ],
    "atomic_claim_required": true,
    "windows_evidence_bounded_only": true
  },
  "allowed_v0_1_scope": {
    "transports": [
      "local-cli",
      "stdio"
    ],
    "auth_store": "filesystem-fixture",
    "actuator": "protected-file-append",
    "resource": "EFFECT_LOG"
  }
}
3. SPECIFICATION.md
Markdown
# Arcstone Execution Boundary — 0.1 Experimental Definition

> **Implementation Note:** The v0.1.0 release is officially indexed on the Model Context Protocol (MCP) Registry as `io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0`.

This document defines the research question, architectural invariants, evidence requirements, and experimental boundaries for the downstream realization `arcstone-mcp-sidecar` (package `arcstone-execution-boundary`).

---

## 1. Experimental Frame

### 1.1 Primary Research Question
> Can a deterministic downstream execution boundary prevent an untrusted or nondeterministic producer from causing a protected side effect unless explicit boundary-controlled authorization has been satisfied?

### 1.2 Upstream Dependency
- **Continuity Core:** Frozen reference surface. Non-authoritative for actuation.
- **Path A Ingress Lab:** Sibling investigation.

### 1.3 Target Invariants
- **I1 (Non-Authorization Safety):** `NO VALID AUTHORIZATION ⇒ ZERO PROTECTED ACTUATION`
- **I2 (Single-Use Authority):** `ONE VALID SINGLE-USE AUTHORIZATION ⇒ AT MOST ONE PROTECTED ACTUATION`
- **I3 (Exclusive Actuation Authority):** `UNTRUSTED PRODUCER ⇏ PROTECTED RESOURCE` (Boundary is the sole valid mutation path).

---

## 2. System Boundaries & Deferred Features

The v0.1.0 baseline evaluates the core execution boundary interface via local CLI and stdio mechanisms.

**Intentionally Deferred Scope:**
- WASM runtimes
- Cryptographic signatures / HMAC / PKI
- Generalized network transports
- Dynamic policy DSL engines
4. FAQ.md
Markdown
# Frequently Asked Questions

### What is the primary role of this repository?
`arcstone-mcp-sidecar` (crate `arcstone-execution-boundary`) serves as a deterministic execution boundary proxy designed to enforce safety invariants ($I1, I2, I3$) between untrusted producers and protected system side effects.

### What is the MCP registration status?
The repository is officially registered and published on the Model Context Protocol (MCP) Registry under the reverse-DNS identifier:
`io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0`

### What invariants does this system test?
- **I1:** No valid authorization $\Rightarrow$ Zero protected actuation.
- **I2:** One valid single-use authorization $\Rightarrow$ At most one protected actuation.
- **I3:** Untrusted producer $\nRightarrow$ Protected resource.

### Is this a production authorization server?
No. This implementation is classified as **Experimental / Downstream / Non-Canonical** and serves as
