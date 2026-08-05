mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaConversationProfileInput, AlibabaRetainedConversationProfileInput,
    AlibabaRunProfileInput, AlibabaSessionManagementInput, ENDPOINT_AUDIENCE, EXACT_MODEL_ID,
    MODEL_ROUTE_ID, prepare_alibaba_model_studio,
};
use swallowtail_core::{
    EntitlementMetering, ExecutionHostId, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, ProviderSessionBindingOrigin, ProviderSessionEffectTruth,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, OperationContent, PreparationStage, RequestId,
    RuntimeTurnId, TerminalStatus, TurnRequest, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};
use swallowtail_testkit::{
    assert_observable_activity_trace, assert_prepared_operation_evidence_matches_plan,
};

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
            let (collected, outcome) = block_on(async {
                let mut collected = Vec::new();
                while let Some(event) = events.next().await {
                    collected.push(event.expect("event succeeds"));
                }
                (collected, terminal.await)
            });
            assert_observable_activity_trace(
                conversation.evidence().observable_activity(),
                &collected,
            );
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
fn delete_on_close_conversation_restoration_opens_a_fresh_replacement() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("Alibaba integration prepares");
    let conversation = prepared
        .prepare_conversation(profile_input("replacement"))
        .expect("conversation prepares");
    let interrupted = RuntimeTurnId::new("alibaba-interrupted").expect("turn id");
    let restoration = conversation.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
        panic!("fresh session replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let (_, replacement) = replacement.into_parts();
    assert!(replacement.provider_session_ref().is_none());
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);
    let requests = fixture.requests();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].target, "/compatible-mode/v1/conversations");
    assert_eq!(requests[1].method, "GET");
    assert!(
        requests[2..6]
            .iter()
            .all(|request| request.method == "DELETE" && request.target.contains("/items/"))
    );
    assert_eq!(requests[6].method, "DELETE");
    assert_eq!(
        requests[6].target,
        "/compatible-mode/v1/conversations/conv_fixture_01"
    );
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
        let (collected, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            (collected, terminal.await)
        });
        assert_observable_activity_trace(run.evidence().observable_activity(), &collected);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.response_attempts(), 1);
        assert_eq!(fixture.requests().len(), 1);
        assert_eq!(fixture.releases(), 1);
    }
}

#[test]
fn retained_prepared_session_loads_replay_and_preserves_both_attachments() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let retained = prepared
        .prepare_retained_conversation(retained_profile_input("prepared-retained"))
        .expect("retained conversation prepares");
    assert_eq!(
        retained
            .plan()
            .requirements()
            .session_provider_state_policy(),
        Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    );
    assert!(
        retained
            .plan()
            .requirements()
            .capabilities()
            .any(|required| { required.capability() == swallowtail_core::Capability::LoadSession })
    );
    assert!(
        !retained
            .plan()
            .requirements()
            .capabilities()
            .any(|required| {
                required.capability() == swallowtail_core::Capability::OwnedRemoteResourceDeletion
            })
    );

    let session = block_on(retained.open_session(fixture.services())).expect("session opens");
    let resume = session.resume_binding().expect("resume binding").clone();
    assert_eq!(
        session
            .management_binding()
            .expect("management binding")
            .origin(),
        ProviderSessionBindingOrigin::Created
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let loaded = block_on(
        retained
            .load_session(
                RequestId::new("prepared-load").expect("request id"),
                resume,
                fixture.services(),
            )
            .expect("load prepares"),
    )
    .expect("retained session loads");
    assert_eq!(loaded.replay().len(), 4);
    let (_, loaded) = loaded.into_parts();
    assert_eq!(
        loaded
            .management_binding()
            .expect("loaded management binding")
            .origin(),
        ProviderSessionBindingOrigin::Loaded
    );
    assert_eq!(block_on(loaded.close()), CleanupOutcome::Clean);
    assert!(
        fixture
            .requests()
            .iter()
            .all(|request| request.method != "DELETE")
    );
    assert_eq!(fixture.releases(), 2);
}

#[test]
fn retained_cleanup_requires_separate_management_authority() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let retained = prepared
        .prepare_retained_conversation(retained_profile_input("cleanup-source"))
        .expect("retained conversation prepares");
    let session = block_on(retained.open_session(fixture.services())).expect("session opens");
    let management = session
        .management_binding()
        .expect("management binding")
        .clone();
    assert!(session.resume_binding().is_some());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.requests().len(), 1);

    let deletion = prepared
        .prepare_delete_retained_conversation(AlibabaSessionManagementInput::new(
            RequestId::new("delete-retained").expect("request id"),
            management,
        ))
        .expect("deletion prepares");
    let outcome = block_on(deletion.execute(fixture.services())).expect("deletion executes");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert!(outcome.diagnostic().is_none());
    let requests = fixture.requests();
    assert_eq!(requests[1].method, "GET");
    assert!(requests[2].target.ends_with("after=msg_output_01"));
    assert!(
        requests[3..7]
            .iter()
            .all(|request| request.method == "DELETE" && request.target.contains("/items/"))
    );
    assert_eq!(requests[7].method, "DELETE");
    assert_eq!(fixture.releases(), 2);
}

#[test]
fn retained_cleanup_cancellation_and_binding_drift_fail_before_effects() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let retained = prepared
        .prepare_retained_conversation(retained_profile_input("cancel-source"))
        .expect("retained conversation prepares");
    let session = block_on(retained.open_session(fixture.services())).expect("session opens");
    let management = session
        .management_binding()
        .expect("management binding")
        .clone();
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let deletion = prepared
        .prepare_delete_retained_conversation(AlibabaSessionManagementInput::new(
            RequestId::new("cancel-delete").expect("request id"),
            management.clone(),
        ))
        .expect("deletion prepares");
    block_on(deletion.request().cancellation().request()).expect("cancellation requests");
    let outcome = block_on(deletion.execute(fixture.services())).expect("cancellation is evidence");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert_eq!(fixture.requests().len(), 1);

    let foreign = DriverFixture::for_host(
        ServerScenario::Success,
        ExecutionHostId::new("alibaba.foreign.host").expect("host id"),
    );
    let foreign_prepared =
        prepare_alibaba_model_studio(foreign.preparation_input(), &foreign.services())
            .expect("foreign integration prepares");
    let failure = foreign_prepared
        .prepare_delete_retained_conversation(AlibabaSessionManagementInput::new(
            RequestId::new("drift-delete").expect("request id"),
            management,
        ))
        .expect_err("cross-host binding rejects");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
    assert!(foreign.requests().is_empty());
}

#[test]
fn retained_cleanup_preserves_after_effect_uncertainty() {
    let fixture = DriverFixture::new(ServerScenario::ManagedDeleteFailure);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let retained = prepared
        .prepare_retained_conversation(retained_profile_input("uncertain-source"))
        .expect("retained conversation prepares");
    let session = block_on(retained.open_session(fixture.services())).expect("session opens");
    let management = session
        .management_binding()
        .expect("management binding")
        .clone();
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let deletion = prepared
        .prepare_delete_retained_conversation(AlibabaSessionManagementInput::new(
            RequestId::new("uncertain-delete").expect("request id"),
            management,
        ))
        .expect("deletion prepares");
    let outcome = block_on(deletion.execute(fixture.services())).expect("uncertainty is evidence");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert!(outcome.diagnostic().is_some());
    assert!(!fixture.requests().iter().any(|request| {
        request.method == "DELETE"
            && request.target == "/compatible-mode/v1/conversations/conv_fixture_01"
    }));
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

fn retained_profile_input(id: &str) -> AlibabaRetainedConversationProfileInput {
    AlibabaRetainedConversationProfileInput::new(
        RequestId::new(id).expect("request id"),
        ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
        ModelRouteRevision::new("2026-08-05").expect("route revision"),
        ModelId::new(EXACT_MODEL_ID).expect("model id"),
    )
}
