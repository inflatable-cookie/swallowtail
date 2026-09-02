//! Contract 061 contributions for the prepared Claude Agent adapter routes.
//!
//! Prepared facades publish only exact plan, request, activity, and access
//! truth. ACP interactive-session acknowledgement and negotiated model-options
//! observation enter the projection only through the additive prepared-open
//! result.

#[path = "consumer_route_projection/builder.rs"]
mod builder;
#[path = "consumer_route_projection/contribution.rs"]
mod contribution;
#[path = "consumer_route_projection/open.rs"]
mod open;

pub use open::{
    ClaudeAgentProjectionOpenFailure, ClaudeAgentProjectionOpenFuture,
    ClaudeAgentProjectionOpenOutcome,
};
