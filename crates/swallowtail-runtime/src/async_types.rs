use crate::{CallbackRequest, DirectToolCall, RealtimeMediaEvent, RuntimeEvent, RuntimeFailure};
use futures_core::Stream;
use std::future::Future;
use std::pin::Pin;

/// Boxed sendable future used by executor-neutral runtime ports.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
/// Boxed stream of portable runtime events.
pub type BoxEventStream =
    Pin<Box<dyn Stream<Item = Result<RuntimeEvent, RuntimeFailure>> + Send + 'static>>;
/// Boxed stream of exactly-once callback requests.
pub type BoxCallbackStream =
    Pin<Box<dyn Stream<Item = Result<CallbackRequest, RuntimeFailure>> + Send + 'static>>;
/// Boxed stream of direct-model tool calls.
pub type BoxDirectToolCallStream =
    Pin<Box<dyn Stream<Item = Result<DirectToolCall, RuntimeFailure>> + Send + 'static>>;
/// Boxed stream of ordered realtime media events.
pub type BoxRealtimeMediaEventStream =
    Pin<Box<dyn Stream<Item = Result<RealtimeMediaEvent, RuntimeFailure>> + Send + 'static>>;
