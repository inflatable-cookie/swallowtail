mod support;

use futures_executor::block_on;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::topology::{driver, open_request, plan_for, request_id};
use support::{host_services_for, session_resume_binding_for};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OperationContent, ResumeSessionRequest,
    RuntimeFailure, RuntimeTurnId, SessionAccessPolicy, TerminalStatus, TurnRequest,
};
use swallowtail_testkit::ExecutionTopologyFixture;

fn expect_failure<T>(result: Result<T, RuntimeFailure>, message: &str) -> RuntimeFailure {
    match result {
        Ok(_) => panic!("{message}"),
        Err(failure) => failure,
    }
}

#[test]
fn local_and_remote_hosts_bind_open_resume_process_and_resource_authority() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let plan = plan_for(&topology, []);
        let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let services = host_services_for(topology.execution_host_id().clone(), process);
        let session =
            block_on(driver().open_session(plan, open_request("open", &topology), services))
                .expect("session opens on the selected host");
        let binding = session
            .resume_binding()
            .expect("open session exposes a resume binding")
            .clone();

        assert_eq!(binding.execution_host_id(), topology.execution_host_id());
        assert_eq!(
            binding.configured_instance_id(),
            topology.configured_instance_id()
        );
        let observed = state.request();
        assert_eq!(
            observed.executable,
            topology.instance_target().as_host_value()
        );
        assert_eq!(
            observed.working_resource.as_deref(),
            Some(topology.working_resource().as_host_value())
        );
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert!(state.waited());

        let plan = plan_for(&topology, []);
        let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let services = host_services_for(topology.execution_host_id().clone(), process);
        let mut resumed = block_on(
            driver().resume_session(
                plan,
                ResumeSessionRequest::new(
                    request_id("resume", &topology),
                    binding.clone(),
                    topology.working_resource().clone(),
                    None,
                )
                .with_access_policy(SessionAccessPolicy::read_only()),
                services.clone(),
            ),
        )
        .expect("session resumes on its bound host");
        assert_eq!(resumed.resume_binding(), Some(&binding));
        let resume = state
            .messages()
            .into_iter()
            .find(|message| message["method"] == "thread/resume")
            .expect("thread/resume is sent");
        assert_eq!(
            resume["params"]["threadId"],
            binding.provider_session_ref().as_provider_value()
        );

        let mut turn = block_on(resumed.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("turn-after-resume").expect("turn id is valid"),
                OperationContent::new("continue").expect("content is valid"),
            ),
            services,
        ))
        .expect("resumed turn starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
        assert!(state.waited());
    }
}

#[test]
fn host_instance_and_provider_session_substitution_fail_at_the_boundary() {
    let local = ExecutionTopologyFixture::local();
    let remote = ExecutionTopologyFixture::remote_authoritative();

    let local_plan = plan_for(&local, []);
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let failure = expect_failure(
        block_on(driver().open_session(
            local_plan,
            open_request("wrong-host", &local),
            host_services_for(remote.execution_host_id().clone(), process),
        )),
        "services from another host must fail",
    );
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!state.started());
    assert!(!format!("{failure}").contains(remote.execution_host_id().as_str()));

    let local_plan = plan_for(&local, []);
    let local_binding = session_resume_binding_for(
        &local_plan,
        "thread-provider-existing",
        local.working_resource().clone(),
    );
    let remote_plan = plan_for(&remote, []);
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let failure = expect_failure(
        block_on(
            driver().resume_session(
                remote_plan,
                ResumeSessionRequest::new(
                    request_id("wrong-binding", &remote),
                    local_binding,
                    remote.working_resource().clone(),
                    None,
                )
                .with_access_policy(SessionAccessPolicy::read_only()),
                host_services_for(remote.execution_host_id().clone(), process),
            ),
        ),
        "a session cannot move to another configured instance",
    );
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.resume_binding_mismatch"
    );
    assert!(!state.started());

    let plan = plan_for(&local, []);
    let binding = session_resume_binding_for(
        &plan,
        "thread-provider-existing",
        local.working_resource().clone(),
    );
    let (process, state) = ScriptedAppServer::new(AppServerMode::SubstituteResume);
    let failure = expect_failure(
        block_on(
            driver().resume_session(
                plan,
                ResumeSessionRequest::new(
                    request_id("provider-substitution", &local),
                    binding,
                    local.working_resource().clone(),
                    None,
                )
                .with_access_policy(SessionAccessPolicy::read_only()),
                host_services_for(local.execution_host_id().clone(), process),
            ),
        ),
        "provider session substitution must fail",
    );
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.resume_provider_mismatch"
    );
    assert!(state.forced());
    assert!(state.waited());

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let local_services = host_services_for(local.execution_host_id().clone(), process);
    let mut session = block_on(driver().open_session(
        plan_for(&local, []),
        open_request("turn-host", &local),
        local_services,
    ))
    .expect("local session opens");
    let (remote_process, remote_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let failure = expect_failure(
        block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("wrong-host-turn").expect("turn id is valid"),
                OperationContent::new("do not send").expect("content is valid"),
            ),
            host_services_for(remote.execution_host_id().clone(), remote_process),
        )),
        "a turn cannot switch execution hosts",
    );
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!state.methods().contains(&"turn/start".to_owned()));
    assert!(!remote_state.started());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}
