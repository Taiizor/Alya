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
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `2.4 ms` | **`9.8 ms`** | `13.5 ms` | `128.6 ms` | **4.1x** | **13.2x faster** | **1.4x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `3.5 ms` | **`8.9 ms`** | `10.1 ms` | `133.7 ms` | **2.5x** | **15.1x faster** | **1.1x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `1.1 ms` | **`1.7 ms`** | `6.8 ms` | `19.5 ms` | **1.6x** | **11.6x faster** | **4.0x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `5.2 ms` | **`7.9 ms`** | `14.1 ms` | `473.9 ms` | **1.5x** | **60.2x faster** | **1.8x faster** |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native assembly (ARM64, x64, x86) adhering strictly to platform ABIs with direct branch and link (`bl` / `call`) and return instructions. There are no virtual machine dispatch loops, garbage collection pauses, or interpreter frames.
* **Result:** **4.1x of C (-O2)**, **1.4x faster than Bun**, and **13.2x faster than Python**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float operations directly to hardware floating-point registers (`d0-d2` on ARM64, `xmm0-xmm1` on x64/x86) and fuses loop comparisons directly into single conditional branches.
* **Result:** **2.5x of C (-O2)**, **1.1x faster than Bun**, and **15.1x faster than Python**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya performs single-comparison unsigned bounds checks (`b.hs` / `jae`) and calculates element addresses with native scaled base + index pointer arithmetic (`[x0, x1, lsl #3]` / `[rax + rbx*8]`).
* **Result:** **1.6x of C (-O2)**, **4.0x faster than Bun**, and **11.6x faster than Python**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Why Alya is Fast:** Direct string index intrinsics bypass runtime function call overhead; bitwise masking is optimized natively (`ubfx` on ARM64, direct immediate bitwise ops on x64/x86); and loop conditions use zero-overhead branch fusion.
* **Result:** **1.5x of C (-O2)**, **1.8x faster than Bun**, and **60.2x faster than Python**.

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Mean | Error | StdDev | Min | Max | Allocated | Alloc Ratio | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 1393 | `287.25 µs` | `728.00 ns` | `8.26 µs` | `280.37 µs` | `438.32 µs` | **`508.73 KB`** | `1.00` | **75.6 MB/s** |
| **`Parser::parse`** | 586 | `683.17 µs` | `4.61 µs` | `33.89 µs` | `616.70 µs` | `855.80 µs` | **`981.01 KB`** | `1.93` | **1722851 lines/s** |
| **`ProgramInference::analyze`** | 40 | `10.24 ms` | `51.09 µs` | `93.39 µs` | `10.17 ms` | `10.73 ms` | **`170.71 KB`** | `0.34` | **98 ops/s** |
| **`CodeGen::generate (x64)`** | 11 | `37.49 ms` | `201.14 µs` | `161.13 µs` | `37.31 ms` | `37.70 ms` | **`2.92 MB`** | `5.89` | **280057 asm lines/s** |
| **`Full Frontend Pipeline`** | 14 | `38.00 ms` | `259.07 µs` | `234.14 µs` | `37.81 ms` | `38.58 ms` | **`4.05 MB`** | `8.15` | **26.3 files/s** |

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
