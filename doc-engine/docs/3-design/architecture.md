# doc-engine Architecture

**Audience**: Developers, architects

## TLDR

doc-engine uses a Single-Crate Modular SEA architecture with a CLI binary and reusable library. Rules are defined in TOML (simple checks declaratively, complex checks via builtin Rust handlers). The engine walks the project directory, evaluates each rule, and reports pass/fail/skip results. It also supports YAML and markdown spec file validation.

## Overview

doc-engine is a Rust CLI tool and library that programmatically audits any project against the 68 compliance checks (53 base + 15 spec) defined by the template-engine documentation framework. It follows the Single-Crate Modular SEA (Stratified Encapsulation Architecture) pattern.

Rules are **config-driven**: simple checks are defined declaratively in a TOML file (`rules.toml`), complex checks are implemented as builtin Rust handlers referenced by name from the same TOML file. A default rules file is embedded in the binary; users can override with `--rules <path>`.

Beyond markdown compliance, doc-engine supports **spec file validation** in two formats:
- **YAML specs** (`.spec.yaml`, `.arch.yaml`, `.test.yaml`, `.deploy.yaml`) — structured data with typed schemas, parsed via `serde_yaml`
- **Markdown specs** (`.spec`, `.arch`, `.test`, `.deploy`) — template-engine domain extensions with structured metadata headers, parsed via regex

Both formats are discovered, validated, and cross-referenced. YAML specs additionally support markdown generation. Spec checks (54-68) are opt-in — they produce `Skip` if no spec files of either format exist.

## Project Scaffold

```
/mnt/c/phd-systems/swe-labs/swe-compliance/
├── README.md
├── LICENSE                          # MIT
├── CHANGELOG.md
├── Cargo.toml                       # workspace root (members = ["doc-engine"])
├── doc-engine/
│   ├── Cargo.toml                   # lib + [[bin]] doc-engine
│   ├── rules.toml                   # default rules (embedded via include_str!)
│   └── src/
│       ├── lib.rs                   # pub use saf::*
│       ├── spi/
│       │   ├── mod.rs
│       │   ├── traits.rs            # FileScanner, CheckRunner, Reporter traits
│       │   ├── types.rs             # CheckId, CheckResult, Severity, Violation
│       │   └── spec_types.rs        # SpecKind, SpecStatus, Priority, CrossRefResult, SpecDiagnostic
│       ├── api/
│       │   ├── mod.rs
│       │   ├── traits.rs            # ComplianceEngine + SpecEngine traits
│       │   ├── types.rs             # ScanConfig, ScanReport, ScanSummary, RuleDef
│       │   └── spec_types.rs        # BrdSpec, FeatureRequestSpec, ArchSpec, TestSpec, DeploySpec,
│       │                            # SpecEnvelope, ParsedSpec, SpecValidationReport, CrossRefReport
│       ├── core/                    # mod core (PRIVATE)
│       │   ├── mod.rs
│       │   ├── scanner.rs           # FileSystemScanner impl
│       │   ├── engine.rs            # ComplianceEngine impl, orchestrates checks
│       │   ├── reporter.rs          # Text + JSON output (scan + spec reports)
│       │   ├── rules.rs             # TOML parser, CheckRegistry builder
│       │   ├── declarative.rs       # DeclarativeCheck: runs all TOML-defined checks
│       │   ├── builtins/
│       │   │   ├── mod.rs           # handler lookup by name
│       │   │   ├── structure.rs     # module_docs_plural, sdlc_phase_numbering
│       │   │   ├── naming.rs        # snake_lower_case, guide_naming, testing_placement
│       │   │   ├── content.rs       # tldr_conditional, glossary_format/alpha/acronyms
│       │   │   ├── navigation.rs    # w3h_hub, hub_links_phases, no_deep_links
│       │   │   ├── cross_ref.rs     # link_resolution
│       │   │   ├── adr.rs           # adr_naming, adr_index_completeness
│       │   │   ├── traceability.rs  # phase_artifact_presence, design_traces_requirements, plan_traces_design
│       │   │   └── spec.rs          # 12 spec check handlers (thin wrappers to core/spec/)
│       │   └── spec/
│       │       ├── mod.rs           # DocSpecEngine impl
│       │       ├── parser.rs        # Dual-format: YAML parsing + kind dispatch, markdown metadata extraction
│       │       ├── discovery.rs     # Find all spec files by extension
│       │       ├── validate.rs      # Schema validation per kind
│       │       ├── cross_ref.rs     # Dependency, SDLC chain, inventory, test trace checks
│       │       └── generate.rs      # Markdown generation matching template-engine templates
│       ├── saf/
│       │   └── mod.rs               # Re-exports: scan(), scan_with_config(), spec_*(), types
│       └── main.rs                  # CLI entry point (clap): scan + spec subcommands
└── docs/                            # Self-compliant with template-engine
    ├── README.md
    ├── glossary.md
    ├── 1-requirements/
    │   └── requirements.md
    ├── 2-planning/
    │   └── implementation_plan.md
    └── 3-design/
        └── architecture.md
```

## SEA Layer Model

> Implements: NFR-100, NFR-101

```
┌─────────────────────────────────────────┐
│  L4: SAF (Surface API Facade)           │  pub: scan(), scan_with_config(),
│  saf/mod.rs                             │  spec_validate(), spec_cross_ref(),
│  FR-600, FR-601, FR-602                 │  spec_generate()
├─────────────────────────────────────────┤
│  L3: main.rs (CLI)                      │  clap-based entry point
│  Depends on SAF + Reporter              │  scan + spec subcommands
│  FR-402, FR-500-504, FR-750-755         │
├─────────────────────────────────────────┤
│  L2: API (Application Interface)        │  pub trait ComplianceEngine, SpecEngine
│  api/traits.rs, api/types.rs            │  ScanConfig, ScanReport, RuleDef
│  api/spec_types.rs                      │  BrdSpec, FeatureRequestSpec, ...
│  FR-102, FR-303, FR-304,               │  SpecValidationReport, CrossRefReport
│  FR-710-716, FR-726                     │
├─────────────────────────────────────────┤
│  L1: SPI (Service Provider Interface)   │  pub trait FileScanner, CheckRunner, Reporter
│  spi/traits.rs, spi/types.rs            │  CheckId, CheckResult, Severity, Violation
│  spi/spec_types.rs                      │  SpecKind, SpecStatus, Priority,
│  FR-303, FR-304, FR-700-703             │  CrossRefResult, SpecDiagnostic
├─────────────────────────────────────────┤
│  L0: Core (Private Implementation)      │  FileSystemScanner, DocComplianceEngine
│  core/scanner.rs, core/engine.rs        │  TextReporter, JsonReporter
│  core/rules.rs, core/declarative.rs     │  TOML parser, DeclarativeCheck
│  core/builtins/*.rs                     │  Builtin check handlers (scan + spec)
│  core/spec/*.rs                         │  DocSpecEngine: parse, validate,
│  FR-100-105, FR-200-202, FR-300-302,    │  cross-ref, generate
│  FR-400, FR-401, FR-700-742             │
└─────────────────────────────────────────┘
```

### Dependency Rules

- **SAF** depends on API + SPI (re-exports public types)
- **CLI (main.rs)** depends on SAF + Core (wires concrete impls)
- **API** depends on SPI (traits reference SPI types)
- **SPI** depends on nothing (leaf layer)
- **Core** depends on SPI + API (implements traits, parses rules)
- Consumers (other crates) depend only on SAF

> **Note on CLI -> Core**: In a single-crate SEA, the CLI (L3) is inside the crate and has `pub(crate)` access to Core (L0). This is by design — `main.rs` constructs concrete types (`DocComplianceEngine`, `TextReporter`, `JsonReporter`) and wires them together, while the SAF layer provides the simplified public API for external library consumers. The encapsulation boundary is the crate boundary, not the CLI module.

## Rule Sourcing

> Implements: FR-100, FR-101, FR-102, NFR-400, NFR-401

### Two Rule Categories

Every check — whether declarative or builtin — is defined as a `[[rules]]` entry in TOML. The engine treats them uniformly through the `CheckRunner` trait.

**Declarative rules** (~60% of checks): Fully defined in TOML. A single `DeclarativeCheck` struct interprets the rule type and executes it. No Rust code changes needed to add, modify, or remove these.

**Builtin rules** (~40% of checks): TOML entry has `type = "builtin"` and a `handler` name. The `builtins/mod.rs` module looks up the handler and returns a `Box<dyn CheckRunner>`. These require Rust code for complex logic (cross-file analysis, ordering, conditional checks).

### Rules File Location

1. **Default**: Embedded in binary via `include_str!("../rules.toml")` in `core/rules.rs`
2. **Override**: `--rules <path>` CLI flag loads an external TOML file instead

### rules.toml Schema (FR-102)

```toml
# Declarative rule: file exists
[[rules]]
id = 1
category = "structure"
description = "Root docs/ folder exists (plural)"
severity = "error"
type = "dir_exists"
path = "docs"

# Declarative rule: file exists
[[rules]]
id = 2
category = "structure"
description = "docs/README.md hub document exists"
severity = "error"
type = "file_exists"
path = "docs/README.md"

# Declarative rule: anti-pattern detection
[[rules]]
id = 12
category = "structure"
description = "Developer guides in 4-development/guide/ (singular, not guides/)"
severity = "warning"
type = "dir_not_exists"
path = "docs/4-development/guides"
message = "Use guide/ (singular), not guides/"

# Declarative rule: content pattern in single file
[[rules]]
id = 7
category = "structure"
description = "Compliance checklist references architecture.md"
severity = "info"
type = "file_content_matches"
path = "docs/3-design/compliance/compliance_checklist.md"
pattern = "architecture\\.md"

# Declarative rule: all files matching glob must contain pattern
[[rules]]
id = 33
category = "content"
description = "Every .md file in docs/ has **Audience**: declaration"
severity = "error"
type = "glob_content_matches"
glob = "docs/**/*.md"
pattern = "\\*\\*Audience\\*\\*:"

# Declarative rule: filename pattern enforcement
[[rules]]
id = 22
category = "naming"
description = "All filenames in docs/ use underscores, not hyphens"
severity = "error"
type = "glob_naming_not_matches"
glob = "docs/**/*.md"
pattern = "-"
exclude_paths = ["docs/3-design/adr/", "docs/0-", "docs/1-", "docs/2-", "docs/3-", "docs/4-", "docs/5-", "docs/6-", "docs/7-"]
message = "Filename contains hyphens; use underscores (snake_lower_case)"

# Declarative rule: anti-pattern in content
[[rules]]
id = 46
category = "cross_ref"
description = "All references use docs/ (plural), not doc/"
severity = "error"
type = "glob_content_not_matches"
glob = "docs/**/*.md"
pattern = "\\bdoc/"
exclude_pattern = "docs/"

# Builtin rule: complex logic in Rust
[[rules]]
id = 4
category = "structure"
description = "All module doc folders use docs/ (plural), not doc/"
severity = "error"
type = "builtin"
handler = "module_docs_plural"

# Builtin rule with project_type filter
[[rules]]
id = 31
category = "root_files"
description = "CODE_OF_CONDUCT.md and SUPPORT.md exist (open-source)"
severity = "warning"
type = "builtin"
handler = "open_source_community_files"
project_type = "open_source"

# Builtin rule: glossary ordering
[[rules]]
id = 38
category = "content"
description = "Glossary terms are alphabetized"
severity = "warning"
type = "builtin"
handler = "glossary_alphabetized"
```

### Declarative Rule Types (FR-103)

| Type | Fields | Behavior |
|------|--------|----------|
| `file_exists` | `path` | Pass if `root/path` exists as file |
| `dir_exists` | `path` | Pass if `root/path` exists as directory |
| `dir_not_exists` | `path`, `message` | Pass if `root/path` does NOT exist |
| `file_content_matches` | `path`, `pattern` | Pass if file contains regex |
| `file_content_not_matches` | `path`, `pattern` | Pass if file does NOT contain regex |
| `glob_content_matches` | `glob`, `pattern` | Pass if ALL matching files contain regex |
| `glob_content_not_matches` | `glob`, `pattern`, `exclude_pattern?` | Pass if NO matching files contain regex (excluding lines matching exclude_pattern) |
| `glob_naming_matches` | `glob`, `pattern` | Pass if ALL matching filenames match regex |
| `glob_naming_not_matches` | `glob`, `pattern`, `exclude_paths?` | Pass if NO matching filenames match regex (excluding listed path prefixes) |

### Builtin Handlers (FR-104, FR-105)

| Handler | Checks | Logic |
|---------|--------|-------|
| `checklist_completeness` | 8 | Verify every enforceable rule in compliance checklist has a checkbox |
| `module_docs_plural` | 4-5 | Scan crates/modules/packages for `doc/` vs `docs/` |
| `sdlc_phase_numbering` | 9-10 | Validate 0-7 numbering and correct ordering of phase dirs |
| `snake_lower_case` | 21-23 | Validate filenames in docs/ are lowercase, underscore-separated, space-free |
| `guide_naming` | 24 | Validate guide/ files follow `name_{phase}_guide.md` convention |
| `testing_file_placement` | 25 | No `*_testing_*` files outside `5-testing/` |
| `tldr_conditional` | 35-36 | Require TLDR on 200+ line docs, flag TLDR on shorter docs |
| `glossary_format` | 37 | Parse and validate `**Term** - Definition.` format |
| `glossary_alphabetized` | 38 | Check term alphabetical ordering |
| `glossary_acronyms` | 39 | Check acronym expansions present |
| `w3h_hub` | 41 | Detect W3H structure in docs/README.md |
| `hub_links_phases` | 42 | Verify hub links to all present SDLC phase directories |
| `no_deep_links` | 43 | Root README doesn't deep-link into docs/ subdirs |
| `link_resolution` | 44-45 | Resolve all markdown links to existing files |
| `adr_naming` | 49 | Validate `NNN-title.md` naming in adr/ |
| `adr_index_completeness` | 50 | Cross-reference ADR index against ADR files |
| `open_source_community_files` | 31 | Check CODE_OF_CONDUCT.md, SUPPORT.md (open-source only) |
| `open_source_github_templates` | 32 | Check .github/ISSUE_TEMPLATE/, PULL_REQUEST_TEMPLATE.md |
| `phase_artifact_presence` | 51 | Verify SDLC phase dirs contain expected artifacts |
| `design_traces_requirements` | 52 | Design docs reference requirements |
| `plan_traces_design` | 53 | Planning docs reference architecture |

## SPI Layer (L1)

> Implements: FR-303, FR-304, FR-700-703

### traits.rs

```rust
pub trait FileScanner {
    fn scan_files(&self, root: &Path) -> Vec<PathBuf>;
    fn file_exists(&self, root: &Path, relative: &str) -> bool;
    fn read_file(&self, path: &Path) -> Result<String, ScanError>;
}

pub trait CheckRunner: Send + Sync {
    fn id(&self) -> CheckId;
    fn category(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, ctx: &ScanContext) -> CheckResult;
}

pub trait Reporter {
    fn report(&self, report: &ScanReport) -> String;
}
```

### types.rs

```rust
pub struct CheckId(pub u8);                    // 1-68
pub enum Severity { Error, Warning, Info }
pub struct Violation { check: CheckId, path: Option<PathBuf>, message: String, severity: Severity }
pub enum CheckResult { Pass, Fail(Vec<Violation>), Skip(String) }
pub struct ScanContext<'a> { root, files, scanner, project_type }
pub enum ScanError { Io(std::io::Error), Path(String) }
```

### spec_types.rs

> Implements: FR-700, FR-701, FR-703, FR-704, FR-706

```rust
/// Whether a spec file is YAML (.spec.yaml) or markdown (.spec)
pub enum SpecFormat { Yaml, Markdown }

/// SDLC role of a spec file — determined by extension
pub enum SpecKind { Brd, FeatureRequest, Architecture, TestPlan, Deployment }

/// Status of a feature request or requirement
pub enum SpecStatus { Draft, Proposed, Approved, Implemented, Verified, Deprecated }

/// Priority (MoSCoW)
pub enum Priority { Must, Should, May, Wont }

/// A discovered spec file with its format and kind
pub struct DiscoveredSpec {
    pub path: PathBuf,
    pub format: SpecFormat,
    pub kind: SpecKind,
    pub stem: String,         // feature stem for SDLC chain matching (e.g., "compiler_design")
}

/// Result of a single cross-reference check
pub enum CrossRefResult {
    Pass { description: String },
    Fail { description: String, details: Vec<String> },
}

/// Diagnostic from spec parsing or validation
pub struct SpecDiagnostic {
    pub file: PathBuf,
    pub line: Option<usize>,
    pub kind: SpecDiagnosticKind,
    pub message: String,
}

pub enum SpecDiagnosticKind { ParseError, SchemaError, CrossRefError }
```

## API Layer (L2)

> Implements: FR-102, FR-600, FR-601, FR-602, FR-710-716, FR-726

### traits.rs

```rust
pub trait ComplianceEngine {
    fn scan(&self, root: &Path) -> Result<ScanReport, ScanError>;
    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError>;
}

pub trait SpecEngine {
    fn validate(&self, root: &Path) -> SpecValidationReport;
    fn cross_ref(&self, root: &Path) -> CrossRefReport;
    fn generate(&self, file: &Path, output: Option<&Path>) -> Result<String, ScanError>;
}
```

### types.rs

```rust
pub enum ProjectType { OpenSource, Internal }
pub struct ScanConfig {
    project_type: ProjectType,
    checks: Option<Vec<u8>>,
    rules_path: Option<PathBuf>,    // external rules file override
}
pub struct ScanSummary { total: u8, passed: u8, failed: u8, skipped: u8 }
pub struct CheckEntry {
    pub id: CheckId,
    pub category: String,
    pub description: String,
    pub result: CheckResult,
}
pub struct ScanReport { results: Vec<CheckEntry>, summary: ScanSummary, project_type: ProjectType }
pub struct RuleSet { pub rules: Vec<RuleDef> }

/// Deserialized from TOML [[rules]] entry.
pub struct RuleDef {
    pub id: u8,
    pub category: String,
    pub description: String,
    pub severity: Severity,
    pub rule_type: RuleType,
    pub project_type: Option<ProjectType>,  // None = all project types
}

pub enum RuleType {
    // Declarative
    FileExists { path: String },
    DirExists { path: String },
    DirNotExists { path: String, message: String },
    FileContentMatches { path: String, pattern: String },
    FileContentNotMatches { path: String, pattern: String },
    GlobContentMatches { glob: String, pattern: String },
    GlobContentNotMatches { glob: String, pattern: String, exclude_pattern: Option<String> },
    GlobNamingMatches { glob: String, pattern: String },
    GlobNamingNotMatches { glob: String, pattern: String, exclude_paths: Option<Vec<String>> },
    // Builtin
    Builtin { handler: String },
}
```

### spec_types.rs

> Implements: FR-704, FR-705, FR-711-715, FR-726

```rust
/// Envelope: common fields for all YAML spec files
pub struct SpecEnvelope {
    pub kind: SpecKind,
    pub schema_version: String,
    pub title: String,
}

/// Parsed and validated spec — kind-specific payload
pub enum ParsedSpec {
    // YAML format
    Brd(BrdSpec),
    FeatureRequest(FeatureRequestSpec),
    Architecture(ArchSpec),
    TestPlan(TestSpec),
    Deployment(DeploySpec),
    // Markdown format
    Markdown(MarkdownSpec),
}

/// Metadata extracted from a markdown-format spec file (.spec, .arch, .test, .deploy)
pub struct MarkdownSpec {
    pub path: PathBuf,
    pub kind: SpecKind,
    pub title: String,                            // from first # heading
    pub version: Option<String>,                  // **Version:** value
    pub status: Option<String>,                   // **Status:** value
    pub related: Option<String>,                  // **Related:** value
    pub spec_link: Option<(String, String)>,      // **Spec:** linked name + path
    pub arch_link: Option<(String, String)>,      // **Arch:** linked name + path
    pub requirements_range: Option<String>,        // **Requirements:** value
    pub test_cases: Vec<MarkdownTestCase>,         // extracted from test tables
}

/// A test case row extracted from a markdown .test file table
pub struct MarkdownTestCase {
    pub id: String,
    pub description: String,
    pub verifies: String,
    pub priority: Option<String>,
}

/// BRD: master requirements inventory (brd.spec.yaml)
pub struct BrdSpec {
    pub kind: SpecKind,
    pub schema_version: String,
    pub title: String,
    pub domains: Vec<BrdDomain>,
}
pub struct BrdDomain {
    pub name: String,
    pub spec_count: usize,
    pub specs: Vec<BrdSpecRef>,
}
pub struct BrdSpecRef {
    pub id: String,       // FR-### ID
    pub title: String,
    pub file: String,     // relative path to .spec.yaml
}

/// Feature request spec ({name}.spec.yaml)
pub struct FeatureRequestSpec {
    pub kind: SpecKind,
    pub schema_version: String,
    pub id: String,               // FR-### pattern
    pub title: String,
    pub status: SpecStatus,
    pub priority: Priority,
    pub requirements: Vec<Requirement>,
    pub dependencies: Option<Vec<Dependency>>,
    pub related_documents: Option<Vec<String>>,
}
pub struct Requirement {
    pub id: String,               // e.g., "REQ-001"
    pub description: String,
    pub priority: Priority,
}
pub struct Dependency {
    pub ref_id: String,           // FR-### reference
    pub file: String,             // path to spec file
}

/// Architecture spec (*.arch.yaml)
pub struct ArchSpec {
    pub kind: SpecKind,
    pub schema_version: String,
    pub spec: String,             // spec ID this architecture maps to
    pub title: String,
    pub components: Vec<Component>,
    pub dependencies: Option<Vec<Dependency>>,
    pub related_documents: Option<Vec<String>>,
}
pub struct Component {
    pub name: String,
    pub description: String,
    pub module: Option<String>,   // code module path
}

/// Test plan spec (*.test.yaml)
pub struct TestSpec {
    pub kind: SpecKind,
    pub schema_version: String,
    pub spec: String,             // spec ID this test plan covers
    pub title: String,
    pub test_cases: Vec<TestCase>,
}
pub struct TestCase {
    pub id: String,
    pub description: String,
    pub verifies: String,         // requirement ID (e.g., "REQ-001")
    pub test_type: Option<String>,
}

/// Deployment spec (*.deploy.yaml)
pub struct DeploySpec {
    pub kind: SpecKind,
    pub schema_version: String,
    pub spec: String,             // spec ID
    pub title: String,
    pub environments: Vec<Environment>,
}
pub struct Environment {
    pub name: String,
    pub description: String,
}

/// Validation report from spec validate
pub struct SpecValidationReport {
    pub files_scanned: usize,
    pub diagnostics: Vec<SpecDiagnostic>,
    pub passed: usize,
    pub failed: usize,
}

/// Cross-reference report from spec cross-ref
pub struct CrossRefReport {
    pub dependency_results: Vec<CrossRefResult>,
    pub sdlc_chain_results: Vec<CrossRefResult>,
    pub inventory_results: Vec<CrossRefResult>,
    pub test_trace_results: Vec<CrossRefResult>,
    pub arch_trace_results: Vec<CrossRefResult>,
    pub related_doc_results: Vec<CrossRefResult>,
    pub total_checks: usize,
    pub passed: usize,
    pub failed: usize,
}
```

## Core Layer (L0)

### scanner.rs — FileSystemScanner (FR-200, FR-201, FR-202, NFR-201)

- Uses `walkdir` crate for recursive file discovery
- Skips hidden directories, `target/`, `node_modules/`
- Returns paths relative to project root

### rules.rs — TOML Parser + CheckRegistry (FR-100, FR-101, FR-102, FR-105, NFR-501)

- Parses `rules.toml` (embedded default or external file) into `Vec<RuleDef>`
- `build_registry(rules: Vec<RuleDef>) -> Vec<Box<dyn CheckRunner>>`
  - Declarative rules -> `DeclarativeCheck` wrapper
  - Builtin rules -> handler lookup in `builtins/mod.rs`
  - Sorted by check ID

### declarative.rs — DeclarativeCheck (FR-103, NFR-400)

- Single struct that wraps a `RuleDef` with a declarative `RuleType`
- Implements `CheckRunner` by dispatching on the rule type variant
- Uses `regex` crate for pattern matching, glob matching against the file list

### builtins/ — Builtin Handlers (FR-104, FR-105, NFR-401)

- `mod.rs`: `fn get_handler(name: &str, def: &RuleDef) -> Option<Box<dyn CheckRunner>>`
- Each module contains handler functions returning `Box<dyn CheckRunner>`

### engine.rs — DocComplianceEngine (FR-300, FR-301, FR-302, NFR-500)

- Implements `ComplianceEngine` trait
- Orchestration flow:
  1. Load rules (embedded or external)
  2. Build CheckRegistry
  3. Scanner discovers all files under root
  4. Creates `ScanContext` with file list + scanner ref
  5. Iterates through registry (filtered by config checks + project_type)
  6. Collects `(CheckId, CheckResult)` pairs
  7. Computes `ScanSummary`
  8. Returns `ScanReport`

### reporter.rs — TextReporter + JsonReporter + SpecReporters (FR-400, FR-401, FR-754, FR-755)

- `TextReporter`: Groups results by category, shows violations with paths
- `JsonReporter`: Serializes `ScanReport` via serde_json
- `SpecTextReporter`: Formats `SpecValidationReport` and `CrossRefReport` as human-readable text
- `SpecJsonReporter`: Serializes `SpecValidationReport` and `CrossRefReport` to JSON via serde_json

### builtins/spec.rs — Spec Check Handlers (FR-740, FR-741, FR-727)

Twelve builtin handlers for checks 54-68. Each is a thin wrapper that delegates to `core/spec/` modules:

| Handler | Checks | Delegates to |
|---------|--------|-------------|
| `spec_brd_exists` | 54 | `spec::discovery` |
| `spec_domain_coverage` | 55 | `spec::discovery` |
| `spec_schema_valid` | 56-59 | `spec::parser` + `spec::validate` |
| `spec_id_format` | 60 | `spec::validate` |
| `spec_no_duplicate_ids` | 61 | `spec::validate` |
| `spec_test_coverage` | 62 | `spec::cross_ref` |
| `spec_deps_resolve` | 63 | `spec::cross_ref` |
| `spec_inventory_accuracy` | 64 | `spec::cross_ref` |
| `spec_links_resolve` | 65 | `spec::cross_ref` |
| `spec_test_traces` | 66 | `spec::cross_ref` |
| `spec_naming_convention` | 67 | `spec::discovery` (filename validation) |
| `spec_stem_consistency` | 68 | `spec::cross_ref` |

#### spec_schema_valid dispatch by CheckId

The `spec_schema_valid` handler appears as four separate `[[rules]]` entries in `rules.toml` (checks 56-59), all with `handler = "spec_schema_valid"`. The handler dispatches by its own `CheckId` to determine which extension pair to validate:

| CheckId | Extensions validated |
|---------|---------------------|
| 56 | `.spec`, `.spec.yaml` |
| 57 | `.arch`, `.arch.yaml` |
| 58 | `.test`, `.test.yaml` |
| 59 | `.deploy`, `.deploy.yaml` |

All spec handlers implement opt-in behavior (FR-727): if no spec files of either format are discovered, they return `CheckResult::Skip("No spec files found")`.

### spec/ — DocSpecEngine (FR-700-742)

#### spec/mod.rs — DocSpecEngine

- Implements `SpecEngine` trait
- Orchestration: discover -> parse (format-aware) -> validate -> cross-ref / generate
- **Dual-mode file sourcing** (NFR-201 compliance):
  - **Scan pipeline mode** (checks 54-68): receives `ScanContext.files` from the already-completed walkdir traversal and filters by spec extensions — no second traversal
  - **Standalone mode** (`spec validate`, `spec cross-ref`, `spec generate`): performs its own `FileSystemScanner::scan_files()` traversal since no prior scan context exists

#### spec/parser.rs — Dual-Format Parser (FR-700, FR-701, FR-703, FR-704, FR-705)

- **YAML path**: Uses `serde_yaml` to parse `.spec.yaml`/`.arch.yaml`/`.test.yaml`/`.deploy.yaml`; reads `kind` field first (via `SpecEnvelope`) to dispatch to the correct schema type
- **Markdown path**: Uses regex to extract metadata from `.spec`/`.arch`/`.test`/`.deploy` files:
  - Headers: `**Version:**`, `**Status:**`, `**Related:**`, `**Spec:**`, `**Arch:**`, `**Requirements:**`
  - Test tables: parses `| ID | Test | Verifies | Priority |` markdown tables to extract `MarkdownTestCase` rows
- Reports parse errors as `SpecDiagnostic` with file path and error details

#### spec/discovery.rs — Spec File Discovery (FR-702, FR-706)

- Finds all files matching both extension sets:
  - YAML: `*.spec.yaml`, `*.arch.yaml`, `*.test.yaml`, `*.deploy.yaml`
  - Markdown: `*.spec`, `*.arch`, `*.test`, `*.deploy`
- Tags each with `SpecFormat` (Yaml/Markdown), `SpecKind`, and feature stem
- Groups discovered files by kind for downstream processing
- Feature stem extraction: `compiler_design.spec` -> stem `compiler_design`; `login.spec.yaml` -> stem `login`
- Identifies BRD file (`brd.spec.yaml` or `brd.spec` in `1-requirements/`)

#### spec/validate.rs — Schema Validation (FR-710-716)

**YAML specs:**
- Validates required fields per kind (FR-710)
- Validates BRD schema: domains, specCount, specs (FR-711)
- Validates feature request schema: id format, requirements (FR-712)
- Validates architecture schema: spec reference, components (FR-713)
- Validates test plan schema: spec reference, testCases with verifies (FR-714)
- Validates deployment schema: spec reference, environments (FR-715)
- Detects duplicate spec IDs across all spec files (FR-716)

**Markdown specs:**
- Validates required metadata: `**Version:**`, `**Status:**`, top-level heading (FR-710)
- Validates `.arch`/`.test`/`.deploy` have `**Spec:**` link (FR-713, FR-714, FR-715)
- Validates `.test` files have parseable test tables with `Verifies` column (FR-714)

#### spec/cross_ref.rs — Cross-Reference Analysis (FR-720-727)

Cross-referencing works across both formats, using format-appropriate strategies:

- **Dependency resolution** (FR-720): YAML `dependencies[].ref` + `file` resolve; markdown `**Related:**` IDs resolve
- **SDLC chain** (FR-721): Both formats match by feature stem (e.g., `compiler_design.spec` -> `.arch` -> `.test` -> `.deploy`; `login.spec.yaml` -> `.arch.yaml` -> `.test.yaml` -> `.deploy.yaml`)
- **BRD inventory** (FR-722): YAML `specCount` matches actual files; markdown inventory table counts match actual `.spec` file counts per domain
- **Test traceability** (FR-723): YAML `testCases[].verifies` matches requirement IDs; markdown `Verifies` column values match requirement IDs in linked spec
- **Architecture traceability** (FR-724): YAML `.arch.yaml` `spec` field matches a spec ID; markdown `.arch` `**Spec:**` link resolves to existing `.spec` file
- **Related documents** (FR-725): YAML `relatedDocuments` paths resolve; markdown `**Spec:**`/`**Arch:**` links resolve

#### spec/generate.rs — Markdown Generation (FR-730-735)

- Generates markdown matching template-engine templates
- Per-kind templates:
  - BRD -> domain inventory table with links
  - Feature request -> requirements table with traceability
  - Architecture -> component descriptions
  - Test plan -> test case table with verifies column
  - Deployment -> environment configuration table
- Output to file (`--output`) or stdout (FR-735)

## Detailed Check Specifications (FR-300)

### Checks 1-13: Structure

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 1 | Root `docs/` folder exists (plural) | `dir_exists` | Error |
| 2 | `docs/README.md` hub document exists | `file_exists` | Error |
| 3 | `docs/glossary.md` exists | `file_exists` | Error |
| 4 | All module doc folders use `docs/` (plural) | `builtin: module_docs_plural` | Error |
| 5 | No module has both `doc/` and `docs/` | `builtin: module_docs_plural` | Error |
| 6 | `docs/3-design/compliance/compliance_checklist.md` exists | `file_exists` | Warning |
| 7 | Compliance checklist references `architecture.md` | `file_content_matches` | Info |
| 8 | Every enforceable rule has a checkbox | `builtin: checklist_completeness` | Info |
| 9 | SDLC phase numbering correct | `builtin: sdlc_phase_numbering` | Warning |
| 10 | Phases in correct order | `builtin: sdlc_phase_numbering` | Warning |
| 11 | ADRs in `3-design/adr/` | `dir_exists` | Warning |
| 12 | Developer guides in `guide/` (singular) | `dir_not_exists` (guides/) | Warning |
| 13 | UX/UI assets in `uxui/` | `dir_not_exists` (uiux/) | Warning |

### Checks 14-25: Naming

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 14-20 | Git files UPPERCASE | `file_exists` (exact casing) | Error |
| 21-23 | Docs snake_lower_case | `builtin: snake_lower_case` | Error |
| 24 | Guide naming convention | `builtin: guide_naming` | Warning |
| 25 | No testing files misplaced | `builtin: testing_file_placement` | Warning |

### Checks 26-32: Root Files

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 26 | README.md exists | `file_exists` | Error |
| 27 | CONTRIBUTING.md exists | `file_exists` | Error |
| 28 | CHANGELOG.md exists | `file_exists` | Warning |
| 29 | SECURITY.md exists | `file_exists` | Error |
| 30 | LICENSE exists | `file_exists` | Error |
| 31 | Open-source community files | `builtin: open_source_community_files` | Warning |
| 32 | Open-source GitHub templates | `builtin: open_source_github_templates` | Warning |

### Checks 33-39: Content

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 33 | Audience in all docs/ .md | `glob_content_matches` | Error |
| 34 | Audience in module READMEs | `glob_content_matches` | Error |
| 35-36 | TLDR conditional | `builtin: tldr_conditional` | Warning/Info |
| 37 | Glossary format | `builtin: glossary_format` | Warning |
| 38 | Glossary alphabetized | `builtin: glossary_alphabetized` | Warning |
| 39 | Acronym expansion | `builtin: glossary_acronyms` | Info |

### Checks 40-43: Navigation

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 40 | Root README links to docs/README.md | `file_content_matches` | Error |
| 41 | Hub is W3H | `builtin: w3h_hub` | Warning |
| 42 | Hub links all phases | `builtin: hub_links_phases` | Warning |
| 43 | No deep links from root | `builtin: no_deep_links` | Warning |

### Checks 44-47: Cross-References

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 44-45 | Links resolve | `builtin: link_resolution` | Error/Warning |
| 46 | `docs/` not `doc/` | `glob_content_not_matches` | Error |
| 47 | `guide/` not `guides/` | `glob_content_not_matches` | Error |

### Checks 48-50: ADR

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 48 | ADR index exists | `file_exists` | Warning |
| 49 | ADR NNN-title.md naming | `builtin: adr_naming` | Warning |
| 50 | ADR index completeness | `builtin: adr_index_completeness` | Info |

### Checks 51-53: Traceability

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 51 | Populated SDLC phase dirs contain expected artifact | `builtin: phase_artifact_presence` | Warning |
| 52 | Design documents reference requirements | `builtin: design_traces_requirements` | Warning |
| 53 | Planning documents reference architecture | `builtin: plan_traces_design` | Warning |

### Checks 54-68: Spec (opt-in)

> Implements: FR-740, FR-741, FR-742, FR-727

All spec checks produce `Skip` if no spec files of either format exist in the project (opt-in behavior, FR-727).

| Check | Description | Rule Type | Severity |
|-------|-------------|-----------|----------|
| 54 | BRD spec file exists (`brd.spec.yaml` or `brd.spec`) | `builtin: spec_brd_exists` | Warning |
| 55 | Domain directories have spec files (`.spec.yaml` or `.spec`) | `builtin: spec_domain_coverage` | Warning |
| 56 | All `.spec.yaml` and `.spec` files parse correctly | `builtin: spec_schema_valid` | Error |
| 57 | All `.arch.yaml` and `.arch` files parse correctly | `builtin: spec_schema_valid` | Error |
| 58 | All `.test.yaml` and `.test` files parse correctly | `builtin: spec_schema_valid` | Error |
| 59 | All `.deploy.yaml` and `.deploy` files parse correctly | `builtin: spec_schema_valid` | Error |
| 60 | Spec IDs match `[A-Z]+-\d{3}` pattern (both formats) | `builtin: spec_id_format` | Error |
| 61 | No duplicate spec IDs across spec files (both formats) | `builtin: spec_no_duplicate_ids` | Error |
| 62 | Every spec has a matching test plan (both formats) | `builtin: spec_test_coverage` | Warning |
| 63 | Dependency/related refs resolve | `builtin: spec_deps_resolve` | Error |
| 64 | BRD inventory counts match actual files | `builtin: spec_inventory_accuracy` | Warning |
| 65 | Spec cross-links resolve (`relatedDocuments` / `**Spec:**`) | `builtin: spec_links_resolve` | Warning |
| 66 | Test verifies fields trace to valid requirement IDs | `builtin: spec_test_traces` | Error |
| 67 | Spec files follow `snake_lower_case` naming (both formats) | `builtin: spec_naming_convention` | Warning |
| 68 | Spec file stems match across SDLC phases | `builtin: spec_stem_consistency` | Warning |

## CLI Design

> Implements: FR-402, FR-500, FR-501, FR-502, FR-503, FR-504, FR-750, FR-751, FR-752, FR-753, FR-754, FR-755

### Scan subcommand

```
doc-engine scan <PATH>                  # FR-500: scan project, exit 1 if any failures
doc-engine scan <PATH> --json           # FR-501: JSON output
doc-engine scan <PATH> --checks 1-13    # FR-502: run specific checks only
doc-engine scan <PATH> --type internal  # FR-503: override project type detection
doc-engine scan <PATH> --rules my.toml  # FR-504: use custom rules file
```

### Spec subcommand

```
doc-engine spec validate <PATH>              # FR-750: validate all spec files under PATH
doc-engine spec validate <PATH> --json       # FR-754: JSON output
doc-engine spec cross-ref <PATH>             # FR-751: cross-reference analysis
doc-engine spec cross-ref <PATH> --json      # FR-754: JSON output
doc-engine spec generate <FILE>              # FR-752: generate markdown to stdout
doc-engine spec generate <FILE> --output DIR # FR-735: generate markdown to file
```

Uses `clap` derive API. Exit code 0 = all pass, 1 = failures/violations found, 2 = error (FR-402, FR-753).

### Spec File Conventions

**YAML format** (structured, typed schemas):

| Extension | Kind | Directory | Naming Convention |
|-----------|------|-----------|-------------------|
| `brd.spec.yaml` | `brd` | `1-requirements/` | Exactly `brd.spec.yaml` |
| `{name}.spec.yaml` | `feature_request` | `1-requirements/{domain}/` | `[a-z_]+\.spec\.yaml` |
| `{name}.arch.yaml` | `architecture` | `3-design/{domain}/` | `[a-z_]+\.arch\.yaml` |
| `{name}.test.yaml` | `test_plan` | `5-testing/{domain}/` | `[a-z_]+\.test\.yaml` |
| `{name}.deploy.yaml` | `deployment` | `6-deployment/{domain}/` | `[a-z_]+\.deploy\.yaml` |

**Markdown format** (template-engine domain extensions):

| Extension | Kind | Directory | Naming Convention |
|-----------|------|-----------|-------------------|
| `brd.spec` | `brd` | `1-requirements/` | Exactly `brd.spec` |
| `{name}.spec` | `spec` | `1-requirements/{domain}/` | `[a-z_]+\.spec` |
| `{name}.arch` | `architecture` | `3-design/{domain}/` | `[a-z_]+\.arch` |
| `{name}.test` | `test_plan` | `5-testing/{domain}/` | `[a-z_]+\.test` |
| `{name}.deploy` | `deployment` | `6-deployment/{domain}/` | `[a-z_]+\.deploy` |

**SDLC chain matching**: Both formats use **feature stems** — the shared filename without the domain extension (e.g., `compiler_design.spec` -> `compiler_design.arch` -> `compiler_design.test` -> `compiler_design.deploy`; `login.spec.yaml` -> `login.arch.yaml` -> `login.test.yaml` -> `login.deploy.yaml`). Spec IDs (e.g., `RS-030`) live inside the file content, not in filenames.

### Cross-Referencing Rules (detailed)

Each rule applies to both YAML and markdown formats using format-appropriate extraction:

1. **Dependency resolution** (FR-720):
   - YAML: Each `.spec.yaml` dependency entry's `ref` + `file` must resolve
   - Markdown: Each `**Related:**` reference must correspond to an existing spec file
2. **SDLC chain** (FR-721):
   - Both formats: Each `{name}.spec[.yaml]` should have matching `{name}.arch[.yaml]`, `{name}.test[.yaml]`, `{name}.deploy[.yaml]` in the corresponding SDLC phase directories (e.g., `compiler_design.spec` in `1-requirements/compiler/` -> `compiler_design.arch` in `3-design/compiler/`; `login.spec.yaml` in `1-requirements/auth/` -> `login.arch.yaml` in `3-design/auth/`)
3. **BRD inventory** (FR-722):
   - YAML: `specCount` per domain matches actual `.spec.yaml` file count; `specs[].file` resolves
   - Markdown: Inventory table counts match actual `.spec` file counts per domain directory
4. **Test traceability** (FR-723):
   - YAML: Each `testCases[].verifies` matches a requirement ID in the linked spec's `requirements[]`
   - Markdown: Each `Verifies` column value in `.test` tables matches a requirement ID defined in the linked `.spec`
5. **Architecture traceability** (FR-724):
   - YAML: Each `.arch.yaml`'s `spec` field matches a spec ID in a `.spec.yaml`
   - Markdown: Each `.arch`'s `**Spec:**` link resolves to an existing `.spec` file
6. **Related documents** (FR-725):
   - YAML: All `relatedDocuments` paths resolve to existing files
   - Markdown: All `**Spec:**`, `**Arch:**` links resolve to existing files
