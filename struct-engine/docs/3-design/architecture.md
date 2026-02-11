# struct-engine Architecture

**Audience**: Developers, architects

## Overview

struct-engine is a single-crate Rust application following Single-Crate Modular SEA (Stratified Encapsulation Architecture). It audits Rust project structure against configurable compliance rules, supporting both declarative TOML-defined checks and builtin Rust handlers.

## Module Layout

```
main/src/
├── lib.rs              # Library root, declares modules
├── main.rs             # CLI binary (clap)
├── api/                # L1: Public types and traits
│   ├── mod.rs
│   └── types.rs        # ScanConfig, ScanReport, CheckResult, RuleDef, etc.
├── core/               # L0: Private implementations
│   ├── mod.rs
│   ├── engine.rs       # ComplianceEngine — orchestrates scanning
│   ├── scanner.rs      # FileScanner — recursive file discovery
│   ├── rules.rs        # TOML rule parsing, validation
│   ├── reporter.rs     # Text and JSON output formatting
│   ├── declarative.rs  # Generic runner for TOML-defined rule types
│   └── builtins/       # Named Rust handlers for complex checks
│       ├── mod.rs      # Handler registry
│       ├── cargo_toml.rs
│       ├── source_layout.rs
│       ├── test_org.rs
│       ├── naming.rs
│       └── documentation.rs
└── saf/                # L3: Surface API Facade (public re-exports)
    └── mod.rs          # scan(), scan_with_config(), format_report_*
```

## SEA Layers

| Layer | Visibility | Responsibility |
|-------|-----------|----------------|
| L3: SAF | `pub` | Re-exports for library consumers |
| L2: CLI | binary | `main.rs` — argument parsing, exit codes |
| L1: API | `pub` | Type definitions, traits |
| L0: Core | `pub(crate)` | All implementation logic |

**Dependency rule**: No layer depends on a layer above it. Core depends on API, not vice versa.

## Data Flow

```
CLI args / ScanConfig
        │
        ▼
    rules.rs          parse TOML → Vec<RuleDef>
        │
        ▼
    scanner.rs        walk project dir → Vec<PathBuf>
        │
        ▼
    engine.rs         for each rule:
        │               ├── declarative.rs  (TOML rule types)
        │               └── builtins/       (named handlers)
        │             → Vec<CheckEntry>
        ▼
    reporter.rs       format → text or JSON
        │
        ▼
    stdout + exit code (0/1/2)
```

## Rule System

**Declarative rules** are defined entirely in `config/rules.toml` and executed by `declarative.rs`. They support 11 rule types: `file_exists`, `dir_exists`, `dir_not_exists`, `file_content_matches`, `file_content_not_matches`, `glob_content_matches`, `glob_content_not_matches`, `glob_naming_matches`, `glob_naming_not_matches`, `cargo_key_exists`, `cargo_key_matches`.

**Builtin rules** reference a named Rust handler via `type = "builtin"` and `handler = "<name>"`. Used for checks requiring complex logic (e.g., Cargo.toml target path validation, test file organization analysis).

## Key Design Decisions

- **Single-crate modular** rather than multi-crate workspace — the engine is small enough that sub-crate overhead is not justified
- **Synchronous only** — no async runtime; file system operations are inherently sequential on a single disk
- **Embedded default rules** — `include_str!` bakes `rules.toml` into the binary for zero-config usage
- **44 checks across 7 categories** — structure, cargo_metadata, cargo_targets, naming, test_org, documentation, hygiene
