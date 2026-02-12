# Feature Spec: Scan Execution

**Version:** 1.0
**Status:** Draft
**Section:** 4.3

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-300 | Trigger doc-engine scan | Must | Test | `POST /api/v1/scans` with `{"project_id": "uuid", "engine": "doc-engine", "scope": "medium"}` returns 202 with `{"scan_id": "uuid", "status": "queued"}`; the scan runs asynchronously using `doc_engine_scan::scan_with_config()` |
| REQ-002 | FR-301 | Trigger struct-engine scan | Should | Test | `POST /api/v1/scans` with `{"project_id": "uuid", "engine": "struct-engine"}` returns 202; the scan runs using `struct_engine::scan_with_config()` |
| REQ-003 | FR-302 | Real-time scan progress | Must | Demonstration | Connecting to `WS /api/v1/scans/{id}/progress` receives JSON messages with `{"check_id": N, "category": "...", "status": "pass\|fail\|skip"}` as each check completes; the UI updates a progress bar showing completed/total checks |
| REQ-004 | FR-303 | Scan result retrieval | Must | Test | `GET /api/v1/scans/{id}` returns the full `ScanReport` JSON (matching doc-engine's `ScanReport` struct: standard, clause, tool, tool_version, timestamp, project_root, results, summary, project_type, project_scope) with HTTP 200 when complete, or `{"status": "in_progress"}` with HTTP 200 when still running |
| REQ-005 | FR-304 | Scan filtering options | Should | Test | `POST /api/v1/scans` accepts optional `checks` (e.g., "1-13"), `phase` (e.g., "testing,module"), and `module` (e.g., "scan,cli") fields; these are passed through to `ScanConfig` |
| REQ-006 | FR-305 | Scan history | Must | Test | `GET /api/v1/projects/{id}/scans` returns an array of past scan summaries ordered by timestamp descending, each containing scan_id, timestamp, engine, summary (passed/failed/skipped), and scope |

## Acceptance Criteria

- **REQ-001** (FR-300): `POST /api/v1/scans` with `{"project_id": "uuid", "engine": "doc-engine", "scope": "medium"}` returns 202 with `{"scan_id": "uuid", "status": "queued"}`; the scan runs asynchronously using `doc_engine_scan::scan_with_config()`
- **REQ-002** (FR-301): `POST /api/v1/scans` with `{"project_id": "uuid", "engine": "struct-engine"}` returns 202; the scan runs using `struct_engine::scan_with_config()`
- **REQ-003** (FR-302): Connecting to `WS /api/v1/scans/{id}/progress` receives JSON messages with `{"check_id": N, "category": "...", "status": "pass|fail|skip"}` as each check completes; the UI updates a progress bar showing completed/total checks
- **REQ-004** (FR-303): `GET /api/v1/scans/{id}` returns the full `ScanReport` JSON (matching doc-engine's `ScanReport` struct: standard, clause, tool, tool_version, timestamp, project_root, results, summary, project_type, project_scope) with HTTP 200 when complete, or `{"status": "in_progress"}` with HTTP 200 when still running
- **REQ-005** (FR-304): `POST /api/v1/scans` accepts optional `checks` (e.g., "1-13"), `phase` (e.g., "testing,module"), and `module` (e.g., "scan,cli") fields; these are passed through to `ScanConfig`
- **REQ-006** (FR-305): `GET /api/v1/projects/{id}/scans` returns an array of past scan summaries ordered by timestamp descending, each containing scan_id, timestamp, engine, summary (passed/failed/skipped), and scope

