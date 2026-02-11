use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

use doc_engine_scan::{scan_with_config, format_report_text, format_report_json, ScanConfig, ProjectScope, ProjectType};
use doc_engine_scaffold::{scaffold_from_srs, ScaffoldConfig};

#[cfg(feature = "ai")]
use doc_engine_ai::{DocEngineAiConfig, DefaultDocEngineAiService, DocEngineAiService};

#[derive(Parser)]
#[command(name = "doc-engine", version, about = "Documentation compliance engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a project for documentation compliance
    Scan {
        /// Path to the project root
        path: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Run only specific checks (e.g., "1-13" or "1,5,10")
        #[arg(long)]
        checks: Option<String>,

        /// Project type: open-source or internal
        #[arg(long = "type", value_name = "TYPE")]
        project_type: Option<String>,

        /// Project scope: small, medium, or large
        #[arg(long)]
        scope: String,

        /// Path to custom rules file
        #[arg(long)]
        rules: Option<PathBuf>,

        /// Filter checks by SDLC phase/category (comma-separated, e.g. "testing,module")
        #[arg(long)]
        phase: Option<String>,

        /// Filter module checks to specific modules (comma-separated, e.g. "scan,cli")
        #[arg(long)]
        module: Option<String>,

        /// Save report to file (default: docs/7-operations/compliance/documentation_audit_report_v{version}.json)
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
    /// AI-powered compliance analysis (requires --features ai)
    #[cfg(feature = "ai")]
    Ai {
        #[command(subcommand)]
        action: AiAction,
    },
    /// Generate SDLC spec file scaffold from an SRS document
    Scaffold {
        /// Path to the SRS markdown file
        srs_path: PathBuf,

        /// Output directory (defaults to current directory)
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,

        /// Generate only specific SDLC phases (comma-separated: requirements,design,testing,deployment)
        #[arg(long)]
        phase: Option<String>,

        /// Generate only specific file types (comma-separated: yaml,spec,arch,test,exec,deploy)
        #[arg(long = "type", value_name = "TYPE")]
        file_type: Option<String>,

        /// Save scaffold report as JSON
        #[arg(long)]
        report: Option<PathBuf>,
    },
}

#[cfg(feature = "ai")]
#[derive(Subcommand)]
enum AiAction {
    /// Chat with the compliance auditor agent
    Chat {
        /// The message to send
        message: String,
    },
    /// Run an AI-powered compliance audit
    Audit {
        /// Path to the project root
        path: PathBuf,

        /// Project scope: small, medium, or large
        #[arg(long, default_value = "small")]
        scope: String,
    },
}

fn parse_checks(input: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() != 2 {
                return Err(format!("Invalid range: '{}'", part));
            }
            let start: u8 = bounds[0].trim().parse()
                .map_err(|_| format!("Invalid number: '{}'", bounds[0]))?;
            let end: u8 = bounds[1].trim().parse()
                .map_err(|_| format!("Invalid number: '{}'", bounds[1]))?;
            if start > end {
                return Err(format!("Invalid range: {} > {}", start, end));
            }
            for i in start..=end {
                result.push(i);
            }
        } else {
            let num: u8 = part.parse()
                .map_err(|_| format!("Invalid check number: '{}'", part))?;
            result.push(num);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single() {
        assert_eq!(parse_checks("5").unwrap(), vec![5]);
    }

    #[test]
    fn test_parse_comma_list() {
        assert_eq!(parse_checks("1,5,10").unwrap(), vec![1, 5, 10]);
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_checks("1-5").unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_mixed() {
        assert_eq!(parse_checks("1-3,7,10-12").unwrap(), vec![1, 2, 3, 7, 10, 11, 12]);
    }

    #[test]
    fn test_parse_invalid_range() {
        assert!(parse_checks("5-3").is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        assert!(parse_checks("abc").is_err());
    }

}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, json, checks, project_type, scope, rules, phase, module, output } => {
            // Canonicalize path early so auto-detection can read LICENSE
            let root = match path.canonicalize() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: cannot resolve path '{}': {}", path.display(), e);
                    process::exit(2);
                }
            };

            // Parse project type: explicit --type overrides, None = auto-detect from LICENSE
            let pt = match project_type.as_deref() {
                Some("internal") => Some(ProjectType::Internal),
                Some("open-source") | Some("open_source") => Some(ProjectType::OpenSource),
                None => None, // auto-detect from LICENSE in engine
                Some(other) => {
                    eprintln!("Error: unknown project type '{}' (use 'open-source' or 'internal')", other);
                    process::exit(2);
                }
            };

            // Parse project scope (required)
            let ps = match scope.as_str() {
                "small" => ProjectScope::Small,
                "medium" => ProjectScope::Medium,
                "large" => ProjectScope::Large,
                other => {
                    eprintln!("Error: unknown scope '{}' (use 'small', 'medium', or 'large')", other);
                    process::exit(2);
                }
            };

            // Parse check filter
            let check_ids = match checks {
                Some(ref s) => match parse_checks(s) {
                    Ok(ids) => Some(ids),
                    Err(e) => {
                        eprintln!("Error parsing --checks: {}", e);
                        process::exit(2);
                    }
                },
                None => None,
            };

            // Parse --phase filter
            let valid_phases = [
                "structure", "naming", "root_files", "content", "navigation", "cross_ref", "adr",
                "traceability", "ideation", "requirements", "planning", "design", "development",
                "testing", "deployment", "operations", "module", "backlog",
            ];
            let phases: Option<Vec<String>> = match phase {
                Some(ref s) => {
                    let parsed: Vec<String> = s.split(',').map(|p| p.trim().to_lowercase()).collect();
                    for p in &parsed {
                        if !valid_phases.contains(&p.as_str()) {
                            eprintln!("Error: unknown phase '{}' (valid: {})", p, valid_phases.join(", "));
                            process::exit(2);
                        }
                    }
                    Some(parsed)
                }
                None => None,
            };

            // Parse --module filter (comma-separated, case-sensitive)
            let module_filter: Option<Vec<String>> = match module {
                Some(ref s) => {
                    Some(s.split(',').map(|m| m.trim().to_string()).collect())
                }
                None => None,
            };

            let config = ScanConfig {
                project_type: pt,
                project_scope: ps,
                checks: check_ids,
                rules_path: rules,
                phases,
                module_filter,
            };

            match scan_with_config(&root, &config) {
                Ok(report) => {
                    let formatted = if json {
                        format_report_json(&report)
                    } else {
                        format_report_text(&report)
                    };
                    print!("{}", formatted);

                    // Persist report: use --output if provided, otherwise default to
                    // docs/7-operations/compliance/documentation_audit_report_v{version}.json
                    let out_path = output.unwrap_or_else(|| {
                        root.join(format!(
                            "docs/7-operations/compliance/documentation_audit_report_v{}.json",
                            report.tool_version
                        ))
                    });
                    let json_report = serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
                        eprintln!("Error: JSON serialization failed: {}", e);
                        process::exit(2);
                    });
                    if let Some(parent) = out_path.parent() {
                        if !parent.exists() {
                            if let Err(e) = std::fs::create_dir_all(parent) {
                                eprintln!("Error: cannot create directory '{}': {}", parent.display(), e);
                                process::exit(2);
                            }
                        }
                    }
                    if let Err(e) = std::fs::write(&out_path, &json_report) {
                        eprintln!("Error: cannot write report to '{}': {}", out_path.display(), e);
                        process::exit(2);
                    }
                    eprintln!("Report saved to {}", out_path.display());

                    if report.summary.failed > 0 {
                        process::exit(1);
                    } else {
                        process::exit(0);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(2);
                }
            }
        }
        #[cfg(feature = "ai")]
        Commands::Ai { action } => {
            let rt = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
                eprintln!("Error: failed to start async runtime: {}", e);
                process::exit(2);
            });

            rt.block_on(async {
                let config = DocEngineAiConfig::from_env();
                let service = match DefaultDocEngineAiService::new(config).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(2);
                    }
                };

                match action {
                    AiAction::Chat { message } => {
                        match service.chat(&message).await {
                            Ok(response) => println!("{}", response),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                    AiAction::Audit { path, scope } => {
                        let root = match path.canonicalize() {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("Error: cannot resolve path '{}': {}", path.display(), e);
                                process::exit(2);
                            }
                        };

                        match service.audit(root.to_str().unwrap_or_default(), &scope).await {
                            Ok(response) => {
                                println!("{}", response.summary);
                                if !response.recommendations.is_empty() {
                                    println!("\nRecommendations:");
                                    for rec in &response.recommendations {
                                        println!("  - {}", rec);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                }
            });
        }
        Commands::Scaffold { srs_path, output, force, phase, file_type, report } => {
            let srs_resolved = match srs_path.canonicalize() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: cannot resolve SRS path '{}': {}", srs_path.display(), e);
                    process::exit(2);
                }
            };

            let output_dir = output.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

            let valid_phases = ["requirements", "design", "testing", "deployment"];
            let phases: Vec<String> = match phase {
                Some(ref s) => {
                    let parsed: Vec<String> = s.split(',').map(|p| p.trim().to_lowercase()).collect();
                    for p in &parsed {
                        if !valid_phases.contains(&p.as_str()) {
                            eprintln!("Error: unknown phase '{}' (valid: {})", p, valid_phases.join(", "));
                            process::exit(2);
                        }
                    }
                    parsed
                }
                None => vec![],
            };

            let valid_types = ["yaml", "spec", "arch", "test", "exec", "deploy"];
            let file_types: Vec<String> = match file_type {
                Some(ref s) => {
                    let parsed: Vec<String> = s.split(',').map(|t| t.trim().to_lowercase()).collect();
                    for t in &parsed {
                        if !valid_types.contains(&t.as_str()) {
                            eprintln!("Error: unknown file type '{}' (valid: {})", t, valid_types.join(", "));
                            process::exit(2);
                        }
                    }
                    parsed
                }
                None => vec![],
            };

            let config = ScaffoldConfig {
                srs_path: srs_resolved,
                output_dir,
                force,
                phases,
                file_types,
            };

            match scaffold_from_srs(&config) {
                Ok(result) => {
                    for path in &result.created {
                        println!("  + {}", path.display());
                    }
                    for path in &result.skipped {
                        println!("  ~ {}", path.display());
                    }
                    println!(
                        "\nScaffold complete: {} domains, {} requirements, {} files created, {} skipped",
                        result.domain_count,
                        result.requirement_count,
                        result.created.len(),
                        result.skipped.len(),
                    );

                    if let Some(ref report_path) = report {
                        let json = serde_json::to_string_pretty(&result).unwrap_or_else(|e| {
                            eprintln!("Error: JSON serialization failed: {}", e);
                            process::exit(2);
                        });
                        if let Some(parent) = report_path.parent() {
                            if !parent.exists() {
                                if let Err(e) = std::fs::create_dir_all(parent) {
                                    eprintln!("Error: cannot create directory '{}': {}", parent.display(), e);
                                    process::exit(2);
                                }
                            }
                        }
                        if let Err(e) = std::fs::write(report_path, &json) {
                            eprintln!("Error: cannot write report to '{}': {}", report_path.display(), e);
                            process::exit(2);
                        }
                        eprintln!("Report saved to {}", report_path.display());
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(2);
                }
            }
        }
    }
}
