# Feature Spec: Reporting

**Version:** 1.0
**Status:** Draft
**Section:** 4.4

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-400 | Text output (default) | Must | Demonstration | Running without `--json` prints grouped results with check IDs, descriptions, statuses, violations, and a summary line |
| REQ-002 | FR-401 | JSON output | Must | Test | `--json` output parses as valid JSON and deserializes to `ScanReport` |
| REQ-003 | FR-402 | Exit codes | Must | Test | Clean project returns 0; project with violations returns 1; invalid path returns 2 |
| REQ-004 | FR-403 | Report file output | Should | Test | `--output <path>` writes a JSON audit report to the specified path, creating parent directories as needed; the recommended filename follows ISO/IEC/IEEE 15289:2019: `documentation_audit_report_v{version}.json` |
| REQ-005 | FR-831 | Audit status report (ISO 15289 clause 9.2) | Should | Test | `--output <path>` persists a JSON audit status report conforming to ISO/IEC/IEEE 15289:2019 clause 9.2; the report contains: standard, clause, tool, tool_version, timestamp (ISO 8601 UTC), project_root (absolute path), project_type, project_scope, results, summary |

## Acceptance Criteria

- **REQ-001** (FR-400): Running without `--json` prints grouped results with check IDs, descriptions, statuses, violations, and a summary line
- **REQ-002** (FR-401): `--json` output parses as valid JSON and deserializes to `ScanReport`
- **REQ-003** (FR-402): Clean project returns 0; project with violations returns 1; invalid path returns 2
- **REQ-004** (FR-403): `--output <path>` writes a JSON audit report to the specified path, creating parent directories as needed; the recommended filename follows ISO/IEC/IEEE 15289:2019: `documentation_audit_report_v{version}.json`
- **REQ-005** (FR-831): `--output <path>` persists a JSON audit status report conforming to ISO/IEC/IEEE 15289:2019 clause 9.2; the report contains: standard, clause, tool, tool_version, timestamp (ISO 8601 UTC), project_root (absolute path), project_type, project_scope, results, summary

