//! Resource-free direct-model tool continuation.
//!
//! A consumer authorizes each inference attempt, executes returned tools, and
//! submits the exact correlated result set. Provider-private continuation state
//! remains session-bound, redacted, non-serializable, and unusable after the
//! session is invalidated or closed.

#![deny(missing_docs)]

mod binding;
mod exchange;
mod request;
mod state;
mod tool;

pub use binding::{DirectContinuationBinding, ProviderPrivateContinuationRecord};
pub use exchange::{DirectToolExchange, DirectToolResultSubmitter};
pub use request::{
    DirectAttemptAuthorizationKind, DirectContinuationTurnRequest, DirectInferenceAttempt,
    OpenDirectContinuationSessionRequest, validate_direct_continuation_plan,
};
pub use state::DirectContinuationState;
pub use tool::{DirectToolArguments, DirectToolCall, DirectToolResult, DirectToolResultContent};

use crate::RuntimeFailure;
use swallowtail_core::SafeDiagnostic;

fn runtime_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests;
