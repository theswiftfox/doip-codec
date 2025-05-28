#![cfg_attr(not(feature = "std"), no_std)] // Use no_std when the "std" feature is disabled
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//! # Diagnostics over Internet Protocol Codec Crate
//!
//! The purpose of this crate is to provide an easy way to encode and decode
//! `DoIP` Messages defined in the `doip-definitions` crate.
//!

// region:      --- Modules

// Python bindings (only available when std is enabled)
#[cfg(feature = "std")]
#[cfg(any(not(test), rust_analyzer))]
mod bindings;

mod doip_codec;
mod error;

// Flatten
pub use crate::error::{Error, Result};
pub use doip_codec::*;

// endregion:   --- Modules
