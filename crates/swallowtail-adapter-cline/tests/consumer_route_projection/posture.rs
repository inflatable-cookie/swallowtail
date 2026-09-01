#[test]
fn every_maximal_cline_row_keeps_exact_posture_and_source() {
    let prepared_source = source("cline.posture.prepared");
    let active_source = source("cline.posture.active");
    let (prepared, _, services) = session(Scenario::ModelExact, true, "posture");
    let outcome = block_on(prepared.open_session_with_projection(
        prepared_source.clone(),
        active_source.clone(),
        services,
    ))
    .unwrap_or_else(|failure| panic!("open failed: {}", failure.failure()));
    let contribution = outcome.contribution();
    assert_eq!(projection_rows(contribution).count(), 9);
    for row in contribution.selection_rows() {
        assert_cline_row(row, "selection-summary", &prepared_source, &active_source);
    }
    for row in contribution.session_start_rows() {
        assert_cline_row(row, "session-start", &prepared_source, &active_source);
    }
    for row in contribution.active_session_rows() {
        assert_cline_row(row, "active-session", &prepared_source, &active_source);
    }
    assert_eq!(block_on(outcome.into_parts().0.close()), CleanupOutcome::Clean);
}

fn assert_cline_row(
    row: &swallowtail_runtime::ConsumerRouteProjectionRow,
    view: &str,
    prepared_source: &ConsumerRouteProjectionSourceId,
    active_source: &ConsumerRouteProjectionSourceId,
) {
    use swallowtail_runtime::{
        ConsumerRouteActorPosture as Actor, ConsumerRouteEvidenceStrength as Evidence,
        ConsumerRouteLifecycle as Lifecycle, ConsumerRouteOmissionSemantics as Omission,
        ConsumerRouteProjectionSourceKind as SourceKind,
        ConsumerRouteSourceClass as Class, ConsumerRouteStateSupport as State,
        ConsumerRouteValueKind as Kind,
    };

    assert_eq!(row.applicability().operation_shape(), swallowtail_core::OperationShape::InteractiveSession);
    assert_eq!(row.support(), swallowtail_runtime::ConsumerRouteSupportPosture::Supported);
    assert_eq!(row.availability(), swallowtail_runtime::ConsumerRouteAvailability::Available);
    let semantic = match row.identity() {
        identity if identity.namespaced_extension().is_some() => identity
            .namespaced_extension()
            .expect("extension")
            .semantic_id(),
        swallowtail_runtime::ConsumerRouteRowIdentity::Feature(feature) => match feature {
            swallowtail_runtime::ConsumerRouteFeatureId::InteractiveSession => {
                "feature.interactive-session"
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
            other => panic!("unexpected Cline feature {other:?}"),
        },
        other => panic!("unexpected Cline row {other:?}"),
    };
    match semantic {
        "feature.prepared-facade" => assert_exact(
            row, view, "selection-summary", prepared_source, Class::PreparedOperationRecord,
            Evidence::PreparedOperation, Lifecycle::SelectionSummary, Actor::Informational,
            State::descriptor_only(), SourceKind::AdapterContribution, false, None, None, None,
        ),
        "feature.interactive-session" | "feature.streaming-events"
        | "feature.cancellation-or-interruption" | "feature.working-resource" => assert_exact(
            row, view, "selection-summary", prepared_source, Class::CapabilityProfile,
            Evidence::PreparedOperation, Lifecycle::SelectionSummary, Actor::Informational,
            State::descriptor_only(), SourceKind::AdapterContribution, false, None, None, None,
        ),
        "feature.activity-observation" => assert_exact(
            row, view, "active-session", prepared_source, Class::PreparedOperationRecord,
            Evidence::PreparedOperation, Lifecycle::PostOpenObservationOnly,
            Actor::ObservationOnly, State::descriptor_only(), SourceKind::AdapterContribution,
            false, None, None, None,
        ),
        "control.harness-mode" => {
            assert!(row.mutation_authority().is_prepared_session_start());
            assert_exact(
                row, view, "session-start", prepared_source, Class::AdapterPreparedInput,
                Evidence::RouteValidation, Lifecycle::SessionStartOnly, Actor::ConsumerSelectable,
                State::descriptor_only().with_requested().with_prepared().with_pending(),
                SourceKind::AdapterContribution, true, Some(Kind::AcknowledgedEnumeration),
                Some(ExpectedDomain::Enumerated("plan")),
                Some(Omission::PreservesRouteBehavior),
            );
        }
        "feature.active-session-plan-ack" => {
            assert!(row.mutation_authority().is_acknowledged());
            assert_exact(
                row, view, "active-session", active_source, Class::RouteAcknowledgementEvidence,
                Evidence::WireAcknowledgement, Lifecycle::PostOpenObservationOnly,
                Actor::ObservationOnly,
                State::descriptor_only().with_requested().with_provider_effective(),
                SourceKind::ActiveSessionObservation, true, Some(Kind::AcknowledgementState),
                Some(ExpectedDomain::Enumerated("plan")), Some(Omission::NotSelectable),
            );
        }
        "feature.negotiated-model-options-observation" => assert_exact(
            row, view, "active-session", active_source, Class::RouteAcknowledgementEvidence,
            Evidence::WireAcknowledgement, Lifecycle::PostOpenObservationOnly,
            Actor::ObservationOnly, State::descriptor_only().with_observed(),
            SourceKind::ActiveSessionObservation, false, Some(Kind::Observation),
            Some(ExpectedDomain::Unenumerated(
                "exact bounded negotiated model options on the open session",
            )), Some(Omission::NotSelectable),
        ),
        other => panic!("unexpected Cline semantic {other}"),
    }
}

#[derive(Clone, Copy)]
enum ExpectedDomain {
    Enumerated(&'static str),
    Unenumerated(&'static str),
}

#[allow(clippy::too_many_arguments)]
fn assert_exact(
    row: &swallowtail_runtime::ConsumerRouteProjectionRow,
    view: &str,
    expected_view: &str,
    source: &ConsumerRouteProjectionSourceId,
    source_class: swallowtail_runtime::ConsumerRouteSourceClass,
    evidence: swallowtail_runtime::ConsumerRouteEvidenceStrength,
    lifecycle: swallowtail_runtime::ConsumerRouteLifecycle,
    actor: swallowtail_runtime::ConsumerRouteActorPosture,
    state: swallowtail_runtime::ConsumerRouteStateSupport,
    source_kind: swallowtail_runtime::ConsumerRouteProjectionSourceKind,
    expected_mutation: bool,
    value_kind: Option<swallowtail_runtime::ConsumerRouteValueKind>,
    value_domain: Option<ExpectedDomain>,
    omission: Option<swallowtail_runtime::ConsumerRouteOmissionSemantics>,
) {
    assert_eq!(view, expected_view);
    assert_eq!(row.source().id(), source);
    assert_eq!(row.source().kind(), source_kind);
    assert_eq!(row.source_class(), source_class);
    assert_eq!(row.evidence_strength(), evidence);
    assert_eq!(row.lifecycle(), lifecycle);
    assert_eq!(row.actor_posture(), actor);
    assert_eq!(row.state_support(), state);
    assert_eq!(row.control_value().map(|value| value.kind()), value_kind);
    assert_value_domain(row, value_domain);
    assert_eq!(row.control_value().map(|value| value.omission()), omission);
    if expected_mutation {
        assert_eq!(row.mutation_authority().source(), Some(source));
    } else {
        assert!(row.mutation_authority().source().is_none());
    }
}

fn assert_value_domain(
    row: &swallowtail_runtime::ConsumerRouteProjectionRow,
    expected: Option<ExpectedDomain>,
) {
    use swallowtail_runtime::ConsumerRouteValueDomain;

    match (row.control_value().map(|value| value.domain()), expected) {
        (None, None) => {}
        (
            Some(ConsumerRouteValueDomain::Enumerated(values)),
            Some(ExpectedDomain::Enumerated(text)),
        ) => {
            assert_eq!(
                values
                    .values()
                    .map(|value| value.as_str())
                    .collect::<Vec<_>>(),
                [text]
            );
        }
        (
            Some(ConsumerRouteValueDomain::Unenumerated(value)),
            Some(ExpectedDomain::Unenumerated(text)),
        ) => {
            assert_eq!(value.as_str(), text);
        }
        (actual, expected) => panic!(
            "unexpected value domain: actual={actual:?}, expected={}",
            match expected {
                None => "none",
                Some(ExpectedDomain::Enumerated(_)) => "enumerated",
                Some(ExpectedDomain::Unenumerated(_)) => "unenumerated",
            }
        ),
    }
}
