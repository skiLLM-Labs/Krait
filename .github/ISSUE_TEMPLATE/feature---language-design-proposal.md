---
name: Feature / Language Design Proposal
about: Propose syntax changes, standard library expansions, or tooling features.
title: "[PROPOSAL]"
labels: ''
assignees: ''

---

### Core Philosophy Check
Krait's core philosophy is: **"Reduce cognitive noise without reducing power."**
- [ ] My proposal keeps the language clean and pythonic.
- [ ] This proposal does not require adding a garbage collector.

### Problem Statement
Is your feature request related to a problem or limitation? Please describe it clearly. 
*(e.g., "Writing recursive functions for custom algorithms requires too much boilerplate because...")*

### Proposed Solution / New Syntax
Describe the design or syntax you want to see. Provide a concrete code example showing how it would look in a `.kr` file.

```python
# Example of your proposed syntax/feature in action

```

### Impact Analysis

* **How does this affect memory safety?** (Does it alter Move semantics, Auto-Drop, or compile-time lifecycles?)
* **How does this affect tooling?** (Will `krait fmt` or the VS Code colorizer need significant updates?)

### Alternatives Considered

Briefly outline any alternative patterns or workarounds you can currently use in Krait v1.0.0 to achieve a similar result.

```

### How this looks in your repo
Once you push these files to your `main` branch, when anyone clicks **"New Issue"** on your repository, GitHub will present them with a clean UI showing two distinct options:
1. **Bug Report / Compiler Error** (Forces them to give you code snippets and system info)
2. **Feature / Language Design Proposal** (Forces them to defend why their idea fits Krait's minimalist architecture)

```
