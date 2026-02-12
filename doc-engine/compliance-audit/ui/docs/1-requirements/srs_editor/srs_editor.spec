# Feature Spec: SRS Editor

**Version:** 1.0
**Status:** Draft
**Section:** 4.9

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-900 | Markdown editor | Should | Demonstration | The SRS editor provides a split-pane view: a code editor with markdown syntax highlighting on the left, and a rendered preview on the right; editing the markdown updates the preview within 500ms |
| REQ-002 | FR-901 | Doc-engine format validation | Should | Test | `POST /api/v1/editor/validate` with SRS markdown content invokes `parse_srs()` and returns validation results: `{"valid": true\|false, "domains": N, "requirements": N, "errors": [...]}` where errors include missing attribute table fields, duplicate FR IDs, and malformed section headings |
| REQ-003 | FR-902 | Requirement ID auto-generation | Could | Demonstration | When the user types `#### FR-` in a new domain section, the editor suggests the next sequential FR ID based on the domain's numbering range (e.g., FR-201 if the domain starts at FR-200 and FR-200 exists); accepting the suggestion inserts the complete requirement header with an empty attribute table |
| REQ-004 | FR-903 | SRS save and load | Should | Test | `PUT /api/v1/projects/{id}/srs` with SRS markdown content saves the document to the project's `docs/1-requirements/srs.md` path; `GET /api/v1/projects/{id}/srs` returns the current SRS content with 200, or 404 if no SRS exists |

## Acceptance Criteria

- **REQ-001** (FR-900): The SRS editor provides a split-pane view: a code editor with markdown syntax highlighting on the left, and a rendered preview on the right; editing the markdown updates the preview within 500ms
- **REQ-002** (FR-901): `POST /api/v1/editor/validate` with SRS markdown content invokes `parse_srs()` and returns validation results: `{"valid": true|false, "domains": N, "requirements": N, "errors": [...]}` where errors include missing attribute table fields, duplicate FR IDs, and malformed section headings
- **REQ-003** (FR-902): When the user types `#### FR-` in a new domain section, the editor suggests the next sequential FR ID based on the domain's numbering range (e.g., FR-201 if the domain starts at FR-200 and FR-200 exists); accepting the suggestion inserts the complete requirement header with an empty attribute table
- **REQ-004** (FR-903): `PUT /api/v1/projects/{id}/srs` with SRS markdown content saves the document to the project's `docs/1-requirements/srs.md` path; `GET /api/v1/projects/{id}/srs` returns the current SRS content with 200, or 404 if no SRS exists

