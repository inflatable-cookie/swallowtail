use crate::{CallbackRequest, RuntimeEvent, RuntimeFailure};
use futures_core::Stream;
use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxEventStream =
    Pin<Box<dyn Stream<Item = Result<RuntimeEvent, RuntimeFailure>> + Send + 'static>>;
pub type BoxCallbackStream =
    Pin<Box<dyn Stream<Item = Result<CallbackRequest, RuntimeFailure>> + Send + 'static>>;
