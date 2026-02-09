# doc-engine Implementation Plan

**Audience**: Developers, architects

## TLDR

This plan describes the implementation strategy for doc-engine using a Single-Crate Modular SEA architecture. It covers three milestones: a working CLI with 53 base checks (50 structural + 3 traceability), a comprehensive test suite, and self-compliance. Rules are config-driven via TOML with builtin Rust handlers for complex checks.

## Architecture Decision

> Implements: NFR-100, NFR-101

Per the SEA decision tree:
- Pluggable backends? **No** — one scanning implementation
- Domain-specific service? **Yes**
- Need clean API/impl separation? **Yes** — library usable programmatically + CLI
- Expected size? **2k-10k lines** -> **Single-Crate Modular SEA**

## Rule Sourcing Decision

> Implements: FR-100, FR-101, FR-102, FR-103, FR-104, NFR-400, NFR-401

**Option B: Config-driven rules (hybrid)**

- Simple checks (file exists, dir exists, pattern match, content grep) are **declarative** — defined entirely in a TOML rules file.
- Complex checks (link resolution, glossary ordering, cross-referencing, conditional logic) are **builtin** — referenced by handler name in TOML, implemented in Rust.
- A default `rules.toml` ships embedded in the binary via `include_str!`.
- Users can override with `--rules <path>` to add/modify/disable rules without recompiling.

### Why Hybrid

- Pure hardcoded: checklist drift, recompile for any rule tweak
- Pure config: can't express link resolution, alphabetization, or cross-file analysis
- Hybrid: declarative rules cover ~60% of checks, builtins handle the rest, both share the same TOML schema and CheckRunner pipeline

## Dependencies

> Implements: NFR-200

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"        # rules file parsing
walkdir = "2"       # recursive file discovery
regex = "1"         # filename pattern matching, link extraction
serde_yaml = "0.9"  # YAML spec file parsing
```

No async needed — file system scanning is synchronous.

## Implementation Order

### Phase 1: Scaffold

> Implements: NFR-100, NFR-101, FR-100

- Cargo workspace at `/mnt/c/phd-systems/swe-labs/swe-compliance/`
- `doc-engine/` crate with `lib` + `[[bin]]`
- SEA module structure (spi/, api/, core/, saf/)
- `main.rs` stub with clap
- Embed default `rules.toml` via `include_str!`

### Phase 2: SPI + API Types

> Implements: FR-102, FR-303, FR-304, FR-602

- All types: `CheckId`, `CheckResult`, `Severity`, `Violation`, `ScanReport`, `ScanSummary`, `ScanConfig`, `ProjectType`, `ScanContext`, `ScanError`
- Rule config types: `RuleSet`, `RuleDef`, `RuleType` (declarative variants + builtin)
- All traits: `FileScanner`, `CheckRunner`, `Reporter`, `ComplianceEngine`

### Phase 3: Rules Parser + Registry

> Implements: FR-100, FR-101, FR-102, FR-105, NFR-501

- Parse TOML `rules.toml` into `RuleSet`
- `CheckRegistry` that:
  - Instantiates `DeclarativeCheck` for each declarative rule
  - Looks up builtin handler by name for each `type = "builtin"` rule
  - Returns a `Vec<Box<dyn CheckRunner>>` ordered by check ID

### Phase 4: Scanner

> Implements: FR-200, FR-201, FR-202, NFR-201

- `FileSystemScanner` impl using walkdir
- File discovery (skip hidden dirs, target/, node_modules/)
- `file_exists()` and `read_file()` methods

### Phase 5: Declarative Check Runner

> Implements: FR-103, NFR-400

A single `DeclarativeCheck` struct that handles all declarative rule types:
- `file_exists` — check if path exists relative to root
- `dir_exists` — check if directory exists relative to root
- `dir_not_exists` — check a directory does NOT exist (anti-pattern detection)
- `file_content_matches` — single file contains a regex pattern
- `file_content_not_matches` — single file does NOT contain a pattern
- `glob_content_matches` — all files matching glob contain a pattern
- `glob_content_not_matches` — no files matching glob contain a pattern
- `glob_naming_matches` — all files matching glob have filenames matching a regex
- `glob_naming_not_matches` — no files matching glob have filenames matching a regex

### Phase 6: Builtin Check Handlers

> Implements: FR-104, FR-105, NFR-401

Rust implementations for complex checks, registered by handler name:

| Handler | Checks | Logic |
|---------|--------|-------|
| `checklist_completeness` | 8 | Verify every enforceable rule has a checkbox |
| `module_docs_plural` | 4-5 | Scan crates/modules/packages for doc/ vs docs/ |
| `sdlc_phase_numbering` | 9-10 | Validate 0-7 numbering and ordering |
| `snake_lower_case` | 21-23 | Validate filenames in docs/ are lowercase, underscore-separated, space-free |
| `guide_naming` | 24 | Validate guide/ files follow `name_{phase}_guide.md` |
| `testing_file_placement` | 25 | No `*_testing_*` outside 5-testing/ |
| `tldr_conditional` | 35-36 | Line-count-conditional TLDR check |
| `glossary_format` | 37 | Parse `**Term** - Definition.` format |
| `glossary_alphabetized` | 38 | Check alphabetical ordering of terms |
| `glossary_acronyms` | 39 | Check acronym expansions |
| `w3h_hub` | 41 | Detect W3H structure in docs/README.md |
| `hub_links_phases` | 42 | Verify hub links to all present SDLC dirs |
| `no_deep_links` | 43 | Root README doesn't deep-link into docs/ |
| `link_resolution` | 44-45 | Resolve all markdown links to existing files |
| `open_source_community_files` | 31 | Check CODE_OF_CONDUCT.md, SUPPORT.md (open-source only) |
| `open_source_github_templates` | 32 | Check .github/ISSUE_TEMPLATE/, PULL_REQUEST_TEMPLATE.md |
| `adr_naming` | 49 | Validate NNN-title.md naming |
| `adr_index_completeness` | 50 | Cross-reference ADR index against ADR files |

### Phase 7: Engine

> Implements: FR-300, FR-301, FR-302, NFR-500

- `DocComplianceEngine` implements `ComplianceEngine`
- Loads rules (embedded default or `--rules` path)
- Builds `CheckRegistry` from parsed rules
- Orchestrates: scanner discovers files -> creates `ScanContext` -> runs checks -> builds `ScanReport`
- Respects `ScanConfig.checks` filter

### Phase 8: Reporter

> Implements: FR-400, FR-401

- `TextReporter`: Human-readable grouped output
- `JsonReporter`: Serializes `ScanReport` to JSON via serde

### Phase 9: CLI Wiring

> Implements: FR-402, FR-500, FR-501, FR-502, FR-503, FR-504

- Wire clap args to `DocComplianceEngine` + reporters
- `--rules` flag for custom rules file
- Parse `--checks` ranges (e.g., `1-13` -> vec of u8)
- Exit code logic: 0/1/2

### Phase 10: Default rules.toml + Self-Compliant Docs

> Implements: FR-300 (53 checks in rules.toml), FR-600, FR-601

- Write the default `rules.toml` with all 53 checks (50 structural + 3 traceability)
- `README.md` (root)
- `docs/README.md` (hub)
- `docs/glossary.md`
- `docs/3-design/architecture.md`

### Phase 11: Spec Types + Dual-Format Parser + Discovery

> Implements: FR-700, FR-701, FR-702, FR-703, FR-704, FR-705, FR-706

- Add `serde_yaml = "0.9"` to `Cargo.toml`
- Create `spi/spec_types.rs`: `SpecFormat`, `SpecKind`, `SpecStatus`, `Priority`, `DiscoveredSpec`, `CrossRefResult`, `SpecDiagnostic`, `SpecDiagnosticKind`
- Create `api/spec_types.rs`: `SpecEnvelope`, `ParsedSpec`, `MarkdownSpec`, `MarkdownTestCase`, `BrdSpec`, `FeatureRequestSpec`, `ArchSpec`, `TestSpec`, `DeploySpec` and all nested types (`BrdDomain`, `BrdSpecRef`, `Requirement`, `Dependency`, `Component`, `TestCase`, `Environment`, `SpecValidationReport`, `CrossRefReport`)
- Create `core/spec/mod.rs`: `DocSpecEngine` struct declaration
- Create `core/spec/parser.rs`: Dual-format parser:
  - YAML path: `serde_yaml` parsing, kind-based dispatch via `SpecEnvelope`
  - Markdown path: Regex-based metadata extraction (`**Version:**`, `**Status:**`, `**Spec:**`, etc.)
  - Markdown test table parsing: extract `Verifies` column from `| ID | Test | Verifies | Priority |` tables (FR-705)
  - Error reporting as `SpecDiagnostic` for both formats
- Create `core/spec/discovery.rs`: find both extension sets:
  - YAML: `*.spec.yaml`, `*.arch.yaml`, `*.test.yaml`, `*.deploy.yaml`
  - Markdown: `*.spec`, `*.arch`, `*.test`, `*.deploy`
  - Tag each with `SpecFormat`, `SpecKind`, and feature stem (FR-706)
- Wire modules: update `spi/mod.rs`, `api/mod.rs`, `core/mod.rs`

### Phase 12: Schema Validation

> Implements: FR-710, FR-711, FR-712, FR-713, FR-714, FR-715, FR-716

- Create `core/spec/validate.rs` with dual-format validation:
  - **YAML specs:**
    - Required fields validation per kind (FR-710)
    - BRD schema: `domains[]` with `name`, `specCount`, `specs[]` (FR-711)
    - Feature request schema: `id` matching `[A-Z]+-\d{3}`, `status`, `priority`, `requirements[]` (FR-712)
    - Architecture schema: `spec` (spec ID ref), `components[]` (FR-713)
    - Test plan schema: `spec` (spec ID ref), `testCases[]` with `verifies` (FR-714)
    - Deployment schema: `spec` (spec ID ref), `environments[]` (FR-715)
    - Duplicate spec ID detection across all spec files (FR-716)
  - **Markdown specs:**
    - Required metadata: `**Version:**`, `**Status:**`, top-level heading (FR-710)
    - `.arch`/`.test`/`.deploy` must have `**Spec:**` link (FR-713, FR-714, FR-715)
    - `.test` files must have parseable test tables with `Verifies` column (FR-714)
- Add `SpecEngine` trait to `api/traits.rs`: `validate()`, `cross_ref()`, `generate()`
- `DocSpecEngine` implements `SpecEngine::validate()`

### Phase 13: Cross-Referencing

> Implements: FR-720, FR-721, FR-722, FR-723, FR-724, FR-725, FR-726, FR-727

- Create `core/spec/cross_ref.rs` with dual-format cross-referencing:
  - Dependency resolution: YAML `ref` + `file` pairs; markdown `**Related:**` refs (FR-720)
  - SDLC chain: Both formats match by feature stem (e.g., `compiler_design.spec` -> `.arch` -> `.test` -> `.deploy`; `login.spec.yaml` -> `.arch.yaml` -> `.test.yaml` -> `.deploy.yaml`) (FR-721)
  - BRD inventory: YAML `specCount` vs actual files; markdown inventory table counts vs actual `.spec` files (FR-722)
  - Test traceability: YAML `verifies` fields; markdown `Verifies` table columns (FR-723)
  - Architecture traceability: YAML `spec` field; markdown `**Spec:**` links (FR-724)
  - Related documents: YAML `relatedDocuments`; markdown `**Spec:**`/`**Arch:**` links (FR-725)
- `CrossRefReport` aggregates categorized results across both formats (FR-726)
- `DocSpecEngine` implements `SpecEngine::cross_ref()`
- Opt-in logic: skip all cross-ref if no spec files of either format found (FR-727)

### Phase 14: Markdown Generation

> Implements: FR-730, FR-731, FR-732, FR-733, FR-734, FR-735

- Create `core/spec/generate.rs`:
  - Feature request -> markdown with requirements table, dependencies, related docs (FR-730)
  - BRD -> domain inventory table with spec links (FR-731)
  - Architecture -> component descriptions and module mapping (FR-732)
  - Test plan -> test case table with verifies column (FR-733)
  - Deployment -> environment configuration table (FR-734)
  - Output to stdout (default) or file via `--output` (FR-735)
- `DocSpecEngine` implements `SpecEngine::generate()`

### Phase 15: Scan Pipeline Checks 54-68

> Implements: FR-740, FR-741, FR-742

- Create `core/builtins/spec.rs`: 12 new `CheckRunner` implementations
  - `spec_brd_exists` (check 54)
  - `spec_domain_coverage` (check 55)
  - `spec_schema_valid` (checks 56-59, dispatches by CheckId to validate specific extension pair)
  - `spec_id_format` (check 60)
  - `spec_no_duplicate_ids` (check 61)
  - `spec_test_coverage` (check 62)
  - `spec_deps_resolve` (check 63)
  - `spec_inventory_accuracy` (check 64)
  - `spec_links_resolve` (check 65)
  - `spec_test_traces` (check 66)
  - `spec_naming_convention` (check 67, validates snake_lower_case on discovered spec files of both formats)
  - `spec_stem_consistency` (check 68, verifies stems match across SDLC phases)
- Register all spec handlers in `core/builtins/mod.rs`
- Update `rules.toml` with checks 54-68 (category: `spec`)
- All spec handlers implement opt-in: `Skip` if no spec files exist

### Phase 16: CLI `spec` Subcommand

> Implements: FR-750, FR-751, FR-752, FR-753, FR-754, FR-755

- Extend `main.rs` clap structure with `spec` subcommand and sub-subcommands:
  - `spec validate <PATH> [--json]` (FR-750, FR-754, FR-755)
  - `spec cross-ref <PATH> [--json]` (FR-751, FR-754, FR-755)
  - `spec generate <FILE> [--output <DIR>]` (FR-752, FR-735)
- Exit codes: 0 = clean, 1 = violations, 2 = error (FR-753)
- Extend `core/reporter.rs` with `SpecTextReporter` and `SpecJsonReporter` (FR-754, FR-755)

### Phase 17: SAF Exports + Library API

> Implements: FR-600, FR-601, FR-602 (extended)

- Extend `saf/mod.rs` with:
  - `spec_validate(root: &Path) -> SpecValidationReport`
  - `spec_cross_ref(root: &Path) -> CrossRefReport`
  - `spec_generate(file: &Path, output: Option<&Path>) -> Result<String, ScanError>`
- Re-export all new public types: `SpecFormat`, `SpecKind`, `SpecStatus`, `Priority`, `DiscoveredSpec`, `BrdSpec`, `FeatureRequestSpec`, `ArchSpec`, `TestSpec`, `DeploySpec`, `MarkdownSpec`, `MarkdownTestCase`, `SpecEnvelope`, `ParsedSpec`, `SpecValidationReport`, `CrossRefReport`, `SpecDiagnostic`, `CrossRefResult`

### Phase 18: Documentation + Self-Compliance YAML

> Implements: Documentation updates

- Update `docs/1-requirements/requirements.md` with FR-700 through FR-755
- Update `docs/3-design/architecture.md` with spec module descriptions, scaffold, check tables
- Update `docs/2-planning/implementation_plan.md` with phases 11-18
- Verify all FR-700+ have `Traces to` pointing to an architecture component
- Verify all new phases have `> Implements:` FR references
- Verify architecture scaffold, check tables, and module descriptions cover all new modules
- Verify traceability matrices are updated

### Phase 19: Testing

> Implements: Verification of all FR and NFR requirements

- Create `tests/` directory with integration test modules
- **Fixture projects**: Build minimal project directories exercising each check:
  - `tests/fixtures/compliant/` — passes all 68 checks
  - `tests/fixtures/non_compliant/` — fails known checks with predictable violations
  - `tests/fixtures/spec_yaml/` — valid and invalid YAML spec files
  - `tests/fixtures/spec_markdown/` — valid and invalid markdown spec files
  - `tests/fixtures/no_specs/` — project with no spec files (verifies opt-in Skip behavior)
- **Unit tests** (in-module `#[cfg(test)]`):
  - `core/declarative.rs`: one test per declarative rule type (9 types, FR-103)
  - `core/rules.rs`: parse valid TOML, reject malformed TOML (NFR-501), reject unknown handler (FR-105)
  - `core/scanner.rs`: recursive discovery, exclusions, relative paths (FR-200-202)
  - `core/spec/parser.rs`: YAML kind dispatch, markdown metadata extraction, parse error reporting (FR-700-705)
  - `core/spec/validate.rs`: required fields per kind per format, duplicate ID detection (FR-710-716)
  - `core/spec/cross_ref.rs`: dependency resolution, SDLC chain, inventory accuracy, test traceability (FR-720-727)
  - `core/spec/generate.rs`: markdown output per kind (FR-730-734)
- **Integration tests** (`tests/`):
  - Full scan of compliant fixture returns exit code 0, all 68 checks pass
  - Full scan of non-compliant fixture returns exit code 1, expected violations
  - JSON output deserializes to `ScanReport` (FR-401)
  - `--checks` filter produces correct subset (FR-301)
  - `--type internal` skips open-source-only rules (FR-302)
  - `--rules` loads external file (FR-101)
  - Spec validate/cross-ref/generate subcommands produce correct output (FR-750-755)
  - Library API (`doc_engine::scan()`, `doc_engine::scan_with_config()`) returns expected results (FR-600, FR-601)
- **Regression tests**: one test per bugfix, named after the issue

## Traceability: Phase -> Requirements

| Phase | FR | NFR |
|-------|----|-----|
| 1: Scaffold | FR-100 | NFR-100, NFR-101 |
| 2: SPI + API Types | FR-102, FR-303, FR-304, FR-602 | |
| 3: Rules Parser + Registry | FR-100, FR-101, FR-102, FR-105 | NFR-501 |
| 4: Scanner | FR-200, FR-201, FR-202 | NFR-201 |
| 5: Declarative Check Runner | FR-103 | NFR-400 |
| 6: Builtin Check Handlers | FR-104, FR-105 | NFR-401 |
| 7: Engine | FR-300, FR-301, FR-302 | NFR-500 |
| 8: Reporter | FR-400, FR-401 | |
| 9: CLI Wiring | FR-402, FR-500, FR-501, FR-502, FR-503, FR-504 | |
| 10: Default rules.toml + Docs | FR-300, FR-600, FR-601 | |
| 11: Spec Types + Dual-Format Parser + Discovery | FR-700, FR-701, FR-702, FR-703, FR-704, FR-705, FR-706 | |
| 12: Schema Validation | FR-710, FR-711, FR-712, FR-713, FR-714, FR-715, FR-716 | |
| 13: Cross-Referencing | FR-720, FR-721, FR-722, FR-723, FR-724, FR-725, FR-726, FR-727 | |
| 14: Markdown Generation | FR-730, FR-731, FR-732, FR-733, FR-734, FR-735 | |
| 15: Scan Pipeline Checks 54-68 | FR-740, FR-741, FR-742 | |
| 16: CLI `spec` Subcommand | FR-750, FR-751, FR-752, FR-753, FR-754, FR-755 | |
| 17: SAF Exports + Library API | FR-600, FR-601, FR-602 (extended) | |
| 18: Documentation + Self-Compliance | — | |
| 19: Testing | FR-103, FR-105, FR-200-202, FR-300-302, FR-401, FR-600, FR-601, FR-700-755 | NFR-500, NFR-501 |

## Verification

```bash
# Build
cd /mnt/c/phd-systems/swe-labs/swe-compliance && cargo build

# Self-scan (doc-engine scans itself)
cargo run -- scan .

# Scan agent-serv (known-compliant project)
cargo run -- scan /mnt/c/phd-systems/swe-labs/agent-serv

# JSON output
cargo run -- scan /mnt/c/phd-systems/swe-labs/agent-serv --json

# Custom rules
cargo run -- scan /mnt/c/phd-systems/swe-labs/agent-serv --rules custom.toml

# Spec validation (validates YAML spec files)
cargo run -- spec validate docs/

# Spec cross-reference analysis
cargo run -- spec cross-ref docs/

# Spec cross-reference with JSON output
cargo run -- spec cross-ref docs/ --json

# Markdown generation from YAML spec
cargo run -- spec generate docs/1-requirements/auth/login.spec.yaml

# Markdown generation to file
cargo run -- spec generate docs/1-requirements/auth/login.spec.yaml --output generated/
```
