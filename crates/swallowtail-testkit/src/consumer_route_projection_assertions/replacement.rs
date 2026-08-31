use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEnumerableValue, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass, ConsumerRouteSupportPosture, ConsumerRouteValueDomain,
    ConsumerRouteValueKind,
};

use crate::{
    CONSUMER_ROUTE_PRIVATE_CREDENTIAL, CONSUMER_ROUTE_PRIVATE_TARGET,
    ConsumerRouteProjectionFixture, consumer_route_projection_source,
};

use super::support::*;

pub(super) fn assert_identical_row_source_replacement() {
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

pub(super) fn assert_unknown_and_absent_truth_survives() {
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

pub(super) fn assert_no_raw_or_presentation_data() {
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
