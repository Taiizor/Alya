# Alya Benchmark Suite

Comprehensive performance benchmarks evaluating both the **Alya Compiler (throughput)** and **Alya Runtime (native execution speed)** against established languages: **C (GCC -O2)**, **Bun (JavaScript JIT)**, and **Python 3.12**.

---

## 📊 Cross-Language Execution Performance

All implementations solve the exact same algorithmic problem on identical inputs, with mathematically verified outputs across all targets.

### Test Environment
* **Operating System:** Windows 11 Pro x64
* **C Compiler:** gcc (GCC) 16.2.0 (`-O2` optimization)
* **JavaScript Engine:** Bun 1.4.2 (JavaScriptCore JIT)
* **Python Runtime:** Python 3.12.5
* **Alya Version:** 0.0.5 (Compiled with `alyac build` in Release mode)
* **Measurement Methodology:** 1 warmup run, followed by 5 timed runs. Median execution time reported.

---

### Benchmark Scoreboard

| Category | Benchmark | Target Workload | C (GCC -O2) | Alya (Native) | Bun (JS JIT) | Python 3.12 | Alya vs C | Alya vs Python | Alya vs Bun |
| :--- | :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `Algorithms` | **Recursive Fibonacci** | `fib(30)` (~2.69M calls) | `10.8 ms` | **`16.1 ms`** | `28.1 ms` | `131.3 ms` | **1.5x** | **8.2x faster** | **1.8x faster** |
| `Algorithms` | **In-Place Quicksort** | 50,000 items in-place sort | `22.2 ms` | **`122.0 ms`** | `42.3 ms` | `1475.6 ms` | **5.5x** | **12.1x faster** | `2.9x slower` |
| `Algorithms` | **Sieve of Eratosthenes** | Primes under 50,000 | `14.3 ms` | **`18.7 ms`** | `28.9 ms` | `55.3 ms` | **1.3x** | **3.0x faster** | **1.5x faster** |
| `Collections` | **Binary Trees** | Heap tree allocation & traversal | `263.6 ms` | **`467.0 ms`** | `145.5 ms` | `2165.0 ms` | **1.8x** | **4.6x faster** | `3.2x slower` |
| `Collections` | **Hash Map** | 20k insertions, updates & lookups | `17.5 ms` | **`22.5 ms`** | `38.0 ms` | `43.4 ms` | **1.3x** | **1.9x faster** | **1.7x faster** |
| `Numeric` | **Mandelbrot Fractal** | 200×100 grid, 200 iters | `12.9 ms` | **`19.5 ms`** | `28.0 ms` | `111.8 ms` | **1.5x** | **5.7x faster** | **1.4x faster** |
| `Numeric` | **Matrix Multiply** | 120×120 dense integer matrix mult | `12.6 ms` | **`18.7 ms`** | `30.6 ms` | `153.6 ms` | **1.5x** | **8.2x faster** | **1.6x faster** |
| `Strings` | **FNV-1a String Hash** | 50,000 hash calculations | `15.9 ms` | **`22.1 ms`** | `36.8 ms` | `380.7 ms` | **1.4x** | **17.3x faster** | **1.7x faster** |

---

## 🔬 Benchmark Details & Insights

### 1. Recursive Fibonacci (`fib(30)`)
* **Measures:** Function call overhead, standard ABI calling conventions, stack frame push/pop.
* **Why Alya is Fast:** Alya emits native assembly (ARM64, x64, x86) adhering strictly to platform ABIs with direct branch and link (`bl` / `call`) and return instructions. There are no virtual machine dispatch loops, garbage collection pauses, or interpreter frames.
* **Result:** **1.5x of C (-O2)**, **1.8x faster than Bun**, and **8.2x faster than Python**.

### 2. Mandelbrot Fractal (`200x100x200`)
* **Measures:** Double-precision floating-point arithmetic (`f64`), tight nested loops, register persistence.
* **Why Alya is Fast:** Alya binds 64-bit float operations directly to hardware floating-point registers (`d0-d2` on ARM64, `xmm0-xmm1` on x64/x86) and fuses loop comparisons directly into single conditional branches.
* **Result:** **1.5x of C (-O2)**, **1.4x faster than Bun**, and **5.7x faster than Python**.

### 3. Sieve of Eratosthenes (50,000 elements)
* **Measures:** Memory allocation, dynamic array indexing, bounds safety overhead.
* **Why Alya is Fast:** Alya performs single-comparison unsigned bounds checks (`b.hs` / `jae`) and calculates element addresses with native scaled base + index pointer arithmetic (`[x0, x1, lsl #3]` / `[rax + rbx*8]`).
* **Result:** **1.3x of C (-O2)**, **1.5x faster than Bun**, and **3.0x faster than Python**.

### 4. FNV-1a String Hashing (50,000 iterations)
* **Measures:** String iteration, character lookup (`char_at`, `ord`), bitwise XOR and integer multiplication.
* **Why Alya is Fast:** Direct string index intrinsics bypass runtime function call overhead; bitwise masking is optimized natively (`ubfx` on ARM64, direct immediate bitwise ops on x64/x86); and loop conditions use zero-overhead branch fusion.
* **Result:** **1.4x of C (-O2)**, **1.7x faster than Bun**, and **17.3x faster than Python**.

### 5. In-Place Quicksort (50,000 items)
* **Measures:** In-place array mutation, cache locality, deep recursive partitioning.
* **Why Alya is Fast:** Alya provides direct zero-overhead array index writes with native register swapping and minimal function call overhead.
* **Result:** **5.5x of C (-O2)**, **2.9x slower than Bun**, and **12.1x faster than Python**.

### 6. Binary Trees (Depth 14)
* **Measures:** Dynamic memory allocation, recursive tree traversal, struct dereferencing, heap stress.
* **Why Alya is Fast:** Alya allocates structs on a fast native heap with aligned word layouts, dereferencing fields with single-instruction displacement addressing (`[rax + offset]`).
* **Result:** **1.8x of C (-O2)**, **3.2x slower than Bun**, and **4.6x faster than Python**.

### 7. Matrix Multiplication (120x120)
* **Measures:** CPU-bound 3-level nested loops, integer arithmetic, tight sequential memory access.
* **Why Alya is Fast:** Inner loops are compiled directly to native register increments and conditional jumps with loop condition hoisting and zero branch misprediction penalty.
* **Result:** **1.5x of C (-O2)**, **1.6x faster than Bun**, and **8.2x faster than Python**.

### 8. Hash Map Operations (20,000 items)
* **Measures:** String hashing (djb2), bucket collisions, dynamic rehashing, key-value lookup throughput.
* **Why Alya is Fast:** Built-in native hash table implementation with bitwise mask indexing and inline string equality checking.
* **Result:** **1.3x of C (-O2)**, **1.7x faster than Bun**, and **1.9x faster than Python**.

---

## ⚡ Compiler Throughput Benchmarks (`cargo bench`)

Alya features a lightweight single-pass frontend with immediate native x64 assembly generation, avoiding heavy intermediate representation (IR) overhead:

> **Workload:** 1,177 lines, 22.24 KB synthetic program (50+ functions, structs, control flow)

| Benchmark Stage | Iterations | Mean | Error | StdDev | Min | Max | Allocated | Alloc Ratio | Measured Throughput |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`Lexer::tokenize`** | 899 | `445.11 µs` | `13.58 µs` | `123.73 µs` | `341.30 µs` | `3.63 ms` | **`508.73 KB`** | `1.00` | **48.8 MB/s** |
| **`Parser::parse`** | 615 | `650.80 µs` | `12.61 µs` | `95.06 µs` | `582.00 µs` | `1.30 ms` | **`981.01 KB`** | `1.93` | **1808554 lines/s** |
| **`ProgramInference::analyze`** | 52 | `7.76 ms` | `460.67 µs` | `960.09 µs` | `7.39 ms` | `13.81 ms` | **`170.71 KB`** | `0.34` | **129 ops/s** |
| **`CodeGen::generate (x64)`** | 13 | `32.04 ms` | `921.09 µs` | `802.18 µs` | `31.09 ms` | `33.53 ms` | **`2.92 MB`** | `5.89` | **332033 asm lines/s** |
| **`Full Frontend Pipeline`** | 16 | `32.73 ms` | `697.29 µs` | `718.30 µs` | `32.06 ms` | `34.92 ms` | **`4.05 MB`** | `8.15` | **30.6 files/s** |

---

## 🚀 How to Run the Benchmarks

### Run Cross-Language Benchmark Suite
Run the automated runner with Bun:
```bash
# Run all benchmarks (comprehensive suite)
bun run benchmarks/cross_lang/runner.ts

# Run specific category only (algorithms, collections, numeric, strings)
bun run benchmarks/cross_lang/runner.ts --category algorithms
bun run benchmarks/cross_lang/runner.ts --category collections

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
# Algorithms
alyac run benchmarks/cross_lang/algorithms/fibonacci.alya
alyac run benchmarks/cross_lang/algorithms/quicksort.alya
alyac run benchmarks/cross_lang/algorithms/sieve.alya

# Collections & Data Structures
alyac run benchmarks/cross_lang/collections/binary_trees.alya
alyac run benchmarks/cross_lang/collections/hash_map.alya

# Numeric & Math
alyac run benchmarks/cross_lang/numeric/mandelbrot.alya
alyac run benchmarks/cross_lang/numeric/matrix_mult.alya

# Strings & Hashing
alyac run benchmarks/cross_lang/strings/str_hash.alya

# Run with profiling enabled
alyac run benchmarks/cross_lang/algorithms/fibonacci.alya --time
```

---

## 📁 Directory Structure

```text
benchmarks/
├── cross_lang/
│   ├── algorithms/                 # Algorithmic & Sorting benchmarks
│   │   ├── fibonacci.alya          # Recursive Fibonacci (n=30)
│   │   ├── fibonacci.c
│   │   ├── fibonacci.js
│   │   ├── fibonacci.py
│   │   ├── quicksort.alya          # In-Place Quicksort (50,000 items)
│   │   ├── quicksort.c
│   │   ├── quicksort.js
│   │   ├── quicksort.py
│   │   ├── sieve.alya              # Sieve of Eratosthenes (50,000)
│   │   ├── sieve.c
│   │   ├── sieve.js
│   │   └── sieve.py
│   ├── collections/                # Data structures & Heap allocations
│   │   ├── binary_trees.alya       # Binary Trees (Depth 14)
│   │   ├── binary_trees.c
│   │   ├── binary_trees.js
│   │   ├── binary_trees.py
│   │   ├── hash_map.alya           # Hash Map Operations (20,000 items)
│   │   ├── hash_map.c
│   │   ├── hash_map.js
│   │   └── hash_map.py
│   ├── numeric/                    # Numeric & Floating-Point compute
│   │   ├── mandelbrot.alya         # Mandelbrot Fractal (200x100x200)
│   │   ├── mandelbrot.c
│   │   ├── mandelbrot.js
│   │   ├── mandelbrot.py
│   │   ├── matrix_mult.alya        # Matrix Multiplication (120x120)
│   │   ├── matrix_mult.c
│   │   ├── matrix_mult.js
│   │   └── matrix_mult.py
│   ├── strings/                    # String processing & Hashing
│   │   ├── str_hash.alya           # FNV-1a String Hash (50,000 iters)
│   │   ├── str_hash.c
│   │   ├── str_hash.js
│   │   └── str_hash.py
│   └── runner.ts                   # Automated test orchestrator & markdown reporter
└── README.md                       # This documentation file
```
