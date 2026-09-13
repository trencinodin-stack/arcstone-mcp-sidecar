# BUILD REPORT

**Repository:** `arcstone-mcp-sidecar`
**System:** Arcstone Execution Boundary
**Version:** 0.1.0 implementation starter
**Status:** INTEGRATED T0-T17 PASS / RUN 001 FROZEN

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
- corrected Windows producer-access harness
- CI workflow

## Established

- successful compile and test on the target Windows machine
- local Windows verification script PASS
- separate non-administrator producer and authority principals established for the bounded Windows experiment
- explicit filesystem ACL separation established in a dedicated disposable runtime
- T7 direct protected-resource bypass denial: PASS under the tested ACL configuration
- T8 direct actuator bypass: PASS - bounded public-API/structural result
- T9 forged-issuance denial: PASS under the tested ACL configuration
- authorized execution: `ISSUED` -> `CONSUMED`, decision `ALLOW`, actuation `SUCCEEDED`
- replay of consumed authorization: `DENY`, actuation `NOT_ATTEMPTED`
- I2 single-use / at-most-once authority supported for the tested authorization
- I3 producer-to-protected-resource separation supported for the tested Windows configuration
- bounded Windows evidence package frozen and hash-anchored
- final integrated T0-T17 evidence run: PASS
- Execution Boundary RUN-001 evidence package frozen and hash-anchored

## Not Yet Established

- production authorization security
- general Windows sandbox security
- arbitrary hostile-code containment
- universal bypass resistance
- distributed exactly-once execution

## Evidence boundary

The completed Windows authority-boundary experiment is a bounded environment-level result.

It establishes that, under the recorded two-principal Windows configuration and ACLs, the tested producer principal could not directly create the protected-resource probe or forge an authorization record. The tested authorization was consumed on successful execution, and replay was denied before another actuation attempt.

T8 additionally establishes that an external Rust consumer cannot import the production actuator through the crate's public API. This is a bounded structural result and is not a claim that no conceivable Windows bypass exists.

The authority principal retains filesystem authority within the experimental runtime. The experiment therefore demonstrates tested producer/authority separation, not exclusive process-level control by `arcstone-exec`.

See [`docs/WINDOWS-AUTHORITY-BOUNDARY.md`](docs/WINDOWS-AUTHORITY-BOUNDARY.md) for the detailed evidence record, preserved hashes, configuration, and limitations.

## Machine Decision

    implementation_present = true
    windows_bounded_evidence_complete = true
    experiment_complete = true
    evidence_frozen = true
    production_ready = false
    canonical_authority = false

`windows_bounded_evidence_complete = true` refers only to the completed and frozen bounded Windows authority-boundary experiment.

`experiment_complete = true` and `evidence_frozen = true` reflect completion and freeze of the bounded v0.1 RUN-001 experiment and evidence package. They do not imply production readiness or canonical authority.

Do not infer more than these fields support.
