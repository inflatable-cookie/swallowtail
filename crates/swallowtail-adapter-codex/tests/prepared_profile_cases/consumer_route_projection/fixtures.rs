use super::super::provider_session_import::{catalogue_candidate, catalogue_input, session_input};
use super::super::session_management::support::lifecycle_binding;
use super::super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_codex::{
    CodexSessionHistoryInput, CodexSessionManagementInput, CodexSessionReconciliationInput,
};
use swallowtail_idioms::IdiomScope;
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, IdiomSessionOption, ProviderSessionHistoryBounds,
    ProviderSessionHistoryId, ProviderSessionReconciliationBounds, SessionResumeBinding,
};

use super::ledger::*;
use super::naming::*;

pub(super) fn history_binding(
    prepared_app: &swallowtail_adapter_codex::CodexPreparedIntegration,
) -> SessionResumeBinding {
    let session = prepared_app
        .prepare_read_only_session(session_input("projection-history-session"))
        .expect("read-only session prepares");
    let plan = session.plan();
    SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("thread-provider-import").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        working_resource(),
        session.request().access_policy().clone(),
    )
}

pub(super) fn session_options() -> SessionOptions {
    SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("low").unwrap())
        .with_tools([tool("lookup")])
        .with_developer_instructions(OperationContent::new("private instructions").unwrap())
        .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).unwrap())
}

pub(super) fn bounded_session_profile(suffix: &str) -> CodexSessionProfileInput {
    CodexSessionProfileInput::new(
        RequestId::new(format!("projection-{suffix}")).unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default().with_reasoning_mode(ReasoningMode::new("low").unwrap()),
    )
    .with_user_input_exchange()
}

pub(super) fn session_profile(suffix: &str) -> CodexSessionProfileInput {
    CodexSessionProfileInput::new(
        RequestId::new(format!("projection-{suffix}")).unwrap(),
        model(),
        working_resource(),
        None,
        session_options(),
    )
    .with_user_input_exchange()
}

/// Collects the exact rows every prepared app-server facade emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        FIXTURE_VERSION,
        &recording,
        true,
    );

    let catalogue = prepared_app
        .prepare_catalogue(RequestId::new("projection-catalogue").unwrap(), None)
        .expect("model catalogue prepares");
    let read_only = prepared_app
        .prepare_read_only_session(session_profile("read-only"))
        .expect("read-only session prepares");
    let bounded = prepared_app
        .prepare_bounded_workspace_session(bounded_session_profile("bounded-workspace"))
        .expect("bounded workspace session prepares");
    let session_catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("projection"))
        .expect("thread catalogue prepares");
    let candidate = catalogue_candidate(&session_catalogue, &recording);
    let import = prepared_app
        .prepare_read_only_session_import(
            &session_catalogue,
            candidate,
            session_input("projection-import"),
        )
        .expect("thread import prepares");
    let binding = history_binding(&prepared_app);
    let history = prepared_app
        .prepare_session_history(CodexSessionHistoryInput::new(
            RequestId::new("projection-history").unwrap(),
            ProviderSessionHistoryId::new("codex-history-projection").unwrap(),
            model(),
            binding.clone(),
            ProviderSessionHistoryBounds::new(
                NonZeroU32::new(2).unwrap(),
                NonZeroU64::new(4096).unwrap(),
                NonZeroU32::new(64).unwrap(),
                NonZeroU32::new(8).unwrap(),
            ),
        ))
        .expect("history prepares");
    let reconciliation = prepared_app
        .prepare_session_reconciliation(CodexSessionReconciliationInput::new(
            RequestId::new("projection-reconcile").unwrap(),
            model(),
            binding,
            RuntimeTurnId::new("runtime-projection").unwrap(),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(8).unwrap(),
                NonZeroU64::new(4096).unwrap(),
            ),
        ))
        .expect("reconciliation prepares");
    let archive = prepared_app
        .prepare_archive_session(CodexSessionManagementInput::new(
            RequestId::new("projection-archive").unwrap(),
            lifecycle_binding(&prepared_app, FIXTURE_VERSION),
        ))
        .expect("archive prepares");
    let restore = prepared_app
        .prepare_restore_session(CodexSessionManagementInput::new(
            RequestId::new("projection-restore").unwrap(),
            lifecycle_binding(&prepared_app, FIXTURE_VERSION),
        ))
        .expect("restore prepares");
    let delete = prepared_app
        .prepare_delete_session(CodexSessionManagementInput::new(
            RequestId::new("projection-delete").unwrap(),
            lifecycle_binding(&prepared_app, FIXTURE_VERSION),
        ))
        .expect("delete prepares");

    let mut observed = BTreeMap::new();
    let mut record = |facade: &'static str, contribution: &ConsumerRouteProjectionContribution| {
        observed
            .entry(facade)
            .or_insert_with(BTreeSet::new)
            .extend(rows(contribution));
    };
    record(
        CATALOGUE,
        &catalogue
            .consumer_route_projection_contribution(source("codex.catalogue"))
            .expect("catalogue contributes"),
    );
    record(
        SESSION,
        &read_only
            .consumer_route_projection_contribution(source("codex.session.read-only"))
            .expect("read-only session contributes"),
    );
    record(
        SESSION,
        &bounded
            .consumer_route_projection_contribution(source("codex.session.bounded-workspace"))
            .expect("bounded workspace session contributes"),
    );
    record(
        SESSION_CATALOGUE,
        &session_catalogue
            .consumer_route_projection_contribution(source("codex.session-catalogue"))
            .expect("session catalogue contributes"),
    );
    record(
        SESSION_HISTORY,
        &history
            .consumer_route_projection_contribution(source("codex.session-history"))
            .expect("history contributes"),
    );
    record(
        SESSION_IMPORT,
        &import
            .consumer_route_projection_contribution(source("codex.session-import"))
            .expect("import contributes"),
    );
    record(
        SESSION_RECONCILIATION,
        &reconciliation
            .consumer_route_projection_contribution(source("codex.session-reconciliation"))
            .expect("reconciliation contributes"),
    );
    record(
        ARCHIVE,
        &archive
            .consumer_route_projection_contribution(source("codex.archive"))
            .expect("archive contributes"),
    );
    record(
        RESTORE,
        &restore
            .consumer_route_projection_contribution(source("codex.restore"))
            .expect("restore contributes"),
    );
    record(
        DELETE,
        &delete
            .consumer_route_projection_contribution(source("codex.delete"))
            .expect("delete contributes"),
    );
    observed
}
