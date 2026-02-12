# Feature Spec: Violation Browser

**Version:** 1.0
**Status:** Draft
**Section:** 4.4

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-400 | Violation list | Must | Demonstration | Given a completed scan, the violation browser displays all failed checks with: check ID, category, description, severity (error/warning/info), and each violation's file path and message; the data is derived from `CheckEntry` objects where `result.status == "fail"` |
| REQ-002 | FR-401 | Violation filtering | Must | Test | The violation browser supports filtering by: category (dropdown of 18 categories), severity (error/warning/info checkboxes), and free-text search on check description and violation message; filters are applied client-side and update the view without a new API call |
| REQ-003 | FR-402 | Violation sorting | Must | Demonstration | Violations can be sorted by: check ID (ascending/descending), severity (error first), category (alphabetical), and file path (alphabetical); the default sort is severity descending then check ID ascending |
| REQ-004 | FR-403 | Fix guidance display | Should | Demonstration | Clicking a violation expands a detail panel showing: the check's full description from `rules.toml`, the violation file path, the expected condition (from the rule definition), and a suggested fix action |
| REQ-005 | FR-404 | Violation export | Should | Test | `GET /api/v1/scans/{id}/violations?format=csv` returns a CSV file with columns: check_id, category, severity, file_path, message; `format=json` returns the violations array as JSON |

## Acceptance Criteria

- **REQ-001** (FR-400): Given a completed scan, the violation browser displays all failed checks with: check ID, category, description, severity (error/warning/info), and each violation's file path and message; the data is derived from `CheckEntry` objects where `result.status == "fail"`
- **REQ-002** (FR-401): The violation browser supports filtering by: category (dropdown of 18 categories), severity (error/warning/info checkboxes), and free-text search on check description and violation message; filters are applied client-side and update the view without a new API call
- **REQ-003** (FR-402): Violations can be sorted by: check ID (ascending/descending), severity (error first), category (alphabetical), and file path (alphabetical); the default sort is severity descending then check ID ascending
- **REQ-004** (FR-403): Clicking a violation expands a detail panel showing: the check's full description from `rules.toml`, the violation file path, the expected condition (from the rule definition), and a suggested fix action
- **REQ-005** (FR-404): `GET /api/v1/scans/{id}/violations?format=csv` returns a CSV file with columns: check_id, category, severity, file_path, message; `format=json` returns the violations array as JSON

