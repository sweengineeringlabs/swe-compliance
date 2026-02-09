# Software Requirements Specification: doc-engine

**Audience**: Developers, architects, project stakeholders

## TLDR

This SRS defines requirements for doc-engine, a Rust CLI tool that audits project documentation against 53 compliance checks (50 structural + 3 traceability) from the template-engine framework. It covers stakeholder needs, functional requirements for rule evaluation and reporting, non-functional requirements for performance and extensibility, and traceability from stakeholder goals to implementation modules.

**Version**: 1.0
**Date**: 2026-02-07
**Standard**: ISO/IEC/IEEE 29148:2018

---

## 1. Introduction

### 1.1 Purpose

This SRS defines the stakeholder, system, and software requirements for **doc-engine**, a Rust CLI tool and library that audits project documentation against the compliance standard defined by the template-engine documentation framework. The engine supports the original 50 structural checks, 3 traceability checks (51-53), and an extended set of YAML spec validation checks (54-68).

### 1.2 Scope

doc-engine is a single-crate Rust project within the `swe-compliance` workspace. It:

- Scans any project directory for documentation compliance
- Sources rules from a TOML configuration file (declarative + builtin handlers)
- Reports results as text or JSON
- Is usable as both a CLI binary and a Rust library
- Validates spec files in two formats: YAML (`.spec.yaml`, `.arch.yaml`, `.test.yaml`, `.deploy.yaml`) and markdown (`.spec`, `.arch`, `.test`, `.deploy`) for structure, cross-references, and SDLC coverage
- Generates markdown documentation from YAML spec files

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

### 1.4 References

| Document | Location |
|----------|----------|
| ISO/IEC/IEEE 29148:2018 | Requirements engineering standard (this document conforms to) |
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
| Architect | Audits projects, defines standards | Customizable rules, comprehensive coverage of 68 checks (53 base + 15 spec) |
| Documentation maintainer | Tweaks rules without coding | Declarative TOML rules, no recompilation for simple changes |
| CI system | Automated gate in pipeline | JSON output, deterministic exit codes, non-interactive |
| Library consumer | Integrates scanning programmatically | Clean public API, well-typed report structures |

### 2.2 Operational Scenarios

#### OS-1: Developer local scan

A developer runs `doc-engine scan .` from their project root. The tool discovers all files, runs all 68 checks (53 base + 15 spec), and prints a text report showing which checks passed and which failed with file paths and messages. The developer fixes violations and re-runs until clean.

#### OS-2: CI pipeline gate

A CI job runs `doc-engine scan . --json`. The tool outputs a JSON report. The CI job parses the exit code: 0 passes the gate, 1 fails the build with violation details, 2 indicates a configuration error.

#### OS-3: Custom rules for internal project

An architect copies the default `rules.toml`, removes open-source-specific checks, adds an internal-only check for `INTERNAL_USAGE.md`, and runs `doc-engine scan . --rules internal.toml --type internal`.

#### OS-4: Documentation maintainer adds a check

A documentation maintainer needs to require a new file `docs/CODEOWNERS`. They add a `[[rules]]` entry with `type = "file_exists"` and `path = "docs/CODEOWNERS"` to `rules.toml`. No Rust code changes or recompilation needed.

#### OS-5: Library integration

Another Rust crate calls `doc_engine::scan(path)` programmatically and inspects the returned `ScanReport` to generate a compliance dashboard.

#### OS-6: YAML spec validation

An architect runs `doc-engine spec validate docs/` to verify all `.spec.yaml`, `.arch.yaml`, `.test.yaml`, and `.deploy.yaml` files parse correctly and conform to their schemas. The tool reports schema violations per file.

#### OS-7: Cross-reference analysis

An architect runs `doc-engine spec cross-ref docs/` to verify that all dependency references resolve, every feature request has matching test and architecture specs, the BRD inventory counts match actual files, and test cases trace to valid requirement IDs.

#### OS-8: Markdown generation from YAML

A developer runs `doc-engine spec generate docs/1-requirements/auth/login.spec.yaml --output generated/` to produce a markdown document from a YAML spec file, matching the template-engine template format.

### 2.3 Stakeholder Requirements

| ID | Requirement | Source | Priority | Rationale |
|----|-------------|--------|----------|-----------|
| STK-01 | The tool shall audit any project directory against 53 documentation compliance checks | Compliance Checklist | Must | Replaces manual bash-based auditing |
| STK-02 | Simple rules shall be modifiable without recompiling | Architect feedback | Must | Non-developers need to customize rules |
| STK-03 | The tool shall produce machine-readable output for CI integration | CI pipeline needs | Must | Enables automated compliance gating |
| STK-04 | The tool shall be usable as a Rust library | Library consumer needs | Should | Enables programmatic integration |
| STK-05 | The tool shall report clear, actionable violation messages with file paths | Developer feedback | Must | Developers need to locate and fix issues quickly |
| STK-06 | The tool shall run without network access | Security constraint | Must | Scans local file system only |
| STK-07 | The tool shall support both open-source and internal project types | Architect needs | Must | Different projects have different required files |
| STK-08 | The tool shall validate YAML spec files for schema conformance, cross-references, and generate markdown from them | Architect feedback | Should | Structured specs enable automated traceability and doc generation |

---

## 3. System Requirements (SyRS)

### 3.1 System Context

```
template-engine/templates/
├── framework.md              ← defines the standard
└── compliance-checklist.md   ← defines the 53 checks
         │
         ▼
doc-engine/rules.toml         ← encodes checks as TOML rules
         │
         ▼
doc-engine scan <project>     ← audits any project against them
         │
         ▼
stdout (text or JSON)         ← results + exit code
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
| **Acceptance** | When no `--rules` flag is provided, the engine loads rules from the embedded default and produces a valid `ScanReport` with 68 check results |

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
| `id` | u8 | Yes | Unique check number (1-68) |
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

#### FR-300: All checks (53 base + 15 spec)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01, STK-08 -> `core/engine.rs`, `core/builtins/spec.rs`, `core/builtins/traceability.rs`, `rules.toml` |
| **Acceptance** | Default `rules.toml` contains 68 rules; a full scan produces 53 base results + up to 15 spec results (spec checks produce Skip if no `.spec.yaml` files exist) |

The engine shall support 68 checks:

| Category | Check IDs | Count |
|----------|-----------|-------|
| structure | 1-13 | 13 |
| naming | 14-25 | 12 |
| root_files | 26-32 | 7 |
| content | 33-39 | 7 |
| navigation | 40-43 | 4 |
| cross_ref | 44-47 | 4 |
| adr | 48-50 | 3 |
| traceability | 51-53 | 3 |
| spec | 54-68 | 15 |
| **Total** | | **68** |

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
| 2 | Scan error (invalid path, IO error, rules parse error, unknown handler) |

### 4.5 CLI Interface

#### FR-500: Scan command

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-01 -> `main.rs` |
| **Acceptance** | `doc-engine scan <PATH>` executes a full scan and prints results |

```
doc-engine scan <PATH>
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

### 4.6 Library API

#### FR-600: Public scan function

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | `doc_engine::scan(path)` compiles and returns a `Result<ScanReport, ScanError>` |

The library shall expose `scan(root: &Path) -> Result<ScanReport, ScanError>` via the SAF layer.

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

Public via SAF: `ScanConfig`, `ScanReport`, `ScanSummary`, `ProjectType`, `CheckId`, `CheckResult`, `Severity`, `Violation`, `RuleDef`, `RuleType`, `SpecFormat`, `SpecKind`, `SpecStatus`, `Priority`, `DiscoveredSpec`, `BrdSpec`, `FeatureRequestSpec`, `ArchSpec`, `TestSpec`, `DeploySpec`, `MarkdownSpec`, `MarkdownTestCase`, `SpecEnvelope`, `ParsedSpec`, `SpecValidationReport`, `CrossRefReport`, `SpecDiagnostic`, `CrossRefResult`.

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
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/builtins/spec.rs` |
| **Acceptance** | A project with no spec files (neither `.spec`/`.arch`/`.test`/`.deploy` nor `.spec.yaml`/`.arch.yaml`/`.test.yaml`/`.deploy.yaml`) produces `Skip` for checks 54-68, not `Fail` |

Spec checks (54-68) shall be opt-in: if no spec files of either format exist in the project, all spec checks shall produce `Skip` results.

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
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-08 -> `core/builtins/spec.rs`, `rules.toml` |
| **Acceptance** | Checks 54-68 appear in `rules.toml` and execute during a normal `doc-engine scan`; results appear in both text and JSON output |

Checks 54-68 shall be integrated into the standard scan pipeline as builtin handlers in `rules.toml`.

#### FR-741: Spec check category

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-08 -> `rules.toml` |
| **Acceptance** | All checks 54-68 have `category = "spec"` in `rules.toml` |

All spec checks shall use the `spec` category for grouping in reports.

#### FR-742: Spec check descriptions

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-05, STK-08 -> `rules.toml` |
| **Acceptance** | Each check 54-68 has a unique, descriptive `description` field in `rules.toml` |

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

These requirements specify the behavioral constraints for checks identified in the [backlog](../2-planning/backlog.md). They apply to checks 69+ (after the reserved spec range 54-68).

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

### Software -> Architecture

| FR / NFR | Architecture Component |
|----------|----------------------|
| FR-100, FR-101 | `core/rules.rs` |
| FR-102, FR-103 | `api/types.rs` (RuleDef, RuleType), `core/declarative.rs` |
| FR-104, FR-105 | `core/builtins/mod.rs` |
| FR-200-202 | `core/scanner.rs` |
| FR-300-304 | `core/engine.rs`, `spi/types.rs` |
| FR-400-402 | `core/reporter.rs`, `main.rs` |
| FR-500-504 | `main.rs` |
| FR-600-602 | `saf/mod.rs` |
| FR-700-706 | `core/spec/parser.rs`, `core/spec/discovery.rs`, `spi/spec_types.rs` |
| FR-710-716 | `core/spec/validate.rs`, `api/spec_types.rs` |
| FR-720-727 | `core/spec/cross_ref.rs`, `core/builtins/spec.rs` |
| FR-730-735 | `core/spec/generate.rs` |
| FR-740-742 | `core/builtins/spec.rs`, `rules.toml` |
| FR-750-755 | `main.rs`, `core/reporter.rs` |
| NFR-100-101 | Module structure (spi/, api/, core/, saf/) |
| NFR-400-401 | `rules.toml`, `core/declarative.rs`, `core/builtins/` |
| NFR-500-501 | `core/engine.rs`, `core/rules.rs` |
