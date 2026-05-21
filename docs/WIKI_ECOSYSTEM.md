# 📦 Krait Ecosystem, Tooling & LLM Guide

Krait is designed as a complete, modern development ecosystem. This guide covers how to use Krait's command-line interface (CLI), modular standard library, high-performance compiler optimizations, and provides a clear prompting blueprint to enable LLMs to write perfect Krait code.

---

## 🛠️ Command-Line Interface (CLI)

The `krait` executable provides all the tools needed to scaffold, check, run, format, and compile code.

| Command | Action | Example |
| :--- | :--- | :--- |
| `krait` | Starts the high-performance Interactive Shell (REPL) | `krait` |
| `krait new <name>` | Scaffolds a new project directory structure | `krait new my_app` |
| `krait run <file>` | Parses and executes a `.kr` file dynamically | `krait run examples/calculator.kr` |
| `krait check <file>` | Runs type-checking, module imports, and static ownership checks | `krait check examples/ownership_error.kr` |
| `krait fmt <file>` | Automatically formats indentation and code in-place | `krait fmt src/main.kr` |
| `krait build <file>` | Compiles code to a highly optimized native executable | `krait build examples/calculator.kr` |

### 1. Scaffolding Projects (`krait new`)
Initializes a standard Krait project layout. This creates a configuration file and a main script template:
```text
my_app/
├── krait.toml       # Package metadata and dependencies
└── src/
    └── main.kr      # Code entry point
```

### 2. High-Performance Native Compilation (`krait build`)
When you compile code natively, the compiler performs highly optimized steps:
- Merges recursive `import` AST nodes.
- Performs full type checking and ownership borrow checks.
- Translates AST to structured LLVM-IR.
- Invokes `clang` under the hood with maximum system optimizations:
  - `-O3`: Maximum loop unrolling, register allocation, and execution optimizations.
  - `-flto`: Link-Time Optimization across the whole binary.
  - `-march=x86-64-v2`: Broad instruction-set optimization suitable for modern development systems as well as older processors (e.g. Intel Celeron, 10th Gen Core i3 laptops).

---

## 📚 Standard Library Modules

Krait includes modular standard library components located in the system `lib/` directory.

### 1. `import io`
Exposes the system input/output FFI.
- **Function**: `putchar(char_code)` - Writes a character to the console by integer ASCII value.
- **Usage**: Used to print text dynamically without high runtime memory overhead.
```python
import io

make print_newline()
    putchar(10)
```

### 2. `import math`
Provides high-performance, pure Krait math functions.
- **Function**: `abs(n)` - Computes the absolute value of an integer `n`.
- **Function**: `power(base, exp)` - Computes exponentiation ($base^{exp}$) using an optimized native loop.
```python
import math

set val = abs(0 - 50)
set sq = power(val, 2)
show sq # Prints 2500
```

---

## 🤖 LLM Prompting & Code-Generation Blueprint

Krait's visual syntax and strict static typing are explicitly designed to be **LLM-Friendly**. Because the compiler outputs clear, structured boxes explaining errors, autonomous coding models can easily fix their mistakes.

To get an LLM to generate correct, production-ready Krait code on the first try, use the following prompt instructions:

```markdown
You are an expert compiler engineer and senior developer in Krait (v1.0.0).
Please generate valid Krait source code matching these rules:

1. Formatting:
   - Use strictly spaces for indentation. Never use tabs.
   - Do NOT use curly braces `{}` or semicolons `;`. Scoping is entirely indentation-based.
   - Write comments using `#`.

2. Variables and Types:
   - Declare/assign variables using `set var = expr`.
   - Modifying struct fields uses `set obj.field = value`.
   - Never mix types in assignments or operations; type coercion does not exist (strongly, statically typed).
   - Use `new StructName` to instantiate heap-allocated structures.

3. Functions & Control Flow:
   - Declare functions using `make name(param1, param2)`.
   - Declare conditionals using `when condition` (must evaluate to a Bool).
   - There is NO 'else' or 'elif' keyword. Use nested/multiple `when` blocks or early returns.
   - Declare loops using `repeat count times`. Do not use 'for' or 'while'.

4. Output and Memory:
   - The `show` keyword prints integer representation values of expressions.
   - For string output, import `io` and call `putchar` with ASCII values inside loops.
   - Assigning a struct variable to another transfers (moves) ownership. Do not use the original variable after moving it.

5. Libraries:
   - You can `import math` and `import io` to access pre-defined systems functions.
```

By providing these rules in your developer workflow or system instructions, LLMs can vibe-code and generate correct Krait software at scale!
