#![allow(clippy::new_without_default, dead_code)]

//! Telegram bots api simple rust wrapper, and no more.
//!
//! For a overview, see [GitHub repository](https://github.com/khusnetdinov/telegram_bots_api).
//!

/// Api related wrapping structures, traits, methods.
pub mod api;

/// Implementation for async, sync clients.
pub mod clients;

/// Lib configuration struct.
pub mod config;
mod errors;
mod tests;
