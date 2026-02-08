use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Create a minimal project structure that passes core checks.
pub fn create_minimal_project() -> TempDir {
    let tmp = TempDir::new().unwrap();
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
        "# Hub\n\n**Audience**: All\n\n## Who\nTeam\n## What\nProduct\n## Why\nReason\n## How\nProcess\n\n- [0-overview](0-overview/)\n- [1-requirements](1-requirements/)\n- [3-design](3-design/)\n- [4-development](4-development/)\n- [5-testing](5-testing/)\n"
    );
    write_file(root, "docs/glossary.md",
        "# Glossary\n\n**Audience**: All\n\n**API** - Application Programming Interface\n**CLI** - Command Line Interface\n**SDK** - Software Development Kit\n"
    );

    // Phase directories
    for phase in &["0-overview", "1-requirements", "3-design", "4-development", "5-testing"] {
        let dir = format!("docs/{}", phase);
        write_file(root, &format!("{}/README.md", dir),
            &format!("# {}\n\n**Audience**: Developers\n", phase));
    }

    // Compliance checklist
    let checklist_content: String = (1..=20)
        .map(|i| format!("- [x] Rule {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    write_file(root, "docs/3-design/compliance/compliance_checklist.md",
        &format!("# Compliance Checklist\n\n**Audience**: All\n\n{}\n\nSee [architecture](../architecture.md)\n", checklist_content));

    // architecture.md for the checklist reference
    write_file(root, "docs/3-design/architecture.md", "# Architecture\n\n**Audience**: Developers\n");

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
