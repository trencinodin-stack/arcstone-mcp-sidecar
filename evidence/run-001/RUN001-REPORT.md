# Arcstone Execution Boundary - RUN-001 Report

**System:** Arcstone Execution Boundary
**Version:** 0.1.0
**Run:** RUN-001
**Classification:** Experimental / Downstream / Non-Canonical
**Baseline Commit:** `0e0a0c6486334e996f43ff6781dfedb746481828`
**Determination:** PASS - FROZEN

## 1. Purpose

RUN-001 is the final integrated evidence run for the Arcstone Execution Boundary v0.1 experimental specification.

Its purpose is to determine whether the implemented downstream execution boundary satisfies the frozen T0-T17 test matrix while preserving the existing authorization, decision, actuation, and evidence semantics.

RUN-001 does not establish production readiness or canonical authority.

## 2. Baseline

RUN-001 began from a clean `main` worktree at:

    0e0a0c6486334e996f43ff6781dfedb746481828

The governing RUN-001 procedure was committed before execution.

No implementation source change was introduced to obtain the RUN-001 result.

## 3. Integrated Verification

Native verification was executed with:

    cargo test --all-targets

All discovered native tests passed with zero failures.

The Windows repository verification entry point was also executed and returned:

    Arcstone Execution Boundary local Rust verification PASS.
    NOTE: OS-principal bypass tests are separate and are NOT proven by cargo test.

The separation between native verification and OS-principal authority evidence was therefore preserved.

## 4. T0-T17 Determination

Every test identifier T0 through T17 is explicitly accounted for in:

`evidence/run-001/evidence-map.json`

Native executable evidence covers T0-T6 and T10-T17.

T7 and T9 are supported by the already-completed and frozen bounded Windows authority-boundary experiment.

T8 is supported by the bounded structural/public-API probe together with the tested Windows authority configuration.

Integrated determination:

    T0-T17: PASS

## 5. Invariant Determination

### I1 - Non-Authorization Safety

    NO VALID AUTHORIZATION
            =>
    ZERO PROTECTED ACTUATION

**Determination:** PASS

Basis includes denied-path native evidence across absent, invalid, mutated, substituted, malformed, and favorable-core-without-authorization cases.

### I2 - Single-Use Authority

    ONE VALID SINGLE-USE AUTHORIZATION
            =>
    AT MOST ONE PROTECTED ACTUATION

**Determination:** PASS

Basis includes valid exact authorization, replay denial, consumed-on-failure behavior, concurrency behavior, and the frozen Windows replay result.

### I3 - Exclusive Actuation Authority

    UNTRUSTED PRODUCER
            !=>
    PROTECTED RESOURCE

**Determination:** PASS_BOUNDED

This determination is limited to the tested Windows ACL authority configuration and bounded structural/public-API evidence already preserved by the repository.

It is not a universal operating-system security claim.

## 6. Preserved State Separation

RUN-001 preserves the distinction between:

    authorization state
    decision
    actuation outcome
    observed effect

A favorable core observation does not constitute authorization.

Producer identity or provenance does not constitute authorization.

`ALLOW` does not mean successful actuation.

A failed actuation after successful claim leaves authorization consumed.

Exactly-once execution is not claimed.

## 7. Frozen Windows Evidence Reference

The previously frozen Windows authority-boundary package is referenced by SHA-256:

    e8beae870214d1e53cf4f7930341b79ffdb7ac884e563cac2c52ef1d8818ccaa

RUN-001 does not rerun or reinterpret that experiment merely for procedural uniformity.

## 8. Interpretation Boundary

A successful RUN-001 does not establish:

- production security
- general Windows sandbox security
- arbitrary hostile-code containment
- universal bypass resistance
- process isolation against the trusted authority principal
- distributed exactly-once execution
- cryptographic capability security
- MCP security
- WASM security
- network authorization security

The repository remains experimental, downstream, non-canonical, and not production-ready.

## 9. Evidence Artifacts

RUN-001 currently contains:

    evidence/run-001/
      manifest.json
      environment.json
      cargo-test.txt
      windows-verify.txt
      evidence-map.json
      RUN001-REPORT.md

A final `SHA256SUMS.txt` SHALL be generated during the freeze step.

## 10. Determination

The integrated evidence supports:

    T0-T17 = PASS
    I1 = PASS
    I2 = PASS
    I3 = PASS_BOUNDED

No RUN-001 stop condition was observed.

RUN-001 is therefore eligible for final checksum anchoring and freeze.

Final checksum anchoring is complete. The formal state is:

    freeze_state = FROZEN

> Evidence before expansion. Preserve the core. Test the boundary. Freeze completed evidence.

