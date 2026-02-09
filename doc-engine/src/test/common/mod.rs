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
        "# Hub\n\n**Audience**: All\n\n## Who\nTeam\n## What\nProduct\n## Why\nReason\n## How\nProcess\n\n- [0-overview](0-overview/)\n- [1-requirements](1-requirements/)\n- [2-planning](2-planning/)\n- [3-design](3-design/)\n- [4-development](4-development/)\n- [5-testing](5-testing/)\n"
    );
    write_file(root, "docs/glossary.md",
        "# Glossary\n\n**Audience**: All\n\n**API** - Application Programming Interface\n**CLI** - Command Line Interface\n**SDK** - Software Development Kit\n"
    );

    // Phase directories
    for phase in &["0-overview", "1-requirements", "2-planning", "3-design", "4-development", "5-testing"] {
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

    // Traceability artifacts (checks 51-53) + 29148 attributes (check 89)
    write_file(root, "docs/1-requirements/requirements.md",
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

    // developer_guide.md for check 69 + W3H for check 74 + 26514 sections for check 94
    write_file(root, "docs/4-development/developer_guide.md",
        "# Developer Guide\n\n**Audience**: Developers\n\n\
         ## What\nDevelopment guide.\n\n\
         ## Why\nOnboarding.\n\n\
         ## How\n\n\
         ### Build & Test\nRun `cargo build` and `cargo test`.\n\n\
         ### Project Structure\nSee src/ for code layout.\n\n\
         ### Adding New Features\nExtend the codebase.\n");

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
