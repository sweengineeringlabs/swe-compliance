use crate::util::api::{api_post};
use crate::features::scaffold::scaffold_type::{ParsedDomain, ScaffoldResult};

/// Parse SRS content (FR-500).
pub async fn parse_srs(content: &str) -> Result<Vec<ParsedDomain>, String> {
    let body = json_stringify(&json!({ "content": content }));
    let response = api_post("/scaffold/parse", &body).await.map_err(|e| e.message)?;
    json_parse_vec(&response).map_err(|e| format!("parse error: {e}"))
}

/// Execute scaffolding (FR-502).
pub async fn execute_scaffold(srs_path: &str, output_dir: &str, phases: &[String], file_types: &[String], force: bool) -> Result<ScaffoldResult, String> {
    let body = json_stringify(&json!({
        "srs_path": srs_path, "output_dir": output_dir,
        "phases": phases, "file_types": file_types, "force": force,
    }));
    let response = api_post("/scaffold/execute", &body).await.map_err(|e| e.message)?;
    json_parse_obj(&response).map_err(|e| format!("parse error: {e}"))
}
