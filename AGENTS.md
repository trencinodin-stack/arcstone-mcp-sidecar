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
