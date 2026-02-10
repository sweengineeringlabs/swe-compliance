use super::types::SrsDomain;

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
            req.title,
            req.priority.as_deref().unwrap_or("Unknown"),
            req.verification.as_deref().unwrap_or("Test"),
            req.acceptance.as_deref().unwrap_or("—"),
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
                req.id, traces, desc,
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
            req.title,
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
        out.push_str(&format!(
            "| {} | {}: {} ({}) | _TODO_ | {} |\n",
            tc_id, req.id, req.title, method, acceptance,
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
            tc_id, req.id, req.title, method, req_id,
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
                    priority: Some("May".to_string()),
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
}
