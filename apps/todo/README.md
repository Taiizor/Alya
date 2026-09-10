# Alya Todo Manager (`todo`)

A sleek, persistent terminal task manager and productivity tracker built in **Alya**.

Designed with clean ANSI terminal UI, priority color coding, category tags, progress tracking, and persistent disk storage.

---

## Features

- **Persistent Disk Storage**: Saves your tasks into a local database file (`todo.db`) across terminal sessions.
- **Priority Badging**: Classify tasks by priority with distinctive ANSI badges:
  - 🔴 **HIGH**: Critical and urgent milestones
  - 🟡 **MED**: Normal tasks and features
  - 🟢 **LOW**: Nice-to-have improvements
- **Category Tagging**: Organize tasks by domain or team (e.g. `#core`, `#docs`, `#apps`, `#ui`).
- **Progress Tracking**: Real-time ASCII progress bar and completion rate metrics.
- **Productivity Dashboard**: Summary statistics breaking down completion percentages and priority distributions.
- **Dual Operational Modes**:
  - **CLI Mode**: Fast one-liner terminal commands for shell scripting and automation.
  - **Interactive REPL**: Focused, interactive command prompt (`alya-todo>`).
- **Zero Dependencies**: Powered entirely by the Alya Standard Library (`std/fs`, `std/str`, `std/color`, `std/console`, `std/os`, `std/time`).

---

## Quick Start

### 1. Run as a Package
```bash
# Inside apps/todo (automatically detects alya.toml and main.alya)
cd apps/todo
alyac run
```

### 2. Direct Compilation & Execution
```bash
# Interactive REPL shell
alyac run apps/todo/main.alya

# Add new tasks
alyac run apps/todo/main.alya -- add "Implement C FFI Engine" --pri high --tag core
alyac run apps/todo/main.alya -- add "Write LSP language server" --pri med --tag tooling
alyac run apps/todo/main.alya -- add "Refactor snake collision" --pri low --tag apps

# List all tasks
alyac run apps/todo/main.alya -- list

# Mark task #1 as done
alyac run apps/todo/main.alya -- done 1

# Filter pending tasks
alyac run apps/todo/main.alya -- list --pending

# View productivity metrics
alyac run apps/todo/main.alya -- stats

# Automated smoke test suite
alyac run apps/todo/main.alya -- --test
```

---

## Commands & Options

| Command | Arguments / Options | Description |
| :--- | :--- | :--- |
| `add` | `<title> [--pri high\|med\|low] [--tag <tag>]` | Add a new task with priority and tag |
| `list` | `[--all] [--pending] [--done] [--tag <tag>]` | Render formatted ANSI task table |
| `done` | `<id>` | Mark specified task as completed |
| `undone` | `<id>` | Reopen a completed task |
| `rm` / `delete` | `<id>` | Permanently remove a task |
| `clear` | *(none)* | Purge all completed tasks |
| `stats` | *(none)* | Show total tasks, completion percentage, and distribution |
| `help` | *(none)* | Display help banner and usage instructions |
| `exit` / `quit` | *(none)* | Exit interactive mode |

---

## Architecture & Source Code

- [`alya.toml`](alya.toml): Project manifest defining package name, version, and entry point.
- [`main.alya`](main.alya): Complete implementation including file serialization, ANSI table formatter, CLI argument parsing, and interactive loop.
