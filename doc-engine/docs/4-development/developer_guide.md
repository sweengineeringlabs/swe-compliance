# Developer Guide

**Audience**: Developers, contributors

## What

Guide for developing and extending doc-engine, a Rust CLI tool and library for documentation compliance checking.

## Why

New contributors need a clear entry point to understand the build process, project structure, and extension model for adding new compliance checks.

## How

### Build & Test

```bash
cargo build          # compile
cargo test           # run all tests (unit + integration + E2E)
cargo run -- scan .  # self-compliance check
```

### Adding a New Check

1. Add a `[[rules]]` entry to `rules.toml` with the next available ID
2. For declarative checks (file_exists, dir_exists, etc.), no code changes needed
3. For builtin checks, add a handler struct in `src/core/builtins/` implementing `CheckRunner`
4. Register the handler in `src/core/builtins/mod.rs` `get_handler()` match arm
5. Update test fixtures in `tests/common/mod.rs` if the minimal project needs new files
6. Update hardcoded check counts in test assertions

### Project Structure

- `src/api/` — Public API traits and types
- `src/spi/` — Service provider interface (CheckRunner, FileScanner)
- `src/core/` — Engine, rules parser, builtins, declarative checks
- `tests/` — Integration and E2E tests
- `rules.toml` — 66 compliance check definitions
