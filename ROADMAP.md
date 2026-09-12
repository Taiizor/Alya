# Alya Project Roadmap

This document outlines the evolutionary milestones, architectural goals, and feature roadmap for the **Alya** programming language.

---

## Vision & Philosophy

Alya is designed to balance the ergonomics of an expressive, readable language with the raw performance of a direct-to-assembly native compiler.

* **Zero Middleware**: Native GNU/Mach-O assembly generation without LLVM or intermediate representations (IR).
* **Batteries Included**: Comprehensive standard library (`std/net`, `std/thread`, `std/fs`, `std/mem`...) requiring zero external runtime dependencies.
* **Instant Productivity**: Built-in developer tooling (`fmt`, `test`, `repl`, `--time`, `--bundle`) embedded directly into the single `alyac` binary.

---

## Status Overview

| Status | Meaning |
| :---: | :--- |
| ✅ | **Completed** — Implemented, verified with unit/e2e tests, and available in stable releases. |
| 🚧 | **In Progress** — Active implementation or architecture design underway. |
| 📋 | **Planned** — Core roadmap target with prioritized technical specification. |
| 💡 | **Exploration** — Conceptual research and RFC stage. |

---

## Milestone Progress

### 🟢 Completed Milestones (v0.0.5)

- [x] **Core Language Specification & Parser** ✅
  - Dynamic type system with compile-time struct type inference.
  - Expressive control flow (`if`/`elif`/`else`, `when` pattern matching, `while`, `for .. in`, `repeat N`).
  - First-class functions, default parameters, multi-return values, and recursion.
  - Robust exception handling (`try ... catch ... finally`, `throw`).
  - Composite data structures: dynamic arrays, hash maps, structs, and tuples.
- [x] **Direct Native Assembly Codegen** ✅
  - Multi-architecture assembly emitters: **ARM64** (Apple Silicon Mach-O & Linux ELF64), **x64** (Windows MinGW, Linux ELF64, macOS Mach-O), and **x86** (32-bit).
  - Codegen optimizations: branch fusion, immediate range splitting (`movz`/`movk`), zero-cycle idioms.
  - Sub-millisecond parser throughput (~2M lines/sec) and near-C execution performance.
- [x] **Batteries-Included Standard Library** ✅
  - Networking (`std/net` with raw TCP/UDP socket I/O and non-blocking `tcp_poll`; HTTP unbundled to `alya-lang/http`).
  - Concurrency (`std/thread` with native OS worker threads).
  - System I/O (`std/fs`, `std/path`, `std/os`, `std/time`, `std/console`, `std/color`).
  - Utilities (`std/rand` with core PRNG/LCG, `std/json` basic parser/stringifier, `std/glob`, `std/hash`, `std/collections`, `std/test`, `std/mem` Arena allocator).
  - Official Standalone Packages (`csv`, `url`, `http`, `crypto`, `rand`, `uuid`, `jwt`, `mime`, `cli`, `logger`, `json`).
- [x] **Integrated Tooling & Platform Packaging** ✅
  - In-place code formatter (`alyac fmt`).
  - Test runner (`alyac test`).
  - Interactive REPL shell (`alyac repl`).
  - Windows executable resource embedding (`winres`, `alyac.exe` PE icon and metadata).
  - macOS Application Bundling (`alyac build --bundle`, automatic `Info.plist`, `.app` structure, and multi-resolution Apple `.icns` packaging).
  - Official brand asset suite (`alya-file`, `alyac`, `alya-app`, `alya-icon`).
  - Real-world application collection (`apps/` featuring HTTP server, benchmark tool, port scanner, Conway's Game of Life, Snake, and TicTacToe).

---

## Upcoming Milestones: The 4 Strategic Pillars

The next evolution of Alya transitions the project from a complete standalone language to an extensible, industry-grade ecosystem.

```text
┌──────────────────────────────────────────────────────────────────────────┐
│                             ALYA NEXT LEVEL                              │
├─────────────────────┬────────────────────┬───────────────────────────────┤
│ 1. Package Manager  │ 2. C FFI Engine    │ 3. Language Server (LSP)     │
│    (alyac pkg)      │    (extern "C")    │    (Editor Intel & IDEs)      │
├─────────────────────┴────────────────────┴───────────────────────────────┤
│ 4. Memory Resilience & Cycle Detection (Weak Refs & Graph Reclamation)   │
└──────────────────────────────────────────────────────────────────────────┘
```

---

### Pillar 1: Package Manager & Dependency Ecosystem (`alyac pkg`) ✅

Enable community library sharing, versioned dependency resolution, and automated build workflows.

#### Completed Capabilities
- **Project Manifest (`alya.toml`)**:
  - Pure Rust built-in TOML parser and serializer for `[package]` and `[dependencies]`.
  - Supports local path dependencies (`{ path = "..." }`), remote Git repositories (`{ git = "...", tag = "...", branch = "..." }`), and version constraints.
  - Compiler compatibility guard (`alya-version = "0.0.5"`) enforcing minimum required compiler version for packages.
  - Rich package metadata (`homepage`, `repository`, `keywords`, `authors`, `license`).
- **Deterministic Lockfile (`alya.lock`)**:
  - Embedded pure Rust SHA-256 cryptographic verification (FIPS 180-4 / RFC 6234).
  - Reproducible builds recording resolved dependencies, entry points, sources, and content checksums.
- **CLI Subcommands & Shortcuts**:
  - `alyac init [path] [--name <name>] [--lib]`: Generate starter package with `alya.toml`, entry file, and `.gitignore`.
  - `alyac add <name> [--path <path>] [--git <url>] [--tag <tag>] [--branch <branch>]`: Add dependency and automatically lock.
  - `alyac install`: Resolve, fetch, and lock all dependencies declared in `alya.toml`.
  - `alyac pkg [init|add|install|list|update]`: Complete package lifecycle manager.
- **Automatic Entry Point Discovery**:
  - Running `alyac run`, `alyac build`, or `alyac check` without an input file inside any package directory automatically locates `alya.toml` and compiles its designated entry file.
- **Compiler Module Resolution Integration**:
  - Native compiler import engine seamlessly resolves package imports (`import "pkg"` / `import "pkg/sub" as alias`) through the manifest, with clear diagnostic errors directing users to `alyac install` if dependencies are missing.
- **Global Package Cache & Storage (`~/.alya/cache`)**:
  - Global package repository cache storing clean checkouts with `.alya-source` metadata.
  - Zero-network project cloning skipping `.git` overhead (`skip_git: true`) for instant local package provisioning.
  - Storage management commands: `alyac pkg cache`, `alyac pkg clean`, and `alyac pkg cache clean`.
- **Transitive Dependency Resolution (Queue / BFS)**:
  - Recursive multi-tier dependency tree resolution locking all nested packages in a flat, deterministic `alya.lock`.
  - Compiler import resolver walks up parent directories to seamlessly bind transitive sub-dependencies.
- **Compiler Migration Diagnostics**:
  - Automatic diagnostics directing legacy standard library imports (`std/csv`, `std/url`, `std/uuid`, `std/crypto`) to their official standalone packages (`alyac add <pkg>`).

---

### Standard Library (Stdlib) vs Package (Pkg) Architecture

To preserve compiler binary lightness, rapid community evolution, and zero-middleware performance, Alya adheres to a clear three-tier architectural separation between embedded standard library modules (`std/*`) and standalone packages (`alya.toml`).

#### 1. Architectural Philosophy

| Criterion | Standard Library (`std/*`) | Standalone Package (`alya-lang/*`) |
| :--- | :--- | :--- |
| **Role** | Core runtime extension and OS syscall abstractions. | Domain-specific, feature-rich ecosystems. |
| **Dependency** | Built directly into compiler, zero external dependencies. | Managed via `alya.toml` and locked in `alya.lock`. |
| **Versioning** | Tied to compiler release (`alyac v0.0.x`). | Independent Semantic Versioning (`v0.1.0`, `v1.2.0`). |
| **Binary Footprint** | Embedded in compiler binary (`include_str!`); must remain minimal. | Zero impact on compiler binary; resolved at build time per project. |
| **Velocity** | Ultra-stable, highly conservative, avoids API breakage. | Rapid iteration, community-driven, continuous feature releases. |
| **Testing** | Core compiler CI test suites. | Dedicated CI matrices, micro-benchmarks, and extensive documentation. |

#### 2. The 3-Tier Classification Model

```text
                  ┌────────────────────────────────────────────────────────┐
                  │                 ALL ALYA MODULES                       │
                  └──────────────────────────┬─────────────────────────────┘
                                             │
             ┌───────────────────────────────┼──────────────────────────────┐
             ▼                               ▼                              ▼
     [TIER 1: CORE STDLIB]           [TIER 2: HYBRID CORE]          [TIER 3: STANDALONE PKG]
  Never externalized; core        Minimal core in stdlib; rich    Fully decoupled domain
  syscalls and language runtime.  API delegated to package.       libraries (unbundled).
  ─────────────────────────────── ─────────────────────────────── ──────────────────────────
  • os, fs, path, time            • cli  (only raw args/flags)    • crypto (SHA, HMAC, KDF)
  • math, str, mem                • net  (only raw TCP/UDP)       • csv    (RFC-4180 parser)
  • collections, thread           • log  (only console colors)    • url    (WHATWG standard)
  • test, bench, console          • rand (only core LCG PRNG)     • http   (client/server)
                                  • json (only basic parse/str)   • uuid   (v4, v7, ULID)
                                                                  • jwt    (RFC-7519 tokens)
```

1. **Tier 1 (Core Stdlib - Preserved & Protected):**
   - Essential system calls and data type intrinsics (`os`, `fs`, `path`, `time`, `math`, `str`, `collections`, `thread`, `mem`, `console`, `test`, `bench`).
   - Must remain built-in for zero-setup execution of standalone scripts and CLI tools.

2. **Tier 2 (Hybrid Modules - Pruned & Lightweight):**
   - **`std/cli`**: Stripped from 570 lines to ~100 lines; provides fast, lightweight OS argument accessors (`cli_raw_args`, `cli_has_flag`). Advanced parsing (subcommands, automated `--help`, validation) lives in `alya-lang/cli`.
   - **`std/net`**: Stripped from 730 lines to ~200 lines; dedicated purely to raw TCP/UDP socket I/O. All HTTP client/server/routing is in `alya-lang/http`.
   - **`std/log`**: Retained as a fast, single-file console logger with ANSI colors. JSON formatting, file rotation, and pipelines live in `alya-lang/logger`.
   - **`std/json`**: Minimal recursive parser and stringifier for basic scripting. Full AST DOM, streaming tokenizer, schema validation, and pretty-printer live in `alya-lang/json`.
   - **`std/rand`**: Stripped from 425 lines to ~85 lines; contains only global LCG PRNG, range generators, and probabilities. Complex distributions, multi-engine PRNGs (SplitMix64, PCG32, Xorshift64), and sampling live in `alya-lang/rand`.

3. **Tier 3 (Standalone Domain Packages - Unbundled):**
   - Domain-heavy libraries completely removed from compiler binary.
   - Friendly compiler migration diagnostics guide users when deprecated stdlib paths are imported (e.g. `import "std/csv"` ➔ suggests `alyac add csv`, `import "std/url"` ➔ `alyac add url`, `import "std/uuid"` ➔ `alyac add uuid`, `import "std/crypto"` ➔ `alyac add crypto`).

#### 3. Official Package Ecosystem Directory

All official packages are published under the `alya-lang` GitHub organization with clean, acyclic dependency graphs (DAG):

| Package | Repository | Description | Dependencies |
| :--- | :--- | :--- | :--- |
| **`rand`** | [`alya-lang/rand`](https://github.com/alya-lang/rand) | Multi-engine PRNG (SplitMix, Xorshift, PCG), statistical distributions, sampling, and byte entropy | None |
| **`crypto`** | [`alya-lang/crypto`](https://github.com/alya-lang/crypto) | Cryptographic hashes (SHA-256, SHA-224, SHA-1, MD5), HMAC, PBKDF2, Base64/Base64URL, timing protection | `rand` |
| **`uuid`** | [`alya-lang/uuid`](https://github.com/alya-lang/uuid) | RFC 4122 UUID v4, RFC 9562 UUID v7 (time-ordered), Crockford Base32 ULID, NanoID, parser & validators | `rand` |
| **`jwt`** | [`alya-lang/jwt`](https://github.com/alya-lang/jwt) | RFC 7519 JSON Web Token signing, verification, and claim validation | `crypto` |
| **`mime`** | [`alya-lang/mime`](https://github.com/alya-lang/mime) | Complete database of 1,000+ MIME types, file extensions, and charset resolution | None |
| **`url`** | [`alya-lang/url`](https://github.com/alya-lang/url) | WHATWG-compliant URL parser, `UrlSearchParams`, percent-encoding, and path normalization | None |
| **`http`** | [`alya-lang/http`](https://github.com/alya-lang/http) | Production HTTP client, server, parametric router, and middleware (CORS, Static, Recovery) | `url`, `mime` |
| **`cli`** | [`alya-lang/cli`](https://github.com/alya-lang/cli) | Advanced command-line argument parser, flag clustering, subcommands, and auto-generated help | None |
| **`logger`** | [`alya-lang/logger`](https://github.com/alya-lang/logger) | High-throughput structured JSON logger, file rotation, and multi-appender pipelines | None |
| **`json`** | [`alya-lang/json`](https://github.com/alya-lang/json) | JSON AST DOM, streaming tokenizer, schema validator, and indentation pretty-printer | None |
| **`csv`** | [`alya-lang/csv`](https://github.com/alya-lang/csv) | RFC 4180 compliant CSV/TSV state machine, streaming parser, and header-to-map record mapping | None |

---

### Pillar 2: Foreign Function Interface (C FFI Engine) 📋

Enable direct interoperability with existing C, C++, and system libraries without writing glue code or wrappers.

#### Goals & Architecture
- **External Declaration Syntax (`extern "C"`)**:
  ```alya
  # Declare foreign C functions from shared libraries (.so, .dylib, .dll)
  extern "C" from "sqlite3"
      function sqlite3_open(filename: str, db: ptr) -> i32
      function sqlite3_close(db: ptr) -> i32
      function sqlite3_errmsg(db: ptr) -> str
  end

  # Seamless native invocation
  let db_ptr = null
  let rc = sqlite3_open("test.db", db_ptr)
  if rc != 0
      say "Failed to open database: " + sqlite3_errmsg(db_ptr)
  end
  ```
- **ABI & Data Marshalling**:
  - Standard C ABI calling conventions: System V AMD64 ABI (Linux/macOS), Microsoft x64 Calling Convention (Windows), and ARM64 AAPCS.
  - Native type mapping: primitives (`i8`–`i64`, `f32`, `f64`, `bool`), C-string pointers (`*const char`), raw pointers (`ptr`), and opaque structs.
  - Function pointer callbacks: passing Alya functions as C function pointers.
- **Dynamic & Static Linking**:
  - Runtime dynamic library resolution via `dlopen`/`dlsym` (POSIX) and `LoadLibrary`/`GetProcAddress` (Windows).
  - Compile-time linking flags (`-l<lib>`, `-L<path>`).

---

### Pillar 3: Language Server Protocol (Alya LSP) 📋

Provide modern IDE capabilities across VS Code, JetBrains, Neovim, and Sublime Text.

#### Goals & Architecture
- **Dedicated LSP Subcommand (`alyac lsp`)**:
  - Standard Microsoft Language Server Protocol v3.17 server running over JSON-RPC (stdio or TCP).
  - Incremental AST parsing with fault tolerance for incomplete syntax during typing.
- **Editor Features**:
  - **Real-Time Diagnostics**: Instant error squiggles and syntax warnings as the user types.
  - **Autocompletion**: Context-aware suggestions for keywords, variables, struct fields, functions, and standard library modules (`std/*`).
  - **Go to Definition**: Jump directly to function definitions, imported modules, or struct declarations.
  - **Hover Information**: Inline type info, parameter hints, and module documentation.
  - **Document Formatting**: Direct integration with `alyac fmt` for format-on-save.
  - **Semantic Highlighting**: Rich token coloring beyond static regex grammars.
- **Official Editor Extensions**:
  - Publish `vscode-alya` on Visual Studio Code Marketplace and Open VSX Registry.
  - Treesitter grammar and Neovim LSP configuration presets.

---

### Pillar 4: Memory Resilience & Cycle Detection (ARC Enhancements) 📋

Enhance Alya's Automatic Reference Counting (ARC) with advanced cyclic graph reclamation and memory profiling.

#### Goals & Architecture
- **The Cyclic Reference Problem**:
  - Currently, ARC immediately frees objects when reference counts reach zero with zero latency.
  - Self-referencing structures (e.g., node `A` references node `B`, and `B` references `A`) keep reference counts above zero, causing memory leaks upon disconnection.
- **Weak References (`weak_ref`)**:
  - Introduce weak reference primitives (`weak_ref(obj)`) that observe targets without incrementing strong reference counts.
  - Safe dereferencing (`weak_upgrade(w)`) returning `null` if the target was already freed.
- **Background / Scoped Cycle Collector**:
  - Non-blocking, trial-deletion cycle detection algorithm (inspired by Bacon-Rajan).
  - Triggered periodically or on-demand to identify and sweep isolated cyclic reference islands.
- **Memory Diagnostics & Heap Trace**:
  - `alyac run <file> --mem-trace`: Detailed heap allocation counter, live reference inspector, and leak detector output on program exit.

---

## Future Explorations 💡

- **Optional Type Annotations (`let x: int`, `fn add(a: int, b: int): int`)**: Gradual typing for high-performance JIT/AOT code generation and compile-time contract enforcement.
- **WebAssembly Target (`wasm32-unknown-unknown`)**: Compile Alya code directly to WebAssembly for browser sandboxes and edge compute runtimes.
- **Native GUI Toolkit Integration**: Direct bindings to lightweight native windowing (e.g., Cocoa on macOS, Win32/DirectX on Windows, Wayland/X11 on Linux).
- **SIMD Vectorization**: Explicit vector primitives (`f64x4`, `i32x8`) mapping directly to AVX2/AVX-512 and ARM Neon instructions.

---

## Contributing to the Roadmap

Have ideas or want to champion a roadmap milestone?
- Review [CONTRIBUTING.md](CONTRIBUTING.md) to get started with the codebase.
- Open an issue or discussion on [GitHub](https://github.com/alya-lang/alya/issues) tagged with `[RFC]` or `[Roadmap]`.
