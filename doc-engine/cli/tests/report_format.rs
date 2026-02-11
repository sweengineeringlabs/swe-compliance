mod common;

use doc_engine_scan::{scan_with_config, format_report_text, format_report_json, ScanConfig, ScanReport, ProjectScope};

#[test]
fn test_text_contains_header() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: None,
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
        phases: None,
        module_filter: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    let text = format_report_text(&report);
    assert!(text.contains("doc-engine scan results"));
}

#[test]
fn test_text_contains_summary() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: None,
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
        phases: None,
        module_filter: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    let text = format_report_text(&report);
    assert!(text.contains("passed,"));
    assert!(text.contains("failed,"));
    assert!(text.contains("skipped"));
}

#[test]
fn test_json_valid() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: None,
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
        phases: None,
        module_filter: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    let json = format_report_json(&report);
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(val.is_object());
}

#[test]
fn test_json_has_results_and_summary() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: None,
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
        phases: None,
        module_filter: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    let json = format_report_json(&report);
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(val.get("results").is_some());
    assert!(val.get("summary").is_some());
    assert!(val["results"].is_array());
    assert!(val["summary"].is_object());
    // ISO/IEC/IEEE 15289:2019 clause 9.2 metadata
    assert_eq!(val["standard"], "ISO/IEC/IEEE 15289:2019");
    assert_eq!(val["clause"], "9.2");
    assert_eq!(val["tool"], "doc-engine");
    assert!(val["tool_version"].is_string());
    assert!(val["timestamp"].is_string());
    assert!(val["project_root"].is_string());
}

#[test]
fn test_json_roundtrip() {
    let tmp = common::create_minimal_project();
    let config = ScanConfig {
        project_type: None,
        project_scope: ProjectScope::Large,
        checks: None,
        rules_path: None,
        phases: None,
        module_filter: None,
    };
    let report = scan_with_config(tmp.path(), &config).unwrap();
    let json = format_report_json(&report);
    let deserialized: ScanReport = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.results.len(), report.results.len());
    assert_eq!(deserialized.summary.total, report.summary.total);
    assert_eq!(deserialized.summary.passed, report.summary.passed);
    assert_eq!(deserialized.summary.failed, report.summary.failed);
    assert_eq!(deserialized.summary.skipped, report.summary.skipped);
    // ISO metadata survives roundtrip
    assert_eq!(deserialized.standard, report.standard);
    assert_eq!(deserialized.clause, report.clause);
    assert_eq!(deserialized.tool, report.tool);
    assert_eq!(deserialized.tool_version, report.tool_version);
    assert_eq!(deserialized.timestamp, report.timestamp);
    assert_eq!(deserialized.project_root, report.project_root);
}
