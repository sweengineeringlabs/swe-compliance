# Feature Spec: Compliance Dashboard

**Version:** 1.0
**Status:** Draft
**Section:** 4.2

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-200 | Compliance overview | Must | Demonstration | The dashboard page displays for each project: project name, last scan date, total/passed/failed/skipped counts, and a compliance percentage (passed / total * 100); the data is sourced from `GET /api/v1/projects` |
| REQ-002 | FR-201 | Category breakdown chart | Must | Demonstration | Selecting a project displays a bar or stacked chart with pass/fail/skip counts per category (structure, naming, root_files, content, navigation, cross_ref, adr, traceability, ideation, requirements, planning, design, development, testing, deployment, operations, backlog, module) |
| REQ-003 | FR-202 | Trend over time | Should | Demonstration | `GET /api/v1/projects/{id}/trends?period=30d` returns an array of `{timestamp, passed, failed, skipped}` objects; the UI renders a line chart showing compliance score over the selected period |
| REQ-004 | FR-203 | Multi-engine summary | Should | Demonstration | For Rust projects, the dashboard shows both doc-engine (128 checks) and struct-engine (44 checks) results side by side with separate compliance scores and a combined total |

## Acceptance Criteria

- **REQ-001** (FR-200): The dashboard page displays for each project: project name, last scan date, total/passed/failed/skipped counts, and a compliance percentage (passed / total * 100); the data is sourced from `GET /api/v1/projects`
- **REQ-002** (FR-201): Selecting a project displays a bar or stacked chart with pass/fail/skip counts per category (structure, naming, root_files, content, navigation, cross_ref, adr, traceability, ideation, requirements, planning, design, development, testing, deployment, operations, backlog, module)
- **REQ-003** (FR-202): `GET /api/v1/projects/{id}/trends?period=30d` returns an array of `{timestamp, passed, failed, skipped}` objects; the UI renders a line chart showing compliance score over the selected period
- **REQ-004** (FR-203): For Rust projects, the dashboard shows both doc-engine (128 checks) and struct-engine (44 checks) results side by side with separate compliance scores and a combined total

