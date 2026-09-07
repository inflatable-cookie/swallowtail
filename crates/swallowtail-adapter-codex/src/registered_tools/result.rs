//! Translation of one settled registered outcome into the Codex tool result.
//!
//! The Codex dynamic tool result is the only response path this route has, so
//! an Allow and a Deny both travel on it. A denial, a typed failure, and an
//! unknown execution outcome are reported as the exact failure the provider
//! receives; none of them is ever rendered as a successful tool result, and no
//! call is retried automatically.

use crate::callback_exchange::dynamic_tool_result;
use serde_json::Value;
use swallowtail_runtime::{
    RegisteredToolExecutionDisposition, RegisteredToolOutcome, RuntimeFailure,
};

/// Renders one settled outcome, or a pre-dispatch refusal, for the provider.
pub(crate) fn provider_result(
    settled: Result<RegisteredToolOutcome, RuntimeFailure>,
) -> (Value, bool) {
    match settled {
        Ok(outcome) => rendered_outcome(&outcome),
        // The kernel refused before or after dispatch: an unsupported tool, a
        // duplicate correlation, a revoked admission, a frozen lease. Its safe
        // diagnostic is the exact denial the provider receives.
        Err(error) => (
            dynamic_tool_result(false, error.diagnostic().message()),
            false,
        ),
    }
}

fn rendered_outcome(outcome: &RegisteredToolOutcome) -> (Value, bool) {
    if let Some(result) = outcome.result() {
        return match std::str::from_utf8(result.payload().expose_for_execution()) {
            Ok(text) => (dynamic_tool_result(true, text), true),
            // A bounded result that is not text cannot cross this wire. The
            // call still executed, so the provider is told it failed here
            // rather than being handed raw bytes.
            Err(_) => (
                dynamic_tool_result(false, "Registered tool result was not valid text"),
                true,
            ),
        };
    }
    let message = outcome.failure().map_or(
        "Registered tool call did not settle with a result",
        |failure| failure.kind().message(),
    );
    let text = match outcome.disposition() {
        RegisteredToolExecutionDisposition::Unknown => {
            format!("{message}; execution outcome unknown and not retried")
        }
        RegisteredToolExecutionDisposition::Executed
        | RegisteredToolExecutionDisposition::NotExecuted => message.to_owned(),
    };
    (dynamic_tool_result(false, &text), false)
}
