mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::AlibabaModelStudioDriver;
use swallowtail_core::{CredentialMechanism, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent,
    ProviderObservation, RequestId, RuntimeEventKind, RuntimeTurnId, TerminalStatus, TurnRequest,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile, run_all_synthetic_profiles,
    run_provider_conversation_boundary_assertions,
};

#[test]
fn provider_neutral_conversation_assertions_remain_adapter_independent() {
    run_provider_conversation_boundary_assertions();
}

#[test]
fn local_and_remote_authority_preserve_the_exact_conversation_lifecycle() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = DriverFixture::for_host(
            ServerScenario::Success,
            topology.execution_host_id().clone(),
        );
        let plan = fixture.plan();
        assert_eq!(plan.execution_host_id(), topology.execution_host_id());
        assert_eq!(
            plan.instance_id().as_str(),
            "alibaba-model-studio.sg.workspace-dedicated"
        );
        assert_eq!(
            plan.instance_target_ref().as_host_value(),
            "alibaba-model-studio.sg.workspace-endpoint"
        );
        assert_eq!(
            plan.endpoint_audience().as_str(),
            "model-studio.workspace.ap-southeast-1"
        );
        assert_eq!(plan.credential_mechanism(), &CredentialMechanism::ApiKey);
        assert_eq!(
            plan.model_route_id().expect("route").as_str(),
            "alibaba-model-studio.sg.qwen3.7-plus-2026-05-26"
        );
        assert_eq!(
            plan.model_id().expect("model").as_str(),
            "qwen3.7-plus-2026-05-26"
        );

        let request = OpenSessionRequest::resource_free(
            RequestId::new("topology-session").expect("request id"),
            None,
        )
        .with_provider_state_policy(SessionProviderStatePolicy::DurableConversationDeleteOnClose);
        let mut session = block_on(AlibabaModelStudioDriver::new().open_session(
            plan,
            request,
            fixture.services(),
        ))
        .expect("session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("topology-turn").expect("turn id"),
                OperationContent::new("private topology input").expect("content"),
            ),
            fixture.services(),
        ))
        .expect("turn starts");
        let mut events = turn.take_events().expect("events exist");
        let terminal = turn.take_terminal_outcome().expect("terminal exists");
        let (events, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event"));
            }
            (collected, terminal.await)
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
        )));
        assert!(!format!("{events:?}").contains("private topology input"));
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases(), 1);
    }
}

#[test]
fn all_provider_neutral_profiles_keep_the_common_contract() {
    let reports = run_all_synthetic_profiles();
    assert_eq!(reports.len(), 12);
    assert_eq!(
        reports
            .iter()
            .filter(|report| report.profile() == SyntheticProfile::HostedDirectApi)
            .count(),
        1
    );
    for report in reports {
        assert!(report.covers(ConformanceAssertion::PreflightBeforeSideEffects));
        assert!(report.covers(ConformanceAssertion::NoImplicitFallback));
        assert!(report.covers(ConformanceAssertion::Redaction));
    }
}
