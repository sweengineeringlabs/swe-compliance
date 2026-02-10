use regex::Regex;

use crate::spi::types::ScanError;
use super::types::{ReqKind, SrsDomain, SrsRequirement};

/// Slugify a title: lowercase, replace non-alphanumeric runs with `_`, trim edges.
pub(crate) fn slugify(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    let mut prev_underscore = true; // prevent leading underscore
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            prev_underscore = false;
        } else if !prev_underscore {
            slug.push('_');
            prev_underscore = true;
        }
    }
    // trim trailing underscore
    if slug.ends_with('_') {
        slug.pop();
    }
    slug
}

/// Extract a capture-group value from a single line using the given regex.
fn extract_attr(line: &str, re: &Regex) -> Option<String> {
    re.captures(line).map(|c| c[1].trim().to_string())
}

/// Parse an SRS markdown document into a list of domains with their requirements.
///
/// Sections without any FR/NFR blocks are dropped (no empty spec files).
pub fn parse_srs(content: &str) -> Result<Vec<SrsDomain>, ScanError> {
    let section_heading_re = Regex::new(r"^###\s+(\d+\.\d+)\s+(.+)$").unwrap();
    let fr_heading_re = Regex::new(r"^####\s+((?:FR|NFR)-\d+):\s+(.+)$").unwrap();
    let any_heading_re = Regex::new(r"^#{1,4}\s+").unwrap();

    let priority_re = Regex::new(r"\|\s*\*\*Priority\*\*\s*\|\s*(.+?)\s*\|").unwrap();
    let state_re = Regex::new(r"\|\s*\*\*State\*\*\s*\|\s*(.+?)\s*\|").unwrap();
    let verification_re = Regex::new(r"\|\s*\*\*Verification\*\*\s*\|\s*(.+?)\s*\|").unwrap();
    let traces_re = Regex::new(r"\|\s*\*\*(?:Traces\s+to|Traceability)\*\*\s*\|\s*(.+?)\s*\|").unwrap();
    let acceptance_re = Regex::new(r"\|\s*\*\*Acceptance\*\*\s*\|\s*(.+?)\s*\|").unwrap();
    let table_line_re = Regex::new(r"^\s*\|").unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut domains: Vec<SrsDomain> = Vec::new();
    let mut current_domain: Option<SrsDomain> = None;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // Check for section heading: ### X.Y Title
        if let Some(caps) = section_heading_re.captures(line) {
            // Save previous domain if it had requirements
            if let Some(domain) = current_domain.take() {
                if !domain.requirements.is_empty() {
                    domains.push(domain);
                }
            }
            let section = caps[1].to_string();
            let title = caps[2].trim().to_string();
            let slug = slugify(&title);
            current_domain = Some(SrsDomain {
                section,
                title,
                slug,
                requirements: Vec::new(),
            });
            i += 1;
            continue;
        }

        // Check for FR/NFR heading: #### FR-100: Title
        if let Some(caps) = fr_heading_re.captures(line) {
            let id = caps[1].to_string();
            let title = caps[2].trim().to_string();
            let kind = if id.starts_with("NFR") {
                ReqKind::NonFunctional
            } else {
                ReqKind::Functional
            };

            // Collect block body until next heading
            i += 1;
            let mut priority = None;
            let mut state = None;
            let mut verification = None;
            let mut traces_to = None;
            let mut acceptance = None;
            let mut narrative_lines: Vec<&str> = Vec::new();
            let mut past_table = false;

            while i < lines.len() {
                let bline = lines[i];
                // Stop at any heading
                if any_heading_re.is_match(bline) {
                    break;
                }

                // Extract attributes from table lines
                if table_line_re.is_match(bline) {
                    if priority.is_none() {
                        priority = extract_attr(bline, &priority_re);
                    }
                    if state.is_none() {
                        state = extract_attr(bline, &state_re);
                    }
                    if verification.is_none() {
                        verification = extract_attr(bline, &verification_re);
                    }
                    if traces_to.is_none() {
                        traces_to = extract_attr(bline, &traces_re);
                    }
                    if acceptance.is_none() {
                        acceptance = extract_attr(bline, &acceptance_re);
                    }
                } else if !bline.trim().is_empty() {
                    past_table = true;
                } else if past_table {
                    // blank line after narrative still counts
                }

                if past_table && !table_line_re.is_match(bline) {
                    narrative_lines.push(bline);
                }

                i += 1;
            }

            let description = narrative_lines
                .into_iter()
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();

            let req = SrsRequirement {
                id,
                title,
                kind,
                priority,
                state,
                verification,
                traces_to,
                acceptance,
                description,
            };

            if let Some(ref mut domain) = current_domain {
                domain.requirements.push(req);
            }
            // Don't increment i; the while loop already advanced past the block
            continue;
        }

        i += 1;
    }

    // Save final domain
    if let Some(domain) = current_domain.take() {
        if !domain.requirements.is_empty() {
            domains.push(domain);
        }
    }

    Ok(domains)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_basic() {
        assert_eq!(slugify("Rule Loading"), "rule_loading");
    }

    #[test]
    fn test_slugify_special_chars() {
        assert_eq!(slugify("CI/CD & Deployment"), "ci_cd_deployment");
    }

    #[test]
    fn test_slugify_numbers() {
        assert_eq!(slugify("Phase 3 Design"), "phase_3_design");
    }

    #[test]
    fn test_slugify_collapses_runs() {
        assert_eq!(slugify("Foo---Bar"), "foo_bar");
    }

    #[test]
    fn test_parse_single_domain() {
        let srs = "\
### 4.1 Rule Loading

#### FR-100: Default rules embedded in binary

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Approved |
| **Verification** | Test |
| **Traces to** | STK-01 |
| **Acceptance** | Engine loads embedded rules |

The binary shall embed a default rules.toml.
";
        let domains = parse_srs(srs).unwrap();
        assert_eq!(domains.len(), 1);
        assert_eq!(domains[0].section, "4.1");
        assert_eq!(domains[0].title, "Rule Loading");
        assert_eq!(domains[0].slug, "rule_loading");
        assert_eq!(domains[0].requirements.len(), 1);

        let req = &domains[0].requirements[0];
        assert_eq!(req.id, "FR-100");
        assert_eq!(req.title, "Default rules embedded in binary");
        assert_eq!(req.kind, ReqKind::Functional);
        assert_eq!(req.priority.as_deref(), Some("Must"));
        assert_eq!(req.state.as_deref(), Some("Approved"));
        assert_eq!(req.verification.as_deref(), Some("Test"));
        assert_eq!(req.traces_to.as_deref(), Some("STK-01"));
        assert_eq!(req.acceptance.as_deref(), Some("Engine loads embedded rules"));
        assert!(req.description.contains("binary shall embed"));
    }

    #[test]
    fn test_parse_multiple_domains() {
        let srs = "\
### 4.1 Rule Loading

#### FR-100: Default rules

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Rule loading desc.

### 4.2 File Discovery

#### FR-200: Recursive scanning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Scanner desc.
";
        let domains = parse_srs(srs).unwrap();
        assert_eq!(domains.len(), 2);
        assert_eq!(domains[0].slug, "rule_loading");
        assert_eq!(domains[1].slug, "file_discovery");
    }

    #[test]
    fn test_parse_mixed_fr_nfr() {
        let srs = "\
### 5.1 Architecture

#### NFR-100: SEA compliance

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Must follow SEA.

#### FR-800: Module discovery

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |

Discover modules.
";
        let domains = parse_srs(srs).unwrap();
        assert_eq!(domains.len(), 1);
        assert_eq!(domains[0].requirements.len(), 2);
        assert_eq!(domains[0].requirements[0].kind, ReqKind::NonFunctional);
        assert_eq!(domains[0].requirements[1].kind, ReqKind::Functional);
    }

    #[test]
    fn test_parse_partial_attributes() {
        let srs = "\
### 4.1 Minimal

#### FR-100: Minimal req

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |

Just a description.
";
        let domains = parse_srs(srs).unwrap();
        let req = &domains[0].requirements[0];
        assert_eq!(req.priority.as_deref(), Some("Should"));
        assert!(req.state.is_none());
        assert!(req.verification.is_none());
        assert!(req.traces_to.is_none());
        assert!(req.acceptance.is_none());
    }

    #[test]
    fn test_parse_narrative_extraction() {
        let srs = "\
### 4.1 Test

#### FR-100: With narrative

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

First paragraph of description.

Second paragraph continues.
";
        let domains = parse_srs(srs).unwrap();
        let req = &domains[0].requirements[0];
        assert!(req.description.contains("First paragraph"));
        assert!(req.description.contains("Second paragraph"));
    }

    #[test]
    fn test_parse_empty_srs() {
        let srs = "# Software Requirements Specification\n\nNo sections here.\n";
        let domains = parse_srs(srs).unwrap();
        assert!(domains.is_empty());
    }

    #[test]
    fn test_parse_section_without_frs_skipped() {
        let srs = "\
### 1.1 Purpose

This section has no FR/NFR blocks.

### 4.1 Rule Loading

#### FR-100: Default rules

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

Desc.
";
        let domains = parse_srs(srs).unwrap();
        assert_eq!(domains.len(), 1);
        assert_eq!(domains[0].section, "4.1");
    }

    #[test]
    fn test_parse_traces_to_with_traceability_keyword() {
        let srs = "\
### 4.1 Test

#### FR-100: Trace test

| Attribute | Value |
|-----------|-------|
| **Traceability** | STK-02 -> core/mod.rs |

Desc.
";
        let domains = parse_srs(srs).unwrap();
        assert_eq!(
            domains[0].requirements[0].traces_to.as_deref(),
            Some("STK-02 -> core/mod.rs")
        );
    }

    #[test]
    fn test_parse_multiple_reqs_per_domain() {
        let srs = "\
### 4.1 Rule Loading

#### FR-100: First

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |

First desc.

#### FR-101: Second

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |

Second desc.

#### FR-102: Third

| Attribute | Value |
|-----------|-------|
| **Priority** | May |

Third desc.
";
        let domains = parse_srs(srs).unwrap();
        assert_eq!(domains[0].requirements.len(), 3);
        assert_eq!(domains[0].requirements[0].id, "FR-100");
        assert_eq!(domains[0].requirements[1].id, "FR-101");
        assert_eq!(domains[0].requirements[2].id, "FR-102");
    }
}
