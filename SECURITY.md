# Security Policy

We take the security, stability, and memory-safety invariants of the Krait programming language seriously. If you discover a vulnerability or a bypass in our compile-time ownership model, we appreciate your help in reporting it responsibly.

## Supported Versions

Security fixes are actively backported to the current stable release branch. We do not officially patch older minor or major versions unless a vulnerability poses an extraordinary risk.

| Version | Supported |
| --- | --- |
| v1.0.x | Stable Release) |
| < v1.0.0 | ❌ (End of Life) |

## What Constitutes a Security Vulnerability?

For a systems language like Krait, we classify the following as security vulnerabilities:

* **Memory Safety Violations:** Any compiled `.kr` code that successfully bypasses the compile-time Ownership Memory Model to cause an untracked double-free, use-after-free, or data race at runtime without throwing a compiler error.
* **Compiler Exploits:** Maliciously crafted source files that cause the AST parser, type inference engine, or LLVM-IR generator to execute arbitrary code on the host developer machine during a `krait build` or `krait run`.
* **Supply Chain Risks:** Vulnerabilities within the native pre-compiled distribution scripts (`install.sh` / `install.ps1`) or standard library modules (`math`, `io`).

## Reporting a Vulnerability

> ⚠️ **CRITICAL:** Please do **NOT** open a public GitHub Issue for security vulnerabilities.

To give us time to investigate and patch the vulnerability before it can be exploited maliciously, please follow our coordinated disclosure process:

1. **Do Not Publicly Disclose:** Keep the details confidential until we have published a patch.
2. **Submit via Email:** Send a detailed report to **[Insert Your Security Email, e.g., security@skillm-labs.com]** or use the GitHub Private Vulnerability Reporting feature if enabled on this repository.
3. **What to Include:**
* A clear description of the vulnerability.
* A minimal, reproducible `.kr` proof-of-concept (PoC) script.
* The impact (e.g., local code execution, memory corruption).

## Our Process

* **Acknowledgement:** We will acknowledge receipt of your vulnerability report within **48 hours**.
* **Investigation:** The core maintainers will attempt to reproduce the issue and determine its root cause in the compiler engine.
* **Patching & Release:** Once fixed, we will release a coordinated patch update (e.g., `v1.0.1`) and publicly credit you for the discovery if you want (unless you prefer to remain anonymous).
