# Alya Benchmark Suite

Comprehensive performance benchmarks evaluating both the **Alya Compiler (throughput)** and **Alya Runtime (native execution speed)** against established languages: **C (GCC -O2)**, **Bun (JavaScript JIT)**, and **Python 3.12**.

---

## 📊 Cross-Language Execution Performance

All implementations solve the exact same algorithmic problem on identical inputs, with mathematically verified outputs across all targets.

### Test Environment
* **Operating System:** Ubuntu 24.04.5 LTS (x64)
* **C Compiler:** gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0 (`-O2` optimization)
* **JavaScript Engine:** Bun 1.4.2 (JavaScriptCore JIT)
* **Python Runtime:** Python 3.12.14
* **Alya Version:** 0.0.5 (Compiled with `alyac build` in Release mode)
* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.

---

### Benchmark Scoreboard

| Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `2.4 ms` | **`9.6 ms`** | `12.8 ms` | `125.8 ms` | **4.1x** | **13.0x faster** | **1.3x faster** |
| **Mandelbrot Fractal** | 200×100 grid, 200 iters | `3.5 ms` | **`8.9 ms`** | `10.1 ms` | `131.6 ms` | **2.5x** | **14.8x faster** | **1.1x faster** |
| **Sieve of Eratosthenes** | Primes under 50,000 | `1.1 ms` | **`1.7 ms`** | `6.4 ms` | `18.9 ms` | **1.5x** | **11.4x faster** | **3.9x faster** |
| **FNV-1a String Hash** | 50,000 hash calculations | `5.2 ms` | **`7.9 ms`** | `13.4 ms` | `476.9 ms` | **1.5x** | **60.4x faster** | **1.7x faster** |
| **In-Place Quicksort** | 50,000 items in-place sort | `16.5 ms` | **`133.0 ms`** | `34.3 ms` | `2087.0 ms` | **8.0x** | **15.7x faster** | `3.9x slower` |
| **Binary Trees** | Heap tree allocation & traversal | `118.2 ms` | **`465.4 ms`** | `98.4 ms` | `2858.3 ms` | **3.9x** | **6.1x faster** | `4.7x slower` |
| **Matrix Multiply** | 120×120 dense integer matrix mult | `1.3 ms` | **`8.2 ms`** | `12.5 ms` | `236.0 ms` | **6.3x** | **28.8x faster** | **1.5x faster** |
| **Hash Map** | 20k insertions, updates & lookups | `4.2 ms` | **`11.4 ms`** | `14.9 ms` | `26.8 ms` | **2.7x** | **2.4x faster** | **1.3x faster** |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native assembly (ARM64, x64, x86) adhering strictly to platform ABIs with direct branch and link (`bl` / `call`) and return instructions. There are no virtual machine dispatch loops, garbage collection pauses, or interpreter frames.
* **Result:** **4.1x of C (-O2)**, **1.3x faster than Bun**, and **13.0x faster than Python**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float operations directly to hardware floating-point registers (`d0-d2` on ARM64, `xmm0-xmm1` on x64/x86) and fuses loop comparisons directly into single conditional branches.
* **Result:** **2.5x of C (-O2)**, **1.1x faster than Bun**, and **14.8x faster than Python**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya performs single-comparison unsigned bounds checks (`b.hs` / `jae`) and calculates element addresses with native scaled base + index pointer arithmetic (`[x0, x1, lsl #3]` / `[rax + rbx*8]`).
* **Result:** **1.5x of C (-O2)**, **3.9x faster than Bun**, and **11.4x faster than Python**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Why Alya is Fast:** Direct string index intrinsics bypass runtime function call overhead; bitwise masking is optimized natively (`ubfx` on ARM64, direct immediate bitwise ops on x64/x86); and loop conditions use zero-overhead branch fusion.
* **Result:** **1.5x of C (-O2)**, **1.7x faster than Bun**, and **60.4x faster than Python**.

### 5. In-Place Quicksort (50,000 items)
* **Measures:** In-place array mutation, cache locality, deep recursive partitioning.
* **Why Alya is Fast:** Alya provides direct zero-overhead array index writes with native register swapping and minimal function call overhead.
* **Result:** **8.0x of C (-O2)**, **3.9x slower than Bun**, and **15.7x faster than Python**.

### 6. Binary Trees (Depth 14)
* **Measures:** Dynamic memory allocation, recursive tree traversal, struct dereferencing, heap stress.
* **Why Alya is Fast:** Alya allocates structs on a fast native heap with aligned word layouts, dereferencing fields with single-instruction displacement addressing (`[rax + offset]`).
* **Result:** **3.9x of C (-O2)**, **4.7x slower than Bun**, and **6.1x faster than Python**.

### 7. Matrix Multiplication (120x120)
* **Measures:** CPU-bound 3-level nested loops, integer arithmetic, tight sequential memory access.
* **Why Alya is Fast:** Inner loops are compiled directly to native register increments and conditional jumps with loop condition hoisting and zero branch misprediction penalty.
* **Result:** **6.3x of C (-O2)**, **1.5x faster than Bun**, and **28.8x faster than Python**.

### 8. Hash Map Operations (20,000 items)
* **Measures:** String hashing (djb2), bucket collisions, dynamic rehashing, key-value lookup throughput.
* **Why Alya is Fast:** Built-in native hash table implementation with bitwise mask indexing and inline string equality checking.
* **Result:** **2.7x of C (-O2)**, **1.3x faster than Bun**, and **2.4x faster than Python**.

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Mean | Error | StdDev | Min | Max | Allocated | Alloc Ratio | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 1373 | `291.39 µs` | `1.05 µs` | `11.88 µs` | `281.97 µs` | `382.73 µs` | **`508.73 KB`** | `1.00` | **74.5 MB/s** |
| **`Parser::parse`** | 589 | `678.87 µs` | `2.42 µs` | `17.85 µs` | `619.84 µs` | `752.02 µs` | **`981.01 KB`** | `1.93` | **1733755 lines/s** |
| **`ProgramInference::analyze`** | 39 | `10.35 ms` | `297.23 µs` | `536.48 µs` | `9.96 ms` | `11.18 ms` | **`170.71 KB`** | `0.34` | **97 ops/s** |
| **`CodeGen::generate (x64)`** | 11 | `36.76 ms` | `125.93 µs` | `100.89 µs` | `36.63 ms` | `36.94 ms` | **`2.92 MB`** | `5.89` | **289408 asm lines/s** |
| **`Full Frontend Pipeline`** | 14 | `37.68 ms` | `79.81 µs` | `72.13 µs` | `37.49 ms` | `37.79 ms` | **`4.05 MB`** | `8.15` | **26.5 files/s** |

---

## 🚀 How to Run the Benchmarks

### Run Cross-Language Benchmark Suite
Run the automated runner with Bun:
```bash
# Run all benchmarks (comprehensive suite)
bun run benchmarks/cross_lang/runner.ts

# Run standard 4 benchmarks only
bun run benchmarks/cross_lang/runner.ts --suite standard

# Custom iterations (e.g. 10 runs)
bun run benchmarks/cross_lang/runner.ts --iterations 10

# Update README and benchmark documentation
bun run benchmarks/cross_lang/runner.ts --update-readme
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
alyac run benchmarks/cross_lang/quicksort.alya
alyac run benchmarks/cross_lang/binary_trees.alya
alyac run benchmarks/cross_lang/matrix_mult.alya
alyac run benchmarks/cross_lang/hash_map.alya

# Run with profiling enabled
alyac run benchmarks/cross_lang/fibonacci.alya --time
```

---

## 📁 Directory Structure

```text
benchmarks/
├── cross_lang/
│   ├── binary_trees.alya   # Binary Trees benchmark
│   ├── binary_trees.c
│   ├── binary_trees.js
│   ├── binary_trees.py
│   ├── fibonacci.alya      # Recursive Fibonacci
│   ├── fibonacci.c
│   ├── fibonacci.js
│   ├── fibonacci.py
│   ├── hash_map.alya       # Hash Map operations
│   ├── hash_map.c
│   ├── hash_map.js
│   ├── hash_map.py
│   ├── mandelbrot.alya     # Mandelbrot fractal
│   ├── mandelbrot.c
│   ├── mandelbrot.js
│   ├── mandelbrot.py
│   ├── matrix_mult.alya    # Matrix multiplication
│   ├── matrix_mult.c
│   ├── matrix_mult.js
│   ├── matrix_mult.py
│   ├── quicksort.alya      # In-place quicksort
│   ├── quicksort.c
│   ├── quicksort.js
│   ├── quicksort.py
│   ├── sieve.alya          # Sieve of Eratosthenes
│   ├── sieve.c
│   ├── sieve.js
│   ├── sieve.py
│   ├── str_hash.alya       # FNV-1a string hash
│   ├── str_hash.c
│   ├── str_hash.js
│   ├── str_hash.py
│   └── runner.ts           # Automated test orchestrator & markdown reporter
└── README.md               # This documentation file
```
