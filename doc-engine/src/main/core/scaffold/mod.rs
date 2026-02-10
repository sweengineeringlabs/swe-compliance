pub(crate) mod types;
pub(crate) mod parser;
pub(crate) mod yaml_gen;
pub(crate) mod markdown_gen;

use std::fs;
use std::path::PathBuf;

use crate::spi::types::ScanError;
use types::{ScaffoldConfig, ScaffoldResult};

/// Generate SDLC spec file scaffold from an SRS document.
///
/// Reads the SRS, extracts domains and requirements, then generates:
/// - Per-domain `.spec.yaml`/`.spec`, `.arch.yaml`/`.arch`, `.test.yaml`/`.test`, `.deploy.yaml`/`.deploy`
/// - A BRD master inventory `brd.spec.yaml`/`brd.spec`
pub fn scaffold_from_srs(config: &ScaffoldConfig) -> Result<ScaffoldResult, ScanError> {
    let content = fs::read_to_string(&config.srs_path).map_err(|e| {
        ScanError::Path(format!(
            "cannot read SRS file '{}': {}",
            config.srs_path.display(),
            e
        ))
    })?;

    let domains = parser::parse_srs(&content)?;

    if domains.is_empty() {
        return Err(ScanError::Config(
            "no domains with requirements found in SRS".to_string(),
        ));
    }

    let mut result = ScaffoldResult {
        created: Vec::new(),
        skipped: Vec::new(),
        domain_count: domains.len(),
        requirement_count: domains.iter().map(|d| d.requirements.len()).sum(),
    };

    let include_phase = |phase: &str| -> bool {
        config.phases.is_empty() || config.phases.iter().any(|p| p == phase)
    };

    for domain in &domains {
        // Spec files: 4 YAML + 4 markdown + 2 exec per domain (filtered by phase)
        let mut files: Vec<(String, String)> = Vec::new();

        if include_phase("requirements") {
            files.push((
                format!("docs/1-requirements/{}/{}.spec.yaml", domain.slug, domain.slug),
                yaml_gen::generate_feature_spec_yaml(domain),
            ));
            files.push((
                format!("docs/1-requirements/{}/{}.spec", domain.slug, domain.slug),
                markdown_gen::generate_feature_spec_md(domain),
            ));
        }

        if include_phase("design") {
            files.push((
                format!("docs/3-design/{}/{}.arch.yaml", domain.slug, domain.slug),
                yaml_gen::generate_arch_spec_yaml(domain),
            ));
            files.push((
                format!("docs/3-design/{}/{}.arch", domain.slug, domain.slug),
                markdown_gen::generate_arch_spec_md(domain),
            ));
        }

        if include_phase("testing") {
            files.push((
                format!("docs/5-testing/{}/{}.test.yaml", domain.slug, domain.slug),
                yaml_gen::generate_test_spec_yaml(domain),
            ));
            files.push((
                format!("docs/5-testing/{}/{}.test", domain.slug, domain.slug),
                markdown_gen::generate_test_spec_md(domain),
            ));
            files.push((
                format!("docs/5-testing/{}/{}.manual.exec", domain.slug, domain.slug),
                markdown_gen::generate_manual_exec_md(domain),
            ));
            files.push((
                format!("docs/5-testing/{}/{}.auto.exec", domain.slug, domain.slug),
                markdown_gen::generate_auto_exec_md(domain),
            ));
        }

        if include_phase("deployment") {
            files.push((
                format!("docs/6-deployment/{}/{}.deploy.yaml", domain.slug, domain.slug),
                yaml_gen::generate_deploy_spec_yaml(domain),
            ));
            files.push((
                format!("docs/6-deployment/{}/{}.deploy", domain.slug, domain.slug),
                markdown_gen::generate_deploy_spec_md(domain),
            ));
        }

        for (rel_path, content) in files {
            write_file(&config.output_dir, &rel_path, &content, config.force, &mut result)?;
        }
    }

    // BRD master inventory (only when requirements phase is included)
    if include_phase("requirements") {
        let brd_files = vec![
            (
                "docs/1-requirements/brd.spec.yaml".to_string(),
                yaml_gen::generate_brd_yaml(&domains),
            ),
            (
                "docs/1-requirements/brd.spec".to_string(),
                markdown_gen::generate_brd_md(&domains),
            ),
        ];

        for (rel_path, content) in brd_files {
            write_file(&config.output_dir, &rel_path, &content, config.force, &mut result)?;
        }
    }

    Ok(result)
}

/// Write a file at `output_dir/rel_path`, creating parent dirs as needed.
/// If the file exists and `force` is false, skip it.
fn write_file(
    output_dir: &PathBuf,
    rel_path: &str,
    content: &str,
    force: bool,
    result: &mut ScaffoldResult,
) -> Result<(), ScanError> {
    let full_path = output_dir.join(rel_path);

    if full_path.exists() && !force {
        result.skipped.push(PathBuf::from(rel_path));
        return Ok(());
    }

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            ScanError::Path(format!(
                "cannot create directory '{}': {}",
                parent.display(),
                e
            ))
        })?;
    }

    fs::write(&full_path, content).map_err(|e| {
        ScanError::Io(e)
    })?;

    result.created.push(PathBuf::from(rel_path));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn fixture_srs() -> String {
        "\
### 4.1 Rule Loading

#### FR-100: Default rules

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 |
| **Acceptance** | Engine loads rules |

The binary embeds rules.
"
        .to_string()
    }

    fn setup_config(tmp: &Path) -> (ScaffoldConfig, PathBuf) {
        let srs_path = tmp.join("srs.md");
        fs::write(&srs_path, fixture_srs()).unwrap();
        let output_dir = tmp.join("output");
        let config = ScaffoldConfig {
            srs_path,
            output_dir: output_dir.clone(),
            force: false,
            phases: vec![],
        };
        (config, output_dir)
    }

    #[test]
    fn test_scaffold_creates_directories() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (config, output_dir) = setup_config(tmp.path());
        let result = scaffold_from_srs(&config).unwrap();

        assert!(output_dir.join("docs/1-requirements/rule_loading").is_dir());
        assert!(output_dir.join("docs/3-design/rule_loading").is_dir());
        assert!(output_dir.join("docs/5-testing/rule_loading").is_dir());
        assert!(output_dir.join("docs/6-deployment/rule_loading").is_dir());
        assert_eq!(result.domain_count, 1);
        assert_eq!(result.requirement_count, 1);
    }

    #[test]
    fn test_scaffold_creates_all_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (config, output_dir) = setup_config(tmp.path());
        let result = scaffold_from_srs(&config).unwrap();

        // 10 per domain + 2 BRD = 12
        assert_eq!(result.created.len(), 12);
        assert!(result.skipped.is_empty());

        // Check specific files
        assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml").exists());
        assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec").exists());
        assert!(output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml").exists());
        assert!(output_dir.join("docs/3-design/rule_loading/rule_loading.arch").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.test").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.manual.exec").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.auto.exec").exists());
        assert!(output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy.yaml").exists());
        assert!(output_dir.join("docs/6-deployment/rule_loading/rule_loading.deploy").exists());
        assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
        assert!(output_dir.join("docs/1-requirements/brd.spec").exists());
    }

    #[test]
    fn test_scaffold_skip_existing() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (config, _output_dir) = setup_config(tmp.path());

        // First run
        scaffold_from_srs(&config).unwrap();

        // Second run without --force
        let result = scaffold_from_srs(&config).unwrap();
        assert_eq!(result.skipped.len(), 12);
        assert!(result.created.is_empty());
    }

    #[test]
    fn test_scaffold_force_overwrite() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (mut config, _output_dir) = setup_config(tmp.path());

        // First run
        scaffold_from_srs(&config).unwrap();

        // Second run with --force
        config.force = true;
        let result = scaffold_from_srs(&config).unwrap();
        assert_eq!(result.created.len(), 12);
        assert!(result.skipped.is_empty());
    }

    #[test]
    fn test_scaffold_empty_srs_error() {
        let tmp = tempfile::TempDir::new().unwrap();
        let srs_path = tmp.path().join("empty.md");
        fs::write(&srs_path, "# SRS\n\nNo domains.\n").unwrap();

        let config = ScaffoldConfig {
            srs_path,
            output_dir: tmp.path().join("out"),
            force: false,
            phases: vec![],
        };

        let err = scaffold_from_srs(&config).unwrap_err();
        assert!(err.to_string().contains("no domains"));
    }

    #[test]
    fn test_scaffold_phase_filter_testing_only() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (mut config, output_dir) = setup_config(tmp.path());
        config.phases = vec!["testing".to_string()];

        let result = scaffold_from_srs(&config).unwrap();

        // 1 domain × 4 testing files (test.yaml, test, manual.exec, auto.exec) = 4
        assert_eq!(result.created.len(), 4);
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.test.yaml").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.test").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.manual.exec").exists());
        assert!(output_dir.join("docs/5-testing/rule_loading/rule_loading.auto.exec").exists());
        // No other phases
        assert!(!output_dir.join("docs/1-requirements/rule_loading").exists());
        assert!(!output_dir.join("docs/3-design/rule_loading").exists());
        assert!(!output_dir.join("docs/6-deployment/rule_loading").exists());
        assert!(!output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
    }

    #[test]
    fn test_scaffold_phase_filter_multiple() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (mut config, output_dir) = setup_config(tmp.path());
        config.phases = vec!["requirements".to_string(), "design".to_string()];

        let result = scaffold_from_srs(&config).unwrap();

        // 1 domain × (2 req + 2 design) + 2 BRD = 6
        assert_eq!(result.created.len(), 6);
        assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec.yaml").exists());
        assert!(output_dir.join("docs/1-requirements/rule_loading/rule_loading.spec").exists());
        assert!(output_dir.join("docs/3-design/rule_loading/rule_loading.arch.yaml").exists());
        assert!(output_dir.join("docs/3-design/rule_loading/rule_loading.arch").exists());
        assert!(output_dir.join("docs/1-requirements/brd.spec.yaml").exists());
        assert!(output_dir.join("docs/1-requirements/brd.spec").exists());
        // No testing or deployment
        assert!(!output_dir.join("docs/5-testing/rule_loading").exists());
        assert!(!output_dir.join("docs/6-deployment/rule_loading").exists());
    }

    #[test]
    fn test_scaffold_phase_filter_empty_means_all() {
        let tmp = tempfile::TempDir::new().unwrap();
        let (config, _output_dir) = setup_config(tmp.path());

        let result = scaffold_from_srs(&config).unwrap();
        // Empty phases = all phases: 1 domain × 10 + 2 BRD = 12
        assert_eq!(result.created.len(), 12);
    }
}
