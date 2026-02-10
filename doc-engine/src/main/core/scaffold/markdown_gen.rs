use super::types::{SrsDomain, SrsRequirement};

/// Escape pipe characters for markdown table cells.
fn escape_pipe(s: &str) -> String {
    s.replace('|', "\\|")
}

/// Extract the first single-backtick code span from text.
///
/// Skips double/triple backtick fences. Returns the content between the
/// first matched pair of single backticks, or `None` if no span is found.
fn extract_backtick_command(text: &str) -> Option<&str> {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == b'`' {
            // Count consecutive backticks
            let start = i;
            while i < len && bytes[i] == b'`' {
                i += 1;
            }
            let tick_count = i - start;
            if tick_count != 1 {
                // Skip double/triple backtick sequences — advance past their closing fence
                continue;
            }
            // Single backtick: find the closing one
            let content_start = i;
            while i < len && bytes[i] != b'`' {
                i += 1;
            }
            if i < len {
                // Found closing backtick
                let span = &text[content_start..i];
                i += 1; // skip closing backtick
                if !span.is_empty() {
                    return Some(span);
                }
            }
        } else {
            i += 1;
        }
    }
    None
}

/// Extract the first file path after `->` in a traces_to string.
///
/// Handles formats like:
/// - `"STK-01 -> core/rules.rs"` → `Some("core/rules.rs")`
/// - `"STK-02 -> api/types.rs (RuleDef)"` → `Some("api/types.rs")`
/// - `"STK-01 -> \`core/rules.rs\`"` → `Some("core/rules.rs")`
/// - `"SYS-01"` (no arrow) → `None`
fn extract_trace_file(traces_to: &str) -> Option<&str> {
    let arrow_pos = traces_to.find("->")?;
    let after_arrow = traces_to[arrow_pos + 2..].trim();
    if after_arrow.is_empty() {
        return None;
    }
    // Strip parenthetical suffix: "`api/types.rs` (RuleDef)" → "`api/types.rs`"
    let trimmed = if let Some(paren_pos) = after_arrow.find('(') {
        after_arrow[..paren_pos].trim()
    } else {
        after_arrow.trim()
    };
    // Strip backticks last, after parenthetical removal, so inner backticks
    // like `api/types.rs` (RuleDef) are handled correctly.
    let path = trimmed.trim_matches('`');
    if path.is_empty() { None } else { Some(path) }
}

/// Check whether a backtick span looks like a runnable CLI command.
///
/// Returns `true` when the span starts with an ASCII letter, contains at
/// least one space (i.e. has arguments — bare words like `scan` are too
/// ambiguous), and does not look like a key-value pair (`kind: brd`,
/// `project_type = "open_source"`).
fn is_command_like(span: &str) -> bool {
    let bytes = span.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    // Must start with an ASCII letter (rules out `--flag`, `.file`, `/path`, `(?regex)`)
    if !bytes[0].is_ascii_alphabetic() {
        return false;
    }
    // Must contain at least one space (command + arguments)
    if !span.contains(' ') {
        return false;
    }
    // Must not look like a key-value pair
    if span.contains(": ") || span.contains("= ") {
        return false;
    }
    true
}

/// Generate a step description for a Test-verified requirement.
fn generate_test_steps(req: &SrsRequirement) -> String {
    if let Some(ref acceptance) = req.acceptance {
        if let Some(cmd) = extract_backtick_command(acceptance) {
            if is_command_like(cmd) {
                return format!("Run `{}`", cmd);
            }
        }
    }
    "_TODO_".to_string()
}

/// Generate a step description for a Demonstration-verified requirement.
///
/// Only emits a step when a runnable command is found in acceptance.
/// Prose acceptance is never used — it already appears in the Expected column.
fn generate_demonstration_steps(req: &SrsRequirement) -> String {
    if let Some(ref acceptance) = req.acceptance {
        if let Some(cmd) = extract_backtick_command(acceptance) {
            if is_command_like(cmd) {
                return format!("Execute `{}` and observe output", cmd);
            }
        }
    }
    "_TODO_".to_string()
}

/// Generate a step description for an Inspection-verified requirement.
///
/// Uses the trace file as the reviewable artifact. Acceptance is never used
/// here — it already appears in the Expected column.
fn generate_inspection_steps(req: &SrsRequirement) -> String {
    if let Some(ref traces) = req.traces_to {
        if let Some(file) = extract_trace_file(traces) {
            return format!("Review `{}`", file);
        }
    }
    "_TODO_".to_string()
}

/// Generate a step description for an Analysis-verified requirement.
///
/// Uses the trace file (what to analyze) or the description (how to analyze).
/// Acceptance is never used — it already appears in the Expected column.
fn generate_analysis_steps(req: &SrsRequirement) -> String {
    if let Some(ref traces) = req.traces_to {
        if let Some(file) = extract_trace_file(traces) {
            return format!("Analyze `{}`", file);
        }
    }
    if !req.description.is_empty() {
        if let Some(first_line) = req.description.lines().next() {
            if !first_line.is_empty() {
                return format!("Analyze: {}", first_line);
            }
        }
    }
    "_TODO_".to_string()
}

/// Dispatch to the appropriate step generator based on verification method.
fn generate_steps(req: &SrsRequirement) -> String {
    match req.verification.as_deref().unwrap_or("Test") {
        "Test" => generate_test_steps(req),
        "Demonstration" => generate_demonstration_steps(req),
        "Inspection" => generate_inspection_steps(req),
        "Analysis" => generate_analysis_steps(req),
        _ => generate_test_steps(req),
    }
}

/// Generate a `.spec` markdown file for a domain.
pub(crate) fn generate_feature_spec_md(domain: &SrsDomain) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Feature Spec: {}\n\n", domain.title));
    out.push_str(&format!("**Version:** 1.0\n"));
    out.push_str(&format!("**Status:** Draft\n"));
    out.push_str(&format!("**Section:** {}\n\n", domain.section));
    out.push_str("## Requirements\n\n");
    out.push_str("| ID | Source | Title | Priority | Verification | Acceptance |\n");
    out.push_str("|-----|--------|-------|----------|--------------|------------|\n");

    for (idx, req) in domain.requirements.iter().enumerate() {
        let req_id = format!("REQ-{:03}", idx + 1);
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            req_id,
            req.id,
            escape_pipe(&req.title),
            req.priority.as_deref().unwrap_or("Unknown"),
            req.verification.as_deref().unwrap_or("Test"),
            escape_pipe(req.acceptance.as_deref().unwrap_or("—")),
        ));
    }

    out.push_str("\n## Acceptance Criteria\n\n");
    for (idx, req) in domain.requirements.iter().enumerate() {
        let req_id = format!("REQ-{:03}", idx + 1);
        out.push_str(&format!(
            "- **{}** ({}): {}\n",
            req_id,
            req.id,
            req.acceptance.as_deref().unwrap_or("To be defined"),
        ));
    }
    out.push('\n');
    out
}

/// Generate an `.arch` markdown file for a domain.
pub(crate) fn generate_arch_spec_md(domain: &SrsDomain) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Architecture: {}\n\n", domain.title));
    out.push_str(&format!("**Version:** 1.0\n"));
    out.push_str(&format!("**Status:** Draft\n"));
    out.push_str(&format!(
        "**Spec:** [Feature Spec](../1-requirements/{}/{}.spec)\n\n",
        domain.slug, domain.slug,
    ));
    out.push_str("## Components\n\n");
    out.push_str("| Component | Traces To | Description |\n");
    out.push_str("|-----------|-----------|-------------|\n");

    for req in &domain.requirements {
        if let Some(ref traces) = req.traces_to {
            let desc = if req.description.is_empty() {
                &req.title
            } else {
                req.description.lines().next().unwrap_or(&req.title)
            };
            out.push_str(&format!(
                "| {} handler | {} | {} |\n",
                req.id, escape_pipe(traces), escape_pipe(desc),
            ));
        }
    }

    out.push_str("\n## Related Documents\n\n");
    out.push_str(&format!(
        "- [Feature Spec](../1-requirements/{}/{}.spec)\n",
        domain.slug, domain.slug,
    ));
    out.push_str(&format!(
        "- [Test Plan](../5-testing/{}/{}.test)\n",
        domain.slug, domain.slug,
    ));
    out.push_str(&format!(
        "- [Deployment](../6-deployment/{}/{}.deploy)\n",
        domain.slug, domain.slug,
    ));
    out.push('\n');
    out
}

/// Generate a `.test` markdown file for a domain.
pub(crate) fn generate_test_spec_md(domain: &SrsDomain) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Test Plan: {}\n\n", domain.title));
    out.push_str(&format!("**Version:** 1.0\n"));
    out.push_str(&format!("**Status:** Draft\n"));
    out.push_str(&format!(
        "**Spec:** [Feature Spec](../1-requirements/{}/{}.spec)\n\n",
        domain.slug, domain.slug,
    ));
    out.push_str("## Test Cases\n\n");
    out.push_str("| ID | Test | Verifies | Priority |\n");
    out.push_str("|----|------|----------|----------|\n");

    for (idx, req) in domain.requirements.iter().enumerate() {
        let tc_id = format!("TC-{:03}", idx + 1);
        let req_id = format!("REQ-{:03}", idx + 1);
        let method = req.verification.as_deref().unwrap_or("Test");
        out.push_str(&format!(
            "| {} | {}: {} ({}) | {} | {} |\n",
            tc_id,
            req.id,
            escape_pipe(&req.title),
            method,
            req_id,
            req.priority.as_deref().unwrap_or("Unknown"),
        ));
    }
    out.push('\n');
    out
}

/// Generate a `.manual.exec` markdown file for a domain.
///
/// Lists every test case (aligned with `.test` and `.auto.exec`) with
/// columns a human tester can fill in: Steps, Expected, Tester, Date,
/// Pass/Fail, Notes.
pub(crate) fn generate_manual_exec_md(domain: &SrsDomain) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Manual Test Execution: {}\n\n", domain.title));
    out.push_str(&format!(
        "> **TLDR:** Manual test checklist for {} — step-by-step procedures with expected outcomes.\n\n",
        domain.title,
    ));
    out.push_str("**Version:** 1.0\n");
    out.push_str("**Status:** Pending\n");
    out.push_str(&format!(
        "**Test Plan:** [Test Plan]({}.test)\n\n",
        domain.slug,
    ));
    out.push_str("---\n\n");
    out.push_str("## Test Cases\n\n");
    out.push_str("| TC | Test | Steps | Expected |\n");
    out.push_str("|----|------|-------|----------|\n");

    for (idx, req) in domain.requirements.iter().enumerate() {
        let tc_id = format!("TC-{:03}", idx + 1);
        let method = req.verification.as_deref().unwrap_or("Test");
        let acceptance = req.acceptance.as_deref().unwrap_or("To be defined");
        let steps = escape_pipe(&generate_steps(req));
        out.push_str(&format!(
            "| {} | {}: {} ({}) | {} | {} |\n",
            tc_id, req.id, escape_pipe(&req.title), method, steps, escape_pipe(acceptance),
        ));
    }

    out.push_str("\n---\n\n");
    out.push_str("## Execution Log\n\n");
    out.push_str("| TC | Tester | Date | Pass/Fail | Notes |\n");
    out.push_str("|----|--------|------|-----------|-------|\n");

    for (idx, _req) in domain.requirements.iter().enumerate() {
        let tc_id = format!("TC-{:03}", idx + 1);
        out.push_str(&format!(
            "| {} | | | | |\n",
            tc_id,
        ));
    }

    out.push('\n');
    out
}

/// Generate an `.auto.exec` markdown file for a domain.
///
/// Lists every test case (aligned with `.test` and `.manual.exec`) with
/// columns for CI execution tracking: Verifies, CI Job, Build, Status,
/// Last Run.
pub(crate) fn generate_auto_exec_md(domain: &SrsDomain) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Automated Test Execution: {}\n\n", domain.title));
    out.push_str(&format!(
        "> **TLDR:** CI/automated test tracker for {} — maps each test case to a CI job and build.\n\n",
        domain.title,
    ));
    out.push_str("**Version:** 1.0\n");
    out.push_str("**Status:** Pending\n");
    out.push_str(&format!(
        "**Test Plan:** [Test Plan]({}.test)\n\n",
        domain.slug,
    ));
    out.push_str("---\n\n");
    out.push_str("## Test Cases\n\n");
    out.push_str("| TC | Test | Verifies | CI Job | Build | Status | Last Run |\n");
    out.push_str("|----|------|----------|--------|-------|--------|----------|\n");

    for (idx, req) in domain.requirements.iter().enumerate() {
        let tc_id = format!("TC-{:03}", idx + 1);
        let req_id = format!("REQ-{:03}", idx + 1);
        let method = req.verification.as_deref().unwrap_or("Test");
        out.push_str(&format!(
            "| {} | {}: {} ({}) | {} | | | Pending | |\n",
            tc_id, req.id, escape_pipe(&req.title), method, req_id,
        ));
    }

    out.push('\n');
    out
}

/// Generate a `.deploy` markdown file for a domain.
pub(crate) fn generate_deploy_spec_md(domain: &SrsDomain) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Deployment: {}\n\n", domain.title));
    out.push_str(&format!("**Version:** 1.0\n"));
    out.push_str(&format!("**Status:** Draft\n"));
    out.push_str(&format!(
        "**Spec:** [Feature Spec](../1-requirements/{}/{}.spec)\n\n",
        domain.slug, domain.slug,
    ));
    out.push_str("## Environments\n\n");
    out.push_str("| Environment | Description |\n");
    out.push_str("|-------------|-------------|\n");
    out.push_str(&format!(
        "| staging | Staging environment for {} validation |\n",
        domain.title,
    ));
    out.push_str(&format!(
        "| production | Production environment for {} |\n",
        domain.title,
    ));

    out.push_str("\n## Build\n\n");
    out.push_str("_TODO: Define build steps._\n\n");
    out.push_str("## Rollback\n\n");
    out.push_str("_TODO: Define rollback procedures._\n");
    out
}

/// Generate a `brd.spec` markdown file covering all domains.
pub(crate) fn generate_brd_md(domains: &[SrsDomain]) -> String {
    let mut out = String::new();
    out.push_str("# Business Requirements Document\n\n");
    out.push_str("**Version:** 1.0\n");
    out.push_str("**Status:** Draft\n\n");
    out.push_str("## Domain Inventory\n\n");
    out.push_str("| Section | Domain | Requirements | Spec | Arch | Test | Deploy |\n");
    out.push_str("|---------|--------|-------------|------|------|------|--------|\n");

    for d in domains {
        out.push_str(&format!(
            "| {} | {} | {} | [spec]({1}/{1}.spec) | [arch](../../3-design/{1}/{1}.arch) | [test](../../5-testing/{1}/{1}.test) | [deploy](../../6-deployment/{1}/{1}.deploy) |\n",
            d.section, d.slug, d.requirements.len(),
        ));
    }

    out.push_str("\n## Domain Specifications\n\n");
    for d in domains {
        out.push_str(&format!(
            "### {} {} ({})\n\n",
            d.section, d.title, d.slug,
        ));
        out.push_str(&format!("- **Requirements:** {}\n", d.requirements.len()));
        out.push_str(&format!(
            "- **Spec:** `docs/1-requirements/{slug}/{slug}.spec.yaml`\n",
            slug = d.slug,
        ));
        out.push_str(&format!(
            "- **Architecture:** `docs/3-design/{slug}/{slug}.arch.yaml`\n",
            slug = d.slug,
        ));
        out.push_str(&format!(
            "- **Test Plan:** `docs/5-testing/{slug}/{slug}.test.yaml`\n",
            slug = d.slug,
        ));
        out.push_str(&format!(
            "- **Deployment:** `docs/6-deployment/{slug}/{slug}.deploy.yaml`\n\n",
            slug = d.slug,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scaffold::types::{ReqKind, SrsRequirement};

    fn sample_domain() -> SrsDomain {
        SrsDomain {
            section: "4.1".to_string(),
            title: "Rule Loading".to_string(),
            slug: "rule_loading".to_string(),
            requirements: vec![SrsRequirement {
                id: "FR-100".to_string(),
                title: "Default rules".to_string(),
                kind: ReqKind::Functional,
                priority: Some("Must".to_string()),
                state: Some("Approved".to_string()),
                verification: Some("Test".to_string()),
                traces_to: Some("STK-01".to_string()),
                acceptance: Some("Engine loads embedded rules".to_string()),
                description: "Embed rules in binary.".to_string(),
            }],
        }
    }

    #[test]
    fn test_feature_spec_md_has_metadata() {
        let md = generate_feature_spec_md(&sample_domain());
        assert!(md.contains("**Version:** 1.0"));
        assert!(md.contains("**Status:** Draft"));
        assert!(md.contains("# Feature Spec: Rule Loading"));
    }

    #[test]
    fn test_feature_spec_md_has_table() {
        let md = generate_feature_spec_md(&sample_domain());
        assert!(md.contains("| REQ-001 |"));
        assert!(md.contains("FR-100"));
        assert!(md.contains("Must"));
    }

    #[test]
    fn test_arch_spec_md_has_links() {
        let md = generate_arch_spec_md(&sample_domain());
        assert!(md.contains("**Spec:**"));
        assert!(md.contains("rule_loading.spec"));
        assert!(md.contains("## Related Documents"));
    }

    #[test]
    fn test_test_spec_md_has_table() {
        let md = generate_test_spec_md(&sample_domain());
        assert!(md.contains("# Test Plan: Rule Loading"));
        assert!(md.contains("| TC-001 |"));
        assert!(md.contains("| REQ-001 |"));
    }

    #[test]
    fn test_brd_md_has_inventory() {
        let domains = vec![sample_domain()];
        let md = generate_brd_md(&domains);
        assert!(md.contains("# Business Requirements Document"));
        assert!(md.contains("| 4.1 |"));
        assert!(md.contains("rule_loading"));
    }

    fn mixed_domain() -> SrsDomain {
        SrsDomain {
            section: "4.2".to_string(),
            title: "CLI Interface".to_string(),
            slug: "cli_interface".to_string(),
            requirements: vec![
                SrsRequirement {
                    id: "FR-500".to_string(),
                    title: "Scan command".to_string(),
                    kind: ReqKind::Functional,
                    priority: Some("Must".to_string()),
                    state: Some("Approved".to_string()),
                    verification: Some("Demonstration".to_string()),
                    traces_to: None,
                    acceptance: Some("CLI scans directory".to_string()),
                    description: String::new(),
                },
                SrsRequirement {
                    id: "FR-501".to_string(),
                    title: "JSON flag".to_string(),
                    kind: ReqKind::Functional,
                    priority: Some("Should".to_string()),
                    state: Some("Approved".to_string()),
                    verification: Some("Test".to_string()),
                    traces_to: None,
                    acceptance: Some("--json outputs JSON".to_string()),
                    description: String::new(),
                },
                SrsRequirement {
                    id: "FR-502".to_string(),
                    title: "Verbose mode".to_string(),
                    kind: ReqKind::Functional,
                    priority: Some("Could".to_string()),
                    state: Some("Approved".to_string()),
                    verification: Some("Inspection".to_string()),
                    traces_to: None,
                    acceptance: None,
                    description: String::new(),
                },
            ],
        }
    }

    #[test]
    fn test_manual_exec_md_structure() {
        let md = generate_manual_exec_md(&mixed_domain());
        assert!(md.contains("# Manual Test Execution: CLI Interface"));
        assert!(md.contains("**Status:** Pending"));
        assert!(md.contains("**Test Plan:** [Test Plan](cli_interface.test)"));
        assert!(md.contains("> **TLDR:**"));
        // Test Cases table — actionable columns
        assert!(md.contains("| TC | Test | Steps | Expected |"));
        // All requirements present (no filtering)
        assert!(md.contains("| TC-001 |"));
        assert!(md.contains("FR-500"));
        assert!(md.contains("(Demonstration)"));
        assert!(md.contains("| TC-002 |"));
        assert!(md.contains("FR-501"));
        assert!(md.contains("| TC-003 |"));
        assert!(md.contains("FR-502"));
        assert!(md.contains("(Inspection)"));
        // Expected column uses acceptance text
        assert!(md.contains("CLI scans directory"));
        assert!(md.contains("--json outputs JSON"));
        // Execution Log table
        assert!(md.contains("| Tester | Date | Pass/Fail | Notes |"));
    }

    #[test]
    fn test_auto_exec_md_structure() {
        let md = generate_auto_exec_md(&mixed_domain());
        assert!(md.contains("# Automated Test Execution: CLI Interface"));
        assert!(md.contains("**Status:** Pending"));
        assert!(md.contains("**Test Plan:** [Test Plan](cli_interface.test)"));
        assert!(md.contains("> **TLDR:**"));
        // Table columns
        assert!(md.contains("| CI Job | Build | Status | Last Run |"));
        // All requirements present (no filtering)
        assert!(md.contains("| TC-001 |"));
        assert!(md.contains("FR-500"));
        assert!(md.contains("| TC-002 |"));
        assert!(md.contains("FR-501"));
        assert!(md.contains("| TC-003 |"));
        assert!(md.contains("FR-502"));
    }

    #[test]
    fn test_manual_and_auto_exec_aligned_tc_ids() {
        let manual = generate_manual_exec_md(&mixed_domain());
        let auto = generate_auto_exec_md(&mixed_domain());
        // Both files have all 3 TCs — aligned row-for-row
        for tc in &["TC-001", "TC-002", "TC-003"] {
            assert!(manual.contains(tc), "manual.exec missing {}", tc);
            assert!(auto.contains(tc), "auto.exec missing {}", tc);
        }
    }

    #[test]
    fn test_brd_md_spec_links_use_slug_not_section() {
        let domains = vec![
            sample_domain(),
            SrsDomain {
                section: "5.2".to_string(),
                title: "Performance".to_string(),
                slug: "performance".to_string(),
                requirements: vec![SrsRequirement {
                    id: "NFR-100".to_string(),
                    title: "Latency target".to_string(),
                    kind: ReqKind::NonFunctional,
                    priority: Some("Should".to_string()),
                    state: Some("Approved".to_string()),
                    verification: Some("Test".to_string()),
                    traces_to: None,
                    acceptance: Some("p99 < 200ms".to_string()),
                    description: String::new(),
                }],
            },
        ];
        let md = generate_brd_md(&domains);

        // Spec links must use slug as directory, not section number
        assert!(md.contains("[spec](rule_loading/rule_loading.spec)"));
        assert!(md.contains("[spec](performance/performance.spec)"));

        // Must NOT contain section-number-based paths
        assert!(!md.contains("[spec](4.1/"));
        assert!(!md.contains("[spec](5.2/"));
    }

    fn domain_with_pipe_in_acceptance() -> SrsDomain {
        SrsDomain {
            section: "4.13".to_string(),
            title: "Behavioral".to_string(),
            slug: "behavioral".to_string(),
            requirements: vec![SrsRequirement {
                id: "FR-801".to_string(),
                title: "W3H detection scope".to_string(),
                kind: ReqKind::Functional,
                priority: Some("Must".to_string()),
                state: Some("Proposed".to_string()),
                verification: Some("Test".to_string()),
                traces_to: Some("BL-03".to_string()),
                acceptance: Some("Regex `(?i)^##\\s*(what|why|how)` matches headings.".to_string()),
                description: String::new(),
            }],
        }
    }

    #[test]
    fn test_pipe_in_acceptance_escaped_in_manual_exec() {
        let md = generate_manual_exec_md(&domain_with_pipe_in_acceptance());
        // The pipe inside the regex must be escaped so the table row stays intact
        assert!(md.contains("what\\|why\\|how"));
        // Row must have exactly 4 columns (TC, Test, Steps, Expected)
        let row = md.lines().find(|l| l.contains("TC-001")).unwrap();
        // Count unescaped pipes (column delimiters) — should be 5 for 4 columns
        let unescaped = row.matches('|').count() - row.matches("\\|").count();
        assert_eq!(unescaped, 5, "table row has wrong column count: {}", row);
    }

    #[test]
    fn test_pipe_in_acceptance_escaped_in_feature_spec() {
        let md = generate_feature_spec_md(&domain_with_pipe_in_acceptance());
        assert!(md.contains("what\\|why\\|how"));
    }

    #[test]
    fn test_pipe_in_title_escaped_in_test_spec() {
        let mut domain = domain_with_pipe_in_acceptance();
        domain.requirements[0].title = "Check A|B".to_string();
        let md = generate_test_spec_md(&domain);
        assert!(md.contains("Check A\\|B"));
    }

    // ---- extract_backtick_command tests ----

    #[test]
    fn test_extract_backtick_command_basic() {
        assert_eq!(
            extract_backtick_command("`doc-engine scan .`"),
            Some("doc-engine scan ."),
        );
    }

    #[test]
    fn test_extract_backtick_command_no_backticks() {
        assert_eq!(extract_backtick_command("plain text"), None);
    }

    #[test]
    fn test_extract_backtick_command_skips_double_backticks() {
        assert_eq!(
            extract_backtick_command("``code block`` then `real`"),
            Some("real"),
        );
    }

    #[test]
    fn test_extract_backtick_command_first_match() {
        assert_eq!(
            extract_backtick_command("`first` and `second`"),
            Some("first"),
        );
    }

    #[test]
    fn test_extract_backtick_command_empty_span() {
        // Empty backtick span `` should be skipped (treated as double backtick)
        assert_eq!(extract_backtick_command("`` `real`"), Some("real"));
    }

    // ---- is_command_like tests ----

    #[test]
    fn test_is_command_like_real_command() {
        assert!(is_command_like("doc-engine scan ."));
        assert!(is_command_like("cargo test --release"));
        assert!(is_command_like("echo hello world"));
    }

    #[test]
    fn test_is_command_like_rejects_bare_flag() {
        assert!(!is_command_like("--json"));
        assert!(!is_command_like("--rules"));
        assert!(!is_command_like("--output <path>"));
        assert!(!is_command_like("--checks 1-13"));
    }

    #[test]
    fn test_is_command_like_rejects_bare_word() {
        assert!(!is_command_like("scan"));
        assert!(!is_command_like("rules.toml"));
        assert!(!is_command_like(".git/"));
    }

    #[test]
    fn test_is_command_like_rejects_key_value() {
        assert!(!is_command_like("kind: brd"));
        assert!(!is_command_like("project_type = \"open_source\""));
        assert!(!is_command_like("handler = \"nonexistent\""));
    }

    #[test]
    fn test_is_command_like_rejects_regex_and_special() {
        assert!(!is_command_like("(?i)^##\\s*(what|why|how)"));
        assert!(!is_command_like("[[rules]]"));
        assert!(!is_command_like("/"));
    }

    #[test]
    fn test_is_command_like_empty() {
        assert!(!is_command_like(""));
    }

    // ---- extract_trace_file tests ----

    #[test]
    fn test_extract_trace_file_basic() {
        assert_eq!(
            extract_trace_file("STK-01 -> core/rules.rs"),
            Some("core/rules.rs"),
        );
    }

    #[test]
    fn test_extract_trace_file_with_parenthetical() {
        assert_eq!(
            extract_trace_file("STK-02 -> api/types.rs (RuleDef)"),
            Some("api/types.rs"),
        );
    }

    #[test]
    fn test_extract_trace_file_no_arrow() {
        assert_eq!(extract_trace_file("SYS-01"), None);
    }

    #[test]
    fn test_extract_trace_file_with_backticks() {
        assert_eq!(
            extract_trace_file("STK-03 -> `saf/mod.rs`"),
            Some("saf/mod.rs"),
        );
    }

    #[test]
    fn test_extract_trace_file_backticks_with_parenthetical() {
        assert_eq!(
            extract_trace_file("STK-02 -> `api/types.rs` (RuleDef, RuleType)"),
            Some("api/types.rs"),
        );
    }

    // ---- generate_steps: Test method ----

    #[test]
    fn test_generate_steps_test_with_backtick_command() {
        let req = SrsRequirement {
            id: "FR-501".to_string(),
            title: "JSON flag".to_string(),
            kind: ReqKind::Functional,
            priority: Some("Should".to_string()),
            state: Some("Approved".to_string()),
            verification: Some("Test".to_string()),
            traces_to: None,
            acceptance: Some("`doc-engine scan <PATH> --json` outputs valid JSON".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "Run `doc-engine scan <PATH> --json`");
    }

    #[test]
    fn test_generate_steps_test_no_backtick_fallback() {
        let req = SrsRequirement {
            id: "FR-502".to_string(),
            title: "Plain test".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Test".to_string()),
            traces_to: None,
            acceptance: Some("Output is correct".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_test_no_acceptance() {
        let req = SrsRequirement {
            id: "FR-503".to_string(),
            title: "No data".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Test".to_string()),
            traces_to: None,
            acceptance: None,
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_test_rejects_flag_backtick() {
        let req = SrsRequirement {
            id: "FR-504".to_string(),
            title: "Flag only".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Test".to_string()),
            traces_to: None,
            acceptance: Some("`--json` outputs valid JSON".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_test_rejects_filename_backtick() {
        let req = SrsRequirement {
            id: "FR-505".to_string(),
            title: "Filename only".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Test".to_string()),
            traces_to: None,
            acceptance: Some("`rules.toml` contains 128 rules".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_test_rejects_key_value_backtick() {
        let req = SrsRequirement {
            id: "FR-506".to_string(),
            title: "Key-value".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Test".to_string()),
            traces_to: None,
            acceptance: Some("`kind: brd` deserializes correctly".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    // ---- generate_steps: Demonstration method ----

    #[test]
    fn test_generate_steps_demonstration_with_backtick() {
        let req = SrsRequirement {
            id: "FR-600".to_string(),
            title: "Demo cmd".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Demonstration".to_string()),
            traces_to: None,
            acceptance: Some("`cargo run -- help` shows usage".to_string()),
            description: String::new(),
        };
        assert_eq!(
            generate_steps(&req),
            "Execute `cargo run -- help` and observe output",
        );
    }

    #[test]
    fn test_generate_steps_demonstration_prose_no_command_is_todo() {
        let req = SrsRequirement {
            id: "FR-601".to_string(),
            title: "Demo prose".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Demonstration".to_string()),
            traces_to: None,
            acceptance: Some("CLI scans directory".to_string()),
            description: String::new(),
        };
        // Prose acceptance is never used for Steps — it already appears in Expected
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_demonstration_no_data() {
        let req = SrsRequirement {
            id: "FR-602".to_string(),
            title: "Demo empty".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Demonstration".to_string()),
            traces_to: None,
            acceptance: None,
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_demonstration_non_command_backtick_is_todo() {
        let req = SrsRequirement {
            id: "FR-603".to_string(),
            title: "Demo with filename backtick".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: Some("Demonstration".to_string()),
            traces_to: None,
            acceptance: Some("`login.spec.yaml` generates a report".to_string()),
            description: String::new(),
        };
        // Non-command backtick span is rejected, no prose fallback
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    // ---- generate_steps: Inspection method ----

    #[test]
    fn test_generate_steps_inspection_with_trace_file() {
        let req = SrsRequirement {
            id: "NFR-100".to_string(),
            title: "SEA compliance".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Inspection".to_string()),
            traces_to: Some("STK-01 -> saf/mod.rs".to_string()),
            acceptance: Some("No upward dependencies".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "Review `saf/mod.rs`");
    }

    #[test]
    fn test_generate_steps_inspection_no_trace_file_is_todo() {
        let req = SrsRequirement {
            id: "NFR-101".to_string(),
            title: "Inspect prose".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Inspection".to_string()),
            traces_to: Some("SYS-01".to_string()), // no arrow → no file
            acceptance: Some("Module boundaries respected".to_string()),
            description: String::new(),
        };
        // Acceptance is never used for Steps — it already appears in Expected
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_inspection_no_data() {
        let req = SrsRequirement {
            id: "NFR-102".to_string(),
            title: "Inspect empty".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Inspection".to_string()),
            traces_to: None,
            acceptance: None,
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    // ---- generate_steps: Analysis method ----

    #[test]
    fn test_generate_steps_analysis_acceptance_only_is_todo() {
        let req = SrsRequirement {
            id: "NFR-200".to_string(),
            title: "Single pass".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Analysis".to_string()),
            traces_to: None,
            acceptance: Some("Profiling shows exactly one walkdir traversal".to_string()),
            description: String::new(),
        };
        // Acceptance is never used for Steps — it already appears in Expected
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    #[test]
    fn test_generate_steps_analysis_with_trace_file() {
        let req = SrsRequirement {
            id: "NFR-200".to_string(),
            title: "Single pass".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Analysis".to_string()),
            traces_to: Some("SYS-02 -> core/scanner.rs".to_string()),
            acceptance: Some("Profiling shows exactly one walkdir traversal".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "Analyze `core/scanner.rs`");
    }

    #[test]
    fn test_generate_steps_analysis_description_fallback() {
        let req = SrsRequirement {
            id: "NFR-201".to_string(),
            title: "Analyze desc".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Analysis".to_string()),
            traces_to: None,
            acceptance: None,
            description: "Complexity must be O(n).".to_string(),
        };
        assert_eq!(
            generate_steps(&req),
            "Analyze: Complexity must be O(n).",
        );
    }

    #[test]
    fn test_generate_steps_analysis_no_data() {
        let req = SrsRequirement {
            id: "NFR-202".to_string(),
            title: "Analyze empty".to_string(),
            kind: ReqKind::NonFunctional,
            priority: None,
            state: None,
            verification: Some("Analysis".to_string()),
            traces_to: None,
            acceptance: None,
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "_TODO_");
    }

    // ---- generate_steps: default (no verification) ----

    #[test]
    fn test_generate_steps_default_uses_test() {
        let req = SrsRequirement {
            id: "FR-999".to_string(),
            title: "No method".to_string(),
            kind: ReqKind::Functional,
            priority: None,
            state: None,
            verification: None,
            traces_to: None,
            acceptance: Some("`cargo test` passes".to_string()),
            description: String::new(),
        };
        assert_eq!(generate_steps(&req), "Run `cargo test`");
    }

    // ---- Integration tests ----

    #[test]
    fn test_manual_exec_all_four_methods() {
        let domain = SrsDomain {
            section: "4.5".to_string(),
            title: "Mixed Methods".to_string(),
            slug: "mixed_methods".to_string(),
            requirements: vec![
                SrsRequirement {
                    id: "FR-001".to_string(),
                    title: "Test method".to_string(),
                    kind: ReqKind::Functional,
                    priority: None,
                    state: None,
                    verification: Some("Test".to_string()),
                    traces_to: None,
                    acceptance: Some("`cargo test` passes".to_string()),
                    description: String::new(),
                },
                SrsRequirement {
                    id: "FR-002".to_string(),
                    title: "Demo method".to_string(),
                    kind: ReqKind::Functional,
                    priority: None,
                    state: None,
                    verification: Some("Demonstration".to_string()),
                    traces_to: None,
                    acceptance: Some("`cargo run` shows help".to_string()),
                    description: String::new(),
                },
                SrsRequirement {
                    id: "NFR-003".to_string(),
                    title: "Inspect method".to_string(),
                    kind: ReqKind::NonFunctional,
                    priority: None,
                    state: None,
                    verification: Some("Inspection".to_string()),
                    traces_to: Some("STK-01 -> src/lib.rs".to_string()),
                    acceptance: None,
                    description: String::new(),
                },
                SrsRequirement {
                    id: "NFR-004".to_string(),
                    title: "Analyze method".to_string(),
                    kind: ReqKind::NonFunctional,
                    priority: None,
                    state: None,
                    verification: Some("Analysis".to_string()),
                    traces_to: Some("SYS-02 -> core/scanner.rs".to_string()),
                    acceptance: Some("O(n) complexity".to_string()),
                    description: String::new(),
                },
            ],
        };
        let md = generate_manual_exec_md(&domain);
        assert!(md.contains("Run `cargo test`"), "Test step missing");
        assert!(
            md.contains("Execute `cargo run` and observe output"),
            "Demonstration step missing",
        );
        assert!(md.contains("Review `src/lib.rs`"), "Inspection step missing");
        assert!(
            md.contains("Analyze `core/scanner.rs`"),
            "Analysis step missing",
        );
    }

    #[test]
    fn test_manual_exec_pipe_in_steps_escaped() {
        let domain = SrsDomain {
            section: "4.6".to_string(),
            title: "Pipe Steps".to_string(),
            slug: "pipe_steps".to_string(),
            requirements: vec![SrsRequirement {
                id: "FR-900".to_string(),
                title: "Piped cmd".to_string(),
                kind: ReqKind::Functional,
                priority: None,
                state: None,
                verification: Some("Test".to_string()),
                traces_to: None,
                acceptance: Some("`echo a | grep a` succeeds".to_string()),
                description: String::new(),
            }],
        };
        let md = generate_manual_exec_md(&domain);
        // The pipe in the command must be escaped in the table
        assert!(md.contains("Run `echo a \\| grep a`"));
    }

    #[test]
    fn test_manual_exec_todo_fallback_when_no_data() {
        let domain = SrsDomain {
            section: "4.7".to_string(),
            title: "Fallback".to_string(),
            slug: "fallback".to_string(),
            requirements: vec![SrsRequirement {
                id: "FR-000".to_string(),
                title: "Empty req".to_string(),
                kind: ReqKind::Functional,
                priority: None,
                state: None,
                verification: Some("Test".to_string()),
                traces_to: None,
                acceptance: None,
                description: String::new(),
            }],
        };
        let md = generate_manual_exec_md(&domain);
        assert!(md.contains("_TODO_"));
    }
}
