//! struct-engine: a compliance auditing library for Rust package structure.
//!
//! Validates Rust project structure against 44 compliance checks covering
//! directory layout, Cargo.toml metadata, naming conventions, test organization,
//! documentation presence, and project hygiene. Provides both a library API
//! ([`scan`], [`scan_with_config`]) and a CLI binary.
//!
//! # Quick Start
//!
//! ```no_run
//! use struct_engine::{scan, format_report_text};
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
