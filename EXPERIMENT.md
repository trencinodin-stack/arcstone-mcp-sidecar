# Arcstone Execution Boundary v0.1 Experiment

## Research question

Can a deterministic downstream execution boundary prevent an untrusted or nondeterministic producer from causing a protected side effect unless an explicit, boundary-controlled authorization condition has been satisfied?

## Primary invariants

### I1 — Non-Authorization Safety

```text
NO VALID AUTHORIZATION
        ⇒
ZERO PROTECTED ACTUATION
```

### I2 — Single-Use Authority

```text
ONE VALID SINGLE-USE AUTHORIZATION
        ⇒
AT MOST ONE PROTECTED ACTUATION
```

### I3 — Exclusive Actuation Authority

```text
UNTRUSTED PRODUCER
        ⇏
PROTECTED RESOURCE
```

## State separation

```text
authorization = ABSENT | INVALID | ISSUED | CONSUMED
decision      = ALLOW | DENY(reason)
actuation     = NOT_ATTEMPTED | SUCCEEDED | FAILED
effect        = absent | exact protected bytes
```

## Consumption semantics

v0.1 uses at-most-once authorization semantics:

```text
ISSUED
   ↓
atomic claim / consume
   ↓
CONSUMED
   ↓
actuation attempt
   ├── SUCCEEDED
   └── FAILED
```

If actuation fails after a successful claim, the authorization remains consumed.

Exactly-once execution is not claimed.

## Adversarial matrix

- T0 valid exact authorization
- T1 no authorization
- T2 invalid / invented authorization
- T3 payload mutation
- T4 resource substitution
- T5 action substitution
- T6 replay after consumption
- T7 direct protected-resource bypass
- T8 direct actuator bypass
- T9 forged issuance
- T10 issued authorization never submitted
- T11 producer identity/provenance change
- T12 favorable core observation without authorization
- T13 valid authorization with core observation omitted
- T14 malformed request
- T15 repeated invalid attempts do not consume
- T16 authorized request with induced actuator failure
- T17 concurrent double submit

## Important experimental split

T0–T6, T10–T17 are primarily executable in the native test harness.

T7 and T9 require a real OS authority configuration using separate principals. T8 is partly structural in the Rust implementation and must also be validated in the target authority environment.

The experiment is not complete until the environment-level bypass tests are recorded.
