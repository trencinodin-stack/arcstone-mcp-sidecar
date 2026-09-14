# Arcstone Execution Boundary — Distribution Gate

**Repository identity:** `arcstone-mcp-sidecar`
**Implementation package:** `arcstone-execution-boundary`
**Version:** `0.1.0`
**Classification:** Experimental / Downstream / Non-Canonical
**Gate:** Distribution
**Status:** Pre-Validation

## Purpose

This document defines the prospective distribution contract for the Arcstone Execution Boundary v0.1.

The Distribution Gate asks one bounded question:

> Can an independent user obtain, verify, and execute the frozen v0.1 implementation through a public distribution surface without introducing an Arcstone-operated runtime dependency or changing the already-validated execution semantics?

Distribution does not expand the scientific or execution claims of the repository.

## Preserved baseline

The Distribution Gate MUST preserve:

- the frozen RUN-001 evidence package
- the validated T0–T17 execution behavior
- the validated CLI interface contract
- the existing authorization lifecycle
- the existing gate decision semantics
- the existing actuation semantics
- the Experimental / Downstream / Non-Canonical classification

Distribution work MUST NOT reinterpret or modify frozen upstream authority.

## Initial distribution surface

The initial v0.1 distribution surface is:

1. public GitHub repository
2. GitHub Release
3. source distribution
4. explicitly supported release binaries
5. SHA-256 checksums for release assets
6. Apache-2.0 license

GitHub is a distribution convenience. It is not a required runtime authority or service.

## Release identity

The following identities MUST agree before release:

```text
Cargo package version: 0.1.0
CLI version:           0.1.0
Git tag:               v0.1.0
GitHub Release:        v0.1.0
```

No `v0.1.0` tag or public release may be created until all pre-release Distribution Gate criteria, except public cold-start validation, have been satisfied. The tag and public release are evidence-producing steps within this gate. The Distribution Gate closes only after the public release exists and cold-start validation against that public-facing surface has passed.

## Binary artifacts

A binary artifact may be advertised only for a platform that is explicitly built and tested by the release process.

The release process MUST NOT imply support for an untested platform.

Each distributed binary artifact MUST have a corresponding SHA-256 digest in the release checksum manifest.

## Source verification

The distributed source MUST remain independently buildable and testable using the documented Rust toolchain path.

The existing verification surface MUST remain available, including:

```text
cargo test --all-targets
```

Windows-specific authority claims remain bounded by the frozen Windows authority-boundary evidence and MUST NOT be generalized by the distribution process.

## Runtime independence

Once a release has been obtained, core execution MUST NOT require:

- an Arcstone-operated service
- an Arcstone account
- an Arcstone API
- an Arcstone network endpoint
- an Arcstone database
- an Arcstone license server
- a hosted Arcstone model
- continuing intervention by Arcstone

Distribution infrastructure is not runtime authority.

Loss of a distribution channel after acquisition MUST NOT invalidate an already-obtained executable artifact.

## Licensing

The v0.1 public distribution uses the SPDX license identifier:

```text
Apache-2.0
```

The repository MUST contain the corresponding license text before public release.

Licensing does not change the Experimental / Downstream / Non-Canonical classification.

## Deferred distribution surfaces

The following are not required to close the initial v0.1 Distribution Gate:

- crates.io publication
- npm publication
- MCP Registry publication
- Docker images
- Kubernetes deployment
- WASM distribution
- hosted APIs
- package-manager-specific wrappers
- generalized installers
- automatic update services

These may be evaluated independently after the initial distribution surface is proven.

## Cold-start validation

Before the Distribution Gate closes, a cold-start verification MUST demonstrate that an independent user can:

1. identify the artifact and version
2. identify its Experimental / Downstream / Non-Canonical status
3. obtain the release
4. verify the release checksum
5. invoke `--help`
6. invoke `--version`
7. execute the documented verification path
8. understand the bounded claims and explicit non-claims

Cold-start validation MUST use the public-facing distribution surface rather than undocumented local knowledge.

## Repository visibility

The repository SHOULD remain private while the distribution surface is being prepared and audited.

Public visibility is a release action, not a prerequisite for constructing or testing the distribution machinery.

## Gate closure criteria

The Distribution Gate may close only when all of the following are established:

```text
license_present                    = true
release_identity_consistent        = true
source_distribution_available      = true
claimed_binaries_built             = true
claimed_binaries_tested            = true
release_checksums_present          = true
cli_contract_preserved             = true
execution_semantics_preserved      = true
cold_start_validation_passed       = true
arcstone_runtime_dependency        = false
production_ready                   = false
canonical_authority                = false
```

## Stop conditions

Stop distribution work and reopen review if:

- distribution requires modification of frozen execution semantics
- release packaging changes authorization behavior
- release packaging changes gate decision behavior
- release packaging changes actuation behavior
- a claimed binary cannot be independently verified
- version identity diverges across release surfaces
- an Arcstone-operated runtime dependency becomes mandatory
- closing the gate would require claiming production readiness or canonical authority

## Working principle

> **Evidence before expansion. Preserve the core. Distribute only what has been proven.**
