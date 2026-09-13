# Windows Setup

This repository is intended to be easy to execute from Windows CMD and VS Code.

## Required

- Git
- Rust stable toolchain
- Cargo
- Visual Studio Build Tools / MSVC toolchain as required by Rust on Windows
- VS Code optional

## Verify tools

```cmd
rustc --version
cargo --version
git --version
```

## Local verification

From the repository root:

```cmd
cargo test
```

or:

```cmd
scripts\windows\verify.cmd
```

## Important

A successful Rust test run does **not** establish the OS-level exclusive-authority invariant.

The final v0.1 evidence run requires separate Windows principals / ACLs. See:

```text
docs/WINDOWS-AUTHORITY-BOUNDARY.md
```

Do not weaken system security policy merely to make the test easier.
