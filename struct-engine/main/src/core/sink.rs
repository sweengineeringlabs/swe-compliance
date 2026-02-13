use std::path::PathBuf;

use crate::api::traits::{ReportSink, Reporter};
use crate::api::types::{ReportFormat, ScanError, ScanReport};
use crate::core::reporter::{TextReporter, JsonReporter};

/// Writes the formatted report to stdout.
pub struct StdoutSink {
    /// The output format to use.
    pub format: ReportFormat,
}

impl ReportSink for StdoutSink {
    fn emit(&self, report: &ScanReport) -> Result<(), ScanError> {
        let output = match self.format {
            ReportFormat::Text => TextReporter.report(report),
            ReportFormat::Json => JsonReporter.report(report),
        };
        print!("{}", output);
        Ok(())
    }
}

/// Writes the report as pretty-printed JSON to a file.
///
/// Creates parent directories if they do not exist.
pub struct FileSink {
    /// The file path to write the report to.
    pub path: PathBuf,
}

impl ReportSink for FileSink {
    fn emit(&self, report: &ScanReport) -> Result<(), ScanError> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| ScanError::Config(format!("JSON serialization failed: {}", e)))?;
        if let Some(parent) = self.path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(&self.path, &json)?;
        Ok(())
    }
}

/// Sends the report as JSON to a Kafka topic via the wire protocol.
#[cfg(feature = "kafka")]
pub struct KafkaSink {
    /// The Kafka broker address (e.g. "localhost:9092").
    pub broker: String,
    /// The Kafka topic to publish reports to.
    pub topic: String,
}

#[cfg(feature = "kafka")]
impl ReportSink for KafkaSink {
    fn emit(&self, report: &ScanReport) -> Result<(), ScanError> {
        let json = serde_json::to_string(report)
            .map_err(|e| ScanError::Config(format!("JSON serialization failed: {}", e)))?;
        let producer = kafka_sink::KafkaProducer::new(&self.broker, &self.topic);
        producer.produce(json.as_bytes())
            .map_err(|e| ScanError::Config(format!("Kafka produce failed: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{
        CheckEntry, CheckId, CheckResult, ProjectKind, ScanSummary,
    };

    fn make_report() -> ScanReport {
        ScanReport {
            results: vec![CheckEntry {
                id: CheckId(1),
                category: "structure".to_string(),
                description: "Cargo.toml exists".to_string(),
                result: CheckResult::Pass,
            }],
            summary: ScanSummary {
                total: 1,
                passed: 1,
                failed: 0,
                skipped: 0,
            },
            project_kind: ProjectKind::Library,
            member_reports: vec![],
        }
    }

    #[test]
    fn test_stdout_sink_text() {
        let sink = StdoutSink { format: ReportFormat::Text };
        let report = make_report();
        assert!(sink.emit(&report).is_ok());
    }

    #[test]
    fn test_stdout_sink_json() {
        let sink = StdoutSink { format: ReportFormat::Json };
        let report = make_report();
        assert!(sink.emit(&report).is_ok());
    }

    #[test]
    fn test_file_sink_creates_file() {
        let dir = std::env::temp_dir().join("struct_engine_sink_test_creates");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("report.json");

        let sink = FileSink { path: path.clone() };
        let report = make_report();
        sink.emit(&report).unwrap();

        assert!(path.exists());
        let contents = std::fs::read_to_string(&path).unwrap();
        let _: serde_json::Value = serde_json::from_str(&contents).unwrap();

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_sink_creates_parent_dirs() {
        let dir = std::env::temp_dir().join("struct_engine_sink_test_parents");
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join("a").join("b").join("report.json");

        let sink = FileSink { path: path.clone() };
        let report = make_report();
        sink.emit(&report).unwrap();

        assert!(path.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_sink_roundtrip() {
        let dir = std::env::temp_dir().join("struct_engine_sink_test_roundtrip");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("report.json");

        let sink = FileSink { path: path.clone() };
        let report = make_report();
        sink.emit(&report).unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        let deserialized: ScanReport = serde_json::from_str(&contents).unwrap();
        assert_eq!(deserialized.summary.total, 1);
        assert_eq!(deserialized.summary.passed, 1);
        assert_eq!(deserialized.summary.failed, 0);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(feature = "kafka")]
    #[test]
    fn test_kafka_sink_construction() {
        let sink = super::KafkaSink {
            broker: "localhost:9092".to_string(),
            topic: "test-topic".to_string(),
        };
        assert_eq!(sink.broker, "localhost:9092");
        assert_eq!(sink.topic, "test-topic");
    }

    #[cfg(feature = "kafka")]
    #[test]
    fn test_kafka_sink_emit_no_broker() {
        use crate::api::traits::ReportSink;
        let sink = super::KafkaSink {
            broker: "127.0.0.1:1".to_string(),
            topic: "test-topic".to_string(),
        };
        let report = make_report();
        let result = sink.emit(&report);
        assert!(result.is_err());
    }
}
