# Contributing to Krait

First off, thank you for checking out Krait! As a fast, memory-safe systems programming language, keeping the core engine robust, stable, and highly optimized via our LLVM backend is our top priority.

To maintain strict architectural control over the compiler, type inference engine, and memory ownership model, we follow a specific contribution workflow.

## Pull Requests Are Not Accepted

> **CRITICAL:** Please do not open Pull Requests (PRs) against this repository.

All direct code contributions, patches, documentation updates, and feature implementations are handled exclusively by the core Krait maintainers. **Any unauthorized Pull Request opened against this repository will be closed automatically without review.**

## How to Contribute: Issue-Only Model

We welcome community feedback, bug hunting, and design discussions! If you want to contribute to the evolution of Krait, you can do so entirely through the **GitHub Issues** tab.

### 1. Reporting Bugs & Compiler Regressions

Found an edge-case memory leak, a false-positive ownership error, or an issue with `krait fmt`? Let us know.

* **Check existing issues:** Ensure someone else hasn't already reported the bug.
* **Provide an isolated snippet:** Provide the absolute minimum amount of `.kr` code required to reproduce the error.
* **Include environment details:** Tell us your OS, Krait version (`krait --version`), and host toolchain architecture (e.g., `clang` version, CPU type).
* **Paste the compiler output:** Include the full stack trace or the actionable diagnostic block printed by the compiler.

### 2. Feature Requests & Language Design Proposals

Krait is designed under a strict philosophy: *"Reduce cognitive noise without reducing power."* If you want to propose syntax enhancements, new features, or updates to the standard `math`/`io` libraries:

* **Explain the "Why":** What developer pain point does this feature solve?
* **Show Syntax Examples:** Provide a mock code snippet showing how the proposed syntax looks and behaves.
* **Address the Complexity:** How does this proposal impact compile-time memory checks or the current AST parser layout?

## Building & Testing Locally (For Forks)

If you have forked Krait to experiment with the codebase locally, you can compile and verify the Rust-based engine using the following workflow:

```bash
# Clone your personal fork
git clone https://github.com/YOUR-USERNAME/Krait.git
cd Krait

# Run the test suite to verify internal engine logic
cargo test

# Compile an optimized production binary locally
cargo build --release

# Install the updated engine to your path to test end-to-end execution
cargo install --path .

```

If your local experimentation successfully resolves a bug or brings a great feature to life, please write up a detailed overview of your architecture and findings inside a **GitHub Issue** so the core maintainers can review the approach and merge it into the main distribution line manually.
