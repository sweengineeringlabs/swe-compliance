#[cfg(feature = "ai")]
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

use doc_engine_scan::{scan_with_config, ScanConfig, ProjectScope, ProjectType, StdoutSink, FileSink, ReportFormat};
use doc_engine_scan::api::traits::ReportSink;
#[cfg(feature = "kafka")]
use doc_engine_scan::{KafkaConfig, KafkaSink};
use doc_engine_scaffold::{scaffold_from_srs, ScaffoldConfig};

#[cfg(feature = "ai")]
use doc_engine_compliance_chat::{ComplianceChat, ComplianceChatConfig};
#[cfg(feature = "ai")]
use doc_engine_compliance_audit::{ComplianceAuditor, AuditConfig};
#[cfg(feature = "ai")]
use doc_engine_command_generator::{CommandGenerator, CommandGeneratorConfig, GenerateCommandsRequest, RequirementContext};

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

        /// Path to kafka.toml config file
        #[cfg(feature = "kafka")]
        #[arg(long = "kafka-config", value_name = "PATH")]
        kafka_config: Option<PathBuf>,

        /// Kafka broker address (overrides config/env)
        #[cfg(feature = "kafka")]
        #[arg(long = "kafka-broker", value_name = "ADDR")]
        kafka_broker: Option<String>,

        /// Kafka topic name (overrides config/env)
        #[cfg(feature = "kafka")]
        #[arg(long = "kafka-topic", value_name = "TOPIC")]
        kafka_topic: Option<String>,

        /// Kafka client ID (overrides config/env)
        #[cfg(feature = "kafka")]
        #[arg(long = "kafka-client-id", value_name = "ID")]
        kafka_client_id: Option<String>,

        /// Kafka partition (overrides config/env)
        #[cfg(feature = "kafka")]
        #[arg(long = "kafka-partition", value_name = "N")]
        kafka_partition: Option<i32>,

        /// Kafka produce timeout in ms (overrides config/env)
        #[cfg(feature = "kafka")]
        #[arg(long = "kafka-timeout", value_name = "MS")]
        kafka_timeout: Option<i32>,
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

        /// Generate only specific file types (comma-separated: yaml,spec,arch,test,exec,deploy,plan)
        #[arg(long = "type", value_name = "TYPE")]
        file_type: Option<String>,

        /// Include ONLY feature-gated domains (comma-separated, e.g. --feature ai)
        #[arg(long, conflicts_with = "exclude_feature")]
        feature: Option<String>,

        /// Exclude feature-gated domains (optionally specify features, e.g. --exclude-feature ai)
        #[arg(long, num_args = 0..=1, default_missing_value = "", conflicts_with = "feature")]
        exclude_feature: Option<String>,

        /// Path to a TOML command map (FR-ID → CLI command for test steps)
        #[arg(long = "command-map", value_name = "PATH")]
        command_map: Option<PathBuf>,

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
    /// Generate test commands for requirements missing them
    GenerateCommands {
        /// Path to the SRS markdown file
        srs_path: PathBuf,

        /// Output file (default: stdout)
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Existing command map to merge with (existing entries always win)
        #[arg(long)]
        merge: Option<PathBuf>,

        /// Process all requirements, not just those missing commands
        #[arg(long)]
        all: bool,
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

    #[cfg(feature = "ai")]
    #[test]
    fn test_needs_command_with_backtick_command() {
        // Acceptance with a command-like backtick span → already has a command
        assert!(needs_command("`cargo test -p scan rules`"));
        assert!(needs_command("Run `cargo test -p scan` and check output"));
    }

    #[cfg(feature = "ai")]
    #[test]
    fn test_needs_command_without_command() {
        // Descriptive text without command spans → needs AI command
        assert!(!needs_command("Engine loads embedded rules"));
        assert!(!needs_command("The system shall comply with standards"));
    }

    #[cfg(feature = "ai")]
    #[test]
    fn test_needs_command_with_non_command_backtick() {
        // Backtick span that is NOT a command (no space, or key-value)
        assert!(!needs_command("The `config` value must be set"));
        assert!(!needs_command("Uses `key: value` format"));
    }

    #[cfg(feature = "ai")]
    #[test]
    fn test_format_command_map_toml_sorted() {
        let mut map = HashMap::new();
        map.insert("FR-200".to_string(), "cargo test -p scan discovery".to_string());
        map.insert("FR-100".to_string(), "cargo test -p scan rules".to_string());
        let toml = format_command_map_toml(&map);

        assert!(toml.starts_with("[commands]\n"));
        // FR-100 should come before FR-200
        let pos_100 = toml.find("FR-100").unwrap();
        let pos_200 = toml.find("FR-200").unwrap();
        assert!(pos_100 < pos_200);

        // Should be valid TOML
        let parsed: toml::Table = toml.parse().expect("should be valid TOML");
        let cmds = parsed.get("commands").unwrap().as_table().unwrap();
        assert_eq!(cmds.get("FR-100").unwrap().as_str().unwrap(), "cargo test -p scan rules");
    }

}

/// Returns `true` if the acceptance text already contains a command-like backtick span,
/// meaning no AI-generated command is needed.
///
/// Replicates the `is_command_like` + `BacktickScanner` heuristic from scaffold's markdown_gen.
#[cfg(feature = "ai")]
fn needs_command(acceptance: &str) -> bool {
    // Scan for single-backtick spans that look like commands.
    let bytes = acceptance.as_bytes();
    let len = bytes.len();
    let mut pos = 0;
    while pos < len {
        if bytes[pos] == b'`' {
            // Skip code fences (``` or more)
            let tick_start = pos;
            while pos < len && bytes[pos] == b'`' {
                pos += 1;
            }
            let tick_count = pos - tick_start;
            if tick_count > 1 {
                // Multi-backtick: skip to closing sequence
                continue;
            }
            // Single backtick: find closing backtick
            let span_start = pos;
            while pos < len && bytes[pos] != b'`' {
                pos += 1;
            }
            if pos < len {
                let span = &acceptance[span_start..pos];
                pos += 1; // skip closing backtick
                if is_command_like(span) {
                    return true;
                }
            }
        } else {
            pos += 1;
        }
    }
    false
}

/// Check if a backtick span looks like a runnable CLI command.
///
/// Replicates scaffold's `is_command_like` heuristic.
#[cfg(feature = "ai")]
fn is_command_like(span: &str) -> bool {
    let bytes = span.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    // Must start with an ASCII letter
    if !bytes[0].is_ascii_alphabetic() {
        return false;
    }
    // Must contain at least one space (command + arguments)
    if !span.contains(' ') {
        return false;
    }
    // Must not look like a key-value pair
    if span.contains(": ") || span.contains("= ") {
        return false;
    }
    true
}

/// Format a command map as a sorted TOML string with a `[commands]` table.
#[cfg(feature = "ai")]
fn format_command_map_toml(map: &HashMap<String, String>) -> String {
    let mut sorted: Vec<(&String, &String)> = map.iter().collect();
    sorted.sort_by_key(|(k, _)| *k);

    let mut out = String::from("[commands]\n");
    for (id, cmd) in sorted {
        // Escape the command value for TOML string
        let escaped = cmd.replace('\\', "\\\\").replace('"', "\\\"");
        out.push_str(&format!("{} = \"{}\"\n", id, escaped));
    }
    out
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path, json, checks, project_type, scope, rules, phase, module, output,
            #[cfg(feature = "kafka")]
            kafka_config,
            #[cfg(feature = "kafka")]
            kafka_broker,
            #[cfg(feature = "kafka")]
            kafka_topic,
            #[cfg(feature = "kafka")]
            kafka_client_id,
            #[cfg(feature = "kafka")]
            kafka_partition,
            #[cfg(feature = "kafka")]
            kafka_timeout,
        } => {
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
                    let format = if json { ReportFormat::Json } else { ReportFormat::Text };
                    let stdout_sink = StdoutSink { format };
                    if let Err(e) = stdout_sink.emit(&report) {
                        eprintln!("Error: {}", e);
                        process::exit(2);
                    }

                    // Persist report: use --output if provided, otherwise default to
                    // docs/7-operations/compliance/documentation_audit_report_v{version}.json
                    let out_path = output.unwrap_or_else(|| {
                        root.join(format!(
                            "docs/7-operations/compliance/documentation_audit_report_v{}.json",
                            report.tool_version
                        ))
                    });
                    let file_sink = FileSink { path: out_path };
                    if let Err(e) = file_sink.emit(&report) {
                        eprintln!("Error: {}", e);
                        process::exit(2);
                    }
                    eprintln!("Report saved to {}", file_sink.path.display());

                    // Kafka sink: emit report if any kafka flag is present
                    #[cfg(feature = "kafka")]
                    {
                        let has_kafka = kafka_config.is_some()
                            || kafka_broker.is_some()
                            || kafka_topic.is_some()
                            || kafka_client_id.is_some()
                            || kafka_partition.is_some()
                            || kafka_timeout.is_some();

                        if has_kafka {
                            // Resolution order: file -> env -> CLI flags
                            let mut kconfig = match kafka_config {
                                Some(ref p) => match KafkaConfig::from_file(p) {
                                    Ok(c) => c,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        process::exit(2);
                                    }
                                },
                                None => KafkaConfig::default(),
                            };

                            kconfig.merge_env();

                            if let Some(ref v) = kafka_broker { kconfig.broker = v.clone(); }
                            if let Some(ref v) = kafka_topic { kconfig.topic = v.clone(); }
                            if let Some(ref v) = kafka_client_id { kconfig.client_id = v.clone(); }
                            if let Some(v) = kafka_partition { kconfig.partition = v; }
                            if let Some(v) = kafka_timeout { kconfig.timeout_ms = v; }

                            let kafka_sink = KafkaSink { config: kconfig };
                            if let Err(e) = kafka_sink.emit(&report) {
                                eprintln!("Kafka error: {}", e);
                                process::exit(2);
                            }
                        }
                    }

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
                match action {
                    AiAction::Chat { message } => {
                        let service = match ComplianceChat::new(ComplianceChatConfig::from_env()).await {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(2);
                            }
                        };
                        match service.chat(&message).await {
                            Ok(response) => println!("{}", response),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                    AiAction::Audit { path, scope } => {
                        let service = match ComplianceAuditor::new(AuditConfig::from_env()).await {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(2);
                            }
                        };
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
                    AiAction::GenerateCommands { srs_path, output, merge, all } => {
                        let service = match CommandGenerator::new(CommandGeneratorConfig::from_env()).await {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(2);
                            }
                        };

                        // 1. Read and parse the SRS.
                        let srs_content = match std::fs::read_to_string(&srs_path) {
                            Ok(c) => c,
                            Err(e) => {
                                eprintln!("Error: cannot read SRS '{}': {}", srs_path.display(), e);
                                process::exit(2);
                            }
                        };
                        let domains = match doc_engine_scaffold::parse_srs(&srs_content) {
                            Ok(d) => d,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(2);
                            }
                        };

                        // 2. Optionally load existing command map.
                        let existing_map: HashMap<String, String> = match merge {
                            Some(ref p) => match doc_engine_scaffold::load_command_map(p) {
                                Ok(m) => m,
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                    process::exit(2);
                                }
                            },
                            None => HashMap::new(),
                        };

                        // 3. Collect requirements needing commands.
                        let all_reqs: Vec<_> = domains.iter()
                            .flat_map(|d| d.requirements.iter())
                            .collect();

                        let reqs_for_ai: Vec<RequirementContext> = all_reqs.iter()
                            .filter(|r| {
                                if !all {
                                    // Skip if already in existing map
                                    if existing_map.contains_key(&r.id) {
                                        return false;
                                    }
                                    // Skip if backtick heuristic finds a command
                                    if let Some(ref acc) = r.acceptance {
                                        if needs_command(acc) {
                                            return false;
                                        }
                                    }
                                }
                                true
                            })
                            .map(|r| RequirementContext {
                                id: r.id.clone(),
                                title: r.title.clone(),
                                verification: r.verification.clone().unwrap_or_default(),
                                acceptance: r.acceptance.clone().unwrap_or_default(),
                                traces_to: r.traces_to.clone().unwrap_or_default(),
                                description: r.description.clone(),
                            })
                            .collect();

                        if reqs_for_ai.is_empty() {
                            eprintln!("All requirements already have commands.");
                            // Still output existing map if merging
                            if !existing_map.is_empty() {
                                let toml_out = format_command_map_toml(&existing_map);
                                if let Some(ref out_path) = output {
                                    if let Err(e) = std::fs::write(out_path, &toml_out) {
                                        eprintln!("Error: cannot write to '{}': {}", out_path.display(), e);
                                        process::exit(2);
                                    }
                                    eprintln!("Wrote {} entries to {}", existing_map.len(), out_path.display());
                                } else {
                                    print!("{}", toml_out);
                                }
                            }
                            process::exit(0);
                        }

                        eprintln!("Sending {} requirements to LLM...", reqs_for_ai.len());

                        // 4. Call the AI service.
                        let request = GenerateCommandsRequest {
                            requirements: reqs_for_ai,
                            project_context: "Rust workspace (doc-engine) with crates: \
                                scan (doc-engine-scan), scaffold (doc-engine-scaffold), \
                                ai (doc-engine-ai, feature-gated), cli (doc-engine-cli). \
                                Test framework: cargo test. Binary: doc-engine.".to_string(),
                        };

                        let response = match service.generate_commands(&request).await {
                            Ok(r) => r,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                process::exit(1);
                            }
                        };

                        // 5. Merge: existing entries always win.
                        let mut merged = existing_map.clone();
                        for (id, cmd) in &response.commands {
                            merged.entry(id.clone()).or_insert_with(|| cmd.clone());
                        }

                        // 6. Output sorted TOML.
                        let toml_out = format_command_map_toml(&merged);
                        if let Some(ref out_path) = output {
                            if let Err(e) = std::fs::write(out_path, &toml_out) {
                                eprintln!("Error: cannot write to '{}': {}", out_path.display(), e);
                                process::exit(2);
                            }
                            eprintln!("Wrote {} entries to {}", merged.len(), out_path.display());
                        } else {
                            print!("{}", toml_out);
                        }

                        // 7. Report skipped to stderr.
                        if !response.skipped.is_empty() {
                            eprintln!("\nSkipped {} requirements:", response.skipped.len());
                            for s in &response.skipped {
                                eprintln!("  {}: {}", s.id, s.reason);
                            }
                        }

                        eprintln!(
                            "\nGenerated {} commands, skipped {}",
                            response.commands.len(),
                            response.skipped.len()
                        );
                    }
                }
            });
        }
        Commands::Scaffold { srs_path, output, force, phase, file_type, feature, exclude_feature, command_map, report } => {
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

            let valid_types = ["yaml", "spec", "arch", "test", "exec", "deploy", "plan"];
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

            let features: Vec<String> = feature
                .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                .unwrap_or_default();
            let exclude_features: Option<Vec<String>> = exclude_feature.map(|s| {
                if s.is_empty() { vec![] }
                else { s.split(',').map(|t| t.trim().to_string()).collect() }
            });

            let config = ScaffoldConfig {
                srs_path: srs_resolved,
                output_dir,
                force,
                phases,
                file_types,
                features,
                exclude_features,
                command_map_path: command_map,
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
