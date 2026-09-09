# Alya Benchmark Suite

Comprehensive performance benchmarks evaluating both the **Alya Compiler (throughput)** and **Alya Runtime (native execution speed)** against established languages: **C (GCC -O2)**, **Bun (JavaScript JIT)**, and **Python 3.12**.

---

## 📊 Cross-Language Execution Performance

All implementations solve the exact same algorithmic problem on identical inputs, with mathematically verified outputs across all targets.

### Test Environment
* **Operating System:** Ubuntu 24.04.4 LTS (x64)
* **C Compiler:** gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0 (`-O2` optimization)
* **JavaScript Engine:** Bun 1.4.2 (JavaScriptCore JIT)
* **Python Runtime:** Python 3.12.14
* **Alya Version:** 0.0.5 (Compiled with `alyac build` in Release mode)
* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.

---

### Benchmark Scoreboard

| Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `2.4 ms` | **`8.6 ms`** | `12.4 ms` | `121.1 ms` | **3.6x** | **14.0x faster** | **1.4x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `3.2 ms` | **`8.1 ms`** | `9.1 ms` | `121.4 ms` | **2.5x** | **15.0x faster** | **1.1x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `1.0 ms` | **`1.6 ms`** | `6.3 ms` | `17.3 ms` | **1.5x** | **10.7x faster** | **3.9x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `4.7 ms` | **`8.7 ms`** | `12.7 ms` | `424.9 ms` | **1.8x** | **48.9x faster** | **1.5x faster** |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native assembly (ARM64, x64, x86) adhering strictly to platform ABIs with direct branch and link (`bl` / `call`) and return instructions. There are no virtual machine dispatch loops, garbage collection pauses, or interpreter frames.
* **Result:** **3.6x of C (-O2)**, **1.4x faster than Bun**, and **14.0x faster than Python**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float operations directly to hardware floating-point registers (`d0-d2` on ARM64, `xmm0-xmm1` on x64/x86) and fuses loop comparisons directly into single conditional branches.
* **Result:** **2.5x of C (-O2)**, **1.1x faster than Bun**, and **15.0x faster than Python**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya performs single-comparison unsigned bounds checks (`b.hs` / `jae`) and calculates element addresses with native scaled base + index pointer arithmetic (`[x0, x1, lsl #3]` / `[rax + rbx*8]`).
* **Result:** **1.5x of C (-O2)**, **3.9x faster than Bun**, and **10.7x faster than Python**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Why Alya is Fast:** Direct string index intrinsics bypass runtime function call overhead; bitwise masking is optimized natively (`ubfx` on ARM64, direct immediate bitwise ops on x64/x86); and loop conditions use zero-overhead branch fusion.
* **Result:** **1.8x of C (-O2)**, **1.5x faster than Bun**, and **48.9x faster than Python**.

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Mean | Error | StdDev | Min | Max | Allocated | Alloc Ratio | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 1362 | `293.75 µs` | `605.00 ns` | `6.79 µs` | `285.44 µs` | `379.82 µs` | **`508.73 KB`** | `1.00` | **73.9 MB/s** |
| **`Parser::parse`** | 591 | `677.52 µs` | `2.80 µs` | `20.68 µs` | `614.09 µs` | `784.68 µs` | **`981.01 KB`** | `1.93` | **1737222 lines/s** |
| **`ProgramInference::analyze`** | 51 | `7.97 ms` | `42.68 µs` | `88.09 µs` | `7.67 ms` | `8.21 ms` | **`74.86 KB`** | `0.15` | **126 ops/s** |
| **`CodeGen::generate (x64)`** | 11 | `38.95 ms` | `467.77 µs` | `374.74 µs` | `38.35 ms` | `39.55 ms` | **`2.79 MB`** | `5.61` | **264208 asm lines/s** |
| **`Full Frontend Pipeline`** | 13 | `39.84 ms` | `800.68 µs` | `697.31 µs` | `38.88 ms` | `41.01 ms` | **`3.91 MB`** | `7.88` | **25.1 files/s** |

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
