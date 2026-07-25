use super::fixtures::{joined_cleanup, prepared, profile_input};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_core::{
    ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation, ResourceAccess,
    SessionAccessPolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, RequestId, RuntimeTurnId, SessionOptions, TerminalStatus,
    TurnRequest,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_new_prompt_write_and_interruption_preserve_explicit_ambient_authority() {
    for host_id in [
        ExecutionHostId::new("fixture.prepared.local").unwrap(),
        ExecutionHostId::new("fixture.prepared.remote").unwrap(),
    ] {
        let host = FixtureHost::new(Scenario::Complete);
        let prepared = prepared(&host, host_id.clone(), "0.28.1");
        let profile = prepared
            .prepare_session(profile_input("new", SessionOptions::default()))
            .expect("Kimi session prepares");

        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile
                .plan()
                .requirements()
                .harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
        );
        assert_eq!(
            profile.request().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile
                .evidence()
                .observation()
                .version()
                .version()
                .as_str(),
            "0.28.1"
        );

        let services = host.services(host_id);
        let mut session =
            block_on(profile.open_session(services.clone())).expect("prepared session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("prepared-prompt").unwrap(),
                OperationContent::new("private prepared prompt").unwrap(),
            ),
            services,
        ))
        .expect("prepared prompt starts");
        let outcome = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            host.resource_writes(),
            [(
                "src/generated.rs".to_owned(),
                "pub fn generated() {}\n".to_owned()
            )]
        );
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }

    let host_id = ExecutionHostId::new("fixture.prepared.interrupt").unwrap();
    let host = FixtureHost::new(Scenario::HoldPrompt);
    let prepared = prepared(&host, host_id.clone(), "0.28.1");
    let profile = prepared
        .prepare_session(profile_input("interrupt", SessionOptions::default()))
        .expect("Kimi session prepares");
    let services = host.services(host_id);
    let mut session =
        block_on(profile.open_session(services.clone())).expect("prepared session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("prepared-interrupt").unwrap(),
            OperationContent::new("interrupt this prompt").unwrap(),
        ),
        services,
    ))
    .expect("prepared prompt starts");
    block_on(turn.cancellation().request()).expect("turn interruption is relayed");
    let outcome = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.cleanup_events(), joined_cleanup());
}

#[test]
fn prepared_load_replays_history_while_resume_remains_replay_free() {
    for host_id in [
        ExecutionHostId::new("fixture.prepared.load.local").unwrap(),
        ExecutionHostId::new("fixture.prepared.load.remote").unwrap(),
    ] {
        let opening_host = FixtureHost::new(Scenario::Complete);
        let prepared = prepared(&opening_host, host_id.clone(), "0.28.1");
        let profile = prepared
            .prepare_session(profile_input("binding-source", SessionOptions::default()))
            .expect("Kimi session prepares");
        let session = block_on(profile.open_session(opening_host.services(host_id.clone())))
            .expect("source session opens");
        let binding = session
            .resume_binding()
            .expect("source session supplies a binding")
            .clone();
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

        let load_host = FixtureHost::new(Scenario::Complete);
        let loaded = block_on(
            profile
                .load_session(
                    RequestId::new("prepared-load").unwrap(),
                    binding.clone(),
                    load_host.services(host_id.clone()),
                )
                .expect("load request derives from the plan"),
        )
        .expect("session loads");
        let (replay, loaded_session) = loaded.into_parts();
        assert_eq!(replay.len(), 2);
        assert_eq!(
            replay[1].content().expect("agent replay").as_str(),
            "Previous answer."
        );
        assert_eq!(block_on(loaded_session.close()), CleanupOutcome::Clean);
        assert_eq!(load_host.cleanup_events(), joined_cleanup());

        let resume_host = FixtureHost::new(Scenario::Complete);
        let resumed = block_on(
            profile
                .resume_session(
                    RequestId::new("prepared-resume").unwrap(),
                    binding,
                    resume_host.services(host_id),
                )
                .expect("resume request derives from the plan"),
        )
        .expect("session resumes without replay");
        assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
        assert_eq!(resume_host.cleanup_events(), joined_cleanup());
        assert!(
            resume_host
                .wire_methods()
                .contains(&"session/resume".to_owned())
        );
        assert!(
            !resume_host
                .wire_methods()
                .contains(&"session/load".to_owned())
        );
    }
}
