# 📝 Krait Syntax & Types Reference

Krait is designed to have a Pythonic frontend with an extremely efficient C/Rust-like static type backend. This guide serves as a precise reference for both human developers and LLM context windows to understand Krait's language grammar, type system, and structural rules.

---

## 📐 Formatting & Indentation Rules

Krait enforces strict, visual clean-code principles:
1. **No Curly Braces (`{}`)**: Block scopes are defined entirely by indentation.
2. **No Semicolons (`;`)**: Line endings are delimited by newlines (`\n`).
3. **Indentation Method**: You must use **spaces** (typically 4 spaces) consistently. Mixing tabs and spaces, or using inconsistent indentation depths, will trigger syntax errors during parsing.
4. **Comments**: Code comments start with a `#` character and extend to the end of the line.

```python
# This is a valid comment in Krait
make add_one(x)
    return x + 1
```

---

## 💎 Primitive Types

Krait is a **statically typed** language, meaning that every variable's type is determined at compile time. However, to keep code clean and free of cognitive noise, Krait features **implicit type inference**. You do not write explicit type annotations (like `int` or `float`); instead, the compiler infers them from assignments and expressions.

| Primitive Type | Description | Example Lit |
| :--- | :--- | :--- |
| `Int` | 64-bit signed integer | `42`, `0 - 5` |
| `Float` | 64-bit floating-point number | `3.14`, `1.0` |
| `Str` | UTF-8 encoded string literal | `"hello"`, `"Krait"` |
| `Bool` | Boolean truth value | `true`, `false` |
| `Void` | Indicates the absence of a value | Implicitly returned by side-effect functions |

---

## 🏷️ Variables & Assignment

Variables in Krait are declared or updated using the `set` keyword. Because of strict static typing, once a variable has been declared with a specific type, its type cannot be changed.

### Variable Declaration
```python
set age = 21          # Infers Int
set price = 19.99     # Infers Float
set name = "Alice"    # Infers Str
set is_active = true  # Infers Bool
```

### Reassignment
```python
set age = 22          # Valid: age is still an Int
# set age = "twenty"  # Compile Error! Cannot reassign Int to Str.
```

---

## 🛠️ Functions

Functions are defined using the `make` keyword followed by the function name, parameters in parentheses, and an indented block.

### Structure
```python
make function_name(param1, param2)
    # body of function
    return value
```

### Key Rules
- Parameter types are inferred based on their usage in the function body.
- Return values are optional; if no `return` is provided, or `return` is called without an argument, `Void` is returned.
- Recursive function definitions are fully supported and compiled efficiently.

### Example
```python
make calculate_area(width, height)
    return width * height

set area = calculate_area(10, 5) # area is Int (50)
```

---

## 🏗️ User-Defined Structs

Structs in Krait represent custom data structures. They are defined using `make` followed by the Struct name (no parentheses) and an indented list of fields with their default values.

### Definition
```python
make Point
    x = 0
    y = 0
```

### Instantiation
Instantiate a struct using the `new` operator:
```python
set p = new Point
```
> [!NOTE]
> Instantiating a struct returns a pointer to a heap-allocated memory structure. Heap allocations are managed via Krait's compile-time Ownership Model (see [Memory Model & Ownership](WIKI_OWNERSHIP.md)).

### Field Access & Assignment
Read or write field values using the dot (`.`) operator:
```python
set p.x = 10
set p.y = 20
show p.x # Prints 10
```

---

## 🔄 Control Flow

Krait keeps control flow minimal to eliminate runtime overhead and optimize branching.

### 1. Conditionals (`when`)
Conditional execution uses the `when` keyword. There is no `else` keyword; instead, structure code with early returns or sequential `when` checks.

```python
make sign_of(n)
    when n < 0
        return 0 - 1
    when n > 0
        return 1
    return 0
```

> [!IMPORTANT]
> The condition expression in a `when` statement **must** evaluate to a `Bool` type. Any other type will trigger a compile-time Type Mismatch error.

### 2. Repetition (`repeat`)
Loops are represented by the `repeat` block, which executes its body a specific number of times.

```python
make power(base, exp)
    set result = 1
    repeat exp times
        set result = result * base
    return result
```

---

## 🔌 FFI & Externs

To support ultra-fast system integration and avoid runtime performance bottlenecks, Krait features a built-in Foreign Function Interface (FFI) to import and invoke native C functions directly.

### Syntax
```python
extern make external_function_name(param1, param2)
```

### Example
You can import the standard C `putchar` function directly to write custom console output:
```python
extern make putchar(char_code)

make print_hi()
    putchar(72)  # 'H'
    putchar(105) # 'i'
    putchar(10)  # '\n'
```

---

## 📦 Module System (`import`)

Krait includes a modern module compiler that resolves code files recursively. 

- The `import module_name` command looks up the file `lib/module_name.kr` relative to the build path.
- The compiler parses and merges the imported module's abstract syntax tree (AST) into your application namespace before executing semantic checks or generating LLVM-IR.

### Example
```python
# Import math standard library
import math

# Use functions defined inside lib/math.kr
set p = power(2, 8)
show p # Prints 256
```
