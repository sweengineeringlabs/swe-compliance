# struct-engine

Rust package structure compliance engine auditing source layout conventions.

struct-engine scans Rust projects against configurable compliance rules derived from the `{main,tests}` layout, SEA layering, Cargo target paths, naming conventions, and test organization standards.

## Quick Start

```bash
# Scan current project
struct-engine scan .

# JSON output for CI
struct-engine scan . --json

# Selective checks
struct-engine scan . --checks 1-8
```

## Documentation

See [docs/README.md](docs/README.md) for full documentation.
