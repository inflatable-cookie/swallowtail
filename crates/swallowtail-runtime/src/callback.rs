#![deny(missing_docs)]

use crate::{
    BoxCallbackStream, BoxFuture, CallbackId, Deadline, HarnessUserInputResponse,
    InputLimitExceeded, RuntimeFailure, RuntimeRunId, RuntimeTurnId,
};
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{ProviderExtension, ProviderRequestRef};

include!("callback/request.rs");

/// Consumer-declared reason that a callback could not produce a result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackFailureKind {
    /// The callback referenced no admitted consumer declaration.
    UnknownDeclaration,
    /// The consumer does not support the requested operation.
    Unsupported,
    /// Consumer execution failed.
    ConsumerFailed,
    /// The owning runtime operation was cancelled.
    Cancelled,
    /// The callback exceeded its deadline.
    TimedOut,
}

/// Exactly one consumer result returned for a callback request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CallbackResult {
    /// Successful opaque result payload.
    Success(CallbackPayload),
    /// Validated response to a typed harness question.
    UserInput(HarnessUserInputResponse),
    /// Failure with an optional bounded provider-facing detail payload.
    Failure {
        /// Portable failure classification.
        kind: CallbackFailureKind,
        /// Optional bounded detail translated by the exact adapter.
        detail: Option<CallbackPayload>,
    },
}

/// Consumer response correlated to one exact callback and operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackResponse {
    callback_id: CallbackId,
    operation_id: CallbackOperationId,
    result: CallbackResult,
}

impl CallbackResponse {
    /// Creates a response by copying exact correlation from its request.
    #[must_use]
    pub fn for_request(request: &CallbackRequest, result: CallbackResult) -> Self {
        Self {
            callback_id: request.callback_id().clone(),
            operation_id: request.operation_id().clone(),
            result,
        }
    }

    #[must_use]
    /// Creates a response for one interactive turn.
    pub const fn new(
        callback_id: CallbackId,
        turn_id: RuntimeTurnId,
        result: CallbackResult,
    ) -> Self {
        Self {
            callback_id,
            operation_id: CallbackOperationId::Turn(turn_id),
            result,
        }
    }

    #[must_use]
    /// Creates a response for one structured run.
    pub const fn for_run(
        callback_id: CallbackId,
        run_id: RuntimeRunId,
        result: CallbackResult,
    ) -> Self {
        Self {
            callback_id,
            operation_id: CallbackOperationId::Run(run_id),
            result,
        }
    }

    #[must_use]
    /// Returns the callback identity.
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    /// Returns the owning runtime operation.
    pub const fn operation_id(&self) -> &CallbackOperationId {
        &self.operation_id
    }

    #[must_use]
    /// Returns the owning turn when this is a turn callback.
    pub const fn turn_id(&self) -> Option<&RuntimeTurnId> {
        match &self.operation_id {
            CallbackOperationId::Turn(turn_id) => Some(turn_id),
            CallbackOperationId::Run(_) => None,
        }
    }

    #[must_use]
    /// Returns the owning run when this is a run callback.
    pub const fn run_id(&self) -> Option<&RuntimeRunId> {
        match &self.operation_id {
            CallbackOperationId::Run(run_id) => Some(run_id),
            CallbackOperationId::Turn(_) => None,
        }
    }

    #[must_use]
    /// Returns the consumer-supplied result.
    pub const fn result(&self) -> &CallbackResult {
        &self.result
    }
}

/// Reason an unanswered callback stopped accepting responses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackAbandonment {
    /// The owning turn was cancelled.
    TurnCancelled,
    /// The callback or operation deadline elapsed.
    TimedOut,
    /// The owning turn reached another terminal state.
    TurnTerminated,
    /// The callback exchange closed.
    Closed,
}

/// Exactly-once lifecycle state of one admitted callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackWaitState {
    /// A response may still be accepted.
    Waiting,
    /// One response was accepted.
    Responded,
    /// The callback closed without a response.
    Abandoned(CallbackAbandonment),
}

/// Exactly-once response port for callback requests from one operation.
pub trait CallbackResponder: Send + Sync {
    /// Submits one correlated response or rejects it with a safe runtime failure.
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}

/// Take-once request stream paired with its response port.
pub struct CallbackExchange {
    requests: Option<BoxCallbackStream>,
    responder: Arc<dyn CallbackResponder>,
}

impl CallbackExchange {
    /// Creates an exchange from one request stream and responder.
    #[must_use]
    pub fn new(requests: BoxCallbackStream, responder: Arc<dyn CallbackResponder>) -> Self {
        Self {
            requests: Some(requests),
            responder,
        }
    }

    /// Takes the ordered callback request stream, if not already taken.
    pub fn take_requests(&mut self) -> Option<BoxCallbackStream> {
        self.requests.take()
    }

    #[must_use]
    /// Clones the response port for submission from consumer UI or tool work.
    pub fn responder(&self) -> Arc<dyn CallbackResponder> {
        Arc::clone(&self.responder)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CallbackPayload, CallbackRequest, CallbackRequestKind, CallbackResponse, CallbackResult,
    };
    use crate::{CallbackId, CallbackOperationId, RuntimeRunId, RuntimeTurnId};

    #[test]
    fn callback_records_are_bounded_correlated_and_redacted() {
        let request = CallbackRequest::tool_call(
            CallbackId::new("callback-private").expect("callback id is valid"),
            RuntimeTurnId::new("turn-private").expect("turn id is valid"),
            4,
            None,
            "task_ledger",
            CallbackPayload::new(br#"{"private":true}"#.to_vec(), 128).expect("payload is bounded"),
        )
        .expect("request is valid");

        assert_eq!(request.event_sequence(), 4);
        assert!(matches!(
            request.kind(),
            CallbackRequestKind::ToolCall { .. }
        ));
        let rendered = format!("{request:?}");
        assert!(!rendered.contains("callback-private"));
        assert!(!rendered.contains("turn-private"));
        assert!(!rendered.contains("private\":true"));
        assert!(CallbackPayload::new(vec![0; 5], 4).is_err());
    }

    #[test]
    fn response_can_copy_run_correlation_from_its_request() {
        let request = CallbackRequest::run_tool_call(
            CallbackId::new("callback-private").unwrap(),
            RuntimeRunId::new("run-private").unwrap(),
            4,
            None,
            "task_ledger",
            CallbackPayload::new(Vec::new(), 0).unwrap(),
        )
        .unwrap();
        let response = CallbackResponse::for_request(
            &request,
            CallbackResult::Success(CallbackPayload::new(Vec::new(), 0).unwrap()),
        );

        assert_eq!(response.callback_id(), request.callback_id());
        assert!(matches!(
            response.operation_id(),
            CallbackOperationId::Run(run_id) if run_id == request.run_id().unwrap()
        ));
    }
}
