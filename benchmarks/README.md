# Alya Benchmark Suite

Comprehensive performance benchmarks evaluating both the **Alya Compiler (throughput)** and **Alya Runtime (native execution speed)** against established languages: **C (GCC -O2)**, **Bun (JavaScript JIT)**, and **Python 3.12**.

---

## 📊 Cross-Language Execution Performance

All implementations solve the exact same algorithmic problem on identical inputs, with mathematically verified outputs across all targets.

### Test Environment
* **Operating System:** Windows 11 Pro x64
* **C Compiler:** GCC 10.3.0 (MinGW-w64, `-O2` optimization)
* **JavaScript Engine:** Bun 1.4.2 (JavaScriptCore JIT)
* **Python Runtime:** CPython 3.12.5 (64-bit)
* **Alya Version:** 0.0.3 (Compiled with `alyac build` in Release mode)
* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.

---

### Benchmark Scoreboard

| Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `10.6 ms` | **`14.2 ms`** | `31.9 ms` | `127.4 ms` | **1.3x** | **9.0x faster** | **2.2x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `11.5 ms` | **`17.1 ms`** | `25.5 ms` | `120.6 ms` | **1.5x** | **7.0x faster** | **1.5x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `8.8 ms` | **`10.4 ms`** | `26.2 ms` | `54.8 ms` | **1.2x** | **5.3x faster** | **2.5x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `12.0 ms` | **`92.6 ms`** | `28.7 ms` | `389.7 ms` | **7.7x** | **4.2x faster** | `3.2x slower` |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native x64 assembly obeying the platform ABI with direct `call` and `ret` instructions. There are no virtual machine dispatch loops, garbage collection stops, or interpreter frames.
* **Result:** **1.3x of C (-O2)**, outperforming Bun by **2.2x** and Python by **9.0x**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float math directly to SSE2/AVX hardware registers (`xmm0`, `xmm1`) and emits hardware instructions (`mulsd`, `addsd`, `subsd`, `comisd`).
* **Result:** **1.5x of C (-O2)**, **7.0x faster than Python**, and **1.5x faster than Bun**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya calculates array element addresses directly via base + index pointer arithmetic (`[rax + rbx*8]`), performing closely to C heap-allocated buffers.
* **Result:** **1.2x of C (-O2)**, **5.3x faster than Python**, and **2.5x faster than Bun**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Observation:** Alya is **4.2x faster than Python 3.12**. Bun and C are faster here because C uses raw byte pointers (`*s++`) and V8 inlines `charCodeAt` as an intrinsic.
* **Roadmap Note:** Adding built-in inlining for `char_at` and `ord` in the Alya compiler will close this gap to C-level speeds (~15 ms).

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Average Time | Min Time | Max Time | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 971 | `412.29 µs` | `327.20 µs` | `648.20 µs` | **52.7 MB/s** |
| **`Parser::parse`** | 665 | `601.74 µs` | `511.50 µs` | `954.60 µs` | **1,955,996 lines/s** |
| **`ProgramInference::analyze`** | 80 | `5.04 ms` | `4.94 ms` | `6.13 ms` | **198 ops/s** |
| **`CodeGen::generate (x64)`** | 19 | `21.86 ms` | `21.44 ms` | `24.07 ms` | **545,180 asm lines/s** |
| **`Full Frontend Pipeline`** | 22 | `22.92 ms` | `22.32 ms` | `27.44 ms` | **43.6 files/s** |

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
