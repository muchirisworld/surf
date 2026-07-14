# Surf 🌊

A high-performance, command-line search utility written in Rust. `surf` recursively searches directories for file patterns using parallel execution, custom ignore lists, configuration presets, and context-aware rendering.

Built as an idiomatic exercise in systems engineering, `surf` prioritizes structured concurrency, zero-copy borrowing, clean diagnostic boundaries, and comprehensive test coverage.

---

## Features

- 🏎️ **Parallel Search**: Dynamically scales to system thread availability to parallelize search workloads across multiple cores.
- ⚙️ **Hierarchical Configuration**: Merges global defaults, local project settings (`.surf.toml`), and command-line arguments.
- 🚫 **Ignore Engines**: Traverses files recursively while respecting custom blocklists (e.g. `.surfignore` or specified ignore files).
- 🧩 **Advanced Matching Modes**:
  - Substring & Exact whole-line matching
  - Case-sensitivity toggle
  - Inverted matching (`-v`)
- 🔍 **Context-Aware Rendering**: Print matching lines with line numbers (`-n`), terminal colorization options, and before/after (`-B`/`-A`) context lines.
- 🛠️ **Robust Diagnostics**: Custom error types with clear exit code mapping (success, general failure, usage mistakes).

---

## Getting Started

### Prerequisites

You need Rust and Cargo installed. (Minimum supported edition: Rust 2024).

```bash
cargo --version
```

### Installation & Build

Clone the repository and build the binary in release mode:

```bash
cargo build --release
```

The resulting binary will be located at `target/release/surf`.

---

## Usage

```bash
surf [OPTIONS] <pattern> <path>...
```

### Examples

Search for "fn main" recursively in the current directory:
```bash
surf -r "fn main" .
```

Case-insensitive search with line numbers:
```bash
surf -i -n "todo" src/
```

Search while printing 2 lines of context before and 3 lines after each match:
```bash
surf -B 2 -A 3 "struct" src/
```

Invert matches to find lines *not* containing "use":
```bash
surf -v "use" src/main.rs
```

Explicitly supply an ignore file to bypass paths:
```bash
surf --ignore-file .gitignore "target" .
```

---

## Configuration (`.surf.toml`)

`surf` automatically looks for a `.surf.toml` configuration file in your directory to load default options. Overrides can still be supplied as CLI arguments.

**Example `.surf.toml`:**
```toml
recursive = true
line_numbers = true
ignore_case = true
color = "auto"
before_context = 1
after_context = 1
```

---

## Project Structure

```text
src/
 ├── main.rs          # Command line entry point
 ├── lib.rs           # Orchestrator & high-level runner pipeline
 ├── args.rs          # Token-based command-line argument parser
 ├── config.rs        # TOML configuration loader and settings merger
 ├── walk.rs          # Recursive file collector
 ├── ignore.rs        # IgnoreSet pattern builder & path filter
 ├── matcher.rs       # Single-line matching rules
 ├── search.rs        # Multi-line match extractor & context manager
 ├── parallel.rs      # Worker pool runner using Arc/Mutex task queues
 ├── render.rs        # Thread-safe terminal stdout formatter
 └── diagnostic.rs    # Structured error and exit-code designs
tests/
 └── cli.rs           # Process-level integration tests
```

---

## Running Tests

`surf` is backed by a robust suite of unit and integration tests verifying matching, ignore sets, argument parsing, and binary outputs.

```bash
cargo test
```
