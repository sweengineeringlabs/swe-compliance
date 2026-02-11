use std::any::Any;
use std::path::Path;

use async_trait::async_trait;
use serde_json::Value;
use tool::{RiskLevel, Tool, ToolError, ToolOutput, ToolResult};

use doc_engine_scan::{
    format_report_text, scan_with_config, ProjectScope, ScanConfig,
};

/// Tool that wraps `doc-engine-scan::scan_with_config()`.
///
/// Agents invoke this tool to run compliance scans without leaving the
/// LLM conversation loop.
pub struct ComplianceScanTool;

impl ComplianceScanTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ComplianceScanTool {
    fn name(&self) -> &str {
        "compliance_scan"
    }

    fn description(&self) -> &str {
        "Scan a project directory for documentation compliance against \
         ISO 15289/29148/12207 standards"
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the project root directory to scan"
                },
                "scope": {
                    "type": "string",
                    "enum": ["small", "medium", "large"],
                    "description": "Project scope determining which checks apply",
                    "default": "small"
                },
                "format": {
                    "type": "string",
                    "enum": ["json", "text"],
                    "description": "Output format for scan results",
                    "default": "json"
                }
            },
            "required": ["path"]
        })
    }

    fn risk_level(&self) -> RiskLevel {
        RiskLevel::ReadOnly
    }

    async fn execute(&self, args: Value) -> ToolResult<ToolOutput> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidArguments("missing 'path' parameter".into()))?;

        let scope_str = args
            .get("scope")
            .and_then(|v| v.as_str())
            .unwrap_or("small");

        let format = args
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("json");

        let project_scope = match scope_str {
            "small" => ProjectScope::Small,
            "medium" => ProjectScope::Medium,
            "large" => ProjectScope::Large,
            other => {
                return Err(ToolError::InvalidArguments(format!(
                    "invalid scope '{}': use small, medium, or large",
                    other
                )));
            }
        };

        let config = ScanConfig {
            project_type: None,
            project_scope,
            checks: None,
            rules_path: None,
            phases: None,
            module_filter: None,
        };

        let path = Path::new(path_str);
        let report = scan_with_config(path, &config)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let content = match format {
            "text" => Value::String(format_report_text(&report)),
            _ => serde_json::to_value(&report)
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?,
        };

        Ok(ToolOutput {
            content,
            success: true,
            error: None,
            metadata: None,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_metadata() {
        let t = ComplianceScanTool::new();
        assert_eq!(t.name(), "compliance_scan");
        assert_eq!(t.risk_level(), RiskLevel::ReadOnly);
        let schema = t.parameters_schema();
        assert!(schema.get("properties").is_some());
    }
}
