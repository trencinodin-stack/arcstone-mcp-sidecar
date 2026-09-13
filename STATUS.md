# STATUS

**Repository:** `arcstone-mcp-sidecar`
**System:** Arcstone Execution Boundary
**Version:** 0.1.0 implementation starter
**Classification:** Experimental / Downstream / Non-Canonical
**Current stage:** Minimal native implementation / bounded Windows evidence complete

## Frozen upstream

- Arcstone Continuity Core: frozen
- Arcstone Path A Ingress Lab Run 001 / Run 002: complete and frozen

## Admission status

- Blind architecture reconstruction: completed across Gemini, ChatGPT, and Grok
- Downstream specification sanity audit: approved for implementation
- Master Substrate upstream admission audit: approved for downstream admission
- Blocking architectural conflicts: none identified in the admission record

## Current implementation scope

Included:

- explicit request model
- trusted issuer fixture
- boundary-owned authorization records
- payload/action/resource binding
- atomic single-use claim using local filesystem `create_new`
- deterministic gate decision
- protected-file actuator
- evidence capture
- replay and concurrency tests
- core/evaluation separation tests

Not yet validated:

- final T0–T17 evidence run
- frozen Run 001 evidence for this repository

## Windows authority-boundary evidence

- bounded Windows two-principal authority experiment: complete
- T7 direct protected-resource bypass: PASS under tested ACL configuration
- T8 direct actuator bypass: PASS — bounded public-API/structural result
- T9 forged authorization issuance: PASS under tested ACL configuration
- I2 single-use / at-most-once authority: supported for the tested authorization
- I3 producer-to-protected-resource separation: supported for the tested Windows configuration
- evidence package frozen and hash-anchored
- no claim of general Windows sandbox security

## Deferred

- MCP
- WASM
- cryptographic capabilities
- network transport
- product/distribution work
