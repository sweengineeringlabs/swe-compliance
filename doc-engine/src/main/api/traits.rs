use std::path::Path;

use super::types::{ScanConfig, ScanReport};
use crate::spi::types::ScanError;

/// Engine for running documentation compliance scans.
///
/// Implementors walk a project directory, execute compliance checks, and
/// produce a [`ScanReport`].
pub trait ComplianceEngine {
    /// Scan a project directory with the supplied [`ScanConfig`].
    fn scan_with_config(&self, root: &Path, config: &ScanConfig) -> Result<ScanReport, ScanError>;
}
