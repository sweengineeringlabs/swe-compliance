# doc-engine Backlog

**Audience**: Developers, architects, project stakeholders

## TLDR

All 17 backlog items (BL-01 through BL-17) are complete — 76 checks implemented covering all SDLC phases plus ISO/IEC/IEEE standards validation, with dependency graph and duplicate consolidation. All 10 production blocker items (PB-01 through PB-10) are complete: CI/CD pipeline, deprecated dep removal, clippy clean, dependency auditing, API docs, regex LazyLock, Cargo metadata, README enhancement, release automation, and missing_docs lint.

## Status: 17/17 backlog items complete, 10/10 production blockers complete

## Overview

Gap analysis of compliance checks missing from doc-engine relative to the [template-engine documentation framework](https://github.com/sweengineeringlabs/template-engine/blob/main/templates/framework.md). The current 53 checks cover Phases 0-2 well (git files, foundation, design structure) but have minimal coverage of Phase 3 (development docs), zero coverage of Phase 4 (module docs), and zero coverage of Phase 5 (backlog/planning).

## Backlog Items

### High Priority

- [x] **BL-02** — Developer guide hub existence check (Phase 3 gap) — 2026-02-09
- [x] **BL-03** — Module README W3H structure checks (Phase 4 gap) — 2026-02-09
- [x] **BL-04** — Module examples & tests checks (Phase 4 gap — framework marks "Critical!") — 2026-02-09
- [x] **BL-05** — Module toolchain documentation checks (Phase 4 gap) — 2026-02-09
- [x] **BL-06** — Module deployment documentation checks (Phase 4 gap) — 2026-02-09
- [x] **BL-07** — INTERNAL_USAGE.md for internal projects (Phase 0 governance gap) — 2026-02-09

### Medium Priority

- [x] **BL-01** — Backlog file existence checks (Phase 5 gap) — 2026-02-09
- [x] **BL-08** — Templates directory checks (Phase 1 gap) — 2026-02-09
- [x] **BL-09** — W3H structure enforcement across all docs (framework principle gap) — 2026-02-09
- [x] **BL-10** — Root README line count check (best practice gap) — 2026-02-09

### Low Priority

- [x] **BL-11** — Feature-prefixed artifact naming checks (conditional — only for FR-tracking projects) — 2026-02-09

### Added During Implementation

- [x] **BL-12** — Backlog→requirements traceability (check 82) — 2026-02-09
- [x] **BL-13** — Planning phase artifacts: risk register, estimation, schedule, resource plan, communication plan, quality plan (checks 83-88, FR-804) — 2026-02-09
- [x] **BL-14** — SRS 29148 attribute validation (check 89, FR-805) — 2026-02-09
- [x] **BL-15** — ISO/IEC/IEEE standards validation: 42010 architecture + 29119-3 testing (checks 90-91, FR-806, FR-807) — 2026-02-09

### Architecture — Check Engine

- [x] **BL-16** — Check dependency graph: `depends_on` field in rules.toml, automatic skip-propagation (FR-305) — 2026-02-11
- [x] **BL-17** — Duplicate check consolidation: merge 5 duplicate existence check pairs (FR-306, blocked by BL-16) — 2026-02-11

---

## Item Details

### BL-01: Backlog File Existence Checks

**Framework reference**: Phase 5 checklist (lines 616-618), directory structure (lines 61-62, 79)

**What**: The framework requires `docs/backlog.md` (index), `docs/framework-backlog.md` (cross-cutting), and per-module `backlog.md` files. No checks exist.

**Proposed checks**:
- `docs/backlog.md` exists — severity: warning
- `docs/framework-backlog.md` exists — severity: info
- Populated modules have a `backlog.md` — severity: info

**Priority**: Medium — Phase 5 is the last implementation phase; backlogs are planning artifacts, not structural. Projects can function without formalized backlog files.

**Estimated checks**: 2-3

---

### BL-02: Developer Guide Hub Existence

**Framework reference**: Document types §6 (lines 334-346), Phase 3 checklist (line 590), navigation flow (lines 222-223)

**What**: `docs/4-development/developer_guide.md` is a required Phase 3 artifact and the second hub document (alongside `architecture.md`). It is the entry point for developers and contributors. No check exists.

**Proposed checks**:
- `docs/4-development/developer_guide.md` exists — severity: warning

**Why warning not error**: The framework says "Start with `3-design/` and `4-development/` at minimum" but also notes phases are optional. Not all projects have enough development guidance to warrant a dedicated hub. Architecture.md is more universally applicable.

**Priority**: High — explicitly required by framework, navigation flow assumes it exists.

**Estimated checks**: 1

---

### BL-03: Module README W3H Structure

**Framework reference**: Document types §7 (lines 348-361), Phase 4 checklist (lines 596-604), key principles (line 511)

**What**: Every module's `docs/README.md` must follow W3H structure: WHO (**Audience**), WHAT (description), WHY (problems solved), HOW (usage). Existing check 34 only validates **Audience** — the remaining three W3H sections are unchecked. The framework also requires a Prerequisites section.

**Proposed checks**:
- Module READMEs contain WHAT/WHY/HOW sections — severity: warning
- Module READMEs contain Prerequisites section — severity: info

**Priority**: High — W3H is the framework's core documentation principle.

**Estimated checks**: 2

---

### BL-04: Module Examples & Tests

**Framework reference**: "Examples and Tests (Critical!)" section (lines 450-495), Phase 4 checklist (lines 601-604), success metrics (lines 795-797)

**What**: Every module/crate must have `examples/basic.rs` and `tests/integration.rs`. Module READMEs must link to examples and tests. The framework explicitly labels this **"Critical!"** — the strongest language used for any single requirement. Zero checks exist.

**Proposed checks**:
- Each module has an `examples/` directory with at least one file — severity: warning
- Each module has integration tests — severity: warning
- Module README contains "Examples" or "Tests" section — severity: info

**Priority**: High — only framework item marked "Critical!", compile-checked documentation.

**Estimated checks**: 2-3

---

### BL-05: Module Toolchain Documentation

**Framework reference**: Document types §8 (lines 363-405), Phase 4 checklist (lines 605-609), success metrics (line 799)

**What**: Every module needs `docs/3-design/toolchain.md` documenting tools used, with required sections: Overview, Tools (what/version/install/why/how), Version Matrix, Verification. No checks exist.

**Proposed checks**:
- Each module has `docs/3-design/toolchain.md` — severity: warning
- Toolchain.md contains required sections (Overview, Tools, Verification) — severity: info

**Priority**: High — explicit framework requirement with detailed template.

**Estimated checks**: 1-2

---

### BL-06: Module Deployment Documentation

**Framework reference**: Document types §9 (lines 407-434), Phase 4 checklist (lines 610-613), success metrics (line 800)

**What**: Every module needs `docs/6-deployment/` with three required files: `README.md` (index), `prerequisites.md` (system requirements), `installation.md` (installation guides). No checks exist.

**Proposed checks**:
- Each module has `docs/6-deployment/README.md` — severity: warning
- Each module has `docs/6-deployment/prerequisites.md` — severity: info
- Each module has `docs/6-deployment/installation.md` — severity: info

**Priority**: High — explicit requirement. Checks should skip modules without a `docs/6-deployment/` directory since not all modules are independently deployable.

**Estimated checks**: 2-3

---

### BL-07: INTERNAL_USAGE.md for Internal Projects

**Framework reference**: Phase 0 checklist (line 570), internal project requirements (lines 198-203), validation (line 625)

**What**: Internal/proprietary projects must have `INTERNAL_USAGE.md` documenting approved use cases. Existing check 31 validates open-source community files (`CODE_OF_CONDUCT.md`, `SUPPORT.md`) when `project_type = "open_source"`, but the inverse check for internal projects is missing.

**Proposed checks**:
- `INTERNAL_USAGE.md` exists when `project_type = "internal"` — severity: warning

**Priority**: High — Phase 0 governance requirement. Without this, internal projects have incomplete governance.

**Estimated checks**: 1

---

### BL-08: Templates Directory

**Framework reference**: Phase 1 checklist (line 579), directory structure (lines 63-66)

**What**: `docs/templates/` should contain documentation templates (`crate_readme.template.md`, `framework_doc.template.md`). Phase 1 foundation artifact. No checks exist.

**Proposed checks**:
- `docs/templates/` directory exists — severity: info
- `docs/templates/` contains at least one `.template.md` file — severity: info

**Why info not warning**: Templates are supportive tooling. Their absence doesn't mean documentation is non-compliant — it means the team hasn't formalized their templates. Useful but not critical.

**Priority**: Medium — Phase 1 artifact, but supportive rather than structural.

**Estimated checks**: 1-2

---

### BL-09: W3H Structure in All Docs

**Framework reference**: Key principles (line 511-513: "W³H Pattern Universal"), documentation rules table (lines 499-509), validation checklist (line 636), success metrics (line 792)

**What**: All framework docs should follow W3H (WHO-WHAT-WHY-HOW). Currently only check 41 validates W3H for the `docs/README.md` hub. Check 33 validates **Audience** (WHO) across all docs. The WHAT/WHY/HOW sections are not validated beyond the hub.

**Proposed checks**:
- Design docs (`docs/3-design/*.md`) contain W3H sections — severity: info
- Development guides (`docs/4-development/guide/*.md`) contain W3H sections — severity: info

**Priority**: Medium — important principle. Audience check (33) already covers the most enforceable part (WHO). Limit scope to hub documents and module READMEs where structure is predictable.

**Estimated checks**: 1-2

---

### BL-10: Root README Line Count

**Framework reference**: Best practices (line 717: "Keep README lean (< 100 lines)"), success metrics (line 790: "README < 100 lines")

**What**: Root `README.md` should be under 100 lines to serve as a lean entry point that redirects to `docs/README.md` for details. No check exists.

**Proposed checks**:
- Root `README.md` is under 100 lines — severity: info

**Why info not warning**: The framework lists this as "Best Practice", not a phase checklist requirement. Some projects legitimately need >100 lines for critical quick-start information (complex installation, multiple platform instructions).

**Priority**: Medium — enforces lean philosophy, but too strict at warning/error level.

**Estimated checks**: 1

---

### BL-11: Feature-Prefixed Artifact Naming

**Framework reference**: Naming rules (lines 147-148), feature-prefixed artifacts section (lines 150-176)

**What**: Feature-scoped folders use `FR_{###}/` and files use `FR_{###}_{name}.ext` (underscores, not hyphens). `FR-{###}` with hyphens is reserved for prose identifiers. No checks exist.

**Proposed checks**:
- Feature-scoped folders match `FR_\d{3}` pattern — severity: info
- Feature-scoped files in FR folders use `FR_{###}_{name}` naming — severity: info
- Flag `FR-{###}` (hyphens) in file paths — severity: info

**Priority**: Low — conditional, applies only to FR-tracking projects. Checks should skip entirely if no FR-prefixed artifacts are found.

**Estimated checks**: 1-2

---

### BL-12: Backlog→Requirements Traceability

**Framework reference**: Traceability pattern (checks 52-53), SRS linkage

**What**: The backlog document should reference the SRS/requirements to maintain the traceability chain: design→requirements (check 52), planning→architecture (check 53), backlog→requirements (check 82).

**Proposed checks**:
- `docs/2-planning/backlog.md` references requirements/SRS — severity: warning

**Priority**: High — completes the traceability chain.

**Estimated checks**: 1

---

### BL-13: Planning Phase Artifacts (FR-804)

**Framework reference**: Traditional SDLC planning phase best practices

**What**: Standard planning phases produce artifacts beyond the implementation plan and backlog: risk register, estimation records, schedule, resource plan, communication plan, and quality plan. Schedule, resource plan, and communication plan are scoped to `open_source` projects only — internal/small projects skip these.

**Proposed checks**:
- `docs/2-planning/risk_register.md` exists — severity: info (all projects)
- `docs/2-planning/estimation.md` exists — severity: info (all projects)
- `docs/2-planning/schedule.md` exists — severity: info (open-source only)
- `docs/2-planning/resource_plan.md` exists — severity: info (open-source only)
- `docs/2-planning/communication_plan.md` exists — severity: info (open-source only)
- `docs/2-planning/quality_plan.md` exists — severity: info (all projects)

**Priority**: Medium — universally valuable for risk/quality/estimation; schedule/resource/communication optional for small projects.

**Estimated checks**: 6

### BL-14: SRS 29148 Attribute Validation (FR-805)

**Framework reference**: ISO/IEC/IEEE 29148:2018, SRS attribute tables

**What**: The template-engine framework references the ISO/IEC/IEEE 29148:2018 standard for SRS structure. doc-engine's own SRS follows 29148 format — each FR/NFR requirement has an attribute table with Priority, State, Verification, Traces to, and Acceptance. No check validated this structure.

**Proposed checks**:
- SRS requirement blocks (FR-xxx, NFR-xxx) have all 5 mandatory 29148 attributes — severity: warning

**Priority**: Medium — enforces standards compliance on requirements documents.

**Estimated checks**: 1

---

### BL-16: Check Dependency Graph

**SRS reference**: FR-305 (Check dependency graph)

**What**: Checks currently execute independently with no awareness of prerequisite results. Content validation, naming convention, and cross-reference checks run even when the target file does not exist, producing misleading "file not found" violations that obscure the root cause. For example, when `README.md` is missing, checks 14 (naming), 26 (root_files), and 40 (navigation content) all fail independently — the user sees 3 failures when the actual problem is 1 missing file.

**Design**: Add an optional `depends_on` field to the TOML rule schema — an array of check IDs that must pass before this check executes. The engine resolves the dependency graph before execution, topologically sorts checks, and automatically skips any check whose parent failed. Skipped checks report `"Skipped: dependency check {id} failed"` and count toward the `skipped` total. Cyclic dependencies are rejected at load time.

**Known dependency pairs**:

| Parent (existence) | Children (content/naming) | File |
|---------------------|--------------------------|------|
| 26 | 14, 40 | README.md |
| 27 | 15 | CONTRIBUTING.md |
| 28 | 16 | CHANGELOG.md |
| 29 | 17 | SECURITY.md |
| 30 | 18 | LICENSE |
| 3 | 37, 38, 39 | docs/glossary.md |
| 6 | 7 | docs/3-design/compliance/compliance_checklist.md |
| 1 | 33, 34, 46, 47 | docs/ directory |
| 72 | 73 | docs/templates/ |

**Implementation scope**:
1. Extend `Rule` struct with `depends_on: Vec<u8>` field (default empty)
2. Add topological sort in engine before check execution
3. Add cycle detection at rule load time
4. Propagate parent Fail → child Skip in the execution loop
5. Add `depends_on` to 15-20 existing rules in `rules.toml`

**Priority**: High — eliminates misleading noise in scan reports, prerequisite for BL-17 (duplicate consolidation).

**Estimated checks**: 0 new checks (engine infrastructure change). Affects ~15-20 existing rules via `depends_on` annotations.

---

### BL-17: Duplicate Check Consolidation

**SRS reference**: FR-306 (Duplicate check consolidation)

**Blocked by**: BL-16 (dependency graph must exist first)

**What**: Five root files are each checked for existence in two separate categories — once in `naming` (checks 14-18) and once in `root_files` (checks 26-30). When a file is missing, both checks fail, doubling the noise in the report. With the dependency graph (BL-16), the `root_files` checks become the authoritative existence checks, and the `naming` checks can be converted to naming-convention validators (e.g., "README must be uppercase", "LICENSE must have no extension") or removed entirely.

**Duplicate pairs to consolidate**:

| Naming check | Root_files check | File | Resolution |
|-------------|-----------------|------|------------|
| 14 | 26 | README.md | 14 → naming convention (uppercase) with `depends_on = [26]` |
| 15 | 27 | CONTRIBUTING.md | 15 → naming convention with `depends_on = [27]` |
| 16 | 28 | CHANGELOG.md | 16 → naming convention with `depends_on = [28]` |
| 17 | 29 | SECURITY.md | 17 → naming convention with `depends_on = [29]` |
| 18 | 30 | LICENSE | 18 → naming convention (no extension) with `depends_on = [30]` |

**Priority**: Medium — depends on BL-16. Reduces duplicate failures from 10 to 5 for projects missing all root files.

**Estimated checks**: 0 new checks (refactors 5 existing checks).

---

## Summary

| ID | Gap | Framework Phase | Priority | Est. Checks | Severity |
|----|-----|-----------------|----------|-------------|----------|
| BL-01 | Backlog files | Phase 5 | Medium | 2-3 | warning/info |
| BL-02 | Developer guide hub | Phase 3 | High | 1 | warning |
| BL-03 | Module README W3H | Phase 4 | High | 2 | warning/info |
| BL-04 | Module examples & tests | Phase 4 | High | 2-3 | warning/info |
| BL-05 | Module toolchain docs | Phase 4 | High | 1-2 | warning/info |
| BL-06 | Module deployment docs | Phase 4 | High | 2-3 | warning/info |
| BL-07 | INTERNAL_USAGE.md | Phase 0 | High | 1 | warning |
| BL-08 | Templates directory | Phase 1 | Medium | 1-2 | info |
| BL-09 | W3H in all docs | Principle | Medium | 1-2 | info |
| BL-10 | README line count | Best practice | Medium | 1 | info |
| BL-11 | FR_{###} naming | Naming | Low | 1-2 | info |
| BL-12 | Backlog→requirements traceability | Traceability | High | 1 | warning |
| BL-13 | Planning phase artifacts (FR-804) | Planning | Medium | 6 | info |
| BL-14 | SRS 29148 attribute validation (FR-805) | Requirements | Medium | 1 | warning |
| BL-15 | ISO standards: 42010 arch + 29119-3 testing (FR-806, FR-807) | Requirements | Medium | 2 | info |
| BL-16 | Check dependency graph (FR-305) | Engine | High | 0 (infra) | — |
| BL-17 | Duplicate check consolidation (FR-306) | Engine | Medium | 0 (refactor) | — |
| | | **Total** | | **~26-34** | |

## Completed

- [x] Checks 1-50: Structural compliance (Milestone 1) — 2026-02-08
- [x] Checks 51-53: Traceability (phase artifacts, design→requirements, plan→architecture) — 2026-02-09
- [x] Checks 69-81: Backlog checks BL-01 through BL-11 (13 new checks) — 2026-02-09
- [x] Check 82: Backlog→requirements traceability (FR-804 precursor) — 2026-02-09
- [x] Checks 83-88: Planning phase artifacts per FR-804 (risk register, estimation, schedule, resource plan, communication plan, quality plan) — 2026-02-09
- [x] Check 89: SRS 29148 attribute validation per FR-805 — 2026-02-09
- [x] Checks 90-91: ISO/IEC/IEEE 42010 architecture + 29119-3 testing per FR-806, FR-807 — 2026-02-09
- [x] PB-01: CI/CD pipeline (GitHub Actions: test, clippy, audit, self-compliance) — 2026-02-09
- [x] PB-02: Removed unused serde_yaml dependency — 2026-02-09
- [x] PB-03: Clippy clean (7 warnings fixed, zero-warning policy) — 2026-02-09
- [x] PB-04: Dependency auditing (cargo-deny with deny.toml) — 2026-02-09
- [x] PB-05: Public API documentation (crate doc, SAF functions, all traits) — 2026-02-09
- [x] PB-06: Regex LazyLock (~40 regexes across 9 files) — 2026-02-09
- [x] PB-07: Cargo.toml metadata (repository, authors, keywords, categories) — 2026-02-09
- [x] PB-08: README enhancement (badge, examples, exit codes, contributing) — 2026-02-09
- [x] PB-09: Release automation (tag-triggered workflow) — 2026-02-09
- [x] PB-10: missing_docs lint enabled with full API coverage — 2026-02-09

## Blockers

| Blocker | Impact | Owner | Status |
|---------|--------|-------|--------|
| Module discovery logic needed for BL-03 through BL-06 | High — four backlog items depend on reliably detecting modules/crates | — | Resolved (2026-02-09) |
| `project_type` conditional check support needed for BL-07 | Medium — internal project governance check blocked | — | Resolved (already supported) |

---

## Production Blockers

Production readiness audit (2026-02-09) identified 10 areas; 1 critical blocker, 7 warnings, 2 passing. Items below are prioritized by blast radius and reversibility.

### Critical

- [x] **PB-01** — CI/CD pipeline: GitHub Actions workflow for `cargo test`, `cargo clippy`, `cargo audit`, and self-compliance scan — 2026-02-09

### High Priority

- [x] **PB-02** — Remove unused `serde_yaml` 0.9 (deprecated) from dependencies — 2026-02-09
- [x] **PB-03** — Clippy clean: fix 7 warnings (`collapsible_if`, `unnecessary_map_or`, `manual_strip`) — 2026-02-09
- [x] **PB-04** — Dependency auditing: add `cargo-deny` with `deny.toml` to CI — 2026-02-09

### Medium Priority

- [x] **PB-05** — Public API documentation: `//!` crate-level doc, doc comments on SAF functions and trait definitions — 2026-02-09
- [x] **PB-06** — Regex initialization: replaced ~40 `Regex::new().unwrap()` calls with `LazyLock` statics across 9 handler files — 2026-02-09
- [x] **PB-07** — Cargo.toml metadata: added `repository`, `authors`, `keywords`, `categories` — 2026-02-09
- [x] **PB-08** — README enhancement: CI badge, usage examples for all flags, spec examples, exit codes, contributing link — 2026-02-09

### Low Priority

- [x] **PB-09** — Release automation: tag-triggered GitHub Actions release workflow with binary attachment — 2026-02-09
- [x] **PB-10** — Enable `#![warn(missing_docs)]` lint with full public API coverage — 2026-02-09

---

## Production Blocker Details

### PB-01: CI/CD Pipeline

**What**: No `.github/workflows/` directory exists. Zero automated testing, linting, or compliance verification on push/PR.

**Proposed pipeline** (GitHub Actions):
- `cargo test` — all 251 tests
- `cargo clippy -- -D warnings` — zero-warning policy
- `cargo audit` — dependency vulnerability scan
- `cargo run -- scan .` — self-compliance (exit 1 on failures)
- Matrix: stable + MSRV (if declared)

**Priority**: Critical — without CI, regressions can ship silently. Every other production improvement depends on CI to enforce it.

---

### PB-02: Migrate Deprecated serde_yaml

**What**: `serde_yaml = "0.9"` is marked `+deprecated` upstream. The spec module (`core/spec/parser.rs`) uses it for YAML spec parsing. The recommended replacement is `serde_yml` (maintained fork) or `serde_json` + a different YAML library.

**Impact**: Medium — functional today, but deprecated crates stop receiving security patches. Supply-chain risk grows over time.

**Approach**: Replace `serde_yaml` with `serde_yml` in `Cargo.toml` and update import paths. API is largely compatible.

---

### PB-03: Clippy Clean

**What**: 7 clippy warnings across 4 files — all auto-fixable style/modernization issues:
- `collapsible_if` in `scanner.rs`
- `unnecessary_map_or` in `structure.rs`, `navigation.rs`, `module.rs` (5 occurrences)
- `manual_strip` in `traceability.rs`

**Approach**: `cargo clippy --fix` resolves all 7 automatically. No logic changes.

---

### PB-04: Dependency Auditing

**What**: No `cargo-audit` or `cargo-deny` configuration. Vulnerable dependencies would go undetected.

**Approach**: Add `cargo audit` step to CI (PB-01). Optionally add `deny.toml` for `cargo-deny` with license and advisory policies.

---

### PB-05: Public API Documentation

**What**: Struct and enum types have doc comments. Key public entry points do not:
- `lib.rs` — no `//!` crate-level doc
- `saf/mod.rs` — `scan()`, `scan_with_config()`, `format_report_text()`, `format_report_json()` undocumented
- `api/traits.rs` — `ComplianceEngine`, `SpecEngine` traits undocumented
- `spi/traits.rs` — `FileScanner`, `CheckRunner`, `Reporter` traits undocumented

**Impact**: Library consumers cannot understand the API from `cargo doc` output alone.

---

### PB-06: Regex Initialization

**What**: ~30 `Regex::new(...).unwrap()` calls in production handler `run()` methods. All use compile-time literal patterns that will never fail, but:
- Each call recompiles the regex on every check execution (performance)
- `unwrap()` in production is a code-quality signal

**Approach**: Use `std::sync::LazyLock` (stable since Rust 1.80) to compile each regex once. Eliminates both the `unwrap()` and the per-call compilation cost.

---

### PB-07: Cargo.toml Metadata

**What**: Missing fields for crates.io publishing:
- `repository` — link to GitHub repo
- `authors` — maintainer list
- `keywords` — discovery tags (e.g., `documentation`, `compliance`, `audit`, `cli`)
- `categories` — crates.io categories (e.g., `command-line-utilities`, `development-tools`)

---

### PB-08: README Enhancement

**What**: Root `README.md` has basic overview and single install command. Missing:
- CI status badge
- Usage examples for `--json`, `--checks 1-13`, `--type internal`, `--rules custom.toml`
- Spec subcommand examples
- Link to CONTRIBUTING.md

---

### PB-09: Release Automation

**What**: Version is `0.1.0` with no release process. CHANGELOG has one entry. No git tags, no release tooling.

**Approach**: Configure `cargo-release` or `release-plz` for semver bumps, CHANGELOG generation, and tag-based GitHub releases.

---

### PB-10: Missing Docs Lint

**What**: `#![warn(missing_docs)]` is not enabled. Public API items can be added without documentation and no compiler warning fires.

**Approach**: Enable after PB-05 is complete (otherwise it produces dozens of warnings immediately). Gates future public API additions.

---

## Production Blocker Summary

| ID | Area | Priority | Blocks | Est. Effort |
|----|------|----------|--------|-------------|
| PB-01 | CI/CD pipeline | Critical | PB-03, PB-04 | 1-2 hours |
| PB-02 | Migrate serde_yaml | High | — | 1 hour |
| PB-03 | Clippy clean | High | — | 15 min |
| PB-04 | Dependency auditing | High | PB-01 | 30 min |
| PB-05 | Public API docs | Medium | PB-10 | 2-3 hours |
| PB-06 | Regex LazyLock | Medium | — | 1-2 hours |
| PB-07 | Cargo.toml metadata | Medium | — | 15 min |
| PB-08 | README enhancement | Medium | PB-01 (badge) | 1 hour |
| PB-09 | Release automation | Low | PB-01 | 1-2 hours |
| PB-10 | missing_docs lint | Low | PB-05 | 30 min |

## Review Items

Items to evaluate for potential adoption into backlog content validation standards:

- [ ] **RV-01** — PMBOK 7th Edition: mentions backlogs under adaptive planning but doesn't prescribe a format. Review whether PMBOK adaptive planning outputs map to additional backlog sections.
- [ ] **RV-02** — ISO 21502:2020 (Project management): covers work planning and tracking generically. Evaluate whether its planning process outputs suggest additional backlog structure requirements.
- [ ] **RV-03** — ISO/IEC/IEEE 12207:2017 (Software lifecycle processes): defines "planning process" outputs but doesn't mandate backlog structure. Review whether lifecycle process outputs (e.g., work breakdown, resource allocation) warrant dedicated backlog sections.
- [ ] **RV-04** — Scrum Guide (2020): defines Product Backlog with ordering, transparency, and refinement concepts. Evaluate whether a "Sprint Goal" or "Definition of Done" section should be recommended.
- [ ] **RV-05** — SAFe (Scaled Agile Framework): distinguishes team backlog, program backlog, and portfolio backlog with capacity allocation. Evaluate whether multi-level backlog structure applies to multi-module projects.

## Notes

- Check IDs 54-68 are reserved for the spec module (planned). New checks from this backlog would start at 69+.
- BL-03 through BL-06 all require module/crate discovery. Implementing a shared `ModuleDiscovery` component first would unblock all four items simultaneously.
- Severity levels follow the framework's own language: Phase checklist items are warnings, best practices are info, governance requirements match existing patterns.
- The framework states "Not all projects need all phases" (line 31). Checks for optional phases should skip gracefully when the phase directory doesn't exist.
- BL-17 (duplicate consolidation) is blocked by BL-16 (dependency graph). The dependency graph must exist before duplicate existence checks can be safely consolidated — without it, removing a duplicate check would lose coverage.
