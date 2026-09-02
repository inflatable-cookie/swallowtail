mod support;

#[path = "direct_driver/failure_cases.rs"]
mod failure_cases;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverCall, DriverFixture, ServerScenario, cleanup_request};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaModelStudioDriver, alibaba_model_studio_descriptor,
};
use swallowtail_core::{DriverRole, SessionProviderStatePolicy};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CleanupOutcome, DriverRegistration,
    InteractiveSessionDriver, OpenSessionRequest, OperationContent, OperationPolicy,
    ProviderObservation, ProviderSessionManagementDriver, RequestId, RunHandle, RuntimeEvent,
    RuntimeEventKind, RuntimeTurnId, SchemaDocument, SessionAccessPolicy, SessionPlanAgreement,
    StructuredOutputDescriptor, StructuredRunDriver, StructuredRunRequest, TerminalOutcome,
    TerminalStatus, TurnHandle, TurnRequest,
};

#[test]
fn descriptor_registers_the_direct_interactive_and_structured_roles() {
    let descriptor = alibaba_model_studio_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.alibaba-model-studio.conversations-responses"
    );
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(descriptor.supports_role(DriverRole::ProviderSessionManagement));
    assert_eq!(descriptor.transport_family().as_str(), "https-sse");
    let driver = std::sync::Arc::new(AlibabaModelStudioDriver::new());
    let registration = DriverRegistration::new(descriptor)
        .with_interactive_session(
            std::sync::Arc::clone(&driver) as std::sync::Arc<dyn InteractiveSessionDriver>
        )
        .expect("declared role registers")
        .with_structured_run(driver as std::sync::Arc<dyn StructuredRunDriver>)
        .expect("declared role registers");
    let driver = std::sync::Arc::new(AlibabaModelStudioDriver::new());
    let registration = registration
        .with_provider_session_management(
            driver as std::sync::Arc<dyn ProviderSessionManagementDriver>,
        )
        .expect("declared management role registers");
    assert!(registration.interactive_session().is_some());
    assert!(registration.structured_run().is_some());
}

#[test]
fn structured_run_uses_one_unstored_response_without_a_conversation() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let request = StructuredRunRequest::new(
        RequestId::new("one-response").expect("request id"),
        OperationContent::new("one private request").expect("content"),
        OperationPolicy::offline(),
    );
    let mut run = block_on(AlibabaModelStudioDriver::new().start_run(
        fixture.run_plan(),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    assert!(run.provider_run_ref().is_none());
    let (events, outcome) = complete_run(&mut run);
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
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);
    assert_eq!(fixture.release_after_blocking(), [1]);

    let requests = fixture.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].target, "/compatible-mode/v1/responses");
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("request JSON");
    assert_eq!(body["model"], "qwen3.7-plus-2026-05-26");
    assert_eq!(body["stream"], true);
    assert_eq!(body["store"], false);
    assert_eq!(body["reasoning"]["effort"], "none");
    assert!(body.get("conversation").is_none());
    assert!(body.get("previous_response_id").is_none());
}

#[test]
fn structured_run_rejects_unsupported_input_before_network_access() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let requests = [
        base_run_request("unsupported-maximum")
            .with_maximum_output_tokens(std::num::NonZeroU64::new(100).expect("nonzero")),
        base_run_request("unsupported-attachment").with_attachments([AttachmentDescriptor::new(
            AttachmentRef::new("fixture-image").expect("attachment ref"),
            "image/png",
            AttachmentRole::Input,
        )
        .expect("attachment")]),
        base_run_request("unsupported-schema").with_structured_output(
            StructuredOutputDescriptor::new(
                SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1_024).expect("schema"),
                "application/schema+json",
                "json-schema-2020-12",
            )
            .expect("structured output"),
        ),
    ];
    for request in requests {
        let error = block_on(AlibabaModelStudioDriver::new().start_run(
            fixture.run_plan(),
            request,
            fixture.services(),
        ))
        .err()
        .expect("unsupported input rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.alibaba_model_studio.unsupported_input"
        );
    }
    assert!(fixture.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn structured_run_cancellation_is_terminal_and_joins_before_release() {
    let fixture = DriverFixture::new(ServerScenario::WaitForCancel);
    let request = StructuredRunRequest::new(
        RequestId::new("cancel-run").expect("request id"),
        OperationContent::new("wait until cancelled").expect("content"),
        OperationPolicy::offline(),
    );
    let mut run = block_on(AlibabaModelStudioDriver::new().start_run(
        fixture.run_plan(),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    block_on(run.cancellation().request()).expect("cancellation requests");
    let (_, outcome) = complete_run(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);
    let calls = fixture.calls.calls();
    assert!(last(&calls, DriverCall::BlockingWork) < last(&calls, DriverCall::CredentialRelease));
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
    assert_eq!(
        block_on(session.close(cleanup_request(&fixture), fixture.services())),
        CleanupOutcome::Clean
    );
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
    OpenSessionRequest::resource_free(
        RequestId::new(id).expect("request id"),
        None,
        plan_agreement(),
    )
}

fn plan_agreement() -> SessionPlanAgreement {
    SessionPlanAgreement::explicit(
        SessionAccessPolicy::resource_free(),
        Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose),
        None,
    )
}

fn turn_request(id: &str, content: &str) -> TurnRequest {
    TurnRequest::new(
        RuntimeTurnId::new(id).expect("turn id"),
        OperationContent::new(content).expect("content"),
    )
}

fn base_run_request(id: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id"),
        OperationContent::new("must not leave the host").expect("content"),
        OperationPolicy::offline(),
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

fn complete_run(run: &mut Box<dyn RunHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut events = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
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
