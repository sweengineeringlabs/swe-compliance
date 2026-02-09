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
| **Test Scope** | All 78 compliance check handlers, scan API, CLI interface |
| **Entry Criteria** | Code compiles, `cargo check` passes |
| **Exit Criteria** | All tests pass, zero clippy warnings, self-compliance scan passes |

## Test Categories

| Category | Count | Location | Purpose |
|----------|-------|----------|---------|
| Unit | 213 | `src/core/builtins/*.rs`, `src/core/*.rs` | Handler logic, engine, rules, scanner |
| Integration | 17 | `tests/scan_api.rs` | Scan API with minimal project fixtures |
| Config filtering | 5 | `tests/config_filtering.rs` | `--checks`, `--type`, `--rules` filtering |
| Error handling | 3 | `tests/error_handling.rs` | Invalid paths, malformed rules |
| Report format | 5 | `tests/report_format.rs` | JSON and text output correctness |
| CLI E2E | 19 | `tests/cli_e2e.rs` | Binary invocation, flags, exit codes |
| Main | 6 | `src/main.rs` | CLI argument parsing |
| Doc test | 1 | `src/lib.rs` | Crate-level Quick Start example |

## Test Pyramid

```
        ┌─────────┐
        │  E2E    │  19 tests — full binary, real filesystem
        │  (CLI)  │
       ─┼─────────┼─
       │Integration│  30 tests — scan API + config + errors + format
       │           │
      ─┼───────────┼─
      │    Unit     │  219 tests — handler logic, engine, rules, scanner
      │             │
      └─────────────┘
```

## Coverage Targets

| Metric | Target | Current |
|--------|--------|---------|
| Check handlers | 100% of handlers have pass + fail + skip tests | Met |
| New checks | Every new check ID requires unit tests before merge | Enforced by review |
| Self-compliance | `cargo run -- scan doc-engine` exits 0 with 0 failures | Met (76/78 pass, 2 skip) |
| Clippy | Zero warnings with `-D warnings` | Met |

## Test Procedures

| Procedure | Test Cases | Environment | Execution Order |
|-----------|-----------|-------------|-----------------|
| Smoke | `cargo check`, `cargo test --lib` | Local / CI | First |
| Full suite | `cargo test` (all 269 tests) | CI | After smoke |
| Lint | `cargo clippy -- -D warnings` | CI | Parallel with tests |
| Self-compliance | `cargo run -- scan doc-engine` | CI | After tests |
| Audit | `cargo-deny check` | CI | Parallel |

## Related Documents

- **Requirements**: [requirements.md](../1-requirements/requirements.md)
- **Architecture**: [architecture.md](../3-design/architecture.md)
- **Production Readiness**: [production_readiness.md](../6-deployment/production_readiness.md)
