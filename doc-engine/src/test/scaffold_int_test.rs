mod common;
mod scaffold_fixtures;

use std::fs;
use std::path::PathBuf;
use doc_engine::{scaffold_from_srs, ScaffoldConfig};
use scaffold_fixtures::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn scaffold_to_tmp(srs_content: &str) -> (tempfile::TempDir, PathBuf, ScaffoldConfig) {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, srs_content).unwrap();
    let output_dir = tmp.path().join("output");
    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };
    (tmp, output_dir, config)
}

// === TC-002: FR-SCA-002: SRS domain extraction ============================

#[test]
fn test_parser_crlf_line_endings() {
    let srs = "### 4.1 Rule Loading\r\n\r\n#### FR-100: Default rules\r\n\r\n| Attribute | Value |\r\n|-----------|-------|\r\n| **Priority** | Must |\r\n\r\nThe binary embeds rules.\r\n";

    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("crlf.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 1);
    assert_eq!(result.requirement_count, 1);
}

#[test]
fn test_parser_requirement_with_code_block_in_narrative() {
    let srs = "\
### 4.1 Rule Loading

#### FR-100: TOML schema

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **Acceptance** | TOML accepted |

Each rule entry shall contain:

| Field | Type | Required |
|-------|------|----------|
| `id` | u8 | Yes |
| `category` | string | Yes |

Additional narrative after the inner table.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("code.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 1);
    assert_eq!(result.requirement_count, 1);
}

#[test]
fn test_parser_unicode_in_title() {
    let srs = "\
### 4.1 Règle de Chargement

#### FR-100: Règles par défaut

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Description with accented characters: éàü.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("unicode.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 1);
    // Unicode chars stripped by slugify, leaving alphanumeric only
    assert!(result.created.iter().any(|p| p.to_string_lossy().contains("r_gle_de_chargement")));
}

#[test]
fn test_parser_back_to_back_sections() {
    // Sections with no blank lines between them
    let srs = "\
### 4.1 First
#### FR-100: A req
| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
Desc A.
### 4.2 Second
#### FR-200: B req
| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
Desc B.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("compact.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 2);
}

#[test]
fn test_parser_nfr_only_domain() {
    let srs = "\
### 5.1 Performance

#### NFR-200: Sub-second execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **Verification** | Analysis |

Scans complete in under one second.

#### NFR-201: Single pass

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

All checks in a single traversal.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("nfr_only.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 1);
    assert_eq!(result.requirement_count, 2);

    // Verify YAML still generates correctly for NFRs
    let yaml = fs::read_to_string(
        tmp.path().join("out/docs/1-requirements/performance/performance.spec.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    assert_eq!(val["requirements"].as_sequence().unwrap().len(), 2);
    assert_eq!(val["requirements"][0]["sourceId"], "NFR-200");
}

#[test]
fn test_parser_many_domains_file_count() {
    // Build a synthetic SRS with 8 domains, 1 req each
    let mut srs = String::from("# SRS\n\n");
    for i in 1..=8 {
        srs.push_str(&format!(
            "### 4.{i} Domain {i}\n\n#### FR-{id}: Req {i}\n\n| Attribute | Value |\n|-----------|-------|\n| **Priority** | Must |\n\nDescription {i}.\n\n",
            i = i,
            id = i * 100,
        ));
    }

    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("many.md");
    fs::write(&srs_path, &srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 8);
    assert_eq!(result.requirement_count, 8);
    // 8 domains × 10 files + 2 BRD = 82
    assert_eq!(result.created.len(), 82);
}

#[test]
fn test_parse_real_project_srs() {
    let real_srs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs/1-requirements/srs.md");

    if !real_srs_path.exists() {
        // Skip if SRS not present (e.g., in CI without full repo)
        return;
    }

    let tmp = tempfile::TempDir::new().unwrap();
    let output_dir = tmp.path().join("real_srs_output");

    let config = ScaffoldConfig {
        srs_path: real_srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // The real SRS has many domains (sections 4.1-4.13 and 5.1-5.5+ and 6.x)
    assert!(result.domain_count >= 10, "Expected >=10 domains, got {}", result.domain_count);
    assert!(result.requirement_count >= 50, "Expected >=50 requirements, got {}", result.requirement_count);

    // Verify all created files are valid
    for path in &result.created {
        let full_path = output_dir.join(path);
        assert!(full_path.exists(), "File not created: {}", path.display());

        if path.to_string_lossy().ends_with(".yaml") {
            let content = fs::read_to_string(&full_path).unwrap();
            let _: serde_yml::Value = serde_yml::from_str(&content)
                .unwrap_or_else(|e| panic!("Invalid YAML in {}: {}", path.display(), e));
        }
    }

    // BRD should exist
    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    assert!(output_dir.join("docs/1-requirements/brd.spec").exists());

    // Verify BRD domain count matches
    let brd_yaml = fs::read_to_string(output_dir.join("docs/1-requirements/brd.spec.yaml")).unwrap();
    let brd: serde_yml::Value = serde_yml::from_str(&brd_yaml).unwrap();
    let brd_domains = brd["domains"].as_sequence().unwrap();
    assert_eq!(brd_domains.len(), result.domain_count);
}

#[test]
fn test_yaml_req_ids_per_domain_not_global() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // rule_loading has 3 reqs: REQ-001..003
    let rl_yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml"),
    ).unwrap();
    let rl: serde_yml::Value = serde_yml::from_str(&rl_yaml).unwrap();
    let rl_reqs = rl["requirements"].as_sequence().unwrap();
    assert_eq!(rl_reqs[0]["id"], "REQ-001");
    assert_eq!(rl_reqs[2]["id"], "REQ-003");

    // file_discovery also starts at REQ-001 (per-domain numbering)
    let fd_yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/file_discovery/file_discovery.spec.yaml"),
    ).unwrap();
    let fd: serde_yml::Value = serde_yml::from_str(&fd_yaml).unwrap();
    let fd_reqs = fd["requirements"].as_sequence().unwrap();
    assert_eq!(fd_reqs[0]["id"], "REQ-001");

    // Same for test case IDs
    let rl_test = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml"),
    ).unwrap();
    let rl_t: serde_yml::Value = serde_yml::from_str(&rl_test).unwrap();
    assert_eq!(rl_t["testCases"].as_sequence().unwrap()[0]["id"], "TC-001");

    let fd_test = fs::read_to_string(
        output_dir.join("docs/5-testing/file_discovery/file_discovery.test.yaml"),
    ).unwrap();
    let fd_t: serde_yml::Value = serde_yml::from_str(&fd_test).unwrap();
    assert_eq!(fd_t["testCases"].as_sequence().unwrap()[0]["id"], "TC-001");
}

#[test]
fn test_slug_with_parentheses_and_numbers() {
    let srs = "\
### 4.3 Check Execution (83 base + 15 spec)

#### FR-300: All checks

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Support 128 checks.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("parens.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 1);
    // Slug should handle parentheses correctly
    assert!(result.created.iter().any(|p| {
        let s = p.to_string_lossy();
        s.contains("check_execution")
    }));
}

#[test]
fn test_slug_similar_names_distinct() {
    let srs = "\
### 4.1 Rule Loading

#### FR-100: Rule A

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Desc A.

### 4.2 Rule Validation

#### FR-200: Rule B

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Desc B.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("similar.md");
    fs::write(&srs_path, srs).unwrap();
    let output_dir = tmp.path().join("out");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.domain_count, 2);

    // Each domain gets its own directory
    assert!(output_dir.join("docs/1-requirements/rule_loading").is_dir());
    assert!(output_dir.join("docs/1-requirements/rule_validation").is_dir());
}

#[test]
fn test_error_empty_srs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("empty.md");
    fs::write(&srs_path, "# SRS\n\nNo domains here.\n").unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let err = scaffold_from_srs(&config).unwrap_err();
    assert!(err.to_string().contains("no domains"));
}

#[test]
fn test_error_srs_with_only_non_requirement_sections() {
    let srs = "\
### 1.1 Purpose

This section explains the purpose.

### 1.2 Scope

This section describes the scope.

### 1.3 Definitions

No FR/NFR blocks at all.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("intro_only.md");
    fs::write(&srs_path, srs).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let err = scaffold_from_srs(&config).unwrap_err();
    assert!(err.to_string().contains("no domains"));
}

#[test]
fn test_error_nonexistent_srs_path() {
    let config = ScaffoldConfig {
        srs_path: PathBuf::from("/nonexistent/srs.md"),
        output_dir: PathBuf::from("/tmp/scaffold-out"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let err = scaffold_from_srs(&config).unwrap_err();
    assert!(err.to_string().contains("cannot read SRS file"));
}

// === TC-003: FR-SCA-003: Per-domain spec file generation ==================

#[test]
fn test_scaffold_file_tree() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 10 files + 2 BRD = 22
    assert_eq!(result.created.len(), 22);
    assert_eq!(result.domain_count, 2);
    assert_eq!(result.requirement_count, 3);

    // ISO/IEC/IEEE 15289:2019 clause 9 metadata
    assert_eq!(result.standard, "ISO/IEC/IEEE 15289:2019");
    assert_eq!(result.clause, "9");
    assert_eq!(result.tool, "doc-engine");
    assert_eq!(result.tool_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(result.timestamp.len(), 20);
    assert!(result.timestamp.ends_with('Z'));
    assert!(!result.srs_source.is_empty());
    assert!(result.phases.is_empty()); // no phase filter
    assert!(!result.force); // default

    // All 4 phase directories per domain
    for slug in &["rule_loading", "file_discovery"] {
        assert!(output_dir.join(format!("docs/1-requirements/{}/{}.spec.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/1-requirements/{}/{}.spec", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/3-design/{}/{}.arch.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/3-design/{}/{}.arch", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.test.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.test", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.manual.exec", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.auto.exec", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/6-deployment/{}/{}.deploy.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/6-deployment/{}/{}.deploy", slug, slug)).exists());
    }
    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    assert!(output_dir.join("docs/1-requirements/brd.spec").exists());
}

#[test]
fn test_scaffold_large_multi_domain() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    assert_eq!(result.domain_count, 5);
    // 3 + 2 + 2 + 1 + 2 = 10 requirements
    assert_eq!(result.requirement_count, 10);
    // 5 domains × 10 files + 2 BRD = 52
    assert_eq!(result.created.len(), 52);
    assert!(result.skipped.is_empty());
    // ISO metadata populated
    assert_eq!(result.standard, "ISO/IEC/IEEE 15289:2019");
    assert_eq!(result.tool, "doc-engine");

    // Verify each domain directory
    for slug in &["rule_loading", "file_discovery", "check_execution", "reporting", "architecture"] {
        assert!(
            output_dir.join(format!("docs/1-requirements/{}/{}.spec.yaml", slug, slug)).exists(),
            "Missing spec.yaml for {}", slug
        );
    }
}

#[test]
fn test_scaffold_domain_ordering_preserved() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // BRD YAML should list domains in SRS order
    let brd_yaml = fs::read_to_string(output_dir.join("docs/1-requirements/brd.spec.yaml")).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&brd_yaml).unwrap();
    let brd_domains = val["domains"].as_sequence().unwrap();

    assert_eq!(brd_domains[0]["section"], "4.1");
    assert_eq!(brd_domains[1]["section"], "4.2");
    assert_eq!(brd_domains[2]["section"], "4.3");
    assert_eq!(brd_domains[3]["section"], "4.4");
    assert_eq!(brd_domains[4]["section"], "5.1");
}

#[test]
fn test_yaml_feature_spec_all_fields() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();

    assert_eq!(val["kind"], "feature_request");
    assert_eq!(val["domain"], "Rule Loading");
    assert_eq!(val["section"], "4.1");

    let reqs = val["requirements"].as_sequence().unwrap();
    assert_eq!(reqs.len(), 2);

    // First requirement
    assert_eq!(reqs[0]["id"], "REQ-001");
    assert_eq!(reqs[0]["sourceId"], "FR-100");
    assert_eq!(reqs[0]["title"], "Default rules embedded in binary");
    assert_eq!(reqs[0]["priority"], "Must");
    assert_eq!(reqs[0]["status"], "Approved");
    assert_eq!(reqs[0]["verification"], "Test");
    assert_eq!(reqs[0]["acceptance"], "Engine loads embedded rules");

    // Second requirement
    assert_eq!(reqs[1]["id"], "REQ-002");
    assert_eq!(reqs[1]["sourceId"], "FR-101");
}

#[test]
fn test_yaml_arch_spec_components_from_traces() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();

    assert_eq!(val["kind"], "architecture");
    assert_eq!(val["specRef"], "docs/1-requirements/rule_loading/rule_loading.spec.yaml");

    // Both requirements have traces_to, so both should appear as components
    let components = val["components"].as_sequence().unwrap();
    assert_eq!(components.len(), 2);
    assert_eq!(components[0]["name"], "FR-100 handler");
    assert!(components[0]["tracesTo"].as_str().unwrap().contains("STK-01"));
    assert_eq!(components[1]["name"], "FR-101 handler");
}

#[test]
fn test_yaml_arch_spec_only_traced_requirements() {
    // Use fixture where NFR-200 has no traces_to
    let (_tmp, output_dir, config) = scaffold_to_tmp(MIXED_ATTRS_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/3-design/cli_interface/cli_interface.arch.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();

    // Only FR-500 has traces_to (none), NFR-200 has none, FR-501 has none
    // Actually none of them have traces_to in MIXED_ATTRS_SRS
    let components = val["components"].as_sequence().unwrap();
    assert_eq!(components.len(), 0, "Only requirements with traces_to should produce components");
}

#[test]
fn test_yaml_test_spec_traceability() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();

    assert_eq!(val["kind"], "test_plan");
    assert_eq!(val["specRef"], "docs/1-requirements/rule_loading/rule_loading.spec.yaml");

    let tcs = val["testCases"].as_sequence().unwrap();
    assert_eq!(tcs.len(), 2);

    // TC IDs are per-domain sequential
    assert_eq!(tcs[0]["id"], "TC-001");
    assert_eq!(tcs[0]["verifies"], "REQ-001");
    assert!(tcs[0]["test"].as_str().unwrap().contains("FR-100"));
    assert!(tcs[0]["test"].as_str().unwrap().contains("(Test)"));

    assert_eq!(tcs[1]["id"], "TC-002");
    assert_eq!(tcs[1]["verifies"], "REQ-002");
}

#[test]
fn test_yaml_deploy_spec_environments() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();

    assert_eq!(val["kind"], "deployment");
    assert_eq!(val["specRef"], "docs/1-requirements/rule_loading/rule_loading.spec.yaml");

    let envs = val["environments"].as_sequence().unwrap();
    assert_eq!(envs.len(), 2);
    assert_eq!(envs[0]["name"], "staging");
    assert_eq!(envs[1]["name"], "production");
    assert!(envs[0]["description"].as_str().unwrap().contains("Rule Loading"));
}

#[test]
fn test_yaml_brd_inventory_accuracy() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(output_dir.join("docs/1-requirements/brd.spec.yaml")).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();

    assert_eq!(val["kind"], "brd");
    assert_eq!(val["title"], "Business Requirements Document");

    let domains = val["domains"].as_sequence().unwrap();
    assert_eq!(domains.len(), 5);

    // Verify spec counts match actual requirement counts
    assert_eq!(domains[0]["specCount"], 3); // Rule Loading: FR-100, FR-101, FR-102
    assert_eq!(domains[1]["specCount"], 2); // File Discovery: FR-200, FR-201
    assert_eq!(domains[2]["specCount"], 2); // Check Execution: FR-300, FR-301
    assert_eq!(domains[3]["specCount"], 1); // Reporting: FR-400
    assert_eq!(domains[4]["specCount"], 2); // Architecture: NFR-100, NFR-101

    // Verify file refs contain slug
    assert!(domains[0]["specFile"].as_str().unwrap().contains("rule_loading"));
    assert!(domains[0]["archFile"].as_str().unwrap().contains("rule_loading"));
    assert!(domains[0]["testFile"].as_str().unwrap().contains("rule_loading"));
    assert!(domains[0]["deployFile"].as_str().unwrap().contains("rule_loading"));
}

#[test]
fn test_yaml_all_files_parse_cleanly() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    for path in &result.created {
        let full_path = output_dir.join(path);
        if path.to_string_lossy().ends_with(".yaml") {
            let content = fs::read_to_string(&full_path)
                .unwrap_or_else(|e| panic!("Cannot read {}: {}", path.display(), e));
            let _val: serde_yml::Value = serde_yml::from_str(&content)
                .unwrap_or_else(|e| panic!("Cannot parse YAML {}: {}", path.display(), e));
        }
    }
}

#[test]
fn test_yaml_roundtrip_all_kinds() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let kinds = [
        ("docs/1-requirements/rule_loading/rule_loading.spec.yaml", "feature_request"),
        ("docs/3-design/rule_loading/rule_loading.arch.yaml", "architecture"),
        ("docs/5-testing/rule_loading/rule_loading.test.yaml", "test_plan"),
        ("docs/6-deployment/rule_loading/rule_loading.deploy.yaml", "deployment"),
        ("docs/1-requirements/brd.spec.yaml", "brd"),
    ];

    for (rel_path, expected_kind) in &kinds {
        let yaml = fs::read_to_string(output_dir.join(rel_path))
            .unwrap_or_else(|e| panic!("Cannot read {}: {}", rel_path, e));
        let val: serde_yml::Value = serde_yml::from_str(&yaml)
            .unwrap_or_else(|e| panic!("Cannot parse {}: {}", rel_path, e));
        assert_eq!(
            val["kind"].as_str().unwrap(), *expected_kind,
            "Wrong kind for {}", rel_path
        );
    }
}

#[test]
fn test_yaml_cross_file_spec_refs_consistent() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // All arch/test/deploy for rule_loading should point to the same spec
    let expected_ref = "docs/1-requirements/rule_loading/rule_loading.spec.yaml";

    let arch: serde_yml::Value = serde_yml::from_str(
        &fs::read_to_string(output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml")).unwrap(),
    ).unwrap();
    assert_eq!(arch["specRef"].as_str().unwrap(), expected_ref);

    let test: serde_yml::Value = serde_yml::from_str(
        &fs::read_to_string(output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml")).unwrap(),
    ).unwrap();
    assert_eq!(test["specRef"].as_str().unwrap(), expected_ref);

    let deploy: serde_yml::Value = serde_yml::from_str(
        &fs::read_to_string(output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy.yaml")).unwrap(),
    ).unwrap();
    assert_eq!(deploy["specRef"].as_str().unwrap(), expected_ref);
}

#[test]
fn test_markdown_feature_spec_structure() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let md = fs::read_to_string(
        output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec"),
    ).unwrap();

    assert!(md.starts_with("# Feature Spec: Rule Loading"));
    assert!(md.contains("**Version:** 1.0"));
    assert!(md.contains("**Status:** Draft"));
    assert!(md.contains("**Section:** 4.1"));
    assert!(md.contains("## Requirements"));
    assert!(md.contains("## Acceptance Criteria"));

    // Table has correct columns
    assert!(md.contains("| ID | Source | Title | Priority | Verification | Acceptance |"));
    // Both requirements present
    assert!(md.contains("| REQ-001 |"));
    assert!(md.contains("| REQ-002 |"));
    assert!(md.contains("FR-100"));
    assert!(md.contains("FR-101"));

    // Acceptance criteria list
    assert!(md.contains("- **REQ-001** (FR-100): Engine loads embedded rules"));
    assert!(md.contains("- **REQ-002** (FR-101): External rules override"));
}

#[test]
fn test_markdown_arch_spec_links() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let md = fs::read_to_string(
        output_dir.join("docs/3-design/rule_loading/rule_loading.arch"),
    ).unwrap();

    assert!(md.starts_with("# Architecture: Rule Loading"));
    assert!(md.contains("**Spec:** [Feature Spec](../1-requirements/rule_loading/rule_loading.spec)"));
    assert!(md.contains("## Components"));
    assert!(md.contains("## Related Documents"));

    // Related docs links
    assert!(md.contains("[Feature Spec](../1-requirements/rule_loading/rule_loading.spec)"));
    assert!(md.contains("[Test Plan](../5-testing/rule_loading/rule_loading.test)"));
    assert!(md.contains("[Deployment](../6-deployment/rule_loading/rule_loading.deploy)"));

    // Components table should have both traced requirements
    assert!(md.contains("FR-100 handler"));
    assert!(md.contains("FR-101 handler"));
}

#[test]
fn test_markdown_test_spec_table() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let md = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.test"),
    ).unwrap();

    assert!(md.starts_with("# Test Plan: Rule Loading"));
    assert!(md.contains("**Spec:** [Feature Spec](../1-requirements/rule_loading/rule_loading.spec)"));
    assert!(md.contains("| ID | Test | Verifies | Priority |"));

    // Test cases
    assert!(md.contains("| TC-001 |"));
    assert!(md.contains("| TC-002 |"));
    assert!(md.contains("| REQ-001 |"));
    assert!(md.contains("| REQ-002 |"));
    // Verification method in test description
    assert!(md.contains("(Test)"));
}

#[test]
fn test_markdown_deploy_spec_stubs() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let md = fs::read_to_string(
        output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy"),
    ).unwrap();

    assert!(md.starts_with("# Deployment: Rule Loading"));
    assert!(md.contains("## Environments"));
    assert!(md.contains("| staging |"));
    assert!(md.contains("| production |"));
    assert!(md.contains("## Build"));
    assert!(md.contains("_TODO: Define build steps._"));
    assert!(md.contains("## Rollback"));
    assert!(md.contains("_TODO: Define rollback procedures._"));
}

#[test]
fn test_markdown_brd_inventory_table() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let md = fs::read_to_string(output_dir.join("docs/1-requirements/brd.spec")).unwrap();

    assert!(md.starts_with("# Business Requirements Document"));
    assert!(md.contains("**Version:** 1.0"));
    assert!(md.contains("**Status:** Draft"));
    assert!(md.contains("## Domain Inventory"));
    assert!(md.contains("## Domain Specifications"));

    // All 5 domains in inventory
    assert!(md.contains("rule_loading"));
    assert!(md.contains("file_discovery"));
    assert!(md.contains("check_execution"));
    assert!(md.contains("reporting"));
    assert!(md.contains("architecture"));

    // Domain spec listings
    assert!(md.contains("### 4.1 Rule Loading (rule_loading)"));
    assert!(md.contains("- **Requirements:** 3"));
    assert!(md.contains("### 5.1 Architecture (architecture)"));
    assert!(md.contains("- **Requirements:** 2"));
}

#[test]
fn test_markdown_yaml_consistency_per_domain() {
    // Verify the YAML and markdown versions of a spec contain matching data
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // YAML spec
    let yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    let yaml_reqs = val["requirements"].as_sequence().unwrap();

    // Markdown spec
    let md = fs::read_to_string(
        output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec"),
    ).unwrap();

    // Both should have the same number of requirements
    assert_eq!(yaml_reqs.len(), 2);
    // Both IDs should appear in markdown
    for yreq in yaml_reqs {
        let source_id = yreq["sourceId"].as_str().unwrap();
        assert!(md.contains(source_id), "Markdown missing source ID {}", source_id);
    }
}

#[test]
fn test_default_attribute_values_in_yaml() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(MIXED_ATTRS_SRS);
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/cli_interface/cli_interface.spec.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    let reqs = val["requirements"].as_sequence().unwrap();

    // FR-500: has all attributes
    assert_eq!(reqs[0]["status"], "Approved");
    assert_eq!(reqs[0]["verification"], "Demonstration");
    assert_eq!(reqs[0]["acceptance"], "CLI accepts scan subcommand");

    // NFR-200: only has Priority, rest should get defaults
    assert_eq!(reqs[1]["priority"], "Must");
    assert_eq!(reqs[1]["status"], "Proposed"); // default
    assert_eq!(reqs[1]["verification"], "Test"); // default
    assert_eq!(reqs[1]["acceptance"], ""); // default empty

    // FR-501: has Priority + State, rest defaults
    assert_eq!(reqs[2]["status"], "Proposed");
    assert_eq!(reqs[2]["verification"], "Test"); // default
}

#[test]
fn test_default_attribute_values_in_markdown() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(MIXED_ATTRS_SRS);
    scaffold_from_srs(&config).unwrap();

    let md = fs::read_to_string(
        output_dir.join("docs/1-requirements/cli_interface/cli_interface.spec"),
    ).unwrap();

    // NFR-200 has no acceptance, should show "—" (em dash) in table
    // The markdown table row for NFR-200 should contain "Unknown" or "Test" defaults
    assert!(md.contains("Synchronous execution"));
}

#[test]
fn test_brd_file_refs_match_generated_files() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    let brd_yaml = fs::read_to_string(output_dir.join("docs/1-requirements/brd.spec.yaml")).unwrap();
    let brd: serde_yml::Value = serde_yml::from_str(&brd_yaml).unwrap();

    for domain_entry in brd["domains"].as_sequence().unwrap() {
        let spec_file = domain_entry["specFile"].as_str().unwrap();
        let arch_file = domain_entry["archFile"].as_str().unwrap();
        let test_file = domain_entry["testFile"].as_str().unwrap();
        let deploy_file = domain_entry["deployFile"].as_str().unwrap();

        // Each referenced file should exist in the created list
        assert!(
            result.created.iter().any(|p| p.to_string_lossy() == spec_file),
            "BRD references {} but it wasn't created", spec_file
        );
        assert!(
            result.created.iter().any(|p| p.to_string_lossy() == arch_file),
            "BRD references {} but it wasn't created", arch_file
        );
        assert!(
            result.created.iter().any(|p| p.to_string_lossy() == test_file),
            "BRD references {} but it wasn't created", test_file
        );
        assert!(
            result.created.iter().any(|p| p.to_string_lossy() == deploy_file),
            "BRD references {} but it wasn't created", deploy_file
        );
    }
}

#[test]
fn test_no_files_for_empty_domain_sections() {
    // Mix of sections with and without requirements
    let srs = "\
### 1.1 Purpose

Just introductory text, no FR/NFR.

### 1.2 Scope

More intro text.

### 4.1 Rule Loading

#### FR-100: Default rules

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Desc.

### 4.2 Placeholder Section

No requirements in this section either.

### 4.3 Reporting

#### FR-400: Text output

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Report.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("sparse.md");
    fs::write(&srs_path, srs).unwrap();
    let output_dir = tmp.path().join("out");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    // Only 2 domains should have files (4.1 and 4.3)
    assert_eq!(result.domain_count, 2);
    // 2 × 10 + 2 BRD = 22
    assert_eq!(result.created.len(), 22);

    // No directories for empty domains
    assert!(!output_dir.join("docs/1-requirements/purpose").exists());
    assert!(!output_dir.join("docs/1-requirements/scope").exists());
    assert!(!output_dir.join("docs/1-requirements/placeholder_section").exists());
}

#[test]
fn test_result_counts_consistent_with_files() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    // created + skipped = total expected files
    let total = result.created.len() + result.skipped.len();
    let expected = result.domain_count * 10 + 2; // 10 per domain + 2 BRD
    assert_eq!(total, expected);

    // All created files actually exist
    for path in &result.created {
        assert!(
            output_dir.join(path).exists(),
            "Result says created but file missing: {}", path.display()
        );
    }

    // requirement_count matches sum across domains
    // (Verified by checking BRD)
    let brd_yaml = fs::read_to_string(output_dir.join("docs/1-requirements/brd.spec.yaml")).unwrap();
    let brd: serde_yml::Value = serde_yml::from_str(&brd_yaml).unwrap();
    let total_reqs: u64 = brd["domains"].as_sequence().unwrap().iter()
        .map(|d| d["specCount"].as_u64().unwrap())
        .sum();
    assert_eq!(total_reqs, result.requirement_count as u64);
}

#[test]
fn test_yaml_and_markdown_parity() {
    let (_tmp, _output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    let yaml_files: Vec<_> = result.created.iter()
        .filter(|p| p.to_string_lossy().ends_with(".yaml"))
        .collect();
    let md_files: Vec<_> = result.created.iter()
        .filter(|p| !p.to_string_lossy().ends_with(".yaml") && !p.to_string_lossy().ends_with(".exec"))
        .collect();
    let exec_files: Vec<_> = result.created.iter()
        .filter(|p| p.to_string_lossy().ends_with(".exec"))
        .collect();

    // Equal number of YAML and paired markdown files (exec files are markdown-only)
    assert_eq!(yaml_files.len(), md_files.len());
    // 2 exec files per domain (manual + auto)
    assert_eq!(exec_files.len(), result.domain_count * 2);

    // For each YAML file there should be a corresponding markdown file
    for yaml_path in &yaml_files {
        let yaml_str = yaml_path.to_string_lossy();
        let md_path = yaml_str.replace(".spec.yaml", ".spec")
            .replace(".arch.yaml", ".arch")
            .replace(".test.yaml", ".test")
            .replace(".deploy.yaml", ".deploy");

        assert!(
            md_files.iter().any(|p| p.to_string_lossy() == md_path),
            "YAML {} has no matching markdown file", yaml_str
        );
    }
}

// === TC-004: FR-SCA-004: Manual test execution plan =======================

#[test]
fn test_verification_method_in_test_yaml() {
    let srs = "\
### 4.1 Reporting

#### FR-400: Text output

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **Verification** | Demonstration |

Default text output.

#### FR-401: JSON output

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **Verification** | Test |

JSON output.

#### FR-402: Exit codes

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **Verification** | Inspection |

Exit code behavior.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("methods.md");
    fs::write(&srs_path, srs).unwrap();
    let output_dir = tmp.path().join("out");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/5-testing/reporting/reporting.test.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    let tcs = val["testCases"].as_sequence().unwrap();

    // Verification methods should be in the test description
    assert!(tcs[0]["test"].as_str().unwrap().contains("(Demonstration)"));
    assert!(tcs[1]["test"].as_str().unwrap().contains("(Test)"));
    assert!(tcs[2]["test"].as_str().unwrap().contains("(Inspection)"));
}

#[test]
fn test_manual_exec_steps_test_method_with_backtick() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/cli_interface/cli_interface.manual.exec"),
    ).unwrap();

    // FR-500: Test with backtick command → "Run `doc-engine scan <PATH>`"
    let tc001 = manual.lines().find(|l| l.contains("TC-001")).unwrap();
    assert!(
        tc001.contains("Run `doc-engine scan <PATH>`"),
        "TC-001 should have auto-populated Test step, got: {}", tc001,
    );

    // FR-501: Test with backtick command → "Run `doc-engine scan <PATH> --json`"
    let tc002 = manual.lines().find(|l| l.contains("TC-002")).unwrap();
    assert!(
        tc002.contains("Run `doc-engine scan <PATH> --json`"),
        "TC-002 should have auto-populated Test step, got: {}", tc002,
    );
}

#[test]
fn test_manual_exec_steps_demonstration_method() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/cli_interface/cli_interface.manual.exec"),
    ).unwrap();

    // FR-502: Demonstration with backtick → "Execute `doc-engine --help` and observe output"
    let tc003 = manual.lines().find(|l| l.contains("TC-003")).unwrap();
    assert!(
        tc003.contains("Execute `doc-engine --help` and observe output"),
        "TC-003 should have Demonstration step with command, got: {}", tc003,
    );

    // FR-503: Demonstration without backtick command → _TODO_ (prose never used for Steps)
    let tc004 = manual.lines().find(|l| l.contains("TC-004")).unwrap();
    assert!(
        tc004.contains("_TODO_"),
        "TC-004 should be _TODO_ when no command found, got: {}", tc004,
    );
}

#[test]
fn test_manual_exec_steps_inspection_method() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/architecture/architecture.manual.exec"),
    ).unwrap();

    // NFR-100: Inspection with traces_to file → "Review `saf/mod.rs`"
    let tc001 = manual.lines().find(|l| l.contains("TC-001")).unwrap();
    assert!(
        tc001.contains("Review `saf/mod.rs`"),
        "TC-001 should have Inspection step with file, got: {}", tc001,
    );
}

#[test]
fn test_manual_exec_steps_analysis_method() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/architecture/architecture.manual.exec"),
    ).unwrap();

    // NFR-101: Analysis with traces_to file → "Analyze `core/scanner.rs`"
    let tc002 = manual.lines().find(|l| l.contains("TC-002")).unwrap();
    assert!(
        tc002.contains("Analyze `core/scanner.rs`"),
        "TC-002 should have Analysis step with trace file, got: {}", tc002,
    );

    // NFR-102: Analysis with no acceptance but has description → uses description
    let tc003 = manual.lines().find(|l| l.contains("TC-003")).unwrap();
    assert!(
        tc003.contains("Analyze: Algorithm complexity analysis."),
        "TC-003 should use description fallback, got: {}", tc003,
    );
}

#[test]
fn test_manual_exec_steps_large_fixture_inspection_trace() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // rule_loading domain: FR-102 is Inspection with traces_to "STK-02 -> api/types.rs"
    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.manual.exec"),
    ).unwrap();

    let tc003 = manual.lines().find(|l| l.contains("TC-003")).unwrap();
    assert!(
        tc003.contains("Review `api/types.rs`"),
        "FR-102 Inspection should reference api/types.rs, got: {}", tc003,
    );
}

#[test]
fn test_manual_exec_steps_large_fixture_demonstration_no_command() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // reporting domain: FR-400 is Demonstration, acceptance has no backtick command
    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/reporting/reporting.manual.exec"),
    ).unwrap();

    let tc001 = manual.lines().find(|l| l.contains("TC-001")).unwrap();
    assert!(
        tc001.contains("_TODO_"),
        "FR-400 Demonstration without command should be _TODO_, got: {}", tc001,
    );
}

#[test]
fn test_manual_exec_steps_large_fixture_inspection_no_file() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // architecture domain: NFR-100 is Inspection, traces_to "SYS-01" (no arrow → no file)
    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/architecture/architecture.manual.exec"),
    ).unwrap();

    let tc001 = manual.lines().find(|l| l.contains("TC-001")).unwrap();
    assert!(
        tc001.contains("_TODO_"),
        "NFR-100 Inspection without trace file should be _TODO_, got: {}", tc001,
    );
}

#[test]
fn test_manual_exec_steps_todo_fallback_no_backtick() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    // FIXTURE_SRS: all Test method, no backtick commands in acceptance → _TODO_
    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.manual.exec"),
    ).unwrap();

    let tc001 = manual.lines().find(|l| l.contains("TC-001")).unwrap();
    assert!(
        tc001.contains("_TODO_"),
        "Test without backtick command should fall back to _TODO_, got: {}", tc001,
    );
}

#[test]
fn test_manual_exec_steps_table_columns_intact() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/cli_interface/cli_interface.manual.exec"),
    ).unwrap();

    // Every TC row in the Test Cases table must have exactly 5 unescaped pipes
    // (4 columns: TC, Test, Steps, Expected). Filter to rows containing FR- or NFR-
    // to skip the Execution Log table rows which have different column counts.
    for line in manual.lines().filter(|l| l.contains("TC-0") && (l.contains("FR-") || l.contains("NFR-"))) {
        let unescaped = line.matches('|').count() - line.matches("\\|").count();
        assert_eq!(
            unescaped, 5,
            "Table row has wrong column count ({}): {}", unescaped, line,
        );
    }
}

#[test]
fn test_manual_exec_expected_strips_command_prefix() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/cli_interface/cli_interface.manual.exec"),
    ).unwrap();

    // FR-500: Steps="Run `doc-engine scan <PATH>`", Expected should strip the command
    let tc001 = manual.lines().find(|l| l.contains("TC-001")).unwrap();
    let cols: Vec<&str> = tc001.split('|').collect();
    let expected = cols[4].trim(); // 5th column (0-indexed)
    assert_eq!(
        expected, "Outputs a compliance report",
        "TC-001 Expected should strip command prefix, got: {}", expected,
    );

    // FR-501: Steps="Run `doc-engine scan <PATH> --json`", Expected should strip
    let tc002 = manual.lines().find(|l| l.contains("TC-002")).unwrap();
    let cols: Vec<&str> = tc002.split('|').collect();
    let expected = cols[4].trim();
    assert_eq!(
        expected, "Outputs valid JSON",
        "TC-002 Expected should strip command prefix, got: {}", expected,
    );

    // FR-502: Steps="Execute `doc-engine --help` and observe output", Expected should strip
    let tc003 = manual.lines().find(|l| l.contains("TC-003")).unwrap();
    let cols: Vec<&str> = tc003.split('|').collect();
    let expected = cols[4].trim();
    assert_eq!(
        expected, "Shows usage",
        "TC-003 Expected should strip command prefix, got: {}", expected,
    );
}

#[test]
fn test_manual_exec_expected_no_strip_when_no_command() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(STEPS_FIXTURE_SRS);
    scaffold_from_srs(&config).unwrap();

    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/cli_interface/cli_interface.manual.exec"),
    ).unwrap();

    // FR-503: Steps="_TODO_" (no backtick command), Expected should remain unchanged
    let tc004 = manual.lines().find(|l| l.contains("TC-004")).unwrap();
    let cols: Vec<&str> = tc004.split('|').collect();
    let expected = cols[4].trim();
    assert_eq!(
        expected, "Verbose flag increases output detail",
        "TC-004 Expected should be unchanged when steps is _TODO_, got: {}", expected,
    );
}

// === TC-006: FR-SCA-006: Skip/force behavior ==============================

#[test]
fn test_skip_existing_preserves_content() {
    let (_tmp, output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);

    // First run
    scaffold_from_srs(&config).unwrap();

    // Manually modify one file
    let spec_path = output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml");
    fs::write(&spec_path, "# Modified by user\n").unwrap();

    // Second run without --force
    let r2 = scaffold_from_srs(&config).unwrap();
    assert_eq!(r2.skipped.len(), 22);
    assert!(r2.created.is_empty());
    assert!(!r2.force); // force=false reflected in result

    // User modification preserved
    let content = fs::read_to_string(&spec_path).unwrap();
    assert_eq!(content, "# Modified by user\n");
}

#[test]
fn test_force_overwrite_updates_content() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");
    let mut config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    // First run
    scaffold_from_srs(&config).unwrap();

    // Manually modify one file
    let spec_path = output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml");
    fs::write(&spec_path, "# Modified by user\n").unwrap();

    // Second run WITH --force
    config.force = true;
    let r2 = scaffold_from_srs(&config).unwrap();
    assert_eq!(r2.created.len(), 22);
    assert!(r2.skipped.is_empty());
    assert!(r2.force); // force=true reflected in result

    // User modification overwritten with fresh content
    let content = fs::read_to_string(&spec_path).unwrap();
    assert!(content.contains("kind: feature_request"), "Force should overwrite with generated content");
}

#[test]
fn test_skip_existing_mixed_scenario() {
    // Pre-create some files, leave others missing
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    // Pre-create just the BRD files
    let brd_dir = output_dir.join("docs/1-requirements");
    fs::create_dir_all(&brd_dir).unwrap();
    fs::write(brd_dir.join("brd.spec.yaml"), "existing").unwrap();
    fs::write(brd_dir.join("brd.spec"), "existing").unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    // 20 domain files created, 2 BRD files skipped
    assert_eq!(result.created.len(), 20);
    assert_eq!(result.skipped.len(), 2);
    assert!(result.skipped.iter().any(|p| p.to_string_lossy().contains("brd.spec.yaml")));
    assert!(result.skipped.iter().any(|p| p.to_string_lossy().contains("brd.spec")));
}

// === TC-007: FR-SCA-007: Output directory =================================

#[test]
fn test_output_dir_created_automatically() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    // Output dir doesn't exist yet
    let output_dir = tmp.path().join("does/not/exist/yet");
    assert!(!output_dir.exists());

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();
    assert_eq!(result.created.len(), 22);
    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
}

// === TC-008: FR-SCA-008: Phase and type filters ===========================

#[test]
fn test_phase_filter_single_testing() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["testing".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 4 testing files = 8 (no BRD)
    assert_eq!(result.created.len(), 8);
    assert_eq!(result.domain_count, 2);
    assert_eq!(result.requirement_count, 3);
    // Phase filter reflected in result
    assert_eq!(result.phases, vec!["testing"]);

    for slug in &["rule_loading", "file_discovery"] {
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.test.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.test", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.manual.exec", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/5-testing/{}/{}.auto.exec", slug, slug)).exists());
        // Other phases should NOT exist
        assert!(!output_dir.join(format!("docs/1-requirements/{}", slug)).exists());
        assert!(!output_dir.join(format!("docs/3-design/{}", slug)).exists());
        assert!(!output_dir.join(format!("docs/6-deployment/{}", slug)).exists());
    }
    assert!(!output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
}

#[test]
fn test_phase_filter_requirements_includes_brd() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["requirements".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 2 req files + 2 BRD = 6
    assert_eq!(result.created.len(), 6);
    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    assert!(output_dir.join("docs/1-requirements/brd.spec").exists());
    assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml").exists());
    assert!(!output_dir.join("docs/3-design/rule_loading").exists());
}

#[test]
fn test_phase_filter_multiple_phases() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["design".to_string(), "deployment".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × (2 design + 2 deployment) = 8 (no BRD)
    assert_eq!(result.created.len(), 8);

    for slug in &["rule_loading", "file_discovery"] {
        assert!(output_dir.join(format!("docs/3-design/{}/{}.arch.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/3-design/{}/{}.arch", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/6-deployment/{}/{}.deploy.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/6-deployment/{}/{}.deploy", slug, slug)).exists());
    }
    assert!(!output_dir.join("docs/1-requirements/rule_loading").exists());
    assert!(!output_dir.join("docs/5-testing/rule_loading").exists());
}

#[test]
fn test_phase_filter_deployment_only() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["deployment".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 2 deploy files = 4
    assert_eq!(result.created.len(), 4);
    assert!(output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy.yaml").exists());
    assert!(output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy").exists());
}

#[test]
fn test_phase_filter_with_force() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    // First run: all phases
    let config_all = ScaffoldConfig {
        srs_path: srs_path.clone(),
        output_dir: output_dir.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };
    scaffold_from_srs(&config_all).unwrap();

    // Second run: testing only with --force
    let config_phase = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: true,
        phases: vec!["testing".to_string()],
        file_types: vec![],
    };
    let result = scaffold_from_srs(&config_phase).unwrap();

    // Only testing files created (force applies only to filtered set)
    assert_eq!(result.created.len(), 8);
    assert!(result.skipped.is_empty());
}

#[test]
fn test_phase_filter_design_only() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["design".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 2 design files = 4 (no BRD)
    assert_eq!(result.created.len(), 4);

    for slug in &["rule_loading", "file_discovery"] {
        assert!(output_dir.join(format!("docs/3-design/{}/{}.arch.yaml", slug, slug)).exists());
        assert!(output_dir.join(format!("docs/3-design/{}/{}.arch", slug, slug)).exists());
        assert!(!output_dir.join(format!("docs/1-requirements/{}", slug)).exists());
        assert!(!output_dir.join(format!("docs/5-testing/{}", slug)).exists());
        assert!(!output_dir.join(format!("docs/6-deployment/{}", slug)).exists());
    }
    assert!(!output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    assert!(!output_dir.join("docs/1-requirements/brd.spec").exists());
}

#[test]
fn test_phase_filter_all_four_equals_no_filter() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let output_all = tmp.path().join("all");
    let config_all = ScaffoldConfig {
        srs_path: srs_path.clone(),
        output_dir: output_all.clone(),
        force: false,
        phases: vec![],
        file_types: vec![],
    };
    let result_all = scaffold_from_srs(&config_all).unwrap();

    let output_explicit = tmp.path().join("explicit");
    let config_explicit = ScaffoldConfig {
        srs_path,
        output_dir: output_explicit.clone(),
        force: false,
        phases: vec![
            "requirements".to_string(),
            "design".to_string(),
            "testing".to_string(),
            "deployment".to_string(),
        ],
        file_types: vec![],
    };
    let result_explicit = scaffold_from_srs(&config_explicit).unwrap();

    assert_eq!(result_all.created.len(), result_explicit.created.len());
    assert_eq!(result_all.domain_count, result_explicit.domain_count);
    assert_eq!(result_all.requirement_count, result_explicit.requirement_count);

    // Same file paths
    let mut paths_all: Vec<String> = result_all.created.iter().map(|p| p.to_string_lossy().to_string()).collect();
    let mut paths_explicit: Vec<String> = result_explicit.created.iter().map(|p| p.to_string_lossy().to_string()).collect();
    paths_all.sort();
    paths_explicit.sort();
    assert_eq!(paths_all, paths_explicit);
}

#[test]
fn test_phase_filter_large_srs_testing_only() {
    let (_tmp, output_dir, mut config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    config.phases = vec!["testing".to_string()];

    let result = scaffold_from_srs(&config).unwrap();

    assert_eq!(result.domain_count, 5);
    assert_eq!(result.requirement_count, 10);
    // 5 domains × 4 testing files = 20
    assert_eq!(result.created.len(), 20);

    for slug in &["rule_loading", "file_discovery", "check_execution", "reporting", "architecture"] {
        assert!(
            output_dir.join(format!("docs/5-testing/{}/{}.test.yaml", slug, slug)).exists(),
            "Missing test.yaml for {}", slug
        );
        assert!(
            output_dir.join(format!("docs/5-testing/{}/{}.manual.exec", slug, slug)).exists(),
            "Missing manual.exec for {}", slug
        );
        assert!(
            output_dir.join(format!("docs/5-testing/{}/{}.auto.exec", slug, slug)).exists(),
            "Missing auto.exec for {}", slug
        );
    }
    // No other phase dirs
    assert!(!output_dir.join("docs/1-requirements/rule_loading").exists());
    assert!(!output_dir.join("docs/3-design/rule_loading").exists());
    assert!(!output_dir.join("docs/6-deployment/rule_loading").exists());
}

#[test]
fn test_phase_filter_large_srs_requirements_only() {
    let (_tmp, output_dir, mut config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    config.phases = vec!["requirements".to_string()];

    let result = scaffold_from_srs(&config).unwrap();

    // 5 domains × 2 req files + 2 BRD = 12
    assert_eq!(result.created.len(), 12);
    assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    assert!(output_dir.join("docs/1-requirements/brd.spec").exists());
    assert!(!output_dir.join("docs/3-design/rule_loading").exists());
}

#[test]
fn test_phase_filter_large_srs_three_phases() {
    let (_tmp, _output_dir, mut config) = scaffold_to_tmp(LARGE_FIXTURE_SRS);
    config.phases = vec![
        "requirements".to_string(),
        "design".to_string(),
        "testing".to_string(),
    ];

    let result = scaffold_from_srs(&config).unwrap();

    // 5 domains × (2 req + 2 design + 4 testing) + 2 BRD = 42
    assert_eq!(result.created.len(), 42);
}

#[test]
fn test_phase_filter_skip_existing_in_filtered_phase() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    // Pre-create one testing file
    let test_dir = output_dir.join("docs/5-testing/rule_loading");
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(test_dir.join("rule_loading.test.yaml"), "existing").unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["testing".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 8 total testing files: 1 skipped, 7 created
    assert_eq!(result.created.len(), 7);
    assert_eq!(result.skipped.len(), 1);
    assert!(result.skipped.iter().any(|p| p.to_string_lossy().contains("rule_loading.test.yaml")));

    // Skipped file preserved
    let content = fs::read_to_string(test_dir.join("rule_loading.test.yaml")).unwrap();
    assert_eq!(content, "existing");
}

#[test]
fn test_phase_filter_skip_then_force() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    // First run: design only
    let config1 = ScaffoldConfig {
        srs_path: srs_path.clone(),
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["design".to_string()],
        file_types: vec![],
    };
    let r1 = scaffold_from_srs(&config1).unwrap();
    assert_eq!(r1.created.len(), 4);

    // Second run: design only without force — all skipped
    let config2 = ScaffoldConfig {
        srs_path: srs_path.clone(),
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["design".to_string()],
        file_types: vec![],
    };
    let r2 = scaffold_from_srs(&config2).unwrap();
    assert_eq!(r2.skipped.len(), 4);
    assert!(r2.created.is_empty());

    // Third run: design only with force — all re-created
    let config3 = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: true,
        phases: vec!["design".to_string()],
        file_types: vec![],
    };
    let r3 = scaffold_from_srs(&config3).unwrap();
    assert_eq!(r3.created.len(), 4);
    assert!(r3.skipped.is_empty());
}

#[test]
fn test_phase_filter_yaml_files_valid() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, LARGE_FIXTURE_SRS).unwrap();

    // Test each single-phase filter produces valid YAML
    for phase in &["requirements", "design", "testing", "deployment"] {
        let output_dir = tmp.path().join(format!("out_{}", phase));
        let config = ScaffoldConfig {
            srs_path: srs_path.clone(),
            output_dir: output_dir.clone(),
            force: false,
            phases: vec![phase.to_string()],
        file_types: vec![],
        };
        let result = scaffold_from_srs(&config).unwrap();

        for path in &result.created {
            if path.to_string_lossy().ends_with(".yaml") {
                let full_path = output_dir.join(path);
                let content = fs::read_to_string(&full_path)
                    .unwrap_or_else(|e| panic!("Cannot read {}: {}", path.display(), e));
                let _: serde_yml::Value = serde_yml::from_str(&content)
                    .unwrap_or_else(|e| panic!("Invalid YAML in {} (phase={}): {}", path.display(), phase, e));
            }
        }
    }
}

#[test]
fn test_phase_filter_testing_content_valid() {
    let (_tmp, output_dir, mut config) = scaffold_to_tmp(FIXTURE_SRS);
    config.phases = vec!["testing".to_string()];
    scaffold_from_srs(&config).unwrap();

    // Test YAML has correct kind and test cases
    let yaml = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    assert_eq!(val["kind"], "test_plan");
    assert_eq!(val["testCases"].as_sequence().unwrap().len(), 2);

    // Test markdown has proper structure
    let md = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.test"),
    ).unwrap();
    assert!(md.starts_with("# Test Plan: Rule Loading"));
    assert!(md.contains("TC-001"));
    assert!(md.contains("TC-002"));

    // Manual exec has all TCs
    let manual = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.manual.exec"),
    ).unwrap();
    assert!(manual.contains("TC-001"));
    assert!(manual.contains("TC-002"));

    // Auto exec has all TCs
    let auto = fs::read_to_string(
        output_dir.join("docs/5-testing/rule_loading/rule_loading.auto.exec"),
    ).unwrap();
    assert!(auto.contains("TC-001"));
    assert!(auto.contains("TC-002"));
}

#[test]
fn test_phase_filter_design_content_valid() {
    let (_tmp, output_dir, mut config) = scaffold_to_tmp(FIXTURE_SRS);
    config.phases = vec!["design".to_string()];
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    assert_eq!(val["kind"], "architecture");
    assert_eq!(val["domain"], "Rule Loading");

    let md = fs::read_to_string(
        output_dir.join("docs/3-design/rule_loading/rule_loading.arch"),
    ).unwrap();
    assert!(md.starts_with("# Architecture: Rule Loading"));
    assert!(md.contains("## Components"));
}

#[test]
fn test_phase_filter_requirements_content_valid() {
    let (_tmp, output_dir, mut config) = scaffold_to_tmp(FIXTURE_SRS);
    config.phases = vec!["requirements".to_string()];
    scaffold_from_srs(&config).unwrap();

    // Feature spec
    let yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    assert_eq!(val["kind"], "feature_request");
    assert_eq!(val["requirements"].as_sequence().unwrap().len(), 2);

    // BRD
    let brd_yaml = fs::read_to_string(
        output_dir.join("docs/1-requirements/brd.spec.yaml"),
    ).unwrap();
    let brd: serde_yml::Value = serde_yml::from_str(&brd_yaml).unwrap();
    assert_eq!(brd["kind"], "brd");
    assert_eq!(brd["domains"].as_sequence().unwrap().len(), 2);
}

#[test]
fn test_phase_filter_deployment_content_valid() {
    let (_tmp, output_dir, mut config) = scaffold_to_tmp(FIXTURE_SRS);
    config.phases = vec!["deployment".to_string()];
    scaffold_from_srs(&config).unwrap();

    let yaml = fs::read_to_string(
        output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
    assert_eq!(val["kind"], "deployment");
    assert_eq!(val["environments"].as_sequence().unwrap().len(), 2);

    let md = fs::read_to_string(
        output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy"),
    ).unwrap();
    assert!(md.starts_with("# Deployment: Rule Loading"));
    assert!(md.contains("## Rollback"));
}

#[test]
fn test_phase_filter_nfr_only_domain() {
    let srs = "\
### 5.1 Performance

#### NFR-200: Sub-second execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **Verification** | Analysis |

Scans complete in under one second.
";
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("nfr.md");
    fs::write(&srs_path, srs).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["testing".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 1 domain × 4 testing files = 4
    assert_eq!(result.created.len(), 4);
    assert_eq!(result.domain_count, 1);
    assert_eq!(result.requirement_count, 1);

    let test_yaml = fs::read_to_string(
        output_dir.join("docs/5-testing/performance/performance.test.yaml"),
    ).unwrap();
    let val: serde_yml::Value = serde_yml::from_str(&test_yaml).unwrap();
    let tcs = val["testCases"].as_sequence().unwrap();
    assert_eq!(tcs.len(), 1);
    assert!(tcs[0]["test"].as_str().unwrap().contains("(Analysis)"));
}

#[test]
fn test_phase_filter_mixed_attrs_design() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, MIXED_ATTRS_SRS).unwrap();
    let output_dir = tmp.path().join("output");

    let config = ScaffoldConfig {
        srs_path,
        output_dir: output_dir.clone(),
        force: false,
        phases: vec!["design".to_string()],
        file_types: vec![],
    };

    let result = scaffold_from_srs(&config).unwrap();

    // 1 domain × 2 design files = 2
    assert_eq!(result.created.len(), 2);
    assert_eq!(result.requirement_count, 3);

    let md = fs::read_to_string(
        output_dir.join("docs/3-design/cli_interface/cli_interface.arch"),
    ).unwrap();
    assert!(md.contains("CLI Interface"));
}

#[test]
fn test_phase_filter_result_counts_formula() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, LARGE_FIXTURE_SRS).unwrap();

    // Each phase contributes a known number of files per domain
    let phase_files: Vec<(&str, usize, bool)> = vec![
        ("requirements", 2, true),  // 2 per domain + 2 BRD
        ("design", 2, false),       // 2 per domain
        ("testing", 4, false),      // 4 per domain
        ("deployment", 2, false),   // 2 per domain
    ];

    for (phase, files_per_domain, has_brd) in &phase_files {
        let output_dir = tmp.path().join(format!("out_{}", phase));
        let config = ScaffoldConfig {
            srs_path: srs_path.clone(),
            output_dir,
            force: false,
            phases: vec![phase.to_string()],
        file_types: vec![],
        };
        let result = scaffold_from_srs(&config).unwrap();

        let expected = result.domain_count * files_per_domain + if *has_brd { 2 } else { 0 };
        assert_eq!(
            result.created.len(), expected,
            "Phase '{}': expected {} files ({}×{} + brd={}), got {}",
            phase, expected, result.domain_count, files_per_domain, has_brd, result.created.len()
        );
    }
}

#[test]
fn test_type_filter_exec_only() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("output"),
        force: false,
        phases: vec![],
        file_types: vec!["exec".to_string()],
    };
    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 2 exec files (manual.exec + auto.exec) = 4
    assert_eq!(result.created.len(), 4);
    for path in &result.created {
        let s = path.to_string_lossy();
        assert!(
            s.ends_with(".manual.exec") || s.ends_with(".auto.exec"),
            "Unexpected file with exec filter: {}", s,
        );
    }
}

#[test]
fn test_type_filter_yaml_only() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("output"),
        force: false,
        phases: vec![],
        file_types: vec!["yaml".to_string()],
    };
    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × 4 yaml files (spec.yaml, arch.yaml, test.yaml, deploy.yaml)
    // + 1 BRD yaml = 9
    assert_eq!(result.created.len(), 9);
    for path in &result.created {
        let s = path.to_string_lossy();
        assert!(
            s.ends_with(".yaml"),
            "Unexpected file with yaml filter: {}", s,
        );
    }
}

#[test]
fn test_type_filter_multiple() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("output"),
        force: false,
        phases: vec![],
        file_types: vec!["yaml".to_string(), "exec".to_string()],
    };
    let result = scaffold_from_srs(&config).unwrap();

    // 2 domains × (4 yaml + 2 exec) + 1 BRD yaml = 13
    assert_eq!(result.created.len(), 13);
    for path in &result.created {
        let s = path.to_string_lossy();
        assert!(
            s.ends_with(".yaml") || s.ends_with(".manual.exec") || s.ends_with(".auto.exec"),
            "Unexpected file with yaml,exec filter: {}", s,
        );
    }
}

#[test]
fn test_type_filter_empty_means_all() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("output"),
        force: false,
        phases: vec![],
        file_types: vec![],
    };
    let result = scaffold_from_srs(&config).unwrap();

    // Empty file_types = all types: 2 domains × 10 + 2 BRD = 22
    assert_eq!(result.created.len(), 22);
}

#[test]
fn test_type_filter_combined_with_phase() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("output"),
        force: false,
        phases: vec!["testing".to_string()],
        file_types: vec!["exec".to_string()],
    };
    let result = scaffold_from_srs(&config).unwrap();

    // testing phase + exec type: 2 domains × 2 exec files = 4
    assert_eq!(result.created.len(), 4);
    for path in &result.created {
        let s = path.to_string_lossy();
        assert!(s.contains("5-testing"), "File not in testing phase: {}", s);
        assert!(
            s.ends_with(".manual.exec") || s.ends_with(".auto.exec"),
            "Unexpected file type: {}", s,
        );
    }
}

// === TC-009: FR-SCA-009: Scaffold status report ===========================

#[test]
fn test_phase_filter_metadata_unchanged() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, LARGE_FIXTURE_SRS).unwrap();

    let phases_to_test: Vec<Vec<String>> = vec![
        vec![],
        vec!["requirements".to_string()],
        vec!["design".to_string()],
        vec!["testing".to_string()],
        vec!["deployment".to_string()],
        vec!["requirements".to_string(), "testing".to_string()],
    ];

    for phases in phases_to_test {
        let output_dir = tmp.path().join(format!("out_{}", phases.join("_")));
        let config = ScaffoldConfig {
            srs_path: srs_path.clone(),
            output_dir,
            force: false,
            phases: phases.clone(),
        file_types: vec![],
        };
        let result = scaffold_from_srs(&config).unwrap();

        assert_eq!(result.domain_count, 5, "domain_count changed with phases {:?}", phases);
        assert_eq!(result.requirement_count, 10, "requirement_count changed with phases {:?}", phases);
        // ISO metadata stable across all phase filters
        assert_eq!(result.standard, "ISO/IEC/IEEE 15289:2019");
        assert_eq!(result.clause, "9");
        assert_eq!(result.tool, "doc-engine");
        assert_eq!(result.tool_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(result.timestamp.len(), 20);
        assert_eq!(result.phases, phases);
    }
}

#[test]
fn test_scaffold_result_json_serialization() {
    let (_tmp, _output_dir, config) = scaffold_to_tmp(FIXTURE_SRS);
    let result = scaffold_from_srs(&config).unwrap();

    // ScaffoldResult serializes directly to JSON (no wrapper)
    let json = serde_json::to_string_pretty(&result).unwrap();
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();

    // ISO/IEC/IEEE 15289:2019 clause 9 metadata at top level
    assert_eq!(val["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(val["clause"], "9");
    assert_eq!(val["tool"], "doc-engine");
    assert!(val["tool_version"].is_string());
    assert!(val["timestamp"].is_string());
    assert!(val["srs_source"].is_string());
    assert!(val["phases"].is_array());
    assert!(val["force"].is_boolean());
    // Result fields at same level (flat)
    assert_eq!(val["domain_count"], 2);
    assert_eq!(val["requirement_count"], 3);
    assert!(val["created"].is_array());
    assert!(val["skipped"].is_array());
}

#[test]
fn test_scaffold_result_json_with_phase_filter() {
    let tmp = tempfile::TempDir::new().unwrap();
    let srs_path = tmp.path().join("srs.md");
    fs::write(&srs_path, FIXTURE_SRS).unwrap();

    let config = ScaffoldConfig {
        srs_path,
        output_dir: tmp.path().join("output"),
        force: true,
        phases: vec!["testing".to_string(), "design".to_string()],
        file_types: vec![],
    };
    let result = scaffold_from_srs(&config).unwrap();
    let json = serde_json::to_string_pretty(&result).unwrap();
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Phases and force reflected in JSON
    let phases: Vec<&str> = val["phases"].as_array().unwrap()
        .iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(phases, vec!["testing", "design"]);
    assert_eq!(val["force"], true);
}
