# Alya Documentation Wiki

Welcome to the official **Alya Programming Language** documentation wiki. This guide is organized progressively—from the simplest fundamental concepts to production-grade architectural patterns.

---

## 🧭 Syllabus & Learning Path

```text
Getting Started ──► Language Basics ──► Control Flow ──► Functions & Modules
                                                                │
Architecture ◄── Standard Library ◄── Error Handling ◄── Data Structures
```

| Chapter | Topic | Highlights | Complexity |
|:---|:---|:---|:---:|
| **[1. Getting Started](getting-started.md)** | Toolchain & Workflow | Installing `alyac`, compiling binaries, running scripts, CLI flags (`--time`, `--arch`) | 🟢 Beginner |
| **[2. Language Basics](basics.md)** | Syntax & Fundamentals | Variables (`let`), numbers, strings, comments, operators, `say`, `ask`, interpolation | 🟢 Beginner |
| **[3. Control Flow](control-flow.md)** | Decision & Iteration | `if`/`elif`/`else`, `while`, `for .. in`, `repeat`, `break`/`continue`, `when` pattern matching | 🟢 Beginner |
| **[4. Functions & Modules](functions-and-modules.md)** | Code Organization | Defining functions, recursion, file imports (`import`), circular dependency prevention | 🟡 Intermediate |
| **[5. Data Structures](data-structures.md)** | Collections & Structs | Dynamic arrays, Hash Maps (`map()`), custom composite types (`struct Point`), field mutation | 🟡 Intermediate |
| **[6. Error Handling](error-handling.md)** | Safety & Exceptions | Structured `try ... catch ... finally`, runtime guards (div-by-zero, bounds), custom `throw` | 🟡 Intermediate |
| **[7. Standard Library Reference](standard-library.md)** | Batteries Included | Comprehensive reference for `std/str`, `std/math`, `std/fs`, `std/path`, `std/hash`, `std/json`, `std/test`, `std/mem` | 🔴 Advanced |
| **[8. Architecture & Internals](architecture-and-internals.md)** | Compiler & Codegen | Pipeline overview, ARM64 / x64 / x86 codegen, Branch Fusion, Unsigned Bounds Checks | 🔴 Advanced |

---

## 🎯 Progressive Pedagogy

Every topic in this wiki is structured in three progressive tiers:
1. **Level 1 — Pure & Minimal**: The simplest, single-purpose form with zero boilerplate.
2. **Level 2 — Practical & Idiomatic**: Real-world usage combining control flow and built-ins.
3. **Level 3 — Advanced & Real-World**: Complex, robust implementations handling edge cases and performance considerations.

---

## ⚡ Quick Links
* 🚀 **GitHub Repository**: [Taiizor/Alya](https://github.com/Taiizor/Alya)
* 📊 **Performance Benchmarks**: [benchmarks/README.md](../benchmarks/README.md)
* 💡 **Code Examples**: [examples/](../examples/)
* 📦 **Releases & Downloads**: [GitHub Releases](https://github.com/Taiizor/Alya/releases)
