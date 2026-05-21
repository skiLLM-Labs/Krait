# Welcome to the Krait Wiki!

Krait is a fast, minimal, and memory-safe compiled systems programming language. It is designed to combine the clean syntax and readability of Python with the performance, type safety, and memory-safety guarantees of Rust and C++. 

This wiki is built for both **humans** (developers, systems designers, vibe coders) and **LLMs** (autonomous coding agents, GPTs, Claude models). By structuring this guide with clear rules, type models, and concrete semantics, we enable both engineers and AI models to write, read, and reason about Krait with 100% precision.

---

## 🗺️ Wiki Table of Contents

Explore the following articles to master Krait:

### 1. [Syntax & Types](WIKI_SYNTAX.md)
- Learn Krait's Pythonic formatting rules, blocks, function declarations, type inference, structures, and control flow.

### 2. [Memory Model & Ownership](WIKI_OWNERSHIP.md)
- Dive deep into Krait's compile-time memory model (Move Semantics and Auto-Drop) which prevents garbage collector overhead and runtime memory bugs.

### 3. [Ecosystem & Tooling](WIKI_ECOSYSTEM.md)
- Understand standard CLI commands (`build`, `run`, `check`, `fmt`), modular standard libraries (`math`, `io`), FFI capabilities, and how to program successfully in Krait using LLMs.

---

## 💡 Core Philosophy

> *"Reduce cognitive noise without reducing power."*

Systems languages have historically been cluttered with visual syntax noise (`{}`, `;`, `()`, complex lifetime annotations). Scripting languages achieved readability at the cost of execution speed, predictable resource control, and compile-time type safety.

Krait changes this by utilizing:
1. **Indentation-Based Scoping**: Clean visual hierarchy that mirrors code execution structure.
2. **Implicit Compile-Time Type Inference**: Strong, static typing without requiring explicit visual type decorations.
3. **Implicit Ownership Lifetimes**: Rust-like compile-time guarantees (Moves & Scope Drops) that are handled automatically behind the scenes, leaving your source code visual noise-free.
