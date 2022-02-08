//! This crate contains library code for validity predicate WASM. Most of the
//! code is re-exported from the `anoma_vm_env` crate.

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

#[cfg(not(tarpaulin))]
pub use anoma_vm_env::vp_prelude::*;

/// Log a string in a debug build. The message will be printed at the
/// `tracing::Level::Info`. Any `debug_log!` statements are only enabled in
/// non optimized builds by default. An optimized build will not execute
/// `debug_log!` statements unless `-C debug-assertions` is passed to the
/// compiler.
#[macro_export]
#[cfg(not(tarpaulin))]
macro_rules! debug_log {
    ($($arg:tt)*) => {{
        (if cfg!(debug_assertions) { log_string(format!($($arg)*)) })
    }}
}
