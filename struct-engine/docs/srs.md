# Software Requirements Specification: struct-engine

**Audience**: Developers, architects, project stakeholders

## TLDR

This SRS defines requirements for struct-engine, a Rust CLI tool and library that audits Rust project structure against configurable compliance rules derived from the [rustboot project structure](../../../langboot/rustboot/docs/3-design/project_structure.md) and [testing strategy](../../../langboot/rustboot/doc/5_testing/testing_strategy.md) conventions. It enforces the `{main,tests}` layout, SEA layering, Cargo target paths, naming conventions, test organization, documentation, hygiene, umbrella workspace validation, and optional directory placement. The engine currently supports 44 checks across 7 categories with a target of ~59 checks across 10 categories. It covers stakeholder needs, functional requirements for rule evaluation and reporting, non-functional requirements for performance and extensibility, and traceability from stakeholder goals to implementation modules.

**Version**: 1.0
**Date**: 2026-02-10
**Standard**: ISO/IEC/IEEE 29148:2018

---

## 1. Introduction

### 1.1 Purpose

This SRS defines the stakeholder, system, and software requirements for **struct-engine**, a Rust CLI tool and library that audits Rust project structure against compliance rules. The engine validates directory layout (`{main,tests}`), SEA layering (`api/`, `core/`, `saf/`), Cargo.toml metadata and target paths, naming conventions, test file organization, documentation presence, and project hygiene. It currently supports 44 checks across 7 categories (structure 1-8, cargo_metadata 9-18, cargo_targets 19-26, naming 27-32, test_org 33-38, documentation 39-42, hygiene 43-44), with a target of ~59 checks across 10 categories after adding SEA layer validation, umbrella validation, and optional directory checks.

### 1.2 Scope

struct-engine is a single-crate Rust project within the `swe-compliance` workspace. It:

- Scans any Rust project directory for structural compliance
- Sources rules from a TOML configuration file (declarative + builtin handlers)
- Reports results as text or JSON
- Is usable as both a CLI binary and a Rust library
- Auto-detects project kind (library, binary, both, workspace) from `Cargo.toml`

struct-engine does **not**:

- Enforce documentation content conventions (that is doc-engine's domain)
- Enforce code-level style or lint rules (that is clippy/rustfmt's domain)
- Require network access (local file system only)

### 1.3 Definitions and Acronyms

| Term | Definition |
|------|-----------|
| **SEA** | Stratified Encapsulation Architecture — layered module pattern (API/Core/SAF) |
| **SAF** | Surface API Facade — public re-export layer for library consumers |
| **{main,tests}** | Layout convention where source lives in `main/src/` and tests live in `tests/` |
| **Declarative rule** | A check defined entirely in TOML, executed by the generic DeclarativeCheck runner |
| **Builtin rule** | A check referencing a named Rust handler for complex logic |
| **Project kind** | Auto-detected type: Library, Binary, Both, or Workspace |
| **Umbrella** | A virtual workspace (`[workspace]` only, no `[package]`) grouping 2+ sub-crates |
| **Compliance check** | A single rule that produces Pass, Fail (with violations), or Skip |

### 1.4 References

| Document | Location |
|----------|----------|
| Rustboot Project Structure | `langboot/rustboot/docs/3-design/project_structure.md` |
| Rustboot Testing Strategy | `langboot/rustboot/doc/5_testing/testing_strategy.md` |
| SEA Architecture Reference | `langboot/rustratify/doc/3-design/architecture.md` |
| struct-engine Backlog | `struct-engine/docs/backlog.md` |
| struct-engine Architecture | `struct-engine/docs/architecture.md` (planned) |
| doc-engine SRS (format reference) | `doc-engine/docs/1-requirements/srs.md` |

---

## 2. Stakeholder Requirements (StRS)

### 2.1 Stakeholders

| Stakeholder | Role | Needs |
|-------------|------|-------|
| Developer | Runs scans during local development | Fast feedback on structure compliance, clear violation messages |
| Architect | Audits projects, defines structure standards | Customizable rules, comprehensive coverage of layout/naming/SEA conventions |
| CI system | Automated gate in pipeline | JSON output, deterministic exit codes, non-interactive |
| Library consumer | Integrates scanning programmatically | Clean public API, well-typed report structures |
| Documentation maintainer | Tweaks rules without coding | Declarative TOML rules, no recompilation for simple changes |

### 2.2 Operational Scenarios

#### OS-1: Developer local scan

A developer runs `struct-engine scan .` from their project root. The tool discovers all files, auto-detects the project kind, runs all 44 checks, and prints a text report showing which checks passed, failed, or were skipped. The developer fixes violations and re-runs until clean.

#### OS-2: CI pipeline gate

A CI job runs `struct-engine scan . --json`. The tool outputs a JSON report. The CI job parses the exit code: 0 passes the gate, 1 fails the build with violation details, 2 indicates a configuration error.

#### OS-3: Custom rules for internal project

An architect copies the default `rules.toml`, removes library-specific checks, adds internal naming requirements, and runs `struct-engine scan . --rules internal.toml`.

#### OS-4: Documentation maintainer adds a check

A documentation maintainer needs to require a `config/` directory. They add a `[[rules]]` entry with `type = "dir_exists"` and `path = "config"` to `rules.toml`. No Rust code changes or recompilation needed.

#### OS-5: Library integration

Another Rust crate calls `struct_engine::scan_with_config(path, &config)` programmatically and inspects the returned `ScanReport` to generate a compliance dashboard.

#### OS-6: Selective check execution

A developer runs `struct-engine scan . --checks 1-8` to validate only the structure category, or `--checks 9-18` for cargo metadata only.

#### OS-7: Project kind override

A developer with a binary crate that also has library components runs `struct-engine scan . --kind both` to ensure both library and binary checks are applied.

### 2.3 Stakeholder Requirements

| ID | Requirement | Source | Priority | Rationale |
|----|-------------|--------|----------|-----------|
| STK-01 | The tool shall audit any Rust project directory against structural compliance checks | Project structure spec | Must | Replaces manual structure review |
| STK-02 | Simple rules shall be modifiable without recompiling | Architect feedback | Must | Non-developers need to customize rules |
| STK-03 | The tool shall produce machine-readable output for CI integration | CI pipeline needs | Must | Enables automated compliance gating |
| STK-04 | The tool shall be usable as a Rust library | Library consumer needs | Should | Enables programmatic integration |
| STK-05 | The tool shall report clear, actionable violation messages with file paths | Developer feedback | Must | Developers need to locate and fix issues quickly |
| STK-06 | The tool shall run without network access | Security constraint | Must | Scans local file system only |
| STK-07 | The tool shall auto-detect project kind and scope checks accordingly | Developer feedback | Must | Library-only checks should not apply to binaries |

---

## 3. System Requirements (SyRS)

### 3.1 System Context

```
rustboot/docs/3-design/project_structure.md    ← defines structure conventions
rustboot/doc/5_testing/testing_strategy.md     ← defines test conventions
         │
         ▼
struct-engine/config/rules.toml                ← encodes checks as TOML rules
         │
         ▼
struct-engine scan <project>                   ← audits any Rust project
         │
         ├── stdout (text or JSON)             ← results + exit code
         └── exit code (0, 1, 2)
```

### 3.2 System Functions

| ID | Function | Description |
|----|----------|-------------|
| SYS-01 | Rule loading | Parse TOML rules file (embedded default or external override) |
| SYS-02 | File discovery | Recursively scan project directory for all files |
| SYS-03 | Project kind detection | Auto-detect library/binary/both/workspace from Cargo.toml |
| SYS-04 | Check execution | Run each rule against the project (declarative or builtin) |
| SYS-05 | Result aggregation | Collect pass/fail/skip per check, compute summary |
| SYS-06 | Reporting | Output results as human-readable text or machine-readable JSON |

### 3.3 System Constraints

- **Language**: Rust (2021 edition)
- **Architecture**: Single-Crate Modular SEA (API/Core/SAF)
- **No async**: Synchronous file system operations only
- **No network**: Local file system scanning only
- **Platform**: Linux, macOS, Windows (via standard Rust cross-compilation)

### 3.4 Assumptions and Dependencies

- Projects being scanned have a `Cargo.toml` at the root
- External crate dependencies: `walkdir`, `regex`, `toml`, `clap`, `serde`, `serde_json`
- Dev dependencies: `tempfile`, `assert_cmd`, `predicates`

---

## 4. Software Requirements (SRS)

### Requirement Attributes

Each requirement includes:

| Attribute | Description |
|-----------|-------------|
| **ID** | Unique identifier (FR-nnn for functional, NFR-nnn for non-functional) |
| **Priority** | Must / Should / Could / Won't (MoSCoW) |
| **State** | Proposed / Approved / Implemented / Verified |
| **Verification** | Test / Inspection / Analysis / Demonstration |
| **Traces to** | Stakeholder requirement (STK-nn), architecture component |
| **Acceptance criteria** | Condition(s) that prove the requirement is met |

### 4.1 Rule Loading

#### FR-100: Default rules embedded in binary

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01, STK-02 -> `core/rules.rs` |
| **Acceptance** | When no `--rules` flag is provided, the engine loads rules from the embedded default and produces a valid `ScanReport` with 44 check results |

The binary shall embed a default `rules.toml` via `include_str!`. When no `--rules` flag is provided, the embedded rules are used.

#### FR-101: External rules file override

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-02 -> `core/rules.rs` |
| **Acceptance** | When `--rules custom.toml` is provided, only rules in `custom.toml` are executed; embedded defaults are ignored |

When `--rules <path>` is provided, the engine shall load and parse the external TOML file instead of the embedded default.

#### FR-102: TOML rules schema

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-02 -> `api/types.rs` (RuleDef, RuleType) |
| **Acceptance** | The TOML parser accepts all fields below without error; missing required fields produce exit code 2 |

Each rule entry shall contain:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | u8 | Yes | Unique check number |
| `category` | string | Yes | Grouping category |
| `description` | string | Yes | Human-readable description |
| `severity` | string | Yes | `"error"`, `"warning"`, or `"info"` |
| `type` | string | Yes | Rule type (see FR-103, FR-104) |
| `path` | string | Conditional | Required for file/dir rule types |
| `pattern` | string | Conditional | Required for content/naming rule types |
| `glob` | string | Conditional | Required for glob rule types |
| `handler` | string | Conditional | Required for builtin rule types |
| `message` | string | No | Custom failure message |
| `key` | string | Conditional | Required for cargo_key rule types |
| `project_kind` | string | No | `"library"`, `"binary"`, `"workspace"` — if set, rule only runs for that kind |
| `exclude_paths` | string[] | No | Path prefixes to exclude from glob matching |
| `exclude_pattern` | string | No | Regex pattern for lines to exclude |

#### FR-103: Declarative rule types

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-02 -> `core/declarative.rs` |
| **Acceptance** | Each of the 11 rule types produces correct Pass/Fail results when tested against a fixture project |

The engine shall support 11 declarative rule types:

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
| `cargo_key_exists` | Pass if TOML key at dotted `key` path exists in Cargo.toml |
| `cargo_key_matches` | Pass if TOML key value at `key` matches regex `pattern` |

#### FR-104: Builtin rule types

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/builtins/` |
| **Acceptance** | Each handler produces correct results when tested against compliant and non-compliant fixture projects |

When `type = "builtin"`, the engine shall look up a Rust handler by `handler` name. Supported handlers:

| Handler | Module | Description |
|---------|--------|-------------|
| `crate_root_exists` | `cargo_toml` | src/lib.rs or src/main.rs exists |
| `rustboot_crate_root_exists` | `cargo_toml` | main/src/lib.rs or main/src/main.rs exists |
| `benches_dir_if_declared` | `cargo_toml` | benches/ exists if benchmarks declared |
| `license_field_exists` | `cargo_toml` | package.license or package.license-file exists |
| `lib_path_correct` | `source_layout` | [lib] path matches existing file |
| `bin_path_correct` | `source_layout` | [[bin]] path matches existing file |
| `test_targets_declared` | `source_layout` | [[test]] targets declared for tests/ files |
| `bench_harness_false` | `source_layout` | [[bench]] targets have harness = false |
| `no_undeclared_tests` | `source_layout` | No undeclared test files |
| `no_undeclared_benches` | `source_layout` | No undeclared bench files |
| `example_targets_if_dir` | `source_layout` | [[example]] targets exist if examples/ present |
| `test_paths_resolve` | `source_layout` | [[test]] paths resolve to existing files |
| `test_file_suffixes` | `test_org` | Test files use correct suffixes |
| `test_fn_prefixes` | `test_org` | Test functions use category prefixes |
| `test_fn_suffixes` | `test_org` | Test functions use scenario suffixes |
| `int_tests_location` | `test_org` | Integration tests in tests/ |
| `unit_tests_colocated` | `test_org` | Unit tests colocated in source files |
| `no_test_in_src` | `test_org` | No non-#[cfg(test)] test code in src/ |
| `module_names_match` | `naming` | Module file names match mod declarations |
| `bin_names_valid` | `naming` | Binary names use hyphens or underscores |
| `doc_dir_exists` | `documentation` | doc/ or docs/ directory exists (if library) |
| `examples_dir_lib` | `documentation` | examples/ directory exists (if library) |

#### FR-105: Unknown handler error

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-05 -> `core/builtins/mod.rs` |
| **Acceptance** | A rules file with `handler = "nonexistent"` produces exit code 2 and a message naming the unknown handler |

If a `builtin` rule references an unknown handler name, the engine shall return a scan error (exit code 2) with a descriptive message.

### 4.2 File Discovery

#### FR-200: Recursive scanning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | Given a project with nested directories 5 levels deep, all files are discovered |

The scanner shall recursively discover all files under the provided project root directory.

#### FR-201: Directory exclusions

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | Files inside `.git/`, `target/`, and `node_modules/` are not included in the file list |

The scanner shall skip: hidden directories (names starting with `.`), `target/`, `node_modules/`.

#### FR-202: Relative paths

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | All paths in the file list are relative (no leading `/` or absolute prefix) |

All discovered file paths shall be relative to the project root.

### 4.3 Project Kind Detection

#### FR-250: Auto-detection from Cargo.toml

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-07, SYS-03 -> `core/engine.rs` |
| **Acceptance** | A Cargo.toml with `[workspace]` detects as Workspace; with `[lib]` as Library; with `[[bin]]` as Binary; with both as Both |

The engine shall auto-detect project kind from Cargo.toml:

| Kind | Detection |
|------|-----------|
| Workspace | Has `[workspace]` section |
| Library | Has `[lib]` or `src/lib.rs` or `main/src/lib.rs` |
| Binary | Has `[[bin]]` or `src/main.rs` or `main/src/main.rs` |
| Both | Has both library and binary indicators |

#### FR-251: Kind override

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-07 -> `core/engine.rs` |
| **Acceptance** | `--kind library` forces Library kind regardless of detected kind |

When `--kind` is provided, the engine shall use the specified kind instead of auto-detection.

#### FR-252: Kind-based check filtering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-07 -> `core/engine.rs` |
| **Acceptance** | Rules with `project_kind = "library"` are skipped when scanning a binary-only project |

When a rule has `project_kind` set, it shall only run if the scan's detected or overridden project kind matches.

### 4.4 Check Execution

#### FR-300: All checks

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/engine.rs`, `config/rules.toml` |
| **Acceptance** | Default `rules.toml` contains 44 rules; a full scan produces 44 check results |

The engine shall support 44 checks across 7 categories:

| Category | Check IDs | Count |
|----------|-----------|-------|
| structure | 1-8 | 8 |
| cargo_metadata | 9-18 | 10 |
| cargo_targets | 19-26 | 8 |
| naming | 27-32 | 6 |
| test_org | 33-38 | 6 |
| documentation | 39-42 | 4 |
| hygiene | 43-44 | 2 |
| **Total** | | **44** |

#### FR-301: Check filtering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/engine.rs` |
| **Acceptance** | `--checks 1-8` produces exactly 8 results; `--checks 1,2,3` produces exactly 3 |

When `--checks` is provided with a range or comma-separated list, only matching checks shall be executed.

#### FR-302: Check result types

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-05 -> `api/types.rs` |
| **Acceptance** | The `CheckResult` enum has exactly three variants: Pass, Fail (with violations), Skip (with reason) |

Each check shall produce one of: **Pass**, **Fail** (with `Violation` records), or **Skip** (with reason string).

#### FR-303: Violation record

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-05 -> `api/types.rs` |
| **Acceptance** | Each `Violation` contains check ID, optional file path, message, and severity |

Each violation shall contain: Check ID (u8), file path (optional), message (string), severity (error/warning/info).

### 4.5 Reporting

#### FR-400: Text output (default)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Demonstration |
| **Traces to** | STK-05 -> `core/reporter.rs` |
| **Acceptance** | Running without `--json` prints grouped results with check IDs, descriptions, statuses, violations, and a summary line |

Default output shall be human-readable text grouped by category, showing check ID, description, pass/fail/skip, violations with file paths, and a summary line.

#### FR-401: JSON output

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-03 -> `core/reporter.rs` |
| **Acceptance** | `--json` output parses as valid JSON and deserializes to `ScanReport` |

When `--json` is provided, output shall be a JSON `ScanReport`.

#### FR-402: Exit codes

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-03 -> `main.rs` |
| **Acceptance** | Clean project returns 0; project with violations returns 1; invalid path returns 2 |

| Code | Meaning |
|------|---------|
| 0 | All executed checks passed |
| 1 | One or more checks failed |
| 2 | Scan error (invalid path, IO error, rules parse error, unknown handler) |

### 4.6 CLI Interface

#### FR-500: Scan command

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Demonstration |
| **Traces to** | STK-01 -> `main.rs` |
| **Acceptance** | `struct-engine scan <PATH>` executes a scan and prints results |

```
struct-engine scan <PATH> [--json] [--checks SPEC] [--kind KIND] [--rules FILE]
```

#### FR-501: JSON flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-03 -> `main.rs` |
| **Acceptance** | `struct-engine scan <PATH> --json` outputs valid JSON |

#### FR-502: Check filter flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `main.rs` |
| **Acceptance** | `--checks 1-8` runs exactly 8 checks; `--checks 1,2,3,9-18` runs exactly 13 |

Supports ranges and comma-separated values.

#### FR-503: Project kind flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-07 -> `main.rs` |
| **Acceptance** | `--kind library` overrides auto-detection; supported values: library, binary, both, workspace |

#### FR-504: Rules file flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-02 -> `main.rs` |
| **Acceptance** | `--rules custom.toml` loads and uses the specified file; missing file produces exit code 2 |

### 4.7 Library API

#### FR-600: Public scan function

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | `struct_engine::scan(path)` scans with default config and returns `Result<ScanReport, ScanError>` |

The library shall expose `scan(root: &Path) -> Result<ScanReport, ScanError>` via the SAF layer.

#### FR-601: Configurable scan function

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | `struct_engine::scan_with_config(path, &config)` respects `ScanConfig` fields and returns `Result<ScanReport, ScanError>` |

The library shall expose `scan_with_config(root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError>` via the SAF layer.

#### FR-602: Public types

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | All listed types are importable from `struct_engine::` |

Public via SAF: `ScanConfig`, `ScanReport`, `ScanSummary`, `ProjectKind`, `CheckId`, `CheckResult`, `Severity`, `Violation`, `CheckEntry`, `ScanError`, `RuleDef`, `RuleType`, `RuleSet`, `CargoManifest`.

#### FR-603: Report formatters

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | `format_report_text(&report)` returns human-readable text; `format_report_json(&report)` returns valid JSON |

The library shall expose `format_report_text` and `format_report_json` via the SAF layer.

#### FR-604: Project kind detection

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-04, STK-07 -> `saf/mod.rs` |
| **Acceptance** | `detect_project_kind(path)` returns the correct `ProjectKind` |

The library shall expose `detect_project_kind(root: &Path) -> ProjectKind` via the SAF layer.

#### FR-605: Default rule count

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-04 -> `saf/mod.rs` |
| **Acceptance** | `default_rule_count()` returns the current number of embedded rules (44) |

The library shall expose `default_rule_count() -> usize` via the SAF layer.

### 4.8 Structure Checks (Current)

#### FR-700: Cargo.toml existence (Check 1)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `config/rules.toml` (declarative `file_exists`) |
| **Acceptance** | Check 1 produces Pass when `Cargo.toml` exists; Fail when absent |

#### FR-701: Source directory existence (Check 2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `config/rules.toml` (declarative `dir_exists`) |
| **Acceptance** | Check 2 produces Pass when `src/` exists; Fail when absent |

#### FR-702: Crate root exists (Check 3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/builtins/cargo_toml.rs` |
| **Acceptance** | Check 3 produces Pass when `src/lib.rs` or `src/main.rs` exists |

#### FR-703: Rustboot main/src/ existence (Check 4)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `config/rules.toml` (declarative `dir_exists`, workspace only) |
| **Acceptance** | Check 4 produces Pass when `main/src/` exists; skipped for non-workspace projects |

#### FR-704: Rustboot crate root exists (Check 5)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/builtins/cargo_toml.rs` (workspace only) |
| **Acceptance** | Check 5 produces Pass when `main/src/lib.rs` or `main/src/main.rs` exists; skipped for non-workspace projects |

#### FR-705: Tests directory existence (Check 6)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `config/rules.toml` (declarative `dir_exists`) |
| **Acceptance** | Check 6 produces Pass when `tests/` exists; Warning when absent |

#### FR-706: No nested src/src/ (Check 7)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `config/rules.toml` (declarative `dir_not_exists`) |
| **Acceptance** | Check 7 produces Pass when `src/src/` does not exist; Fail when present |

#### FR-707: Benches directory (Check 8)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 -> `core/builtins/cargo_toml.rs` |
| **Acceptance** | Check 8 produces Pass when `benches/` exists if benchmarks declared; Skip when no benchmarks |

### 4.9 Planned Structure Enhancements

These requirements define the target state for structure checks per the [backlog](backlog.md) Phase 1.

#### FR-710: main/src/ as default layout (Backlog B-1.1, B-1.2, B-1.6)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-1.1, B-1.2, B-1.6 -> `config/rules.toml` |
| **Acceptance** | Checks 2 and 3 validate `main/src/` and `main/src/lib.rs` or `main/src/main.rs` as the default (not workspace-only). Checks 4 and 5 are merged into 2 and 3. |

The `{main,tests}` layout shall be enforced as the default for all projects, not only workspace members. Check 2 shall validate `main/src/` instead of `src/`. Check 3 shall validate `main/src/lib.rs` or `main/src/main.rs`.

#### FR-711: Legacy src/ warning (Backlog B-1.3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-1.3 -> `config/rules.toml` |
| **Acceptance** | A new check produces Warning when `src/` directory exists (should be `main/src/`) |

#### FR-712: No nested main/src/src/ (Backlog B-1.4)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-1.4 -> `config/rules.toml` |
| **Acceptance** | Check 7 validates `main/src/src/` instead of `src/src/` |

### 4.10 SEA Layer Validation (Planned)

These requirements define new checks for SEA layering per the [backlog](backlog.md) Phase 2.

#### FR-720: SEA directory existence (Backlog B-2.1, B-2.2, B-2.3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-2.1, B-2.2, B-2.3 -> `config/rules.toml` |
| **Acceptance** | Three new checks produce Warning when `main/src/api/`, `main/src/core/`, or `main/src/saf/` is missing (library projects only) |

#### FR-721: SEA mod.rs existence (Backlog B-2.4, B-2.5, B-2.6)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-2.4, B-2.5, B-2.6 -> `config/rules.toml` |
| **Acceptance** | Three new checks produce Warning when `main/src/api/mod.rs`, `main/src/core/mod.rs`, or `main/src/saf/mod.rs` is missing |

#### FR-722: lib.rs SEA module declarations (Backlog B-2.7)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-2.7 -> `config/rules.toml` |
| **Acceptance** | A new check validates `lib.rs` declares `pub mod api; mod core; mod saf;` |

#### FR-723: lib.rs SAF re-export (Backlog B-2.8)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-2.8 -> `config/rules.toml` |
| **Acceptance** | A new check validates `lib.rs` has `pub use saf::*;` |

#### FR-724: No unjustified spi/ (Backlog B-2.9)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-2.9 -> `config/rules.toml` |
| **Acceptance** | A new check produces Info when `main/src/spi/` exists |

### 4.11 Cargo Target Path Updates (Planned)

These requirements define target path updates per the [backlog](backlog.md) Phase 3.

#### FR-730: lib path to main/src/ (Backlog B-3.1)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-3.1 -> `core/builtins/source_layout.rs` |
| **Acceptance** | Check 19 validates that `[lib] path` points to `main/src/lib.rs` |

#### FR-731: bin path to main/src/ (Backlog B-3.2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-3.2 -> `core/builtins/source_layout.rs` |
| **Acceptance** | Check 20 validates that `[[bin]] path` points to `main/src/main.rs` |

#### FR-732: Test target naming pattern (Backlog B-3.3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-3.3 -> `core/builtins/source_layout.rs` |
| **Acceptance** | A new check validates `[[test]] name` follows `<crate>_<category>_test` pattern |

### 4.12 Test Organization Updates (Planned)

These requirements define test organization updates per the [backlog](backlog.md) Phase 4.

#### FR-740: Test file naming convention (Backlog B-4.1)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-4.1 -> `core/builtins/test_org.rs` |
| **Acceptance** | Check 33 enforces `<crate>_<category>_test.rs` naming as the default (not workspace-only) |

#### FR-741: Valid test categories (Backlog B-4.2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-4.2 -> `core/builtins/test_org.rs` |
| **Acceptance** | A new check validates category is one of: `int`, `stress`, `perf`, `load`, `e2e`, `security` |

#### FR-742: Flat tests directory (Backlog B-4.3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-4.3 -> `core/builtins/test_org.rs` |
| **Acceptance** | A new check produces Warning when `tests/` contains subdirectories |

#### FR-743: No test code outside cfg(test) (Backlog B-4.4)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-4.4 -> `core/builtins/test_org.rs` |
| **Acceptance** | Check 38 validates path from `main/src/` instead of `src/` |

### 4.13 Naming Convention Updates (Planned)

These requirements define naming convention updates per the [backlog](backlog.md) Phase 5.

#### FR-750: Source file naming glob update (Backlog B-5.1)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-5.1 -> `config/rules.toml` |
| **Acceptance** | Check 30 uses glob `main/src/**/*.rs` instead of `src/**/*.rs` |

#### FR-751: Kebab-case folder names (Backlog B-5.2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-5.2 -> `config/rules.toml` |
| **Acceptance** | A new check validates folder names use kebab-case |

#### FR-752: Kebab-case package name (Backlog B-5.3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-5.3 -> `config/rules.toml` |
| **Acceptance** | Check 27 enforces kebab-case only (not snake_case-or-kebab) |

### 4.14 Umbrella Validation (Planned)

These requirements define new checks for virtual workspace umbrellas per the [backlog](backlog.md) Phase 6.

#### FR-760: Umbrella workspace-only Cargo.toml (Backlog B-6.1)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-6.1 -> `core/builtins/` |
| **Acceptance** | A new check produces Fail when an umbrella Cargo.toml has both `[workspace]` and `[package]` |

#### FR-761: Umbrella has no source (Backlog B-6.2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-6.2 -> `core/builtins/` |
| **Acceptance** | A new check produces Fail when an umbrella has `src/` or `main/` directory |

#### FR-762: Umbrella has no lib.rs (Backlog B-6.3)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-6.3 -> `core/builtins/` |
| **Acceptance** | A new check produces Fail when an umbrella has `lib.rs` anywhere |

#### FR-763: Workspace members exist (Backlog B-6.4)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-6.4 -> `core/builtins/` |
| **Acceptance** | A new check produces Fail when any `[workspace] members` directory does not exist |

### 4.15 Optional Directory Validation (Planned)

These requirements define new checks for recognized directories per the [backlog](backlog.md) Phase 7.

#### FR-770: Recognized crate-level directories (Backlog B-7.1)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-7.1 -> `core/builtins/` |
| **Acceptance** | A new check produces Info when unrecognized directories exist at crate level. Recognized: `assets/`, `benches/`, `config/`, `examples/`, `fixtures/`, `main/`, `migrations/`, `templates/`, `tests/` |

#### FR-771: Project-level directories not inside crates (Backlog B-7.2)

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01, Backlog B-7.2 -> `core/builtins/` |
| **Acceptance** | A new check produces Info when `docs/`, `infra/`, or `scripts/` directories exist inside a crate (they belong at project/umbrella level) |

---

## 5. Non-Functional Requirements

### 5.1 Architecture

#### NFR-100: SEA compliance

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-04 -> module structure |
| **Acceptance** | Code review confirms 3-layer SEA: API (public types/traits) <- Core (private implementations) <- SAF (re-exports) <- CLI |

The crate shall follow Single-Crate Modular SEA:

| Layer | Visibility | Contents |
|-------|-----------|----------|
| L3: SAF | `pub` | Re-exports for library consumers |
| L2: CLI | binary only | `main.rs` with clap |
| L1: API | `pub` | `FileScanner`, `CheckRunner`, `ComplianceEngine`, `Reporter` traits; config/report types |
| L0: Core | `pub(crate)` | All implementations |

#### NFR-101: Dependency direction

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-04 -> module structure |
| **Acceptance** | No `use core::` in api/; core depends on api, not vice versa |

No layer shall depend on a layer above it.

### 5.2 Performance

#### NFR-200: Synchronous execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-06 -> `Cargo.toml` |
| **Acceptance** | No `tokio`, `async-std`, or other async runtime in dependencies |

#### NFR-201: Single pass

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Analysis |
| **Traces to** | SYS-02 -> `core/scanner.rs` |
| **Acceptance** | Profiling shows exactly one `walkdir` traversal per scan invocation |

The scanner shall discover files in a single directory walk.

#### NFR-202: Sub-second scan time

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-01 |
| **Acceptance** | Scan completes in < 1 second for projects with < 10,000 files |

### 5.3 Portability

#### NFR-300: Cross-platform

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Demonstration |
| **Traces to** | STK-01 |
| **Acceptance** | `cargo build` succeeds on Linux, macOS, and Windows |

### 5.4 Extensibility

#### NFR-400: Declarative rule extensibility

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Demonstration |
| **Traces to** | STK-02 -> `config/rules.toml`, `core/declarative.rs` |
| **Acceptance** | Adding a `[[rules]]` entry with `type = "file_exists"` to `rules.toml` and running `--rules` enforces the new rule without recompilation |

New declarative rules shall be addable by editing `rules.toml` alone.

#### NFR-401: Builtin handler extensibility

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Implemented |
| **Verification** | Inspection |
| **Traces to** | STK-02 -> `core/builtins/` |
| **Acceptance** | A new handler can be added in 3 steps: implement function, register in mod.rs, add TOML entry |

### 5.5 Reliability

#### NFR-500: Graceful error handling

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-05 |
| **Acceptance** | Scanning a non-existent path produces exit code 2 and a message, not a panic |

IO errors and missing files shall produce `Skip` results or clear error messages, not panics.

#### NFR-501: Invalid rules detection

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Implemented |
| **Verification** | Test |
| **Traces to** | STK-02 |
| **Acceptance** | Malformed TOML produces exit code 2 with a parse error message |

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
| Output | Report | `Result<ScanReport, ScanError>` |

### 6.3 Rules File Interface

| Aspect | Detail |
|--------|--------|
| Format | TOML |
| Schema | `[[rules]]` array of tables (FR-102) |
| Default location | Embedded in binary via `include_str!` (`config/rules.toml`) |
| Override | `--rules <path>` CLI flag |

---

## 7. Risk Analysis

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Rules TOML schema changes break existing custom rules files | High | Low | Version the schema; validate with clear error messages (NFR-501) |
| Project structure conventions change in rustboot upstream | Medium | Medium | Track upstream changes; version rules files |
| Regex patterns in TOML are hard to debug | Medium | Medium | Test each pattern in isolation; document in rules.toml comments |
| Legacy `src/` vs `main/src/` transition creates confusion | Medium | High | Keep both check variants during migration; clear deprecation warnings (FR-711) |
| Builtin handler logic diverges from convention intent | Medium | Low | Each handler traces to specific check IDs; test against fixture projects |
| Umbrella detection conflicts with workspace member detection | Medium | Medium | Clear project kind hierarchy: workspace > both > library > binary |

---

## Appendix A: Traceability Matrix

### Stakeholder -> System

| STK | SYS |
|-----|-----|
| STK-01 | SYS-01, SYS-02, SYS-04, SYS-05, SYS-06 |
| STK-02 | SYS-01 |
| STK-03 | SYS-06 |
| STK-04 | SYS-04, SYS-05 |
| STK-05 | SYS-05, SYS-06 |
| STK-06 | SYS-02 |
| STK-07 | SYS-03, SYS-04 |

### Stakeholder -> Software

| STK | FR / NFR |
|-----|----------|
| STK-01 | FR-100, FR-300, FR-301, FR-500, FR-502, FR-700-707 |
| STK-02 | FR-100, FR-101, FR-102, FR-103, FR-504, NFR-400, NFR-401 |
| STK-03 | FR-401, FR-402, FR-501 |
| STK-04 | FR-600, FR-601, FR-602, FR-603, FR-604, FR-605, NFR-100 |
| STK-05 | FR-302, FR-303, FR-400, NFR-500 |
| STK-06 | FR-201, NFR-200 |
| STK-07 | FR-250, FR-251, FR-252, FR-503 |

### Software -> Architecture

| FR / NFR | Architecture Component |
|----------|----------------------|
| FR-100, FR-101 | `core/rules.rs` |
| FR-102, FR-103 | `api/types.rs` (RuleDef, RuleType), `core/declarative.rs` |
| FR-104, FR-105 | `core/builtins/mod.rs` |
| FR-200-202 | `core/scanner.rs` |
| FR-250-252 | `core/engine.rs` |
| FR-300-303 | `core/engine.rs`, `api/types.rs` |
| FR-400-402 | `core/reporter.rs`, `main.rs` |
| FR-500-504 | `main.rs` |
| FR-600-605 | `saf/mod.rs` |
| FR-700-707 | `config/rules.toml`, `core/builtins/cargo_toml.rs` |
| FR-710-712 | `config/rules.toml` (proposed) |
| FR-720-724 | `config/rules.toml` (proposed) |
| FR-730-732 | `core/builtins/source_layout.rs` (proposed) |
| FR-740-743 | `core/builtins/test_org.rs` (proposed) |
| FR-750-752 | `config/rules.toml` (proposed) |
| FR-760-763 | `core/builtins/` (proposed) |
| FR-770-771 | `core/builtins/` (proposed) |
| NFR-100-101 | Module structure (api/, core/, saf/) |
| NFR-400-401 | `config/rules.toml`, `core/declarative.rs`, `core/builtins/` |
| NFR-500-501 | `core/engine.rs`, `core/rules.rs` |

---

## Appendix B: Check Catalog

### B.1 Current Checks (44)

| ID | Category | Description | Severity | Type |
|----|----------|-------------|----------|------|
| 1 | structure | Cargo.toml exists at root | error | file_exists |
| 2 | structure | src/ directory exists | error | dir_exists |
| 3 | structure | src/lib.rs or src/main.rs exists | error | builtin |
| 4 | structure | main/src/ directory exists (rustboot) | error | dir_exists |
| 5 | structure | main/src/lib.rs or main/src/main.rs exists (rustboot) | error | builtin |
| 6 | structure | tests/ directory exists | warning | dir_exists |
| 7 | structure | No nested src/src/ | error | dir_not_exists |
| 8 | structure | benches/ exists if declared | warning | builtin |
| 9 | cargo_metadata | package.name exists | error | cargo_key_exists |
| 10 | cargo_metadata | package.version exists | error | cargo_key_exists |
| 11 | cargo_metadata | package.edition exists | warning | cargo_key_exists |
| 12 | cargo_metadata | package.description exists | warning | cargo_key_exists |
| 13 | cargo_metadata | license field exists | warning | builtin |
| 14 | cargo_metadata | package.repository exists | info | cargo_key_exists |
| 15 | cargo_metadata | package.authors exists | info | cargo_key_exists |
| 16 | cargo_metadata | package.rust-version exists | info | cargo_key_exists |
| 17 | cargo_metadata | package.keywords exists (libraries) | info | cargo_key_exists |
| 18 | cargo_metadata | package.categories exists (libraries) | info | cargo_key_exists |
| 19 | cargo_targets | [lib] path correct | error | builtin |
| 20 | cargo_targets | [[bin]] path correct | error | builtin |
| 21 | cargo_targets | [[test]] targets declared | info | builtin |
| 22 | cargo_targets | [[bench]] harness = false | warning | builtin |
| 23 | cargo_targets | No undeclared tests | info | builtin |
| 24 | cargo_targets | No undeclared benches | info | builtin |
| 25 | cargo_targets | [[example]] targets if examples/ | info | builtin |
| 26 | cargo_targets | [[test]] paths resolve | error | builtin |
| 27 | naming | Package name snake/kebab-case | warning | cargo_key_matches |
| 28 | naming | Package name prefix (workspace) | error | cargo_key_matches |
| 29 | naming | Source files snake_case | warning | glob_naming_matches |
| 30 | naming | No uppercase in src/ dirs | warning | glob_naming_not_matches |
| 31 | naming | Module names match | error | builtin |
| 32 | naming | Binary names valid | warning | builtin |
| 33 | test_org | Test file suffixes | warning | builtin |
| 34 | test_org | Test fn category prefixes | info | builtin |
| 35 | test_org | Test fn scenario suffixes | info | builtin |
| 36 | test_org | Integration tests location | warning | builtin |
| 37 | test_org | Unit tests colocated | info | builtin |
| 38 | test_org | No test code in src/ | warning | builtin |
| 39 | documentation | README.md exists | error | file_exists |
| 40 | documentation | docs/ directory exists (libraries) | info | builtin |
| 41 | documentation | examples/ directory exists (libraries) | info | builtin |
| 42 | documentation | CHANGELOG.md exists | warning | file_exists |
| 43 | hygiene | .gitignore with target/ | warning | file_content_matches |
| 44 | hygiene | No target/ committed | error | dir_not_exists |

### B.2 Planned Checks (~15 new)

| ID (est.) | Category | Description | Severity | Source |
|-----------|----------|-------------|----------|--------|
| — | structure | Legacy src/ warning | warning | B-1.3 |
| — | sea_layer | api/ directory exists | warning | B-2.1 |
| — | sea_layer | core/ directory exists | warning | B-2.2 |
| — | sea_layer | saf/ directory exists | warning | B-2.3 |
| — | sea_layer | api/mod.rs exists | warning | B-2.4 |
| — | sea_layer | core/mod.rs exists | warning | B-2.5 |
| — | sea_layer | saf/mod.rs exists | warning | B-2.6 |
| — | sea_layer | lib.rs module declarations | info | B-2.7 |
| — | sea_layer | lib.rs SAF re-export | info | B-2.8 |
| — | sea_layer | No unjustified spi/ | info | B-2.9 |
| — | test_org | Valid test categories | warning | B-4.2 |
| — | test_org | Flat tests/ directory | warning | B-4.3 |
| — | umbrella | Workspace-only Cargo.toml | error | B-6.1 |
| — | umbrella | No src/ or main/ | error | B-6.2 |
| — | umbrella | No lib.rs | error | B-6.3 |
| — | umbrella | Members exist | error | B-6.4 |
| — | optional_dirs | Recognized crate-level dirs | info | B-7.1 |
| — | optional_dirs | Project-level dirs placement | info | B-7.2 |
