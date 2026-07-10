#![forbid(unsafe_code)]
//! OverlayFS, fuse-overlayfs, and copy snapshot selection boundary.
/// Stable component identifier.
pub const fn component_name() -> &'static str { "bh-snapshotter" }
