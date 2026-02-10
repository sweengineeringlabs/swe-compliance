# Feature Spec: Spec Subcommand

**Version:** 1.0
**Status:** Draft
**Section:** 4.12

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-750 | Spec validate subcommand | Should | Demonstration | `doc-engine spec validate <PATH>` validates all spec files under PATH and prints schema diagnostics |
| REQ-002 | FR-751 | Spec cross-ref subcommand | Should | Demonstration | `doc-engine spec cross-ref <PATH>` analyzes cross-references and prints a categorized report |
| REQ-003 | FR-752 | Spec generate subcommand | Should | Demonstration | `doc-engine spec generate <FILE> --output <DIR>` produces a markdown file from the given YAML spec |
| REQ-004 | FR-753 | Spec subcommand exit codes | Should | Test | Exit code 0 = clean, 1 = violations found, 2 = error (same semantics as `scan`) |
| REQ-005 | FR-754 | Spec JSON output | Should | Test | `doc-engine spec validate <PATH> --json` outputs valid JSON deserializable to `SpecValidationReport`; `doc-engine spec cross-ref <PATH> --json` outputs valid JSON deserializable to `CrossRefReport` |
| REQ-006 | FR-755 | Spec text output | Should | Demonstration | Default (no `--json`) output is human-readable text with file paths, diagnostic messages, and a summary line |

## Acceptance Criteria

- **REQ-001** (FR-750): `doc-engine spec validate <PATH>` validates all spec files under PATH and prints schema diagnostics
- **REQ-002** (FR-751): `doc-engine spec cross-ref <PATH>` analyzes cross-references and prints a categorized report
- **REQ-003** (FR-752): `doc-engine spec generate <FILE> --output <DIR>` produces a markdown file from the given YAML spec
- **REQ-004** (FR-753): Exit code 0 = clean, 1 = violations found, 2 = error (same semantics as `scan`)
- **REQ-005** (FR-754): `doc-engine spec validate <PATH> --json` outputs valid JSON deserializable to `SpecValidationReport`; `doc-engine spec cross-ref <PATH> --json` outputs valid JSON deserializable to `CrossRefReport`
- **REQ-006** (FR-755): Default (no `--json`) output is human-readable text with file paths, diagnostic messages, and a summary line

