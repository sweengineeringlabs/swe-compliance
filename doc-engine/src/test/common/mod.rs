use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Create a minimal project structure that passes core checks.
pub fn create_minimal_project() -> TempDir {
    let tmp = tempfile::Builder::new().prefix("test_").tempdir().unwrap();
    let root = tmp.path();

    // Root files
    write_file(root, "README.md", "# Project\n\nSee [docs](docs/README.md) for details.\n");
    write_file(root, "CONTRIBUTING.md", "# Contributing\n");
    write_file(root, "CHANGELOG.md", "# Changelog\n");
    write_file(root, "SECURITY.md", "# Security\n");
    write_file(root, "LICENSE", "MIT License\n");
    write_file(root, ".gitignore", "target/\n");
    write_file(root, ".editorconfig", "root = true\n");
    write_file(root, "CODE_OF_CONDUCT.md", "# Code of Conduct\n");
    write_file(root, "SUPPORT.md", "# Support\n");

    // .github templates
    fs::create_dir_all(root.join(".github/ISSUE_TEMPLATE")).unwrap();
    write_file(root, ".github/PULL_REQUEST_TEMPLATE.md", "## PR Template\n");

    // docs/ structure
    write_file(root, "docs/README.md",
        "# Hub\n\n**Audience**: All\n\n## Who\nTeam\n## What\nProduct\n## Why\nReason\n## How\nProcess\n\n- [0-ideation](0-ideation/)\n- [1-requirements](1-requirements/)\n- [2-planning](2-planning/)\n- [3-design](3-design/)\n- [4-development](4-development/)\n- [5-testing](5-testing/)\n- [6-deployment](6-deployment/)\n- [7-operations](7-operations/)\n"
    );
    write_file(root, "docs/glossary.md",
        "# Glossary\n\n**Audience**: All\n\n**API** - Application Programming Interface\n**CLI** - Command Line Interface\n**SDK** - Software Development Kit\n"
    );

    // Phase directories
    for phase in &["0-ideation", "1-requirements", "2-planning", "3-design", "4-development", "5-testing", "6-deployment", "7-operations"] {
        let dir = format!("docs/{}", phase);
        write_file(root, &format!("{}/README.md", dir),
            &format!("# {}\n\n**Audience**: Developers\n", phase));
    }

    // Testing strategy for check 91 (29119-3 sections)
    write_file(root, "docs/5-testing/testing_strategy.md",
        "# Testing Strategy\n\n**Audience**: Developers\n\n\
         ## Test Strategy\nRequirements-based testing approach.\n\n\
         ## Test Categories\nUnit, integration, and E2E tests.\n\n\
         ## Coverage Targets\n80% line coverage target.\n");

    // Testing phase artifacts (checks 99-102)
    write_file(root, "docs/5-testing/test_plan.md",
        "# Test Plan\n\n**Audience**: Developers\n\nLevel-specific test planning.\n");
    write_file(root, "docs/5-testing/test_design.md",
        "# Test Design Specification\n\n**Audience**: Developers\n\nTest design approach.\n");
    write_file(root, "docs/5-testing/test_cases.md",
        "# Test Case Specification\n\n**Audience**: Developers\n\nTest case definitions.\n");
    write_file(root, "docs/5-testing/verification_report.md",
        "# Verification Report\n\n**Audience**: Developers\n\nVerification and validation results.\n");

    // Traceability artifacts (checks 51-53) + 29148 attributes (check 89)
    write_file(root, "docs/1-requirements/srs.md",
        "# Requirements\n\n**Audience**: Developers\n\n\
         #### FR-001: Sample requirement\n\n\
         | Attribute | Value |\n\
         |-----------|-------|\n\
         | **Priority** | Must |\n\
         | **State** | Approved |\n\
         | **Verification** | Test |\n\
         | **Traces to** | STK-01 |\n\
         | **Acceptance** | System meets criteria |\n\n\
         The system shall do the thing.\n");
    write_file(root, "docs/2-planning/implementation_plan.md",
        "# Implementation Plan\n\n**Audience**: Developers\n\nSee architecture.md for the design.\n");

    // Compliance checklist
    let checklist_content: String = (1..=20)
        .map(|i| format!("- [x] Rule {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    write_file(root, "docs/3-design/compliance/compliance_checklist.md",
        &format!("# Compliance Checklist\n\n**Audience**: All\n\n{}\n\nSee [architecture](../architecture.md)\n", checklist_content));

    // architecture.md for the checklist reference (also references requirements for check 52)
    // W3H sections for check 74; W3H also satisfies 42010 (Who=stakeholders, Why=concerns, What+How=viewpoints)
    write_file(root, "docs/3-design/architecture.md",
        "# Architecture\n\n**Audience**: Developers\n\n## Who\nStakeholders: developers, architects.\n\n## What\nSystem architecture.\n\n## Why\nDesign rationale and concerns.\n\n## How\nComponent design.\n\nSee requirements.md for FR-001.\n");

    // Design phase artifacts (checks 107-108)
    write_file(root, "docs/3-design/design_description.md",
        "# Design Description\n\n**Audience**: Developers\n\nDetailed design of system components.\n\nSee srs.md for FR-001.\n");
    write_file(root, "docs/3-design/interface_description.md",
        "# Interface Description\n\n**Audience**: Developers\n\nSystem interface specifications.\n\nSee srs.md for FR-001.\n");

    // developer_guide.md for check 69 + W3H for check 74 + 26514 sections for check 94
    write_file(root, "docs/4-development/developer_guide.md",
        "# Developer Guide\n\n**Audience**: Developers\n\n\
         ## What\nDevelopment guide.\n\n\
         ## Why\nOnboarding.\n\n\
         ## How\n\n\
         ### Build & Test\nRun `cargo build` and `cargo test`.\n\n\
         ### Project Structure\nSee src/ for code layout.\n\n\
         ### Adding New Features\nExtend the codebase.\n");

    // Development phase artifacts (checks 103-106)
    write_file(root, "docs/4-development/integration_plan.md",
        "# Integration Plan\n\n**Audience**: Developers\n\nComponent integration strategy.\n");
    write_file(root, "docs/4-development/user_documentation.md",
        "# User Documentation\n\n**Audience**: Users\n\nEnd-user documentation.\n");
    write_file(root, "docs/4-development/api_documentation.md",
        "# API Documentation\n\n**Audience**: Developers\n\nAPI reference.\n");
    write_file(root, "docs/4-development/build_procedures.md",
        "# Build Procedures\n\n**Audience**: Developers\n\nBuild and packaging steps.\n");

    // Setup guide for check 58
    write_file(root, "docs/4-development/setup_guide.md",
        "# Setup Guide\n\n**Audience**: Developers\n\nHow to set up the development environment.\n");

    // backlog.md for check 71 (in 2-planning/); references architecture for check 53;
    // references requirements for check 82; backlog sections for check 95
    write_file(root, "docs/2-planning/backlog.md",
        "# Backlog\n\n**Audience**: Developers\n\n\
         See architecture.md for design context. Derived from requirements.md.\n\n\
         ## Backlog Items\n\n\
         ### High Priority\n\n- [ ] Initial implementation\n\n\
         ## Completed\n\n- [x] Project setup — 2026-01-01\n\n\
         ## Blockers\n\n| Blocker | Impact | Owner | Status |\n\
         |---------|--------|-------|--------|\n\
         | None | — | — | — |\n");

    // Planning phase artifacts (checks 109-113); each references architecture for check 53
    write_file(root, "docs/2-planning/project_management_plan.md",
        "# Project Management Plan\n\n**Audience**: Developers\n\nProject management approach.\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/configuration_management_plan.md",
        "# Configuration Management Plan\n\n**Audience**: Developers\n\nConfiguration management approach.\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/risk_management_plan.md",
        "# Risk Management Plan\n\n**Audience**: Developers\n\nRisk management approach.\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/verification_plan.md",
        "# Verification Plan\n\n**Audience**: Developers\n\nVerification approach.\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/test_plan.md",
        "# Test Plan\n\n**Audience**: Developers\n\nProject-level test planning.\n\nSee architecture.md for system context.\n");

    // Planning phase artifacts (checks 83-88); each references architecture for check 53
    write_file(root, "docs/2-planning/risk_register.md",
        "# Risk Register\n\n**Audience**: Developers\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/estimation.md",
        "# Estimation\n\n**Audience**: Developers\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/schedule.md",
        "# Schedule\n\n**Audience**: Developers\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/resource_plan.md",
        "# Resource Plan\n\n**Audience**: Developers\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/communication_plan.md",
        "# Communication Plan\n\n**Audience**: Developers\n\nSee architecture.md for system context.\n");
    write_file(root, "docs/2-planning/quality_plan.md",
        "# Quality Plan\n\n**Audience**: Developers\n\nSee architecture.md for system context.\n");

    // templates directory for checks 72-73
    write_file(root, "docs/templates/check_template.md",
        "# Check Template\n\n**Audience**: Developers\n");

    // Production readiness for checks 92-98 (25010 + 12207 + 25010 supp + 25040 sections)
    write_file(root, "docs/6-deployment/production_readiness.md",
        "# Production Readiness Review\n\n**Audience**: Developers\n\n\
         ## Verdict: READY\n\n\
         | Area | Status |\n|------|--------|\n\
         | CI/CD Pipeline | PASS |\n| Dependency Health | PASS |\n\
         | Static Analysis | PASS |\n| Dependency Auditing | PASS |\n\
         | API Documentation | PASS |\n| Runtime Safety | PASS |\n\
         | Package Metadata | PASS |\n| README & Onboarding | PASS |\n\
         | Release Automation | PASS |\n| Documentation Lint | PASS |\n\
         | Security | PASS |\n| Test Coverage | PASS |\n\
         | Observability | PASS |\n| Backwards Compatibility | PASS |\n\n\
         ## 1. CI/CD Pipeline\nPipeline runs on every push.\n\n\
         ## 2. Dependency Health\nAll deps maintained.\n\n\
         ## 3. Static Analysis\nZero clippy warnings.\n\n\
         ## 4. Dependency Auditing\nNo advisories.\n\n\
         ## 5. API Documentation\nAll public items documented.\n\n\
         ## 6. Runtime Safety\nNo avoidable panics.\n\n\
         ## 7. Package Metadata\nAll fields set.\n\n\
         ## 8. README & Onboarding\nQuick start provided.\n\n\
         ## 9. Release Automation\nTag-triggered workflow.\n\n\
         ## 10. Documentation Lint\nMissing-docs enabled.\n\n\
         ## 11. Security\nNo hardcoded secrets.\n\n\
         ## 12. Test Coverage\n252 tests pass.\n\n\
         ## 13. Observability\nStructured logging.\n\n\
         ## 14. Backwards Compatibility\nSemver followed.\n\n\
         ## Scoring\n\n| Score | Meaning |\n|-------|---------|\n\
         | PASS | Meets criteria | WARN | Gaps | FAIL | Risk |\n\n\
         ## Sign-Off\n\n| Role | Name | Date | Verdict |\n\
         |------|------|------|---------|");

    // Deployment artifacts (checks 114-116)
    write_file(root, "docs/6-deployment/transition_plan.md",
        "# Transition Plan\n\n**Audience**: Developers\n\nSystem transition approach.\n");
    write_file(root, "docs/6-deployment/release_notes.md",
        "# Release Notes\n\n**Audience**: Users\n\nRelease history.\n");
    write_file(root, "docs/6-deployment/user_manual.md",
        "# User Manual\n\n**Audience**: Users\n\nEnd-user operating instructions.\n");

    // Deployment artifacts (checks 61, 62, 68)
    write_file(root, "docs/6-deployment/deployment_guide.md",
        "# Deployment Guide\n\n**Audience**: Developers\n\nHow to deploy the system.\n");
    write_file(root, "docs/6-deployment/ci_cd.md",
        "# CI/CD Pipeline\n\n**Audience**: Developers\n\nContinuous integration and delivery.\n");
    write_file(root, "docs/6-deployment/installation_guide.md",
        "# Installation Guide\n\n**Audience**: Developers\n\nHow to install the system.\n");

    // Operations artifacts (checks 64-67)
    write_file(root, "docs/7-operations/operations_manual.md",
        "# Operations Manual\n\n**Audience**: Developers\n\nDay-to-day operations.\n");
    write_file(root, "docs/7-operations/troubleshooting.md",
        "# Troubleshooting Guide\n\n**Audience**: Developers\n\nCommon issues and resolutions.\n");
    write_file(root, "docs/7-operations/maintenance_plan.md",
        "# Maintenance Plan\n\n**Audience**: Developers\n\nOngoing maintenance procedures.\n");
    write_file(root, "docs/7-operations/configuration.md",
        "# Configuration Reference\n\n**Audience**: Developers\n\nConfiguration options.\n");

    // Operations artifact (check 117)
    write_file(root, "docs/7-operations/disposal_plan.md",
        "# Disposal Plan\n\n**Audience**: Developers\n\nSystem retirement procedures.\n");

    // ADR directory
    write_file(root, "docs/3-design/adr/README.md",
        "# ADR Index\n\n**Audience**: All\n\n- [001-use-rust.md](001-use-rust.md)\n");
    write_file(root, "docs/3-design/adr/001-use-rust.md",
        "# ADR 001: Use Rust\n\n**Audience**: Developers\n");

    // Root README links to docs/
    write_file(root, "README.md",
        "# Project\n\nSee [docs](docs/README.md) for details.\n");

    tmp
}

/// Write a file at `root/relative_path` with the given content, creating parent dirs.
pub fn write_file(root: &Path, relative_path: &str, content: &str) {
    let full = root.join(relative_path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&full, content).unwrap();
}
