#[cfg(feature = "async")]
/// Async client methods for telegram bots api.
pub mod r#async;

#[cfg(feature = "sync")]
/// Sync client methods for telegram bots api.
pub mod sync;
