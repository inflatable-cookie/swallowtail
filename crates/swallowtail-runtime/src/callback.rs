use crate::{
    BoxCallbackStream, BoxFuture, CallbackId, Deadline, InputLimitExceeded, RuntimeFailure,
    RuntimeTurnId,
};
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{ProviderExtension, ProviderRequestRef};

#[derive(Clone, Eq, PartialEq)]
pub struct CallbackPayload(Vec<u8>);

impl CallbackPayload {
    pub fn new(
        bytes: impl Into<Vec<u8>>,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let bytes = bytes.into();
        if bytes.len() > maximum_bytes {
            Err(InputLimitExceeded::new(
                "callback payload",
                maximum_bytes,
                bytes.len(),
            ))
        } else {
            Ok(Self(bytes))
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for CallbackPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CallbackPayload")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CallbackRequestKind {
    ToolCall {
        tool_name: String,
        arguments: CallbackPayload,
    },
    Extension(ProviderExtension),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackRequest {
    callback_id: CallbackId,
    turn_id: RuntimeTurnId,
    event_sequence: u64,
    deadline: Option<Deadline>,
    provider_request_ref: Option<ProviderRequestRef>,
    kind: CallbackRequestKind,
}

impl CallbackRequest {
    pub fn tool_call(
        callback_id: CallbackId,
        turn_id: RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        tool_name: impl Into<String>,
        arguments: CallbackPayload,
    ) -> Result<Self, crate::InputValueRequired> {
        Ok(Self {
            callback_id,
            turn_id,
            event_sequence,
            deadline,
            provider_request_ref: None,
            kind: CallbackRequestKind::ToolCall {
                tool_name: crate::input::required_text("callback tool name", tool_name)?,
                arguments,
            },
        })
    }

    pub fn extension(
        callback_id: CallbackId,
        turn_id: RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        extension: ProviderExtension,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        if extension.payload().len() > maximum_bytes {
            Err(InputLimitExceeded::new(
                "callback extension payload",
                maximum_bytes,
                extension.payload().len(),
            ))
        } else {
            Ok(Self {
                callback_id,
                turn_id,
                event_sequence,
                deadline,
                provider_request_ref: None,
                kind: CallbackRequestKind::Extension(extension),
            })
        }
    }

    #[must_use]
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    pub const fn event_sequence(&self) -> u64 {
        self.event_sequence
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    pub const fn kind(&self) -> &CallbackRequestKind {
        &self.kind
    }

    #[must_use]
    pub fn with_provider_request_ref(mut self, provider_request_ref: ProviderRequestRef) -> Self {
        self.provider_request_ref = Some(provider_request_ref);
        self
    }

    #[must_use]
    pub const fn provider_request_ref(&self) -> Option<&ProviderRequestRef> {
        self.provider_request_ref.as_ref()
    }
}

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
    turn_id: RuntimeTurnId,
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
            turn_id,
            result,
        }
    }

    #[must_use]
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
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
