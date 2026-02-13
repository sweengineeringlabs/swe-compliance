/// Scan engine variants supported by the system.
pub enum ScanEngine {
    DocEngine,
    StructEngine,
}

impl ScanEngine {
    /// Returns the API-compatible string identifier for this engine.
    pub fn as_str(&self) -> &str {
        match self {
            ScanEngine::DocEngine => "doc-engine",
            ScanEngine::StructEngine => "struct-engine",
        }
    }

    /// Parse an engine identifier from a string. Returns None for unknown values.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "doc-engine" => Some(ScanEngine::DocEngine),
            "struct-engine" => Some(ScanEngine::StructEngine),
            _ => None,
        }
    }

    /// Human-readable display label.
    pub fn label(&self) -> &str {
        match self {
            ScanEngine::DocEngine => "Doc Engine",
            ScanEngine::StructEngine => "Struct Engine",
        }
    }
}

/// Runtime status of a scan.
pub enum ScanStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}

impl ScanStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ScanStatus::Queued => "queued",
            ScanStatus::InProgress => "in_progress",
            ScanStatus::Completed => "completed",
            ScanStatus::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "queued" => Some(ScanStatus::Queued),
            "in_progress" => Some(ScanStatus::InProgress),
            "completed" => Some(ScanStatus::Completed),
            "failed" => Some(ScanStatus::Failed),
            _ => None,
        }
    }

    /// Badge variant for rsc-ui rendering.
    pub fn badge_variant(&self) -> &str {
        match self {
            ScanStatus::Queued => "default",
            ScanStatus::InProgress => "info",
            ScanStatus::Completed => "success",
            ScanStatus::Failed => "danger",
        }
    }
}

/// A completed or in-progress scan record returned by the API.
pub struct Scan {
    pub id: String,
    pub project_id: String,
    pub engine: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub report: Option<String>,
}

impl Scan {
    /// Parse a Scan from a JSON value.
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(Scan {
            id: value.get_str("id")?.into(),
            project_id: value.get_str("project_id")?.into(),
            engine: value.get_str("engine")?.into(),
            status: value.get_str("status")?.into(),
            started_at: value.get_str("started_at")?.into(),
            finished_at: value.get_str("finished_at").map(|s| s.into()),
            report: value.get_str("report").map(|s| s.into()),
        })
    }

    /// Compute duration string between started_at and finished_at.
    /// Returns "--" if the scan has not finished.
    pub fn duration_display(&self) -> String {
        match &self.finished_at {
            Some(finished) => {
                let start_ms = parse_iso_timestamp_ms(&self.started_at).unwrap_or(0);
                let end_ms = parse_iso_timestamp_ms(finished).unwrap_or(0);
                if end_ms > start_ms {
                    let elapsed_s = (end_ms - start_ms) / 1000;
                    if elapsed_s < 60 {
                        format!("{elapsed_s}s")
                    } else {
                        let mins = elapsed_s / 60;
                        let secs = elapsed_s % 60;
                        format!("{mins}m {secs}s")
                    }
                } else {
                    "--".into()
                }
            }
            None => "--".into(),
        }
    }

    /// Parsed engine enum.
    pub fn engine_enum(&self) -> Option<ScanEngine> {
        ScanEngine::from_str(&self.engine)
    }

    /// Parsed status enum.
    pub fn status_enum(&self) -> Option<ScanStatus> {
        ScanStatus::from_str(&self.status)
    }
}

/// Request body for triggering a new scan (POST /api/v1/scans).
pub struct ScanRequest {
    pub project_id: String,
    pub engine: String,
    pub checks: Option<String>,
    pub phase: Option<String>,
    pub module: Option<String>,
}

impl ScanRequest {
    /// Serialize the request to a JSON string for the API call.
    pub fn to_json(&self) -> String {
        let mut obj = json!({
            "project_id": self.project_id,
            "engine": self.engine,
        });

        if let Some(ref checks) = self.checks {
            if !checks.is_empty() {
                obj.insert("checks", json_value_string(checks));
            }
        }

        if let Some(ref phase) = self.phase {
            if !phase.is_empty() {
                obj.insert("phase", json_value_string(phase));
            }
        }

        if let Some(ref module) = self.module {
            if !module.is_empty() {
                obj.insert("module", json_value_string(module));
            }
        }

        json_stringify(&obj)
    }
}

/// Real-time progress message received over WebSocket during a scan.
pub struct ScanProgress {
    pub scan_id: String,
    pub check_id: u32,
    pub check_description: String,
    pub status: String,
    pub current: u32,
    pub total: u32,
}

impl ScanProgress {
    /// Parse a ScanProgress from a JSON value (WebSocket message payload).
    pub fn from_json(value: &JsonValue) -> Option<Self> {
        Some(ScanProgress {
            scan_id: value.get_str("scan_id")?.into(),
            check_id: value.get_u32("check_id")?,
            check_description: value.get_str("check_description")?.into(),
            status: value.get_str("status")?.into(),
            current: value.get_u32("current")?,
            total: value.get_u32("total")?,
        })
    }

    /// Returns the completion ratio as a float between 0.0 and 1.0.
    pub fn ratio(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.current as f64 / self.total as f64
        }
    }

    /// Returns true when the scan has processed all checks.
    pub fn is_complete(&self) -> bool {
        self.total > 0 && self.current >= self.total
    }
}
