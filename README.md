# Krait

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)](#)
[![License](https://img.shields.io/badge/license-MIT-purple)](#)

Krait is a fast, minimal, and memory-safe compiled systems programming language. 

Designed with the philosophy *"Reduce cognitive noise without reducing power,"* Krait combines the readability and low punctuation density of Python with the raw execution speed of C. It compiles directly to native hardware machine code via an LLVM-IR backend.

---

## Features

- **Visually Clean Syntax:** Indentation-based, minimal keywords, zero syntax soup.
- **True Native Compilation:** Compiles directly to C-ABI compliant native executables (`.exe`, ELF) using a Clang/LLVM toolchain.
- **Built-in REPL:** An interactive shell (IDLE) for instant script evaluation.
- **Zero-Dependency Interpreter:** Run `.kr` scripts instantly via the built-in tree-walking interpreter without compiling.
- **Built-in Package Manager:** Scaffold production projects instantly with `krait new`.
- **VS Code Support:** Official syntax highlighting extension included.

---

## Installation (No Rust Required)

You do not need Cargo or Rust to run Krait. Install the pre-compiled binary directly to your system:

### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/skiLLM-Labs/Krait/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/skiLLM-Labs/Krait/main/install.ps1 | iex
```

> **Note for Native Compilation:** While the Krait Interpreter runs completely standalone, using the `krait build` command to generate standalone native executables requires `clang` to be installed on your system path.

---

## Getting Started

### 1. The Interactive Shell (REPL)
Launch the interactive environment by typing `krait` with no arguments:
```bash
$ krait
Krait 0.1.0 Interactive Shell
Type 'exit' to quit.

>>> set greeting = "Hello, World!"
>>> show greeting
"Hello, World!"
```

### 2. Managing Projects
Create a new project structure automatically:
```bash
krait new my_backend
cd my_backend
krait run src/main.kr
```

### 3. Compiling to Machine Code
Translate your Krait script directly into a highly optimized hardware executable:
```bash
krait build examples/fibonacci.kr
./fibonacci
```

---

## Syntax Example

Krait code is designed to be easily scanned by the eyes. Here is a standard recursive Fibonacci sequence:

```python
# examples/fibonacci.kr

make fib(n)
    when n < 2
        return n
    return fib(n - 1) + fib(n - 2)

set limit = 10
set result = fib(limit)

show "The result is:"
show result
```

---

## VS Code Extension

To enable official syntax highlighting in VS Code:
1. Open the `vscode-krait` directory.
2. Copy the folder to your VS Code extensions directory:
   - **Windows:** `%USERPROFILE%\.vscode\extensions\`
   - **Mac/Linux:** `~/.vscode/extensions/`
3. Restart VS Code. Your `.kr` files will now be properly colorized!

---

## Architecture & Roadmap

Krait is actively moving through its developmental phases:

- [x] **Phase 1: Prototype** (Lexer, Parser, AST, Basic Interpreter)
- [x] **Phase 2: Static Engine** (LLVM Text Generation, Native Dynamic Linking, Advanced Type Deduction)
- [x] **Phase 3: Tooling** (REPL, Project Scaffolding, CLI, VS Code Extension)
- [ ] **Phase 4: Systems Optimization** (Custom Borrow Checker, Concurrency/Channels, Package Registry)

---

## Contributing
Krait is open-source and built in Rust. To build the compiler from source:
```bash
git clone https://github.com/skiLLM-Labs/Krait.git
cd Krait
cargo build --release
cargo install --path .
```
