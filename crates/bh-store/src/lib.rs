#![forbid(unsafe_code)]
//! Durable metadata, content-addressed state, and recovery records.
/// Stable component identifier.
pub const fn component_name() -> &'static str { "bh-store" }
