//! doc-engine: a compliance auditing library for project documentation.
//!
//! Validates project documentation against the template-engine framework's
//! compliance checks. Provides both a library API ([`scan_with_config`])
//! and a CLI binary.
//!
//! # Quick Start
//!
//! ```no_run
//! use doc_engine::{scan_with_config, format_report_text, ScanConfig, ProjectScope};
//! use std::path::Path;
//!
//! let config = ScanConfig {
//!     project_type: None,
//!     project_scope: ProjectScope::Small,
//!     checks: None,
//!     rules_path: None,
//! };
//! let report = scan_with_config(Path::new("."), &config).expect("scan failed");
//! println!("{}", format_report_text(&report));
//! ```

#![warn(missing_docs)]

/// Service Provider Interface: traits and types for extending the engine.
pub mod spi;
/// Application Programming Interface: public traits and configuration types.
pub mod api;
mod core;
mod saf;

pub use saf::*;
