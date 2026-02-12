# Feature Spec: Project Management

**Version:** 1.0
**Status:** Draft
**Section:** 4.1

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-100 | Create project | Must | Test | `POST /api/v1/projects` with `{"name": "my-project", "root_path": "/path/to/project", "scope": "medium", "project_type": "open_source"}` returns 201 with a project ID; the project appears in `GET /api/v1/projects` |
| REQ-002 | FR-101 | List projects | Must | Test | `GET /api/v1/projects` returns a JSON array of project objects with id, name, root_path, scope, project_type, last_scan_timestamp, and compliance_summary fields |
| REQ-003 | FR-102 | Update project configuration | Must | Test | `PATCH /api/v1/projects/{id}` with `{"scope": "large"}` returns 200; subsequent `GET /api/v1/projects/{id}` reflects the updated scope |
| REQ-004 | FR-103 | Delete project | Must | Test | `DELETE /api/v1/projects/{id}` returns 204; subsequent `GET /api/v1/projects/{id}` returns 404; associated scan history is retained for audit trail |
| REQ-005 | FR-104 | Project configuration validation | Must | Test | `POST /api/v1/projects` with `{"scope": "invalid"}` returns 422 with a validation error message; `root_path` that does not exist on the server returns 422 |

## Acceptance Criteria

- **REQ-001** (FR-100): `POST /api/v1/projects` with `{"name": "my-project", "root_path": "/path/to/project", "scope": "medium", "project_type": "open_source"}` returns 201 with a project ID; the project appears in `GET /api/v1/projects`
- **REQ-002** (FR-101): `GET /api/v1/projects` returns a JSON array of project objects with id, name, root_path, scope, project_type, last_scan_timestamp, and compliance_summary fields
- **REQ-003** (FR-102): `PATCH /api/v1/projects/{id}` with `{"scope": "large"}` returns 200; subsequent `GET /api/v1/projects/{id}` reflects the updated scope
- **REQ-004** (FR-103): `DELETE /api/v1/projects/{id}` returns 204; subsequent `GET /api/v1/projects/{id}` returns 404; associated scan history is retained for audit trail
- **REQ-005** (FR-104): `POST /api/v1/projects` with `{"scope": "invalid"}` returns 422 with a validation error message; `root_path` that does not exist on the server returns 422

