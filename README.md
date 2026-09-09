<div align="center">

# Alya

**A simple, fast, intuitive, and modern multi-platform compiled programming language.**

[![CI](https://github.com/Taiizor/Alya/actions/workflows/ci.yml/badge.svg)](https://github.com/Taiizor/Alya/actions/workflows/ci.yml)
[![GitHub Release](https://img.shields.io/github/v/release/Taiizor/Alya?include_prereleases&color=blue)](https://github.com/Taiizor/Alya/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Target Architectures](https://img.shields.io/badge/arch-x86%20%7C%20x64%20%7C%20ARM64-blueviolet)](#platform-support)

<p align="center">
  <a href="#syntax-at-a-glance">Syntax</a> •
  <a href="#key-highlights">Highlights</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#performance">Performance</a> •
  <a href="#platform-support">Platforms</a> •
  <a href="#documentation">Documentation</a>
</p>

</div>

---

## Overview

**Alya** is designed to provide clean, readable syntax inspired by natural language without compromising runtime execution speed. Written in Rust, the `alyac` compiler generates native GNU and Mach-O assembly directly—bypassing heavy intermediate representation (IR) or LLVM overhead—and links with system toolchains to produce standalone, blazing-fast native binaries.

---

## Syntax at a Glance

```alya
# Define custom data structures
struct Player
    name
    score
end

# First-class functions with expressive conditionals
function rank_player(p)
    if p.score >= 90
        return "Master"
    elif p.score >= 75
        return "Expert"
    else
        return "Challenger"
    end
end

# Collections, iteration, and string interpolation
let team = [
    Player { name: "Alice", score: 95 },
    Player { name: "Bob", score: 82 }
]

for member in team
    let tier = rank_player(member)
    say "Player {member.name} scored {member.score} pts -> [{tier}]"
end
```

---

## Key Highlights

- ⚡ **Direct Native Codegen**: Emits clean assembly for **ARM64** (Apple Silicon & AArch64), **x64**, and **x86 (32-bit)** with branch fusion, immediate range splitting (`movz`/`movk`), and zero-cycle idioms.
- 🚀 **Near-C Execution Speed**: Runs within 1.0x–2.0x of C (GCC `-O2`) and outperforms JavaScript JIT engines (Bun / V8) without VM warmup delays.
- 🛠️ **Built-in Developer Tooling**: In-place code formatter (`alyac fmt`) and test runner (`alyac test`) built directly into the compiler binary—no external dependencies needed.
- 📚 **Batteries-Included Standard Library**: Zero-dependency modules for `std/net` (TCP/HTTP), `std/console` (terminal control), `std/glob`, `std/rand` (SplitMix64, UUID v4/v7, ULID), `std/csv`, `std/color`, `std/log`, `std/str`, `std/math`, `std/fs`, `std/path`, `std/json`, `std/hash`, `std/collections`, `std/test`, and `std/mem` (Arena allocator).
- 🛡️ **Safety Without Runtime Penalties**: Single-instruction unsigned bounds checks (`jae` / `b.hs`), division/modulo zero protection, null safety (`null`, `??`), and structured `try ... catch`.
- 🗺️ **First-Class Types & Static Inference**: Dynamic arrays (`[1, 2]`), hash maps (`map()`), 64-bit IEEE 754 floats (`f64`), composite structs (`struct Point ... end`), and compile-time multi-pass struct type inference.
- 🎯 **Lightweight Single-Pass Compiler**: Sub-millisecond parser throughput parsing ~2 million lines per second with rich diagnostics and execution profiling (`--time`).

---

## Quick Start

### 1. Installation

Download pre-built standalone binaries for Linux, macOS, and Windows from [GitHub Releases](https://github.com/Taiizor/Alya/releases), or build from source with [Rust](https://rustup.rs/):

```bash
# Clone and build with Cargo
git clone https://github.com/Taiizor/Alya.git
cd Alya
cargo install --path .
```

### 2. Run & Build Programs

```bash
# Compile and run immediately in one step
alyac run examples/hello.alya

# Run with microsecond execution and compiler stage profiling
alyac run examples/hello.alya --time

# Compile directly to a standalone binary
alyac build examples/calculator.alya -o calculator

# Format source files across project in-place (or --check in CI)
alyac fmt .

# Discover and run test suites across the project
alyac test

# Check syntax only without code generation
alyac check examples/calculator.alya
```

---

## Performance

Alya is engineered for rapid compilation and high-performance native execution across all operating systems and architectures.

### Cross-Language Execution Benchmark (Median of 5 runs)

| Benchmark | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs Bun | Alya vs Python |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci (n=30)** | `2.3 ms` | **`8.6 ms`** | `14.9 ms` | `120.2 ms` | **1.7x faster** | **14.0x faster** |
| **Mandelbrot Fractal (200×100)** | `3.2 ms` | **`8.1 ms`** | `8.7 ms` | `121.6 ms` | **1.1x faster** | **15.0x faster** |
| **Sieve of Eratosthenes (50k)** | `1.0 ms` | **`1.5 ms`** | `6.1 ms` | `16.6 ms` | **4.0x faster** | **10.8x faster** |
| **FNV-1a String Hash (50k)** | `4.7 ms` | **`8.9 ms`** | `12.2 ms` | `428.3 ms` | **1.4x faster** | **48.3x faster** |

> 📊 For full cross-platform benchmark results (Linux, macOS, Windows), compiler throughput benchmarks, and reproduction instructions, see **[benchmarks/README.md](benchmarks/README.md)**.

---

## Platform Support

| Operating System | x86 (32-bit) | x64 (64-bit) | ARM64 (AArch64) |
| :--------------- | :----------: | :----------: | :-------------: |
| **Linux**        | ✅ Supported | ✅ Fully Supported (ELF64) | ✅ Supported |
| **macOS**        | ❌ Deprecated by Apple | ✅ Fully Supported (Mach-O) | ✅ Fully Supported (Apple Silicon) |
| **Windows**      | ✅ Supported | ✅ Fully Supported (MinGW-w64) | ⚠️ Cross-compiler required |

---

## Documentation

- 📚 **[Alya Documentation Wiki](docs/README.md)**: Structured 8-chapter guide progressing from beginner concepts to advanced compiler architectures.
- 📖 **[Single-Page Language Guide](docs/language-guide.md)**: Quick full-language reference and syntax cheat-sheet.
- 🧪 **[Code Examples](examples/)**: 50+ practical programs, algorithms, interactive terminal apps (TicTacToe), and self-hosting compiler prototypes.
- ⚡ **[Benchmark Suite](benchmarks/)**: Cross-language performance benchmark sources and runner.

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and adhere to our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

---

## License

This project is licensed under the [MIT License](LICENSE).
