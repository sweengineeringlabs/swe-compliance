# Feature Spec: Rule Loading

**Version:** 1.0
**Status:** Draft
**Section:** 4.1

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-100 | Default rules embedded in binary | Must | Test | When no `--rules` flag is provided, the engine loads rules from the embedded default and produces a valid `ScanReport` with 128 check results |
| REQ-002 | FR-101 | External rules file override | Must | Test | When `--rules custom.toml` is provided, only rules in `custom.toml` are executed; embedded defaults are ignored |
| REQ-003 | FR-102 | TOML rules schema | Must | Inspection | The TOML parser accepts all fields below without error; missing required fields produce exit code 2 |
| REQ-004 | FR-103 | Declarative rule types | Must | Test | Each of the 9 rule types produces correct Pass/Fail results when tested against a fixture project |
| REQ-005 | FR-104 | Builtin rule types | Must | Test | Each handler produces correct results when tested against compliant and non-compliant fixture projects |
| REQ-006 | FR-105 | Unknown handler error | Must | Test | A rules file with `handler = "nonexistent"` produces exit code 2 and a message naming the unknown handler |

## Acceptance Criteria

- **REQ-001** (FR-100): When no `--rules` flag is provided, the engine loads rules from the embedded default and produces a valid `ScanReport` with 128 check results
- **REQ-002** (FR-101): When `--rules custom.toml` is provided, only rules in `custom.toml` are executed; embedded defaults are ignored
- **REQ-003** (FR-102): The TOML parser accepts all fields below without error; missing required fields produce exit code 2
- **REQ-004** (FR-103): Each of the 9 rule types produces correct Pass/Fail results when tested against a fixture project
- **REQ-005** (FR-104): Each handler produces correct results when tested against compliant and non-compliant fixture projects
- **REQ-006** (FR-105): A rules file with `handler = "nonexistent"` produces exit code 2 and a message naming the unknown handler

