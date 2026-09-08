# Chapter 1: Getting Started

[← Wiki Home](README.md) • [Next: Language Basics →](basics.md)

---

## 1. Overview

The `alyac` compiler is a standalone binary written in Rust that compiles Alya source files (`.alya`) directly into native assembly language (GNU as or Apple Mach-O) and coordinates with system toolchains (`gcc`, `clang`, or `x86_64-w64-mingw32-gcc`) to link standalone executables.

---

## 2. Prerequisites

To use `alyac` to build standalone native executables, you need a C linker/assembler installed:

* **Linux**: `gcc` (`sudo apt install build-essential`)
* **macOS**: Apple Command Line Tools (`xcode-select --install`)
* **Windows**: MinGW-w64 (`gcc`), WinLibs, or MSYS2.

---

## 3. Installation

### Option A: Pre-built Binaries (Recommended)
Download ready-to-run releases for your platform from the [Alya Releases](https://github.com/Taiizor/Alya/releases) page. Extract the archive and place `alyac` (or `alyac.exe`) into your system `PATH`.

### Option B: Building from Source
Ensure [Rust 1.75+](https://rustup.rs/) is installed on your machine:

```bash
# Clone the repository
git clone https://github.com/Taiizor/Alya.git
cd Alya

# Build the release binary
cargo build --release

# The compiled binary will be located at:
# ./target/release/alyac (or alyac.exe on Windows)

# Optionally install into ~/.cargo/bin:
cargo install --path .
```

---

## 4. CLI Command Reference

`alyac` provides a clean and modern CLI interface:

```text
Usage: alyac [command] [options] <file.alya>

Commands:
  run <file>               Compile and immediately execute the program
  build <file>             Compile directly to a native executable
  check <file>             Validate source code syntax without generating code
  ast <file>               Display the parsed Abstract Syntax Tree
  tokens <file>            Display the lexer token stream

Options:
  -o, --output <file>      Specify output assembly (.s) or executable filename
  -b, --binary             Produce a linked binary executable
  -S, --asm                Produce assembly source code (.s) (default)
  -r, --run                Run the compiled program immediately
      --time               Display detailed microsecond stage timings
      --stats, --bench     Display compilation throughput statistics
  -q, --quiet              Suppress banner and informational compiler output
      --arch <arch>        Target architecture: x86, x64, arm64 (default: host)
      --os <os>            Target OS: windows, linux, macos (default: host)
```

---

## 5. Progressive Examples

### Level 1: Pure & Minimal (One-Liner Execution)
Create `hello.alya`:
```alya
say "Hello, World!"
```

Run it immediately with:
```bash
alyac run hello.alya
```
**Output:**
```text
Hello, World!
```

---

### Level 2: Practical & Idiomatic (Compiling Standalone Binaries)
Compile your code directly to an optimized native binary without needing `alyac` to run it:

```bash
# Compile to a native binary
alyac build hello.alya -o hello_app

# Run the standalone binary directly
./hello_app
```

On Windows:
```powershell
alyac build hello.alya -o hello_app.exe
.\hello_app.exe
```

---

### Level 3: Advanced & Real-World (Profiling & Cross-Target Inspection)

Inspect the generated assembly, measure compiler stage timings, or target another architecture:

```bash
# Measure microsecond breakdown across lexer, parser, codegen, and linker
alyac run hello.alya --time
```
**Output:**
```text
[Alya Profile] Stage Timings:
  Lexing:        34 µs
  Parsing:       52 µs
  Imports:       12 µs
  Codegen:       48 µs
  Linking (GCC): 14,200 µs
  Execution:     1,400 µs
  Total Time:    15,746 µs
```

Generate assembly for a different CPU architecture (e.g. Apple Silicon ARM64 from an x64 machine):
```bash
# Emit clean ARM64 assembly with comments
alyac hello.alya --arch arm64 -o hello_arm64.s
```

Inspect the compiler's Abstract Syntax Tree (AST):
```bash
alyac ast hello.alya
```
