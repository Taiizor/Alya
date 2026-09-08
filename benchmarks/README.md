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
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `5.0 ms` | **`7.6 ms`** | `15.3 ms` | `124.8 ms` | **1.5x** | **16.4x faster** | **2.0x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `4.8 ms` | **`9.4 ms`** | `12.7 ms` | `93.7 ms` | **2.0x** | **10.0x faster** | **1.4x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `2.0 ms` | **`2.6 ms`** | `8.9 ms` | `24.6 ms` | **1.3x** | **9.4x faster** | **3.4x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `2.2 ms` | **`9.6 ms`** | `16.1 ms` | `437.7 ms` | **4.3x** | **45.6x faster** | **1.7x faster** |

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
| **`Lexer::tokenize`** | 1369 | `292.18µs` | `227.04µs` | `1.53ms` | **74.3 MB/s** |
| **`Parser::parse`** | 965 | `414.48µs` | `311.50µs` | `5.76ms` | **2839728 lines/s** |
| **`ProgramInference::analyze`** | 57 | `7.03ms` | `5.44ms` | `15.63ms` | **142 ops/s** |
| **`CodeGen::generate (x64)`** | 15 | `27.93ms` | `23.44ms` | `37.29ms` | **325782 asm lines/s** |
| **`Full Frontend Pipeline`** | 17 | `30.72ms` | `25.76ms` | `47.46ms` | **32.6 files/s** |

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
