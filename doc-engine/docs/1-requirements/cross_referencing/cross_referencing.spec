# Feature Spec: Cross-Referencing

**Version:** 1.0
**Status:** Draft
**Section:** 4.9

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-720 | Dependency resolution | Should | Test | A `.spec.yaml` with `dependencies: [{ref: "FR-002", file: "auth/signup.spec.yaml"}]` passes if the referenced file exists and contains the referenced ID; fails if it does not |
| REQ-002 | FR-721 | SDLC chain completeness | Should | Test | A `.spec` with no matching `.test` produces a cross-ref diagnostic. A `.spec.yaml` with no matching `.test.yaml` produces a cross-ref diagnostic. Both formats are checked independently. |
| REQ-003 | FR-722 | BRD inventory accuracy | Should | Test | A BRD with `specCount: 3` for domain "auth" but only 2 actual spec files produces a diagnostic; all `specs[].file` paths must resolve |
| REQ-004 | FR-723 | Test traceability | Should | Test | A YAML test case with `verifies: "REQ-001"` passes if the linked spec contains that ID. A markdown `.test` table row with `Verifies` = `DESIGN-001` passes if the linked `.spec` defines that ID. |
| REQ-005 | FR-724 | Architecture traceability | Should | Test | An `.arch.yaml` with `spec: "FR-001"` passes if a matching `.spec.yaml` exists. A markdown `.arch` with a `**Spec:**` header containing a linked spec name passes if the linked `.spec` file exists. |
| REQ-006 | FR-725 | Related documents resolution | Should | Test | A spec with `relatedDocuments: ["../3-design/architecture.md"]` passes if the path resolves; fails if it does not |
| REQ-007 | FR-726 | Cross-reference report | Should | Test | `CrossRefReport` contains categorized results (dependency, sdlc_chain, inventory, test_trace, arch_trace, related_docs) with pass/fail per check |
| REQ-008 | FR-727 | Opt-in spec checking | Should | — | Superseded: IDs 54-68 are now phase artifact `file_exists` checks (FR-815). Spec checks use dedicated builtin handlers with their own IDs. |

## Acceptance Criteria

- **REQ-001** (FR-720): A `.spec.yaml` with `dependencies: [{ref: "FR-002", file: "auth/signup.spec.yaml"}]` passes if the referenced file exists and contains the referenced ID; fails if it does not
- **REQ-002** (FR-721): A `.spec` with no matching `.test` produces a cross-ref diagnostic. A `.spec.yaml` with no matching `.test.yaml` produces a cross-ref diagnostic. Both formats are checked independently.
- **REQ-003** (FR-722): A BRD with `specCount: 3` for domain "auth" but only 2 actual spec files produces a diagnostic; all `specs[].file` paths must resolve
- **REQ-004** (FR-723): A YAML test case with `verifies: "REQ-001"` passes if the linked spec contains that ID. A markdown `.test` table row with `Verifies` = `DESIGN-001` passes if the linked `.spec` defines that ID.
- **REQ-005** (FR-724): An `.arch.yaml` with `spec: "FR-001"` passes if a matching `.spec.yaml` exists. A markdown `.arch` with a `**Spec:**` header containing a linked spec name passes if the linked `.spec` file exists.
- **REQ-006** (FR-725): A spec with `relatedDocuments: ["../3-design/architecture.md"]` passes if the path resolves; fails if it does not
- **REQ-007** (FR-726): `CrossRefReport` contains categorized results (dependency, sdlc_chain, inventory, test_trace, arch_trace, related_docs) with pass/fail per check
- **REQ-008** (FR-727): Superseded: IDs 54-68 are now phase artifact `file_exists` checks (FR-815). Spec checks use dedicated builtin handlers with their own IDs.

