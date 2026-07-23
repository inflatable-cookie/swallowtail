use crate::{BoxDirectToolCallStream, BoxFuture, DirectToolResult, RuntimeFailure};
use std::sync::Arc;

/// Consumer submission surface for a completed direct-model tool attempt.
///
/// This is not a harness callback. The provider request that produced the tool
/// call has already ended, and a successful submission authorizes one new
/// inference attempt.
pub trait DirectToolResultSubmitter: Send + Sync {
    fn submit(&self, results: Vec<DirectToolResult>) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}

pub struct DirectToolExchange {
    calls: Option<BoxDirectToolCallStream>,
    submitter: Arc<dyn DirectToolResultSubmitter>,
}

impl DirectToolExchange {
    #[must_use]
    pub fn new(
        calls: BoxDirectToolCallStream,
        submitter: Arc<dyn DirectToolResultSubmitter>,
    ) -> Self {
        Self {
            calls: Some(calls),
            submitter,
        }
    }

    pub fn take_calls(&mut self) -> Option<BoxDirectToolCallStream> {
        self.calls.take()
    }

    #[must_use]
    pub fn submitter(&self) -> Arc<dyn DirectToolResultSubmitter> {
        Arc::clone(&self.submitter)
    }
}
