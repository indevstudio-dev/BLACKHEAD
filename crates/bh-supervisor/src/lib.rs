#![forbid(unsafe_code)]
//! Minimal privileged supervisor boundary.
/// Stable component identifier.
pub const fn component_name() -> &'static str { "bh-supervisor" }
