# Alya Benchmark Suite

Comprehensive performance benchmarks evaluating both the **Alya Compiler (throughput)** and **Alya Runtime (native execution speed)** against established languages: **C (GCC -O2)**, **Bun (JavaScript JIT)**, and **Python 3.12**.

---

## 📊 Cross-Language Execution Performance

All implementations solve the exact same algorithmic problem on identical inputs, with mathematically verified outputs across all targets.

### Test Environment
* **Operating System:** macOS (arm64)
* **C Compiler:** Apple clang version 21.0.0 (clang-2100.1.1.101) (`-O2` optimization)
* **JavaScript Engine:** Bun 1.4.2 (JavaScriptCore JIT)
* **Python Runtime:** Python 3.12.10
* **Alya Version:** 0.0.3 (Compiled with `alyac build` in Release mode)
* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.

---

### Benchmark Scoreboard

| Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `6.3 ms` | **`9.4 ms`** | `18.8 ms` | `132.3 ms` | **1.5x** | **14.1x faster** | **2.0x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `5.6 ms` | **`11.2 ms`** | `16.2 ms` | `86.9 ms` | **2.0x** | **7.8x faster** | **1.5x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `2.1 ms` | **`2.7 ms`** | `10.4 ms` | `22.7 ms` | **1.3x** | **8.5x faster** | **3.9x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `2.0 ms` | **`9.4 ms`** | `13.6 ms` | `483.1 ms` | **4.7x** | **51.3x faster** | **1.4x faster** |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native assembly (ARM64, x64, x86) adhering strictly to platform ABIs with direct branch and link (`bl` / `call`) and return instructions. There are no virtual machine dispatch loops, garbage collection pauses, or interpreter frames.
* **Result:** **1.5x of C (-O2)**, outperforming Bun by **2.0x** and Python by **14.1x**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float operations directly to hardware floating-point registers (`d0-d2` on ARM64, `xmm0-xmm1` on x64/x86) and fuses loop comparisons directly into single conditional branches.
* **Result:** **2.0x of C (-O2)**, **7.8x faster than Python**, and **1.5x faster than Bun**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya performs single-comparison unsigned bounds checks (`b.hs` / `jae`) and calculates element addresses with native scaled base + index pointer arithmetic (`[x0, x1, lsl #3]` / `[rax + rbx*8]`).
* **Result:** **1.3x of C (-O2)**, **8.5x faster than Python**, and **3.9x faster than Bun**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Why Alya is Fast:** Direct string index intrinsics bypass runtime function call overhead; bitwise masking is optimized natively (`ubfx` on ARM64, direct immediate bitwise ops on x64/x86); and loop conditions use zero-overhead branch fusion.
* **Result:** **1.4x faster than Bun** and **51.3x faster than Python 3.12**.

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Average Time | Min Time | Max Time | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 1650 | `242.45µs` | `213.25µs` | `387.12µs` | **89.6 MB/s** |
| **`Parser::parse`** | 899 | `445.12µs` | `311.96µs` | `1.74ms` | **2644231 lines/s** |
| **`ProgramInference::analyze`** | 50 | `8.13ms` | `5.90ms` | `13.30ms` | **123 ops/s** |
| **`CodeGen::generate (x64)`** | 15 | `28.20ms` | `23.94ms` | `38.50ms` | **311851 asm lines/s** |
| **`Full Frontend Pipeline`** | 20 | `25.90ms` | `24.19ms` | `32.64ms` | **38.6 files/s** |

---

## 🚀 How to Run the Benchmarks

### Run the Entire Cross-Language Benchmark Suite
Run the automated runner with Bun:
```bash
bun run benchmarks/cross_lang/runner.ts
```

### Run Rust Compiler Throughput Benchmarks
```bash
cargo bench --bench compiler_bench
```

### Run Individual Alya Benchmarks
```bash
# Run with Alya compiler
alyac run benchmarks/cross_lang/fibonacci.alya
alyac run benchmarks/cross_lang/mandelbrot.alya
alyac run benchmarks/cross_lang/sieve.alya
alyac run benchmarks/cross_lang/str_hash.alya

# Run with profiling enabled
alyac run benchmarks/cross_lang/fibonacci.alya --time
```

---

## 📁 Directory Structure

```text
benchmarks/
├── cross_lang/
│   ├── fibonacci.alya      # Alya implementation
│   ├── fibonacci.c         # C implementation
│   ├── fibonacci.js        # JavaScript (Bun) implementation
│   ├── fibonacci.py        # Python 3 implementation
│   ├── mandelbrot.alya
│   ├── mandelbrot.c
│   ├── mandelbrot.js
│   ├── mandelbrot.py
│   ├── sieve.alya
│   ├── sieve.c
│   ├── sieve.js
│   ├── sieve.py
│   ├── str_hash.alya
│   ├── str_hash.c
│   ├── str_hash.js
│   ├── str_hash.py
│   └── runner.ts           # Automated test orchestrator & markdown reporter
└── README.md               # This documentation file
```
