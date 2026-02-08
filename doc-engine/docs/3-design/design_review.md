# doc-engine Design Review

**Audience**: Developers, architects

**Date**: 2026-02-08
**Reviewers**: Claude Opus 4.6 (automated), project architect
**Status**: Resolved — all findings addressed in requirements.md, architecture.md, implementation_plan.md

---

## Review Scope

Second-pass cross-document consistency and design review of:
- `docs/1-requirements/requirements.md` (v1.0)
- `docs/2-planning/implementation_plan.md`
- `docs/3-design/architecture.md`

## Findings

### DR-01: ScanReport lacks check metadata for reporter (Must)

**Problem**: `ScanReport` contains `Vec<(CheckId, CheckResult)>` but no category or description. The `Reporter` trait receives only `&ScanReport`. `TextReporter` (FR-400) must "group results by category" and "show check ID, description, pass/fail/skip" — impossible without metadata.

**Decision**: Enrich `ScanReport` with a `CheckEntry` struct containing `id`, `category`, `description`, and `result`. The report becomes self-contained and the reporter needs no external state.

**Resolution**: Architecture types updated. `ScanReport.results` changed from `Vec<(CheckId, CheckResult)>` to `Vec<CheckEntry>`.

### DR-02: Spec discovery triggers second walkdir traversal (Must)

**Problem**: NFR-201 requires "exactly one `walkdir` traversal per scan invocation." `DocSpecEngine` "reuses `FileSystemScanner` for file discovery," which would call `scan_files()` again — a second traversal. Spec builtin handlers (checks 51-65) run during a normal scan and delegate to `DocSpecEngine`.

**Decision**: Spec handlers must filter `ScanContext.files` (already walked) by extension rather than re-walking. The `DocSpecEngine` reuses the existing file list when invoked from scan pipeline checks. It only performs its own `walkdir` traversal when invoked standalone via `spec validate` / `spec cross-ref` subcommands.

**Resolution**: Architecture spec module documentation updated to clarify dual-mode file sourcing.

### DR-03: Architecture handler table missing checklist_completeness (Must)

**Problem**: `checklist_completeness` was added to requirements.md (FR-104) and implementation_plan.md (Phase 6) but omitted from architecture.md's builtin handlers table.

**Resolution**: Handler added to architecture.md builtin handlers table and structure.rs module description.

### DR-04: FR-100 and OS-1 reference "50 checks" instead of 65 (Must)

**Problem**: FR-100 acceptance criterion says "50 check results" and OS-1 says "runs all 50 checks." The default `rules.toml` contains 65 entries per FR-300 and FR-740.

**Resolution**: Both updated to reference 65 checks.

### DR-05: scan() returns ScanReport not Result — error path undefined (Must)

**Problem**: `ComplianceEngine::scan()` returns `ScanReport`, not `Result<ScanReport, ScanError>`. FR-402 requires exit code 2 for invalid paths and IO errors. The library API had no way to signal errors.

**Decision**: Change `ComplianceEngine` trait methods and SAF functions to return `Result<ScanReport, ScanError>`. CLI maps `Ok` to exit codes 0/1 (based on failures), `Err` to exit code 2.

**Resolution**: Architecture trait signatures, SAF function signatures, and requirements FR-600/FR-601 acceptance criteria updated.

### DR-06: spec_schema_valid parameterization undocumented (Should)

**Problem**: Handler `spec_schema_valid` maps to checks 53-56, each validating a different extension pair. Four `rules.toml` entries share the same handler name but have different IDs. The dispatch mechanism was undocumented.

**Decision**: The handler dispatches by `CheckId`. When constructed from a `RuleDef`, it receives the check ID. A match table maps: 53 -> `.spec`/`.spec.yaml`, 54 -> `.arch`/`.arch.yaml`, 55 -> `.test`/`.test.yaml`, 56 -> `.deploy`/`.deploy.yaml`.

**Resolution**: Dispatch mechanism documented in architecture spec handler section.

### DR-07: RuleSet type referenced but undefined (Should)

**Problem**: Phase 2 lists `RuleSet` as a type to create. Phase 3 says "Parse TOML into `RuleSet`." But the architecture type definitions only show `RuleDef` and `Vec<RuleDef>`.

**Decision**: Define `RuleSet` as a thin wrapper: `struct RuleSet { pub rules: Vec<RuleDef> }`. Provides a named type for the parsed rules collection rather than a bare `Vec`.

**Resolution**: `RuleSet` added to architecture API types section.

### DR-08: Check 64 glob can't cover both format extensions (Should)

**Problem**: Check 64 was defined as declarative `glob_naming_matches`, but a single glob pattern cannot cover both `*.spec` and `*.spec.yaml` across 4 extension pairs (8 patterns). The Rust `glob` crate does not support brace expansion.

**Decision**: Change check 64 to builtin handler `spec_naming_convention`. The handler filters already-discovered spec files (from `ScanContext.files`) and validates their filenames match `[a-z_]+` snake_lower_case convention. This is consistent with how all other spec checks work and avoids glob limitations.

**Resolution**: Check 64 changed from declarative to builtin across all three documents.

### DR-09: Architecture scaffold "10 spec check handlers" should be "11" (Should)

**Problem**: Scaffold comment on `builtins/spec.rs` said "10 spec check handlers" but Phase 15 count was already corrected to 11. With check 64 now also a builtin (DR-08), the count becomes 12.

**Resolution**: Updated to "12 spec check handlers" (11 original + spec_naming_convention).

---

## Summary

| ID | Severity | Finding | Decision |
|----|----------|---------|----------|
| DR-01 | Must | ScanReport has no category/description | Add `CheckEntry` struct to report |
| DR-02 | Must | Spec discovery violates NFR-201 single pass | Filter ScanContext.files; own walk only for standalone |
| DR-03 | Must | Architecture missing checklist_completeness | Add to handler table |
| DR-04 | Must | "50 checks" should be "65" | Update counts |
| DR-05 | Must | scan() can't signal errors | Return `Result<ScanReport, ScanError>` |
| DR-06 | Should | spec_schema_valid dispatch undocumented | Dispatch by CheckId, document match table |
| DR-07 | Should | RuleSet type undefined | Define as `struct RuleSet { rules: Vec<RuleDef> }` |
| DR-08 | Should | Check 64 glob can't cover both formats | Change to builtin `spec_naming_convention` |
| DR-09 | Should | Scaffold handler count wrong | Update to 12 |
