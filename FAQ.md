# FAQ

## Is this the Arcstone Continuity Core?

No.

The Continuity Core is frozen. This repository is a separate downstream, non-canonical execution-boundary experiment.

## Does a favorable Continuity Core result authorize execution?

No.

```text
deterministic evaluation ≠ authorization
```

The Execution Boundary owns its own explicitly downstream authorization semantics.

## Is the Path A Ingress Lab a runtime dependency?

No.

The Ingress Lab and Execution Boundary are sibling downstream experiments.

## Is the producer trusted?

No.

AI models, agents, scripts, humans, workflows, and other machines are all treated as external proposal producers unless a future experiment explicitly changes that assumption.

## Can producer identity cause ALLOW?

No.

Producer identity/provenance is metadata in v0.1.

## What is the protected action?

Exactly one v0.1 operation:

```text
WRITE_PROTECTED_FILE
```

to the internally mapped resource:

```text
EFFECT_LOG
```

## Is authorization reusable?

No.

v0.1 authorization is single-use and uses at-most-once authorization semantics.

## Does ALLOW mean the actuator succeeded?

No.

```text
authorization decision ≠ actuation outcome ≠ observed effect
```

## Why no MCP yet?

Because MCP is not required to test the first falsifiable execution-boundary invariant.

## Why no HMAC/signature yet?

Because v0.1 keeps authorization state inside the trusted local boundary. Cryptographic authenticity is a later threat-model question.

## Is this production security?

No.

The bounded v0.1 experiment tests one local authority configuration and one protected resource.
