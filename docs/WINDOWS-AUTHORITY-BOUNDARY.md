# Windows Authority Boundary — v0.1 Evidence Requirement

The application code alone cannot prove I3 when every process runs under the same Windows principal.

The final v0.1 evidence run therefore needs a bounded Windows authority configuration with at least two principals:

```text
producer principal
authority principal
```

The producer principal must be unable to:

- write `<root>\auth\issued`;
- write `<root>\auth\consumed`;
- write or create `<root>\protected\effect.bin`.

The authority principal must be able to perform those operations as required by the experiment.

## Why this is separate from the Rust tests

If the producer has the same filesystem authority as the boundary, it can simply ignore the gate and mutate the protected file itself. In that environment the gate is advisory.

This is not repaired by adding HMAC, MCP, WASM, or more application code. The authority boundary must be real.

## Windows tools

Windows ACLs can be inspected and configured with `icacls`.

Do **not** run broad ACL changes against personal directories. Use a dedicated disposable experiment directory.

A separate producer account/session should run:

```cmd
scripts\windows\check-producer-access.cmd C:\path\to\runtime
```

The script must report denial for both direct protected-resource mutation and authorization-store creation.

## Evidence to preserve

For the final run preserve:

- `whoami` for producer and authority principals;
- `icacls` output for runtime, auth, and protected directories;
- direct producer write attempt transcript;
- forged-issuance attempt transcript;
- protected target pre/post state;
- authorization store pre/post state.

Do not claim general Windows sandbox security. The claim is limited to the tested principals, ACLs, target directory, and tested bypass routes.
