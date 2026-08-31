use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjection, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionInput, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES, compose_consumer_route_projection,
};

use crate::{ConsumerRouteProjectionFixture, consumer_route_projection_source};

pub(super) const RECORD_SOURCE: &str = "fixture.source.configured-instance";
pub(super) const EVIDENCE_SOURCE: &str = "fixture.source.prepared-operation";
pub(super) const ADAPTER_SOURCE: &str = "fixture.source.adapter-contribution";
pub(super) const OBSERVATION_SOURCE: &str = "fixture.source.active-session";

pub(super) fn record_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        RECORD_SOURCE,
        ConsumerRouteProjectionSourceKind::ConfiguredInstance,
    )
}

pub(super) fn evidence_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        EVIDENCE_SOURCE,
        ConsumerRouteProjectionSourceKind::PreparedOperation,
    )
}

pub(super) fn adapter_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        ADAPTER_SOURCE,
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    )
}

pub(super) fn observation_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        OBSERVATION_SOURCE,
        ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
    )
}

pub(super) fn feature_row(
    applicability: &ConsumerRouteApplicability,
    feature: ConsumerRouteFeatureId,
    lifecycle: ConsumerRouteLifecycle,
) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Feature(feature),
        applicability.clone(),
        adapter_source(),
        ConsumerRouteSourceClass::PreparedOperationRecord,
        ConsumerRouteEvidenceStrength::PreparedOperation,
        lifecycle,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    .with_availability(ConsumerRouteAvailability::Available)
    .with_actor_posture(ConsumerRouteActorPosture::Informational)
}

pub(super) fn control_row(
    applicability: &ConsumerRouteApplicability,
    control: ConsumerRouteControlId,
    lifecycle: ConsumerRouteLifecycle,
) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Control(control),
        applicability.clone(),
        adapter_source(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        ConsumerRouteEvidenceStrength::RouteValidation,
        lifecycle,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    .with_availability(ConsumerRouteAvailability::Available)
    .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
    .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
        ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id is valid"),
    ))
    .with_state_support(
        ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_prepared(),
    )
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(
            ConsumerRouteEnumeratedValues::new([
                ConsumerRouteEnumerableValue::new("low").expect("value is valid"),
                ConsumerRouteEnumerableValue::new("high").expect("value is valid"),
            ])
            .expect("bounded domain is admitted"),
        ),
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
    ))
}

pub(super) fn selection_features() -> Vec<ConsumerRouteFeatureId> {
    vec![
        ConsumerRouteFeatureId::ModelCatalogue,
        ConsumerRouteFeatureId::StructuredRun,
        ConsumerRouteFeatureId::InteractiveSession,
        ConsumerRouteFeatureId::RealtimeMediaSession,
        ConsumerRouteFeatureId::StreamingEvents,
        ConsumerRouteFeatureId::UsageEvidence,
        ConsumerRouteFeatureId::ActivityObservation,
        ConsumerRouteFeatureId::ReasoningSelection,
        ConsumerRouteFeatureId::StructuredOutput,
        ConsumerRouteFeatureId::Attachments,
        ConsumerRouteFeatureId::ConsumerToolExchange,
        ConsumerRouteFeatureId::QuestionExchange,
        ConsumerRouteFeatureId::CancellationOrInterruption,
        ConsumerRouteFeatureId::LoadSession,
        ConsumerRouteFeatureId::ResumeSession,
        ConsumerRouteFeatureId::ProviderSessionCatalogue,
        ConsumerRouteFeatureId::ProviderSessionImport,
        ConsumerRouteFeatureId::ProviderSessionArchive,
        ConsumerRouteFeatureId::ProviderSessionRestore,
        ConsumerRouteFeatureId::ProviderSessionDelete,
        ConsumerRouteFeatureId::ProviderSessionReconciliation,
        ConsumerRouteFeatureId::ProviderSessionHistory,
        ConsumerRouteFeatureId::PersistentSessionPosture,
        ConsumerRouteFeatureId::WorkingResource,
        ConsumerRouteFeatureId::BoundedWorkspaceTextWrite,
        ConsumerRouteFeatureId::ExternalSearch,
        ConsumerRouteFeatureId::OutputTokenLimit,
        ConsumerRouteFeatureId::PreparedFacade,
        ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
    ]
}

pub(super) fn namespaced_feature(index: usize) -> ConsumerRouteFeatureId {
    ConsumerRouteFeatureId::Namespaced(
        ConsumerRouteNamespacedExtension::new(
            "fixture.route",
            "fixture-version-1",
            format!("fixture.extension.{index}"),
        )
        .expect("bounded extension is admitted"),
    )
}

pub(super) fn rows(
    applicability: &ConsumerRouteApplicability,
    count: usize,
    lifecycle: ConsumerRouteLifecycle,
) -> Vec<ConsumerRouteProjectionRow> {
    let features = selection_features();
    (0..count)
        .map(|index| {
            let feature = features
                .get(index)
                .cloned()
                .unwrap_or_else(|| namespaced_feature(index));
            feature_row(applicability, feature, lifecycle)
        })
        .collect()
}

pub(super) fn contribution(
    applicability: &ConsumerRouteApplicability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active: Vec<ConsumerRouteProjectionRow>,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    ConsumerRouteProjectionContribution::new(
        applicability.clone(),
        [adapter_source(), observation_source()],
        selection,
        session_start,
        active,
    )
}

pub(super) fn compose(
    fixture: &ConsumerRouteProjectionFixture,
    contributions: &[&ConsumerRouteProjectionContribution],
) -> Result<ConsumerRouteProjection, ConsumerRouteProjectionFailure> {
    let record = fixture.record();
    let evidence = fixture.prepared();
    compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(&record, record_source(), &evidence, evidence_source())
            .with_contributions(contributions.iter().copied()),
    )
}

pub(super) fn assert_kind(
    failure: &ConsumerRouteProjectionFailure,
    kind: ConsumerRouteProjectionFailureKind,
) {
    assert_eq!(failure.kind(), kind, "{:?}", failure.diagnostic());
    assert!(!failure.diagnostic().message().is_empty());
    assert!(failure.diagnostic().message().len() <= MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES);
}

/// Builds one consumer-mediated per-turn control row.
///
/// The row is selectable per turn and never claims prepared session-start
/// state or a provider acknowledgement.
pub(super) fn per_turn_control_row(
    applicability: &ConsumerRouteApplicability,
    control: ConsumerRouteControlId,
) -> ConsumerRouteProjectionRow {
    control_row(applicability, control, ConsumerRouteLifecycle::PerTurn)
        .with_mutation_authority(ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn(
            ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id is valid"),
        ))
        .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_requested())
}

/// Composes one projection from the exact record and evidence supplied.
pub(super) fn compose_across(
    record_fixture: &ConsumerRouteProjectionFixture,
    evidence_fixture: &ConsumerRouteProjectionFixture,
) -> Result<ConsumerRouteProjection, ConsumerRouteProjectionFailure> {
    let record = record_fixture.record();
    let evidence = evidence_fixture.prepared();
    compose_consumer_route_projection(ConsumerRouteProjectionInput::new(
        &record,
        record_source(),
        &evidence,
        evidence_source(),
    ))
}
