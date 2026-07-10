#![forbid(unsafe_code)]
//! Semantic runtime backend contracts shared by platform adapters.
/// Stable component identifier.
pub const fn component_name() -> &'static str { "bh-runtime-api" }
