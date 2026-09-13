# Windows Authority Boundary — v0.1 Evidence Record

**Repository:** `arcstone-mcp-sidecar`
**System:** Arcstone Execution Boundary
**Version:** 0.1.0
**Classification:** Experimental / Downstream / Non-Canonical
**Experiment status:** PASS — bounded Windows authority-boundary experiment complete

## Purpose

The application code alone cannot prove the producer-to-resource authority boundary when every process runs under the same Windows principal.

This experiment tested a bounded Windows configuration using separate producer and authority principals, filesystem ACL separation, the existing Arcstone Execution Boundary implementation, and preserved replay evidence.

The experiment targeted T7, T8, T9, I2, and the bounded Windows realization of I3.

## Tested principals

Two ordinary, non-administrator Windows principals were used:

```text
ARCSTONE\ArcstoneProducer
ARCSTONE\ArcstoneAuthority

ArcstoneProducer represented the untrusted producer.

ArcstoneAuthority represented the authority permitted to issue authorization and execute the boundary against the protected runtime.

The authority session was independently checked as non-elevated. An attempted net session returned Access Denied.

Runtime

A dedicated disposable runtime was used:

C:\Arcstone-EB-Experiment

Relevant runtime areas were:

auth\issued
auth\consumed
protected
evidence

The experiment did not modify ACLs on the source repository, user profile, or OneDrive tree.

ACL boundary

The experiment removed the runtime's inherited Authenticated Users Modify permission and established the tested authority separation.

The effective intended relationship was:

ArcstoneAuthority  -> Modify
ArcstoneProducer   -> Read & Execute
SYSTEM             -> Full Control
Administrators     -> Full Control

The producer therefore did not have filesystem write authority over the authorization stores or protected target under the tested configuration.

The exact ACL output is preserved in the frozen evidence package.

T7 — direct protected-resource bypass

Result: PASS

A fresh protected-resource probe path was confirmed absent before the attempt.

Running as ARCSTONE\ArcstoneProducer, direct creation of the protected probe produced:

Access is denied.
T7 PASS - WRITE BLOCKED

The probe did not appear after the attempted write.

This establishes that the tested producer principal could not directly create the tested protected resource under the preserved ACL configuration.

T8 — direct actuator bypass

Result: PASS — bounded

The production actuator module is not exposed through the crate's public Rust API.

An external Rust consumer attempted:

use arcstone_execution_boundary::actuator::FileActuator;

Compilation failed with Rust error E0603 because module actuator is private.

The CLI likewise exposes no direct actuator command. Production actuation is reached through the gate path.

Combined with T7, the tested producer had neither a public Rust actuator entry point nor direct filesystem write authority to the tested protected resource.

This is a bounded structural/API result. It is not a claim that no conceivable Windows or same-crate bypass exists.

T9 — forged authorization issuance

Result: PASS

A fresh forged-authorization probe path was confirmed absent before the attempt.

Running as ARCSTONE\ArcstoneProducer, direct creation in auth\issued produced:

Access is denied.
T9 PASS - FORGED ISSUANCE BLOCKED

The forged authorization probe did not appear after the attempt.

This establishes that the tested producer principal could not directly create authorization state under the preserved ACL configuration.

Authorized execution

The authority issued:

authorization_id: AUTH-OS-001
action: WRITE_PROTECTED_FILE
resource_id: EFFECT_LOG
payload_hex: 48454C4C4F

The payload decodes to:

HELLO

Its SHA-256 was:

3733cd977ff8eb18b987357e22ced99f46097f31ecb239e878ae63760e83e4d5

Before execution, AUTH-OS-001 was observed in the Issued state.

The matching request was then executed through the boundary as the authority principal.

Observed result:

decision: ALLOW
authorization_state_before: ISSUED
authorization_state_after: CONSUMED
actuation: SUCCEEDED
effect_present_after: true
effect_sha256_after: 3733cd977ff8eb18b987357e22ced99f46097f31ecb239e878ae63760e83e4d5

Independent inspection confirmed:

protected\effect.bin = HELLO
AUTH-OS-001 = Consumed
Replay

The identical authorized request was submitted again with a new run identifier.

Observed replay result:

decision: DENY
deny_reason: ConsumedAuthorization
authorization_state_before: CONSUMED
authorization_state_after: CONSUMED
actuation: NOT_ATTEMPTED
effect_present_after: true
effect_sha256_after: 3733cd977ff8eb18b987357e22ced99f46097f31ecb239e878ae63760e83e4d5

The replay therefore did not receive a second actuation opportunity.

For this tested case, the evidence supports the single-use / at-most-once authority invariant I2.

Request hashes

The preserved raw request file has its own file SHA-256:

1b05150311852110f7e52c02a2dfbb75f829dc106c9436ecffa14ee8c2eea045

The boundary-generated evidence records the protocol-level request SHA-256:

39580fbbfb1eb5486d04501587e2b1e9121954190f428666b85b0c0d73e748e0

These identify different representations and must not be conflated: the first hashes the preserved request file artifact, while the second is the request hash recorded by the execution protocol.

Harness correction

During the experiment, the original Windows producer-access harness exposed an observation defect.

CMD redirection failure correctly produced Access is denied, but relying on ERRORLEVEL after the failed redirection could incorrectly classify the result.

The harness was corrected to evaluate the actual filesystem postcondition using if exist and to reject ambiguous tests when probe targets already exist.

This was a test-harness defect, not an authority-boundary failure.

The correction is preserved separately in repository history.

Evidence provenance

The producer-t7-t9.txt transcript was generated under the ArcstoneProducer principal and then copied into the trusted evidence directory by the Administrator for preservation because the Authority principal could not read the Producer principal's private temporary directory.

The runtime ACLs were not weakened for this transfer.

This administrative preservation step did not grant the Producer or Authority additional runtime permissions and does not alter the tested producer/authority separation.

Evidence package

The frozen evidence package is:

WINDOWS-AUTHORITY-BOUNDARY-FROZEN.zip

Frozen ZIP SHA-256:

e8beae870214d1e53cf4f7930341b79ffdb7ac884e563cac2c52ef1d8818ccaa

The evidence package contains a SHA256SUMS.txt manifest covering the preserved experiment artifacts.

SHA-256 of SHA256SUMS.txt:

445fb4bdebaff50a429b48a1fd0b70f30848320e2e7ec79d81eadb4b2f9893b3

The frozen package preserves the tested ACLs, identities, producer denial transcript, authorization record, consumed claim, request, successful execution summary, replay summary, protected effect, T8 compile-failure evidence, implementation environment, corrected harness, and binary used for the experiment.

The ZIP hash anchors this exact evidence snapshot. It is not a claim that recreating the ZIP will produce byte-identical archive metadata.

Determination

PASS — bounded Windows authority-boundary experiment complete.

The evidence supports the following bounded findings:

T7 passed for the tested Windows producer principal, ACL configuration, and protected path.
T8 passed as a bounded public-API/structural test and in combination with the tested filesystem authority separation.
T9 passed for the tested producer principal and authorization-store path.
I2 is supported for the tested authorization: the first valid execution consumed the authorization and the replay was denied before actuation.
I3 is supported for the tested Windows configuration: the producer principal did not possess the tested protected-resource or authorization-store write authority.
Limits of the claim

This experiment does not establish general Windows sandbox security.

It does not establish:

arbitrary hostile-code containment;
process isolation against the trusted Authority principal;
universal Windows bypass resistance;
production authentication or network security;
distributed exactly-once execution;
cryptographic capability security;
MCP or WASM isolation.

The Authority principal intentionally possessed Modify rights within the tested runtime. It could therefore modify protected state outside the boundary binary. The experiment establishes producer/authority principal separation under the tested ACL configuration; it does not establish that arcstone-exec.exe is the only process the Authority principal can use.

T8 establishes that an external Rust consumer cannot import the production actuator through the crate's public API. It does not establish that malicious code compiled inside the crate could never reach internal implementation surfaces.

The authorized request in this experiment was prepared within the authority-side workflow. This experiment therefore does not independently demonstrate a live producer-to-boundary transport. External nondeterministic producer serialization is separately demonstrated by the Arcstone Path A Ingress Lab and must not be conflated with this authority-boundary experiment.

No MCP, WASM, network transport, PKI, service-account architecture, generalized actuator, or additional execution layer is required by this result.

Freeze

This experiment is complete.

The Windows authority-boundary evidence should remain frozen unless a future experiment identifies a specific defect or a new scientific question requires additional evidence.

Evidence before expansion. Preserve core. Test boundary. Freeze evidence.