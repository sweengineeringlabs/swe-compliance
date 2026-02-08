use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

use doc_engine::{scan_with_config, format_report_text, format_report_json, ScanConfig, ProjectType};

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

        /// Path to custom rules file
        #[arg(long)]
        rules: Option<PathBuf>,
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, json, checks, project_type, rules } => {
            // Parse project type
            let pt = match project_type.as_deref() {
                Some("internal") => ProjectType::Internal,
                Some("open-source") | Some("open_source") | None => ProjectType::OpenSource,
                Some(other) => {
                    eprintln!("Error: unknown project type '{}' (use 'open-source' or 'internal')", other);
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

            let config = ScanConfig {
                project_type: pt,
                checks: check_ids,
                rules_path: rules,
            };

            // Canonicalize path
            let root = match path.canonicalize() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: cannot resolve path '{}': {}", path.display(), e);
                    process::exit(2);
                }
            };

            match scan_with_config(&root, &config) {
                Ok(report) => {
                    let output = if json {
                        format_report_json(&report)
                    } else {
                        format_report_text(&report)
                    };
                    print!("{}", output);

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
    }
}
