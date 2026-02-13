use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::KafkaError;

/// Configuration for a Kafka producer connection.
///
/// Supports a three-layer resolution: config file -> env vars -> CLI flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub broker: String,
    pub topic: String,
    pub client_id: String,
    pub partition: i32,
    pub timeout_ms: i32,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            broker: "localhost:9092".to_string(),
            topic: "compliance-reports".to_string(),
            client_id: "compliance-scan".to_string(),
            partition: 0,
            timeout_ms: 30_000,
        }
    }
}

impl KafkaConfig {
    /// Load configuration from a TOML file.
    pub fn from_file(path: &Path) -> Result<Self, KafkaError> {
        let contents = std::fs::read_to_string(path).map_err(|e| {
            KafkaError::Protocol(format!("cannot read config file '{}': {}", path.display(), e))
        })?;
        let config: KafkaConfig = toml::from_str(&contents).map_err(|e| {
            KafkaError::Protocol(format!("invalid TOML in '{}': {}", path.display(), e))
        })?;
        Ok(config)
    }

    /// Build a config from environment variables, falling back to defaults.
    ///
    /// Reads: `KAFKA_BROKER`, `KAFKA_TOPIC`, `KAFKA_CLIENT_ID`, `KAFKA_PARTITION`, `KAFKA_TIMEOUT_MS`.
    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.merge_env();
        config
    }

    /// Overlay environment variables onto an existing config.
    ///
    /// Only overrides fields for which an env var is set and non-empty.
    pub fn merge_env(&mut self) {
        if let Ok(v) = std::env::var("KAFKA_BROKER") {
            if !v.is_empty() {
                self.broker = v;
            }
        }
        if let Ok(v) = std::env::var("KAFKA_TOPIC") {
            if !v.is_empty() {
                self.topic = v;
            }
        }
        if let Ok(v) = std::env::var("KAFKA_CLIENT_ID") {
            if !v.is_empty() {
                self.client_id = v;
            }
        }
        if let Ok(v) = std::env::var("KAFKA_PARTITION") {
            if let Ok(n) = v.parse::<i32>() {
                self.partition = n;
            }
        }
        if let Ok(v) = std::env::var("KAFKA_TIMEOUT_MS") {
            if let Ok(n) = v.parse::<i32>() {
                self.timeout_ms = n;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::Mutex;

    // Env var tests must run serially since they modify process-wide state.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_config() {
        let config = KafkaConfig::default();
        assert_eq!(config.broker, "localhost:9092");
        assert_eq!(config.topic, "compliance-reports");
        assert_eq!(config.client_id, "compliance-scan");
        assert_eq!(config.partition, 0);
        assert_eq!(config.timeout_ms, 30_000);
    }

    #[test]
    fn test_from_env() {
        let _guard = ENV_LOCK.lock().unwrap();

        // Set env vars for this test
        std::env::set_var("KAFKA_BROKER", "env-broker:9093");
        std::env::set_var("KAFKA_TOPIC", "env-topic");
        std::env::set_var("KAFKA_CLIENT_ID", "env-client");
        std::env::set_var("KAFKA_PARTITION", "2");
        std::env::set_var("KAFKA_TIMEOUT_MS", "5000");

        let config = KafkaConfig::from_env();
        assert_eq!(config.broker, "env-broker:9093");
        assert_eq!(config.topic, "env-topic");
        assert_eq!(config.client_id, "env-client");
        assert_eq!(config.partition, 2);
        assert_eq!(config.timeout_ms, 5000);

        // Clean up
        std::env::remove_var("KAFKA_BROKER");
        std::env::remove_var("KAFKA_TOPIC");
        std::env::remove_var("KAFKA_CLIENT_ID");
        std::env::remove_var("KAFKA_PARTITION");
        std::env::remove_var("KAFKA_TIMEOUT_MS");
    }

    #[test]
    fn test_merge_env_overrides() {
        let _guard = ENV_LOCK.lock().unwrap();

        let mut config = KafkaConfig {
            broker: "file-broker:9092".to_string(),
            topic: "file-topic".to_string(),
            client_id: "file-client".to_string(),
            partition: 1,
            timeout_ms: 10_000,
        };

        // Only override broker and topic via env
        std::env::set_var("KAFKA_BROKER", "env-broker:9094");
        std::env::set_var("KAFKA_TOPIC", "env-topic-override");
        std::env::remove_var("KAFKA_CLIENT_ID");
        std::env::remove_var("KAFKA_PARTITION");
        std::env::remove_var("KAFKA_TIMEOUT_MS");

        config.merge_env();

        assert_eq!(config.broker, "env-broker:9094");
        assert_eq!(config.topic, "env-topic-override");
        // These should remain unchanged from the file config
        assert_eq!(config.client_id, "file-client");
        assert_eq!(config.partition, 1);
        assert_eq!(config.timeout_ms, 10_000);

        // Clean up
        std::env::remove_var("KAFKA_BROKER");
        std::env::remove_var("KAFKA_TOPIC");
    }

    #[test]
    fn test_from_file_valid() {
        let dir = std::env::temp_dir().join("kafka_config_test_valid");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("kafka.toml");

        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, r#"broker = "kafka.prod:9092""#).unwrap();
        writeln!(f, r#"topic = "prod-reports""#).unwrap();
        writeln!(f, r#"client_id = "prod-scanner""#).unwrap();
        writeln!(f, "partition = 3").unwrap();
        writeln!(f, "timeout_ms = 15000").unwrap();

        let config = KafkaConfig::from_file(&path).unwrap();
        assert_eq!(config.broker, "kafka.prod:9092");
        assert_eq!(config.topic, "prod-reports");
        assert_eq!(config.client_id, "prod-scanner");
        assert_eq!(config.partition, 3);
        assert_eq!(config.timeout_ms, 15_000);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_from_file_missing() {
        let result = KafkaConfig::from_file(Path::new("/nonexistent/kafka.toml"));
        assert!(result.is_err());
    }
}
