use super::*;
use crate::support::app_server::ThreadCatalogueMode;
use std::num::NonZeroU32;
use swallowtail_adapter_codex::CodexSessionCatalogueInput;
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionBindingOrigin, ProviderSessionCatalogueBounds,
    ProviderSessionImportAvailability, ProviderSessionImportUnavailableReason,
};
use swallowtail_runtime::ProviderSessionCatalogueId;
use swallowtail_testkit::RecordedHostCall;

mod acceptance;

const PRIVATE_TITLE: &str = "Imported thread";
const PRIVATE_PREVIEW: &str = "Bounded provider preview";

#[test]
fn exact_versions_advertise_and_prepare_thread_catalogue_only_inside_the_corpus() {
    for (version, expected) in [
        ("0.104.0", false),
        ("0.105.0", true),
        ("0.107.0", true),
        ("0.110.0", true),
        ("0.146.0", true),
        ("0.147.0", false),
    ] {
        let recording = RecordingHostServices::default();
        let prepared_app = prepared(CodexPreparedDriver::AppServer, version, &recording, true);
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionCatalogue),
            expected,
            "catalogue advertisement for {version}"
        );
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionImport),
            expected,
            "import advertisement for {version}"
        );
        let result = prepared_app.prepare_session_catalogue(catalogue_input(version));
        assert_eq!(
            result.is_ok(),
            expected,
            "catalogue preparation for {version}"
        );
    }
}

#[test]
fn prepared_catalogue_is_resource_scoped_bounded_paginated_and_redacted() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("resource"))
        .expect("thread catalogue prepares");

    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let first = block_on(catalogue.list_sessions(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("first page projects");
    let candidates = first.candidates().collect::<Vec<_>>();
    assert_eq!(candidates.len(), 2);
    assert_eq!(
        candidates[0].candidate_id().as_str(),
        "codex-thread-candidate-0"
    );
    assert_eq!(candidates[0].display().title(), Some(PRIVATE_TITLE));
    assert_eq!(candidates[0].display().preview(), Some(PRIVATE_PREVIEW));
    assert_eq!(
        candidates[0].updated_at_unix_milliseconds(),
        Some(1_775_000_000_000)
    );
    assert_eq!(
        candidates[0].activity(),
        ProviderSessionActivityState::Inactive
    );
    assert_eq!(
        candidates[0].import_availability(),
        ProviderSessionImportAvailability::Available
    );
    assert_eq!(
        candidates[1].activity(),
        ProviderSessionActivityState::Active
    );
    assert_eq!(
        candidates[1].import_availability(),
        ProviderSessionImportAvailability::Unavailable(
            ProviderSessionImportUnavailableReason::Active
        )
    );
    let cursor = first
        .next_cursor()
        .expect("first page has a cursor")
        .clone();
    assert_eq!(cursor.observed_candidates(), 2);
    let list = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/list")
        .expect("thread/list request is captured");
    assert_eq!(list["params"]["limit"], 2);
    assert_eq!(list["params"]["archived"], false);
    assert_eq!(
        list["params"]["sourceKinds"],
        serde_json::json!(["cli", "vscode", "appServer"])
    );
    assert_eq!(list["params"]["cwd"], "/private/recording/workspace");
    assert!(state.waited());

    let next = catalogue
        .next_page_request(RequestId::new("catalogue-page-2").unwrap(), cursor)
        .expect("second page request prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let second = block_on(catalogue.list_page(
        next,
        host_services_with(process, &recording, [HostServiceKind::WorkingResource]),
    ))
    .expect("second page projects");
    let second_candidates = second.candidates().collect::<Vec<_>>();
    assert_eq!(second_candidates.len(), 1);
    assert_eq!(
        second_candidates[0].candidate_id().as_str(),
        "codex-thread-candidate-2"
    );
    assert!(second.next_cursor().is_none());
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 2);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 2);

    let debug = format!("{first:?}{second:?}");
    assert!(!debug.contains(PRIVATE_TITLE));
    assert!(!debug.contains(PRIVATE_PREVIEW));
    assert!(!debug.contains("private-thread-page-2"));
    assert!(!debug.contains("thread-provider-import"));
}

#[test]
fn resource_mismatch_fails_closed_without_projecting_candidates() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("wrong-resource"))
        .expect("thread catalogue prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::WrongResource,
    ));
    let failure = block_on(catalogue.list_sessions(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect_err("another cwd is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.thread_catalogue.resource_mismatch"
    );
    assert!(!format!("{failure:?}").contains("/private/another/workspace"));
    assert!(state.waited());
}

#[test]
fn import_revalidates_exact_thread_and_binding_load_resume_stay_unchanged() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("import"))
        .expect("thread catalogue prepares");
    let candidate = catalogue_candidate(&catalogue, &recording);
    let import = prepared_app
        .prepare_read_only_session_import(&catalogue, candidate, session_input("import-request"))
        .expect("thread import prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let outcome = block_on(import.import_session(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("thread import succeeds");
    assert_eq!(
        outcome.binding().origin(),
        ProviderSessionBindingOrigin::ExplicitlyImported
    );
    assert_eq!(
        outcome.binding().provider_session_ref().as_provider_value(),
        "thread-provider-import"
    );
    let read = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/read")
        .expect("thread/read request is captured");
    assert_eq!(read["params"]["threadId"], "thread-provider-import");
    assert_eq!(read["params"]["includeTurns"], true);
    assert!(state.waited());

    let session = prepared_app
        .prepare_read_only_session(session_input("continuation-profile"))
        .expect("existing read-only continuation profile prepares");
    let (process, load_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let loaded = block_on(
        session
            .load_session(
                RequestId::new("imported-load").unwrap(),
                outcome.binding().clone(),
                support::host_services(process),
            )
            .expect("imported load request prepares"),
    )
    .expect("imported session loads through the existing path");
    assert_eq!(loaded.replay().count(), 2);
    let (_, handle) = loaded.into_parts();
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(load_state.methods().contains(&"thread/resume".to_owned()));
    assert!(load_state.waited());

    let (process, resume_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let handle = block_on(
        session
            .resume_session(
                RequestId::new("imported-resume").unwrap(),
                outcome.binding().clone(),
                support::host_services(process),
            )
            .expect("imported resume request prepares"),
    )
    .expect("imported session resumes through the existing path");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(resume_state.methods().contains(&"thread/resume".to_owned()));
    assert!(resume_state.waited());
}

#[test]
fn stale_missing_active_and_mismatched_revalidation_issue_no_binding() {
    for (mode, expected_code) in [
        (
            ThreadCatalogueMode::Changed,
            "swallowtail.codex.thread_import.candidate_changed",
        ),
        (
            ThreadCatalogueMode::Missing,
            "swallowtail.codex.app_server.request_failed",
        ),
        (
            ThreadCatalogueMode::Active,
            "swallowtail.provider_session_import.revalidation_mismatch",
        ),
        (
            ThreadCatalogueMode::Mismatched,
            "swallowtail.codex.thread_import.candidate_changed",
        ),
        (
            ThreadCatalogueMode::WrongResource,
            "swallowtail.codex.thread_catalogue.resource_mismatch",
        ),
    ] {
        let recording = RecordingHostServices::default();
        let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
        let catalogue = prepared_app
            .prepare_session_catalogue(catalogue_input("revalidate"))
            .expect("thread catalogue prepares");
        let candidate = catalogue_candidate(&catalogue, &recording);
        let import = prepared_app
            .prepare_read_only_session_import(
                &catalogue,
                candidate,
                session_input("revalidation-request"),
            )
            .expect("thread import prepares");
        let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(mode));
        let failure = block_on(import.import_session(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect_err("drifted thread cannot issue a binding");
        assert_eq!(failure.diagnostic().code(), expected_code);
        assert!(state.waited());
    }
}

pub(super) fn catalogue_candidate(
    catalogue: &swallowtail_adapter_codex::CodexPreparedSessionCatalogue,
    recording: &RecordingHostServices,
) -> swallowtail_runtime::ProviderSessionCandidate {
    let (process, _) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    block_on(catalogue.list_sessions(host_services_with(
        process,
        recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("thread catalogue executes")
    .candidates()
    .next()
    .expect("available candidate exists")
    .clone()
}

pub(super) fn catalogue_input(suffix: &str) -> CodexSessionCatalogueInput {
    CodexSessionCatalogueInput::new(
        RequestId::new(format!("catalogue-{suffix}")).unwrap(),
        ProviderSessionCatalogueId::new(format!("codex-catalogue-{suffix}")).unwrap(),
        working_resource(),
        ProviderSessionCatalogueBounds::new(
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(4).unwrap(),
            NonZeroU32::new(64).unwrap(),
            NonZeroU32::new(128).unwrap(),
            NonZeroU32::new(128).unwrap(),
        )
        .unwrap(),
    )
}

pub(super) fn session_input(suffix: &str) -> CodexSessionProfileInput {
    CodexSessionProfileInput::new(
        RequestId::new(suffix).unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default(),
    )
}
