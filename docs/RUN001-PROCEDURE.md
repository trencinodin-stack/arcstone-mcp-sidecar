# Execution Boundary Run 001 Procedure

**System:** Arcstone Execution Boundary  
**Version:** 0.1.0  
**Run:** RUN-001  
**Classification:** Experimental / Downstream / Non-Canonical  
**Purpose:** Final integrated T0-T17 evidence run and evidence freeze

## 1. Objective

RUN-001 determines whether the implemented Arcstone Execution Boundary satisfies the frozen v0.1 experimental test matrix across T0-T17 without changing the execution architecture, authorization semantics, protected actuator, or upstream systems.

RUN-001 is an evidence integration and freeze procedure.

It is not:

- a new architecture experiment
- a production-readiness determination
- a canonical authority claim
- a general operating-system security claim
- an MCP, WASM, network, PKI, or package-distribution experiment

## 2. Governing Sources

RUN-001 SHALL use the repository's existing machine and experiment contracts:

- `EXPERIMENT.md`
- `SPECIFICATION.md`
- `machine/protocol.json`
- `machine/test-matrix.json`
- `machine/repo-policy.json`
- `machine/system-manifest.json`
- `docs/WINDOWS-AUTHORITY-BOUNDARY.md`

No RUN-001 artifact may redefine their protocol semantics.

## 3. Baseline Requirement

RUN-001 SHALL begin from a clean repository worktree.

The exact Git commit used for execution SHALL be recorded in the RUN-001 evidence.

No source-code modification may occur between baseline capture and execution of the evidence run.

If source code changes are required to make RUN-001 pass, RUN-001 SHALL stop and the change SHALL be evaluated separately before a new run is attempted.

## 4. Evidence Sources

### 4.1 Native executable evidence

T0-T6 and T10-T17 SHALL be evaluated using the existing native Rust test harness and implementation.

The integrated native verification SHALL include:

```text
cargo test --all-targets
```

The existing Windows verification entry point SHALL also be executed:

```text
scripts\windows\verify.cmd
```

### 4.2 Frozen Windows authority-boundary evidence

T7 and T9 SHALL use the already-completed and frozen bounded Windows authority-boundary evidence.

T8 SHALL use the already-completed bounded structural/public-API result together with the tested Windows authority environment.

RUN-001 SHALL reference this frozen evidence rather than rerun or reinterpret it merely for procedural uniformity.

The preserved Windows evidence package and its SHA-256 anchor SHALL be recorded in the RUN-001 evidence manifest.

## 5. Test Matrix Requirement

RUN-001 SHALL explicitly account for every test identifier T0 through T17.

For each identifier, the evidence map SHALL record:

- test identifier
- expected result from `machine/test-matrix.json`
- evidence source
- observed result
- PASS or FAIL determination
- relevant artifact or test reference

No identifier may be silently omitted.

## 6. Primary Invariant Determination

RUN-001 SHALL evaluate the following bounded experimental invariants:

### I1 - Non-Authorization Safety

```text
NO VALID AUTHORIZATION
        =>
ZERO PROTECTED ACTUATION
```

### I2 - Single-Use Authority

```text
ONE VALID SINGLE-USE AUTHORIZATION
        =>
AT MOST ONE PROTECTED ACTUATION
```

### I3 - Exclusive Actuation Authority

```text
UNTRUSTED PRODUCER
        !=>
PROTECTED RESOURCE
```

I3 SHALL remain bounded to the tested authority configuration and structural/public-API evidence already recorded by the repository.

## 7. Required State Separation

RUN-001 SHALL preserve the existing distinction between:

```text
authorization state
decision
actuation outcome
observed effect
```

A favorable core observation SHALL NOT constitute authorization.

Producer identity or provenance SHALL NOT constitute authorization.

`ALLOW` SHALL NOT be redefined as successful actuation.

A failed actuation after successful claim SHALL leave the authorization consumed.

Exactly-once execution SHALL NOT be claimed.

## 8. RUN-001 Evidence Package

The integrated evidence package SHALL contain, at minimum:

```text
evidence/run-001/
  manifest.json
  environment.json
  cargo-test.txt
  windows-verify.txt
  evidence-map.json
  RUN001-REPORT.md
  SHA256SUMS.txt
```

Additional evidence may be included where needed, but additional architectural layers SHALL NOT be introduced merely to create the package.

## 9. Manifest Requirements

`manifest.json` SHALL record at minimum:

- schema identifier
- system name
- repository
- version
- run identifier
- classification
- exact Git commit
- worktree state at baseline
- execution environment reference
- protocol reference
- test-matrix reference
- frozen Windows evidence reference
- frozen Windows evidence SHA-256
- integrated T0-T17 determination
- invariant determinations
- RUN-001 freeze state

## 10. Environment Record

`environment.json` SHALL record sufficient information to identify the environment used for the integrated native run, including at minimum:

- operating system
- Rust compiler version
- Cargo version
- Git commit
- repository branch
- execution timestamp

Environment metadata is evidence context and SHALL NOT alter authorization decisions.

## 11. Freeze Conditions

RUN-001 may be declared PASS and frozen only if:

1. every T0-T17 identifier is explicitly accounted for;
2. all required native tests pass;
3. the Windows verification script passes;
4. T7-T9 remain supported by the already-frozen bounded Windows evidence under their recorded limitations;
5. no special RUN-001 authorization or actuation logic was introduced;
6. no upstream system required modification;
7. no protocol semantic was reinterpreted to obtain PASS;
8. evidence artifacts are preserved and hash-anchored;
9. the final evidence map and report agree with the machine test matrix;
10. the repository remains experimental, downstream, non-canonical, and not production-ready.

## 12. Stop Conditions

RUN-001 SHALL stop without freeze if any of the following occurs:

- an upstream modification is required;
- authorization semantics must be changed to obtain PASS;
- producer-controlled input can independently exercise protected authority within the tested boundary;
- request mutation is accepted despite broken authorization binding;
- authorization replay produces an additional allowed actuation attempt;
- test evidence cannot be mapped unambiguously to T0-T17;
- new infrastructure such as MCP, WASM, network services, PKI, hosted APIs, or policy engines becomes necessary merely to satisfy the initial experiment;
- evidence files cannot be deterministically identified and hash-anchored.

A stopped RUN-001 SHALL NOT set completion or freeze state to true.

## 13. State Transition After Successful Freeze

Only after RUN-001 passes and its evidence package is frozen may the repository evidence state be updated to reflect:

```text
final_integrated_t0_t17_complete = true
run_001_frozen = true
```

The following SHALL remain unchanged:

```text
production_ready = false
canonical_authority = false
classification = experimental / downstream / non-canonical
```

Distribution, MCP integration, package publication, and other deferred surfaces remain outside RUN-001.

## 14. Interpretation Boundary

A successful RUN-001 establishes only the bounded claim defined by the v0.1 experiment and its recorded evidence.

It does not establish:

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

## 15. Working Principle

> Evidence before expansion. Preserve the core. Test the boundary. Freeze completed evidence.