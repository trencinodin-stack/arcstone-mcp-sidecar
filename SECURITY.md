# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, execution boundary bypass, atomic claim race condition, or OS authority-separation failure affecting `arcstone-mcp-sidecar` (`arcstone-execution-boundary`), please **do not open a public issue**.

Submit reports through either of the following private channels:

1. **GitHub Private Vulnerability Reporting (Recommended):**
   * Go to the **Security** tab of this repository (`trencinodin-stack/arcstone-mcp-sidecar`).
   * Click **Report a vulnerability**.
   * Fill out the form to submit a private report directly to maintainers.

2. **Email:**
   * Contact us at **security@arcstoneos.com**.

---

## What to Include

Please include, where applicable:
* A description of the vulnerability, authority bypass, or race condition.
* Minimal reproduction steps or test proof-of-concept (`cargo test`).
* The affected commit, CLI flag, or runtime configuration, if known.
* Whether the issue involves:
  * Unauthorized protected actuation or modification of `<root>/protected/effect.bin` without a valid claim;
  * Bypass of atomic claim consumption (`create_new(true)`) allowing replay or double-actuation;
  * Forged authorization issuance in `auth/issued/` or unauthorized mutation of `auth/consumed/`;
  * Discrepancies between requested action/resource parameters and payload bindings;
  * Producer-to-authority principal ACL leakage under Windows or POSIX runtime environments;
  * Divergence from frozen sub-experiment evidence recorded in `docs/WINDOWS-AUTHORITY-BOUNDARY.md`.

---

## Immutability of Frozen Experimental Evidence

The Windows authority-boundary sub-experiment and associated evidence runs are complete and frozen.

* **No Silent Evidence Updates:** Discovering a defect or vulnerability after a freeze does not authorize silent modification, regeneration, or replacement of historical evidence records.
* **Explicit Documentation:** Defects affecting frozen evidence will be documented as explicit annotations. Remediation requiring changes to authority conditions, ACL configurations, or boundary semantics will be executed under a new run or experiment identity.

---

## Bounded Experimental Scope

`arcstone-mcp-sidecar` is an experimental v0.1 downstream realization. Its security claims are bounded strictly to the tested configuration:
* **Target Resource:** Exactly one protected resource (`EFFECT_LOG` mapping to `<root>/protected/effect.bin`).
* **Single-Use Claims:** Atomic file-reservation claims burning payload SHA-256 hashes.

This repository explicitly does **not** claim:
* Production-grade operating system, hypervisor, or kernel sandbox security;
* Universal resistance against arbitrary hostile code or memory-unsafe execution;
* General AI safety, prompt-injection immunity, or agent alignment;
* Production cryptographic key management (PKI/KMS) or remote network security.

---

## Response Timeline

We will acknowledge receipt within **48 hours** and coordinate an appropriate remediation and disclosure timeline.
