# doc-engine Testing Strategy

**Audience**: Developers, contributors

## Test Strategy

> Per ISO/IEC/IEEE 29119-3:2021

Requirements-based testing approach. Every compliance check handler has
corresponding unit tests for pass, fail, and skip cases. Integration and E2E
tests verify the full scan pipeline from CLI invocation to JSON/text output.

| Attribute | Value |
|-----------|-------|
| **Test Strategy** | Requirements-based |
| **Test Scope** | All 128 compliance check handlers, scan API, CLI interface, scaffold |
| **Entry Criteria** | Code compiles, `cargo check` passes |
| **Exit Criteria** | All tests pass, zero clippy warnings, self-compliance scan passes |

## Test Categories

| Category | Count | Location | Purpose |
|----------|-------|----------|---------|
| Unit | 373 | `src/core/builtins/*.rs`, `src/core/*.rs` | Handler logic, engine, rules, scanner |
| Integration | 34 | `tests/scan_api.rs` | Scan API with minimal project fixtures |
| Config filtering | 9 | `tests/config_filtering.rs` | `--checks`, `--type`, `--rules` filtering |
| Error handling | 3 | `tests/error_handling.rs` | Invalid paths, malformed rules |
| Report format | 5 | `tests/report_format.rs` | JSON and text output correctness |
| Scaffold integration | 81 | `tests/scaffold_int_test.rs` | Scaffold feature logic and integration |
| CLI E2E | 48 | `tests/cli_e2e.rs` | Binary invocation, flags, exit codes |
| Scaffold E2E | 44 | `tests/scaffold_e2e_test.rs` | Scaffold end-to-end with real filesystem |
| Main | 6 | `src/main.rs` | CLI argument parsing |
| Doc test | 1 | `src/lib.rs` | Crate-level Quick Start example |

## Test Pyramid

```
        ┌─────────┐
        │  E2E    │  92 tests — full binary, real filesystem
        │  (CLI)  │
       ─┼─────────┼─
       │Integration│  132 tests — scan API + config + errors + format + scaffold
       │           │
      ─┼───────────┼─
      │   Feature   │  379 tests — handler logic, engine, rules, scanner
      │             │
      └─────────────┘
```

## Coverage Targets

| Metric | Target | Current |
|--------|--------|---------|
| Check handlers | 100% of handlers have pass + fail + skip tests | Met |
| New checks | Every new check ID requires unit tests before merge | Enforced by review |
| Self-compliance | `cargo run -- scan --scope large doc-engine` exits 0 with 0 failures | 85/128 pass, 36 fail, 7 skip |
| Clippy | Zero warnings with `-D warnings` | Met |

## Test Procedures

| Procedure | Test Cases | Environment | Execution Order |
|-----------|-----------|-------------|-----------------|
| Smoke | `cargo check`, `cargo test --lib` | Local / CI | First |
| Full suite | `cargo test` (all 604 tests) | CI | After smoke |
| Lint | `cargo clippy -- -D warnings` | CI | Parallel with tests |
| Self-compliance | `cargo run -- scan doc-engine` | CI | After tests |
| Audit | `cargo-deny check` | CI | Parallel |

## Related Documents

- **Requirements**: [srs.md](../1-requirements/srs.md)
- **Architecture**: [architecture.md](../3-design/architecture.md)
- **Production Readiness**: [production_readiness.md](../6-deployment/production_readiness.md)
