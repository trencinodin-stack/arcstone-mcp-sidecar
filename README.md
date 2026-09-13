# Arcstone Execution Boundary

**Repository identity:** `arcstone-mcp-sidecar`
**Implementation package:** `arcstone-execution-boundary`
**Status:** Experimental / Downstream / Non-Canonical / Pre-Evidence
**Version:** 0.1.0 implementation starter

This repository is the initial native implementation of the frozen **Arcstone Execution Boundary v0.1 experimental specification**.

It tests one bounded question:

> Can a deterministic downstream execution boundary prevent an untrusted or nondeterministic producer from causing a protected side effect unless an explicit, boundary-controlled authorization condition has been satisfied?

## Frozen upstream relationship

```text
                 Arcstone Continuity Core
                        FROZEN
                       /      \
                      /        \
                     ▼          ▼
       Path A Ingress Lab    Execution Boundary
          COMPLETE/FROZEN       THIS REPO
```

The Path A Ingress Lab and Execution Boundary are sibling downstream investigations.

This repository does **not** modify or reinterpret either frozen upstream repository.

## v0.1 invariants

```text
I1 — NO VALID AUTHORIZATION
     ⇒ ZERO PROTECTED ACTUATION

I2 — ONE VALID SINGLE-USE AUTHORIZATION
     ⇒ AT MOST ONE PROTECTED ACTUATION

I3 — UNTRUSTED PRODUCER
     ⇏ PROTECTED RESOURCE
```

The implementation explicitly keeps separate:

```text
authorization lifecycle
≠ gate decision
≠ actuation result
≠ observed protected effect
```

## What is intentionally not here

v0.1 does not require or claim:

- MCP
- WASM
- HMAC / signatures / PKI / KMS
- network APIs
- generalized policy engines
- agent frameworks
- package distribution
- exactly-once real-world execution
- production authorization security
- AI alignment or general agent safety

## Local data model

The experimental root has this shape:

```text
runtime/
├── auth/
│   ├── issued/
│   └── consumed/
├── protected/
│   └── effect.bin
└── evidence/
    └── <run-id>/
```

An authorization record is trusted because the untrusted producer is not permitted to create or modify the `auth` tree. The authorization identifier itself carries no authority.

The `consumed/<authorization-id>.claim` file is created with `create_new(true)`. That atomic local reservation is the v0.1 single-use claim primitive.

## Build

Windows CMD:

```cmd
cargo build
cargo test
```

or:

```cmd
scripts\windows\verify.cmd
```

## Example

Create the runtime directories:

```cmd
arcstone-exec init --root runtime
```

Issue one authorization:

```cmd
arcstone-exec issue --root runtime --auth-id AUTH-001 --action WRITE_PROTECTED_FILE --resource-id EFFECT_LOG --payload-hex 48454c4c4f
```

Create `request.json`:

```json
{
  "authorization_id": "AUTH-001",
  "action": "WRITE_PROTECTED_FILE",
  "resource_id": "EFFECT_LOG",
  "payload_hex": "48454c4c4f"
}
```

Execute it:

```cmd
arcstone-exec execute --root runtime --request request.json --run-id manual-001
```

Inspect authorization state:

```cmd
arcstone-exec inspect --root runtime --auth-id AUTH-001
```

A replay of the same request should return a denied result because the claim already exists.

## Important OS-boundary note

The Rust tests can verify deterministic authorization, binding, consumption, replay, failure separation, and concurrent claiming.

They **cannot prove exclusive OS authority when producer and boundary run as the same Windows user**.

The v0.1 evidence run must therefore include a separate authority configuration in which the producer principal cannot:

1. write the protected target;
2. write the authorization store; or
3. invoke a bypass path with equivalent filesystem authority.

See [`docs/WINDOWS-AUTHORITY-BOUNDARY.md`](docs/WINDOWS-AUTHORITY-BOUNDARY.md).

Do not describe the OS authority invariant as validated until that environment-level test passes.



## Machine / AI / autonomous-agent navigation

This repository is intentionally machine-operable and machine-readable.

Start here:

```text
AGENTS.md
machine/system-manifest.json
machine/repo-policy.json
machine/protocol.json
machine/test-matrix.json
STATUS.md
BUILD-REPORT.md
EXPERIMENT.md
```

Recommended automated traversal:

```text
1. Read machine/system-manifest.json
2. Read machine/repo-policy.json
3. Read AGENTS.md
4. Read STATUS.md
5. Read EXPERIMENT.md
6. Run cargo test
7. Do not claim OS authority validation until the separate-principal checks pass
```

Humans are not required in the deterministic request/decision path. Human-facing prose exists primarily for governance, scientific interpretation, and review.

## Working principle

> **Evidence before expansion. Preserve the core. Test the boundary. Freeze completed evidence.**
