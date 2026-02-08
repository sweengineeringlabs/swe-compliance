use std::path::Path;

use super::types::{ScanConfig, ScanReport};
use crate::spi::types::ScanError;

pub trait ComplianceEngine {
    fn scan(&self, root: &Path) -> Result<ScanReport, ScanError>;
    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError>;
}
