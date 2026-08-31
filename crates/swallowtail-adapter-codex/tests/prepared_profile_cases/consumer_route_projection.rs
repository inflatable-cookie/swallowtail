use super::provider_session_import::{catalogue_candidate, catalogue_input, session_input};
use super::session_management::support::lifecycle_binding;
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_codex::{
    CodexSessionHistoryInput, CodexSessionManagementInput, CodexSessionReconciliationInput,
};
use swallowtail_idioms::IdiomScope;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, IdiomSessionOption,
    ProviderSessionHistoryBounds, ProviderSessionHistoryId, ProviderSessionReconciliationBounds,
    SessionResumeBinding,
};

const FIXTURE_VERSION: &str = "0.146.0";

const CATALOGUE: &str = "CodexPreparedCatalogue";
const SESSION: &str = "CodexPreparedSession";
const SESSION_CATALOGUE: &str = "CodexPreparedSessionCatalogue";
const SESSION_HISTORY: &str = "CodexPreparedSessionHistory";
const SESSION_IMPORT: &str = "CodexPreparedSessionImport";
const SESSION_RECONCILIATION: &str = "CodexPreparedSessionReconciliation";
const ARCHIVE: &str = "CodexPreparedArchive";
const RESTORE: &str = "CodexPreparedRestore";
const DELETE: &str = "CodexPreparedDelete";

const CODEX_FACADES: [&str; 9] = [
    CATALOGUE,
    SESSION,
    SESSION_CATALOGUE,
    SESSION_HISTORY,
    SESSION_IMPORT,
    SESSION_RECONCILIATION,
    ARCHIVE,
    RESTORE,
    DELETE,
];

const MATRIX_ONLY: &str =
    "matrix or route-wide posture only; no exact app-server prepared authority";
const EXEC_ONLY: &str = "proved only by the codex.exec prepared route, not codex.app-server";

/// One exact `codex.app-server` census row and its adapter disposition.
struct LedgerEntry {
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 36 `codex.app-server` census rows.
///
/// The ledger claims nothing about the remaining 716 census rows.
const CODEX_FIRST_TRANCHE: [LedgerEntry; 36] = [
    LedgerEntry {
        semantic_id: "feature.model-catalogue",
        emitted_by: &[CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.structured-run",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.interactive-session",
        emitted_by: &[SESSION, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.streaming-events",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: MATRIX_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.structured-output",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.attachments",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.consumer-tool-exchange",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.question-exchange",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: MATRIX_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.load-session",
        emitted_by: &[SESSION, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.resume-session",
        emitted_by: &[SESSION, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-catalogue",
        emitted_by: &[SESSION_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-import",
        emitted_by: &[SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.working-resource",
        emitted_by: &[
            SESSION,
            SESSION_CATALOGUE,
            SESSION_HISTORY,
            SESSION_IMPORT,
            SESSION_RECONCILIATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "no prepared app-server plan requires the bounded workspace text-write capability",
    },
    LedgerEntry {
        semantic_id: "feature.external-search",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-archive",
        emitted_by: &[ARCHIVE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-restore",
        emitted_by: &[RESTORE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-delete",
        emitted_by: &[DELETE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[SESSION_HISTORY, SESSION_IMPORT, SESSION_RECONCILIATION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.prepared-facade",
        emitted_by: &CODEX_FACADES,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.activity-observation",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.model-selection",
        emitted_by: &[
            SESSION,
            SESSION_HISTORY,
            SESSION_IMPORT,
            SESSION_RECONCILIATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.reasoning-selection",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-options",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.tool-declarations",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.developer-instructions",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.idioms",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.user-input-exchange",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.load-session",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.resume-session",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-catalogue-bounds",
        emitted_by: &[SESSION_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-history-bounds",
        emitted_by: &[SESSION_HISTORY],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-reconciliation",
        emitted_by: &[SESSION_RECONCILIATION],
        withheld_because: "",
    },
];

/// Rows the prepared app-server route proves that the 36-row census tranche
/// does not name.
///
/// These are recorded so the ledger stays exact. They are not tranche coverage
/// and claim nothing about the remaining 716 census rows.
const PROVED_BEYOND_THE_CENSUS_TRANCHE: [&str; 2] = [
    "feature.provider-session-history",
    "feature.provider-session-reconciliation",
];

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

fn semantic_id(identity: &ConsumerRouteRowIdentity) -> &'static str {
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::RealtimeMediaSession => "feature.realtime-media-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::StructuredOutput => "feature.structured-output",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::ConsumerToolExchange => "feature.consumer-tool-exchange",
            ConsumerRouteFeatureId::QuestionExchange => "feature.question-exchange",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::LoadSession => "feature.load-session",
            ConsumerRouteFeatureId::ResumeSession => "feature.resume-session",
            ConsumerRouteFeatureId::ProviderSessionCatalogue => {
                "feature.provider-session-catalogue"
            }
            ConsumerRouteFeatureId::ProviderSessionImport => "feature.provider-session-import",
            ConsumerRouteFeatureId::ProviderSessionArchive => "feature.provider-session-archive",
            ConsumerRouteFeatureId::ProviderSessionRestore => "feature.provider-session-restore",
            ConsumerRouteFeatureId::ProviderSessionDelete => "feature.provider-session-delete",
            ConsumerRouteFeatureId::ProviderSessionReconciliation => {
                "feature.provider-session-reconciliation"
            }
            ConsumerRouteFeatureId::ProviderSessionHistory => "feature.provider-session-history",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            ConsumerRouteFeatureId::BoundedWorkspaceTextWrite => {
                "feature.bounded-workspace-text-write"
            }
            ConsumerRouteFeatureId::ExternalSearch => "feature.external-search",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement => {
                "feature.active-session-reasoning-ack"
            }
            ConsumerRouteFeatureId::Namespaced(_) => "feature.namespaced-extension",
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::SessionOptions => "control.session-options",
            ConsumerRouteControlId::ToolDeclarations => "control.tool-declarations",
            ConsumerRouteControlId::DeveloperInstructions => "control.developer-instructions",
            ConsumerRouteControlId::Idioms => "control.idioms",
            ConsumerRouteControlId::UserInputExchange => "control.user-input-exchange",
            ConsumerRouteControlId::LoadSession => "control.load-session",
            ConsumerRouteControlId::ResumeSession => "control.resume-session",
            ConsumerRouteControlId::SessionCatalogueBounds => "control.session-catalogue-bounds",
            ConsumerRouteControlId::SessionHistoryBounds => "control.session-history-bounds",
            ConsumerRouteControlId::SessionReconciliation => "control.session-reconciliation",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            ConsumerRouteControlId::RealtimeMediaConfig => "control.realtime-media-config",
            ConsumerRouteControlId::PlannedConnectionRollover => {
                "control.planned-connection-rollover"
            }
            ConsumerRouteControlId::Namespaced(_) => "control.namespaced-extension",
        },
    }
}

fn rows(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<&'static str> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| semantic_id(row.identity()))
        .collect()
}

fn history_binding(
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

fn session_options() -> SessionOptions {
    SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("low").unwrap())
        .with_tools([tool("lookup")])
        .with_developer_instructions(OperationContent::new("private instructions").unwrap())
        .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).unwrap())
}

fn bounded_session_profile(suffix: &str) -> CodexSessionProfileInput {
    CodexSessionProfileInput::new(
        RequestId::new(format!("projection-{suffix}")).unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default().with_reasoning_mode(ReasoningMode::new("low").unwrap()),
    )
    .with_user_input_exchange()
}

fn session_profile(suffix: &str) -> CodexSessionProfileInput {
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
fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
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

#[test]
fn the_coverage_ledger_dispositions_exactly_the_thirty_six_app_server_rows() {
    let mut ids = BTreeSet::new();
    for entry in &CODEX_FIRST_TRANCHE {
        assert!(
            ids.insert(entry.semantic_id),
            "the ledger repeats {}",
            entry.semantic_id
        );
        assert!(
            entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."),
            "{} is not a census row identity",
            entry.semantic_id
        );
        if entry.emitted_by.is_empty() {
            assert!(
                !entry.withheld_because.is_empty(),
                "{} is withheld without a reason",
                entry.semantic_id
            );
        } else {
            assert!(
                entry.withheld_because.is_empty(),
                "{} is emitted and withheld at once",
                entry.semantic_id
            );
        }
    }
    assert_eq!(CODEX_FIRST_TRANCHE.len(), 36);
    assert_eq!(ids.len(), 36);
}

#[test]
fn every_prepared_facade_emits_exactly_its_ledger_rows() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), CODEX_FACADES.len());
    for facade in CODEX_FACADES {
        let expected = CODEX_FIRST_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.contains(&facade))
            .map(|entry| entry.semantic_id)
            .collect::<BTreeSet<_>>();
        let published = observed.get(facade).expect("every facade contributes");
        let in_tranche = published
            .iter()
            .copied()
            .filter(|id| !PROVED_BEYOND_THE_CENSUS_TRANCHE.contains(id))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            in_tranche, expected,
            "{facade} emitted rows differ from the coverage ledger"
        );
    }
}

#[test]
fn withheld_rows_are_emitted_by_no_prepared_facade() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>();
    let ledger = CODEX_FIRST_TRANCHE
        .iter()
        .map(|entry| entry.semantic_id)
        .collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published) || PROVED_BEYOND_THE_CENSUS_TRANCHE.contains(published),
            "{published} is published without a recorded disposition"
        );
    }
    for entry in &CODEX_FIRST_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !emitted.contains(entry.semantic_id),
                "{} is withheld but was published",
                entry.semantic_id
            );
        } else {
            assert!(
                emitted.contains(entry.semantic_id),
                "{} is claimed but was never published",
                entry.semantic_id
            );
        }
    }
}

#[test]
fn the_per_turn_exchange_stays_per_turn_and_claims_no_provider_mutation() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        FIXTURE_VERSION,
        &recording,
        true,
    );
    let session = prepared_app
        .prepare_read_only_session(session_profile("per-turn"))
        .expect("read-only session prepares");
    let contribution = session
        .consumer_route_projection_contribution(source("codex.session.per-turn"))
        .expect("session contributes");
    let exchange = contribution
        .session_start_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::UserInputExchange)
        })
        .expect("the per-turn exchange is published");
    assert_eq!(exchange.lifecycle(), ConsumerRouteLifecycle::PerTurn);
    assert!(!exchange.state_support().prepared());
    assert!(!exchange.state_support().provider_effective());
    assert!(!exchange.state_support().rejected());
    assert!(!exchange.mutation_authority().is_acknowledged());

    let observation = contribution
        .active_session_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
        })
        .expect("activity observation stays post-open");
    assert_eq!(
        observation.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        observation.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert!(observation.state_support().observed());
    assert!(!observation.state_support().provider_effective());
}

#[test]
fn every_published_row_carries_exact_runtime_or_prepared_authority() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        FIXTURE_VERSION,
        &recording,
        true,
    );
    let session = prepared_app
        .prepare_read_only_session(session_profile("authority"))
        .expect("read-only session prepares");
    let contribution = session
        .consumer_route_projection_contribution(source("codex.session.authority"))
        .expect("session contributes");
    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
    {
        assert!(
            matches!(
                row.source_class(),
                ConsumerRouteSourceClass::PreparedOperationRecord
                    | ConsumerRouteSourceClass::CapabilityProfile
                    | ConsumerRouteSourceClass::AdapterPreparedInput
            ),
            "{:?} does not carry exact runtime or prepared authority",
            row.identity()
        );
        assert!(row.safe_reason().is_none());
    }
    let rendered = format!("{contribution:?}");
    for forbidden in ["codex-app-server-executable", "private instructions"] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, command, or content data"
        );
    }
}
