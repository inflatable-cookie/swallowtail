#[test]
fn every_headless_identity_keeps_exact_posture_source_and_value() {
    let source_id = source("cline.posture.headless");
    let contribution = headless_run(true)
        .consumer_route_projection_contribution(source_id.clone())
        .expect("headless contributes");
    assert_eq!(projection_rows(&contribution).count(), 7);
    for row in contribution.selection_rows() {
        assert_cline_headless_row(row, "selection-summary", &source_id);
    }
    for row in contribution.session_start_rows() {
        assert_cline_headless_row(row, "session-start", &source_id);
    }
    for row in contribution.active_session_rows() {
        assert_cline_headless_row(row, "active-session", &source_id);
    }
}

fn assert_cline_headless_row(
    row: &swallowtail_runtime::ConsumerRouteProjectionRow,
    view: &str,
    source_id: &ConsumerRouteProjectionSourceId,
) {
    use swallowtail_runtime::{
        ConsumerRouteActorPosture as Actor, ConsumerRouteEvidenceStrength as Evidence,
        ConsumerRouteLifecycle as Lifecycle, ConsumerRouteOmissionSemantics as Omission,
        ConsumerRouteProjectionSourceKind as SourceKind, ConsumerRouteSourceClass as Class,
        ConsumerRouteStateSupport as State, ConsumerRouteValueKind as Kind,
    };

    assert_eq!(
        row.applicability().operation_shape(),
        swallowtail_core::OperationShape::StructuredRun
    );
    assert_eq!(
        row.support(),
        swallowtail_runtime::ConsumerRouteSupportPosture::Supported
    );
    assert_eq!(
        row.availability(),
        swallowtail_runtime::ConsumerRouteAvailability::Available
    );
    let semantic = match row.identity() {
        identity if identity.namespaced_extension().is_some() => identity
            .namespaced_extension()
            .expect("extension")
            .semantic_id(),
        swallowtail_runtime::ConsumerRouteRowIdentity::Feature(feature) => match feature {
            swallowtail_runtime::ConsumerRouteFeatureId::StructuredRun => {
                "feature.structured-run"
            }
            swallowtail_runtime::ConsumerRouteFeatureId::StreamingEvents => {
                "feature.streaming-events"
            }
            swallowtail_runtime::ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            swallowtail_runtime::ConsumerRouteFeatureId::WorkingResource => {
                "feature.working-resource"
            }
            swallowtail_runtime::ConsumerRouteFeatureId::ActivityObservation => {
                "feature.activity-observation"
            }
            swallowtail_runtime::ConsumerRouteFeatureId::PreparedFacade => {
                "feature.prepared-facade"
            }
            other => panic!("unexpected Cline headless feature {other:?}"),
        },
        other => panic!("unexpected Cline headless row {other:?}"),
    };
    match semantic {
        "feature.prepared-facade" => assert_exact(
            row,
            view,
            "selection-summary",
            source_id,
            Class::PreparedOperationRecord,
            Evidence::PreparedOperation,
            Lifecycle::SelectionSummary,
            Actor::Informational,
            State::descriptor_only(),
            SourceKind::AdapterContribution,
            false,
            None,
            None,
            None,
        ),
        "feature.structured-run"
        | "feature.streaming-events"
        | "feature.cancellation-or-interruption"
        | "feature.working-resource" => assert_exact(
            row,
            view,
            "selection-summary",
            source_id,
            Class::CapabilityProfile,
            Evidence::PreparedOperation,
            Lifecycle::SelectionSummary,
            Actor::Informational,
            State::descriptor_only(),
            SourceKind::AdapterContribution,
            false,
            None,
            None,
            None,
        ),
        "feature.activity-observation" => assert_exact(
            row,
            view,
            "active-session",
            source_id,
            Class::PreparedOperationRecord,
            Evidence::PreparedOperation,
            Lifecycle::PostOpenObservationOnly,
            Actor::ObservationOnly,
            State::descriptor_only(),
            SourceKind::AdapterContribution,
            false,
            None,
            None,
            None,
        ),
        "control.harness-mode" => {
            assert!(row.mutation_authority().is_prepared_session_start());
            assert_exact(
                row,
                view,
                "session-start",
                source_id,
                Class::AdapterPreparedInput,
                Evidence::RouteValidation,
                Lifecycle::SessionStartOnly,
                Actor::ConsumerSelectable,
                State::descriptor_only().with_requested().with_prepared(),
                SourceKind::AdapterContribution,
                true,
                Some(Kind::BoundedEnumeration),
                Some(ExpectedDomain::Enumerated("plan")),
                Some(Omission::PreservesRouteBehavior),
            );
        }
        other => panic!("unexpected Cline headless semantic {other}"),
    }
}
