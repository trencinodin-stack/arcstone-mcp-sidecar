# BUILD REPORT

**Repository:** `arcstone-mcp-sidecar`
**System:** Arcstone Execution Boundary
**Version:** 0.1.0 implementation starter
**Status:** PRE-EVIDENCE / NOT YET VALIDATED

## Purpose

This report gives humans and machine operators one compact place to determine what has been built, what has been tested, and what remains unproven.

## Implemented

- native Rust crate
- explicit execution request schema
- trusted authorization issuer fixture
- boundary-owned authorization state
- deterministic gate
- action/resource/payload binding
- local atomic single-use authorization claim
- protected-file actuator
- evidence summary emission
- replay tests
- mutation/substitution tests
- core/evaluation separation tests
- actuator-failure test
- concurrent double-submit test
- Windows authority-boundary procedure
- CI workflow

## Not Yet Established

- successful compile/test on the target Windows machine
- real producer-vs-authority Windows principal separation
- T7 direct protected-resource bypass denial
- T8 environment-level direct actuator bypass denial
- T9 forged-issuance denial under Windows ACLs
- frozen Execution Boundary Run 001 evidence

## Machine Decision

```text
implementation_present = true
experiment_complete = false
evidence_frozen = false
production_ready = false
canonical_authority = false
```

Do not infer more than these fields support.
