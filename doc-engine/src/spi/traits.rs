use std::path::{Path, PathBuf};

use super::types::{CheckId, CheckResult, ScanContext, ScanError};
use crate::api::types::ScanReport;

pub trait FileScanner {
    fn scan_files(&self, root: &Path) -> Vec<PathBuf>;
    fn file_exists(&self, root: &Path, relative: &str) -> bool;
    fn read_file(&self, path: &Path) -> Result<String, ScanError>;
}

pub trait CheckRunner: Send + Sync {
    fn id(&self) -> CheckId;
    fn category(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, ctx: &ScanContext) -> CheckResult;
}

pub trait Reporter {
    fn report(&self, report: &ScanReport) -> String;
}
