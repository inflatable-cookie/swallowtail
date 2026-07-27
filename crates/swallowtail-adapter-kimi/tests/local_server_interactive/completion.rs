use super::fixture::{attached_input, id, prepare, probe, session_profile, turn};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::FixtureHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_kimi::{
    KimiLocalServerOwnedInput, KimiLocalServerPermissionMode,
    KimiLocalServerSessionManagementInput, start_kimi_local_server_owned,
};
use swallowtail_core::{
    ExecutionHostId, InstanceTargetRef, ProviderSessionBindingOrigin, ProviderSessionEffectTruth,
};
use swallowtail_runtime::{CleanupOutcome, RequestId, TerminalStatus};

#[test]
fn attached_prepared_session_streams_and_preserves_exact_bindings() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.interactive");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.0");
    let profile = session_profile(&prepared, KimiLocalServerPermissionMode::Manual, "complete");
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");

    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider session is bound")
            .as_provider_value(),
        "interactive-session"
    );
    assert_eq!(
        session
            .management_binding()
            .expect("management binding is returned")
            .origin(),
        ProviderSessionBindingOrigin::Created
    );
    let resume = session
        .resume_binding()
        .expect("resume binding is returned")
        .clone();
    let mut turn = block_on(session.start_turn(turn("turn-complete"), services.clone()))
        .unwrap_or_else(|error| panic!("turn starts: {error:?}; requests={:?}", server.requests()));
    let events = block_on(
        turn.take_events()
            .expect("event stream exists")
            .collect::<Vec<_>>(),
    );
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "fixture result"
    );
    assert!(events.iter().all(Result::is_ok));
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let resumed = block_on(
        profile
            .resume_session(id(RequestId::new, "resume-request"), resume, services)
            .expect("resume prepares"),
    )
    .expect("session resumes");
    assert_eq!(
        resumed
            .management_binding()
            .expect("resumed management binding exists")
            .origin(),
        ProviderSessionBindingOrigin::Resumed
    );
    let management = resumed
        .management_binding()
        .expect("management binding remains available")
        .clone();
    assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
    let archive = prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            id(RequestId::new, "archive-after-close"),
            management,
        ))
        .expect("archive prepares after the attachment closes");
    let archived = block_on(
        archive.execute(host.services(id(ExecutionHostId::new, "fixture.kimi.interactive"), false)),
    )
    .expect("archive executes");
    assert_eq!(
        archived.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert_eq!(host.credential_releases(), 4);
    let requests = server.requests();
    assert!(requests.iter().any(|request| request.starts_with("WS ")));
    assert!(requests.iter().any(|request| {
        request.contains("POST /api/v1/sessions/interactive-session/prompts")
            && request.contains(r#""permission_mode":"manual""#)
    }));
    let resume_index = requests
        .iter()
        .position(|request| request.contains("GET /api/v1/sessions/interactive-session"))
        .expect("resume lookup was observed");
    let archive_index = requests
        .iter()
        .position(|request| request.contains(":archive"))
        .expect("explicit archive was observed");
    assert!(resume_index < archive_index);
}

#[test]
fn owned_session_joins_transport_and_then_its_foreground_child() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.owned-interactive");
    let services = host.services(execution_host.clone(), true);
    let owned = block_on(start_kimi_local_server_owned(
        KimiLocalServerOwnedInput::new(
            attached_input(execution_host, "0.29.0"),
            id(InstanceTargetRef::new, "fixture.kimi.executable"),
        ),
        probe(),
        services.clone(),
    ))
    .expect("owned local server starts");
    let profile = session_profile(
        owned.prepared(),
        KimiLocalServerPermissionMode::Auto,
        "owned-session",
    );
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let mut turn = block_on(session.start_turn(turn("owned-turn"), services)).expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.credential_releases(), 2);
    assert!(!host.process_stopped_and_joined());
    assert_eq!(block_on(owned.close()), CleanupOutcome::Clean);
    assert!(host.process_stopped_and_joined());
}
