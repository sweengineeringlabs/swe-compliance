# Feature Spec: CLI Interface

**Version:** 1.0
**Status:** Draft
**Section:** 4.5

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-500 | Scan command | Must | Demonstration | `doc-engine scan <PATH> --scope <TIER>` executes a scoped scan and prints results |
| REQ-002 | FR-501 | JSON flag | Must | Test | `doc-engine scan <PATH> --json` outputs valid JSON |
| REQ-003 | FR-502 | Check filter flag | Must | Test | `--checks 1-13` runs exactly 13 checks; `--checks 1,2,3,14-25` runs exactly 15 |
| REQ-004 | FR-503 | Project type flag | Must | Test | `--type internal` skips open-source-only rules; `--type open-source` is the default |
| REQ-005 | FR-504 | Rules file flag | Must | Test | `--rules custom.toml` loads and uses the specified file; missing file produces exit code 2 |
| REQ-006 | FR-505 | Scope flag | Must | Test | `--scope small` runs only small-tier checks, skipping medium and large; `--scope medium` runs small+medium; `--scope large` runs all; unknown values produce exit code 2 |
| REQ-007 | FR-506 | Output flag | Should | Test | `--output <path>` or `-o <path>` writes a JSON report to the specified path; parent directories are created automatically; the recommended filename is `documentation_audit_report_v{version}.json` per ISO/IEC/IEEE 15289:2019 |

## Acceptance Criteria

- **REQ-001** (FR-500): `doc-engine scan <PATH> --scope <TIER>` executes a scoped scan and prints results
- **REQ-002** (FR-501): `doc-engine scan <PATH> --json` outputs valid JSON
- **REQ-003** (FR-502): `--checks 1-13` runs exactly 13 checks; `--checks 1,2,3,14-25` runs exactly 15
- **REQ-004** (FR-503): `--type internal` skips open-source-only rules; `--type open-source` is the default
- **REQ-005** (FR-504): `--rules custom.toml` loads and uses the specified file; missing file produces exit code 2
- **REQ-006** (FR-505): `--scope small` runs only small-tier checks, skipping medium and large; `--scope medium` runs small+medium; `--scope large` runs all; unknown values produce exit code 2
- **REQ-007** (FR-506): `--output <path>` or `-o <path>` writes a JSON report to the specified path; parent directories are created automatically; the recommended filename is `documentation_audit_report_v{version}.json` per ISO/IEC/IEEE 15289:2019

