mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_xai::{
    XaiRunProfileInput, XaiSessionProfileInput, XaiWebSocketDriver, prepare_xai_responses_websocket,
};
use swallowtail_core::DriverRole;
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent,
    OperationPolicy, RequestId, StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

use support::{
    DriverCall, DriverFixture, ServerScenario, assert_wire_controls, qualified_model, turn_request,
};

#[test]
fn prepared_xai_unqualified_controls_fail_before_provider_effects() {
    for (model_id, reasoning, maximum) in [
        ("grok-4.5-latest", Some("high"), None),
        ("grok-4.5", Some("xhigh"), None),
        ("grok-4.6", None, Some(i32::MAX as u64 + 1)),
    ] {
        let fixture = DriverFixture::new(ServerScenario::OneResponse);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let mut input = XaiRunProfileInput::new(
            RequestId::new(format!("rejected-{model_id}")).expect("request id is valid"),
            qualified_model(model_id),
            OperationContent::new("rejected response").expect("content is valid"),
            None,
        );
        if let Some(reasoning) = reasoning {
            input = input.with_reasoning_mode(
                swallowtail_core::ReasoningMode::new(reasoning).expect("mode is valid"),
            );
        }
        if let Some(maximum) = maximum {
            input = input
                .with_maximum_output_tokens(NonZeroU64::new(maximum).expect("maximum is positive"));
        }
        let error = prepared
            .prepare_responses_run(input)
            .expect_err("unqualified control is rejected");
        assert!(
            error
                .diagnostic()
                .safe()
                .code()
                .starts_with("swallowtail.xai.preparation.")
        );
        assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
        assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
        assert!(fixture.server.frames().is_empty());
    }
}

#[test]
fn low_level_run_rejects_invalid_output_plan_before_provider_effects() {
    for maximum in [0, i32::MAX as u64 + 1] {
        let fixture = DriverFixture::new(ServerScenario::OneResponse);
        let request = StructuredRunRequest::new(
            RequestId::new(format!("invalid-run-{maximum}")).expect("request id is valid"),
            OperationContent::new("invalid output plan").expect("content is valid"),
            OperationPolicy::offline(),
        );
        let error = block_on(XaiWebSocketDriver::new().start_run(
            fixture.plan_with_output_token_maximum(DriverRole::StructuredRun, maximum),
            request,
            fixture.services(),
        ))
        .err()
        .expect("invalid output plan is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.xai.generation_control_mismatch"
        );
        assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
        assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
        assert!(fixture.server.frames().is_empty());
    }
}

#[test]
fn low_level_session_rejects_invalid_output_plan_before_provider_effects() {
    for maximum in [0, i32::MAX as u64 + 1] {
        let fixture = DriverFixture::new(ServerScenario::Success);
        let plan = fixture.plan_with_output_token_maximum(DriverRole::InteractiveSession, maximum);
        let request = OpenSessionRequest::resource_free_from_plan(
            &plan,
            RequestId::new(format!("invalid-session-{maximum}")).expect("request id is valid"),
            None,
        )
        .expect("session request derives from plan");
        let error =
            block_on(XaiWebSocketDriver::new().open_session(plan, request, fixture.services()))
                .err()
                .expect("invalid output plan is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.xai.generation_control_mismatch"
        );
        assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
        assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
        assert!(fixture.server.frames().is_empty());
    }
}

#[test]
fn prepared_xai_driver_rejects_control_request_drift_before_provider_effects() {
    let run_fixture = DriverFixture::new(ServerScenario::OneResponse);
    let prepared =
        prepare_xai_responses_websocket(run_fixture.preparation_input(), &run_fixture.services())
            .expect("xAI integration prepares");
    let operation = prepared
        .prepare_responses_run(
            XaiRunProfileInput::new(
                RequestId::new("drift-run").expect("request id"),
                qualified_model("grok-4.6"),
                OperationContent::new("drifted request").expect("content"),
                None,
            )
            .with_reasoning_mode(
                swallowtail_core::ReasoningMode::new("high").expect("mode is valid"),
            )
            .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum is positive")),
        )
        .expect("controlled run prepares");
    let (_, plan, request) = operation.into_parts();
    let drifted = StructuredRunRequest::new(
        request.request_id().clone(),
        request.content().clone(),
        OperationPolicy::offline(),
    );
    let error = block_on(prepared.low_level_driver().start_run(
        plan,
        drifted,
        run_fixture.services(),
    ))
    .err()
    .expect("request drift is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.xai.generation_control_mismatch"
    );
    assert_eq!(run_fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(run_fixture.calls.count(DriverCall::CredentialAcquire), 0);
    assert!(run_fixture.server.frames().is_empty());

    let session_fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_xai_responses_websocket(
        session_fixture.preparation_input(),
        &session_fixture.services(),
    )
    .expect("xAI integration prepares");
    let operation = prepared
        .prepare_responses_session(
            XaiSessionProfileInput::new(
                RequestId::new("drift-session").expect("request id"),
                qualified_model("grok-4.6"),
                None,
            )
            .with_reasoning_mode(
                swallowtail_core::ReasoningMode::new("high").expect("mode is valid"),
            )
            .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum is positive")),
        )
        .expect("controlled session prepares");
    let (_, plan, request) = operation.into_parts();
    let drifted = OpenSessionRequest::resource_free_from_plan(
        &plan,
        request.request_id().clone(),
        request.deadline(),
    )
    .expect("drifted session request derives");
    let error = block_on(prepared.low_level_driver().open_session(
        plan,
        drifted,
        session_fixture.services(),
    ))
    .err()
    .expect("session request drift is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.xai.generation_control_mismatch"
    );
    assert_eq!(session_fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(
        session_fixture.calls.count(DriverCall::CredentialAcquire),
        0
    );
    assert!(session_fixture.server.frames().is_empty());
}

#[test]
fn prepared_xai_controls_reach_failed_turns_before_chain_invalidation() {
    for scenario in [
        ServerScenario::PreviousResponseNotFound,
        ServerScenario::Disconnect,
    ] {
        let fixture = DriverFixture::new(scenario);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let operation = prepared
            .prepare_responses_session(
                XaiSessionProfileInput::new(
                    RequestId::new("controlled-failure").expect("request id"),
                    qualified_model("grok-4.6"),
                    None,
                )
                .with_reasoning_mode(
                    swallowtail_core::ReasoningMode::new("high").expect("mode is valid"),
                )
                .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum")),
            )
            .expect("controlled session prepares");
        let mut session =
            block_on(operation.open_session(fixture.services())).expect("session opens");
        let mut turn = block_on(
            session.start_turn(turn_request("controlled-failure-turn"), fixture.services()),
        )
        .expect("turn starts");
        let mut events = turn.take_events().expect("events exist");
        let terminal = turn.take_terminal_outcome().expect("terminal exists");
        let outcome = block_on(async {
            while events.next().await.is_some() {}
            terminal.await
        });
        assert!(matches!(
            outcome.status(),
            TerminalStatus::ProviderFailed(_) | TerminalStatus::RuntimeFailed(_)
        ));
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(
            block_on(fixture.close_session(session)),
            CleanupOutcome::Clean
        );
        let frame = fixture.server.frames().pop().expect("failed turn frame");
        assert_wire_controls(&frame, Some("high"), Some(512), false);
    }
}
