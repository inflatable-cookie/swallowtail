use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CallbackAbandonment, CallbackPayload, CallbackRequest, CallbackResponse, CallbackResult,
    CallbackWaitState, Deadline, RuntimeEvent, RuntimeEventKind,
};

pub struct CallbackExchangeFixture {
    request: CallbackRequest,
    event: RuntimeEvent,
    state: CallbackWaitState,
}

impl CallbackExchangeFixture {
    #[must_use]
    pub fn new(request: CallbackRequest) -> Self {
        let event = RuntimeEvent::new(
            request.event_sequence(),
            RuntimeEventKind::CallbackRequested(request.callback_id().clone()),
        );
        Self {
            request,
            event,
            state: CallbackWaitState::Waiting,
        }
    }

    #[must_use]
    pub const fn request(&self) -> &CallbackRequest {
        &self.request
    }

    #[must_use]
    pub const fn event(&self) -> &RuntimeEvent {
        &self.event
    }

    #[must_use]
    pub const fn state(&self) -> CallbackWaitState {
        self.state
    }

    pub fn respond(&mut self, response: CallbackResponse) -> Result<(), SafeDiagnostic> {
        if self.state != CallbackWaitState::Waiting {
            return Err(failure(
                "swallowtail.testkit.callback_not_waiting",
                "Callback response arrived after the callback stopped waiting",
            ));
        }
        if response.callback_id() != self.request.callback_id()
            || response.operation_id() != self.request.operation_id()
        {
            return Err(failure(
                "swallowtail.testkit.callback_correlation_mismatch",
                "Callback response does not match its request",
            ));
        }
        self.state = CallbackWaitState::Responded;
        Ok(())
    }

    pub fn abandon(&mut self, reason: CallbackAbandonment) -> Result<(), SafeDiagnostic> {
        if self.state != CallbackWaitState::Waiting {
            return Err(failure(
                "swallowtail.testkit.callback_not_waiting",
                "Callback cannot be abandoned after it stopped waiting",
            ));
        }
        self.state = CallbackWaitState::Abandoned(reason);
        Ok(())
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.request.deadline()
    }
}

#[must_use]
pub fn successful_callback_response(request: &CallbackRequest) -> CallbackResponse {
    CallbackResponse::new(
        request.callback_id().clone(),
        request
            .turn_id()
            .expect("fixture callback belongs to a turn")
            .clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"fixture-result".to_vec(), 64)
                .expect("fixture callback payload is bounded"),
        ),
    )
}

fn failure(code: &'static str, message: &'static str) -> SafeDiagnostic {
    SafeDiagnostic::new(code, message)
}

#[cfg(test)]
mod tests {
    use super::{CallbackExchangeFixture, successful_callback_response};
    use swallowtail_runtime::{
        CallbackAbandonment, CallbackId, CallbackPayload, CallbackRequest, CallbackResponse,
        CallbackResult, CallbackWaitState, RuntimeTurnId,
    };

    fn fixture() -> CallbackExchangeFixture {
        CallbackExchangeFixture::new(
            CallbackRequest::tool_call(
                CallbackId::new("callback-1").expect("callback id is valid"),
                RuntimeTurnId::new("turn-1").expect("turn id is valid"),
                2,
                None,
                "fixture_tool",
                CallbackPayload::new(b"{}".to_vec(), 16).expect("payload is bounded"),
            )
            .expect("request is valid"),
        )
    }

    #[test]
    fn response_is_exactly_once_and_correlated() {
        let mut fixture = fixture();
        let response = successful_callback_response(fixture.request());
        fixture.respond(response.clone()).expect("response matches");
        assert_eq!(fixture.state(), CallbackWaitState::Responded);
        assert!(fixture.respond(response).is_err());
    }

    #[test]
    fn mismatched_and_late_responses_fail_explicitly() {
        let mut mismatch = fixture();
        let wrong = CallbackResponse::new(
            CallbackId::new("callback-other").expect("callback id is valid"),
            mismatch
                .request()
                .turn_id()
                .expect("fixture callback belongs to a turn")
                .clone(),
            CallbackResult::Success(
                CallbackPayload::new(Vec::new(), 1).expect("empty payload is bounded"),
            ),
        );
        assert!(mismatch.respond(wrong).is_err());

        let mut abandoned = fixture();
        abandoned
            .abandon(CallbackAbandonment::TurnCancelled)
            .expect("waiting callback is abandoned");
        let late = successful_callback_response(abandoned.request());
        assert!(abandoned.respond(late).is_err());
    }
}
