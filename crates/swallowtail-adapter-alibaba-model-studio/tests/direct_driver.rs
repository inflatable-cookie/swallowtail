mod support;

#[path = "direct_driver/failure_cases.rs"]
mod failure_cases;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverCall, DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaModelStudioDriver, alibaba_model_studio_descriptor,
};
use swallowtail_core::{DriverRole, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, DriverRegistration, InteractiveSessionDriver, OpenSessionRequest,
    OperationContent, ProviderObservation, RequestId, RuntimeEvent, RuntimeEventKind,
    RuntimeTurnId, TerminalOutcome, TerminalStatus, TurnHandle, TurnRequest,
};

#[test]
fn descriptor_registers_only_the_direct_interactive_role() {
    let descriptor = alibaba_model_studio_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.alibaba-model-studio.conversations-responses"
    );
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert_eq!(descriptor.transport_family().as_str(), "https-sse");
    let registration = DriverRegistration::new(descriptor)
        .with_interactive_session(std::sync::Arc::new(AlibabaModelStudioDriver::new()))
        .expect("declared role registers");
    assert!(registration.interactive_session().is_some());
    assert!(registration.structured_run().is_none());
}

#[test]
fn two_serial_turns_use_exact_requests_then_delete_items_before_conversation() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let mut session = open(&fixture, "two-turn-session");
    assert!(session.provider_session_ref().is_none());
    assert!(session.resume_binding().is_none());

    for (turn_id, prompt) in [
        ("turn-one", "private first"),
        ("turn-two", "private second"),
    ] {
        let mut turn =
            block_on(session.start_turn(turn_request(turn_id, prompt), fixture.services()))
                .expect("turn starts");
        let (events, outcome) = complete(&mut turn);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.output().expect("output exists").as_str(),
            "Hello world."
        );
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(_))
        )));
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
        )));
        assert!(!format!("{events:?}").contains(prompt));
        assert!(!format!("{outcome:?}").contains("Hello world."));
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    let error = block_on(session.start_turn(
        turn_request("turn-three", "must reject"),
        fixture.services(),
    ))
    .err()
    .expect("third turn rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.alibaba_model_studio.turn_limit_reached"
    );
    assert_eq!(fixture.server.response_attempts(), 2);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);

    let requests = fixture.requests();
    assert_eq!(requests.len(), 9);
    assert_eq!(
        (&requests[0].method[..], &requests[0].target[..]),
        ("POST", "/compatible-mode/v1/conversations")
    );
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.target == "/compatible-mode/v1/responses")
            .count(),
        2
    );
    for request in requests
        .iter()
        .filter(|request| request.target == "/compatible-mode/v1/responses")
    {
        let body: serde_json::Value = serde_json::from_slice(&request.body).expect("request JSON");
        assert_eq!(body["model"], "qwen3.7-plus-2026-05-26");
        assert_eq!(body["conversation"], "conv_fixture_01");
        assert_eq!(body["stream"], true);
        assert_eq!(body["store"], false);
        assert_eq!(body["reasoning"]["effort"], "none");
        assert_eq!(body.as_object().expect("object").len(), 6);
        assert!(!request.headers.contains_key("x-dashscope-session-cache"));
    }
    let cleanup: Vec<_> = requests[3..]
        .iter()
        .map(|request| (request.method.as_str(), request.target.as_str()))
        .collect();
    assert_eq!(
        cleanup[0],
        (
            "GET",
            "/compatible-mode/v1/conversations/conv_fixture_01/items?limit=100&order=asc"
        )
    );
    assert!(
        cleanup[1..5]
            .iter()
            .all(|(method, target)| *method == "DELETE" && target.contains("/items/msg_"))
    );
    assert_eq!(
        cleanup[5],
        (
            "DELETE",
            "/compatible-mode/v1/conversations/conv_fixture_01"
        )
    );
    assert_eq!(fixture.release_after_blocking(), vec![9]);
    let calls = fixture.calls.calls();
    assert!(last(&calls, DriverCall::TaskJoin) < last(&calls, DriverCall::CredentialRelease));
    assert!(last(&calls, DriverCall::BlockingWork) < last(&calls, DriverCall::CredentialRelease));
}

fn open(
    fixture: &DriverFixture,
    id: &str,
) -> Box<dyn swallowtail_runtime::InteractiveSessionHandle> {
    block_on(AlibabaModelStudioDriver::new().open_session(
        fixture.plan(),
        open_request(id),
        fixture.services(),
    ))
    .expect("session opens")
}

fn open_request(id: &str) -> OpenSessionRequest {
    OpenSessionRequest::resource_free(RequestId::new(id).expect("request id"), None)
        .with_provider_state_policy(SessionProviderStatePolicy::DurableConversationDeleteOnClose)
}

fn turn_request(id: &str, content: &str) -> TurnRequest {
    TurnRequest::new(
        RuntimeTurnId::new(id).expect("turn id"),
        OperationContent::new(content).expect("content"),
    )
}

fn complete(turn: &mut Box<dyn TurnHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut events = turn.take_events().expect("events exist");
    let terminal = turn.take_terminal_outcome().expect("terminal exists");
    block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("event is valid"));
        }
        (collected, terminal.await)
    })
}

fn last(calls: &[DriverCall], expected: DriverCall) -> usize {
    calls
        .iter()
        .rposition(|call| *call == expected)
        .expect("expected call exists")
}
