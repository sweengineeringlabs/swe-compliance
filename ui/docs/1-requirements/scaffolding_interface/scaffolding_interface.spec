# Feature Spec: Scaffolding Interface

**Version:** 1.0
**Status:** Draft
**Section:** 4.5

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-500 | Upload SRS document | Should | Test | `POST /api/v1/scaffold/parse` with the SRS markdown content in the request body returns a JSON representation of extracted domains and requirements (matching `SrsDomain` and `SrsRequirement` structures: section, title, slug, requirements with id, title, kind, priority, state, verification, traces_to, acceptance, description) |
| REQ-002 | FR-501 | Scaffold preview | Should | Demonstration | After uploading an SRS, the UI displays a tree of files that will be generated: per-domain `.spec.yaml`, `.spec`, `.arch.yaml`, `.arch`, `.test.yaml`, `.test`, `.manual.exec`, `.auto.exec`, `.deploy.yaml`, `.deploy`, plus BRD files; each file shows its target path under the output directory |
| REQ-003 | FR-502 | Execute scaffold | Should | Test | `POST /api/v1/scaffold/execute` with `{"project_id": "uuid", "srs_content": "...", "phases": ["requirements", "testing"], "force": false}` invokes `scaffold_from_srs()` and returns a `ScaffoldResult` JSON (standard, clause, tool, tool_version, timestamp, srs_source, phases, force, domain_count, requirement_count, created, skipped) |
| REQ-004 | FR-503 | Phase and type filters | Should | Test | The scaffold UI provides checkboxes for SDLC phases (requirements, design, testing, deployment) and file types (yaml, spec, arch, test, exec, deploy); selections are passed to `ScaffoldConfig.phases` and `ScaffoldConfig.file_types` |
| REQ-005 | FR-504 | Scaffold progress monitoring | Could | Demonstration | During scaffold execution, the UI displays which files are being created (`+` prefix) and which are skipped (`~` prefix) in real-time; upon completion, a summary shows domain_count, requirement_count, created count, and skipped count |

## Acceptance Criteria

- **REQ-001** (FR-500): `POST /api/v1/scaffold/parse` with the SRS markdown content in the request body returns a JSON representation of extracted domains and requirements (matching `SrsDomain` and `SrsRequirement` structures: section, title, slug, requirements with id, title, kind, priority, state, verification, traces_to, acceptance, description)
- **REQ-002** (FR-501): After uploading an SRS, the UI displays a tree of files that will be generated: per-domain `.spec.yaml`, `.spec`, `.arch.yaml`, `.arch`, `.test.yaml`, `.test`, `.manual.exec`, `.auto.exec`, `.deploy.yaml`, `.deploy`, plus BRD files; each file shows its target path under the output directory
- **REQ-003** (FR-502): `POST /api/v1/scaffold/execute` with `{"project_id": "uuid", "srs_content": "...", "phases": ["requirements", "testing"], "force": false}` invokes `scaffold_from_srs()` and returns a `ScaffoldResult` JSON (standard, clause, tool, tool_version, timestamp, srs_source, phases, force, domain_count, requirement_count, created, skipped)
- **REQ-004** (FR-503): The scaffold UI provides checkboxes for SDLC phases (requirements, design, testing, deployment) and file types (yaml, spec, arch, test, exec, deploy); selections are passed to `ScaffoldConfig.phases` and `ScaffoldConfig.file_types`
- **REQ-005** (FR-504): During scaffold execution, the UI displays which files are being created (`+` prefix) and which are skipped (`~` prefix) in real-time; upon completion, a summary shows domain_count, requirement_count, created count, and skipped count

