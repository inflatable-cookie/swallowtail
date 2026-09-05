//! Contract 061 disposition proof for the exact nineteen Alibaba rows.

#![allow(dead_code)]

mod support;

use std::collections::BTreeSet;
use std::num::{NonZeroU32, NonZeroU64};
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    ALIBABA_DEPLOYABLE_MODELS_ACCESS_PROFILE_ID, ALIBABA_DEPLOYABLE_MODELS_ENDPOINT,
    ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE, AlibabaConversationProfileInput,
    AlibabaDeployableModelsPreparationInput, AlibabaDeployableModelsProfileInput,
    AlibabaRetainedConversationProfileInput, AlibabaRunProfileInput, AlibabaSessionHistoryInput,
    AlibabaSessionManagementInput, EXACT_MODEL_ID, MODEL_ROUTE_ID,
    alibaba_model_studio_access_profile, alibaba_model_studio_descriptor,
    prepare_alibaba_deployable_models, prepare_alibaba_model_studio,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, CredentialMechanism, CredentialState,
    EntitlementMetering, EntitlementState, InstanceRevision, InstanceTargetRef, ModelId,
    ModelRouteId, ModelRouteRevision, ProviderSessionBindingOrigin, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SessionRef, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow, ConsumerRouteRowIdentity,
    CredentialRef, PreparedAccessEvidence, ProviderSessionHistoryBounds,
    ProviderSessionManagementBinding, RequestId, SessionResumeBinding,
};
use swallowtail_testkit::RecordingHostServices;

const ROUTE: &str = "alibaba.conversations";
const CATALOGUE: &str = "AlibabaPreparedDeployableModels";
const CONVERSATION: &str = "AlibabaModelStudioPreparedConversation";
const RUN: &str = "AlibabaModelStudioPreparedRun";
const RETAINED: &str = "AlibabaModelStudioPreparedRetainedConversation";
const HISTORY: &str = "AlibabaModelStudioPreparedSessionHistory";
const DELETE: &str = "AlibabaModelStudioPreparedDelete";
const PROFILES: [&str; 6] = [CATALOGUE, CONVERSATION, RUN, RETAINED, HISTORY, DELETE];

const NO_STRUCTURED_PROVIDER_STATE: &str =
    "matrix descriptor only; structured Alibaba runs retain no provider session";
const NO_RESUME: &str =
    "matrix descriptor only; Alibaba exposes load authority without resume authority";

struct LedgerEntry {
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const ALIBABA_TRANCHE: [LedgerEntry; 19] = [
    entry("model-catalogue", "feature.model-catalogue", &[CATALOGUE]),
    entry("structured-run", "feature.structured-run", &[RUN]),
    entry(
        "interactive-session",
        "feature.interactive-session",
        &[CONVERSATION, RETAINED],
    ),
    entry(
        "route-observation",
        "feature.streaming-events",
        &[CONVERSATION, RUN, RETAINED],
    ),
    entry(
        "route-observation",
        "feature.usage-evidence",
        &[CONVERSATION, RUN, RETAINED],
    ),
    entry(
        "route-capability",
        "feature.cancellation-or-interruption",
        &[RUN],
    ),
    entry("session-lifecycle", "feature.load-session", &[RETAINED]),
    entry(
        "session-lifecycle",
        "feature.provider-session-delete",
        &[DELETE],
    ),
    entry(
        "route-capability",
        "feature.owned-remote-resource-cleanup",
        &[CONVERSATION],
    ),
    entry(
        "session-lifecycle",
        "feature.persistent-session-posture",
        &[CONVERSATION, RETAINED],
    ),
    entry(
        "route-capability",
        "feature.prepared-facade",
        &[CATALOGUE, CONVERSATION, RUN, RETAINED, HISTORY, DELETE],
    ),
    entry(
        "route-observation",
        "feature.activity-observation",
        &[CONVERSATION, RUN, RETAINED],
    ),
    entry("structured-run", "control.model-selection", &[RUN]),
    entry(
        "interactive-session",
        "control.model-selection",
        &[CONVERSATION, RETAINED],
    ),
    entry("structured-run", "control.fixed-wire-turn-options", &[RUN]),
    withheld(
        "structured-run",
        "control.provider-state-policy",
        NO_STRUCTURED_PROVIDER_STATE,
    ),
    entry(
        "interactive-session",
        "control.provider-state-policy",
        &[RETAINED],
    ),
    entry("session-management", "control.load-session", &[RETAINED]),
    withheld("session-management", "control.resume-session", NO_RESUME),
];

const fn entry(
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
) -> LedgerEntry {
    LedgerEntry {
        operation_shape,
        semantic_id,
        emitted_by,
        withheld_because: "",
    }
}

const fn withheld(
    operation_shape: &'static str,
    semantic_id: &'static str,
    withheld_because: &'static str,
) -> LedgerEntry {
    LedgerEntry {
        operation_shape,
        semantic_id,
        emitted_by: &[],
        withheld_because,
    }
}

#[test]
fn coverage_ledger_is_exactly_nineteen_rows_with_two_named_withheld_rows() {
    let identities = ALIBABA_TRANCHE
        .iter()
        .map(|entry| (entry.operation_shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(ALIBABA_TRANCHE.len(), 19);
    assert_eq!(identities.len(), 19);
    assert_eq!(
        ALIBABA_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        17
    );
    assert_eq!(
        ALIBABA_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        2
    );
    assert!(ALIBABA_TRANCHE.iter().any(|entry| {
        entry.operation_shape == "structured-run"
            && entry.semantic_id == "control.provider-state-policy"
            && entry.withheld_because == NO_STRUCTURED_PROVIDER_STATE
    }));
    assert!(ALIBABA_TRANCHE.iter().any(|entry| {
        entry.operation_shape == "session-management"
            && entry.semantic_id == "control.resume-session"
            && entry.withheld_because == NO_RESUME
    }));
}

#[test]
fn every_prepared_alibaba_profile_matches_its_ledger_disposition() {
    let observed = [
        (CATALOGUE, catalogue()),
        (CONVERSATION, conversation()),
        (RUN, run()),
        (RETAINED, retained()),
        (HISTORY, history()),
        (DELETE, delete()),
    ]
    .into_iter()
    .map(|(profile, contribution)| {
        assert_eq!(contribution.sources().count(), 1);
        (
            profile,
            contribution
                .selection_rows()
                .chain(contribution.session_start_rows())
                .chain(contribution.active_session_rows())
                .map(|row| row_identity(row, &contribution))
                .collect::<BTreeSet<_>>(),
        )
    })
    .collect::<Vec<_>>();

    assert_eq!(observed.len(), PROFILES.len());
    for (profile, identities) in observed {
        let expected = ALIBABA_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.contains(&profile))
            .map(|entry| (entry.operation_shape, entry.semantic_id))
            .collect::<BTreeSet<_>>();
        assert_eq!(identities, expected, "{profile} disposition differs");
    }
}

#[test]
fn withheld_rows_and_cross_shape_assembly_are_rejected() {
    let run = run();
    let conversation = conversation();
    let run_rows = all_rows(&run).collect::<Vec<_>>();
    let conversation_rows = all_rows(&conversation).collect::<Vec<_>>();
    assert!(
        run_rows
            .iter()
            .any(|row| { semantic_id(row.identity()) == "control.fixed-wire-turn-options" })
    );
    assert!(
        !run_rows
            .iter()
            .any(|row| { semantic_id(row.identity()) == "control.provider-state-policy" })
    );
    assert!(
        !conversation_rows
            .iter()
            .any(|row| { semantic_id(row.identity()) == "control.resume-session" })
    );

    let rejection = ConsumerRouteProjectionContribution::new(
        run.applicability().clone(),
        run.sources().cloned().collect::<Vec<_>>(),
        [(**conversation_rows.first().expect("conversation publishes")).clone()],
        [],
        [],
    )
    .expect_err("interactive rows cannot join a structured run");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

fn row_identity(
    row: &ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
) -> (&'static str, &'static str) {
    (
        census_shape(
            row.identity(),
            contribution.applicability().operation_shape(),
        ),
        semantic_id(row.identity()),
    )
}

fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

fn semantic_id(identity: &ConsumerRouteRowIdentity) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        assert_eq!(extension.route(), ROUTE);
        return match extension.semantic_id() {
            "feature.owned-remote-resource-cleanup" => "feature.owned-remote-resource-cleanup",
            "control.fixed-wire-turn-options" => "control.fixed-wire-turn-options",
            "control.provider-state-policy" => "control.provider-state-policy",
            other => panic!("unexpected Alibaba descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::LoadSession => "feature.load-session",
            ConsumerRouteFeatureId::ProviderSessionDelete => "feature.provider-session-delete",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected Alibaba feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::LoadSession => "control.load-session",
            other => panic!("unexpected Alibaba control {other:?}"),
        },
    }
}

fn census_shape(
    identity: &ConsumerRouteRowIdentity,
    operation_shape: swallowtail_core::OperationShape,
) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "feature.owned-remote-resource-cleanup" => "route-capability",
            "control.fixed-wire-turn-options" => "structured-run",
            "control.provider-state-policy" => match operation_shape {
                swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                swallowtail_core::OperationShape::StructuredRun => "structured-run",
                other => panic!("unexpected Alibaba provider-state shape {other:?}"),
            },
            other => panic!("unexpected Alibaba descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents | ConsumerRouteFeatureId::UsageEvidence => {
                "route-observation"
            }
            ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            ConsumerRouteFeatureId::PersistentSessionPosture => "session-lifecycle",
            ConsumerRouteFeatureId::LoadSession | ConsumerRouteFeatureId::ProviderSessionDelete => {
                "session-lifecycle"
            }
            ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            other => panic!("unexpected Alibaba feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => match operation_shape {
                swallowtail_core::OperationShape::StructuredRun => "structured-run",
                swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                other => panic!("unexpected Alibaba model shape {other:?}"),
            },
            ConsumerRouteControlId::LoadSession => "session-management",
            other => panic!("unexpected Alibaba control {other:?}"),
        },
    }
}

fn catalogue() -> ConsumerRouteProjectionContribution {
    let host = RecordingHostServices::default();
    let access = AccessProfile::new(
        AccessProfileId::new(ALIBABA_DEPLOYABLE_MODELS_ACCESS_PROFILE_ID).expect("access id"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        swallowtail_core::EndpointAudience::new(ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE)
            .expect("audience"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(CredentialRef::new("alibaba.catalogue.key").expect("credential"));
    let prepared = prepare_alibaba_deployable_models(
        AlibabaDeployableModelsPreparationInput::new(
            InstanceRevision::new("fixture-1").expect("revision"),
            host.services().execution_host_id().clone(),
            InstanceTargetRef::new(ALIBABA_DEPLOYABLE_MODELS_ENDPOINT).expect("target"),
            access.clone(),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access.id().clone(),
                CredentialState::Ready,
                EntitlementState::Available,
                swallowtail_core::EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        host.services(),
    )
    .expect("catalogue prepares")
    .prepare_catalogue(AlibabaDeployableModelsProfileInput::new(
        RequestId::new("alibaba.projection.catalogue").expect("request id"),
    ))
    .expect("catalogue operation prepares");
    prepared
        .consumer_route_projection_contribution(source("alibaba.projection.catalogue"))
        .expect("catalogue contributes")
}

fn conversation() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares")
        .prepare_conversation(AlibabaConversationProfileInput::new(
            RequestId::new("alibaba.projection.conversation").expect("request id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("2026-07-22").expect("revision"),
            ModelId::new(EXACT_MODEL_ID).expect("model"),
            SessionProviderStatePolicy::DurableConversationDeleteOnClose,
        ))
        .expect("conversation prepares");
    prepared
        .consumer_route_projection_contribution(source("alibaba.projection.conversation"))
        .expect("conversation contributes")
}

fn run() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares")
        .prepare_run(AlibabaRunProfileInput::new(
            RequestId::new("alibaba.projection.run").expect("request id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("2026-07-22").expect("revision"),
            ModelId::new(EXACT_MODEL_ID).expect("model"),
            swallowtail_runtime::OperationContent::new("projection run").expect("content"),
        ))
        .expect("run prepares");
    prepared
        .consumer_route_projection_contribution(source("alibaba.projection.run"))
        .expect("run contributes")
}

fn retained() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares")
        .prepare_retained_conversation(AlibabaRetainedConversationProfileInput::new(
            RequestId::new("alibaba.projection.retained").expect("request id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("2026-08-05").expect("revision"),
            ModelId::new(EXACT_MODEL_ID).expect("model"),
        ))
        .expect("retained prepares");
    prepared
        .consumer_route_projection_contribution(source("alibaba.projection.retained"))
        .expect("retained contributes")
}

fn history() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let plan = fixture.retained_plan();
    let binding = SessionResumeBinding::resource_free(
        SessionRef::new("alibaba.projection.session").expect("session ref"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route").clone(),
        plan.model_id().expect("model").clone(),
        SessionAccessPolicy::resource_free(),
    );
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares")
        .prepare_session_history(AlibabaSessionHistoryInput::new(
            RequestId::new("alibaba.projection.history").expect("request id"),
            swallowtail_runtime::ProviderSessionHistoryId::new("alibaba.projection.history")
                .expect("history id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("2026-07-22").expect("revision"),
            ModelId::new(EXACT_MODEL_ID).expect("model"),
            binding,
            ProviderSessionHistoryBounds::new(
                NonZeroU32::new(2).expect("items"),
                NonZeroU64::new(64 * 1024).expect("bytes"),
                NonZeroU32::new(64).expect("cursor"),
                NonZeroU32::new(8).expect("snapshot"),
            ),
        ))
        .expect("history prepares");
    prepared
        .consumer_route_projection_contribution(source("alibaba.projection.history"))
        .expect("history contributes")
}

fn delete() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let services = fixture.services();
    let access = alibaba_model_studio_access_profile();
    let evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        swallowtail_core::EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ));
    let integration = prepare_alibaba_model_studio(fixture.preparation_input(), &services)
        .expect("integration prepares");
    let instance = integration.instance().clone();
    let binding = ProviderSessionManagementBinding::from_bound_session(
        SessionRef::new("alibaba.projection.delete").expect("session ref"),
        &alibaba_model_studio_descriptor(),
        &instance,
        evidence,
        None,
        ProviderSessionBindingOrigin::Created,
    )
    .expect("management binding");
    let prepared = integration
        .prepare_delete_retained_conversation(AlibabaSessionManagementInput::new(
            RequestId::new("alibaba.projection.delete").expect("request id"),
            binding,
        ))
        .expect("delete prepares");
    prepared
        .consumer_route_projection_contribution(source("alibaba.projection.delete"))
        .expect("delete contributes")
}

fn source(id: &str) -> swallowtail_runtime::ConsumerRouteProjectionSourceId {
    swallowtail_runtime::ConsumerRouteProjectionSourceId::new(id).expect("source id")
}
