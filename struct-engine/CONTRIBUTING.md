# Contributing to struct-engine

## Getting Started

1. Clone the repository
2. Run `cargo build -p struct-engine`
3. Run `cargo test -p struct-engine`

## Development Workflow

- Create a feature branch from `main`
- Make changes following the existing code style
- Add tests for new functionality
- Run `cargo test -p struct-engine` before submitting
- Submit a pull request

## Adding Rules

**Declarative rules** (no Rust code): Add a `[[rules]]` entry to `config/rules.toml`.

**Builtin rules** (complex logic): Implement a handler in `main/src/core/builtins/`, register it in `mod.rs`, and add the TOML entry.

## Code Style

- Follow Rust 2021 edition conventions
- Use `cargo clippy` and `cargo fmt`
- Maintain SEA layer separation (API/Core/SAF)
