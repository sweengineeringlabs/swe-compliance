# Feature Spec: Spec Schema Validation

**Version:** 1.0
**Status:** Draft
**Section:** 4.8

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-710 | Required fields validation | Should | Test | A `.spec.yaml` missing `kind`, `schemaVersion`, or `title` produces a validation diagnostic. A markdown `.spec` missing `**Version:**` or `**Status:**` produces a validation diagnostic. |
| REQ-002 | FR-711 | BRD schema validation | Should | Test | A valid `brd.spec.yaml` with `kind: brd`, `schemaVersion`, `title`, `domains[]` (each with `name`, `specCount`, `specs[]`) passes validation |
| REQ-003 | FR-712 | Feature request schema validation | Should | Test | A valid `login.spec.yaml` with `kind: feature_request`, `id`, `title`, `status`, `priority`, `requirements[]` passes validation |
| REQ-004 | FR-713 | Architecture schema validation | Should | Test | A valid `.arch.yaml` with `kind: architecture`, `spec` (spec ID reference), `components[]` passes validation |
| REQ-005 | FR-714 | Test plan schema validation | Should | Test | A valid `.test.yaml` with `kind: test_plan`, `spec` (spec ID reference), `testCases[]` (each with `verifies` field) passes validation |
| REQ-006 | FR-715 | Deployment schema validation | Should | Test | A valid `.deploy.yaml` with `kind: deployment`, `spec` (spec ID reference), `environments[]` passes validation |
| REQ-007 | FR-716 | Duplicate ID detection | Should | Test | Two spec files (of either format) with the same spec ID produce a validation diagnostic listing both file paths |

## Acceptance Criteria

- **REQ-001** (FR-710): A `.spec.yaml` missing `kind`, `schemaVersion`, or `title` produces a validation diagnostic. A markdown `.spec` missing `**Version:**` or `**Status:**` produces a validation diagnostic.
- **REQ-002** (FR-711): A valid `brd.spec.yaml` with `kind: brd`, `schemaVersion`, `title`, `domains[]` (each with `name`, `specCount`, `specs[]`) passes validation
- **REQ-003** (FR-712): A valid `login.spec.yaml` with `kind: feature_request`, `id`, `title`, `status`, `priority`, `requirements[]` passes validation
- **REQ-004** (FR-713): A valid `.arch.yaml` with `kind: architecture`, `spec` (spec ID reference), `components[]` passes validation
- **REQ-005** (FR-714): A valid `.test.yaml` with `kind: test_plan`, `spec` (spec ID reference), `testCases[]` (each with `verifies` field) passes validation
- **REQ-006** (FR-715): A valid `.deploy.yaml` with `kind: deployment`, `spec` (spec ID reference), `environments[]` passes validation
- **REQ-007** (FR-716): Two spec files (of either format) with the same spec ID produce a validation diagnostic listing both file paths

