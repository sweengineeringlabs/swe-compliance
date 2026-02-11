pub mod api;
pub mod core;
pub mod spi;

// Re-export primary consumer types.
pub use api::{
    DefaultDocEngineAiService, DocEngineAiError, DocEngineAiService, AuditResponse,
};
pub use spi::DocEngineAiConfig;
