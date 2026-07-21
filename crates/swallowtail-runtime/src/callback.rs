use crate::{
    BoxCallbackStream, BoxFuture, CallbackId, Deadline, InputLimitExceeded, RuntimeFailure,
    RuntimeRunId, RuntimeTurnId,
};
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{ProviderExtension, ProviderRequestRef};

include!("callback/request.rs");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackFailureKind {
    UnknownDeclaration,
    Unsupported,
    ConsumerFailed,
    Cancelled,
    TimedOut,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CallbackResult {
    Success(CallbackPayload),
    Failure {
        kind: CallbackFailureKind,
        detail: Option<CallbackPayload>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackResponse {
    callback_id: CallbackId,
    operation_id: CallbackOperationId,
    result: CallbackResult,
}

impl CallbackResponse {
    #[must_use]
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
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> &CallbackOperationId {
        &self.operation_id
    }

    #[must_use]
    pub const fn turn_id(&self) -> Option<&RuntimeTurnId> {
        match &self.operation_id {
            CallbackOperationId::Turn(turn_id) => Some(turn_id),
            CallbackOperationId::Run(_) => None,
        }
    }

    #[must_use]
    pub const fn run_id(&self) -> Option<&RuntimeRunId> {
        match &self.operation_id {
            CallbackOperationId::Run(run_id) => Some(run_id),
            CallbackOperationId::Turn(_) => None,
        }
    }

    #[must_use]
    pub const fn result(&self) -> &CallbackResult {
        &self.result
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackAbandonment {
    TurnCancelled,
    TimedOut,
    TurnTerminated,
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackWaitState {
    Waiting,
    Responded,
    Abandoned(CallbackAbandonment),
}

pub trait CallbackResponder: Send + Sync {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}

pub struct CallbackExchange {
    requests: Option<BoxCallbackStream>,
    responder: Arc<dyn CallbackResponder>,
}

impl CallbackExchange {
    #[must_use]
    pub fn new(requests: BoxCallbackStream, responder: Arc<dyn CallbackResponder>) -> Self {
        Self {
            requests: Some(requests),
            responder,
        }
    }

    pub fn take_requests(&mut self) -> Option<BoxCallbackStream> {
        self.requests.take()
    }

    #[must_use]
    pub fn responder(&self) -> Arc<dyn CallbackResponder> {
        Arc::clone(&self.responder)
    }
}

#[cfg(test)]
mod tests {
    use super::{CallbackPayload, CallbackRequest, CallbackRequestKind};
    use crate::{CallbackId, RuntimeTurnId};

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
}
