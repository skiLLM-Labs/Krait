<<<<<<< HEAD
<div align="center">

<img height="150" alt="Krait" src="https://github.com/user-attachments/assets/2e7b97cc-b082-4762-8b2d-8f0358797417" />
=======
# Krait (v1.0.0)
>>>>>>> 3d11909 (Release: v1.0.0)

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![Version](https://img.shields.io/badge/version-1.0.0-blue)](#)
[![License](https://img.shields.io/badge/license-MIT-purple)](#)

Krait is a fast, memory-safe, and highly productive systems programming language.

Designed under the philosophy **"Reduce cognitive noise without reducing power,"** Krait combines the elegant, indentation-based readability of Python with the raw execution speed and control of C and Rust. It compiles directly to native machine code via a highly optimized LLVM-IR backend.

</div>

## ⚡ What Makes Krait Different?

### 🦀 Rust-like Memory Safety (Zero-Cost Abstractions)
Krait does not use a garbage collector. Instead, it implements a strict **compile-time Ownership Memory Model**:
- **Move Semantics**: Assigning a heap variable (`set a = b`) transfers ownership. The compiler invalidates `b` at compile time to prevent double-free bugs.
- **Auto-Drop**: When a variable goes out of scope, the compiler automatically generates a native `@free` invocation to deallocate heap memory, guaranteeing zero memory leaks.

### 🤖 LLM-Friendly & Vibe-Coder Ready
Krait is designed from the ground up for modern AI-assisted engineering and automated workflows:
- **Actionable Compiler Diagnostics**: Errors are printed as beautifully formatted, descriptive blocks explaining exactly *why* the bug occurred and presenting multiple concrete code resolutions. LLMs read these diagnostics and can fix code immediately.
- **Standard Code Formatter (`krait fmt`)**: Standardizes style and indentation instantly, ensuring generated and human code always align perfectly.

### 🚀 True Native Speed for All Hardware
Krait links directly to your native system toolchain with optimal compile-time performance passes:
- **Optimal Optimization**: Automatically passes `-O3` and `-flto` (Link-Time Optimization) flags to LLVM/Clang.
- **Broad Hardware Compatibility**: Explicitly compiles with `-march=x86-64-v2`, making executables blazingly fast and compatible on high-end developer rigs as well as legacy and energy-efficient systems (e.g. Intel Celeron, 10th Gen Core i3 laptops).

<<<<<<< HEAD
## Installation (No Rust Required)
=======
---

## ✨ Features

- **Clean Pythonic Syntax**: Indentation-based blocks, minimal punctuation, and low cognitive noise.
- **Modular Imports**: Split code bases into clean modules and reuse them with recursive `import` statements.
- **Built-in Package Scaffolding**: Scaffold new structures instantly using `krait new`.
- **Stand-alone REPL (IDLE)**: An interactive shell for fast prototyping and instant calculation.
- **Stand-alone Interpreter**: Run `.kr` scripts instantly via `krait run` without needing a compilation toolchain.
- **VS Code Extension**: Premium, built-in colorizer support.

---

## 💾 Installation
>>>>>>> 3d11909 (Release: v1.0.0)

You do not need Cargo or Rust to run Krait. Install the pre-compiled binary directly to your system:

### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/skiLLM-Labs/Krait/refs/heads/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/skiLLM-Labs/Krait/refs/heads/main/install.ps1 | iex
```

> **Note for Native Compilation:** While the Krait Interpreter (`krait run`) is completely standalone, using the `krait build` command to generate optimized hardware executables requires `clang` to be installed on your system path.

<<<<<<< HEAD
## Getting Started
=======
---

## 🏁 Getting Started
>>>>>>> 3d11909 (Release: v1.0.0)

### 1. Interactive Shell (REPL)
Launch the interactive environment:
```bash
$ krait
Krait 1.0.0 Interactive Shell
Type 'exit' to quit.

>>> set greeting = "Hello, Krait!"
>>> show greeting
"Hello, Krait!"
```

### 2. Scaffold a New Project
```bash
krait new my_app
cd my_app
krait run src/main.kr
```

### 3. Compiling Natively
Compile a script to a highly optimized hardware executable:
```bash
krait build examples/calculator.kr
./calculator
```

<<<<<<< HEAD
## Syntax Example
=======
---

## 🎨 Syntax Spotlight
>>>>>>> 3d11909 (Release: v1.0.0)

Here is an example demonstrating Krait's clean syntax, standard imports, FFI functions, and recursive functions:

```python
# examples/test_math.kr
import math

# abs(-42) -> 42
set a = abs(0 - 42)
show a

# power(2, 10) -> 1024
set p = power(2, 10)
show p
```

<<<<<<< HEAD
## VS Code Extension

To enable official syntax highlighting in VS Code:
1. Open the `vscode-krait` directory.
2. Copy the folder to your VS Code extensions directory:
   - **Windows:** `%USERPROFILE%\.vscode\extensions\`
   - **Mac/Linux:** `~/.vscode/extensions/`
3. Restart VS Code. Your `.kr` files will now be properly colorized!

## Architecture & Roadmap
=======
And check out our beautiful calculator implementation in [examples/calculator.kr](file:///workspaces/Mantis/examples/calculator.kr)!

---

## 🛠️ Architecture & Roadmap
>>>>>>> 3d11909 (Release: v1.0.0)

With the release of **v1.0.0**, Krait's roadmap is fully realized:

<<<<<<< HEAD
- [x] **Phase 1: Prototype** (Lexer, Parser, AST, Basic Interpreter)
- [x] **Phase 2: Static Engine** (LLVM Text Generation, Native Dynamic Linking, Advanced Type Deduction)
- [x] **Phase 3: Tooling** (REPL, Project Scaffolding, CLI, VS Code Extension)
- [ ] **Phase 4: Systems Optimization** (Custom Borrow Checker, Concurrency/Channels, Package Registry)
=======
- [x] **Phase 1: Prototype Engine** (Lexer, Parser, AST, Basic Tree-Walking Interpreter)
- [x] **Phase 2: Compiler Backend** (LLVM Text IR Generation, Native Linking, Advanced Type Inference)
- [x] **Phase 3: Ownership & Auto-Drop** (Compile-time Move analysis, Scope-based stack deallocations)
- [x] **Phase 4: Ecosystem & Tooling** (REPL, Project Manager, Actionable Diagnostics, standard `math`/`io` libraries, `krait fmt`)

---

## 🤝 Contributing
Krait is open-source and built in Rust. To compile the engine from source:
```bash
git clone https://github.com/skiLLM-Labs/Krait.git
cd Krait
cargo build --release
cargo install --path .
```

## 📄 License
MIT License. See `LICENSE` for details.
>>>>>>> 3d11909 (Release: v1.0.0)
