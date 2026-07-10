#![forbid(unsafe_code)]
//! Structured local observability and redaction boundary.
/// Stable component identifier.
pub const fn component_name() -> &'static str { "bh-telemetry" }
