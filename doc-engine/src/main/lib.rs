//! doc-engine: a compliance auditing library for project documentation.
//!
//! Validates project documentation against the template-engine framework's
//! 76 compliance checks plus 15 opt-in spec file checks. Provides both
//! a library API ([`scan`], [`scan_with_config`]) and a CLI binary.
//!
//! # Quick Start
//!
//! ```no_run
//! use doc_engine::{scan, format_report_text};
//! use std::path::Path;
//!
//! let report = scan(Path::new(".")).expect("scan failed");
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
