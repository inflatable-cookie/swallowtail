use crate::support;

use futures_executor::block_on;
use support::{CleanupEvent, FixtureHost, Scenario, close_session, selection, version_selection};
use swallowtail_adapter_kimi::KimiAcpDriver;
use swallowtail_core::{ExecutionHostId, ResourceAccess, SessionProviderStatePolicy, SessionRef};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    OperationContent, RequestId, ResumeSessionRequest, RuntimeTurnId, SessionAccessPolicy,
    SessionPlanAgreement, SessionReplayKind, SessionResumeBinding, TerminalStatus, TurnRequest,
    WorkingResourceRef,
};

#[test]
fn qualified_versions_preserve_prompt_write_and_cleanup_authority() {
    for (version, scenario) in [
        ("0.28.1", Scenario::Complete),
        ("0.29.0", Scenario::ReasoningEffortSuccess),
        ("0.29.1", Scenario::ReasoningEffort291Success),
        ("0.29.2", Scenario::ReasoningEffort292Success),
        ("0.30.0", Scenario::ReasoningEffort300Success),
        ("0.31.0", Scenario::ReasoningEffort310Success),
        ("0.31.1", Scenario::ReasoningEffort311Success),
    ] {
        for host_id in topologies() {
            let selected = version_selection(host_id.clone(), version);
            let host = FixtureHost::new(scenario);
            let services = host.services(host_id);
            let driver = driver(selected.credential.clone());
            let mut session = block_on(driver.open_session(
                selected.plan,
                open_request("kimi-open", selected.resource.clone()),
                services.clone(),
            ))
            .expect("session opens");
            let binding = session.resume_binding().expect("binding is available");
            assert_eq!(
                binding.access_policy(),
                &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
            );
            assert_eq!(binding.working_resource(), Some(&selected.resource));
            assert!(!format!("{binding:?}").contains("kimi-session-bound"));
            let models = session
                .negotiated_model_options()
                .expect("session model options are retained");
            assert_eq!(models.current_value(), "kimi-coder");
            assert_eq!(
                models
                    .options()
                    .map(|model| model.value())
                    .collect::<Vec<_>>(),
                ["kimi-coder", "kimi-alternate"]
            );
            let mut turn = block_on(session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new("kimi-turn").expect("valid turn"),
                    OperationContent::new("private fixture prompt").expect("valid prompt"),
                ),
                services.clone(),
            ))
            .expect("turn starts");
            let outcome = block_on(
                turn.take_terminal_outcome()
                    .expect("terminal outcome is available"),
            );
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(
                outcome.output().expect("output is present").as_str(),
                "Kimi fixture response."
            );
            assert_eq!(
                host.resource_writes(),
                [(
                    "src/generated.rs".to_owned(),
                    "pub fn generated() {}\n".to_owned()
                )]
            );
            assert_eq!(host.process_arguments(), Some(vec!["acp".to_owned()]));
            assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
            assert_eq!(
                block_on(close_session(session, services)),
                CleanupOutcome::Clean
            );
            assert_eq!(host.cleanup_counts(), (1, 1));
            assert_eq!(host.cleanup_events(), joined_cleanup());
        }
    }
}

#[test]
fn load_replays_history_but_resume_does_not() {
    for host_id in topologies() {
        let selected = selection(host_id.clone());
        let binding = binding(&selected.plan, selected.resource.clone());
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite);

        let load_host = FixtureHost::new(Scenario::Complete);
        let load_services = load_host.services(host_id.clone());
        let loaded = block_on(driver(selected.credential.clone()).load_session(
            selected.plan.clone(),
            LoadSessionRequest::new(
                RequestId::new("kimi-load").expect("valid request"),
                binding.clone(),
                selected.resource.clone(),
                None,
                plan_agreement(policy.clone()),
            ),
            load_services.clone(),
        ))
        .expect("session loads");
        let (replay, session) = loaded.into_parts();
        assert_eq!(replay.len(), 2);
        assert_eq!(replay[0].sequence(), 0);
        assert_eq!(replay[1].sequence(), 1);
        assert_eq!(replay[0].kind(), SessionReplayKind::UserMessage);
        assert_eq!(replay[1].kind(), SessionReplayKind::AgentMessage);
        assert_eq!(
            replay[1].content().expect("agent replay content").as_str(),
            "Previous answer."
        );
        let replay_debug = format!("{replay:?}");
        assert!(!replay_debug.contains("kimi-session-bound"));
        assert!(!replay_debug.contains("Previous answer"));
        assert_eq!(
            block_on(close_session(session, load_services)),
            CleanupOutcome::Clean
        );
        assert_eq!(load_host.cleanup_events(), joined_cleanup());

        let resume_host = FixtureHost::new(Scenario::Complete);
        let resume_services = resume_host.services(host_id);
        let session = block_on(driver(selected.credential).resume_session(
            selected.plan,
            ResumeSessionRequest::new(
                RequestId::new("kimi-resume").expect("valid request"),
                binding,
                selected.resource,
                None,
                plan_agreement(policy),
            ),
            resume_services.clone(),
        ))
        .expect("session resumes without replay");
        assert_eq!(
            block_on(close_session(session, resume_services)),
            CleanupOutcome::Clean
        );
        assert_eq!(resume_host.cleanup_events(), joined_cleanup());
    }
}

#[test]
fn binding_mismatch_fails_before_credentials_or_processes() {
    let host_id = ExecutionHostId::new("fixture.host.rejected").expect("valid host id");
    let selected = selection(host_id.clone());
    let binding = binding(&selected.plan, selected.resource);
    let host = FixtureHost::new(Scenario::Complete);
    let result = block_on(driver(selected.credential).load_session(
        selected.plan,
        LoadSessionRequest::new(
            RequestId::new("kimi-rejected-load").expect("valid request"),
            binding,
            WorkingResourceRef::new("different.resource").expect("valid resource"),
            None,
            plan_agreement(SessionAccessPolicy::ambient_harness(
                ResourceAccess::ReadWrite,
            )),
        ),
        host.services(host_id),
    ));

    assert!(result.is_err());
    assert_eq!(host.credential_acquisitions(), 0);
    assert!(!host.process_started());
}

#[test]
fn active_turn_cancellation_uses_acp_and_keeps_cleanup_joined() {
    for host_id in topologies() {
        let selected = selection(host_id.clone());
        let host = FixtureHost::new(Scenario::HoldPrompt);
        let services = host.services(host_id);
        let mut session = block_on(driver(selected.credential).open_session(
            selected.plan,
            open_request("kimi-cancel-open", selected.resource),
            services.clone(),
        ))
        .expect("session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("kimi-cancel-turn").expect("valid turn"),
                OperationContent::new("cancel this turn").expect("valid prompt"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        block_on(turn.cancellation().request()).expect("cancellation is sent");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(
            block_on(close_session(session, services)),
            CleanupOutcome::Clean
        );
        assert_eq!(host.cleanup_counts(), (1, 1));
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }
}

#[test]
fn disconnect_fails_the_turn_and_session_close_still_joins_cleanup() {
    for host_id in topologies() {
        let selected = selection(host_id.clone());
        let host = FixtureHost::new(Scenario::DisconnectPrompt);
        let services = host.services(host_id);
        let mut session = block_on(driver(selected.credential).open_session(
            selected.plan,
            open_request("kimi-disconnect-open", selected.resource),
            services.clone(),
        ))
        .expect("session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("kimi-disconnect-turn").expect("valid turn"),
                OperationContent::new("disconnect this turn").expect("valid prompt"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(
            block_on(close_session(session, services)),
            CleanupOutcome::Clean
        );
        assert_eq!(host.cleanup_counts(), (1, 1));
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }
}

fn topologies() -> [ExecutionHostId; 2] {
    [
        ExecutionHostId::new("fixture.host.local").expect("valid local host id"),
        ExecutionHostId::new("fixture.host.remote-authoritative").expect("valid remote host id"),
    ]
}

fn joined_cleanup() -> [CleanupEvent; 3] {
    [
        CleanupEvent::ProcessWait,
        CleanupEvent::ResourceRelease,
        CleanupEvent::CredentialRelease,
    ]
}

fn driver(credential: swallowtail_core::CredentialRef) -> KimiAcpDriver {
    KimiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("kimi.fixture.isolated-state")
            .expect("valid environment"),
        credential,
    )
}

fn open_request(id: &str, resource: WorkingResourceRef) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("valid request"),
        resource,
        None,
        plan_agreement(SessionAccessPolicy::ambient_harness(
            ResourceAccess::ReadWrite,
        )),
    )
}

fn plan_agreement(policy: SessionAccessPolicy) -> SessionPlanAgreement {
    SessionPlanAgreement::explicit(policy, Some(SessionProviderStatePolicy::Prohibited), None)
}

fn binding(
    plan: &swallowtail_core::PreflightPlan,
    resource: WorkingResourceRef,
) -> SessionResumeBinding {
    SessionResumeBinding::new(
        SessionRef::new("kimi-session-bound").expect("valid session"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route exists").clone(),
        plan.model_id().expect("model exists").clone(),
        resource,
        SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
    )
}
