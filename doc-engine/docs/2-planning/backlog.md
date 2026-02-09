# doc-engine Backlog

**Audience**: Developers, architects, project stakeholders

## TLDR

Gap analysis identified 11 missing check categories against the template-engine framework. Current 53 checks cover Phases 0-2 (git files, foundation, design) but have zero coverage of Phase 3 (development docs), Phase 4 (module docs), and Phase 5 (backlog). Estimated 16-24 new checks needed, prioritized as 6 high (developer hub, module W3H, examples/tests, toolchain, deployment, internal governance), 4 medium (backlog files, templates, W3H enforcement, README length), and 1 low (FR naming).

## Status: Complete

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
| | | **Total** | | **~23-31** | |

## Completed

- [x] Checks 1-50: Structural compliance (Milestone 1) — 2026-02-08
- [x] Checks 51-53: Traceability (phase artifacts, design→requirements, plan→architecture) — 2026-02-09
- [x] Checks 69-81: Backlog checks BL-01 through BL-11 (13 new checks) — 2026-02-09
- [x] Check 82: Backlog→requirements traceability (FR-804 precursor) — 2026-02-09
- [x] Checks 83-88: Planning phase artifacts per FR-804 (risk register, estimation, schedule, resource plan, communication plan, quality plan) — 2026-02-09

## Blockers

| Blocker | Impact | Owner | Status |
|---------|--------|-------|--------|
| Module discovery logic needed for BL-03 through BL-06 | High — four backlog items depend on reliably detecting modules/crates | — | Resolved (2026-02-09) |
| `project_type` conditional check support needed for BL-07 | Medium — internal project governance check blocked | — | Resolved (already supported) |

## Notes

- Check IDs 54-68 are reserved for the spec module (planned). New checks from this backlog would start at 69+.
- BL-03 through BL-06 all require module/crate discovery. Implementing a shared `ModuleDiscovery` component first would unblock all four items simultaneously.
- Severity levels follow the framework's own language: Phase checklist items are warnings, best practices are info, governance requirements match existing patterns.
- The framework states "Not all projects need all phases" (line 31). Checks for optional phases should skip gracefully when the phase directory doesn't exist.
