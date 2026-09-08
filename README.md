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

- ⚡ **Direct Native Codegen**: Emits clean assembly for **ARM64** (Apple Silicon & AArch64), **x64**, and **x86 (32-bit)** with branch fusion and zero-cycle idioms.
- 🚀 **Near-C Execution Speed**: Runs within 1.0x–2.0x of C (GCC `-O2`) and outperforms JavaScript JIT engines (Bun / V8) without VM warmup delays.
- 📚 **Batteries-Included Standard Library**: Built-in, zero-dependency modules for `std/str`, `std/math`, `std/fs`, `std/path`, `std/json`, `std/hash`, `std/collections`, `std/test`, and `std/mem` (Arena allocator).
- 🛡️ **Safety Without Runtime Penalties**: Single-instruction unsigned bounds checks (`jae` / `b.hs`), division/modulo zero protection, and structured `try ... catch`.
- 🗺️ **First-Class Types**: Dynamic arrays (`[1, 2]`), hash maps (`map()`), 64-bit IEEE 754 floats (`f64`), and composite structs (`struct Point ... end`).
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

# Check syntax only without code generation
alyac check examples/calculator.alya
```

---

## Performance

Alya is engineered for rapid compilation and high-performance native execution across all operating systems and architectures.

### Cross-Language Execution Benchmark (Median of 5 runs)

| Benchmark | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs Bun | Alya vs Python |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci (n=30)** | `1.8 ms` | **`7.7 ms`** | `10.7 ms` | `99.5 ms` | **1.4x faster** | **13.0x faster** |
| **Mandelbrot Fractal (200×100)** | `2.7 ms` | **`6.9 ms`** | `8.3 ms` | `106.1 ms` | **1.2x faster** | **15.5x faster** |
| **Sieve of Eratosthenes (50k)** | `0.8 ms` | **`1.2 ms`** | `5.6 ms` | `15.6 ms` | **4.5x faster** | **12.5x faster** |
| **FNV-1a String Hash (50k)** | `4.0 ms` | **`6.0 ms`** | `11.2 ms` | `372.9 ms` | **1.9x faster** | **61.7x faster** |

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
- 🧪 **[Code Examples](examples/)**: 30+ practical programs, algorithms, and self-hosting compiler prototypes.
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
