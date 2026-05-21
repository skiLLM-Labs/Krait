---
name: Bug Report / Compiler Error
about: Report an issue with the compiler, runtime, or tools (e.g., krait fmt).
title: "[BUG]"
labels: ''
assignees: ''

---

### Prerequisites
- [ ] I have verified that this issue does not already exist in the closed or open issues.
- [ ] This is **not** a Pull Request. I understand that direct PRs will be automatically closed.

### System Environment
- **Krait Version (`krait --version`):** e.g., 1.0.0 (commit 3d11909)
- **Operating System / OS Version:** e.g., Ubuntu 24.04 / Windows 11
- **Host Toolchain / Clang Version:** e.g., Clang 18.1.0 (Required for `krait build`)

### Minimal Reproducible Example
Provide the absolute smallest chunk of `.kr` code that triggers the bug. No external file dependencies if possible.

```python
# Paste your minimal Krait code here

```

### Expected vs. Actual Behaviour

**Expected:** What should the compiler or runtime have done?
**Actual:** What did it actually do? Paste any error outputs or logs here.

```text
# Paste compiler diagnostics or panic logs here

```

### Additional Context / Proposed Fix (Optional)

If you have analyzed the issue locally on your fork or have a theory on where the AST parser / LLVM backend is falling over, share your findings here!

```
