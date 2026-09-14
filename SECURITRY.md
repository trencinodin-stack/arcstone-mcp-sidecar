# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability or execution-boundary issue in `arcstone-mcp-sidecar`, please do not open a public issue.

You can submit reports through either of the following private channels:

1. **GitHub Private Vulnerability Reporting (Recommended):**
   * Go to the **Security** tab of this repository.
   * Click **Report a vulnerability**.
   * Fill out the form to submit a private report directly to maintainers.

2. **Email:**
   * Contact us at **`security@arcstoneos.com`**.

### What to Include

* Description of the authorization, gate, actuator, protected-resource, or execution-boundary bypass.
* Minimal reproduction steps or proof-of-concept.
* The affected version or commit, if known.

### Experimental Scope

`arcstone-mcp-sidecar` is an experimental, downstream, non-canonical reference implementation. Its security claims are limited to the explicitly tested execution-boundary conditions and preserved evidence.

It does not claim production-grade authorization security, arbitrary-code containment, remote-attacker resistance, universal sandbox or kernel security, or general AI safety.

We will acknowledge receipt within 48 hours and work with you on an appropriate resolution timeline.
