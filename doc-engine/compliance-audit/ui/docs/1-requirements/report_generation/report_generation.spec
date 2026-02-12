# Feature Spec: Report Generation

**Version:** 1.0
**Status:** Draft
**Section:** 4.7

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-700 | JSON report export | Must | Test | `GET /api/v1/scans/{id}/report?format=json` returns the `ScanReport` JSON conforming to ISO/IEC/IEEE 15289:2019 clause 9.2 (containing standard, clause, tool, tool_version, timestamp, project_root, results, summary, project_type, project_scope) |
| REQ-002 | FR-701 | Markdown report export | Must | Test | `GET /api/v1/scans/{id}/report?format=markdown` returns a structured markdown document with sections for: executive summary, category breakdown, violation details, and ISO standard mapping; the Content-Type header is `text/markdown` |
| REQ-003 | FR-702 | PDF report export | Should | Test | `GET /api/v1/scans/{id}/report?format=pdf` returns a downloadable PDF with the same content as the markdown report rendered with headers, tables, and charts; the Content-Type header is `application/pdf` |
| REQ-004 | FR-703 | Historical report comparison | Should | Demonstration | The UI allows selecting two scan results for the same project; a diff view highlights checks that changed status between scans (e.g., fail-to-pass, pass-to-fail); new and removed checks are flagged |
| REQ-005 | FR-704 | Audit status report (ISO 15289) | Must | Test | `GET /api/v1/scans/{id}/audit-report` returns the full ISO/IEC/IEEE 15289:2019 clause 9.2 audit status report JSON, matching the structure produced by doc-engine's `--output` flag (standard, clause, tool, tool_version, timestamp, project_root, project_type, project_scope, results, summary) |

## Acceptance Criteria

- **REQ-001** (FR-700): `GET /api/v1/scans/{id}/report?format=json` returns the `ScanReport` JSON conforming to ISO/IEC/IEEE 15289:2019 clause 9.2 (containing standard, clause, tool, tool_version, timestamp, project_root, results, summary, project_type, project_scope)
- **REQ-002** (FR-701): `GET /api/v1/scans/{id}/report?format=markdown` returns a structured markdown document with sections for: executive summary, category breakdown, violation details, and ISO standard mapping; the Content-Type header is `text/markdown`
- **REQ-003** (FR-702): `GET /api/v1/scans/{id}/report?format=pdf` returns a downloadable PDF with the same content as the markdown report rendered with headers, tables, and charts; the Content-Type header is `application/pdf`
- **REQ-004** (FR-703): The UI allows selecting two scan results for the same project; a diff view highlights checks that changed status between scans (e.g., fail-to-pass, pass-to-fail); new and removed checks are flagged
- **REQ-005** (FR-704): `GET /api/v1/scans/{id}/audit-report` returns the full ISO/IEC/IEEE 15289:2019 clause 9.2 audit status report JSON, matching the structure produced by doc-engine's `--output` flag (standard, clause, tool, tool_version, timestamp, project_root, project_type, project_scope, results, summary)

