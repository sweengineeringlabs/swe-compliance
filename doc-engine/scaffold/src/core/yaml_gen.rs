use serde::Serialize;

use crate::api::types::SrsDomain;

// --- Serializable YAML structs ---

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FeatureSpec {
    kind: String,
    domain: String,
    section: String,
    requirements: Vec<FeatureReq>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FeatureReq {
    id: String,
    source_id: String,
    title: String,
    priority: String,
    status: String,
    verification: String,
    acceptance: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ArchSpec {
    kind: String,
    domain: String,
    section: String,
    spec_ref: String,
    components: Vec<ArchComponent>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ArchComponent {
    name: String,
    traces_to: String,
    description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestSpec {
    kind: String,
    domain: String,
    section: String,
    spec_ref: String,
    test_cases: Vec<TestCase>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestCase {
    id: String,
    test: String,
    verifies: String,
    priority: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeploySpec {
    kind: String,
    domain: String,
    section: String,
    spec_ref: String,
    environments: Vec<DeployEnv>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeployEnv {
    name: String,
    description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BrdSpec {
    kind: String,
    title: String,
    domains: Vec<BrdDomainEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BrdDomainEntry {
    section: String,
    domain: String,
    slug: String,
    spec_count: usize,
    spec_file: String,
    arch_file: String,
    test_file: String,
    deploy_file: String,
}

// --- Generator functions ---

/// Generate a `.spec.yaml` file for a domain.
pub(crate) fn generate_feature_spec_yaml(domain: &SrsDomain) -> String {
    let spec = FeatureSpec {
        kind: "feature_request".to_string(),
        domain: domain.title.clone(),
        section: domain.section.clone(),
        requirements: domain
            .requirements
            .iter()
            .enumerate()
            .map(|(idx, req)| {
                let req_id = format!("REQ-{:03}", idx + 1);
                FeatureReq {
                    id: req_id,
                    source_id: req.id.clone(),
                    title: req.title.clone(),
                    priority: req.priority.clone().unwrap_or_else(|| "Unknown".to_string()),
                    status: req.state.clone().unwrap_or_else(|| "Proposed".to_string()),
                    verification: req.verification.clone().unwrap_or_else(|| "Test".to_string()),
                    acceptance: req.acceptance.clone().unwrap_or_default(),
                }
            })
            .collect(),
    };
    serde_yml::to_string(&spec).unwrap_or_default()
}

/// Generate an `.arch.yaml` file for a domain.
pub(crate) fn generate_arch_spec_yaml(domain: &SrsDomain) -> String {
    let spec = ArchSpec {
        kind: "architecture".to_string(),
        domain: domain.title.clone(),
        section: domain.section.clone(),
        spec_ref: format!("docs/1-requirements/{}/{}.spec.yaml", domain.slug, domain.slug),
        components: domain
            .requirements
            .iter()
            .filter(|r| r.traces_to.is_some())
            .map(|req| ArchComponent {
                name: format!("{} handler", req.id),
                traces_to: req.traces_to.clone().unwrap_or_default(),
                description: if req.description.is_empty() {
                    req.title.clone()
                } else {
                    req.description.lines().next().unwrap_or("").to_string()
                },
            })
            .collect(),
    };
    serde_yml::to_string(&spec).unwrap_or_default()
}

/// Generate a `.test.yaml` file for a domain.
pub(crate) fn generate_test_spec_yaml(domain: &SrsDomain) -> String {
    let spec = TestSpec {
        kind: "test_plan".to_string(),
        domain: domain.title.clone(),
        section: domain.section.clone(),
        spec_ref: format!("docs/1-requirements/{}/{}.spec.yaml", domain.slug, domain.slug),
        test_cases: domain
            .requirements
            .iter()
            .enumerate()
            .map(|(idx, req)| {
                let tc_id = format!("TC-{:03}", idx + 1);
                let req_id = format!("REQ-{:03}", idx + 1);
                let method = req.verification.clone().unwrap_or_else(|| "Test".to_string());
                TestCase {
                    id: tc_id,
                    test: format!("{}: {} ({})", req.id, req.title, method),
                    verifies: req_id,
                    priority: req.priority.clone().unwrap_or_else(|| "Unknown".to_string()),
                }
            })
            .collect(),
    };
    serde_yml::to_string(&spec).unwrap_or_default()
}

/// Generate a `.deploy.yaml` file for a domain.
pub(crate) fn generate_deploy_spec_yaml(domain: &SrsDomain) -> String {
    let spec = DeploySpec {
        kind: "deployment".to_string(),
        domain: domain.title.clone(),
        section: domain.section.clone(),
        spec_ref: format!("docs/1-requirements/{}/{}.spec.yaml", domain.slug, domain.slug),
        environments: vec![
            DeployEnv {
                name: "staging".to_string(),
                description: format!("Staging environment for {} validation", domain.title),
            },
            DeployEnv {
                name: "production".to_string(),
                description: format!("Production environment for {}", domain.title),
            },
        ],
    };
    serde_yml::to_string(&spec).unwrap_or_default()
}

/// Generate a `brd.spec.yaml` file covering all domains.
pub(crate) fn generate_brd_yaml(domains: &[SrsDomain]) -> String {
    let spec = BrdSpec {
        kind: "brd".to_string(),
        title: "Business Requirements Document".to_string(),
        domains: domains
            .iter()
            .map(|d| BrdDomainEntry {
                section: d.section.clone(),
                domain: d.title.clone(),
                slug: d.slug.clone(),
                spec_count: d.requirements.len(),
                spec_file: format!("docs/1-requirements/{}/{}.spec.yaml", d.slug, d.slug),
                arch_file: format!("docs/3-design/{}/{}.arch.yaml", d.slug, d.slug),
                test_file: format!("docs/5-testing/{}/{}.test.yaml", d.slug, d.slug),
                deploy_file: format!("docs/6-deployment/{}/{}.deploy.yaml", d.slug, d.slug),
            })
            .collect(),
    };
    serde_yml::to_string(&spec).unwrap_or_default()
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
            requirements: vec![
                SrsRequirement {
                    id: "FR-100".to_string(),
                    title: "Default rules".to_string(),
                    kind: ReqKind::Functional,
                    priority: Some("Must".to_string()),
                    state: Some("Approved".to_string()),
                    verification: Some("Test".to_string()),
                    traces_to: Some("STK-01 -> core/rules.rs".to_string()),
                    acceptance: Some("Engine loads embedded rules".to_string()),
                    description: "The binary shall embed rules.".to_string(),
                },
                SrsRequirement {
                    id: "FR-101".to_string(),
                    title: "External rules".to_string(),
                    kind: ReqKind::Functional,
                    priority: Some("Must".to_string()),
                    state: Some("Approved".to_string()),
                    verification: Some("Test".to_string()),
                    traces_to: Some("STK-02".to_string()),
                    acceptance: Some("External rules override".to_string()),
                    description: "Load external file.".to_string(),
                },
            ],
            feature_gate: None,
        }
    }

    #[test]
    fn test_feature_spec_yaml_valid() {
        let yaml = generate_feature_spec_yaml(&sample_domain());
        assert!(yaml.contains("kind: feature_request"));
        assert!(yaml.contains("domain: Rule Loading"));
        assert!(yaml.contains("REQ-001"));
        assert!(yaml.contains("REQ-002"));
        assert!(yaml.contains("FR-100"));
        // Verify it parses back
        let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(val["kind"], "feature_request");
    }

    #[test]
    fn test_arch_spec_yaml_valid() {
        let yaml = generate_arch_spec_yaml(&sample_domain());
        assert!(yaml.contains("kind: architecture"));
        assert!(yaml.contains("specRef:"));
        assert!(yaml.contains("FR-100 handler"));
        let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(val["kind"], "architecture");
    }

    #[test]
    fn test_test_spec_yaml_valid() {
        let yaml = generate_test_spec_yaml(&sample_domain());
        assert!(yaml.contains("kind: test_plan"));
        assert!(yaml.contains("TC-001"));
        assert!(yaml.contains("verifies: REQ-001"));
        let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(val["kind"], "test_plan");
    }

    #[test]
    fn test_deploy_spec_yaml_valid() {
        let yaml = generate_deploy_spec_yaml(&sample_domain());
        assert!(yaml.contains("kind: deployment"));
        assert!(yaml.contains("staging"));
        assert!(yaml.contains("production"));
        let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(val["kind"], "deployment");
    }

    #[test]
    fn test_brd_yaml_valid() {
        let domains = vec![sample_domain()];
        let yaml = generate_brd_yaml(&domains);
        assert!(yaml.contains("kind: brd"));
        assert!(yaml.contains("rule_loading"));
        assert!(yaml.contains("specCount: 2"));
        let val: serde_yml::Value = serde_yml::from_str(&yaml).unwrap();
        assert_eq!(val["kind"], "brd");
    }
}
