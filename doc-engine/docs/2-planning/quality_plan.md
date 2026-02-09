# Quality Plan

**Audience**: Developers, project stakeholders

## Quality Goals

- All compliance checks must have unit tests
- Integration tests cover the scan API with minimal and empty projects
- E2E tests verify CLI behavior, exit codes, and output formats
- Self-compliance: doc-engine must pass its own checks (0 failures)

## Test Strategy

| Level | Scope | Tool |
|-------|-------|------|
| Unit | Individual check handlers | `cargo test` (inline `#[cfg(test)]`) |
| Integration | `scan()` and `scan_with_config()` API | `tests/scan_api.rs`, `tests/config_filtering.rs` |
| E2E | CLI binary, exit codes, JSON/text output | `tests/cli_e2e.rs` (assert_cmd) |
| Self-compliance | doc-engine against its own docs | `cargo run -- scan .` |

## Acceptance Criteria

- `cargo test` passes all tests (0 failures)
- `cargo run -- scan .` reports 0 failures
- New checks include corresponding test coverage

See [architecture.md](../../docs/3-design/architecture.md) for system design context.
