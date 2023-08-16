#![doc = include_str!("../README.md")]
//! ## Features
//!
//! - **`blocking`:** Enables the [`blocking`] module, which provides a blocking client.
//!

#[cfg(feature = "blocking")]
pub mod blocking;
pub mod detail;
pub mod error;
pub mod image;
pub mod portal;
pub mod publish;
mod reqwest;
pub mod upload;

/// The default base URL for the Factorio mod portal API.
pub const DEFAULT_BASE_URL: &str = "https://mods.factorio.com/api/";
