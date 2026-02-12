# Feature Spec: Template Browser

**Version:** 1.0
**Status:** Draft
**Section:** 4.6

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-600 | List templates | Should | Test | `GET /api/v1/templates` returns a JSON array of template objects with name, sdlc_phase, description, and path fields; the list includes all templates from the configured template-engine templates directory |
| REQ-002 | FR-601 | Template preview with W3H structure | Should | Demonstration | Selecting a template displays its rendered markdown with W3H sections (WHO, WHAT, WHY, HOW) visually highlighted; the raw markdown is also available in a code view |
| REQ-003 | FR-602 | Template copy to project | Could | Test | `POST /api/v1/templates/{name}/copy` with `{"project_id": "uuid", "target_path": "docs/2-planning/quality_plan.md"}` copies the template to the project's directory; returns 201 on success or 409 if file already exists |
| REQ-004 | FR-603 | Compliance checklist display | Should | Demonstration | The template browser includes a dedicated checklist view that displays the 56-point compliance checklist from template-engine with each item's status (pass/fail/skip) mapped to the most recent doc-engine scan results for the selected project |

## Acceptance Criteria

- **REQ-001** (FR-600): `GET /api/v1/templates` returns a JSON array of template objects with name, sdlc_phase, description, and path fields; the list includes all templates from the configured template-engine templates directory
- **REQ-002** (FR-601): Selecting a template displays its rendered markdown with W3H sections (WHO, WHAT, WHY, HOW) visually highlighted; the raw markdown is also available in a code view
- **REQ-003** (FR-602): `POST /api/v1/templates/{name}/copy` with `{"project_id": "uuid", "target_path": "docs/2-planning/quality_plan.md"}` copies the template to the project's directory; returns 201 on success or 409 if file already exists
- **REQ-004** (FR-603): The template browser includes a dedicated checklist view that displays the 56-point compliance checklist from template-engine with each item's status (pass/fail/skip) mapped to the most recent doc-engine scan results for the selected project

