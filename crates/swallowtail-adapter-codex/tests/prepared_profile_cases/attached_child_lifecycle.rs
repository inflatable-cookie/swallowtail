use super::session_management::support::lifecycle_binding;
use super::*;
use crate::support::app_server::ThreadCatalogueMode;
use std::num::NonZeroU32;
use swallowtail_adapter_codex::{CodexSessionCatalogueInput, CodexSessionManagementInput};
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionCatalogueBounds, ProviderSessionEffectTruth,
};
use swallowtail_runtime::{CancellationControl, ProviderSessionCatalogueId};

const TRACE: &str = include_str!("../fixtures/codex-attached-child-lifecycle/trace.json");

fn trace() -> serde_json::Value {
    serde_json::from_str(TRACE).expect("attached-child trace is valid JSON")
}

fn catalogue_input(suffix: &str) -> CodexSessionCatalogueInput {
    CodexSessionCatalogueInput::new(
        RequestId::new(format!("attached-catalogue-{suffix}")).unwrap(),
        ProviderSessionCatalogueId::new(format!("codex-attached-catalogue-{suffix}")).unwrap(),
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

fn delete_outcome(
    version: &str,
    request_id: &str,
    unverified_newer: bool,
    mode: AppServerMode,
) -> (
    swallowtail_runtime::ProviderSessionManagementOutcome,
    std::sync::Arc<crate::support::app_server::AppServerState>,
) {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        version,
        &RecordingHostServices::default(),
        false,
    );
    let mut input = CodexSessionManagementInput::new(
        RequestId::new(request_id.to_owned()).unwrap(),
        lifecycle_binding(&prepared_app, version),
    );
    if unverified_newer {
        input = input.allow_unverified_newer();
    }
    let operation = prepared_app
        .prepare_delete_session(input)
        .expect("delete prepares");
    let (process, state) = ScriptedAppServer::new(mode);
    let outcome = block_on(operation.execute(crate::support::host_services(process)))
        .expect("delete attempt resolves");
    (outcome, state)
}

#[test]
fn attached_child_delete_refusal_is_failed_before_effect() {
    let (outcome, state) = delete_outcome(
        "0.164.0",
        "attached-child-refusal",
        false,
        AppServerMode::AttachedChildDeleteRefusal,
    );
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
    assert_eq!(outcome.effect().affected_scope(), None);
    assert_eq!(
        outcome
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("swallowtail.codex.app_server.request_failed")
    );
    assert_eq!(
        state
            .methods()
            .iter()
            .filter(|method| *method == "thread/delete")
            .count(),
        1
    );
    assert!(
        !state.messages().iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/deleted")
        }),
        "the refusal path sends no deletion notification"
    );
    assert!(state.waited());
}

#[test]
fn unverified_newer_attached_child_refusal_keeps_before_effect_truth() {
    let (outcome, state) = delete_outcome(
        "0.165.0",
        "attached-child-refusal-newer",
        true,
        AppServerMode::AttachedChildDeleteRefusal,
    );
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
    assert!(
        !state.messages().iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/deleted")
        }),
        "an unverified-newer refusal never reports provider deletion"
    );
    assert!(state.waited());
}

#[test]
fn delete_after_dispatch_cancellation_joins_and_stays_unconfirmed() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.164.0",
        &RecordingHostServices::default(),
        false,
    );
    let operation = prepared_app
        .prepare_delete_session(CodexSessionManagementInput::new(
            RequestId::new("attached-child-cancel").unwrap(),
            lifecycle_binding(&prepared_app, "0.164.0"),
        ))
        .expect("delete prepares");
    let cancellation = Arc::clone(operation.request().cancellation());
    let (process, state) = ScriptedAppServer::new(AppServerMode::LifecycleHold);
    let execution = std::thread::spawn({
        let future = operation.execute(crate::support::host_services(process));
        move || block_on(future)
    });
    while !state.methods().contains(&"thread/delete".to_owned()) {
        std::thread::yield_now();
    }
    block_on(cancellation.request()).expect("cancellation wakes the delete");
    let outcome = execution
        .join()
        .expect("delete execution thread joins")
        .expect("cancelled delete resolves");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
    assert!(state.forced());
    assert!(state.waited());
}

#[test]
fn ordinary_catalogue_hides_attached_child_without_widening_scope() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.164.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("attached-child"))
        .expect("thread catalogue prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::AttachedChildHidden,
    ));
    let page = block_on(catalogue.list_sessions(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("first page projects");
    // The provider's ordinary listing omits attached children, so the adapter
    // cannot project a candidate for them and does not invent one.
    assert_eq!(page.candidates().len(), 1);
    assert!(page.next_cursor().is_none());
    assert_eq!(
        page.candidates()
            .next()
            .expect("visible candidate exists")
            .activity(),
        ProviderSessionActivityState::Inactive
    );

    let request = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/list")
        })
        .expect("thread/list was sent");
    let params = request["params"]
        .as_object()
        .expect("thread/list parameters are an object");
    for widened in [
        "attachedToThreadId",
        "relationFilter",
        "parentThreadId",
        "ancestorThreadId",
    ] {
        assert!(
            !params.contains_key(widened),
            "the catalogue never widens its discovery scope with {widened}"
        );
    }
    assert_eq!(params["archived"], serde_json::Value::Bool(false));
}

#[test]
fn load_requires_provider_history_and_resume_excludes_turns() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.164.0", &recording, false);
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("attached-resume").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("session prepares");
    let binding = support::session_resume_binding(profile.plan(), "thread-provider-import");

    let (process, state) = ScriptedAppServer::new(AppServerMode::LoadWithoutTurns);
    let failure = match block_on(
        profile
            .load_session(
                RequestId::new("attached-load").unwrap(),
                binding.clone(),
                crate::support::host_services(process),
            )
            .expect("bound load prepares"),
    ) {
        Ok(_) => panic!("a load without provider history fails visibly"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.replay_malformed"
    );
    assert!(state.waited());

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = crate::support::host_services(process);
    let handle = block_on(
        profile
            .resume_session(
                RequestId::new("attached-resume-bound").unwrap(),
                binding,
                services.clone(),
            )
            .expect("bound resume prepares"),
    )
    .expect("resume opens");
    let resume = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/resume")
        })
        .expect("thread/resume was sent");
    let params = resume["params"]
        .as_object()
        .expect("thread/resume parameters are an object");
    assert_eq!(
        params["excludeTurns"],
        serde_json::Value::Bool(true),
        "resume suppresses provider turn replay"
    );
    for unmapped in [
        "history",
        "path",
        "initialTurnsPage",
        "attachedToThreadId",
        "attachToSource",
    ] {
        assert!(
            !params.contains_key(unmapped),
            "resume never sends the unmapped {unmapped} field"
        );
    }
    assert_eq!(
        block_on(support::close_session(handle, services)),
        CleanupOutcome::Clean
    );
    assert!(state.waited());
}

#[test]
fn attached_child_trace_is_frozen_and_mutation_sensitive() {
    let trace = trace();
    assert_eq!(trace["record"], "Research 467");
    assert_eq!(trace["downloads"]["downloadedBinariesExecuted"], false);
    assert_eq!(
        trace["frozenIdentity"]["0.165.0"]["commit"],
        "360319d8dee62d87e2d59fc0c21805d79519cfed"
    );
    let hashes = trace["contentSha256"]
        .as_object()
        .expect("content hashes are an object");
    let hash = |path: &str, version: &str| {
        hashes
            .get(path)
            .and_then(|entry| entry.get(version))
            .unwrap_or_else(|| panic!("{path} at {version} is frozen"))
            .clone()
    };
    assert_eq!(
        hash(
            "codex-rs/thread-store/src/local/delete_thread.rs",
            "0.165.0"
        ),
        serde_json::json!("f4e6406f1703d2ca29ae400e7ebefdfe3ac3c688386a8bb558a1e0b8c01afeda")
    );
    assert_eq!(
        hash(
            "codex-rs/thread-store/src/local/model_context.rs",
            "0.165.0"
        ),
        hash(
            "codex-rs/thread-store/src/local/model_context.rs",
            "0.164.0"
        )
    );
    assert_eq!(
        hash(
            "codex-rs/thread-store/src/local/history_revision.rs",
            "0.165.0"
        ),
        hash(
            "codex-rs/thread-store/src/local/history_revision.rs",
            "0.164.0"
        )
    );
    assert_ne!(
        hash("codex-rs/state/src/runtime/threads.rs", "0.165.0"),
        hash("codex-rs/state/src/runtime/threads.rs", "0.164.0")
    );
    assert_eq!(
        hashes["codex-rs/state/migrations/0062_threads_attached_parent.sql"]["0.164.0"],
        serde_json::Value::Null
    );

    let guard = &trace["attachedChildGuard"];
    let order = guard["writerRecheck"]["singleDeleteOrder0_165_0"]
        .as_array()
        .expect("delete order is frozen");
    let position = |symbol: &str| {
        order
            .iter()
            .position(|entry| entry.as_str() == Some(symbol))
            .unwrap_or_else(|| panic!("{symbol} is in the frozen delete order"))
    };
    assert!(
        position("acquire_writer_locks") < position("check_attached_children"),
        "the attached-child recheck runs under writer ownership"
    );
    assert!(
        position("check_attached_children") < position("thread_data_cleanup"),
        "the refusal precedes every deletion effect"
    );
    let precheck = guard["precheckOrder0_165_0"]
        .as_array()
        .expect("precheck order is frozen");
    let precheck_position = |symbol: &str| {
        precheck
            .iter()
            .position(|entry| entry.as_str() == Some(symbol))
            .unwrap_or_else(|| panic!("{symbol} is in the frozen precheck order"))
    };
    assert!(
        precheck_position("local_store_ensure_no_attached_threads")
            < precheck_position("prepare_thread_for_delete_per_candidate")
    );
    assert_eq!(
        guard["referenceIndex"]["archivedChildBlocksDelete"],
        serde_json::Value::Bool(true)
    );
    assert_eq!(
        guard["listFilter"]["ordinaryListingHidesAttachedThreads"],
        serde_json::Value::Bool(true)
    );
    assert_eq!(
        guard["listFilter"]["adapterSendsRelationFilter"],
        serde_json::Value::Bool(false)
    );
    assert_eq!(
        guard["adapterCannotCreateAttachedChildrenThereforeSafe"],
        serde_json::Value::Bool(false)
    );

    let resume = &trace["paginatedResume"];
    assert_eq!(
        resume["selectedBySwallowtail"],
        serde_json::Value::Bool(true)
    );
    let conditions = resume["reuseConditions"]
        .as_array()
        .expect("reuse conditions are frozen");
    assert_eq!(conditions.len(), 5);
    assert_eq!(resume["noNewClaim"], serde_json::Value::Bool(true));

    let fallback = &resume["staleFallback"];
    assert_eq!(fallback["evidenceClass"], "source-level-only");
    assert_eq!(fallback["runtimeClaim"], serde_json::Value::Bool(false));
    assert_eq!(
        fallback["triggers"]
            .as_array()
            .expect("fallback triggers are frozen")
            .len(),
        6
    );
    assert_eq!(
        fallback["action"],
        "call load_latest_model_context and rebuild ResumedHistory"
    );
    assert_eq!(
        fallback["concurrentHistory"]["rechecks"]
            .as_array()
            .expect("concurrent-history rechecks are frozen")
            .len(),
        2
    );
    assert!(
        fallback["proofLimit"]
            .as_str()
            .is_some_and(|text| text.contains("not the provider's internal reuse decision"))
    );
}

#[test]
fn attached_child_guard_scope_is_the_local_store_only() {
    let trace = trace();
    let guard = &trace["attachedChildGuard"];
    assert!(
        guard["guardAppliesTo"]
            .as_str()
            .is_some_and(|text| text.contains("LocalThreadStore only"))
    );
    let selection = &guard["storeSelection"];
    assert_eq!(selection["symbol"], "thread_store_from_config");
    assert_eq!(selection["default"], "Local");
    assert_eq!(selection["tomlKey"], "experimental_thread_store");
    assert_eq!(
        selection["adapterSelectsStore"],
        serde_json::Value::Bool(false)
    );
    assert_eq!(
        selection["attachedChildCreationRequiresLocalStore"],
        serde_json::Value::Bool(true)
    );
    assert!(
        selection["attachedChildCreationGuard"]
            .as_str()
            .is_some_and(|text| text.contains("unsupported_thread_store_operation"))
    );
    assert!(
        selection["inMemoryAttachedListing"]
            .as_str()
            .is_some_and(|text| text.contains("Unsupported"))
    );
    assert!(
        selection["residual"]
            .as_str()
            .is_some_and(|text| text.contains("unclaimed"))
    );
    for path in [
        "codex-rs/core/src/thread_manager.rs",
        "codex-rs/core/src/config/mod.rs",
        "codex-rs/config/src/config_toml.rs",
        "codex-rs/thread-store/src/in_memory.rs",
        "codex-rs/app-server/src/message_processor.rs",
    ] {
        assert_ne!(
            trace["contentSha256"][path]["0.164.0"], trace["contentSha256"][path]["0.165.0"],
            "{path} changes across the frozen hop"
        );
    }
}
