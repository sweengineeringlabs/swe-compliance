use crate::api::types::{SrsDomain, SrsRequirement};

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

/// Strip the leading backtick command from acceptance when it duplicates steps.
///
/// If `steps` contains a backtick command (e.g. `` Run `X` ``) and `acceptance`
/// starts with `` `X` `` (the same command wrapped in backticks), strip that
/// prefix and capitalize the first letter of the remainder.
///
/// Returns `acceptance` unchanged when there is no match or steps is `_TODO_`.
fn clean_expected(acceptance: &str, steps: &str) -> String {
    if steps == "_TODO_" {
        return acceptance.to_string();
    }
    if let Some(cmd) = extract_backtick_command(steps) {
        let prefix = format!("`{}`", cmd);
        if let Some(rest) = acceptance.strip_prefix(&prefix) {
            let trimmed = rest.trim_start();
            if trimmed.is_empty() {
                return acceptance.to_string();
            }
            // Capitalize first character
            let mut chars = trimmed.chars();
            if let Some(first) = chars.next() {
                return format!("{}{}", first.to_uppercase(), chars.as_str());
            }
        }
    }
    acceptance.to_string()
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
        let steps_raw = generate_steps(req);
        let expected = clean_expected(acceptance, &steps_raw);
        let steps = escape_pipe(&steps_raw);
        out.push_str(&format!(
            "| {} | {}: {} ({}) | {} | {} |\n",
            tc_id, req.id, escape_pipe(&req.title), method, steps, escape_pipe(&expected),
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
    use crate::api::types::{ReqKind, SrsRequirement};

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
}
