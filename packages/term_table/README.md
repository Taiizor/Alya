# term_table

A reusable, zero-dependency ANSI & Unicode table formatting package for **Alya**.

Provides standardized box-drawing borders (`┌`, `┬`, `┐`, `│`, `├`, `┼`, `┤`, `└`, `┴`, `┘`), cell truncation and padding, and ANSI color badge styling.

---

## Installation

Add to your `alya.toml`:

```toml
[dependencies]
term_table = { path = "../../packages/term_table" }
```

Then install and lock:
```bash
alyac install
```

---

## Usage

```alya
import "term_table" as table

let top = table::box_top(4, 3, 8, 31, 8)
let r = table::row("ID", "ST", "PRIORITY", "TITLE", "TAG")
let bot = table::box_bot(4, 3, 8, 31, 8)

say top
say r
say bot
```
