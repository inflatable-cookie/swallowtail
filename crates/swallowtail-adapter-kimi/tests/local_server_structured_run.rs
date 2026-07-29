#[allow(dead_code)]
#[path = "local_server_interactive/fixture.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "local_server_interactive_support/mod.rs"]
mod interactive_support;
#[allow(dead_code)]
#[path = "local_server_lifecycle_support/mod.rs"]
mod lifecycle_support;

use fixture::{attached_input, id, prepare, probe};
use futures_executor::block_on;
use futures_util::StreamExt;
use interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use lifecycle_support::FixtureHost;
use swallowtail_adapter_kimi::{
    KimiLocalServerOwnedInput, KimiLocalServerPermissionMode, KimiLocalServerRunInput,
    KimiLocalServerSessionConfiguration, KimiModelSelection, start_kimi_local_server_owned,
};
use swallowtail_core::{
    DriverRole, ExecutionHostId, InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision,
    OperationShape, ReasoningMode,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, CancellationAcknowledgement, CleanupOutcome,
    Deadline, MonotonicInstant, OperationContent, OperationPolicy, ProviderCancellationOutcome,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RequestId, StreamReattachmentPolicy,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::{
    ExecutionTopologyFixture, assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn attached_run_completes_once_and_preserves_provider_session_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for version in ["0.29.1", "0.29.2"] {
            let server = InteractiveFixtureServer::start_with_version(
                InteractiveScenario::Complete,
                version,
            );
            let host = FixtureHost::for_endpoint(server.endpoint());
            let execution_host = topology.execution_host_id().clone();
            let services = host.services(execution_host.clone(), false);
            let prepared = prepare(execution_host, services.clone(), version);
            let profile = prepared
                .prepare_run(run_input(
                    &format!("complete-{version}"),
                    KimiLocalServerPermissionMode::Auto,
                ))
                .expect("run prepares");
            assert_eq!(
                profile.plan().requirements().driver_role(),
                DriverRole::StructuredRun
            );
            assert_eq!(
                profile.plan().requirements().operation_shape(),
                OperationShape::StructuredRun
            );
            assert_eq!(
                profile.request().policy().provider_retention(),
                ProviderRetentionPolicy::DurableAllowed
            );
            assert_eq!(
                profile.request().policy().provider_recovery(),
                ProviderRecoveryPolicy::ManagedAllowed
            );
            assert_prepared_operation_evidence_matches_plan(profile.evidence(), profile.plan());

            let mut run = block_on(profile.start_run(services)).expect("structured run starts");
            assert!(run.provider_run_ref().is_none());
            assert!(run.take_callbacks().is_none());
            let events = block_on(
                run.take_events()
                    .expect("events are available")
                    .collect::<Vec<_>>(),
            );
            assert!(events.iter().all(Result::is_ok));
            let outcome = block_on(
                run.take_terminal_outcome()
                    .expect("terminal outcome is available"),
            );
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(
                outcome.output().map(OperationContent::as_str),
                Some("fixture result")
            );
            assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
            assert_eq!(host.credential_releases(), 2);
            let requests = server.requests();
            assert_eq!(
                requests
                    .iter()
                    .filter(|request| request.contains("POST /api/v1/sessions "))
                    .count(),
                1
            );
            assert_eq!(
                requests
                    .iter()
                    .filter(|request| request.contains("/prompts"))
                    .count(),
                1
            );
            for forbidden in [":archive", ":restore", ":delete", "DELETE "] {
                assert!(
                    !requests.iter().any(|request| request.contains(forbidden)),
                    "run close must not perform {forbidden}"
                );
            }
        }
    }
}

#[test]
fn managed_retry_and_one_cursor_reattachment_preserve_one_prompt() {
    for (scenario, reattach) in [
        (InteractiveScenario::Retry, false),
        (InteractiveScenario::Reattach, true),
    ] {
        let server = InteractiveFixtureServer::start_with_version(scenario, "0.29.2");
        let host = FixtureHost::for_endpoint(server.endpoint());
        let execution_host = id(ExecutionHostId::new, "fixture.kimi.retained");
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare(execution_host, services.clone(), "0.29.2");
        let mut input = run_input("retained", KimiLocalServerPermissionMode::Auto);
        if reattach {
            input = input.with_one_stream_reattachment();
        }
        let profile = prepared.prepare_run(input).expect("run prepares");
        assert_eq!(
            profile.request().policy().provider_recovery(),
            ProviderRecoveryPolicy::ManagedAllowed
        );
        assert_eq!(
            profile.request().policy().stream_reattachment(),
            if reattach {
                StreamReattachmentPolicy::Bounded(
                    std::num::NonZeroU32::new(1).expect("one is non-zero"),
                )
            } else {
                StreamReattachmentPolicy::Disabled
            }
        );
        let mut run = block_on(profile.start_run(services)).expect("run starts");
        let events = block_on(
            run.take_events()
                .expect("events are available")
                .collect::<Vec<_>>(),
        );
        assert!(events.iter().all(Result::is_ok));
        let outcome = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let requests = server.requests();
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.contains("/prompts"))
                .count(),
            1
        );
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("WS /api/v1/ws"))
                .count(),
            if reattach { 2 } else { 1 }
        );
    }
}

#[test]
fn manual_approval_and_question_callbacks_remain_explicit() {
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
        let server = InteractiveFixtureServer::start_with_version(scenario, "0.29.2");
        let host = FixtureHost::for_endpoint(server.endpoint());
        let execution_host = id(ExecutionHostId::new, "fixture.kimi.structured.callback");
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare(execution_host, services.clone(), "0.29.2");
        let profile = prepared
            .prepare_run(run_input("callback", KimiLocalServerPermissionMode::Manual))
            .expect("manual run prepares");
        let mut run = block_on(profile.start_run(services)).expect("run starts");
        let mut callbacks = run.take_callbacks().expect("callbacks are exposed");
        let mut requests = callbacks
            .take_requests()
            .expect("callback request stream exists");
        let callback = block_on(requests.next())
            .expect("callback arrives")
            .expect("callback is valid");
        block_on(
            callbacks.responder().respond(CallbackResponse::new(
                callback.callback_id().clone(),
                swallowtail_runtime::RuntimeTurnId::new("kimi-local:run:callback")
                    .expect("turn id is valid"),
                CallbackResult::Success(
                    CallbackPayload::new(response.to_vec(), 512).expect("payload is bounded"),
                ),
            )),
        )
        .expect("callback response succeeds");
        let outcome = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn retention_mismatch_fails_before_session_creation() {
    let server =
        InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, "0.29.2");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.structured.retention");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.2");
    let profile = prepared
        .prepare_run(run_input("retention", KimiLocalServerPermissionMode::Auto))
        .expect("run prepares");
    let requests_before = server.requests();
    let request = StructuredRunRequest::new(
        profile.request().request_id().clone(),
        profile.request().content().clone(),
        OperationPolicy::offline()
            .with_harness_isolation(swallowtail_core::HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(
                swallowtail_core::HarnessConfigurationPosture::Ambient,
            ),
    )
    .with_working_resource(
        profile
            .request()
            .working_resource()
            .expect("resource is bound")
            .clone(),
    )
    .with_deadline(profile.request().deadline().expect("deadline is bound"));
    let result = block_on(profile.low_level_driver().start_run(
        profile.plan().clone(),
        request,
        services,
    ));
    let error = result.err().expect("retention mismatch rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi.local_server.run_input_unsupported"
    );
    assert_eq!(server.requests(), requests_before);
}

#[test]
fn cancellation_timeout_disconnect_and_reasoning_keep_exact_truth() {
    let cancelled = scenario_outcome(InteractiveScenario::Cancel, true, false);
    assert_eq!(cancelled.status(), &TerminalStatus::Cancelled);
    assert_eq!(
        cancelled.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );

    let disconnected = scenario_outcome(InteractiveScenario::Disconnect, false, false);
    assert!(matches!(
        disconnected.status(),
        TerminalStatus::RuntimeFailed(diagnostic)
            if diagnostic.code() == "swallowtail.kimi.local_server.websocket_disconnected"
    ));

    let timed_out = scenario_outcome(InteractiveScenario::Cancel, false, true);
    assert_eq!(timed_out.status(), &TerminalStatus::TimedOut);

    let server =
        InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, "0.29.2");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.structured.reasoning");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.2");
    let profile = prepared
        .prepare_run(
            run_input("reasoning", KimiLocalServerPermissionMode::Auto)
                .with_reasoning(ReasoningMode::new("high").expect("reasoning is valid")),
        )
        .expect("reasoning run prepares");
    let mut run = block_on(profile.start_run(services)).expect("reasoning run starts");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(server.requests().iter().any(|request| {
        request.contains("/prompts") && request.contains(r#""thinking":"high""#)
    }));
}

#[test]
fn owned_run_joins_operation_before_foreground_server() {
    let server =
        InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, "0.29.2");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.structured.owned");
    let services = host.services(execution_host.clone(), true);
    let owned = block_on(start_kimi_local_server_owned(
        KimiLocalServerOwnedInput::new(
            attached_input(execution_host, "0.29.2"),
            id(InstanceTargetRef::new, "fixture.kimi.executable"),
        ),
        probe(),
        services.clone(),
    ))
    .expect("owned server starts");
    let profile = owned
        .prepared()
        .prepare_run(run_input("owned", KimiLocalServerPermissionMode::Auto))
        .expect("owned run prepares");
    let mut run = block_on(profile.start_run(services)).expect("owned run starts");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(!host.process_stopped_and_joined());
    assert_eq!(block_on(owned.close()), CleanupOutcome::Clean);
    assert!(host.process_stopped_and_joined());
}

fn scenario_outcome(scenario: InteractiveScenario, cancel: bool, timeout: bool) -> TerminalOutcome {
    let server = InteractiveFixtureServer::start_with_version(scenario, "0.29.2");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.structured.outcome");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.2");
    let profile = prepared
        .prepare_run(run_input("outcome", KimiLocalServerPermissionMode::Auto))
        .expect("run prepares");
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    if cancel {
        assert_eq!(
            block_on(run.cancellation().request()).expect("cancellation succeeds"),
            CancellationAcknowledgement::Requested
        );
    }
    if timeout {
        host.set_now(100);
    }
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    outcome
}

fn run_input(id_value: &str, permission: KimiLocalServerPermissionMode) -> KimiLocalServerRunInput {
    KimiLocalServerRunInput::new(
        id(RequestId::new, id_value),
        KimiModelSelection::new(
            id(ModelRouteId::new, &format!("fixture.kimi.{id_value}")),
            id(ModelRouteRevision::new, "1"),
            id(ModelId::new, "kimi-k2.5"),
        ),
        OperationContent::new("fixture structured prompt").expect("content is valid"),
        id(WorkingResourceRef::new, "fixture.kimi.workspace"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        KimiLocalServerSessionConfiguration::new(permission),
    )
    .accept_managed_recovery()
}
