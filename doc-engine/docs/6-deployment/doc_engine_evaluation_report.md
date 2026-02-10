# Rules vs template-engine Gap Analysis

**Audience**: Architects, documentation maintainers

**Date**: 2026-02-10

This report compares the 62 unique file/directory existence checks in doc-engine's `rules.toml` (124 total checks) against the templates provided by `../template-engine`. It identifies matches, mismatches, and gaps.

## Standards Applicability

This evaluation is conducted under **ISO/IEC 25040:2024** (Evaluation process). It assesses documentation **artifacts** — whether expected files exist and whether their content meets ISO-mandated section requirements.

**ISO/IEC 33001–33099 (SPICE)** — the process assessment framework — is **out of scope**. SPICE evaluates how well an organization *performs* its processes (capability levels 0–5), not whether the resulting artifacts exist or conform to a structure. doc-engine is a static file scanner; it inspects outputs, not the processes that produced them.

| Concern | Applicable Standard | In Scope |
|---------|--------------------|----------|
| Do documentation artifacts exist? | ISO/IEC 25040:2024, ISO/IEC/IEEE 15289:2019 | Yes |
| Do artifacts contain required sections? | ISO/IEC 25040:2024, per-standard clause mappings | Yes |
| Is the documentation process managed and optimized? | ISO/IEC 33001–33099 (SPICE) | No |

---

## Root Files

| Chk | Expected File | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 14,26 | `README.md` | error | small | `README.md` | Match |
| 15,27 | `CONTRIBUTING.md` | error | med | `CONTRIBUTING.md` | Match |
| 16,28 | `CHANGELOG.md` | err/warn | med | `CHANGELOG.md` | Match |
| 17,29 | `SECURITY.md` | error | med | -- | **GAP** |
| 18,30 | `LICENSE` | error | small | `LICENSE` | Match |
| 19 | `.gitignore` | error | small | `.gitignore` | Match |
| 20 | `.editorconfig` | warn | med | -- | **GAP** |
| 70 | `INTERNAL_USAGE.md` | warn | large | -- | GAP (internal only) |

## docs/ Root

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 1 | `docs/` (dir) | error | small | no `docs/` in SDLC templates | **GAP** |
| 2 | `docs/README.md` | error | small | `backend/docs/README.template.md` | Match (stack variant) |
| 3 | `docs/glossary.md` | error | small | `glossary.template.md` | Match |
| 72 | `docs/templates/` (dir) | info | large | -- | **GAP** |

## Phase 0 — Ideation

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 54 | `docs/0-ideation/README.md` | info | med | `sdlc/0-ideation/idea.template.md` | **MISMATCH** (idea, not README) |
| 118 | `docs/0-ideation/conops.md` | info | large | -- | **GAP** |

## Phase 1 — Requirements

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 55 | `docs/1-requirements/srs.md` | error | med | `brd.template.spec`, `feature_request.template.spec` | **MISMATCH** (BRD/FR, not SRS) |
| 119 | `docs/1-requirements/strs.md` | warn | large | -- | **GAP** |
| 120 | `docs/1-requirements/traceability_matrix.md` | warn | large | -- | **GAP** |

## Phase 2 — Planning

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 56 | `docs/2-planning/implementation_plan.md` | warn | med | `sdlc/2-planning/implementation_plan.template.md` | **Match** |
| 71 | `docs/2-planning/backlog.md` | warn | med | `sdlc/4-development/backlog.template.md` | **MISMATCH** (phase 4, not 2) |
| 83 | `docs/2-planning/risk_register.md` | warn | large | -- | **GAP** |
| 84 | `docs/2-planning/estimation.md` | info | large | -- | **GAP** |
| 85 | `docs/2-planning/schedule.md` | info | large | -- | **GAP** |
| 86 | `docs/2-planning/resource_plan.md` | info | large | -- | **GAP** |
| 87 | `docs/2-planning/communication_plan.md` | info | large | -- | **GAP** |
| 88 | `docs/2-planning/quality_plan.md` | warn | large | -- | **GAP** |
| 109 | `docs/2-planning/project_management_plan.md` | warn | large | -- | **GAP** |
| 110 | `docs/2-planning/configuration_management_plan.md` | warn | large | -- | **GAP** |
| 111 | `docs/2-planning/risk_management_plan.md` | warn | large | -- | **GAP** |
| 112 | `docs/2-planning/verification_plan.md` | warn | large | -- | **GAP** |
| 113 | `docs/2-planning/test_plan.md` | warn | large | -- | **GAP** |
| 121 | `docs/2-planning/progress_reports.md` | warn | large | -- | **GAP** |
| 122 | `docs/2-planning/decision_log.md` | warn | large | -- | **GAP** |
| 123 | `docs/2-planning/audit_report.md` | info | large | -- | **GAP** |

## Phase 3 — Design

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 11 | `docs/3-design/adr/` (dir) | warn | med | `backend/docs/3-design/adr.template.md` | **MISMATCH** (file, not dir) |
| 48 | `docs/3-design/adr/README.md` | warn | med | -- | **GAP** |
| 6 | `docs/3-design/compliance/compliance_checklist.md` | warn | large | `sdlc/3-design/compliance/compliance_checklist.template.md` | **Match** |
| 57 | `docs/3-design/architecture.md` | warn | small | `sdlc/3-design/architecture.template.arch` | Match (ext differs) |
| 107 | `docs/3-design/design_description.md` | warn | large | -- | **GAP** |
| 108 | `docs/3-design/interface_description.md` | info | large | -- | **GAP** |

## Phase 4 — Development

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 58 | `docs/4-development/setup_guide.md` | info | med | `sdlc/4-development/setup_guide.template.setup` | Match (ext differs) |
| 69 | `docs/4-development/developer_guide.md` | warn | small | `sdlc/4-development/developer_guide.template.md` | **Match** |
| 103 | `docs/4-development/integration_plan.md` | warn | large | -- | **GAP** |
| 104 | `docs/4-development/user_documentation.md` | warn | large | -- | **GAP** |
| 105 | `docs/4-development/api_documentation.md` | info | med | -- | **GAP** |
| 106 | `docs/4-development/build_procedures.md` | info | med | -- | **GAP** |

## Phase 5 — Testing

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 59 | `docs/5-testing/testing_strategy.md` | warn | med | `sdlc/5-testing/testing_strategy.template.md` | **Match** |
| 99 | `docs/5-testing/test_plan.md` | warn | med | `sdlc/5-testing/test_plan.template.test` | Match (ext differs) |
| 100 | `docs/5-testing/test_design.md` | warn | large | -- | **GAP** |
| 101 | `docs/5-testing/test_cases.md` | warn | large | -- | **GAP** |
| 102 | `docs/5-testing/verification_report.md` | warn | large | -- | **GAP** |

## Phase 6 — Deployment

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 60 | `docs/6-deployment/README.md` | warn | med | -- | **GAP** |
| 61 | `docs/6-deployment/deployment_guide.md` | info | large | `deploy.template.deploy` | Match (name/ext differs) |
| 62 | `docs/6-deployment/ci_cd.md` | warn | med | `sdlc/6-deployment/ci_cd.template.md` | **Match** |
| 68 | `docs/6-deployment/installation_guide.md` | warn | large | -- | **GAP** |
| 114 | `docs/6-deployment/transition_plan.md` | warn | large | -- | **GAP** |
| 115 | `docs/6-deployment/release_notes.md` | info | med | -- | **GAP** |
| 116 | `docs/6-deployment/user_manual.md` | warn | large | -- | **GAP** |

## Phase 7 — Operations

| Chk | Expected Path | Sev | Scope | template-engine | Status |
|-----|---------------|-----|-------|-----------------|--------|
| 63 | `docs/7-operations/README.md` | warn | med | -- | **GAP** |
| 64 | `docs/7-operations/operations_manual.md` | warn | large | `sdlc/7-operation/ops_manual.template.md` | **MISMATCH** (ops_manual, singular 7-operation) |
| 65 | `docs/7-operations/troubleshooting.md` | info | large | `backend/docs/6-deployment/troubleshooting.template.md` | **MISMATCH** (phase 6, not 7) |
| 66 | `docs/7-operations/maintenance_plan.md` | warn | large | -- | **GAP** |
| 67 | `docs/7-operations/configuration.md` | warn | med | `backend/docs/6-deployment/configuration.template.md` | **MISMATCH** (phase 6, not 7) |
| 117 | `docs/7-operations/disposal_plan.md` | warn | large | -- | **GAP** |

## Builtin Content-Validation Checks (implicit target file)

| Chk | Target File | Standard | template-engine | Status |
|-----|-------------|----------|-----------------|--------|
| 89 | `docs/1-requirements/srs.md` | 29148 | no SRS template | **GAP** |
| 90 | `docs/3-design/architecture.md` | 42010 | `architecture.template.arch` | Match |
| 91 | `docs/5-testing/testing_strategy.md` | 29119-3 | `testing_strategy.template.md` | Match |
| 92-93,96-98 | `docs/6-deployment/production_readiness.md` | 25010/25040/12207 | `production_readiness.template.md` | **Match** |
| 94 | `docs/4-development/developer_guide.md` | 26514 | `developer_guide.template.md` | Match |
| 95 | `docs/2-planning/backlog.md` | internal | `backlog.template.md` (wrong phase) | MISMATCH |
| 124 | `docs/2-planning/audit_report.md` | 1028 | -- | **GAP** |

---

## Summary

| | Count |
|---|---|
| **Total unique file/dir checks** | 62 (deduped) |
| **Match** (direct or ext differs) | 17 |
| **Mismatch** (name/path/phase differs) | 7 |
| **Gap** (no template at all) | 38 |

### Key Structural Mismatches

- template-engine uses `7-operation` (singular) vs doc-engine's `7-operations` (plural)
- template-engine puts `backlog` in phase 4, doc-engine expects phase 2
- template-engine puts `troubleshooting` and `configuration` in phase 6, doc-engine expects phase 7
- template-engine has `ops_manual`, doc-engine expects `operations_manual`

### Largest Gaps (warn/error severity, no template exists)

| Area | Missing Checks | Count |
|------|----------------|-------|
| Planning artifacts | 83, 88, 109-113, 121-122 | 9 |
| Requirements artifacts | 119-120 | 2 |
| Design artifacts | 107 | 1 |
| Development artifacts | 103-104 | 2 |
| Testing artifacts | 100-102 | 3 |
| Deployment artifacts | 68, 114, 116 | 3 |
| Operations artifacts | 66, 117 | 2 |
| Root files | SECURITY.md | 1 |
| **Total warn/error gaps** | | **23** |
