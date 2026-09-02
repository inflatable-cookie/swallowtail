use super::fixture::{id, prepare, session_profile};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::{FixtureHost, close_session};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_kimi::KimiLocalServerPermissionMode;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult, CleanupOutcome,
    HarnessQuestionId, HarnessQuestionOptionId, HarnessUserInputAnswer, HarnessUserInputResponse,
    OperationContent, RuntimeTurnId, TerminalStatus, TurnRequest,
};

#[test]
fn manual_approval_and_question_are_explicit_callback_exchanges() {
    for (scenario, response) in [
        (
            InteractiveScenario::Approval,
            br#"{"decision":"approved","scope":"session"}"#.as_slice(),
        ),
        (
            InteractiveScenario::Question,
            br#"{"answers":{"q1":{"kind":"single","option_id":"yes"}}}"#.as_slice(),
        ),
    ] {
        let server = InteractiveFixtureServer::start(scenario);
        let host = FixtureHost::for_endpoint(server.endpoint());
        let execution_host = id(ExecutionHostId::new, "fixture.kimi.callback");
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare(execution_host, services.clone(), "0.29.0");
        let profile = session_profile(&prepared, KimiLocalServerPermissionMode::Manual, "callback");
        let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
        let turn_id = id(RuntimeTurnId::new, "turn-callback");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                turn_id.clone(),
                OperationContent::new("fixture callback").expect("content"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        let mut callbacks = turn
            .take_callbacks()
            .expect("manual mode exposes callbacks");
        let mut requests = callbacks
            .take_requests()
            .expect("callback request stream exists");
        let request = block_on(requests.next())
            .expect("callback arrives")
            .expect("callback is valid");
        let result = match scenario {
            InteractiveScenario::Question => {
                assert!(matches!(
                    request.kind(),
                    CallbackRequestKind::HarnessUserInput(_)
                ));
                let invalid = CallbackResult::UserInput(
                    HarnessUserInputResponse::new(
                        [HarnessUserInputAnswer::selected(
                            HarnessQuestionId::new("q1").unwrap(),
                            [HarnessQuestionOptionId::new("not-offered").unwrap()],
                            None,
                        )],
                        4,
                        512,
                    )
                    .unwrap(),
                );
                let failure = block_on(callbacks.responder().respond(CallbackResponse::new(
                    request.callback_id().clone(),
                    turn_id.clone(),
                    invalid,
                )))
                .expect_err("an unoffered option is rejected");
                assert_eq!(
                    failure.diagnostic().code(),
                    "swallowtail.kimi.local_server.callback_malformed"
                );
                CallbackResult::UserInput(
                    HarnessUserInputResponse::new(
                        [HarnessUserInputAnswer::selected(
                            HarnessQuestionId::new("q1").unwrap(),
                            [HarnessQuestionOptionId::new("yes").unwrap()],
                            None,
                        )],
                        4,
                        512,
                    )
                    .unwrap(),
                )
            }
            _ => {
                assert!(matches!(request.kind(), CallbackRequestKind::Extension(_)));
                CallbackResult::Success(
                    CallbackPayload::new(response.to_vec(), 512).expect("response is bounded"),
                )
            }
        };
        block_on(callbacks.responder().respond(CallbackResponse::new(
            request.callback_id().clone(),
            turn_id,
            result,
        )))
        .expect("callback response is accepted");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome exists"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(
            block_on(close_session(session, services)),
            CleanupOutcome::Clean
        );
    }
}
