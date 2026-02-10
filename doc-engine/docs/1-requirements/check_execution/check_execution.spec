# Feature Spec: Check Execution

**Version:** 1.0
**Status:** Draft
**Section:** 4.3

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-300 | All checks (83 base + 15 spec) | Must | Test | Default `rules.toml` contains 128 rules; a full scan produces 128 check results |
| REQ-002 | FR-301 | Check filtering | Must | Test | `--checks 1-13` produces exactly 13 results; `--checks 1,2,3` produces exactly 3 |
| REQ-003 | FR-302 | Project type filtering | Must | Test | Rules with `project_type = "open_source"` are skipped when `--type internal` is used, and vice versa |
| REQ-004 | FR-303 | Check result types | Must | Inspection | The `CheckResult` enum has exactly three variants: Pass, Fail (with violations), Skip (with reason) |
| REQ-005 | FR-304 | Violation record | Must | Inspection | Each `Violation` contains check ID, optional file path, message, and severity |

## Acceptance Criteria

- **REQ-001** (FR-300): Default `rules.toml` contains 128 rules; a full scan produces 128 check results
- **REQ-002** (FR-301): `--checks 1-13` produces exactly 13 results; `--checks 1,2,3` produces exactly 3
- **REQ-003** (FR-302): Rules with `project_type = "open_source"` are skipped when `--type internal` is used, and vice versa
- **REQ-004** (FR-303): The `CheckResult` enum has exactly three variants: Pass, Fail (with violations), Skip (with reason)
- **REQ-005** (FR-304): Each `Violation` contains check ID, optional file path, message, and severity

