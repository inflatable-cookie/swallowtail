//! Contract 061 projections for the prepared Kimi routes.

#[path = "consumer_route_projection/builder.rs"]
mod builder;
#[path = "consumer_route_projection/contribution.rs"]
mod contribution;
#[path = "consumer_route_projection/open.rs"]
mod open;

pub use open::{
    KimiProjectionOpenFailure, KimiProjectionOpenFuture, KimiProjectionOpenOutcome,
    KimiProviderValue,
};
