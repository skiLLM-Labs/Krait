# Krait v1.0.0

This major release marks the transition of Krait into a production-ready systems programming language. Krait v1.0.0 brings advanced type safety, standard library modularity, a Rust-like compile-time ownership memory model, loop optimizations, and extensive developer tools.

### Added
- **Rust-like Ownership Memory Model**:
  - Implemented static **Move Semantics** analysis in semantic checks to prevent double-free and use-after-free vulnerabilities.
  - Implemented **Auto-Drop** generation in the compiler: scope variables of struct pointer types are automatically deallocated via native `@free` calls when they exit their parent block.
- **Native Loop Generation (`repeat`)**:
  - Added optimal loop code generation in the LLVM backend.
  - Implemented **Duplicate Alloca Elimination** in the register allocator to support clean variable reassignment inside nested scopes.
- **LLM-Friendly & Actionable Compiler Diagnostics**:
  - Replaced standard error strings in the semantic analyzer with beautifully formatted, colored diagnostic blocks outlining the specific error type, a detailed explanation of why it occurred, and clear options to resolve them.
- **Module System & Import Resolution**:
  - Added support for the `import` statement in the lexer, parser, CLI, and interpreter.
  - Implemented recursive import resolution for resolving standard and local modules dynamically.
- **Standard Library Modules**:
  - Exposed standard input/output FFI functions (`putchar`) in `lib/io.kr`.
  - Added pure Krait standard operations (`abs`, loop-based `power`) in `lib/math.kr`.
- **Formatting CLI Subcommand**:
  - Exposed the standard formatting library directly in the CLI with `krait fmt <file>` and `krait format <file>` commands.

### Changed
- **Compiler backend optimization**:
  - Configured compiler to pass `-O3` (Maximum optimization), `-flto` (Link-Time Optimization), and `-march=x86-64-v2` hardware instruction optimizations to Clang/LLVM.
- **Interpreter block scoping**:
  - Patched the tree-walking interpreter to correctly propagate variable changes back to outer scopes while keeping block-local variables isolated.

# Krait v0.1.0

v0.1.0 was the initial prototype version of krait and was not released on Github as binaries for the general public to download.

### Added
- Standard Python-like lexer and parser.
- Primitive types (Int, Float, Str, Bool, Void) and basic structures.
- Standalone tree-walking interpreter.
- Standalone REPL shell.
- Package scaffolding system (`krait new`).
- Core LLVM-IR generation code.
