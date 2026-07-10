#![forbid(unsafe_code)]
//! Per-container process supervision, stdio, signals, and exit state.
/// Stable component identifier.
pub const fn component_name() -> &'static str { "bh-shim" }
