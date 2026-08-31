use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteAvailabilityDimension, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteNamespacedExtension, ConsumerRouteOmissionSemantics, ConsumerRouteProjection,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionInput, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceId, ConsumerRouteProjectionSourceIdentity,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSafeReason,
    ConsumerRouteSourceClass, ConsumerRouteStateSupport, ConsumerRouteSupportPosture,
    ConsumerRouteValueDomain, ConsumerRouteValueKind, MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS,
    MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES, MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES,
    MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES, MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS,
    MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES, MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS,
    MAX_CONSUMER_ROUTE_SESSION_START_ROWS, MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES,
    MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES, compose_consumer_route_projection,
};

use crate::{
    CONSUMER_ROUTE_PRIVATE_CREDENTIAL, CONSUMER_ROUTE_PRIVATE_TARGET,
    ConsumerRouteProjectionFixture, consumer_route_projection_source,
};

const RECORD_SOURCE: &str = "fixture.source.configured-instance";
const EVIDENCE_SOURCE: &str = "fixture.source.prepared-operation";
const ADAPTER_SOURCE: &str = "fixture.source.adapter-contribution";
const OBSERVATION_SOURCE: &str = "fixture.source.active-session";

/// Runs the complete portable Contract 061 projection conformance suite.
///
/// Every assertion uses runtime and testkit types only. No adapter, provider,
/// transport, or live evidence takes part.
pub fn assert_consumer_route_projection_contract() {
    assert_fixed_maxima();
    assert_failure_kinds();
    assert_named_counterexamples();
    assert_view_and_lifecycle_separation();
    assert_identical_row_source_replacement();
    assert_unknown_and_absent_truth_survives();
    assert_no_raw_or_presentation_data();
}

fn record_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        RECORD_SOURCE,
        ConsumerRouteProjectionSourceKind::ConfiguredInstance,
    )
}

fn evidence_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        EVIDENCE_SOURCE,
        ConsumerRouteProjectionSourceKind::PreparedOperation,
    )
}

fn adapter_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        ADAPTER_SOURCE,
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    )
}

fn observation_source() -> ConsumerRouteProjectionSourceIdentity {
    consumer_route_projection_source(
        OBSERVATION_SOURCE,
        ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
    )
}

fn feature_row(
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

fn control_row(
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

fn selection_features() -> Vec<ConsumerRouteFeatureId> {
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

fn namespaced_feature(index: usize) -> ConsumerRouteFeatureId {
    ConsumerRouteFeatureId::Namespaced(
        ConsumerRouteNamespacedExtension::new(
            "fixture.route",
            "fixture-version-1",
            format!("fixture.extension.{index}"),
        )
        .expect("bounded extension is admitted"),
    )
}

fn rows(
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

fn contribution(
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

fn compose(
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

fn assert_kind(failure: &ConsumerRouteProjectionFailure, kind: ConsumerRouteProjectionFailureKind) {
    assert_eq!(failure.kind(), kind, "{:?}", failure.diagnostic());
    assert!(!failure.diagnostic().message().is_empty());
    assert!(failure.diagnostic().message().len() <= MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES);
}

fn assert_fixed_maxima() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();

    assert_kind(
        &contribution(
            &applicability,
            rows(
                &applicability,
                MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS + 1,
                ConsumerRouteLifecycle::SelectionSummary,
            ),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("selection summary row maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            rows(
                &applicability,
                MAX_CONSUMER_ROUTE_SESSION_START_ROWS + 1,
                ConsumerRouteLifecycle::SessionStartOnly,
            ),
            Vec::new(),
        )
        .expect_err("session start row maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            rows(
                &applicability,
                MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS + 1,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            ),
        )
        .expect_err("active session row maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );

    assert_kind(
        &ConsumerRouteEnumeratedValues::new((0..=MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES).map(
            |index| {
                ConsumerRouteEnumerableValue::new(format!("value-{index}")).expect("value is valid")
            },
        ))
        .expect_err("enumerable value maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &ConsumerRouteEnumerableValue::new(
            "v".repeat(MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES + 1),
        )
        .expect_err("enumerable value byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert!(
        ConsumerRouteEnumerableValue::new("v".repeat(MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES))
            .is_ok(),
        "the exact maximum is admitted rather than truncated"
    );

    assert_kind(
        &ConsumerRouteNamespacedExtension::new(
            "fixture.route",
            "fixture-version-1",
            "x".repeat(MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES + 1),
        )
        .expect_err("extension text byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &contribution(
            &applicability,
            (0..=MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS)
                .map(|index| {
                    feature_row(
                        &applicability,
                        namespaced_feature(index),
                        ConsumerRouteLifecycle::SelectionSummary,
                    )
                })
                .collect(),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("namespaced extension maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );

    assert_kind(
        &ConsumerRouteProjectionSourceId::new("s".repeat(MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES + 1))
            .expect_err("source id byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );
    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            (0..=MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES).map(|index| {
                consumer_route_projection_source(
                    &format!("fixture.source.{index}"),
                    ConsumerRouteProjectionSourceKind::AdapterContribution,
                )
            }),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("source identity maximum is fixed"),
        ConsumerRouteProjectionFailureKind::LimitExceeded,
    );

    assert_kind(
        &ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::Credential,
            ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id is valid"),
            SafeDiagnostic::new(
                "fixture.reason",
                "r".repeat(MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES + 1),
            ),
        )
        .expect_err("safe reason byte maximum is fixed"),
        ConsumerRouteProjectionFailureKind::SafeReasonLimitExceeded,
    );
}

fn assert_failure_kinds() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();

    assert_kind(
        &ConsumerRouteProjectionSourceId::new("   ").expect_err("blank identity is rejected"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );
    assert_kind(
        &ConsumerRouteProjectionSourceId::new("bad\u{7}id")
            .expect_err("control-bearing identity is rejected"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );

    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            [adapter_source(), adapter_source()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a repeated source identity is rejected"),
        ConsumerRouteProjectionFailureKind::DuplicateSource,
    );

    let duplicate = feature_row(
        &applicability,
        ConsumerRouteFeatureId::InteractiveSession,
        ConsumerRouteLifecycle::SelectionSummary,
    );
    assert_kind(
        &contribution(
            &applicability,
            vec![duplicate.clone(), duplicate],
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a repeated row identity is rejected"),
        ConsumerRouteProjectionFailureKind::DuplicateRow,
    );

    assert_kind(
        &contribution(
            &applicability,
            vec![feature_row(
                &applicability,
                ConsumerRouteFeatureId::InteractiveSession,
                ConsumerRouteLifecycle::SessionStartOnly,
            )],
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a row whose lifecycle does not admit it to the view is rejected"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let superseded = ConsumerRouteProjectionFixture::superseded().applicability();
    assert_kind(
        &contribution(
            &applicability,
            vec![feature_row(
                &superseded,
                ConsumerRouteFeatureId::InteractiveSession,
                ConsumerRouteLifecycle::SelectionSummary,
            )],
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a row bound to another revision is rejected"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let foreign = contribution(
        &superseded,
        vec![feature_row(
            &superseded,
            ConsumerRouteFeatureId::InteractiveSession,
            ConsumerRouteLifecycle::SelectionSummary,
        )],
        Vec::new(),
        Vec::new(),
    )
    .expect("the foreign contribution is internally consistent");
    assert_kind(
        &compose(&fixture, &[&foreign]).expect_err("a mixed snapshot is rejected as a whole"),
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
    );

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            vec![
                feature_row(
                    &applicability,
                    ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                )
                .with_state_support(
                    ConsumerRouteStateSupport::descriptor_only().with_provider_effective(),
                ),
            ],
        )
        .expect_err("an effective claim without acknowledgement authority is rejected"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![
                feature_row(
                    &applicability,
                    ConsumerRouteFeatureId::InteractiveSession,
                    ConsumerRouteLifecycle::SessionStartOnly,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
                .with_mutation_authority(
                    ConsumerRouteMutationAuthority::PreparedSessionStart(
                        ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE)
                            .expect("source id is valid"),
                    ),
                ),
            ],
            Vec::new(),
        )
        .expect_err("a selectable row must publish its value domain"),
        ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![
                feature_row(
                    &applicability,
                    ConsumerRouteFeatureId::InteractiveSession,
                    ConsumerRouteLifecycle::SessionStartOnly,
                )
                .with_control_value(ConsumerRouteControlValue::new(
                    ConsumerRouteValueKind::CapabilityState,
                    ConsumerRouteValueDomain::Descriptor,
                    ConsumerRouteOmissionSemantics::SuppliesNothing,
                )),
            ],
            Vec::new(),
        )
        .expect_err("a descriptor-only domain cannot carry selectable omission truth"),
        ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
    );

    assert_kind(
        &ConsumerRouteEnumeratedValues::new([
            ConsumerRouteEnumerableValue::new("low").expect("value is valid"),
            ConsumerRouteEnumerableValue::new("low").expect("value is valid"),
        ])
        .expect_err("a repeated admitted value is rejected"),
        ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
    );
}

fn assert_named_counterexamples() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let superseded_fixture = ConsumerRouteProjectionFixture::superseded();

    let route_wide = feature_row(
        &superseded_fixture.applicability(),
        ConsumerRouteFeatureId::ExternalSearch,
        ConsumerRouteLifecycle::SelectionSummary,
    );
    assert_kind(
        &contribution(&applicability, vec![route_wide], Vec::new(), Vec::new())
            .expect_err("applicability disagreement rejects the row before publication"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let record = fixture.record();
    let stale = superseded_fixture.prepared();
    assert_kind(
        &compose_consumer_route_projection(ConsumerRouteProjectionInput::new(
            &record,
            record_source(),
            &stale,
            evidence_source(),
        ))
        .expect_err("a superseded revision rejects the whole assembly"),
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
    );

    let post_open_selectable = control_row(
        &applicability,
        ConsumerRouteControlId::SessionOptions,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            vec![post_open_selectable],
        )
        .expect_err("a post-open option list may not be presented as selectable"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    let per_turn_as_session_start = control_row(
        &applicability,
        ConsumerRouteControlId::UserInputExchange,
        ConsumerRouteLifecycle::PerTurn,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![per_turn_as_session_start],
            Vec::new(),
        )
        .expect_err("a per-turn exchange may not claim session-start preparation"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    assert_kind(
        &ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::CatalogueResult,
            ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id is valid"),
            SafeDiagnostic::new(
                "fixture.reason",
                "r".repeat(MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES + 1),
            ),
        )
        .expect_err("an unbounded reason claim is rejected"),
        ConsumerRouteProjectionFailureKind::SafeReasonLimitExceeded,
    );

    let unnamed_reason = feature_row(
        &applicability,
        ConsumerRouteFeatureId::ModelCatalogue,
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_safe_reason(
        ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::CatalogueResult,
            ConsumerRouteProjectionSourceId::new("fixture.source.absent")
                .expect("source id is valid"),
            SafeDiagnostic::new("fixture.reason", "catalogue result is unknown"),
        )
        .expect("bounded reason is admitted"),
    );
    assert_kind(
        &contribution(&applicability, vec![unnamed_reason], Vec::new(), Vec::new())
            .expect_err("a reason no named source supplied is rejected"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );
}

fn assert_view_and_lifecycle_separation() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let acknowledged = feature_row(
        &applicability,
        ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
        ConsumerRouteLifecycle::PostOpenObservationOnly,
    )
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_state_support(
        ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_provider_effective(),
    )
    .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
        ConsumerRouteProjectionSourceId::new(OBSERVATION_SOURCE).expect("source id is valid"),
    ));
    let per_turn = control_row(
        &applicability,
        ConsumerRouteControlId::UserInputExchange,
        ConsumerRouteLifecycle::PerTurn,
    )
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed());
    let admitted = contribution(
        &applicability,
        vec![feature_row(
            &applicability,
            ConsumerRouteFeatureId::InteractiveSession,
            ConsumerRouteLifecycle::SelectionSummary,
        )],
        vec![
            control_row(
                &applicability,
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteLifecycle::SessionStartOnly,
            ),
            per_turn,
        ],
        vec![acknowledged],
    )
    .expect("the lifecycle-separated contribution is admitted");
    let projection = compose(&fixture, &[&admitted]).expect("the projection composes");

    assert_eq!(projection.selection_summary().rows().len(), 1);
    assert_eq!(projection.session_start_controls().rows().len(), 2);
    assert_eq!(projection.active_session_state().rows().len(), 1);
    assert!(
        projection
            .selection_summary()
            .rows()
            .all(|row| row.lifecycle() == ConsumerRouteLifecycle::SelectionSummary)
    );
    assert!(projection.session_start_controls().rows().any(|row| {
        row.lifecycle() == ConsumerRouteLifecycle::PerTurn
            && row.state_support().observed()
            && !row.state_support().prepared()
    }));
    assert!(projection.active_session_state().rows().all(|row| {
        row.lifecycle() == ConsumerRouteLifecycle::PostOpenObservationOnly
            && row.mutation_authority().is_acknowledged()
    }));
    assert_eq!(projection.sources().len(), 4);
    assert_eq!(
        projection.identity().applicability(),
        &fixture.applicability()
    );
}

fn assert_identical_row_source_replacement() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let row = feature_row(
        &applicability,
        ConsumerRouteFeatureId::InteractiveSession,
        ConsumerRouteLifecycle::SelectionSummary,
    );
    let first = contribution(&applicability, vec![row.clone()], Vec::new(), Vec::new())
        .expect("the first contribution is admitted");
    let replaced_source = consumer_route_projection_source(
        "fixture.source.adapter-contribution-2",
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    );
    let replaced_row = ConsumerRouteProjectionRow::new(
        row.identity().clone(),
        applicability.clone(),
        replaced_source.clone(),
        row.source_class(),
        row.evidence_strength(),
        row.lifecycle(),
    )
    .with_support(row.support())
    .with_availability(row.availability());
    let second = ConsumerRouteProjectionContribution::new(
        applicability.clone(),
        [replaced_source],
        vec![replaced_row],
        Vec::new(),
        Vec::new(),
    )
    .expect("the replacement contribution is admitted");

    let before = compose(&fixture, &[&first]).expect("the first projection composes");
    let after = compose(&fixture, &[&second]).expect("the replacement projection composes");
    assert_ne!(
        before, after,
        "equal row content under a changed source identity is still a replacement"
    );
    assert_ne!(before.identity(), after.identity());
    assert_eq!(
        before
            .selection_summary()
            .rows()
            .next()
            .map(|row| row.identity()),
        after
            .selection_summary()
            .rows()
            .next()
            .map(|row| row.identity())
    );
}

fn assert_unknown_and_absent_truth_survives() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let unknown = ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ModelCatalogue),
        applicability.clone(),
        adapter_source(),
        ConsumerRouteSourceClass::ModelCatalogueObservation,
        ConsumerRouteEvidenceStrength::RuntimeType,
        ConsumerRouteLifecycle::SelectionSummary,
    );
    let unenumerated = control_row(
        &applicability,
        ConsumerRouteControlId::DeveloperInstructions,
        ConsumerRouteLifecycle::SessionStartOnly,
    )
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::StructuredContent,
        ConsumerRouteValueDomain::Unenumerated(
            ConsumerRouteEnumerableValue::new("bounded developer instruction content")
                .expect("bound is valid"),
        ),
        ConsumerRouteOmissionSemantics::SuppliesNothing,
    ));
    let admitted = contribution(
        &applicability,
        vec![unknown],
        vec![unenumerated],
        Vec::new(),
    )
    .expect("unknown and unenumerated truth is admitted");
    let projection = compose(&fixture, &[&admitted]).expect("the projection composes");

    let row = projection
        .selection_summary()
        .rows()
        .next()
        .expect("the unknown row survives");
    assert_eq!(row.support(), ConsumerRouteSupportPosture::Unknown);
    assert_eq!(row.availability(), ConsumerRouteAvailability::Unknown);
    assert!(row.safe_reason().is_none(), "absence invents no reason");
    assert!(row.state_support().is_descriptor_only());

    let control = projection
        .session_start_controls()
        .rows()
        .next()
        .expect("the unenumerated control survives");
    let value = control.control_value().expect("a control carries a value");
    assert!(matches!(
        value.domain(),
        ConsumerRouteValueDomain::Unenumerated(_)
    ));
    assert_eq!(
        value.omission(),
        ConsumerRouteOmissionSemantics::SuppliesNothing,
        "omission produces no Swallowtail default"
    );
}

fn assert_no_raw_or_presentation_data() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let admitted = contribution(
        &applicability,
        vec![feature_row(
            &applicability,
            ConsumerRouteFeatureId::PreparedFacade,
            ConsumerRouteLifecycle::SelectionSummary,
        )],
        vec![control_row(
            &applicability,
            ConsumerRouteControlId::ModelSelection,
            ConsumerRouteLifecycle::SessionStartOnly,
        )],
        Vec::new(),
    )
    .expect("the contribution is admitted");
    let projection = compose(&fixture, &[&admitted]).expect("the projection composes");
    let rendered = format!("{projection:?}");
    for forbidden in [
        CONSUMER_ROUTE_PRIVATE_TARGET,
        CONSUMER_ROUTE_PRIVATE_CREDENTIAL,
    ] {
        assert!(
            !rendered.contains(forbidden),
            "projection must not carry raw target, credential, path, or environment data"
        );
    }
}
