mod support;

use futures_executor::block_on;
use support::{CleanupEvent, FixtureHost, Scenario, selection};
use swallowtail_adapter_kimi::KimiAcpDriver;
use swallowtail_core::{ExecutionHostId, ResourceAccess, SessionRef};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    OperationContent, RequestId, ResumeSessionRequest, RuntimeTurnId, SessionAccessPolicy,
    SessionReplayKind, SessionResumeBinding, TerminalStatus, TurnRequest, WorkingResourceRef,
};

#[test]
fn new_prompt_write_and_cleanup_preserve_ambient_host_authority() {
    for host_id in topologies() {
        let selected = selection(host_id.clone());
        let host = FixtureHost::new(Scenario::Complete);
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
        assert_eq!(binding.working_resource(), &selected.resource);
        assert!(!format!("{binding:?}").contains("kimi-session-bound"));
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("kimi-turn").expect("valid turn"),
                OperationContent::new("private fixture prompt").expect("valid prompt"),
            ),
            services,
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
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(host.cleanup_counts(), (1, 1));
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }
}

#[test]
fn load_replays_history_but_resume_does_not() {
    for host_id in topologies() {
        let selected = selection(host_id.clone());
        let binding = binding(&selected.plan, selected.resource.clone());
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite);

        let load_host = FixtureHost::new(Scenario::Complete);
        let loaded = block_on(
            driver(selected.credential.clone()).load_session(
                selected.plan.clone(),
                LoadSessionRequest::new(
                    RequestId::new("kimi-load").expect("valid request"),
                    binding.clone(),
                    selected.resource.clone(),
                    None,
                )
                .with_access_policy(policy.clone()),
                load_host.services(host_id.clone()),
            ),
        )
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
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(load_host.cleanup_events(), joined_cleanup());

        let resume_host = FixtureHost::new(Scenario::Complete);
        let session = block_on(
            driver(selected.credential).resume_session(
                selected.plan,
                ResumeSessionRequest::new(
                    RequestId::new("kimi-resume").expect("valid request"),
                    binding,
                    selected.resource,
                    None,
                )
                .with_access_policy(policy),
                resume_host.services(host_id),
            ),
        )
        .expect("session resumes without replay");
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(resume_host.cleanup_events(), joined_cleanup());
    }
}

#[test]
fn binding_mismatch_fails_before_credentials_or_processes() {
    let host_id = ExecutionHostId::new("fixture.host.rejected").expect("valid host id");
    let selected = selection(host_id.clone());
    let binding = binding(&selected.plan, selected.resource);
    let host = FixtureHost::new(Scenario::Complete);
    let result = block_on(
        driver(selected.credential).load_session(
            selected.plan,
            LoadSessionRequest::new(
                RequestId::new("kimi-rejected-load").expect("valid request"),
                binding,
                WorkingResourceRef::new("different.resource").expect("valid resource"),
                None,
            )
            .with_access_policy(SessionAccessPolicy::ambient_harness(
                ResourceAccess::ReadWrite,
            )),
            host.services(host_id),
        ),
    );

    assert!(result.is_err());
    assert_eq!(host.credential_acquisitions(), 0);
    assert!(!host.process_started());
}

#[test]
fn active_turn_cancellation_uses_acp_and_keeps_cleanup_joined() {
    let host_id = ExecutionHostId::new("fixture.host.cancel").expect("valid host id");
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
        services,
    ))
    .expect("turn starts");
    block_on(turn.cancellation().request()).expect("cancellation is sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.cleanup_counts(), (1, 1));
    assert_eq!(host.cleanup_events(), joined_cleanup());
}

#[test]
fn disconnect_fails_the_turn_and_session_close_still_joins_cleanup() {
    let host_id = ExecutionHostId::new("fixture.host.disconnect").expect("valid host id");
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
        services,
    ))
    .expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.cleanup_counts(), (1, 1));
    assert_eq!(host.cleanup_events(), joined_cleanup());
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
    OpenSessionRequest::new(RequestId::new(id).expect("valid request"), resource, None)
        .with_access_policy(SessionAccessPolicy::ambient_harness(
            ResourceAccess::ReadWrite,
        ))
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
