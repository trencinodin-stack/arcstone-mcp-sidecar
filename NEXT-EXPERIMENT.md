---

### 2. `NEXT-EXPERIMENT.md`

```markdown
# NEXT EXPERIMENT: ISOLATION HARDENING (PHASE 6)

## Current state

The v0.1.0 baseline for `arcstone-mcp-sidecar` (crate `arcstone-execution-boundary`) is officially indexed on the Model Context Protocol (MCP) Registry (`io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0`).

The integrated T0–T17 adversarial validation matrix and Execution Boundary Run 001 are completed and frozen.

## Next Target: Phase 6 Isolation Hardening

With the baseline MCP sidecar interface and safety invariants ($I1, I2, I3$) established, the next experimental phase evaluates capability isolation mechanisms:

1. **WASM Runtimes:** Sandboxing execution boundaries in WebAssembly to enforce linear-memory boundaries.
2. **Cryptographic Capability Grants:** Replacing fixture-based authorization stores with signed HMAC/PKI capability tokens.
3. **Restricted OS Capabilities:** Testing native process isolation and restricted system call surfaces under Windows/Linux sandboxing.

No work in Phase 6 may alter or invalidate the frozen Run 001 evidence.

## Entry condition

Proceed only if:

```text
cargo test
passes on the target machine.

Evidence target
A new scientific condition requires a new run identifier and, if it changes the research question, a new specification phase.
