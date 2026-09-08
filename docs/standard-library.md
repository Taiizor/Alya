# Chapter 7: Standard Library Reference

[← Error Handling](error-handling.md) • [Wiki Home](README.md) • [Next: Architecture & Internals →](architecture-and-internals.md)

---

## 1. Overview

Alya comes with a comprehensive, zero-dependency standard library bundled directly into the compiler (`stdlib/`). You can import any module using `import "std/<module>"`. If standard library files are not present on disk, the `alyac` compiler automatically extracts them from its embedded binary fallback table.

---

## 2. Module Catalog

### 🔤 `std/str` — Advanced String Utilities
```alya
import "std/str"
```
* `starts_with(s, prefix)`: Returns `1` if string starts with prefix, else `0`.
* `ends_with(s, suffix)`: Returns `1` if string ends with suffix, else `0`.
* `replace(s, old, new)`: Replaces occurrences of `old` with `new`.
* `str_repeat(s, count)`: Repeats string `s` `count` times.
* `pad_left(s, width, pad_char)`: Pads string on the left to `width`.
* `pad_right(s, width, pad_char)`: Pads string on the right to `width`.
* `capitalize(s)`: Converts the first letter to uppercase.
* `lines(s)`: Splits string by newline `\n` into an array of lines.
* `count_matches(s, sub)`: Counts occurrences of substring `sub`.
* `is_empty(s)`: Returns `1` if string is empty or whitespace.

---

### 📐 `std/math` — Math, Statistics & PRNG
```alya
import "std/math"
```
* **Trigonometry**: `sin(x)`, `cos(x)`, `tan(x)`, `hypot(a, b)`
* **Rounding & Truncation**: `round(x)`, `floor(x)`, `ceil(x)`, `trunc(x)`
* **Pseudo-Random Number Generator**: `rand_seed(seed)`, `rand_range(min, max)`
* **Array Statistics**: `sum(arr)`, `mean(arr)`, `median(arr)`
* **Utilities**: `clamp(val, min, max)`, `sign(x)`, `is_even(n)`, `is_odd(n)`

---

### 📁 `std/fs` & `std/path` — Filesystem & Path Utilities
```alya
import "std/fs"
import "std/path"
```
* **`std/fs`**:
  * `fs_exists(path)`: Check if file exists (`1` or `0`).
  * `fs_read(path)`: Read entire file content into string.
  * `fs_write(path, content)`: Overwrite or create file with content.
  * `fs_append(path, content)`: Append content to file.
  * `fs_size(path)`: File size in bytes.
  * `fs_remove(path)`: Delete file from disk.
  * `copy_file(src, dest)`, `move_file(src, dest)`
* **`std/path`**:
  * `path_join(dir, file)`: Normalize and join path segments.
  * `file_name(path)`, `file_ext(path)`, `file_stem(path)`, `parent_dir(path)`.
  * `is_absolute(path)`, `path_separator()`.

---

### 🔐 `std/hash` — Hashing & Encodings
```alya
import "std/hash"
```
* `fnv1a(str)`: 32-bit FNV-1a non-cryptographic hash integer.
* `djb2(str)`: DJB2 string hash integer.
* `hex_encode(str)`, `hex_decode(hex)`
* `base64_encode(str)`, `base64_decode(b64)`

---

### 📦 `std/collections` — High-Level Data Structures
```alya
import "std/collections"
```
* **Stack**: `stack_new()`, `stack_push(st, val)`, `stack_pop(st)`, `stack_peek(st)`, `stack_size(st)`
* **Queue**: `queue_new()`, `queue_push(q, val)`, `queue_pop(q)`, `queue_peek(q)`, `queue_size(q)`
* **Set**: `set_new()`, `set_add(s, val)`, `set_has(s, val)`, `set_remove(s, val)`, `set_size(s)`

---

### 🧪 `std/test` — Micro-Testing Framework
```alya
import "std/test"
```
* `test_suite("Name")`: Initialize a named test suite.
* `assert(condition, "message")`: General assertion.
* `assert_eq(actual, expected, "message")`: Integer equality assertion.
* `assert_ne(actual, unexpected, "message")`: Integer inequality assertion.
* `assert_str_eq(actual, expected, "message")`: String equality assertion.
* `test_summary()`: Prints pass/fail summary report.

---

### 🧠 `std/mem` — Arena Allocator & Raw Memory
```alya
import "std/mem"
```
High-performance linear allocation arena with instant bulk deallocation:
* `arena_new(capacity)`: Allocate a linear arena buffer.
* `arena_alloc_mem(arena, size)`: Fast O(1) pointer-bump allocation.
* `arena_clear(arena)`: Instant reset of arena offset without per-object free calls.
* `arena_free_all(arena)`: Release entire arena back to OS.

---

## 3. Progressive Examples

### Level 1: Pure & Minimal (String Formatting & Math)
```alya
import "std/str"
import "std/math"

let text = "alya"
say capitalize(text)               # "Alya"
say pad_left("42", 5, "0")          # "00042"

let angle = 0.0
say "cos(0) = " + round(cos(angle)) # 1
```

---

### Level 2: Practical & Idiomatic (Automated Test Suite)
Creating a robust unit test suite validating business rules:

```alya
import "std/test"
import "std/math"
import "std/str"

test_suite("Core Math & String Assertions")

# Math tests
assert_eq(clamp(15, 0, 10), 10, "Upper clamp bound")
assert_eq(clamp(-5, 0, 10), 0,  "Lower clamp bound")
assert_eq(sum([1, 2, 3, 4, 5]), 15, "Array sum")

# String tests
let greeting = "Hello, World!"
assert_eq(starts_with(greeting, "Hello"), 1, "starts_with")
assert_eq(ends_with(greeting, "World!"), 1, "ends_with")
assert_str_eq(replace("foo-bar-foo", "foo", "baz"), "baz-bar-baz", "string replace")

test_summary()
```

---

### Level 3: Advanced & Real-World (JSON Snapshot & Checksum Storage Pipeline)
A production-like data persistence system that serializes an entity to JSON, computes an FNV-1a checksum, writes it to a file path, and verifies integrity:

```alya
import "std/fs"
import "std/path"
import "std/hash"
import "std/json"

function save_entity_snapshot(dir, filename, id, name, score)
    # Ensure directory path is normalized
    let full_path = path_join(dir, filename)

    # Construct JSON payload
    let json_id    = json_number("id", id)
    let json_name  = json_string("name", name)
    let json_score = json_number("score", score)

    let payload = json_object([json_id, json_name, json_score])

    # Calculate checksum for data integrity
    let checksum = fnv1a(payload)
    say "[SNAPSHOT] Generated payload: {payload}"
    say "[SNAPSHOT] FNV-1a Checksum: {checksum}"

    # Write snapshot to disk
    fs_write(full_path, payload)

    # Verify write
    if fs_exists(full_path) == 1
        let read_back = fs_read(full_path)
        let verify_checksum = fnv1a(read_back)

        if checksum == verify_checksum
            say "[SUCCESS] File saved and verified at: {full_path}"
            return 1
        else
            say "[ERROR] Checksum mismatch during verification!"
            return 0
        end
    else
        say "[ERROR] Failed to create snapshot file!"
        return 0
    end
end

# Run the snapshot backup pipeline
save_entity_snapshot(".", "player_state.json", 101, "AlyaDev", 9850)

# Clean up scratch test file
if fs_exists("player_state.json")
    fs_remove("player_state.json")
    say "[CLEANUP] Scratch snapshot removed."
end
```
