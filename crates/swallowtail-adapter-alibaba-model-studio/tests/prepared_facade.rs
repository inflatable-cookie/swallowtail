mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaConversationProfileInput, AlibabaRunProfileInput, ENDPOINT_AUDIENCE, EXACT_MODEL_ID,
    MODEL_ROUTE_ID, prepare_alibaba_model_studio,
};
use swallowtail_core::{
    EntitlementMetering, ExecutionHostId, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, PreparationStage, RequestId, RuntimeTurnId, TerminalStatus,
    TurnRequest,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn exact_conversation_lifecycle_runs_on_both_host_topologies() {
    for host_id in [
        ExecutionHostId::new("alibaba.prepared.local").expect("host id"),
        ExecutionHostId::new("alibaba.prepared.remote-authoritative").expect("host id"),
    ] {
        let fixture = DriverFixture::for_host(ServerScenario::Success, host_id);
        let prepared =
            prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
                .expect("Alibaba integration prepares");
        assert_eq!(
            prepared.access_profile().endpoint_audience().as_str(),
            ENDPOINT_AUDIENCE
        );
        let conversation = prepared
            .prepare_conversation(profile_input("prepared-conversation"))
            .expect("conversation prepares");
        assert_eq!(
            conversation.plan().model_id().expect("model").as_str(),
            EXACT_MODEL_ID
        );
        assert_eq!(
            conversation
                .plan()
                .requirements()
                .session_provider_state_policy(),
            Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose)
        );
        assert_prepared_operation_evidence_matches_plan(
            conversation.evidence().operation(),
            conversation.plan(),
        );

        let mut session =
            block_on(conversation.open_session(fixture.services())).expect("session opens");
        for (turn_id, content) in [
            ("prepared-turn-one", "first prepared turn"),
            ("prepared-turn-two", "second prepared turn"),
        ] {
            let mut turn = block_on(session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(turn_id).expect("turn id"),
                    OperationContent::new(content).expect("content"),
                ),
                fixture.services(),
            ))
            .expect("turn starts");
            let mut events = turn.take_events().expect("events");
            let terminal = turn.take_terminal_outcome().expect("terminal");
            let outcome = block_on(async {
                while let Some(event) = events.next().await {
                    event.expect("event succeeds");
                }
                terminal.await
            });
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        }
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.response_attempts(), 2);
        assert_eq!(fixture.releases(), 1);
        assert_eq!(fixture.release_after_blocking(), [9]);

        let requests = fixture.requests();
        let cleanup: Vec<_> = requests[3..]
            .iter()
            .map(|request| (request.method.as_str(), request.target.as_str()))
            .collect();
        assert_eq!(cleanup[0].0, "GET");
        assert!(
            cleanup[1..5]
                .iter()
                .all(|(method, target)| *method == "DELETE" && target.contains("/items/"))
        );
        assert_eq!(
            cleanup[5],
            (
                "DELETE",
                "/compatible-mode/v1/conversations/conv_fixture_01"
            )
        );
    }
}

#[test]
fn resource_free_structured_run_prepares_on_both_host_topologies() {
    for host_id in [
        ExecutionHostId::new("alibaba.run.local").expect("host id"),
        ExecutionHostId::new("alibaba.run.remote-authoritative").expect("host id"),
    ] {
        let fixture = DriverFixture::for_host(ServerScenario::Success, host_id);
        let prepared =
            prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
                .expect("Alibaba integration prepares");
        let run = prepared
            .prepare_run(AlibabaRunProfileInput::new(
                RequestId::new("prepared-run").expect("request id"),
                ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
                ModelRouteRevision::new("2026-07-22").expect("route revision"),
                ModelId::new(EXACT_MODEL_ID).expect("model"),
                OperationContent::new("one prepared request").expect("content"),
            ))
            .expect("structured run prepares");
        assert_eq!(
            run.plan().requirements().driver_role(),
            swallowtail_core::DriverRole::StructuredRun
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());
        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        let mut events = handle.take_events().expect("events");
        let terminal = handle.take_terminal_outcome().expect("terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.response_attempts(), 1);
        assert_eq!(fixture.requests().len(), 1);
        assert_eq!(fixture.releases(), 1);
    }
}

#[test]
fn plan_access_retention_model_and_target_drift_fail_before_effects() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let failure = prepare_alibaba_model_studio(
        fixture.preparation_input_with_metering(EntitlementMetering::SubscriptionAllowance),
        &fixture.services(),
    )
    .expect_err("plan-key metering is not general API access");
    assert_eq!(failure.stage(), PreparationStage::AccessEvidence);

    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let failure = prepared
        .prepare_conversation(AlibabaConversationProfileInput::new(
            RequestId::new("wrong-retention").expect("request id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("1").expect("revision"),
            ModelId::new(EXACT_MODEL_ID).expect("model"),
            SessionProviderStatePolicy::Prohibited,
        ))
        .expect_err("retention must be explicit");
    assert_eq!(failure.stage(), PreparationStage::Preflight);

    let failure = prepared
        .prepare_conversation(AlibabaConversationProfileInput::new(
            RequestId::new("wrong-model").expect("request id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("1").expect("revision"),
            ModelId::new("qwen-compatible-alias").expect("model"),
            SessionProviderStatePolicy::DurableConversationDeleteOnClose,
        ))
        .expect_err("model alias rejects");
    assert_eq!(failure.stage(), PreparationStage::Preflight);

    let failure = prepared
        .validate_execution_binding(
            prepared.instance().execution_host_id(),
            &InstanceTargetRef::new("another.workspace").expect("target"),
        )
        .expect_err("workspace target drift rejects");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
    assert!(fixture.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

fn profile_input(id: &str) -> AlibabaConversationProfileInput {
    AlibabaConversationProfileInput::new(
        RequestId::new(id).expect("request id"),
        ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
        ModelRouteRevision::new("2026-07-22").expect("route revision"),
        ModelId::new(EXACT_MODEL_ID).expect("model id"),
        SessionProviderStatePolicy::DurableConversationDeleteOnClose,
    )
}
