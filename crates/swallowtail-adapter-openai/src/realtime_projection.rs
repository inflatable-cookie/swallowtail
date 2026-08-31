//! Contract 061 contributions emitted by the prepared OpenAI Realtime route.
//!
//! The prepared contribution publishes selection and session-start truth only.
//! Provider-effective or rejected reasoning state comes from the additive
//! prepared-open result, which is returned only after the exact
//! `session.updated` event the adapter itself parsed.

#[path = "realtime_projection/contribution.rs"]
mod contribution;
#[path = "realtime_projection/open.rs"]
mod open;
#[path = "realtime_projection/rows.rs"]
mod rows;

pub use open::{
    OpenAiRealtimeProjectionOpenFailure, OpenAiRealtimeProjectionOpenFuture,
    OpenAiRealtimeProjectionOpenOutcome,
};
