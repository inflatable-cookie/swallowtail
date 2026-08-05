use super::fixture::{attached_input, id, prepare, probe, session_profile, turn};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::FixtureHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_kimi::{
    KimiLocalServerOwnedInput, KimiLocalServerPermissionMode, KimiLocalServerReconciliationInput,
    KimiLocalServerSessionManagementInput, KimiModelSelection, start_kimi_local_server_owned,
};
use swallowtail_core::{
    ExecutionHostId, InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision,
    ObservableActivityAvailability, ProviderSessionBindingOrigin, ProviderSessionEffectTruth,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationDetachmentAcknowledgement, ProviderSessionReconciliationBounds,
    RequestId, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionRestorationOutcome, TerminalStatus, WorkingStateRestorationMethod,
};

#[test]
fn later_releases_ignore_unsolicited_global_events_from_other_sessions() {
    for version in ["0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1"] {
        let server =
            InteractiveFixtureServer::start_with_version(InteractiveScenario::GlobalNoise, version);
        let host = FixtureHost::for_endpoint(server.endpoint());
        let execution_host = id(
            ExecutionHostId::new,
            &format!("fixture.kimi.global-noise.{version}"),
        );
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare(execution_host, services.clone(), version);
        let profile = session_profile(
            &prepared,
            KimiLocalServerPermissionMode::Auto,
            "global-noise",
        );
        let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
        let mut turn =
            block_on(session.start_turn(turn("global-noise-turn"), services)).expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome exists"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.output().expect("output exists").as_str(),
            "fixture result"
        );
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn attached_prepared_session_streams_and_preserves_exact_bindings() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.interactive");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.0");
    let profile = session_profile(&prepared, KimiLocalServerPermissionMode::Manual, "complete");
    assert_eq!(
        profile.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
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
    assert!(
        events
            .iter()
            .filter_map(|event| event.as_ref().ok())
            .any(|event| {
                event.reconciliation_checkpoint().is_some_and(|checkpoint| {
                    checkpoint.provider_session_ref().as_provider_value() == "interactive-session"
                        && checkpoint.provider_turn_ref().as_provider_value() == "7"
                        && checkpoint.runtime_turn_id().as_str() == "turn-complete"
                })
            })
    );
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
fn persisted_checkpoint_reconciles_the_exact_completed_turn_after_restart() {
    let first_server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let first_host = FixtureHost::for_endpoint(first_server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.reconciliation");
    let first_services = first_host.services(execution_host.clone(), false);
    let first_prepared = prepare(execution_host.clone(), first_services.clone(), "0.29.0");
    let first_profile = session_profile(
        &first_prepared,
        KimiLocalServerPermissionMode::Auto,
        "reconciliation-source",
    );
    let mut session =
        block_on(first_profile.open_session(first_services.clone())).expect("source session opens");
    let binding = session
        .resume_binding()
        .expect("source binding exists")
        .clone();
    let mut turn =
        block_on(session.start_turn(turn("reconciliation-runtime-turn"), first_services))
            .expect("source turn starts");
    let events = block_on(
        turn.take_events()
            .expect("source event stream exists")
            .collect::<Vec<_>>(),
    );
    let checkpoint = events
        .iter()
        .filter_map(|event| event.as_ref().ok())
        .filter_map(|event| event.reconciliation_checkpoint())
        .find(|checkpoint| !checkpoint.cursor().is_empty())
        .expect("source checkpoint exists")
        .clone();
    let persisted = checkpoint
        .export_persisted(first_profile.plan(), &binding)
        .expect("checkpoint persists under source plan");
    assert_eq!(
        block_on(
            turn.take_terminal_outcome()
                .expect("source terminal exists")
        )
        .status(),
        &TerminalStatus::Completed
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    drop(first_server);

    let second_server = InteractiveFixtureServer::start(InteractiveScenario::ReconcileComplete);
    let second_host = FixtureHost::for_endpoint(second_server.endpoint());
    let second_services = second_host.services(execution_host.clone(), false);
    let second_prepared = prepare(execution_host, second_services.clone(), "0.29.0");
    let wrong_host_id = id(ExecutionHostId::new, "fixture.kimi.reconciliation.foreign");
    let wrong_services = second_host.services(wrong_host_id.clone(), false);
    let wrong_prepared = prepare(wrong_host_id, wrong_services, "0.29.0");
    let mismatch = wrong_prepared
        .prepare_session_reconciliation(KimiLocalServerReconciliationInput::new(
            id(RequestId::new, "foreign-reconciliation-request"),
            KimiModelSelection::new(
                id(ModelRouteId::new, "fixture.kimi.route"),
                id(ModelRouteRevision::new, "1"),
                id(ModelId::new, "kimi-k2.5"),
            ),
            binding.clone(),
            persisted.clone(),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(16).expect("bound is non-zero"),
                NonZeroU64::new(4096).expect("bound is non-zero"),
            ),
        ))
        .expect_err("cross-host checkpoint rejects");
    assert_eq!(
        mismatch.diagnostic().safe().code(),
        "swallowtail.provider_operation_checkpoint.attachment_mismatch"
    );
    let input = KimiLocalServerReconciliationInput::new(
        id(RequestId::new, "reconciliation-request"),
        KimiModelSelection::new(
            id(ModelRouteId::new, "fixture.kimi.route"),
            id(ModelRouteRevision::new, "1"),
            id(ModelId::new, "kimi-k2.5"),
        ),
        binding,
        persisted,
        ProviderSessionReconciliationBounds::new(
            NonZeroU32::new(16).expect("bound is non-zero"),
            NonZeroU64::new(4096).expect("bound is non-zero"),
        ),
    );
    let legacy = second_prepared
        .prepare_working_state_restoration(input.clone())
        .expect("read-only restoration still prepares");
    assert_eq!(
        legacy.method(),
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    );
    let session = session_profile(
        &second_prepared,
        KimiLocalServerPermissionMode::Auto,
        "reconciliation-attachment",
    );
    let restoration = second_prepared
        .prepare_session_reconciliation(input)
        .expect("reconciliation prepares")
        .prepare_settled_session_restoration(session, id(RequestId::new, "reconciliation-resume"))
        .expect("settled restoration prepares");
    assert_eq!(
        restoration.attachment_kind(),
        SettledSessionAttachmentKind::Resume
    );
    let restored = block_on(restoration.restore(second_services)).unwrap_or_else(|error| {
        panic!(
            "reconciliation executes: {error:?}; requests={:?}",
            second_server.requests()
        )
    });
    let SettledSessionRestorationOutcome::Attached(attached) = restored else {
        panic!("completed Kimi turn must resume its session");
    };
    let (outcome, attachment) = attached.into_parts();

    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ExactProviderTurn
    );
    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Completed
    );
    assert_eq!(
        outcome
            .provider_turn_ref()
            .expect("exact provider turn remains bound")
            .as_provider_value(),
        "7"
    );
    let SettledSessionAttachment::Resumed(resumed) = attachment else {
        panic!("Kimi local-server settled attachment is replay-free resume");
    };
    assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
    assert!(
        second_server
            .requests()
            .iter()
            .any(|request| { request.contains("GET /api/v1/sessions/interactive-session") })
    );
    assert!(second_server.requests().iter().all(|request| {
        !request.contains("/prompts")
            && !request.contains(":archive")
            && !request.contains("/approvals/")
            && !request.contains("/questions/")
    }));
}

#[test]
fn attached_turn_detaches_without_abort_and_reconciles_as_exact_active_work() {
    let first_server = InteractiveFixtureServer::start(InteractiveScenario::Detach);
    let first_host = FixtureHost::for_endpoint(first_server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.detachment");
    let first_services = first_host.services(execution_host.clone(), false);
    let first_prepared = prepare(execution_host.clone(), first_services.clone(), "0.29.0");
    let first_profile = first_prepared
        .prepare_session(super::fixture::session_input(
            "detachment-source",
            swallowtail_adapter_kimi::KimiLocalServerSessionConfiguration::new(
                KimiLocalServerPermissionMode::Auto,
            )
            .with_active_turn_detachment(),
        ))
        .expect("detachment profile prepares");
    let mut session = block_on(first_profile.open_session(first_services.clone()))
        .expect("detachment session opens");
    let binding = session
        .resume_binding()
        .expect("detachment binding exists")
        .clone();
    let mut turn = block_on(session.start_turn(turn("detachment-runtime-turn"), first_services))
        .expect("detachment turn starts");
    let mut events = turn.take_events().expect("detachment event stream exists");
    let checkpoint = block_on(async {
        loop {
            let event = events
                .next()
                .await
                .expect("event stream remains open")
                .expect("event is valid");
            if let Some(checkpoint) = event.reconciliation_checkpoint() {
                break checkpoint.clone();
            }
        }
    });
    let persisted = checkpoint
        .export_persisted(first_profile.plan(), &binding)
        .expect("detachment checkpoint persists");
    let detachment = turn.detachment().expect("detachment control exists");
    assert_eq!(
        block_on(detachment.request()).expect("detachment requests"),
        OperationDetachmentAcknowledgement::Requested
    );
    assert_eq!(
        block_on(detachment.request()).expect("detachment is idempotent"),
        OperationDetachmentAcknowledgement::AlreadyRequested
    );
    assert_eq!(
        block_on(
            turn.take_terminal_outcome()
                .expect("detachment terminal exists")
        )
        .status(),
        &TerminalStatus::Detached
    );
    drop(events);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let first_requests = first_server.requests();
    assert!(first_requests.contains(&"WS observer closed".to_owned()));
    assert!(!first_requests.contains(&"WS unexpected control text".to_owned()));
    drop(first_server);

    let second_server = InteractiveFixtureServer::start(InteractiveScenario::ReconcileActive);
    let second_host = FixtureHost::for_endpoint(second_server.endpoint());
    let second_services = second_host.services(execution_host.clone(), false);
    let second_prepared = prepare(execution_host, second_services.clone(), "0.29.0");
    let reconciliation = second_prepared
        .prepare_session_reconciliation(KimiLocalServerReconciliationInput::new(
            id(RequestId::new, "detachment-reconciliation"),
            KimiModelSelection::new(
                id(ModelRouteId::new, "fixture.kimi.route"),
                id(ModelRouteRevision::new, "1"),
                id(ModelId::new, "kimi-k2.5"),
            ),
            binding,
            persisted,
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(16).expect("bound is non-zero"),
                NonZeroU64::new(4096).expect("bound is non-zero"),
            ),
        ))
        .expect("detached reconciliation prepares");
    let outcome =
        block_on(reconciliation.execute(second_services)).expect("detached turn reconciles");
    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Active
    );
    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ExactProviderTurn
    );
    assert!(
        second_server
            .requests()
            .iter()
            .all(|request| { !request.contains("/prompts") && !request.contains("abort") })
    );
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
    let detachment_error = owned
        .prepared()
        .prepare_session(super::fixture::session_input(
            "owned-detachment",
            swallowtail_adapter_kimi::KimiLocalServerSessionConfiguration::new(
                KimiLocalServerPermissionMode::Auto,
            )
            .with_active_turn_detachment(),
        ))
        .expect_err("owned topology rejects detachment");
    assert_eq!(
        detachment_error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.detachment_unsupported"
    );
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
