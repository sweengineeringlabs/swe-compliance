// ---------------------------------------------------------------------------
// Shared fixture SRS documents for scaffold integration and E2E tests
// ---------------------------------------------------------------------------

pub const FIXTURE_SRS: &str = "\
# Software Requirements Specification

## 4. Software Requirements

### 4.1 Rule Loading

#### FR-100: Default rules embedded in binary

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> core/rules.rs |
| **Acceptance** | Engine loads embedded rules |

The binary shall embed a default rules.toml.

#### FR-101: External rules file override

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-02 |
| **Acceptance** | External rules override |

Load external TOML file.

### 4.2 File Discovery

#### FR-200: Recursive scanning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | SYS-02 -> core/scanner.rs |
| **Acceptance** | Nested dirs discovered |

Recursively discover all files.
";

/// Large fixture with 5 domains and mixed FR/NFR, exercising many parser paths.
pub const LARGE_FIXTURE_SRS: &str = "\
# Software Requirements Specification

## 4. Software Requirements

### 4.1 Rule Loading

#### FR-100: Default rules embedded in binary

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> core/rules.rs |
| **Acceptance** | Engine loads embedded rules |

The binary shall embed a default rules.toml.

#### FR-101: External rules file override

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-02 |
| **Acceptance** | External rules override embedded |

Load external TOML file.

#### FR-102: TOML rules schema

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | STK-02 -> api/types.rs |
| **Acceptance** | TOML parser accepts all fields |

Each rule entry shall contain required fields.

### 4.2 File Discovery

#### FR-200: Recursive scanning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | SYS-02 -> core/scanner.rs |
| **Acceptance** | Nested dirs 5 levels deep discovered |

Recursively discover all files under root.

#### FR-201: Directory exclusions

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | SYS-02 -> core/scanner.rs |
| **Acceptance** | .git/, target/, node_modules/ excluded |

Skip hidden directories, target/, node_modules/.

### 4.3 Check Execution

#### FR-300: All checks

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> core/engine.rs |
| **Acceptance** | Full scan produces 128 check results |

The engine shall support 128 checks.

#### FR-301: Check filtering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 -> core/engine.rs |
| **Acceptance** | --checks 1-13 produces exactly 13 results |

Comma-separated or range filtering.

### 4.4 Reporting

#### FR-400: Text output

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Traces to** | STK-05 -> core/reporter.rs |
| **Acceptance** | Grouped results with summary line |

Default output shall be human-readable text.

### 5.1 Architecture

#### NFR-100: SEA compliance

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | SYS-01 |
| **Acceptance** | Module graph matches SEA layers |

Must follow Stratified Encapsulation Architecture.

#### NFR-101: Dependency direction

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | SYS-01 |
| **Acceptance** | No upward dependencies |

Dependencies flow inward only.
";

/// Fixture that exercises all 4 verification methods with backtick commands,
/// file traces, and prose — designed to test auto-populated steps.
pub const STEPS_FIXTURE_SRS: &str = "\
### 4.1 CLI Interface

#### FR-500: Scan command

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Acceptance** | `doc-engine scan <PATH>` outputs a compliance report |

The CLI scan command.

#### FR-501: JSON flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Test |
| **Acceptance** | `doc-engine scan <PATH> --json` outputs valid JSON |

JSON output flag.

#### FR-502: Help text

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Acceptance** | `doc-engine --help` shows usage |

Help text display.

#### FR-503: Verbose mode

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Acceptance** | Verbose flag increases output detail |

Verbose output.

### 5.1 Architecture

#### NFR-100: SEA compliance

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Inspection |
| **Traces to** | SYS-01 -> saf/mod.rs |
| **Acceptance** | No upward dependencies |

Module graph follows SEA.

#### NFR-101: Single pass performance

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Analysis |
| **Traces to** | SYS-02 -> core/scanner.rs |
| **Acceptance** | Profiling shows exactly one walkdir traversal |

Single directory traversal.

#### NFR-102: Complexity bound

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Approved |
| **Verification** | Analysis |

Algorithm complexity analysis.
";

/// Fixture with mixed FR/NFR in a single domain, some missing attributes.
pub const MIXED_ATTRS_SRS: &str = "\
### 4.1 CLI Interface

#### FR-500: Scan command

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Acceptance** | CLI accepts scan subcommand |

The CLI shall accept a scan subcommand.

#### NFR-200: Synchronous execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

All operations synchronous.

#### FR-501: JSON flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |

The --json flag enables JSON output.
";

/// Fixture with 4 domains: 2 regular + 1 title-based feature-gated + 1 narrative-based feature-gated.
pub const FEATURE_GATED_FIXTURE_SRS: &str = "\
### 4.1 Rule Loading

#### FR-100: Default rules

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |

The binary embeds rules.

### 4.2 File Discovery

#### FR-200: Recursive scanning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |

Recursively discover files.

### 4.15 AI-Powered Compliance Analysis

This domain is feature-gated behind `#[cfg(feature = \"ai\")]`.

#### FR-850: AI compliance analysis

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |

AI-powered analysis.

### 5.6 Experimental Subsystem (feature-gated)

#### NFR-300: Experimental feature

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |

Experimental subsystem.
";

/// Fixture exercising the external command map.
///
/// - FR-700: Mapped in command map + backtick-heavy acceptance → tests map overrides heuristic
/// - FR-701: Mapped in command map but no acceptance backticks → tests map standalone
/// - FR-702: Not mapped, backtick acceptance → tests fallback via heuristic
/// - FR-703: Not mapped, no backticks → tests _TODO_ fallback
pub const COMMAND_MAP_FIXTURE_SRS: &str = "\
### 4.1 CLI Interface

#### FR-700: Explicit command overrides heuristic

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Acceptance** | `--verbose` flag causes `doc-engine scan --verbose` to produce detailed output |

The explicit command should override heuristic extraction.

#### FR-701: Command standalone without acceptance backticks

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Acceptance** | Scaffold generates all compliance documents |

Command present, acceptance has no backtick commands.

#### FR-702: No command with backtick acceptance (fallback)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Acceptance** | `doc-engine scan --json` outputs valid JSON |

No command map entry — falls back to heuristic backtick scanning.

#### FR-703: No command and no backticks (TODO fallback)

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Acceptance** | All checks pass |

No command map entry and no backtick commands in acceptance.
";

/// Fixture with backtick spans that are NOT commands before the actual command.
/// Exercises `find_command_span` scanning past non-command spans.
pub const MULTI_BACKTICK_FIXTURE_SRS: &str = "\
### 4.1 CLI Interface

#### FR-600: Rules flag

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Acceptance** | `--rules` flag causes `doc-engine scan --rules custom.toml` to load custom rules |

The --rules flag.

#### FR-601: AI service init

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Demonstration |
| **Acceptance** | `DefaultDocEngineAiService::new(config)` initializes, then `doc-engine ai chat hello` responds |

AI service initialization.

#### FR-602: Only non-command spans

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Acceptance** | `--verbose` and `ScanReport` are both present |

No runnable command in any span.
";
