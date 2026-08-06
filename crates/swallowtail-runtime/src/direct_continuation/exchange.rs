use crate::{BoxDirectToolCallStream, BoxFuture, DirectToolResult, RuntimeFailure};
use std::sync::Arc;

/// Consumer submission surface for a completed direct-model tool attempt.
///
/// This is not a harness callback. The provider request that produced the tool
/// call has already ended, and a successful submission authorizes one new
/// inference attempt.
pub trait DirectToolResultSubmitter: Send + Sync {
    /// Submits the complete correlated result set for the pending tool calls.
    ///
    /// A successful submission authorizes exactly one further inference
    /// attempt. Partial, duplicate, or unrelated result sets must be rejected.
    fn submit(&self, results: Vec<DirectToolResult>) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}

/// One direct attempt's tool-call stream and correlated result submitter.
pub struct DirectToolExchange {
    calls: Option<BoxDirectToolCallStream>,
    submitter: Arc<dyn DirectToolResultSubmitter>,
}

impl DirectToolExchange {
    #[must_use]
    /// Creates an exchange from its single-consumer call stream and submitter.
    pub fn new(
        calls: BoxDirectToolCallStream,
        submitter: Arc<dyn DirectToolResultSubmitter>,
    ) -> Self {
        Self {
            calls: Some(calls),
            submitter,
        }
    }

    /// Takes the tool-call stream, leaving no stream for subsequent callers.
    pub fn take_calls(&mut self) -> Option<BoxDirectToolCallStream> {
        self.calls.take()
    }

    #[must_use]
    /// Returns a shared handle to the result-submission surface.
    pub fn submitter(&self) -> Arc<dyn DirectToolResultSubmitter> {
        Arc::clone(&self.submitter)
    }
}
