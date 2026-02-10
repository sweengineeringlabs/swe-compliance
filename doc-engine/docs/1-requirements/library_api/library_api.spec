# Feature Spec: Library API

**Version:** 1.0
**Status:** Draft
**Section:** 4.6

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-600 | Public scan function | Should | — | Removed: `scan()` convenience function was deleted when `ProjectScope` became mandatory. Use `scan_with_config()` (FR-601) instead. |
| REQ-002 | FR-601 | Configurable scan function | Should | Test | `doc_engine::scan_with_config(path, &config)` respects `ScanConfig` fields and returns `Result<ScanReport, ScanError>` |
| REQ-003 | FR-602 | Public types | Should | Inspection | All listed types are importable from `doc_engine::` |

## Acceptance Criteria

- **REQ-001** (FR-600): Removed: `scan()` convenience function was deleted when `ProjectScope` became mandatory. Use `scan_with_config()` (FR-601) instead.
- **REQ-002** (FR-601): `doc_engine::scan_with_config(path, &config)` respects `ScanConfig` fields and returns `Result<ScanReport, ScanError>`
- **REQ-003** (FR-602): All listed types are importable from `doc_engine::`

