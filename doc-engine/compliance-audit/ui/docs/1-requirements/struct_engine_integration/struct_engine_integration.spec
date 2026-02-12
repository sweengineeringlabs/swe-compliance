# Feature Spec: Struct-Engine Integration

**Version:** 1.0
**Status:** Draft
**Section:** 4.11

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-1100 | Struct-engine scan results | Should | Test | `GET /api/v1/scans/{id}` for a struct-engine scan returns the struct-engine `ScanReport` JSON (containing results with check_id, category, description, result per check across 7 categories: structure, cargo_metadata, cargo_targets, naming, test_org, documentation, hygiene) |
| REQ-002 | FR-1101 | Crate layout visualization | Could | Demonstration | The struct-engine results page includes a tree diagram showing the project's actual crate layout (src/, main/, tests/, benches/, examples/) with pass/fail indicators overlaid on directories that correspond to checked paths |
| REQ-003 | FR-1102 | Project kind display | Should | Demonstration | The struct-engine results display the detected project kind (Library, Binary, Both, Workspace) and indicate which checks were skipped due to kind filtering |

## Acceptance Criteria

- **REQ-001** (FR-1100): `GET /api/v1/scans/{id}` for a struct-engine scan returns the struct-engine `ScanReport` JSON (containing results with check_id, category, description, result per check across 7 categories: structure, cargo_metadata, cargo_targets, naming, test_org, documentation, hygiene)
- **REQ-002** (FR-1101): The struct-engine results page includes a tree diagram showing the project's actual crate layout (src/, main/, tests/, benches/, examples/) with pass/fail indicators overlaid on directories that correspond to checked paths
- **REQ-003** (FR-1102): The struct-engine results display the detected project kind (Library, Binary, Both, Workspace) and indicate which checks were skipped due to kind filtering

