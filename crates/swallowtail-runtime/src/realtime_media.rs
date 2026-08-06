//! Bounded realtime media session vocabulary and ordering state.
//!
//! Media bytes remain opaque, redacted, and non-serializable. Consumers retain
//! device, capture, playback, conversion, and privacy policy. Planned connection
//! rollover is transport continuity within one live operation, never durable
//! resume, retry, or reconstruction.

#![deny(missing_docs)]

mod event;
mod failure;
mod input;
mod request;
mod state;

pub use event::{RealtimeMediaEvent, RealtimeMediaEventKind, RealtimeMediaResponseStatus};
pub use failure::{RealtimeMediaFailure, RealtimeMediaFailureKind};
pub use input::{MediaChunk, MediaInputCommit, MediaTranscript};
pub use request::OpenRealtimeMediaSessionRequest;
pub use state::RealtimeMediaSessionState;

#[cfg(test)]
#[path = "realtime_media/tests.rs"]
mod tests;
