mod common;

use doc_engine::{scan, format_report_text, format_report_json, ScanReport};

#[test]
fn test_text_contains_header() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    let text = format_report_text(&report);
    assert!(text.contains("doc-engine scan results"));
}

#[test]
fn test_text_contains_summary() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    let text = format_report_text(&report);
    assert!(text.contains("passed,"));
    assert!(text.contains("failed,"));
    assert!(text.contains("skipped"));
}

#[test]
fn test_json_valid() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    let json = format_report_json(&report);
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(val.is_object());
}

#[test]
fn test_json_has_results_and_summary() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    let json = format_report_json(&report);
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(val.get("results").is_some());
    assert!(val.get("summary").is_some());
    assert!(val["results"].is_array());
    assert!(val["summary"].is_object());
}

#[test]
fn test_json_roundtrip() {
    let tmp = common::create_minimal_project();
    let report = scan(tmp.path()).unwrap();
    let json = format_report_json(&report);
    let deserialized: ScanReport = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.results.len(), report.results.len());
    assert_eq!(deserialized.summary.total, report.summary.total);
    assert_eq!(deserialized.summary.passed, report.summary.passed);
    assert_eq!(deserialized.summary.failed, report.summary.failed);
    assert_eq!(deserialized.summary.skipped, report.summary.skipped);
}
