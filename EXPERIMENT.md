# Arcstone Execution Boundary v0.1 Experiment

## Research question

Can a deterministic downstream execution boundary prevent an untrusted or nondeterministic producer from causing a protected side effect unless an explicit, boundary-controlled authorization condition has been satisfied?

## Primary invariants

### I1 — Non-Authorization Safety

```text
NO VALID AUTHORIZATION
        ⇒
ZERO PROTECTED ACTUATION
I2 — Single-Use Authority
Plaintext
ONE VALID SINGLE-USE AUTHORIZATION
        ⇒
AT MOST ONE PROTECTED ACTUATION
I3 — Exclusive Actuation Authority
Plaintext
UNTRUSTED PRODUCER
        ⇏
PROTECTED RESOURCE
State separation
Plaintext
authorization = ABSENT | INVALID | ISSUED | CONSUMED
decision      = ALLOW | DENY(reason)
actuation     = NOT_ATTEMPTED | SUCCEEDED | FAILED
effect        = absent | exact protected bytes
Consumption semantics
v0.1 uses at-most-once authorization semantics:

Plaintext
ISSUED
   ↓
atomic claim / consume
   ↓
CONSUMED
   ↓
actuation attempt
   ├── SUCCEEDED
   └── FAILED
If actuation fails after a successful claim, the authorization remains consumed.

Exactly-once execution is not claimed.

Adversarial matrix
T0 valid exact authorization

T1 no authorization

T2 invalid / invented authorization

T3 payload mutation

T4 resource substitution

T5 action substitution

T6 replay after consumption

T7 direct protected-resource bypass

T8 direct actuator bypass

T9 forged issuance

T10 issued authorization never submitted

T11 producer identity/provenance change

T12 favorable core observation without authorization

T13 valid authorization with core observation omitted

T14 malformed request

T15 repeated invalid attempts do not consume

T16 authorized request with induced actuator failure

T17 concurrent double submit

Experimental split
T0–T6 and T10–T17 are executable in the native test harness.

T7 and T9 require a real OS authority configuration using separate principals. T8 is structural in the Rust implementation and also depends on the tested authority environment.

The bounded Windows authority-boundary experiment, integrated T0–T17 adversarial matrix, and Execution Boundary Run 001 are completed and frozen under the active v0.1.0 baseline.

The environment-level results were:

T7 direct protected-resource bypass: PASS under the tested ACL configuration

T8 direct actuator bypass: PASS — bounded public-API/structural result

T9 forged authorization issuance: PASS under the tested ACL configuration

I2 single-use / at-most-once authority: supported for the tested authorization

I3 producer-to-protected-resource separation: supported for the tested Windows configuration

The authorized execution transitioned the tested authorization from ISSUED to CONSUMED, performed the protected actuation successfully, and preserved the expected protected effect. Reuse of the same authorization was subsequently denied as consumed before another actuation attempt.

The Windows experiment is complete and frozen as a bounded environment-level result.

See docs/WINDOWS-AUTHORITY-BOUNDARY.md for the detailed evidence record, preserved evidence hashes, test configuration, and limitations.

MCP Registry Baseline & Phase Progression
The v0.1.0 baseline is officially published on the Model Context Protocol (MCP) Registry as io.github.trencinodin-stack/arcstone-mcp-sidecar@0.1.0.

Further phase expansion into WASM runtimes, cryptographic capability grants (HMAC/PKI), network services, or generalized policy engines remains explicitly deferred until required by a new research question.

Interpretation boundary
The completed Windows experiment supports the tested producer/authority separation under the recorded ACL configuration.

It does not establish:

general Windows sandbox security

arbitrary hostile-code containment

process isolation against the trusted authority principal

universal bypass resistance

production authorization or network security

distributed exactly-once execution

cryptographic capability security

WASM security properties

The authority principal retains filesystem authority within the experimental runtime. The result therefore establishes bounded principal-level separation between the tested producer and authority roles; it does not establish that arcstone-exec is the only process the authority principal could use to modify protected state.

T8 establishes that an external Rust consumer cannot import the production actuator through the crate's public API, combined with the tested producer's lack of direct filesystem write authority. It does not establish that no conceivable Windows bypass exists.

Working principle
Evidence before expansion. Preserve the core. Test the boundary. Freeze completed evidence.
