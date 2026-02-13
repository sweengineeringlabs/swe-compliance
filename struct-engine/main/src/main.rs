use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

use struct_engine::{scan_with_config, ScanConfig, ProjectKind, StdoutSink, ReportFormat};
use struct_engine::api::traits::ReportSink;
#[cfg(feature = "kafka")]
use struct_engine::{KafkaConfig, KafkaSink};

#[derive(Parser)]
#[command(name = "struct-engine", version, about = "Rust package structure compliance engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a Rust project for structure compliance
    Scan {
        /// Path to the project root
        path: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Run only specific checks (e.g., "1-13" or "1,5,10")
        #[arg(long)]
        checks: Option<String>,

        /// Project kind: library, binary, both, or workspace
        #[arg(long, value_name = "KIND")]
        kind: Option<String>,

        /// Path to custom rules file
        #[arg(long)]
        rules: Option<PathBuf>,

        /// Recursively scan workspace members
        #[arg(long)]
        recursive: bool,

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
        Commands::Scan {
            path, json, checks, kind, rules, recursive,
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
            // Canonicalize path early so auto-detection can read Cargo.toml
            let root = match path.canonicalize() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: cannot resolve path '{}': {}", path.display(), e);
                    process::exit(2);
                }
            };

            // Parse project kind: explicit --kind overrides auto-detection
            let pk = match kind.as_deref() {
                Some("library") | Some("lib") => Some(ProjectKind::Library),
                Some("binary") | Some("bin") => Some(ProjectKind::Binary),
                Some("both") => Some(ProjectKind::Both),
                Some("workspace") => Some(ProjectKind::Workspace),
                None => None, // auto-detect from Cargo.toml
                Some(other) => {
                    eprintln!("Error: unknown project kind '{}' (use 'library', 'binary', 'both', or 'workspace')", other);
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
                project_kind: pk,
                checks: check_ids,
                rules_path: rules,
                recursive,
            };

            match scan_with_config(&root, &config) {
                Ok(report) => {
                    let format = if json { ReportFormat::Json } else { ReportFormat::Text };
                    let stdout_sink = StdoutSink { format };
                    if let Err(e) = stdout_sink.emit(&report) {
                        eprintln!("Error: {}", e);
                        process::exit(2);
                    }

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
    }
}
