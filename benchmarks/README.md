# Alya Benchmark Suite

Comprehensive performance benchmarks evaluating both the **Alya Compiler (throughput)** and **Alya Runtime (native execution speed)** against established languages: **C (GCC -O2)**, **Bun (JavaScript JIT)**, and **Python 3.12**.

---

## 📊 Cross-Language Execution Performance

All implementations solve the exact same algorithmic problem on identical inputs, with mathematically verified outputs across all targets.

### Test Environment
* **Operating System:** Windows 11 Pro x64
* **C Compiler:** gcc (tdm64-1) 10.3.0 (`-O2` optimization)
* **JavaScript Engine:** Bun 1.4.2 (JavaScriptCore JIT)
* **Python Runtime:** Python 3.12.5
* **Alya Version:** 0.0.3 (Compiled with `alyac build` in Release mode)
* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.

---

### Benchmark Scoreboard

| Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `11.1 ms` | **`14.1 ms`** | `29.0 ms` | `140.9 ms` | **1.3x** | **10.0x faster** | **2.1x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `11.3 ms` | **`15.2 ms`** | `32.1 ms` | `120.6 ms` | **1.3x** | **7.9x faster** | **2.1x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `9.8 ms` | **`13.1 ms`** | `31.6 ms` | `41.2 ms` | **1.3x** | **3.1x faster** | **2.4x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `17.1 ms` | **`15.2 ms`** | `33.1 ms` | `433.9 ms` | **0.9x** | **28.5x faster** | **2.2x faster** |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native assembly (ARM64, x64, x86) adhering strictly to platform ABIs with direct branch and link (`bl` / `call`) and return instructions. There are no virtual machine dispatch loops, garbage collection pauses, or interpreter frames.
* **Result:** **1.3x of C (-O2)**, **2.1x faster than Bun**, and **10.0x faster than Python**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float operations directly to hardware floating-point registers (`d0-d2` on ARM64, `xmm0-xmm1` on x64/x86) and fuses loop comparisons directly into single conditional branches.
* **Result:** **1.3x of C (-O2)**, **2.1x faster than Bun**, and **7.9x faster than Python**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya performs single-comparison unsigned bounds checks (`b.hs` / `jae`) and calculates element addresses with native scaled base + index pointer arithmetic (`[x0, x1, lsl #3]` / `[rax + rbx*8]`).
* **Result:** **1.3x of C (-O2)**, **2.4x faster than Bun**, and **3.1x faster than Python**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Why Alya is Fast:** Direct string index intrinsics bypass runtime function call overhead; bitwise masking is optimized natively (`ubfx` on ARM64, direct immediate bitwise ops on x64/x86); and loop conditions use zero-overhead branch fusion.
* **Result:** **0.9x of C (-O2)**, **2.2x faster than Bun**, and **28.5x faster than Python**.

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Mean | Error | StdDev | Min | Max | Allocated | Alloc Ratio | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 536 | `746.96 µs` | `117.85 µs` | `829.06 µs` | `366.70 µs` | `12.10 ms` | **`508.73 KB`** | `1.00` | **29.1 MB/s** |
| **`Parser::parse`** | 354 | `1.13 ms` | `200.81 µs` | `1.15 ms` | `580.10 µs` | `9.47 ms` | **`1009.57 KB`** | `1.98` | **1041490 lines/s** |
| **`ProgramInference::analyze`** | 44 | `9.11 ms` | `3.24 ms` | `6.21 ms` | `5.18 ms` | `30.97 ms` | **`70.36 KB`** | `0.14` | **110 ops/s** |
| **`CodeGen::generate (x64)`** | 18 | `23.34 ms` | `867.77 µs` | `948.14 µs` | `22.89 ms` | `27.09 ms` | **`656.14 KB`** | `1.29` | **379819 asm lines/s** |
| **`Full Frontend Pipeline`** | 21 | `24.69 ms` | `988.55 µs` | `1.24 ms` | `24.03 ms` | `28.91 ms` | **`1.79 MB`** | `3.61` | **40.5 files/s** |

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
