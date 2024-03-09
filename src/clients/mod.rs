#[cfg(feature = "async")]
/// Async client for telegram bots api.
pub mod r#async;

#[cfg(feature = "sync")]
/// Sync client for telegram bots api.
pub mod sync;
