# Software Requirements Specification: doc-engine

**Audience**: Developers, architects, project stakeholders

## TLDR

This SRS defines requirements for doc-engine, a Rust CLI tool that audits project documentation against 128 compliance checks across 18 categories, mapped to 8 ISO/IEC/IEEE standards, IEEE 1028, and PMBOK. It also scaffolds SDLC spec files from an SRS document, generating per-domain spec, architecture, test plan, deployment, and test execution plan files. It covers stakeholder needs, functional requirements for rule evaluation, reporting, and scaffolding, non-functional requirements for performance and extensibility, and traceability from stakeholder goals to implementation modules.

**Version**: 1.0
**Date**: 2026-02-07
**Standard**: ISO/IEC/IEEE 29148:2018

---

## 1. Introduction

### 1.1 Purpose

This SRS defines the stakeholder, system, and software requirements for **doc-engine**, a Rust CLI tool and library that audits project documentation against the compliance standard defined by the template-engine documentation framework. The engine supports 128 checks across 18 categories: structure (1-13, 69, 72-73), naming (14-25, 76), root_files (26-32, 70), content (33-39, 75), navigation (40-43, 74), cross_ref (44-47), adr (48-50), traceability (51-53, 82, 121-124), ideation (54, 118), requirements (55, 89-98, 119-120), planning (56, 83-88, 109-113), design (57, 107-108), development (58, 103-106), testing (59, 99-102, 125-128), deployment (60-62, 68, 114-116), operations (63-67, 117), backlog (71), and module (77-81). Phase artifact checks (54-68, 99-128) verify the existence of ISO-mandated documentation per SDLC phase; checks 89-98, 124-128 validate content against ISO/IEC/IEEE 29148, 42010, 29119-3, 26514, 12207, 25010, 25040, and IEEE 1028 standards.

### 1.2 Scope

doc-engine is a single-crate Rust project within the `swe-compliance` workspace. It:

- Scans any project directory for documentation compliance
- Sources rules from a TOML configuration file (declarative + builtin handlers)
- Reports results as text or JSON
- Is usable as both a CLI binary and a Rust library
- Validates spec files in two formats: YAML (`.spec.yaml`, `.arch.yaml`, `.test.yaml`, `.deploy.yaml`) and markdown (`.spec`, `.arch`, `.test`, `.deploy`) for structure, cross-references, and SDLC coverage
- Generates markdown documentation from YAML spec files
- Scaffolds per-domain SDLC spec files from an SRS markdown document, including test execution plans (`.manual.exec`, `.auto.exec`), with optional `--phase` filtering by SDLC phase

doc-engine does **not**:

- Enforce code-level conventions (only documentation structure/content)
- Require network access (local file system only)

### 1.3 Definitions and Acronyms

| Term | Definition |
|------|-----------|
| **SEA** | Stratified Encapsulation Architecture — layered module pattern (SPI/API/Core/SAF) |
| **SAF** | Surface API Facade — public re-export layer for library consumers |
| **SPI** | Service Provider Interface — trait definitions and low-level types |
| **W3H** | WHO-WHAT-WHY-HOW — documentation structure pattern from template-engine |
| **SDLC** | Software Development Life Cycle — phases 0-7 mapped to directory names |
| **Documentation audit report** | Per ISO/IEC/IEEE 15289:2019, the standard information item produced by a documentation compliance audit; canonical filename: `documentation_audit_report_v{version}.json` |
| **Audit status report** | A JSON information item conforming to ISO/IEC/IEEE 15289:2019 clause 9.2 (audit reports); persisted via `--output <path>` and containing identification metadata (standard, clause, tool, version, timestamp), scope (project_root, project_type, project_scope), and results (per-check entries with summary) |
| **Project scope** | Tier that determines which rule subset applies: Small (core essentials), Medium (security, ADRs, traceability), Large (complete SDLC with ISO compliance) |
| **ADR** | Architecture Decision Record — numbered decision documents in `docs/3-design/adr/` |
| **Declarative rule** | A check defined entirely in TOML, executed by the generic DeclarativeCheck runner |
| **Builtin rule** | A check referencing a named Rust handler for complex logic |
| **Spec file** | A documentation artifact using domain-specific extensions (`.spec`, `.arch`, `.test`, `.deploy`), in either markdown or YAML format |
| **YAML spec** | A structured data file (`.spec.yaml`, `.arch.yaml`, `.test.yaml`, `.deploy.yaml`) with `kind` discriminator and typed schema |
| **Markdown spec** | A markdown file using bare extensions (`.spec`, `.arch`, `.test`, `.deploy`) with structured metadata headers (`**Version:**`, `**Status:**`, `**Spec:**`) |
| **BRD** | Business Requirements Document — master inventory of domain specs in `1-requirements/` (`brd.spec` or `brd.spec.yaml`) |
| **Spec ID** | Project-level spec identifier matching `[A-Z]+-\d{3}` pattern (e.g., `RS-001`, `FR-001`), declared in `**Related:**` headers |
| **Requirement ID** | Domain-prefixed requirement identifier matching `[A-Z]+-\d{3}` (e.g., `DESIGN-001`, `LINT-014`, `REQ-001`), used in `Verifies` traceability |
| **Feature stem** | The shared name portion of linked spec files (e.g., `compiler_design` in `compiler_design.spec` / `.arch` / `.test` / `.deploy`) |
| **SpecKind** | The SDLC role of a spec file: `brd`, `feature_request`/`spec`, `architecture`, `test_plan`, `deployment` — determined by extension |
| **SpecFormat** | Whether a spec file is YAML (`.spec.yaml`) or markdown (`.spec`) — determines the parsing strategy |
| **Scaffold** | The process of generating a full set of SDLC spec files from an SRS document — creates per-domain `.spec`, `.arch`, `.test`, `.deploy` (YAML + markdown), `.manual.exec`, `.auto.exec`, and a BRD inventory |
| **Manual execution plan** | A `.manual.exec` markdown file listing all test cases with Steps, Expected, Tester, Date, Pass/Fail, and Notes columns — an actionable checklist for human testers |
| **Automated execution plan** | An `.auto.exec` markdown file listing all test cases with Verifies, CI Job, Build, Status, and Last Run columns — a CI/automated test tracker |
| **Phase filter** | A `--phase` CLI flag that restricts scaffold output to specific SDLC phases (`requirements`, `design`, `testing`, `deployment`); when omitted, all phases are generated |
| **Scaffold status report** | A JSON information item conforming to ISO/IEC/IEEE 15289:2019 clause 9 (progress/status reports); persisted via `--report <path>` and containing identification metadata (standard, clause, tool, version, timestamp), scope (srs_source, phases, force), and results (domain_count, requirement_count, created, skipped) |

### 1.4 References

| Document | Location |
|----------|----------|
| ISO/IEC/IEEE 29148:2018 | Requirements engineering standard (checks 55, 89, 118-120; this document conforms to) |
| ISO/IEC/IEEE 12207:2017 | Software life cycle processes (checks 9-10, 51-53, 56, 62, 64, 66, 82-88, 92, 96, 103, 109-113, 117, 120-122) |
| ISO/IEC/IEEE 15289:2019 | Content of life-cycle information items (checks 1-8, 14-20, 26-39, 48-50, 55, 60-68, 69-76, 99, 102-128); also defines the "Audit Report" information item — canonical filename for doc-engine output: `documentation_audit_report_v{version}.json` |
| ISO/IEC/IEEE 26514:2022 | Design and development of information for users (checks 58, 65, 67, 94, 104, 116) |
| ISO/IEC/IEEE 29119-3:2021 | Software testing -- Part 3: Test documentation (checks 59, 91, 99-102, 113, 125-128) |
| ISO/IEC/IEEE 42010:2022 | Architecture description (checks 48-50, 57, 90, 107-108) |
| ISO/IEC 25010:2023 | Product quality model, SQuaRE (checks 93, 97) |
| ISO/IEC 25040:2024 | Evaluation process, SQuaRE (check 98) |
| IEEE 1028:2008 | Standard for Software Reviews and Audits (checks 123, 124) |
| Documentation Framework | `/mnt/c/phd-systems/swe-labs/template-engine/templates/framework.md` |
| Compliance Checklist | `/mnt/c/phd-systems/swe-labs/template-engine/templates/compliance-checklist.md` |
| SEA Architecture Reference | `/mnt/c/phd-systems/swe-labs/langboot/rustratify/docs/3-design/architecture.md` |
| doc-engine Architecture | `../3-design/architecture.md` |
| doc-engine Implementation Plan | `../2-planning/implementation_plan.md` |

---

## 2. Stakeholder Requirements (StRS)

### 2.1 Stakeholders

| Stakeholder | Role | Needs |
|-------------|------|-------|
| Developer | Runs scans during local development | Fast feedback on doc compliance, clear violation messages |
| Architect | Audits projects, defines standards | Customizable rules, comprehensive coverage of 128 checks across 18 categories |
| Documentation maintainer | Tweaks rules without coding | Declarative TOML rules, no recompilation for simple changes |
| CI system | Automated gate in pipeline | JSON output, deterministic exit codes, non-interactive |
| Library consumer | Integrates scanning programmatically | Clean public API, well-typed report structures |

### 2.2 Operational Scenarios

#### OS-1: Developer local scan

A developer runs `doc-engine scan . --scope small` from their project root. The tool discovers all files, runs all checks applicable to the selected scope, and prints a text report showing which checks passed, failed, or were skipped. The developer fixes violations and re-runs until clean.

#### OS-2: CI pipeline gate

A CI job runs `doc-engine scan . --scope large --json`. The tool outputs a JSON report. The CI job parses the exit code: 0 passes the gate, 1 fails the build with violation details, 2 indicates a configuration error.

#### OS-3: Custom rules for internal project

An architect copies the default `rules.toml`, removes open-source-specific checks, adds an internal-only check for `INTERNAL_USAGE.md`, and runs `doc-engine scan . --rules internal.toml --type internal`.

#### OS-4: Documentation maintainer adds a check

A documentation maintainer needs to require a new file `docs/CODEOWNERS`. They add a `[[rules]]` entry with `type = "file_exists"` and `path = "docs/CODEOWNERS"` to `rules.toml`. No Rust code changes or recompilation needed.

#### OS-5: Library integration

Another Rust crate calls `doc_engine::scan_with_config(path, &config)` programmatically with a `ScanConfig` specifying `project_scope: ProjectScope::Large` and inspects the returned `ScanReport` to generate a compliance dashboard.

#### OS-6: YAML spec validation

An architect runs `doc-engine spec validate docs/` to verify all `.spec.yaml`, `.arch.yaml`, `.test.yaml`, and `.deploy.yaml` files parse correctly and conform to their schemas. The tool reports schema violations per file.

#### OS-7: Cross-reference analysis

An architect runs `doc-engine spec cross-ref docs/` to verify that all dependency references resolve, every feature request has matching test and architecture specs, the BRD inventory counts match actual files, and test cases trace to valid requirement IDs.

#### OS-8: Markdown generation from YAML

A developer runs `doc-engine spec generate docs/1-requirements/auth/login.spec.yaml --output generated/` to produce a markdown document from a YAML spec file, matching the template-engine template format.

#### OS-9: Audit report persistence

A developer or CI job runs `doc-engine scan . --scope large --json -o docs/7-operations/compliance/documentation_audit_report_v1.0.0.json`. The tool executes the scan, prints results to stdout, and persists the JSON report to the specified path (creating parent directories as needed). The filename follows the ISO/IEC/IEEE 15289:2019 "Audit Report" information item naming convention: `documentation_audit_report_v{version}.json`.

#### OS-10: SRS scaffold

An architect runs `doc-engine scaffold docs/1-requirements/srs.md --output . --force`. The tool parses the SRS, extracts all domains and their requirements, and generates 10 files per domain (spec.yaml, spec, arch.yaml, arch, test.yaml, test, manual.exec, auto.exec, deploy.yaml, deploy) plus 2 BRD files. The `.manual.exec` files provide actionable checklists with Steps and Expected columns for human testers. The `.auto.exec` files provide CI tracking tables with CI Job and Build columns. Both exec files list all test cases aligned row-for-row with the `.test` plan. The `--phase` flag filters output to specific SDLC phases (e.g., `--phase testing` generates only `docs/5-testing/` files).

### 2.3 Stakeholder Requirements

| ID | Requirement | Source | Priority | Rationale |
|----|-------------|--------|----------|-----------|
| STK-01 | The tool shall audit any project directory against 128 documentation compliance checks | Compliance Checklist | Must | Replaces manual bash-based auditing |
| STK-02 | Simple rules shall be modifiable without recompiling | Architect feedback | Must | Non-developers need to customize rules |
| STK-03 | The tool shall produce machine-readable output for CI integration | CI pipeline needs | Must | Enables automated compliance gating |
| STK-04 | The tool shall be usable as a Rust library | Library consumer needs | Should | Enables programmatic integration |
| STK-05 | The tool shall report clear, actionable violation messages with file paths | Developer feedback | Must | Developers need to locate and fix issues quickly |
| STK-06 | The tool shall run without network access | Security constraint | Must | Scans local file system only |
| STK-07 | The tool shall support both open-source and internal project types | Architect needs | Must | Different projects have different required files |
| STK-08 | The tool shall validate YAML spec files for schema conformance, cross-references, and generate markdown from them | Architect feedback | Should | Structured specs enable automated traceability and doc generation |
| STK-09 | The tool shall persist audit reports as versioned JSON files following ISO/IEC/IEEE 15289:2019 naming | CI pipeline needs, ISO compliance | Should | Enables audit trail, historical comparison, and compliance evidence |
| STK-10 | The tool shall scope checks by project size (small/medium/large) so that smaller projects are not burdened by large-project requirements | Developer feedback | Must | Different project sizes have different documentation needs |
| STK-11 | The tool shall scaffold a complete set of SDLC spec files from an SRS document, including actionable manual and automated test execution plans | Architect feedback | Should | Bootstraps documentation structure from requirements, ensuring consistent traceability from day one |

---

## 3. System Requirements (SyRS)

### 3.1 System Context

```
template-engine/templates/
├── framework.md              ← defines the standard
└── compliance-checklist.md   ← defines the original base checks (now extended to 128 total)
         │
         ▼
doc-engine/rules.toml         ← encodes checks as TOML rules
         │
         ▼
doc-engine scan <project> --scope <tier>  ← audits any project against them
         │
         ├── stdout (text or JSON)        ← results + exit code
         └── -o <path>                    ← documentation_audit_report_v{ver}.json
                                            (ISO/IEC/IEEE 15289:2019 Audit Report)
```

### 3.2 System Functions

| ID | Function | Description |
|----|----------|-------------|
| SYS-01 | Rule loading | Parse TOML rules file (embedded default or external override) |
| SYS-02 | File discovery | Recursively scan project directory for documentation files |
| SYS-03 | Check execution | Run each rule against the project (declarative or builtin) |
| SYS-04 | Result aggregation | Collect pass/fail/skip per check, compute summary |
| SYS-05 | Reporting | Output results as human-readable text or machine-readable JSON |
| SYS-06 | YAML spec processing | Parse, validate, cross-reference, and generate markdown from YAML spec files |
| SYS-07 | SRS scaffold | Parse SRS document, extract domains/requirements, generate per-domain SDLC spec files and test execution plans |

### 3.3 System Constraints

- **Language**: Rust (2021 edition)
- **Architecture**: Single-Crate Modular SEA
- **No async**: Synchronous file system operations only
- **No network**: Local file system scanning only
- **Platform**: Linux, macOS, Windows (via standard Rust cross-compilation)

### 3.4 Assumptions and Dependencies

- Projects being scanned follow a recognizable directory structure (not necessarily fully compliant)
- External crate dependencies: `walkdir`, `regex`, `toml`, `clap`, `serde`, `serde_json`, `serde_yaml`

---

## 4. Software Requirements (SRS)

### Requirement Attributes

Each requirement includes:

| Attribute | Description |
|-----------|-------------|
| **ID** | Unique identifier (FR-nnn for functional, NFR-nnn for non-functional) |
| **Priority** | Must / Should / May (MoSCoW) |
| **State** | Proposed / Approved / Implemented / Verified |
| **Verification** | Test / Inspection / Analysis / Demonstration |
| **Traces to** | Stakeholder requirement (STK-nn), architecture component |
| **Acceptance criteria** | Condition(s) that prove the requirement is met |

### 4.1 Rule Loading

#### FR-100: Default rules embedded in binary

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01, STK-02 -> `core/rules.rs` |
| **Acceptance** | When no `--rules` flag is provided, the engine loads rules from the embedded default and produces a valid `ScanReport` with 128 check results |

The binary shall embed a default `rules.toml` via `include_str!`. When no `--rules` flag is provided, the embedded rules are used.

#### FR-101: External rules file override

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-02 -> `core/rules.rs` |
| **Acceptance** | When `--rules custom.toml` is provided, only rules in `custom.toml` are executed; embedded defaults are ignored |

When `--rules <path>` is provided, the engine shall load and parse the external TOML file instead of the embedded default.

#### FR-102: TOML rules schema

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-02 -> `api/types.rs` (RuleDef, RuleType) |
| **Acceptance** | The TOML parser accepts all fields below without error; missing required fields produce exit code 2 |

Each rule entry shall contain:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | u8 | Yes | Unique check number (1-123) |
| `category` | string | Yes | Grouping category |
| `description` | string | Yes | Human-readable description |
| `severity` | string | Yes | `"error"`, `"warning"`, or `"info"` |
| `type` | string | Yes | Rule type (see FR-103, FR-104) |
| `project_type` | string | No | `"open_source"` or `"internal"` — if set, rule only runs for that type |
| `path` | string | Conditional | Required for file/dir rule types |
| `pattern` | string | Conditional | Required for content/naming rule types |
| `glob` | string | Conditional | Required for glob rule types |
| `handler` | string | Conditional | Required for builtin rule types |
| `message` | string | No | Custom failure message |
| `exclude_paths` | string[] | No | Path prefixes to exclude from glob matching |
| `exclude_pattern` | string | No | Regex pattern for lines to exclude |

#### FR-103: Declarative rule types

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-02 -> `core/declarative.rs` |
| **Acceptance** | Each of the 9 rule types produces correct Pass/Fail results when tested against a fixture project |

The engine shall support 9 declarative rule types:

| Type | Behavior |
|------|----------|
| `file_exists` | Pass if `root/<path>` exists as a file |
| `dir_exists` | Pass if `root/<path>` exists as a directory |
| `dir_not_exists` | Pass if `root/<path>` does NOT exist as a directory |
| `file_content_matches` | Pass if file at `path` contains regex `pattern` |
| `file_content_not_matches` | Pass if file at `path` does NOT contain regex `pattern` |
| `glob_content_matches` | Pass if ALL files matching `glob` contain regex `pattern` |
| `glob_content_not_matches` | Pass if NO files matching `glob` contain regex `pattern` |
| `glob_naming_matches` | Pass if ALL filenames matching `glob` match regex `pattern` |
| `glob_naming_not_matches` | Pass if NO filenames matching `glob` match regex `pattern` |

#### FR-104: Builtin rule types

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/builtins/` |
| **Acceptance** | Each handler produces correct results when tested against compliant and non-compliant fixture projects |

When `type = "builtin"`, the engine shall look up a Rust handler by `handler` name. Supported handlers:

| Handler | Description |
|---------|-------------|
| `module_docs_plural` | Scan for `doc/` vs `docs/` in module directories |
| `sdlc_phase_numbering` | Validate SDLC phase directory numbering and ordering |
| `checklist_completeness` | Verify every enforceable rule in compliance checklist has a checkbox |
| `snake_lower_case` | Validate docs filenames are lowercase, underscore-separated, space-free |
| `guide_naming` | Validate guide naming convention `name_{phase}_guide.md` |
| `testing_file_placement` | Reject `*_testing_*` files outside `5-testing/` |
| `tldr_conditional` | Require TLDR on 200+ line docs, flag on shorter |
| `glossary_format` | Validate `**Term** - Definition.` format |
| `glossary_alphabetized` | Validate alphabetical ordering of glossary terms |
| `glossary_acronyms` | Validate acronym expansions are present |
| `w3h_hub` | Detect W3H structure in docs hub |
| `hub_links_phases` | Verify hub links to all present SDLC directories |
| `no_deep_links` | Reject deep links from root README into docs/ subdirs |
| `link_resolution` | Resolve all markdown links to existing files |
| `adr_naming` | Validate `NNN-title.md` naming convention |
| `adr_index_completeness` | Cross-reference ADR index against ADR files |
| `open_source_community_files` | Check CODE_OF_CONDUCT.md, SUPPORT.md |
| `open_source_github_templates` | Check .github/ISSUE_TEMPLATE/, PULL_REQUEST_TEMPLATE.md |
| `phase_artifact_presence` | Verify SDLC phase dirs contain expected artifacts |
| `design_traces_requirements` | Design docs reference requirements |
| `plan_traces_design` | Planning docs reference architecture |
| `backlog_traces_requirements` | Backlog references requirements/SRS |
| `templates_populated` | Verify docs/templates/ contains template files |
| `w3h_extended` | W3H structure enforcement in hub documents |
| `readme_line_count` | Root README.md under 100 lines |
| `fr_naming` | FR artifacts follow FR_NNN naming |
| `module_readme_w3h` | Module READMEs follow W3H structure |
| `module_examples_tests` | Modules have examples directory and integration tests |
| `module_toolchain_docs` | Modules have toolchain documentation |
| `module_deployment_docs` | Module deployment docs complete |
| `srs_29148_attributes` | SRS requirements have ISO 29148 attribute tables |
| `arch_42010_sections` | Architecture documents have ISO 42010 sections |
| `test_29119_sections` | Testing strategies have ISO 29119-3 sections |
| `prod_readiness_exists` | Production readiness document exists |
| `prod_readiness_25010_sections` | Production readiness has ISO 25010 quality sections |
| `dev_guide_26514_sections` | Developer guide has ISO 26514 sections |
| `backlog_sections` | Backlog has required sections (items, completed, blockers) |
| `prod_readiness_12207_sections` | Production readiness has ISO 12207 lifecycle sections |
| `prod_readiness_25010_supp_sections` | Production readiness has supplementary ISO 25010 quality sections |
| `prod_readiness_25040_sections` | Production readiness has ISO 25040 evaluation sections |
| `spec_brd_exists` | Check BRD spec file exists in 1-requirements/ |
| `spec_domain_coverage` | Check domain directories have .spec.yaml files |
| `spec_schema_valid` | Validate all .spec/.arch/.test/.deploy.yaml files parse and conform to schema |
| `spec_id_format` | Validate spec IDs match `[A-Z]+-\d{3}` pattern (both formats) |
| `spec_no_duplicate_ids` | Check for duplicate spec IDs across spec files (both formats) |
| `spec_test_coverage` | Check every spec has a matching test plan |
| `spec_deps_resolve` | Validate dependency references resolve to existing specs |
| `spec_inventory_accuracy` | Validate BRD inventory counts match actual file counts |
| `spec_links_resolve` | Validate relatedDocuments paths resolve to existing files |
| `spec_test_traces` | Validate test `verifies` fields trace to valid requirement IDs |
| `spec_naming_convention` | Validate spec filenames follow snake_lower_case convention (both formats) |
| `spec_stem_consistency` | Verify spec file stems match across SDLC phases |

#### FR-105: Unknown handler error

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-05 -> `core/builtins/mod.rs` |
| **Acceptance** | A rules file with `handler = "nonexistent"` produces exit code 2 and a message naming the unknown handler |

If a `builtin` rule references an unknown handler name, the engine shall return a scan error (exit code 2) with a descriptive message.

### 4.2 File Discovery

#### FR-200: Recursive scanning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | Given a project with nested directories 5 levels deep, all files are discovered |

The scanner shall recursively discover all files under the provided project root directory.

#### FR-201: Directory exclusions

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | Files inside `.git/`, `target/`, and `node_modules/` are not included in the file list |

The scanner shall skip: hidden directories (names starting with `.`), `target/`, `node_modules/`.

#### FR-202: Relative paths

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | All paths in the file list are relative (no leading `/` or absolute prefix) |

All discovered file paths shall be relative to the project root.

### 4.3 Check Execution

#### FR-300: All checks (83 base + 15 spec)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01, STK-08 -> `core/engine.rs`, `core/builtins/spec.rs`, `core/builtins/traceability.rs`, `core/builtins/requirements.rs`, `rules.toml` |
| **Acceptance** | Default `rules.toml` contains 128 rules; a full scan produces 128 check results |

The engine shall support 128 checks:

| Category | Check IDs | Count |
|----------|-----------|-------|
| structure | 1-13, 69, 72-73 | 16 |
| naming | 14-25, 76 | 13 |
| root_files | 26-32, 70 | 8 |
| content | 33-39, 75 | 8 |
| navigation | 40-43, 74 | 5 |
| cross_ref | 44-47 | 4 |
| adr | 48-50 | 3 |
| traceability | 51-53, 82, 121-124 | 8 |
| ideation | 54, 118 | 2 |
| requirements | 55, 89-98, 119-120 | 13 |
| planning | 56, 83-88, 109-113 | 12 |
| design | 57, 107-108 | 3 |
| development | 58, 103-106 | 5 |
| testing | 59, 99-102, 125-128 | 9 |
| deployment | 60-62, 68, 114-116 | 7 |
| operations | 63-67, 117 | 6 |
| backlog | 71 | 1 |
| module | 77-81 | 5 |
| **Total** | | **128** |

#### FR-301: Check filtering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/engine.rs` |
| **Acceptance** | `--checks 1-13` produces exactly 13 results; `--checks 1,2,3` produces exactly 3 |

When `--checks` is provided with a range or comma-separated list, only matching checks shall be executed.

#### FR-302: Project type filtering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-07 -> `core/engine.rs` |
| **Acceptance** | Rules with `project_type = "open_source"` are skipped when `--type internal` is used, and vice versa |

When a rule has `project_type` set, it shall only run if the scan's project type matches.

#### FR-303: Check result types

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-05 -> `spi/types.rs` |
| **Acceptance** | The `CheckResult` enum has exactly three variants: Pass, Fail (with violations), Skip (with reason) |

Each check shall produce one of: **Pass**, **Fail** (with `Violation` records), or **Skip** (with reason string).

#### FR-304: Violation record

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-05 -> `spi/types.rs` |
| **Acceptance** | Each `Violation` contains check ID, optional file path, message, and severity |

Each violation shall contain: Check ID (u8), file path (optional), message (string), severity (error/warning/info).

### 4.4 Reporting

#### FR-400: Text output (default)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-05 -> `core/reporter.rs` |
| **Acceptance** | Running without `--json` prints grouped results with check IDs, descriptions, statuses, violations, and a summary line |

Default output shall be human-readable text grouped by category, showing check ID, description, pass/fail/skip, violations with file paths, and a summary line.

#### FR-401: JSON output

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-03 -> `core/reporter.rs` |
| **Acceptance** | `--json` output parses as valid JSON and deserializes to `ScanReport` |

When `--json` is provided, output shall be a JSON `ScanReport`.

#### FR-402: Exit codes

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-03 -> `main.rs` |
| **Acceptance** | Clean project returns 0; project with violations returns 1; invalid path returns 2 |

| Code | Meaning |
|------|---------|
| 0 | All executed checks passed |
| 1 | One or more checks failed |
| 2 | Scan error (invalid path, IO error, rules parse error, unknown handler, unknown scope) |

#### FR-403: Report file output

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-09 -> `main.rs` |
| **Acceptance** | `--output <path>` writes a JSON audit report to the specified path, creating parent directories as needed; the recommended filename follows ISO/IEC/IEEE 15289:2019: `documentation_audit_report_v{version}.json` |

When `--output` / `-o` is provided, the engine shall persist the scan report as JSON to the given file path, regardless of whether `--json` was specified for stdout. Parent directories are created automatically. Stdout output is unaffected. If the file cannot be written, the tool exits with code 2.

#### FR-831: Audit status report (ISO 15289 clause 9.2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-09 -> `main.rs`, `core/reporter.rs` |
| **Acceptance** | `--output <path>` persists a JSON audit status report conforming to ISO/IEC/IEEE 15289:2019 clause 9.2; the report contains: standard, clause, tool, tool_version, timestamp (ISO 8601 UTC), project_root (absolute path), project_type, project_scope, results, summary |

When `--output <path>` is provided, the scan command shall serialize an `AuditStatusReport` as pretty-printed JSON. The report wraps `ScanReport` fields (promoted to top level) with ISO 15289:2019 clause 9.2 identification metadata: standard identifier, clause reference, tool name, tool version (from `Cargo.toml`), ISO 8601 UTC timestamp, and canonicalized project root path. The `--json` stdout output is unaffected and continues to emit the bare `ScanReport`. No new dependencies are introduced.

### 4.5 CLI Interface

#### FR-500: Scan command

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-01 -> `main.rs` |
| **Acceptance** | `doc-engine scan <PATH> --scope <TIER>` executes a scoped scan and prints results |

```
doc-engine scan <PATH> --scope <small|medium|large> [--json] [--checks N] [--type TYPE] [--rules FILE] [-o FILE]
```

#### FR-501: JSON flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-03 -> `main.rs` |
| **Acceptance** | `doc-engine scan <PATH> --json` outputs valid JSON |

#### FR-502: Check filter flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> `main.rs` |
| **Acceptance** | `--checks 1-13` runs exactly 13 checks; `--checks 1,2,3,14-25` runs exactly 15 |

Supports ranges and comma-separated values.

#### FR-503: Project type flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-07 -> `main.rs` |
| **Acceptance** | `--type internal` skips open-source-only rules; `--type open-source` is the default |

#### FR-504: Rules file flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-02 -> `main.rs` |
| **Acceptance** | `--rules custom.toml` loads and uses the specified file; missing file produces exit code 2 |

#### FR-505: Scope flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-10 -> `main.rs` |
| **Acceptance** | `--scope small` runs only small-tier checks, skipping medium and large; `--scope medium` runs small+medium; `--scope large` runs all; unknown values produce exit code 2 |

Required flag. Each rule in `rules.toml` carries a `scope = "small"|"medium"|"large"` attribute. Rules whose scope exceeds the configured tier are skipped with a descriptive reason. `ProjectScope` derives `Ord` so that `Small < Medium < Large`.

#### FR-506: Output flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-09 -> `main.rs` |
| **Acceptance** | `--output <path>` or `-o <path>` writes a JSON report to the specified path; parent directories are created automatically; the recommended filename is `documentation_audit_report_v{version}.json` per ISO/IEC/IEEE 15289:2019 |

### 4.6 Library API

#### FR-600: Public scan function

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Superseded by FR-601 |
| **Verification** | — |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | Removed: `scan()` convenience function was deleted when `ProjectScope` became mandatory. Use `scan_with_config()` (FR-601) instead. |

~~The library shall expose `scan(root: &Path) -> Result<ScanReport, ScanError>` via the SAF layer.~~ Superseded: since `ProjectScope` is a required field in `ScanConfig`, a no-argument convenience is no longer meaningful.

#### FR-601: Configurable scan function

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | `doc_engine::scan_with_config(path, &config)` respects `ScanConfig` fields and returns `Result<ScanReport, ScanError>` |

The library shall expose `scan_with_config(root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError>` via the SAF layer.

#### FR-602: Public types

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | All listed types are importable from `doc_engine::` |

Public via SAF: `ScanConfig`, `ScanReport`, `ScanSummary`, `ProjectType`, `ProjectScope`, `CheckId`, `CheckResult`, `Severity`, `Violation`, `RuleDef`, `RuleType`, `SpecFormat`, `SpecKind`, `SpecStatus`, `Priority`, `DiscoveredSpec`, `BrdSpec`, `FeatureRequestSpec`, `ArchSpec`, `TestSpec`, `DeploySpec`, `MarkdownSpec`, `MarkdownTestCase`, `SpecEnvelope`, `ParsedSpec`, `SpecValidationReport`, `CrossRefReport`, `SpecDiagnostic`, `CrossRefResult`.

### 4.7 Spec File Parsing

#### FR-700: Dual-format spec file parsing

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/parser.rs` |
| **Acceptance** | Valid `.spec.yaml` files parse via serde_yaml; valid `.spec` markdown files have metadata extracted via regex; invalid files of either format produce a `SpecDiagnostic` with file path and error message |

The engine shall parse spec files in two formats:
- **YAML format** (`.spec.yaml`, `.arch.yaml`, `.test.yaml`, `.deploy.yaml`): parsed using `serde_yaml`, dispatched by `kind` field
- **Markdown format** (`.spec`, `.arch`, `.test`, `.deploy`): parsed using regex-based metadata extraction of structured headers (`**Version:**`, `**Status:**`, `**Spec:**`, `**Related:**`)

#### FR-701: Kind-based dispatch

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/parser.rs` |
| **Acceptance** | A YAML file with `kind: brd` deserializes to `BrdSpec`; `kind: feature_request` to `FeatureRequestSpec`; `kind: architecture` to `ArchSpec`; `kind: test_plan` to `TestSpec`; `kind: deployment` to `DeploySpec`. A markdown `.spec` file produces a `MarkdownSpec` with extracted metadata. |

For YAML specs, the `kind` field selects the deserialization target. For markdown specs, the file extension determines the kind (`.spec` -> spec, `.arch` -> architecture, `.test` -> test_plan, `.deploy` -> deployment).

#### FR-702: Spec file discovery

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/discovery.rs` |
| **Acceptance** | Given a project with spec files in `1-requirements/`, `3-design/`, `5-testing/`, and `6-deployment/`, all spec files of both formats are discovered, categorized by extension, and tagged with their format (YAML or markdown) |

The engine shall discover all spec files by extension under the project root:
- YAML: `.spec.yaml`, `.arch.yaml`, `.test.yaml`, `.deploy.yaml`
- Markdown: `.spec`, `.arch`, `.test`, `.deploy`

#### FR-703: Parse error reporting

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-05, STK-08 -> `core/spec/parser.rs`, `spi/spec_types.rs` |
| **Acceptance** | A YAML file with a syntax error produces a `SpecDiagnostic` containing the file path, line number (if available), and a descriptive error message. A markdown spec missing required metadata headers produces a `SpecDiagnostic`. |

Parse errors shall be reported as `SpecDiagnostic` records with file path, location, and descriptive message.

#### FR-704: Markdown spec metadata extraction

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/parser.rs` |
| **Acceptance** | A markdown `.spec` file with `**Version:** 0.1.0`, `**Status:** Draft`, and `**Related:** RS-001` headers has all three values extracted. A `.test` file with a `**Spec:**` header containing a linked name and path has the spec link extracted. |

The parser shall extract structured metadata from markdown spec files using regex patterns:
- `**Version:**` — spec version
- `**Status:**` — lifecycle status (Draft, Approved, etc.)
- `**Related:**` — related spec ID
- `**Spec:**` — link to parent spec (in `.arch`, `.test`, `.deploy` files)
- `**Arch:**` — link to architecture doc (in `.test` files)
- `**Requirements:**` — requirement ID range (in `.test` files)

#### FR-705: Markdown test table parsing

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/parser.rs` |
| **Acceptance** | A `.test` file with a markdown table containing `| Verifies |` column has all requirement IDs extracted from that column |

The parser shall extract test-to-requirement traceability from markdown `.test` files by parsing tables with `Verifies` columns.

#### FR-706: Feature stem matching

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/discovery.rs`, `core/spec/cross_ref.rs` |
| **Acceptance** | Given `compiler_design.spec`, `compiler_design.arch`, `compiler_design.test`, `compiler_design.deploy`, all four files are linked as a single SDLC chain via the shared stem `compiler_design` |

Both formats use **feature stem** matching — the filename without its domain extension (e.g., `compiler_design` from `compiler_design.spec`, `login` from `login.spec.yaml`). Spec IDs (e.g., `RS-030`) live inside file content, not in filenames.

### 4.8 Spec Schema Validation

#### FR-710: Required fields validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs` |
| **Acceptance** | A `.spec.yaml` missing `kind`, `schemaVersion`, or `title` produces a validation diagnostic. A markdown `.spec` missing `**Version:**` or `**Status:**` produces a validation diagnostic. |

Each spec kind shall have a set of required fields per format. Missing required fields shall produce validation diagnostics. For YAML specs: `kind`, `schemaVersion`, `title`. For markdown specs: `**Version:**`, `**Status:**`, and a top-level `#` heading.

#### FR-711: BRD schema validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs`, `api/spec_types.rs` |
| **Acceptance** | A valid `brd.spec.yaml` with `kind: brd`, `schemaVersion`, `title`, `domains[]` (each with `name`, `specCount`, `specs[]`) passes validation |

BRD spec files shall require: `kind`, `schemaVersion`, `title`, `domains` (array of domain entries each containing `name`, `specCount`, and `specs` array).

#### FR-712: Feature request schema validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs`, `api/spec_types.rs` |
| **Acceptance** | A valid `login.spec.yaml` with `kind: feature_request`, `id`, `title`, `status`, `priority`, `requirements[]` passes validation |

Feature request spec files shall require: `kind`, `schemaVersion`, `id` (matching `[A-Z]+-\d{3}` pattern), `title`, `status`, `priority`, `requirements` array. Optional: `dependencies`, `relatedDocuments`.

#### FR-713: Architecture schema validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs`, `api/spec_types.rs` |
| **Acceptance** | A valid `.arch.yaml` with `kind: architecture`, `spec` (spec ID reference), `components[]` passes validation |

Architecture spec files shall require: `kind`, `schemaVersion`, `spec` (spec ID reference), `title`, `components` array. Optional: `dependencies`, `relatedDocuments`.

#### FR-714: Test plan schema validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs`, `api/spec_types.rs` |
| **Acceptance** | A valid `.test.yaml` with `kind: test_plan`, `spec` (spec ID reference), `testCases[]` (each with `verifies` field) passes validation |

Test plan spec files shall require: `kind`, `schemaVersion`, `spec` (spec ID reference), `title`, `testCases` array (each with `id`, `description`, `verifies` field referencing requirement IDs).

#### FR-715: Deployment schema validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs`, `api/spec_types.rs` |
| **Acceptance** | A valid `.deploy.yaml` with `kind: deployment`, `spec` (spec ID reference), `environments[]` passes validation |

Deployment spec files shall require: `kind`, `schemaVersion`, `spec` (spec ID reference), `title`, `environments` array.

#### FR-716: Duplicate ID detection

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/validate.rs` |
| **Acceptance** | Two spec files (of either format) with the same spec ID produce a validation diagnostic listing both file paths |

The validator shall detect duplicate spec IDs across all spec files (both YAML `id` fields and markdown `**Related:**` values) and report each duplicate with both file paths.

### 4.9 Cross-Referencing

#### FR-720: Dependency resolution

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/cross_ref.rs` |
| **Acceptance** | A `.spec.yaml` with `dependencies: [{ref: "FR-002", file: "auth/signup.spec.yaml"}]` passes if the referenced file exists and contains the referenced ID; fails if it does not |

Each dependency entry's `ref` and `file` fields shall be validated: the referenced file must exist and contain the referenced spec ID.

#### FR-721: SDLC chain completeness

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/cross_ref.rs` |
| **Acceptance** | A `.spec` with no matching `.test` produces a cross-ref diagnostic. A `.spec.yaml` with no matching `.test.yaml` produces a cross-ref diagnostic. Both formats are checked independently. |

Each spec file should have matching architecture and test files. Both formats use **feature stem** matching — the shared filename without the domain extension (e.g., `compiler_design.spec` matches `compiler_design.arch` and `compiler_design.test`; `login.spec.yaml` matches `login.arch.yaml` and `login.test.yaml`). Spec IDs live inside file content, not in filenames.

Missing SDLC chain links shall produce diagnostics.

#### FR-722: BRD inventory accuracy

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/cross_ref.rs` |
| **Acceptance** | A BRD with `specCount: 3` for domain "auth" but only 2 actual spec files produces a diagnostic; all `specs[].file` paths must resolve |

For YAML BRDs: the `specCount` per domain shall match the actual count of spec files in that domain directory; each `specs[].file` reference shall resolve to an existing file. For markdown BRDs: the spec inventory table counts shall match actual `.spec` file counts per domain directory.

#### FR-723: Test traceability

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/cross_ref.rs` |
| **Acceptance** | A YAML test case with `verifies: "REQ-001"` passes if the linked spec contains that ID. A markdown `.test` table row with `Verifies` = `DESIGN-001` passes if the linked `.spec` defines that ID. |

Each test case's verifies reference shall match a valid requirement ID. For YAML specs: the `verifies` field in `testCases[]`. For markdown specs: the `Verifies` column in test tables.

#### FR-724: Architecture traceability

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/cross_ref.rs` |
| **Acceptance** | An `.arch.yaml` with `spec: "FR-001"` passes if a matching `.spec.yaml` exists. A markdown `.arch` with a `**Spec:**` header containing a linked spec name passes if the linked `.spec` file exists. |

Each architecture file shall trace to a spec file. For YAML: the `spec` field references a spec ID. For markdown: the `**Spec:**` header contains a link to the parent `.spec` file.

#### FR-725: Related documents resolution

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/cross_ref.rs` |
| **Acceptance** | A spec with `relatedDocuments: ["../3-design/architecture.md"]` passes if the path resolves; fails if it does not |

All `relatedDocuments` paths in any spec file shall resolve to existing files relative to the project root.

#### FR-726: Cross-reference report

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `api/spec_types.rs`, `core/spec/cross_ref.rs` |
| **Acceptance** | `CrossRefReport` contains categorized results (dependency, sdlc_chain, inventory, test_trace, arch_trace, related_docs) with pass/fail per check |

Cross-reference analysis shall produce a `CrossRefReport` with results categorized by check type.

#### FR-727: Opt-in spec checking

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Superseded |
| **Verification** | — |
| **Traces to** | STK-08 -> `core/builtins/spec.rs` |
| **Acceptance** | Superseded: IDs 54-68 are now phase artifact `file_exists` checks (FR-815). Spec checks use dedicated builtin handlers with their own IDs. |

~~Spec checks (54-68) shall be opt-in: if no spec files of either format exist in the project, all spec checks shall produce `Skip` results.~~ Superseded: IDs 54-68 were reassigned to phase artifact file existence checks per FR-815.

### 4.10 Markdown Generation

#### FR-730: Spec-to-markdown generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `core/spec/generate.rs` |
| **Acceptance** | Given a valid `login.spec.yaml`, the generator produces a markdown file with title, requirements table, dependencies, and related documents sections |

The engine shall generate markdown documents from YAML spec files, matching the template-engine template format.

#### FR-731: BRD markdown generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `core/spec/generate.rs` |
| **Acceptance** | Given a valid `brd.spec.yaml`, the generator produces a markdown document with domain inventory table and links to individual specs |

#### FR-732: Architecture markdown generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `core/spec/generate.rs` |
| **Acceptance** | Given a valid `.arch.yaml`, the generator produces a markdown document with component descriptions and dependency diagrams |

#### FR-733: Test plan markdown generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `core/spec/generate.rs` |
| **Acceptance** | Given a valid `.test.yaml`, the generator produces a markdown document with test case table including verifies traceability |

#### FR-734: Deployment markdown generation

| Attribute | Value |
|-----------|-------|
| **Priority** | May |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `core/spec/generate.rs` |
| **Acceptance** | Given a valid `.deploy.yaml`, the generator produces a markdown document with environment configuration table |

#### FR-735: Output path control

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/spec/generate.rs` |
| **Acceptance** | `--output <dir>` writes generated markdown to the specified directory; default writes to stdout |

### 4.11 Scan Pipeline Integration

#### FR-740: Spec checks in scan pipeline

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Superseded |
| **Verification** | — |
| **Traces to** | STK-08 -> `core/builtins/spec.rs`, `rules.toml` |
| **Acceptance** | Superseded: IDs 54-68 are now phase artifact `file_exists` checks (FR-815). Spec checks use dedicated builtin handlers. |

~~Checks 54-68 shall be integrated into the standard scan pipeline as builtin handlers in `rules.toml`.~~ Superseded: IDs 54-68 were reassigned to phase artifact checks per FR-815.

#### FR-741: Spec check category

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Superseded |
| **Verification** | — |
| **Traces to** | STK-08 -> `rules.toml` |
| **Acceptance** | Superseded: IDs 54-68 now use per-phase categories (ideation, requirements, planning, design, development, testing, deployment, operations). |

~~All spec checks shall use the `spec` category for grouping in reports.~~ Superseded: IDs 54-68 use per-phase categories per FR-815.

#### FR-742: Spec check descriptions

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-05, STK-08 -> `rules.toml` |
| **Acceptance** | Each check 54-68 and 99-128 has a unique, descriptive `description` field in `rules.toml` |

### 4.12 Spec Subcommand

#### FR-750: Spec validate subcommand

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `main.rs` |
| **Acceptance** | `doc-engine spec validate <PATH>` validates all spec files under PATH and prints schema diagnostics |

```
doc-engine spec validate <PATH> [--json]
```

#### FR-751: Spec cross-ref subcommand

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `main.rs` |
| **Acceptance** | `doc-engine spec cross-ref <PATH>` analyzes cross-references and prints a categorized report |

```
doc-engine spec cross-ref <PATH> [--json]
```

#### FR-752: Spec generate subcommand

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `main.rs` |
| **Acceptance** | `doc-engine spec generate <FILE> --output <DIR>` produces a markdown file from the given YAML spec |

```
doc-engine spec generate <FILE> [--output <DIR>]
```

#### FR-753: Spec subcommand exit codes

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `main.rs` |
| **Acceptance** | Exit code 0 = clean, 1 = violations found, 2 = error (same semantics as `scan`) |

The `spec` subcommand shall use the same exit code semantics as `scan`: 0 = clean, 1 = violations, 2 = error.

#### FR-754: Spec JSON output

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-03, STK-08 -> `core/reporter.rs` |
| **Acceptance** | `doc-engine spec validate <PATH> --json` outputs valid JSON deserializable to `SpecValidationReport`; `doc-engine spec cross-ref <PATH> --json` outputs valid JSON deserializable to `CrossRefReport` |

The `--json` flag on spec subcommands shall produce JSON output matching the report types.

#### FR-755: Spec text output

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-05, STK-08 -> `core/reporter.rs` |
| **Acceptance** | Default (no `--json`) output is human-readable text with file paths, diagnostic messages, and a summary line |

### 4.13 Planned Check Behavioral Requirements

These requirements specify the behavioral constraints for checks identified in the [backlog](../2-planning/backlog.md). They apply to checks 54+ including phase artifact existence checks (54-68, 99-128) and builtin behavioral checks (69+).

#### FR-800: Module discovery strategy

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | BL-03, BL-04, BL-05, BL-06 -> `core/builtins/` |
| **Acceptance** | Module discovery detects Rust (`Cargo.toml`), JavaScript/TypeScript (`package.json`), Python (`setup.py`, `pyproject.toml`), and Java (`pom.xml`, `build.gradle`) project manifests. Projects without recognized manifests produce Skip for all module-level checks. |

Module-level checks (BL-03 through BL-06) depend on reliably discovering modules/crates. The engine shall identify modules by scanning for language-specific manifest files under the project root. A shared `ModuleDiscovery` component shall support multiple languages and be reusable across all module-level handlers.

#### FR-801: W3H detection scope

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | BL-03, BL-09 -> `core/builtins/` |
| **Acceptance** | W3H validation uses flexible regex `(?i)^##\s*(what|why|how)` or equivalent prose markers. W3H enforcement is limited to hub documents (`docs/README.md`, `docs/3-design/architecture.md`, `docs/4-development/developer_guide.md`) and module READMEs (`**/docs/README.md`). General docs are excluded. |

Hub documents and module READMEs have predictable structure suitable for pattern matching. General docs across `docs/3-design/*.md` and `docs/4-development/guide/*.md` use too many heading conventions for reliable automated W3H detection. Limiting scope avoids false positives.

#### FR-802: Module deployment check skip behavior

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | BL-06 -> `core/builtins/` |
| **Acceptance** | Module deployment checks produce Skip (not Fail) for modules that have no `docs/6-deployment/` directory. Only modules with an existing `docs/6-deployment/` directory are validated for required files (`README.md`, `prerequisites.md`, `installation.md`). |

Library crates consumed as dependencies are not independently deployable and should not be required to have deployment documentation. The check triggers only when a module has opted in by creating the deployment directory.

#### FR-803: Feature-prefixed artifact check opt-in

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | BL-11 -> `core/builtins/` |
| **Acceptance** | Feature naming checks produce Skip if no files or directories matching `FR_\d{3}` exist anywhere in the project. When FR-prefixed artifacts do exist, folders must match `FR_\d{3}/` and files must match `FR_\d{3}_.+`. Hyphens (`FR-###`) in file paths produce a warning. |

Not all projects use formal feature request tracking. The check is opt-in: it activates only when the project already contains FR-prefixed artifacts, then validates they follow the underscore convention.

#### FR-804: Planning phase artifact verification

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Checks 83-88 -> `rules.toml` (declarative `file_exists`) |
| **Acceptance** | The tool shall verify that standard planning phase artifacts exist under `docs/2-planning/`: risk register (`risk_register.md`), estimation records (`estimation.md`), schedule (`schedule.md`), resource plan (`resource_plan.md`), communication plan (`communication_plan.md`), and quality plan (`quality_plan.md`). Each missing artifact produces an `info`-level finding. Schedule, resource plan, and communication plan are scoped to `open_source` projects only and produce `Skip` for internal projects. |

Traditional software engineering planning phases produce artifacts beyond the implementation plan and backlog. Risk registers, estimation records, and quality plans are universally valuable regardless of project size. Schedule, resource plan, and communication plan are primarily relevant to larger open-source projects with multiple contributors and stakeholders; internal/small projects may skip these without penalty.

#### FR-805: SRS 29148 attribute validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 89 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 89 validates that every FR-xxx and NFR-xxx block in `docs/1-requirements/srs.md` contains the five mandatory ISO/IEC/IEEE 29148:2018 attributes: Priority, State, Verification, Traces to (or Traceability), and Acceptance. Missing attributes produce per-requirement violations. Projects without an SRS file or without FR/NFR blocks produce Skip. STK-xxx blocks are excluded (they use a consolidated table format). |

The engine shall validate SRS documents for ISO/IEC/IEEE 29148:2018 compliance by checking that each requirement block has the five mandatory attribute table entries.

#### FR-806: Architecture 42010 attribute validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 90 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 90 validates that architecture documents at both project level (`docs/3-design/architecture.md`) and module level (`<module>/docs/3-design/architecture.md`) contain key ISO/IEC/IEEE 42010:2022 sections: stakeholder identification, architectural concerns/rationale, and viewpoints/views. Missing sections produce per-file violations. Projects and modules without an architecture file or with an empty file are skipped. If no architecture files exist anywhere, the check produces Skip. W3H sections (Who/Why/What/How) satisfy the 42010 requirements. |

The engine shall validate architecture documents at project and module level for ISO/IEC/IEEE 42010:2022 compliance by checking for stakeholder, concern, and viewpoint sections.

#### FR-807: Testing 29119-3 attribute validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 91 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 91 validates that testing strategy documents at both project level (`docs/5-testing/testing_strategy.md`) and module level (`<module>/docs/5-testing/testing_strategy.md`) contain key ISO/IEC/IEEE 29119-3:2021 sections: test strategy/scope, test cases/categories, and coverage targets/criteria. Missing sections produce per-file violations. Projects and modules without a testing strategy file or with an empty file are skipped. If no testing strategy files exist anywhere, the check produces Skip. |

The engine shall validate testing strategy documents at project and module level for ISO/IEC/IEEE 29119-3:2021 compliance by checking for test design, test case, and test procedure sections.

#### FR-808: Production readiness document existence

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 92 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 92 validates that `docs/6-deployment/production_readiness.md` exists. Projects without this file produce Skip. |

The engine shall verify that a production readiness review document exists at the project level, per ISO/IEC 25010:2023 quality assessment practices.

#### FR-809: Production readiness ISO/IEC 25010:2023 quality sections

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 93 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 93 validates that `docs/6-deployment/production_readiness.md` contains sections for Security, Test Coverage, Observability, Backwards Compatibility, Runtime Safety, and Verdict. Missing sections produce per-section violations. Projects without the file produce Skip. |

The engine shall validate production readiness documents for core ISO/IEC 25010:2023 quality characteristics: security, reliability (runtime safety), maintainability (test coverage), portability (backwards compatibility), and usability (observability).

#### FR-810: Developer guide ISO/IEC/IEEE 26514:2022 sections

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 94 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 94 validates that `docs/4-development/developer_guide.md` contains sections for Build & Test, Project Structure, and Adding New Features. Missing sections produce per-section violations. Projects without the file produce Skip. |

The engine shall validate developer guide documents for ISO/IEC/IEEE 26514:2022 compliance by checking for task analysis (build & test), information structure (project layout), and writing guidelines (extensibility) sections.

#### FR-811: Backlog content sections

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 95 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 95 validates that `docs/2-planning/backlog.md` contains sections for Backlog Items (or High Priority), Completed, and Blockers. Missing sections produce per-section violations. Projects without the file produce Skip. |

The engine shall validate backlog documents for required content sections: active items, completed items with dates, and blockers with impact/owner/status tracking.

#### FR-812: Production readiness ISO/IEC/IEEE 12207:2017 lifecycle sections

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 96 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 96 validates that `docs/6-deployment/production_readiness.md` contains sections for CI/CD Pipeline, Dependency Health, Dependency Auditing, Package Metadata, and Release Automation. Missing sections produce per-section violations. Projects without the file produce Skip. |

The engine shall validate production readiness documents for ISO/IEC/IEEE 12207:2017 lifecycle process areas: infrastructure management (CI/CD, dependencies), configuration management (package metadata), and transition (release automation).

#### FR-813: Production readiness supplementary ISO/IEC 25010:2023 quality sections

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 97 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 97 validates that `docs/6-deployment/production_readiness.md` contains sections for Static Analysis, API Documentation, README & Onboarding, and Documentation Lint. Missing sections produce per-section violations. Projects without the file produce Skip. |

The engine shall validate production readiness documents for supplementary ISO/IEC 25010:2023 quality characteristics: security integrity (static analysis), maintainability analysability (API documentation), usability learnability (README & onboarding), and maintainability modularity (documentation lint).

#### FR-814: Production readiness ISO/IEC 25040:2024 evaluation sections

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 98 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 98 validates that `docs/6-deployment/production_readiness.md` contains Scoring and Sign-Off sections. The Scoring section defines evaluation criteria (PASS/WARN/FAIL). The Sign-Off section captures role, name, date, and verdict. Missing sections produce per-section violations. Projects without the file produce Skip. |

The engine shall validate production readiness documents for ISO/IEC 25040:2024 evaluation process compliance: evaluation specification (scoring criteria) and evaluation conclusion (sign-off with roles and verdicts).

#### FR-815: Phase artifact file existence checks

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Checks 54-68, 99-128 -> `rules.toml` (declarative `file_exists` and `builtin`) |
| **Acceptance** | The tool shall verify that ISO-mandated documentation artifacts exist for each SDLC phase. Each missing artifact produces a finding at the configured severity. All checks are declarative `file_exists` rules — no Rust code required. |

The engine shall verify the existence of documentation artifacts mandated by ISO/IEC/IEEE 12207:2017, 15289:2019, 29148:2018, 42010:2022, 29119-3:2021, and 26514:2022 across all 8 SDLC phases:

| Phase | Check IDs | Artifacts |
|-------|-----------|-----------|
| 0-ideation | 54, 118 | Phase index (README.md), Concept of Operations (ConOps) |
| 1-requirements | 55, 119-120 | SRS, stakeholder requirements specification (StRS), traceability matrix |
| 2-planning | 56, 109-113 | Implementation plan, project management plan, configuration management plan, risk management plan, verification plan, test plan |
| 3-design | 57, 107-108 | Architecture, design description, interface description |
| 4-development | 58, 103-106 | Setup guide, integration plan, user documentation, API documentation, build procedures |
| 5-testing | 59, 99-102 | Testing strategy, test plan, test design specification, test case specification, verification report |
| 6-deployment | 60-62, 68, 114-116 | Phase index, deployment guide, CI/CD pipeline, installation guide, transition plan, release notes, user manual |
| 7-operations | 63-67, 117 | Phase index, operations manual, troubleshooting guide, maintenance plan, configuration reference, disposal plan |
| Cross-phase | 121-124 | Progress/status reports, decision log, audit report, audit report IEEE 1028 content |
| 5-testing (content) | 125-128 | Test plan 29119-3 sections, test design 29119-3 sections, test cases 29119-3 sections, verification report 29119-3 sections |

Severity mapping: mandatory artifacts (per ISO) use `warning`; recommended artifacts use `info`. Checks 66, 68, 83, 88, 100, 101 are `warning` (matching their mandatory ISO status). Scope mapping: core artifacts use `medium`; supplementary artifacts use `large`.

#### FR-816: Cross-phase and gap-closure artifact checks

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Checks 118-123 -> `rules.toml` (declarative `file_exists`) |
| **Acceptance** | The tool shall verify existence of 6 additional ISO-mandated artifacts: ConOps (29148 Annex B), StRS (29148 Cl. 6.2), traceability matrix (29148 Cl. 5.2.6), progress reports (12207 Cl. 6.3.2), decision log (12207 Cl. 6.3.3), audit report (15289 Cl. 9.2), with IEEE 1028 content validation (check 124). |

#### FR-817: Audit report IEEE 1028 content validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 124 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 124 validates that `docs/2-planning/audit_report.md` contains Scope, Findings, and Recommendations sections per IEEE 1028 clause 4. Missing sections produce per-section violations. Projects without the file produce Skip. |

#### FR-818: Test plan ISO/IEC/IEEE 29119-3 content validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 125 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 125 validates that `docs/5-testing/test_plan.md` contains Objectives/scope, Schedule/milestones, and Environment/resources sections per 29119-3 clause 7. Missing sections produce per-section violations. Projects without the file produce Skip. |

#### FR-819: Test design ISO/IEC/IEEE 29119-3 content validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 126 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 126 validates that `docs/5-testing/test_design.md` contains Test conditions, Test coverage, and Traceability sections per 29119-3 clause 8. Missing sections produce per-section violations. Projects without the file produce Skip. |

#### FR-820: Test cases ISO/IEC/IEEE 29119-3 content validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 127 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 127 validates that `docs/5-testing/test_cases.md` contains Test case ID/title, Pre-conditions/steps, and Expected results sections per 29119-3 clause 9. Missing sections produce per-section violations. Projects without the file produce Skip. |

#### FR-821: Verification report ISO/IEC/IEEE 29119-3 content validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | Check 128 -> `core/builtins/requirements.rs` |
| **Acceptance** | Check 128 validates that `docs/5-testing/verification_report.md` contains Summary/results, Pass/fail status, and Defects/issues sections per 29119-3 clause 10. Missing sections produce per-section violations. Projects without the file produce Skip. |

### 4.14 SRS Scaffold

#### FR-822: Scaffold command

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Demonstration |
| **Traces to** | STK-11 -> `main.rs`, `core/scaffold/mod.rs` |
| **Acceptance** | `doc-engine scaffold <SRS_PATH> [--output DIR] [--force] [--phase PHASES]` parses the SRS, extracts domains and requirements, and generates per-domain SDLC spec files; `--phase` accepts comma-separated phase names to filter output; exit code 0 on success, 2 on error |

```
doc-engine scaffold <SRS_PATH> [--output DIR] [--force]
```

#### FR-823: SRS domain extraction

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `core/scaffold/parser.rs` |
| **Acceptance** | The parser extracts `### X.Y Title` domain sections and `#### FR-NNN: Title` / `#### NFR-NNN: Title` requirement blocks with their attribute tables (Priority, State, Verification, Traces to, Acceptance); domains with no requirements are excluded |

#### FR-824: Per-domain spec file generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `core/scaffold/mod.rs`, `core/scaffold/yaml_gen.rs`, `core/scaffold/markdown_gen.rs` |
| **Acceptance** | For each domain, generates 10 files: `.spec.yaml`, `.spec`, `.arch.yaml`, `.arch`, `.test.yaml`, `.test`, `.manual.exec`, `.auto.exec`, `.deploy.yaml`, `.deploy`; plus 2 BRD files (`brd.spec.yaml`, `brd.spec`); total = `domains × 10 + 2` |

Generated files per domain:

| File | Directory | Format |
|------|-----------|--------|
| `{slug}.spec.yaml` | `docs/1-requirements/{slug}/` | YAML |
| `{slug}.spec` | `docs/1-requirements/{slug}/` | Markdown |
| `{slug}.arch.yaml` | `docs/3-design/{slug}/` | YAML |
| `{slug}.arch` | `docs/3-design/{slug}/` | Markdown |
| `{slug}.test.yaml` | `docs/5-testing/{slug}/` | YAML |
| `{slug}.test` | `docs/5-testing/{slug}/` | Markdown |
| `{slug}.manual.exec` | `docs/5-testing/{slug}/` | Markdown |
| `{slug}.auto.exec` | `docs/5-testing/{slug}/` | Markdown |
| `{slug}.deploy.yaml` | `docs/6-deployment/{slug}/` | YAML |
| `{slug}.deploy` | `docs/6-deployment/{slug}/` | Markdown |

#### FR-825: Manual test execution plan

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `core/scaffold/markdown_gen.rs` |
| **Acceptance** | Each `.manual.exec` file contains a TLDR, a Test Cases table with TC, Test, Steps (`_TODO_`), and Expected (from acceptance criteria) columns, and an Execution Log table with TC, Tester, Date, Pass/Fail, Notes columns; all TCs are aligned row-for-row with `.test` and `.auto.exec` |

#### FR-826: Automated test execution plan

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `core/scaffold/markdown_gen.rs` |
| **Acceptance** | Each `.auto.exec` file contains a TLDR, and a Test Cases table with TC, Test, Verifies, CI Job, Build, Status, Last Run columns; all TCs are aligned row-for-row with `.test` and `.manual.exec` |

#### FR-827: Scaffold skip/force behavior

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `core/scaffold/mod.rs` |
| **Acceptance** | Without `--force`, existing files are skipped (not overwritten) and reported with `~` prefix; with `--force`, all files are overwritten and reported with `+` prefix |

#### FR-828: Scaffold output directory

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `main.rs` |
| **Acceptance** | `--output DIR` specifies the output root directory; parent directories are created automatically; defaults to the current directory if not specified |

#### FR-829: Scaffold phase filter

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `main.rs`, `core/scaffold/mod.rs`, `core/scaffold/types.rs` |
| **Acceptance** | `--phase` accepts a comma-separated list of SDLC phases (`requirements`, `design`, `testing`, `deployment`); only files for the specified phases are generated; BRD files are included only when `requirements` is selected; omitting `--phase` generates all phases; invalid phase names exit with code 2 and a descriptive error; phase names are case-insensitive |

#### FR-830: Scaffold status report (ISO 15289 clause 9)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-11 -> `main.rs`, `core/scaffold/types.rs` |
| **Acceptance** | `--report <path>` persists a JSON scaffold status report conforming to ISO/IEC/IEEE 15289:2019 clause 9; the report contains: standard, clause, tool, tool_version, timestamp (ISO 8601 UTC), srs_source (absolute path), phases, force, domain_count, requirement_count, created, skipped |

When `--report <path>` is provided, the scaffold command shall serialize a `ScaffoldStatusReport` as pretty-printed JSON. The report wraps `ScaffoldResult` fields (promoted to top level) with ISO 15289:2019 clause 9 identification metadata: standard identifier, clause reference, tool name, tool version (from `Cargo.toml`), ISO 8601 UTC timestamp, canonicalized SRS source path, phase filter, and force flag. Parent directories are created automatically. No new dependencies are introduced — the timestamp uses Howard Hinnant's `civil_from_days` algorithm on `std::time::SystemTime`.

---

## 5. Non-Functional Requirements

### 5.1 Architecture

#### NFR-100: SEA compliance

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-04 -> module structure |
| **Acceptance** | Code review confirms 5-layer SEA: SPI (no deps) <- API <- Core (private) <- SAF (re-exports) <- CLI |

The crate shall follow Single-Crate Modular SEA:

| Layer | Visibility | Contents |
|-------|-----------|----------|
| L4: SAF | `pub` | Re-exports for library consumers |
| L3: CLI | binary only | `main.rs` with clap |
| L2: API | `pub` | `ComplianceEngine` trait, config/report types |
| L1: SPI | `pub` | `FileScanner`, `CheckRunner`, `Reporter` traits, core types |
| L0: Core | `pub(crate)` | All implementations |

#### NFR-101: Dependency direction

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-04 -> module structure |
| **Acceptance** | No `use core::` in spi/ or api/; no `use api::` in spi/ |

No layer shall depend on a layer above it.

### 5.2 Performance

#### NFR-200: Synchronous execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-06 -> `Cargo.toml` |
| **Acceptance** | No `tokio`, `async-std`, or other async runtime in dependencies |

#### NFR-201: Single pass

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Analysis |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | Profiling shows exactly one `walkdir` traversal per scan invocation |

The scanner shall discover files in a single directory walk.

### 5.3 Portability

#### NFR-300: Cross-platform

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-01 |
| **Acceptance** | `cargo build` succeeds on Linux, macOS, and Windows |

### 5.4 Extensibility

#### NFR-400: Declarative rule extensibility

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-02 -> `rules.toml`, `core/declarative.rs` |
| **Acceptance** | Adding a `[[rules]]` entry with `type = "file_exists"` to `rules.toml` and running `--rules` enforces the new rule without recompilation |

New declarative rules shall be addable by editing `rules.toml` alone.

#### NFR-401: Builtin handler extensibility

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-02 -> `core/builtins/` |
| **Acceptance** | A new handler can be added in 3 steps: implement function, register in mod.rs, add TOML entry |

### 5.5 Reliability

#### NFR-500: Graceful error handling

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-05 |
| **Acceptance** | Scanning a non-existent path produces exit code 2 and a message, not a panic |

IO errors and missing files shall produce `Skip` results or clear error messages, not panics.

#### NFR-501: Invalid rules detection

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-02 |
| **Acceptance** | Malformed TOML produces exit code 2 with a parse error message identifying the line |

---

## 6. External Interface Requirements

### 6.1 File System Interface

| Direction | Data | Format |
|-----------|------|--------|
| Input | Project directory | Read-only file system access |
| Input | Optional `rules.toml` | TOML (see FR-102) |
| Output | Scan results | Text (FR-400) or JSON (FR-401) to stdout |
| Output | Exit code | 0, 1, or 2 (FR-402) |

### 6.2 Library Interface

| Direction | Data | Type |
|-----------|------|------|
| Input | Project root | `&Path` |
| Input | Configuration | `&ScanConfig` (optional) |
| Output | Report | `ScanReport` |

### 6.3 Rules File Interface

| Aspect | Detail |
|--------|--------|
| Format | TOML |
| Schema | `[[rules]]` array of tables (FR-102) |
| Default location | Embedded in binary via `include_str!` |
| Override | `--rules <path>` CLI flag |

---

## 7. Risk Analysis

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Rules TOML schema changes break existing custom rules files | High | Low | Version the schema; validate with clear error messages (NFR-501) |
| Checklist changes in template-engine are not reflected in rules.toml | Medium | Medium | Document the update process; consider future hash-based drift detection |
| Regex patterns in TOML are hard to debug | Medium | Medium | Provide `--dry-run` or `--list-rules` to inspect loaded rules |
| Large projects with many files cause slow scans | Low | Low | Single-pass scanner (NFR-201); checks operate on cached file list |
| Builtin handler logic diverges from checklist intent | Medium | Low | Each handler traces to specific check IDs; test against fixture projects |

---

## Appendix A: Traceability Matrix

### Stakeholder -> System

| STK | SYS |
|-----|-----|
| STK-01 | SYS-01, SYS-02, SYS-03, SYS-04, SYS-05 |
| STK-02 | SYS-01 |
| STK-03 | SYS-05 |
| STK-04 | SYS-03, SYS-04 |
| STK-05 | SYS-04, SYS-05, SYS-06 |
| STK-06 | SYS-02 |
| STK-07 | SYS-03 |
| STK-08 | SYS-06 |
| STK-09 | SYS-05 |
| STK-10 | SYS-03 |
| STK-11 | SYS-07 |

### Stakeholder -> Software

| STK | FR / NFR |
|-----|----------|
| STK-01 | FR-100, FR-300, FR-301, FR-500, FR-502 |
| STK-02 | FR-100, FR-101, FR-102, FR-103, NFR-400, NFR-401 |
| STK-03 | FR-401, FR-402, FR-501, FR-754 |
| STK-04 | FR-600, FR-601, FR-602, NFR-100 |
| STK-05 | FR-303, FR-304, FR-400, FR-703, FR-742, FR-755, NFR-500 |
| STK-06 | FR-201, NFR-200 |
| STK-07 | FR-302, FR-503 |
| STK-08 | FR-700-706, FR-710-716, FR-720-727, FR-730-735, FR-740-742, FR-750-755 |
| STK-09 | FR-403, FR-506, FR-831 |
| STK-10 | FR-505 |
| STK-11 | FR-822, FR-823, FR-824, FR-825, FR-826, FR-827, FR-828, FR-829, FR-830 |

### Software -> Architecture

| FR / NFR | Architecture Component |
|----------|----------------------|
| FR-100, FR-101 | `core/rules.rs` |
| FR-102, FR-103 | `api/types.rs` (RuleDef, RuleType), `core/declarative.rs` |
| FR-104, FR-105 | `core/builtins/mod.rs` |
| FR-200-202 | `core/scanner.rs` |
| FR-300-304 | `core/engine.rs`, `spi/types.rs` |
| FR-400-403, FR-831 | `core/reporter.rs`, `main.rs` |
| FR-500-506 | `main.rs` |
| FR-600-602 | `saf/mod.rs` |
| FR-700-706 | `core/spec/parser.rs`, `core/spec/discovery.rs`, `spi/spec_types.rs` |
| FR-710-716 | `core/spec/validate.rs`, `api/spec_types.rs` |
| FR-720-727 | `core/spec/cross_ref.rs`, `core/builtins/spec.rs` |
| FR-730-735 | `core/spec/generate.rs` |
| FR-740-742 | `core/builtins/spec.rs`, `rules.toml` |
| FR-750-755 | `main.rs`, `core/reporter.rs` |
| FR-808-821 | `core/builtins/requirements.rs`, `rules.toml` |
| FR-822-830 | `core/scaffold/mod.rs`, `core/scaffold/types.rs`, `core/scaffold/parser.rs`, `core/scaffold/yaml_gen.rs`, `core/scaffold/markdown_gen.rs`, `main.rs` |
| NFR-100-101 | Module structure (spi/, api/, core/, saf/) |
| NFR-400-401 | `rules.toml`, `core/declarative.rs`, `core/builtins/` |
| NFR-500-501 | `core/engine.rs`, `core/rules.rs` |

---

## Appendix B: ISO Standards Mapping

This appendix maps doc-engine's 128 compliance checks to their corresponding ISO/IEC standard clauses. It serves as the traceability matrix between the engine's automated checks and the international standards they implement or support.

### B.1 Standards Referenced

| Standard | Edition | Title | Checks |
|----------|---------|-------|--------|
| ISO/IEC/IEEE 12207 | 2017 | Software life cycle processes | 9-10, 51-53, 56, 62, 64, 66, 82-88, 92, 96, 103, 109-113, 117, 120-122 |
| ISO/IEC/IEEE 15289 | 2019 | Content of life-cycle information items | 1-8, 14-20, 26-39, 48-50, 55, 60-68, 69-76, 99, 102-128 |
| ISO/IEC/IEEE 26514 | 2022 | Design and development of information for users | 58, 65, 67, 94, 104, 116 |
| ISO/IEC/IEEE 29119-3 | 2021 | Software testing -- Part 3: Test documentation | 59, 91, 99-102, 113, 125-128 |
| ISO/IEC/IEEE 29148 | 2018 | Life cycle processes -- Requirements engineering | 55, 89, 118-120 |
| ISO/IEC/IEEE 42010 | 2022 | Architecture description | 57, 90, 107-108 |
| ISO/IEC 25010 | 2023 | Product quality model (SQuaRE) | 93, 97 |
| ISO/IEC 25040 | 2024 | Evaluation process (SQuaRE) | 98 |
| IEEE 1028 | 2008 | Standard for Software Reviews and Audits | 123, 124 |

### B.2 Detailed Mapping

#### ISO/IEC/IEEE 12207:2017 -- Software Life Cycle Processes

The foundational lifecycle standard. doc-engine enforces its process areas through structural, planning, and traceability checks.

| ISO Clause | Clause Title | Check(s) | Check Description |
|------------|-------------|----------|-------------------|
| 6.1 | Agreement processes | -- | Not in scope (contract/supply) |
| 6.2 | Organizational project-enabling processes | 83, 84, 88 | Risk register, estimation, quality plan |
| 6.3.1 | Project planning process | 56, 85, 86, 87, 109 | Implementation plan, schedule, resource plan, communication plan, project management plan |
| 6.3.2 | Project assessment and control | 92, 96, 121 | Production readiness document, CI/CD + dependency + release lifecycle sections, progress/status reports |
| 6.3.3 | Decision management | 122 | Decision log |
| 6.3.4 | Risk management | 111 | Risk management plan |
| 6.3.5 | Configuration management | 110 | Configuration management plan |
| 6.4.1 | Stakeholder needs and requirements definition | 55, 89 | SRS exists, SRS with 29148 attribute tables |
| 6.4.2 | System/software requirements analysis | 82 | Backlog traces to requirements |
| 6.4.3 | Architecture definition | 90, 120 | Architecture docs have 42010 sections, traceability matrix |
| 6.4.5 | Design definition | 52, 107, 108 | Design documents reference requirements, design description, interface description |
| 6.4.6 | Implementation | 69, 103 | Developer guide exists, integration plan |
| 6.4.7 | Integration | 53 | Planning documents reference architecture |
| 6.4.8 | Verification | 62, 91, 112 | CI/CD pipeline docs, testing strategy has 29119-3 sections, verification plan |
| 6.4.9 | Transition | 96, 114 | Release automation/package metadata sections, transition plan |
| 6.4.10 | Validation | 98 | Scoring and sign-off evaluation sections |
| 6.4.11 | Operation | 63-65, 67 | Operations phase index, operations manual, troubleshooting guide, configuration reference |
| 6.4.12 | Maintenance | 64, 66 | Operations manual, maintenance plan |
| 6.4.13 | Disposal | 117 | Disposal plan |
| 7.2.1 | Life cycle model management | 9, 10 | SDLC phase numbering and ordering |
| 7.2.2 | Infrastructure management | 96 | CI/CD pipeline, dependency health sections |
| 7.2.5 | Quality management | 88, 93 | Quality plan + production readiness quality sections |
| 7.2.6 | Configuration management | 51 | Phase dirs contain expected artifacts |
| 7.2.8 | Information management | 1-3, 48 | Docs structure, glossary, ADR index |

#### ISO/IEC/IEEE 15289:2019 -- Content of Life-Cycle Information Items

Defines the content requirements for documentation artifacts produced throughout the software lifecycle.

| ISO Clause | Information Item | Check(s) | Check Description |
|------------|-----------------|----------|-------------------|
| 5.1 | General content requirements | 33, 34 | Audience declaration in all docs |
| 5.2 | Identification and status | 14-20 | Standard file naming (README, LICENSE, etc.) |
| 5.3 | Introduction purpose | 35, 36 | TLDR sections for long/short documents |
| 6.2 | Concept of operations | 41, 74 | W3H structure (Who, What, Why, How) |
| 6.3 | System/software requirements specification | 89 | SRS with ISO 29148 attributes |
| 6.5 | Architecture description | 11, 90 | ADR directory + 42010 sections |
| 6.7 | Design description | 6, 7 | Compliance checklist + architecture reference |
| 6.10 | Test documentation | 25, 91, 125-128 | Test file placement + 29119-3 sections + content validation |
| 6.11 | User documentation | 94 | Developer guide with 26514 sections |
| 6.12 | Configuration management plan | 19 | .gitignore exists |
| 6.13 | Project management plan | 83-88 | Planning phase artifacts |
| 6.14 | Quality assurance plan | 88 | Quality plan exists |
| 7.1 | Glossary | 3, 37-39 | Glossary exists, format, alphabetized, acronyms |
| 7.2 | Index / table of contents | 2, 40, 42 | Hub document, root linkage, phase links |
| 7.3 | Change history | 16, 28 | CHANGELOG.md exists |
| 7.4 | Security considerations | 17, 29 | SECURITY.md exists |
| 7.5 | License information | 18, 30 | LICENSE file exists |
| Annex A | Information item outline examples | 72, 73 | Templates directory with template files |
| 9 | Progress/status reports | 121, FR-830 | Progress/status reports exist; scaffold status report conforms to clause 9 |
| 9 | Decision log | 122 | Decision log exists |
| 9.2 | Audit report | 123, 124, FR-403, FR-831 | Audit report exists (123), IEEE 1028 content validation (124); `--output` persists scan results as `documentation_audit_report_v{version}.json` — the standard information item for compliance audit findings; audit status report conforms to clause 9.2 |

#### ISO/IEC/IEEE 29148:2018 -- Requirements Engineering

Defines the processes and products related to engineering requirements.

| ISO Clause | Clause Title | Check(s) | Check Description |
|------------|-------------|----------|-------------------|
| 5.2.5 | Stakeholder requirements specification | 89 | SRS requirements have attribute tables |
| 5.2.6 | Requirement attributes | 89, 120 | Priority, State, Verification, Traces-to, Acceptance; traceability matrix |
| 5.2.8 | Requirements traceability | 52, 82 | Design traces requirements, backlog traces requirements |
| 6.1 | SRS overview | 89 | SRS document with structured requirements |
| 6.2 | Stakeholder requirements specification | 119 | StRS exists |
| 6.6 | Specific requirements format | 89 | FR-NNN pattern with attribute tables |
| 7 | System requirements specification | 119 | StRS (system/stakeholder level requirements) |
| Annex B | Concept of Operations (ConOps) | 118 | ConOps document exists |

**Validated attributes per requirement (check 89):**

| Attribute | ISO 29148 Reference | Required |
|-----------|-------------------|----------|
| Priority | Table 2 (5.2.6) | Yes |
| State | Table 2 (5.2.6) | Yes |
| Verification | 5.2.7 | Yes |
| Traces to | 5.2.8 | Yes |
| Acceptance | 6.6.3 | Yes |

#### ISO/IEC/IEEE 42010:2022 -- Architecture Description

Defines the practices for creating, interpreting, analyzing, and using architecture descriptions.

| ISO Clause | Clause Title | Check(s) | Check Description |
|------------|-------------|----------|-------------------|
| 5.2 | Architecture description identification | 11 | ADR directory exists |
| 5.3 | Stakeholders and concerns | 90 | Who section (stakeholders identification) |
| 5.4 | Architecture viewpoints | 90 | What + How sections (viewpoint specification) |
| 5.5 | Architecture rationale | 90 | Why section (design rationale and concerns) |
| 5.6 | Correspondence rules | 52 | Design documents reference requirements |
| 5.7 | Architecture decisions | 48-50 | ADR index, naming, completeness |

**Validated sections per architecture document (check 90):**

| Section | Maps to 42010 Clause |
|---------|---------------------|
| Who (stakeholders) | 5.3 Stakeholders and concerns |
| What (system description) | 5.4 Architecture viewpoints |
| Why (rationale) | 5.5 Architecture rationale |
| How (component design) | 5.4 Architecture viewpoints |

#### ISO/IEC/IEEE 29119-3:2021 -- Test Documentation

Defines test documentation throughout the testing process.

| ISO Clause | Clause Title | Check(s) | Check Description |
|------------|-------------|----------|-------------------|
| 7 | Test plan | 125 | Test plan has objectives/scope, schedule, environment sections |
| 8 | Test specification | 126 | Test design has conditions, coverage, traceability sections |
| 8.1 | Test strategy | 91 | Test Strategy section exists |
| 8.2 | Test plan | 91 | Test Categories section exists |
| 9.1 | Test specification | 91 | Coverage Targets section exists |
| 9.2 | Test cases | 127 | Test cases has ID/title, steps, expected results sections |
| 10.1 | Test completion | 128 | Verification report has summary, status, defects sections |

**Validated sections per testing strategy (check 91):**

| Section | Maps to 29119-3 Clause |
|---------|----------------------|
| Test Strategy | 8.1 Organizational test strategy |
| Test Categories | 8.2 Test plan (test approach) |
| Coverage Targets | 9.1 Test design specification |

#### ISO/IEC/IEEE 26514:2022 -- Information for Users

Defines the design and development of information for software users.

| ISO Clause | Clause Title | Check(s) | Check Description |
|------------|-------------|----------|-------------------|
| 6.2 | Task analysis | 94 | Build & Test section (task-oriented instructions) |
| 7.1 | Information structure | 94 | Project Structure section (navigational overview) |
| 8.1 | Writing guidelines | 94 | Adding New Features section (extensibility guide) |
| 9.2 | Quick start | 94 | Build & Test section (getting started) |

**Validated sections per developer guide (check 94):**

| Section | Maps to 26514 Clause |
|---------|---------------------|
| Build & Test | 6.2 Task analysis, 9.2 Quick start |
| Project Structure | 7.1 Information structure |
| Adding New Features | 8.1 Writing guidelines |

#### ISO/IEC 25010:2023 -- Product Quality Model (SQuaRE)

Defines the product quality model with quality characteristics and sub-characteristics.

| Quality Characteristic | Sub-characteristic | Check(s) | Check Description |
|-----------------------|-------------------|----------|-------------------|
| Functional suitability | Functional completeness | 8 | Every enforceable rule has a checkbox |
| Reliability | Maturity | 93 | Runtime Safety section |
| Reliability | Fault tolerance | 93 | Runtime Safety section |
| Security | Confidentiality | 93 | Security section |
| Security | Integrity | 97 | Static Analysis section |
| Maintainability | Modularity | 97 | Documentation Lint section |
| Maintainability | Analysability | 97 | API Documentation section |
| Maintainability | Testability | 93 | Test Coverage section |
| Portability | Adaptability | 93 | Backwards Compatibility section |
| Usability | Learnability | 97 | README & Onboarding section |
| Usability | Operability | 93 | Observability section |

**Check 93 sections (core quality):**

| Section | 25010 Quality Characteristic |
|---------|----------------------------|
| Security | Security |
| Test Coverage | Maintainability > Testability |
| Observability | Usability > Operability |
| Backwards Compatibility | Portability > Adaptability |
| Runtime Safety | Reliability > Maturity |
| Verdict | (Aggregate quality assessment) |

**Check 97 sections (supplementary quality):**

| Section | 25010 Quality Characteristic |
|---------|----------------------------|
| Static Analysis | Security > Integrity |
| API Documentation | Maintainability > Analysability |
| README & Onboarding | Usability > Learnability |
| Documentation Lint | Maintainability > Modularity |

#### ISO/IEC 25040:2024 -- Evaluation Process (SQuaRE)

Defines the process for evaluating software product quality.

| ISO Clause | Clause Title | Check(s) | Check Description |
|------------|-------------|----------|-------------------|
| 7.1 | Establish evaluation requirements | 98 | Scoring section (criteria definition) |
| 7.2 | Specify the evaluation | 98 | Scoring table (PASS/WARN/FAIL meanings) |
| 7.3 | Design the evaluation | 92 | Production readiness document structure |
| 7.4 | Execute the evaluation | 93, 96, 97 | Quality section assessments |
| 7.5 | Conclude the evaluation | 98 | Sign-Off section (role, name, date, verdict) |

**Check 98 sections:**

| Section | Maps to 25040 Clause |
|---------|---------------------|
| Scoring | 7.1-7.2 Evaluation criteria and specification |
| Sign-Off | 7.5 Evaluation conclusion |

### B.3 Coverage Summary

#### Standards with Direct Check Implementation

| Standard | Checks Directly Implementing | Coverage |
|----------|------------------------------|----------|
| ISO/IEC/IEEE 12207:2017 | 9-10, 51-53, 56, 62, 64, 66, 82-88, 92, 96, 103, 107, 109-114, 117, 120-122 | Lifecycle structure, planning, traceability, phase artifacts |
| ISO/IEC/IEEE 15289:2019 | 1-8, 14-20, 26-39, 48-50, 55, 60-68, 69-76, 99, 102-128 | Information item content, format, and phase artifacts |
| ISO/IEC/IEEE 29148:2018 | 55, 89, 118-120 | SRS existence, requirements attribute validation, ConOps, StRS, traceability matrix |
| ISO/IEC/IEEE 42010:2022 | 48-50, 57, 90, 107-108 | Architecture description, design artifacts |
| ISO/IEC/IEEE 29119-3:2021 | 59, 91, 99-102, 113, 125-128 | Test documentation, testing phase artifacts, content validation |
| ISO/IEC/IEEE 26514:2022 | 58, 65, 67, 94, 104, 116 | User information structure, development/operations artifacts |
| ISO/IEC 25010:2023 | 93, 97 | Product quality assessment |
| ISO/IEC 25040:2024 | 98 | Evaluation process |

#### Checks by ISO Standard

```
12207 (Lifecycle)     ████████████████████████████████████ 33 checks
15289 (Info Items)    █████████████████████████████████████████████████████████████████████████ 69 checks
29148 (Requirements)  ██████ 5 checks
42010 (Architecture)  ██████ 5 checks
29119-3 (Testing)     ████████████ 11 checks
26514 (User Info)     ███████ 6 checks
25010 (Quality)       ███ 2 checks
25040 (Evaluation)    ██ 1 check
1028 (Reviews/Audits) ███ 2 checks
```

#### Checks Not Mapped to ISO (Internal Policy)

The following checks enforce project-level conventions not directly traceable to an ISO clause:

| Check(s) | Category | Description |
|----------|----------|-------------|
| 12-13 | Structure | Directory naming conventions (guide/ not guides/, uxui/ not uiux/) |
| 21-24 | Naming | snake_lower_case enforcement, guide naming convention |
| 40, 43 | Navigation | Root README deep-link prevention |
| 44-47 | Cross-ref | Internal link resolution and plural consistency |
| 54 | Ideation | Phase index existence (organizational convention) |
| 70 | Root files | INTERNAL_USAGE.md (internal project policy) |
| 77-81 | Module | Module-level documentation conventions |
| 95 | Backlog | Backlog section structure (items, completed, blockers) |

These checks reflect engineering best practices and organizational conventions that complement but are not derived from ISO standards.

#### Standards Not Yet Covered

| Standard | Edition | Title | Gap Analysis |
|----------|---------|-------|-------------|
| ISO/IEC/IEEE 15288 | 2023 | System life cycle processes | Partially covered via 12207 (software-specific counterpart) |
| ISO/IEC 25023 | 2016 | Measurement of system and software product quality | No quantitative quality metrics (coverage %, defect rates) |
| ISO/IEC 25041 | 2024 | Evaluation guide for developers, acquirers and independent evaluators | Partial via check 98 (sign-off), no acquirer/evaluator roles |
| ISO/IEC/IEEE 24765 | 2017 | Systems and software engineering vocabulary | Glossary checks (37-39) align but don't validate against 24765 terms |
| ISO/IEC 33001-33099 | 2015-2020 | Process assessment (SPICE) | No process capability/maturity assessment |
| ISO/IEC 27001 | 2022 | Information security management | SECURITY.md exists (17, 29) but no ISMS controls mapping |
| ISO 9001 | 2015 | Quality management systems | Quality plan (88) partially aligns but no QMS process checks |

### B.4 Cross-Reference: Check ID to ISO Clause

| Check | ISO Standard | Clause(s) |
|-------|-------------|-----------|
| 1 | 15289, 12207 | 15289:7.2, 12207:7.2.8 |
| 2 | 15289 | 15289:7.2 |
| 3 | 15289 | 15289:7.1 |
| 4-5 | 15289 | 15289:5.2 |
| 6-7 | 15289 | 15289:6.7 |
| 8 | 25010 | 25010:Functional completeness |
| 9-10 | 12207 | 12207:7.2.1 |
| 11 | 42010 | 42010:5.2 |
| 14-20 | 15289 | 15289:5.2 |
| 25 | 15289 | 15289:6.10 |
| 26-30 | 15289 | 15289:7.3-7.5 |
| 31-32 | 15289 | 15289:5.1 (community engagement) |
| 33-34 | 15289 | 15289:5.1 |
| 35-36 | 15289 | 15289:5.3 |
| 37-39 | 15289 | 15289:7.1 |
| 40 | 15289 | 15289:7.2 |
| 41 | 15289 | 15289:6.2 |
| 42 | 15289 | 15289:7.2 |
| 48-50 | 42010 | 42010:5.7 |
| 51 | 12207 | 12207:7.2.6 |
| 52 | 12207, 29148 | 12207:6.4.5, 29148:5.2.8 |
| 53 | 12207 | 12207:6.4.7 |
| 54 | 15288 | 15288:6.4.1 (ideation phase index) |
| 55 | 29148, 15289 | 29148:6.4, 29148:9 |
| 56 | 12207 | 12207:6.3.1 |
| 57 | 42010 | 42010:5-6 |
| 58 | 26514 | 26514:8-9 |
| 59 | 29119-3 | 29119-3:6 |
| 60 | 15289 | 15289:10 (deployment phase index) |
| 61 | 26514, 15289 | 26514:8-9, 15289:10 |
| 62 | 12207 | 12207:6.4.8 |
| 63 | 15289 | 15289:10 (operations phase index) |
| 64 | 12207, 15289 | 12207:6.4.12, 15289:10 |
| 65 | 26514 | 26514:8-9 |
| 66 | 12207, 15289 | 12207:6.4.13, 15289:10 |
| 67 | 26514 | 26514:8-9 |
| 68 | 26514, 15289 | 26514:8, 15289:10 |
| 69 | 12207 | 12207:6.4.6 |
| 71 | 12207 | 12207:6.4.2 |
| 72-73 | 15289 | 15289:Annex A |
| 74 | 15289 | 15289:6.2 |
| 75-76 | 15289 | 15289:5.2 |
| 82 | 12207, 29148 | 12207:6.4.2, 29148:5.2.8 |
| 83-84 | 12207 | 12207:6.2 |
| 85-87 | 12207 | 12207:6.3.1 |
| 88 | 12207 | 12207:7.2.5 |
| 89 | 29148 | 29148:5.2.5, 5.2.6, 6.1, 6.6 |
| 90 | 42010 | 42010:5.3, 5.4, 5.5 |
| 91 | 29119-3 | 29119-3:8.1, 8.2, 9.1 |
| 92 | 12207 | 12207:6.3.2 |
| 93 | 25010 | 25010:Security, Reliability, Maintainability, Portability, Usability |
| 94 | 26514 | 26514:6.2, 7.1, 8.1, 9.2 |
| 95 | -- | Internal policy (backlog structure) |
| 96 | 12207 | 12207:6.3.2, 6.4.9, 7.2.2 |
| 97 | 25010 | 25010:Security, Maintainability, Usability |
| 98 | 25040 | 25040:7.1, 7.2, 7.5 |
| 99 | 29119-3, 15289 | 29119-3:7, 15289:10 |
| 100 | 29119-3 | 29119-3:8 |
| 101 | 29119-3 | 29119-3:8 |
| 102 | 15289, 12207 | 15289:10, 12207:6.4.9 |
| 103 | 15289, 12207 | 15289:10, 12207:6.4.8 |
| 104 | 26514 | 26514:5-9 |
| 105 | 26514, 15289 | 26514:8-9, 15289:10 |
| 106 | 15289 | 15289:10 |
| 107 | 15289, 12207 | 15289:10, 12207:6.4.5 |
| 108 | 15289 | 15289:10 |
| 109 | 15289, 12207 | 15289:10, 12207:6.3.1 |
| 110 | 15289, 12207 | 15289:10, 12207:6.3.5 |
| 111 | 15289, 12207 | 15289:10, 12207:6.3.4 |
| 112 | 15289, 12207 | 15289:10, 12207:6.4.9 |
| 113 | 29119-3 | 29119-3:7 |
| 114 | 15289, 12207 | 15289:10, 12207:6.4.10 |
| 115 | 26514, 15289 | 26514:8-9, 15289:10 |
| 116 | 26514 | 26514:5-9 |
| 117 | 15289, 12207 | 15289:10, 12207:6.4.14 |
| 118 | 29148, 15289 | 29148:Annex B, 15289:6.2 |
| 119 | 29148 | 29148:6.2, 29148:7 |
| 120 | 29148, 12207 | 29148:5.2.6, 12207:6.4.3 |
| 121 | 15289, 12207 | 15289:9, 12207:6.3.2 |
| 122 | 15289, 12207 | 15289:9, 12207:6.3.3 |
| 123 | 15289, 1028 | 15289:9.2, 1028:4 |
| 124 | 1028, 15289 | 1028:4, 15289:9.2 |
| 125 | 29119-3, 15289 | 29119-3:7, 15289:10 |
| 126 | 29119-3, 15289 | 29119-3:8, 15289:10 |
| 127 | 29119-3, 15289 | 29119-3:9.2, 15289:10 |
| 128 | 29119-3, 15289 | 29119-3:10.1, 15289:10 |
