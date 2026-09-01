use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteControlId,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionSourceKind, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteSupportPosture, ConsumerRouteValueDomain,
    ConsumerRouteValueKind,
};

use super::{common, identity_name, rows, source};

pub(super) fn observed_tuples(
    projection: &ConsumerRouteProjectionContribution,
) -> BTreeSet<(String, String, String)> {
    super::identities(projection)
        .into_iter()
        .map(|semantic| {
            let shape = match semantic {
                "feature.structured-run" => "structured-run",
                "feature.interactive-session" => "interactive-session",
                "control.model-selection" => match projection.applicability().operation_shape() {
                    swallowtail_core::OperationShape::StructuredRun => "structured-run",
                    swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                    other => panic!("unexpected Command Code operation {other:?}"),
                },
                "feature.streaming-events"
                | "feature.usage-evidence"
                | "feature.activity-observation" => "route-observation",
                "feature.cancellation-or-interruption"
                | "feature.working-resource"
                | "feature.prepared-facade" => "route-capability",
                _ => panic!("unexpected observed Command Code tuple {semantic}"),
            };
            (
                "command-code.headless".to_owned(),
                shape.to_owned(),
                semantic.to_owned(),
            )
        })
        .collect()
}

type Posture = (
    &'static str,
    &'static str,
    ConsumerRouteLifecycle,
    ConsumerRouteActorPosture,
    ConsumerRouteStateSupport,
    ConsumerRouteSourceClass,
    ConsumerRouteEvidenceStrength,
);

#[test]
fn every_identity_on_both_facades_keeps_exact_posture_and_source() {
    let integration = common::prepare(common::host_id());
    let run = integration
        .prepare_run(common::run_input(common::model(), "posture"))
        .expect("run prepares");
    let session = integration
        .prepare_session(super::CommandCodeSessionProfileInput::new(
            swallowtail_runtime::RequestId::new("command-code.projection.posture.session")
                .expect("request"),
            common::model(),
            swallowtail_runtime::WorkingResourceRef::new("command-code.fixture.workspace")
                .expect("resource"),
        ))
        .expect("session prepares");
    for (prepared, source_id) in [
        (
            run.consumer_route_projection_contribution(source("command-code.posture.run"))
                .expect("run contributes"),
            source("command-code.posture.run"),
        ),
        (
            session
                .consumer_route_projection_contribution(source("command-code.posture.session"))
                .expect("session contributes"),
            source("command-code.posture.session"),
        ),
    ] {
        assert_eq!(observed(&prepared), expected(&prepared));
        for row in rows(&prepared) {
            assert_eq!(row.source().id(), &source_id);
            assert_eq!(
                row.source().kind(),
                ConsumerRouteProjectionSourceKind::AdapterContribution
            );
            assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
            assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
            match row.identity() {
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection) => {
                    assert_eq!(row.mutation_authority().source(), Some(&source_id));
                    let value = row.control_value().expect("model value");
                    assert_eq!(value.kind(), ConsumerRouteValueKind::ExactModelRoute);
                    assert_eq!(value.omission(), ConsumerRouteOmissionSemantics::Required);
                    let ConsumerRouteValueDomain::Enumerated(values) = value.domain() else {
                        panic!("model selection needs an exact enumerated domain");
                    };
                    assert_eq!(
                        values
                            .values()
                            .map(|value| value.as_str())
                            .collect::<Vec<_>>(),
                        [common::FIXTURE_MODEL_ID]
                    );
                }
                _ => {
                    assert!(row.mutation_authority().source().is_none());
                    assert!(row.control_value().is_none());
                }
            }
        }
    }
}

fn expected(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<Posture> {
    rows(contribution)
        .map(|row| match row.identity() {
            ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation) => (
                "feature.activity-observation",
                "active-session",
                ConsumerRouteLifecycle::PostOpenObservationOnly,
                ConsumerRouteActorPosture::ObservationOnly,
                ConsumerRouteStateSupport::descriptor_only(),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
            ),
            ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade) => summary(
                "feature.prepared-facade",
                ConsumerRouteActorPosture::Informational,
                ConsumerRouteStateSupport::descriptor_only(),
                ConsumerRouteSourceClass::PreparedOperationRecord,
            ),
            ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection) => summary(
                "control.model-selection",
                ConsumerRouteActorPosture::ConsumerSelectable,
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
                ConsumerRouteSourceClass::PreparedOperationRecord,
            ),
            ConsumerRouteRowIdentity::Feature(_) => summary(
                identity_name(row),
                ConsumerRouteActorPosture::Informational,
                ConsumerRouteStateSupport::descriptor_only(),
                ConsumerRouteSourceClass::CapabilityProfile,
            ),
            other => panic!("unexpected Command Code posture row {other:?}"),
        })
        .collect()
}

fn summary(
    semantic: &'static str,
    actor: ConsumerRouteActorPosture,
    state: ConsumerRouteStateSupport,
    source_class: ConsumerRouteSourceClass,
) -> Posture {
    (
        semantic,
        "selection-summary",
        ConsumerRouteLifecycle::SelectionSummary,
        actor,
        state,
        source_class,
        ConsumerRouteEvidenceStrength::PreparedOperation,
    )
}

fn observed(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<Posture> {
    contribution
        .selection_rows()
        .map(|row| (row, "selection-summary"))
        .chain(
            contribution
                .active_session_rows()
                .map(|row| (row, "active-session")),
        )
        .map(|(row, view)| {
            (
                identity_name(row),
                view,
                row.lifecycle(),
                row.actor_posture(),
                row.state_support(),
                row.source_class(),
                row.evidence_strength(),
            )
        })
        .collect()
}
