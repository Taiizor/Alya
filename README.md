# Alya Programming Language

Alya is a simple, easy-to-learn programming language designed to be intuitive for beginners while offering unique features that set it apart from other languages.

## Features

- **Simple and Intuitive**: Easy-to-understand syntax inspired by natural language
- **Type-Safe**: Strong typing with type inference
- **Multi-Platform**: Compiles to native code for Windows, Linux, and macOS
- **Multi-Architecture**: Supports x64, x86, and ARM64
- **Fast**: Written in Rust for performance and safety
- **Unique Syntax**: Familiar yet distinctive approach to programming

## Installation

### From Source

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build Alya
cargo build --release

# The compiler will be in target/release/alyac
```

### Using Cargo

```bash
cargo install --path .
```

## Quick Start

### Hello World

```alya
say "Hello, World!"
```

### Variables

```alya
let name = "Alya"
let age = 1
let pi = 3.14159
```

### Arithmetic

```alya
let x = 10
let y = 5
say x + y    # 15
say x - y    # 5
say x * y    # 50
say x / y    # 2
```

### Functions

```alya
function greet(name)
    say "Hello, " + name + "!"
end

greet("World")
```

### Conditionals

```alya
let score = 85

if score >= 90
    say "Excellent!"
else if score >= 70
    say "Good job!"
else
    say "Keep trying!"
end
```

### Loops

```alya
# For loop
for i in 1..5
    say i
end

# While loop
let count = 0
while count < 5
    say count
    count = count + 1
end
```

## Unique Features

### Natural Language Keywords

Alya uses intuitive keywords:
- `say` instead of `print`
- `ask` instead of `input`
- `when` for pattern matching
- `repeat` for infinite loops

### Simple Pattern Matching

```alya
when value
    is 1 then say "One"
    is 2 then say "Two"
    else say "Other"
end
```

### Built-in String Interpolation

```alya
let name = "Alice"
let age = 25
say "My name is {name} and I am {age} years old"
```

## Compilation

```bash
# Compile to assembly
alyac hello.alya

# Compile directly to executable (recommended!)
alyac hello.alya --output-binary
./program

# Specify output filename
alyac hello.alya --output-binary -o hello.exe

# Target specific architecture
alyac hello.alya --arch arm64

# Target specific OS
alyac hello.alya --os windows
```

### Direct Executable Compilation

The `--output-binary` flag compiles directly to an executable in one step:

```bash
# Simple one-command compilation
alyac hello.alya --output-binary
./program

# With custom output name
alyac calculator.alya --output-binary -o calc.exe
./calc.exe

# Works with all options
alyac hello.alya --output-binary --arch x64 --os linux -o hello
```

This automatically:
- Generates assembly code
- Calls GCC to assemble and link
- Cleans up temporary files
- Produces a ready-to-run executable

### Manual Assembly and Linking

If you prefer the two-step process or need more control:

**Standard (x64):**
```bash
alyac hello.alya
gcc output.s -o program -no-pie
./program
```

**For x86 (32-bit):**
```bash
alyac hello.alya --arch x86
gcc -m32 output.s -o program -no-pie  # Requires multilib support
./program
```

**For ARM64 (on x64 Linux):**
```bash
alyac hello.alya --arch arm64
aarch64-linux-gnu-gcc output.s -o program  # Requires ARM64 cross-compiler
# Run on ARM64 device or emulator
```

**Note:** On Windows, only x64 is fully supported with standard MinGW-w64. ARM64 code generation works but requires an ARM64-capable assembler.

## Language Design

Alya's design philosophy:
1. **Readability First**: Code should read like English
2. **Minimal Syntax**: Less punctuation, more words
3. **Explicit Over Implicit**: Clear intent over brevity
4. **Learn by Example**: Documentation through examples

## Examples

See the `examples/` directory for more programs:
- `hello.alya` - Hello World program
- `arithmetic.alya` - Basic arithmetic operations
- `variables.alya` - Variable declarations
- `calculator.alya` - Simple calculator with complex expressions
- `fibonacci.alya` - Fibonacci number sequence demonstration

## Building and Testing

```bash
# Build
cargo build

# Run tests
cargo test

# Run with examples
cargo run -- examples/hello.alya

# Build optimized release
cargo build --release
```

## Architecture

The Alya compiler consists of:
1. **Lexer**: Tokenizes source code
2. **Parser**: Builds Abstract Syntax Tree (AST)
3. **Semantic Analyzer**: Type checking and validation
4. **Code Generator**: Generates assembly for target platform

## Platform Support

| Platform | x64 | x86 | ARM64 |
|----------|-----|-----|-------|
| Linux    | ✅  | ✅  | ✅    |
| Windows  | ✅  | ⚠️  | ⚠️    |
| macOS    | ✅  | ❌  | ✅    |

**Legend:**
- ✅ Fully supported - compiler generates code and system GCC can assemble it
- ⚠️ Code generation supported - requires specific toolchain
- ❌ Not supported by platform

### Platform-Specific Notes

**Windows:**
- **x64**: Fully supported with MinGW-w64 GCC
- **x86**: Code generation works, but requires GCC with multilib support. Compile with: `gcc -m32 output.s -o program`
- **ARM64**: Compiler generates valid ARM64 assembly, but Windows GCC (MinGW) cannot assemble ARM64 code. You would need an ARM64 cross-compiler or native ARM64 Windows toolchain.

**Linux:**
- All architectures fully supported with standard GCC
- For ARM64, use: `aarch64-linux-gnu-gcc` if cross-compiling
- For x86 on x64 system, may need multilib: `sudo apt-get install gcc-multilib`

**macOS:**
- **x64**: Supported on Intel Macs
- **ARM64**: Supported on Apple Silicon (M1/M2/M3)
- **x86**: Not supported (Apple deprecated 32-bit support)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## License

MIT License - See LICENSE file for details

## Comparison with Other Languages

### Python
```python
print("Hello, World!")
```

### JavaScript
```javascript
console.log("Hello, World!");
```

### Alya
```alya
say "Hello, World!"
```

Alya aims to be simpler and more intuitive while maintaining expressiveness.
