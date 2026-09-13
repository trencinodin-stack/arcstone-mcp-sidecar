# AGENTS.md

This repository is an experimental downstream realization.

## Hard boundaries

Do not:

- modify or reinterpret the Arcstone Continuity Core;
- modify frozen Path A Ingress Lab evidence;
- treat a favorable upstream result as authorization;
- make producer identity authoritative;
- add MCP, WASM, cryptographic capability systems, network services, or generalized policy engines to v0.1 without a new research requirement;
- collapse authorization, actuation, and observed effect into one state;
- weaken single-use claim semantics;
- claim OS-level bypass resistance until the separate-principal environment test passes.

## v0.1 target

One explicit authorization record.
One exact action.
One exact resource identity.
One exact payload binding.
One atomic claim.
One protected-file actuator.
At most one protected actuation attempt.

## Resource identity

The only authorized v0.1 resource identifier is:

```text
EFFECT_LOG
```

It maps internally to:

```text
<root>/protected/effect.bin
```

The producer never supplies the filesystem target path used by the actuator.

## Authorization identity

Authorization IDs are explicit caller-supplied fixtures. Do not generate random authorization IDs in v0.1.

## Evidence

Evidence is downstream experimental evidence only. It has no canonical authority over frozen upstream systems.
