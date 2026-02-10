# Feature Spec: Spec File Parsing

**Version:** 1.0
**Status:** Draft
**Section:** 4.7

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-700 | Dual-format spec file parsing | Should | Test | Valid `.spec.yaml` files parse via serde_yaml; valid `.spec` markdown files have metadata extracted via regex; invalid files of either format produce a `SpecDiagnostic` with file path and error message |
| REQ-002 | FR-701 | Kind-based dispatch | Should | Test | A YAML file with `kind: brd` deserializes to `BrdSpec`; `kind: feature_request` to `FeatureRequestSpec`; `kind: architecture` to `ArchSpec`; `kind: test_plan` to `TestSpec`; `kind: deployment` to `DeploySpec`. A markdown `.spec` file produces a `MarkdownSpec` with extracted metadata. |
| REQ-003 | FR-702 | Spec file discovery | Should | Test | Given a project with spec files in `1-requirements/`, `3-design/`, `5-testing/`, and `6-deployment/`, all spec files of both formats are discovered, categorized by extension, and tagged with their format (YAML or markdown) |
| REQ-004 | FR-703 | Parse error reporting | Should | Test | A YAML file with a syntax error produces a `SpecDiagnostic` containing the file path, line number (if available), and a descriptive error message. A markdown spec missing required metadata headers produces a `SpecDiagnostic`. |
| REQ-005 | FR-704 | Markdown spec metadata extraction | Should | Test | A markdown `.spec` file with `**Version:** 0.1.0`, `**Status:** Draft`, and `**Related:** RS-001` headers has all three values extracted. A `.test` file with a `**Spec:**` header containing a linked name and path has the spec link extracted. |
| REQ-006 | FR-705 | Markdown test table parsing | Should | Test | A `.test` file with a markdown table containing ` |
| REQ-007 | FR-706 | Feature stem matching | Should | Test | Given `compiler_design.spec`, `compiler_design.arch`, `compiler_design.test`, `compiler_design.deploy`, all four files are linked as a single SDLC chain via the shared stem `compiler_design` |

## Acceptance Criteria

- **REQ-001** (FR-700): Valid `.spec.yaml` files parse via serde_yaml; valid `.spec` markdown files have metadata extracted via regex; invalid files of either format produce a `SpecDiagnostic` with file path and error message
- **REQ-002** (FR-701): A YAML file with `kind: brd` deserializes to `BrdSpec`; `kind: feature_request` to `FeatureRequestSpec`; `kind: architecture` to `ArchSpec`; `kind: test_plan` to `TestSpec`; `kind: deployment` to `DeploySpec`. A markdown `.spec` file produces a `MarkdownSpec` with extracted metadata.
- **REQ-003** (FR-702): Given a project with spec files in `1-requirements/`, `3-design/`, `5-testing/`, and `6-deployment/`, all spec files of both formats are discovered, categorized by extension, and tagged with their format (YAML or markdown)
- **REQ-004** (FR-703): A YAML file with a syntax error produces a `SpecDiagnostic` containing the file path, line number (if available), and a descriptive error message. A markdown spec missing required metadata headers produces a `SpecDiagnostic`.
- **REQ-005** (FR-704): A markdown `.spec` file with `**Version:** 0.1.0`, `**Status:** Draft`, and `**Related:** RS-001` headers has all three values extracted. A `.test` file with a `**Spec:**` header containing a linked name and path has the spec link extracted.
- **REQ-006** (FR-705): A `.test` file with a markdown table containing `
- **REQ-007** (FR-706): Given `compiler_design.spec`, `compiler_design.arch`, `compiler_design.test`, `compiler_design.deploy`, all four files are linked as a single SDLC chain via the shared stem `compiler_design`

