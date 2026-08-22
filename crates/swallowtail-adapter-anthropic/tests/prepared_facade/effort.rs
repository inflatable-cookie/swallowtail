use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_anthropic::{
    AnthropicDirectDriver, AnthropicModelSelection, AnthropicSessionProfileInput,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision,
    ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, DirectContinuationTurnRequest, DirectToolResult, DirectToolResultContent,
    InteractiveSessionDriver, OpenDirectContinuationSessionRequest, OperationContent,
    OperationPolicy, RequestId, RuntimeTurnId, SchemaDocument, SessionOptions, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus, ToolDeclaration, WorkingStateRestorationOutcome,
};

const EFFORT_VALUES: [&str; 5] = ["low", "medium", "high", "xhigh", "max"];
const QUALIFIED_MODEL: &str = "claude-opus-4-7";

#[test]
fn prepared_effort_values_bind_plan_evidence_policy_and_wire() {
    for (index, value) in EFFORT_VALUES.into_iter().enumerate() {
        let fixture = PreparedFixture::new(
            ExecutionHostId::new(format!("anthropic.effort.run.{index}"))
                .expect("execution host is valid"),
        );
        let mode = ReasoningMode::new(value).expect("official effort value is valid");
        let attempt = fixture
            .prepared()
            .prepare_inference_attempt(
                fixture
                    .attempt_input_for_model("effort-run", QUALIFIED_MODEL)
                    .with_reasoning_mode(mode.clone()),
            )
            .expect("qualified effort prepares");

        assert_eq!(attempt.evidence().reasoning_mode(), Some(&mode));
        assert_eq!(attempt.request().policy().reasoning_mode(), Some(&mode));
        let requirement = attempt
            .plan()
            .requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == Capability::ReasoningSelection)
            .expect("reasoning capability is advertised");
        assert_eq!(
            requirement.constraints().collect::<Vec<_>>(),
            vec![&CapabilityConstraint::ReasoningMode(mode.clone())]
        );

        let mut run = block_on(attempt.start_run(fixture.services())).expect("run starts");
        let mut events = run.take_events().expect("events exist");
        while let Some(event) = block_on(events.next()) {
            event.expect("event succeeds");
        }
        let outcome = block_on(run.take_terminal_outcome().expect("terminal exists"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

        let requests = fixture.server.requests();
        assert_eq!(requests.len(), 1);
        let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
        assert_eq!(body["model"], QUALIFIED_MODEL);
        assert_eq!(body["output_config"]["effort"], value);
        assert!(body.get("thinking").is_none());
    }
}

#[test]
fn absent_effort_keeps_the_existing_request_shape() {
    let fixture = PreparedFixture::new(ExecutionHostId::new("anthropic.effort.absent").unwrap());
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(fixture.attempt_input("effort-absent"))
        .expect("no-effort input prepares");
    assert!(attempt.evidence().reasoning_mode().is_none());
    assert!(attempt.request().policy().reasoning_mode().is_none());
    assert!(
        !attempt
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ReasoningSelection)
    );

    let mut run = block_on(attempt.start_run(fixture.services())).expect("run starts");
    let mut events = run.take_events().expect("events exist");
    while let Some(event) = block_on(events.next()) {
        event.expect("event succeeds");
    }
    let outcome = block_on(run.take_terminal_outcome().expect("terminal exists"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(
        fixture.server.requests()[0].body,
        br#"{"max_tokens":64,"messages":[{"content":"prepared fixture prompt","role":"user"}],"model":"claude-fixture-primary","stream":true}"#
    );
}

#[test]
fn unsupported_effort_model_and_value_fail_before_network() {
    for (index, (model_id, value)) in [
        ("claude-fixture-primary", "low"),
        (QUALIFIED_MODEL, "adaptive"),
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = PreparedFixture::new(
            ExecutionHostId::new(format!("anthropic.effort.reject.{index}"))
                .expect("execution host is valid"),
        );
        let error = fixture
            .prepared()
            .prepare_inference_attempt(
                fixture
                    .attempt_input_for_model("effort-reject", model_id)
                    .with_reasoning_mode(ReasoningMode::new(value).unwrap()),
            )
            .expect_err("unsupported effort must reject");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.anthropic.preparation.reasoning_unsupported"
        );
        assert!(fixture.server.requests().is_empty());
        assert_eq!(fixture.releases(), 0);
    }
}

#[test]
fn structured_plan_request_mismatch_fails_before_endpoint_or_credential_work() {
    let fixture = PreparedFixture::new(ExecutionHostId::new("anthropic.effort.mismatch").unwrap());
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input_for_model("effort-mismatch", QUALIFIED_MODEL)
                .with_reasoning_mode(ReasoningMode::new("xhigh").unwrap()),
        )
        .expect("qualified effort prepares");
    let request = StructuredRunRequest::new(
        RequestId::new("effort-mismatch-request").unwrap(),
        OperationContent::new("prepared fixture prompt").unwrap(),
        OperationPolicy::offline().with_reasoning_mode(ReasoningMode::new("low").unwrap()),
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).unwrap());
    let error = block_on(AnthropicDirectDriver::new().start_run(
        attempt.plan().clone(),
        request,
        fixture.services(),
    ))
    .err()
    .expect("mismatched effort must reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.generation_control_mismatch"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn session_effort_stays_fixed_across_attempts_turns_and_restoration() {
    for (index, value) in EFFORT_VALUES.into_iter().enumerate() {
        let fixture = PreparedFixture::with_stream(
            ExecutionHostId::new(format!("anthropic.effort.session.{index}"))
                .expect("execution host is valid"),
            StreamFixture::ToolContinuation,
        );
        let mode = ReasoningMode::new(value).unwrap();
        let prepared_session = fixture
            .prepared()
            .prepare_session(
                AnthropicSessionProfileInput::new(
                    RequestId::new("effort-session").unwrap(),
                    model(QUALIFIED_MODEL),
                    [fixture_tool()],
                )
                .with_reasoning_mode(mode.clone()),
            )
            .expect("qualified session effort prepares");
        assert_eq!(prepared_session.evidence().reasoning_mode(), Some(&mode));
        assert_eq!(
            prepared_session.request().options().reasoning_mode(),
            Some(&mode)
        );

        let mut session =
            block_on(prepared_session.open_session(fixture.services())).expect("session opens");
        let mut turn = block_on(session.start_direct_continuation_turn(
            DirectContinuationTurnRequest::new(
                RuntimeTurnId::new("effort-turn-1").unwrap(),
                OperationContent::new("Look up the approved fixture customer.").unwrap(),
                swallowtail_runtime::Deadline::at(
                    swallowtail_runtime::MonotonicInstant::from_ticks(100_000),
                ),
            ),
            fixture.services(),
        ))
        .expect("first turn starts");
        let mut exchange = turn
            .take_direct_tool_exchange()
            .expect("tool exchange exists");
        let mut calls = exchange.take_calls().expect("tool calls exist");
        let mut events = turn.take_events().expect("events exist");
        let terminal = turn.take_terminal_outcome().expect("terminal exists");
        let submitter = exchange.submitter();
        let call = block_on(calls.next())
            .expect("tool call arrives")
            .expect("tool call succeeds");
        block_on(submitter.submit(vec![DirectToolResult::new(
            call.call_id().clone(),
            DirectToolResultContent::new(b"approved fixture result".to_vec(), 65_536).unwrap(),
        )]))
        .expect("tool result continues");
        while let Some(event) = block_on(events.next()) {
            event.expect("event succeeds");
        }
        assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);

        let mut later = block_on(session.start_direct_continuation_turn(
            DirectContinuationTurnRequest::new(
                RuntimeTurnId::new("effort-turn-2").unwrap(),
                OperationContent::new("Summarize the approved fixture result.").unwrap(),
                swallowtail_runtime::Deadline::at(
                    swallowtail_runtime::MonotonicInstant::from_ticks(100_000),
                ),
            ),
            fixture.services(),
        ))
        .expect("later turn starts");
        assert!(later.take_direct_tool_exchange().is_none());
        let mut events = later.take_events().expect("events exist");
        let terminal = later.take_terminal_outcome().expect("terminal exists");
        while let Some(event) = block_on(events.next()) {
            event.expect("event succeeds");
        }
        assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
        assert_eq!(block_on(later.close()), CleanupOutcome::Clean);

        let restoration = prepared_session
            .prepare_working_state_restoration(RuntimeTurnId::new("effort-interrupted").unwrap());
        let restored = block_on(restoration.restore(fixture.services())).expect("restores");
        let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
            panic!("fresh session replacement expected");
        };
        let (_, restored_session) = replacement.into_parts();
        assert_eq!(block_on(restored_session.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

        let requests = fixture.server.requests();
        assert_eq!(requests.len(), 3);
        for request in requests {
            let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(body["model"], QUALIFIED_MODEL);
            assert_eq!(body["output_config"]["effort"], value);
            assert!(body.get("thinking").is_none());
        }
        assert_eq!(fixture.releases(), 2);
    }
}

#[test]
fn session_plan_request_mismatch_fails_before_endpoint_or_credential_work() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.effort.session-mismatch").unwrap(),
        StreamFixture::ToolContinuation,
    );
    let prepared_session = fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("effort-session-mismatch").unwrap(),
                model(QUALIFIED_MODEL),
                [fixture_tool()],
            )
            .with_reasoning_mode(ReasoningMode::new("xhigh").unwrap()),
        )
        .expect("qualified session effort prepares");
    let request = OpenDirectContinuationSessionRequest::new(
        RequestId::new("effort-session-mismatch-request").unwrap(),
        prepared_session.request().config().clone(),
    )
    .with_options(
        SessionOptions::default()
            .with_tools([fixture_tool()])
            .with_reasoning_mode(ReasoningMode::new("low").unwrap()),
    );
    let error = block_on(
        AnthropicDirectDriver::new().open_direct_continuation_session(
            prepared_session.plan().clone(),
            request,
            fixture.services(),
        ),
    )
    .err()
    .expect("mismatched session effort must reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.generation_control_mismatch"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

fn model(id: &str) -> AnthropicModelSelection {
    AnthropicModelSelection::new(
        ModelRouteId::new(format!("anthropic.{id}")).unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new(id).unwrap(),
    )
}

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_customer",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"customer_id":{"type":"string"}},"required":["customer_id"],"additionalProperties":false}"#.to_vec(),
            4096,
        )
        .unwrap(),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap()
}
