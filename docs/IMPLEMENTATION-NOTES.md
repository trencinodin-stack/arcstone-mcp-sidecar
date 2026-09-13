# Implementation Notes

## Why `create_new(true)` is used for the claim

The single-host v0.1 experiment needs an atomic "first claimant wins" transition.

The implementation maps:

```text
ISSUED
```

to:

```text
claim marker absent
```

and:

```text
CONSUMED
```

to:

```text
claim marker present
```

The gate first validates action/resource/payload binding and only then tries to create the claim marker with `create_new(true)`.

If another request has already claimed the same authorization, creation fails and the second request is denied.

This is a local implementation choice, not a canonical Arcstone primitive.

## Why the protected path is not producer-selected

`resource_id = EFFECT_LOG` is mapped internally to:

```text
<root>/protected/effect.bin
```

The request cannot supply an arbitrary filesystem path.

This keeps v0.1 resource identity narrow and avoids turning path canonicalization into a new architecture project.

## Why the target uses create-new semantics

Each isolated experimental fixture begins with no protected effect.

The file actuator creates the effect file exactly once. Tests also inspect decision and claim state, so target existence is not used as a substitute for authorization correctness.

## Why the upstream core is optional

The experiment tests downstream authorization. A frozen core observation, when present, is evidence metadata and does not authorize execution.

This makes T12 and T13 possible:

```text
favorable core observation + no authorization → DENY
valid authorization + core omitted            → ALLOW
```

That is deliberate.
