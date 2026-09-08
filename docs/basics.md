# Chapter 2: Language Basics

[← Getting Started](getting-started.md) • [Wiki Home](README.md) • [Next: Control Flow →](control-flow.md)

---

## 1. Comments

Alya offers flexible commenting styles to suit programmers coming from Python, C, Rust, or JavaScript:

```alya
# Python-style single line comment
// C-style single line comment

/*
   C-style multiline block comment
   spanning multiple lines.
*/
```

---

## 2. Variables & Primitive Types

Variables are bound using the `let` keyword. Types are statically inferred by the compiler without requiring verbose type annotations:

```alya
let integer_val = 42          # 64-bit signed integer (i64)
let float_val   = 3.14159     # 64-bit IEEE 754 double precision float (f64)
let string_val  = "Alya"      # Heap-allocated UTF-8 string
let boolean_val = 1           # 1 (true) or 0 (false)
let empty_val   = null        # Null literal representing absence of value
```

---

## 3. Operators & Expressions

### Arithmetic & Assignment
| Operator | Description | Example |
|---|---|---|
| `+` | Addition (or string concatenation) | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division (with zero-division protection) | `a / b` |
| `%` | Modulo | `a % b` |
| `+=`, `-=`, `*=`, `/=` | In-place compound assignments | `count += 1` |

### Bitwise Operators
Alya provides full 64-bit bitwise manipulation with matching compound assignment operators:

| Operator | Description | Compound | Example |
|---|---|---|---|
| `&` | Bitwise AND | `&=` | `flags & 0xFF` |
| `\|` | Bitwise OR | `\|=` | `mode \| 0x02` |
| `^` | Bitwise XOR | `^=` | `mask ^ 0x0F` |
| `~` | Bitwise NOT | — | `~flags` |
| `<<` | Shift left | `<<=` | `1 << 8` |
| `>>` | Shift right | `>>=` | `1024 >> 2` |

### Null Coalescing (`??`)
Returns the right-hand operand when the left-hand operand is `null`:
```alya
let custom_port = null
let port = custom_port ?? 8080   # Evaluates to 8080
```

### Ternary & Inline Conditional
Concise inline branching:
```alya
let score = 85
let status = score >= 50 ? "Passed" : "Failed"

# Equivalent inline if-expression
let label = if score >= 50 then "Passed" else "Failed"
```

### Comparison & Logic
* **Relational**: `==`, `!=`, `<`, `<=`, `>`, `>=` (also supports `val == null` and `val != null`)
* **Logical**: `and` (or `&&`), `or` (or `||`), `not` (or `!`)

---

## 4. Input & Output

### `say` Statement
Outputs values to standard output followed by a newline. Non-string types (numbers, floats, booleans) are automatically converted to strings:
```alya
say "Hello"
say 42
say 3.14
```

### `ask` Expression
Prompts the user for interactive console input and returns the typed string:
```alya
let name = ask "Enter your name: "
say "Hello, " + name
```

### Type Conversions & Numeric Parsing
Since `ask` returns a string, use built-in functions to convert between strings and numbers without requiring imports:
* `int(x)` / `to_int(x)` / `parse_int(x)`: Parses a string into a 64-bit integer, or converts float to int.
* `float(x)` / `to_float(x)` / `parse_float(x)`: Parses a string into a 64-bit float, or converts int to float.
* `str(x)`: Converts any numeric value to its string representation.

```alya
let raw_qty = ask "Enter quantity: "
let qty = int(raw_qty)              # "5" -> 5

let raw_price = ask "Enter price: "
let price = float(raw_price)        # "19.95" -> 19.95

let subtotal = price * float(qty)
say "Total: ${subtotal}"
```

---

## 5. String Interpolation & Built-ins

Expressions enclosed in `{}` inside double quotes are evaluated and formatted automatically:
```alya
let user = "Alice"
let score = 95
say "Player {user} scored {score} points!"
```

### Multiline & Raw Strings
Alya supports standard quoted strings, triple-quoted multiline strings, and backtick raw strings:

```alya
# Multiline string (preserves formatting and newlines)
let banner = """
*-------------------*
| Welcome to Alya!  |
*-------------------*
"""
say banner

# Raw string (disables backslash escape interpretation)
let raw_path = `C:\Program Files\Alya\bin`
say raw_path
```

### Escape Sequences
Standard string literals support:
* `\n`: Newline
* `\t`: Tab
* `\r`: Carriage return
* `\"`: Double quote
* `\\`: Backslash
* `\e` or `\x1b`: ANSI escape code (for terminal styling)

### Built-in String Helpers
```alya
let text = "   Hello Alya World   "

say text.trim()                 # "Hello Alya World"
say text.trim().upper()         # "HELLO ALYA WORLD"
say text.trim().lower()         # "hello alya world"
say text.contains("Alya")       # 1
say text.trim().substring(0, 5) # "Hello"

# Splitting and joining
let parts = "red,green,blue".split(",")
say parts.join(" - ")           # "red - green - blue"
```

---

## 6. Progressive Examples

### Level 1: Pure & Minimal (Basic Math & Output)
```alya
let a = 15
let b = 4

let sum  = a + b
let diff = a - b
let prod = a * b
let quot = a / b
let rem  = a % b

say sum     # 19
say diff    # 11
say prod    # 60
say quot    # 3
say rem     # 3
```

---

### Level 2: Practical & Idiomatic (Interactive Order & Tax Calculator)
```alya
let item_name = ask "Enter product name: "
let raw_price = ask "Enter unit price ($): "
let raw_qty   = ask "Enter quantity: "

let price = float(raw_price)
let qty   = int(raw_qty)

let subtotal = price * float(qty)
let tax_rate = 0.08
let tax      = subtotal * tax_rate
let total    = subtotal + tax

say "-----------------------------"
say "Invoice Summary for: {item_name}"
say "Quantity: {qty} @ ${price}"
say "Subtotal: ${subtotal}"
say "Tax (8%): ${tax}"
say "Total Due: ${total}"
say "-----------------------------"
```

---

### Level 3: Advanced & Real-World (Character Stream Classifier & Sanitizer)
A robust utility that categorizes characters in a string, removes non-alphanumeric symbols, and computes checksum codes:

```alya
let input_text = "User_Account#2026-X!"
let clean_text = ""
let digits_count = 0
let letters_count = 0
let ascii_sum = 0

let i = 0
let total_len = len(input_text)

while i < total_len
    let ch = input_text[i]
    let code = ord(ch)
    ascii_sum += code

    if is_alpha(ch)
        clean_text += ch
        letters_count += 1
    elif is_digit(ch)
        clean_text += ch
        digits_count += 1
    else
        # Replace non-alphanumeric symbols with underscore
        clean_text += "_"
    end

    i += 1
end

say "Original Text: {input_text}"
say "Sanitized:     {clean_text}"
say "Alphabetic:    {letters_count} chars"
say "Numeric:       {digits_count} chars"
say "ASCII Checksum:{ascii_sum}"
```
