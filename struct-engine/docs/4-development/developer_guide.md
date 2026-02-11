# struct-engine Developer Guide

**Audience**: Developers

## Prerequisites

- Rust 2021 edition (1.56+)
- Cargo

## Build

```bash
cargo build -p struct-engine
```

## Test

```bash
# All tests (unit + integration + e2e)
cargo test -p struct-engine

# Specific test target
cargo test --test struct_engine_int_test
cargo test --test struct_engine_e2e_test
```

## Run

```bash
# Scan a project (text output)
struct-engine scan <PATH>

# JSON output
struct-engine scan <PATH> --json

# Selective checks
struct-engine scan <PATH> --checks 1-8

# Custom rules file
struct-engine scan <PATH> --rules custom.toml

# Override project kind
struct-engine scan <PATH> --kind library
```

## Project Structure

```
struct-engine/
├── Cargo.toml
├── config/
│   └── rules.toml          # Declarative rule definitions
├── main/src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── api/                 # Public types
│   ├── core/                # Implementation
│   │   ├── engine.rs        # Scan orchestration
│   │   ├── scanner.rs       # File discovery
│   │   ├── rules.rs         # TOML parsing
│   │   ├── reporter.rs      # Output formatting
│   │   ├── declarative.rs   # Generic TOML rule runner
│   │   └── builtins/        # Named Rust handlers
│   └── saf/                 # Public re-exports
├── tests/
│   ├── struct_engine_int_test.rs
│   └── struct_engine_e2e_test.rs
└── docs/
```

## Adding a Declarative Rule

Add a `[[rules]]` entry to `config/rules.toml`:

```toml
[[rules]]
id = 45
category = "structure"
description = "config/ directory exists"
severity = "info"
type = "dir_exists"
path = "config"
```

No Rust code changes needed.

## Adding a Builtin Rule

1. Implement the handler in `main/src/core/builtins/`
2. Register it in `main/src/core/builtins/mod.rs`
3. Add the TOML entry with `type = "builtin"` and `handler = "<name>"`

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All checks passed |
| 1 | One or more checks failed |
| 2 | Scan error (invalid path, bad rules file, unknown handler) |
