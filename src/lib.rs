#![allow(clippy::new_without_default, dead_code)]

pub use crate::client::Client;

pub mod api;
pub mod client;
mod clients;
mod config;
mod errors;
mod tests;
