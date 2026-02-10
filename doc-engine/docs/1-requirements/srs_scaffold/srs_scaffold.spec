# Feature Spec: SRS Scaffold

**Version:** 1.0
**Status:** Draft
**Section:** 4.14

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-822 | Scaffold command | Should | Demonstration | `doc-engine scaffold <SRS_PATH> [--output DIR] [--force]` parses the SRS, extracts domains and requirements, and generates per-domain SDLC spec files; exit code 0 on success, 2 on error |
| REQ-002 | FR-823 | SRS domain extraction | Should | Test | The parser extracts `### X.Y Title` domain sections and `#### FR-NNN: Title` / `#### NFR-NNN: Title` requirement blocks with their attribute tables (Priority, State, Verification, Traces to, Acceptance); domains with no requirements are excluded |
| REQ-003 | FR-824 | Per-domain spec file generation | Should | Test | For each domain, generates 10 files: `.spec.yaml`, `.spec`, `.arch.yaml`, `.arch`, `.test.yaml`, `.test`, `.manual.exec`, `.auto.exec`, `.deploy.yaml`, `.deploy`; plus 2 BRD files (`brd.spec.yaml`, `brd.spec`); total = `domains × 10 + 2` |
| REQ-004 | FR-825 | Manual test execution plan | Should | Test | Each `.manual.exec` file contains a TLDR, a Test Cases table with TC, Test, Steps (`_TODO_`), and Expected (from acceptance criteria) columns, and an Execution Log table with TC, Tester, Date, Pass/Fail, Notes columns; all TCs are aligned row-for-row with `.test` and `.auto.exec` |
| REQ-005 | FR-826 | Automated test execution plan | Should | Test | Each `.auto.exec` file contains a TLDR, and a Test Cases table with TC, Test, Verifies, CI Job, Build, Status, Last Run columns; all TCs are aligned row-for-row with `.test` and `.manual.exec` |
| REQ-006 | FR-827 | Scaffold skip/force behavior | Should | Test | Without `--force`, existing files are skipped (not overwritten) and reported with `~` prefix; with `--force`, all files are overwritten and reported with `+` prefix |
| REQ-007 | FR-828 | Scaffold output directory | Should | Test | `--output DIR` specifies the output root directory; parent directories are created automatically; defaults to the current directory if not specified |

## Acceptance Criteria

- **REQ-001** (FR-822): `doc-engine scaffold <SRS_PATH> [--output DIR] [--force]` parses the SRS, extracts domains and requirements, and generates per-domain SDLC spec files; exit code 0 on success, 2 on error
- **REQ-002** (FR-823): The parser extracts `### X.Y Title` domain sections and `#### FR-NNN: Title` / `#### NFR-NNN: Title` requirement blocks with their attribute tables (Priority, State, Verification, Traces to, Acceptance); domains with no requirements are excluded
- **REQ-003** (FR-824): For each domain, generates 10 files: `.spec.yaml`, `.spec`, `.arch.yaml`, `.arch`, `.test.yaml`, `.test`, `.manual.exec`, `.auto.exec`, `.deploy.yaml`, `.deploy`; plus 2 BRD files (`brd.spec.yaml`, `brd.spec`); total = `domains × 10 + 2`
- **REQ-004** (FR-825): Each `.manual.exec` file contains a TLDR, a Test Cases table with TC, Test, Steps (`_TODO_`), and Expected (from acceptance criteria) columns, and an Execution Log table with TC, Tester, Date, Pass/Fail, Notes columns; all TCs are aligned row-for-row with `.test` and `.auto.exec`
- **REQ-005** (FR-826): Each `.auto.exec` file contains a TLDR, and a Test Cases table with TC, Test, Verifies, CI Job, Build, Status, Last Run columns; all TCs are aligned row-for-row with `.test` and `.manual.exec`
- **REQ-006** (FR-827): Without `--force`, existing files are skipped (not overwritten) and reported with `~` prefix; with `--force`, all files are overwritten and reported with `+` prefix
- **REQ-007** (FR-828): `--output DIR` specifies the output root directory; parent directories are created automatically; defaults to the current directory if not specified

