#![allow(dead_code)]

#[path = "corpus/common.rs"]
mod common;
mod support;

use std::collections::BTreeSet;
use swallowtail_adapter_command_code::CommandCodeSessionProfileInput;
use swallowtail_core::OperationShape;
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteControlId, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId,
    ConsumerRouteRowIdentity, ConsumerRouteStateSupport, ConsumerRouteSupportPosture, RequestId,
    WorkingResourceRef,
};

#[test]
fn exact_run_and_session_facades_reconcile_the_eleven_row_ledger() {
    let integration = common::prepare(common::host_id());
    let run = integration
        .prepare_run(common::run_input(common::model(), "projection"))
        .expect("run prepares");
    let session = integration
        .prepare_session(CommandCodeSessionProfileInput::new(
            RequestId::new("command-code.projection.session").expect("request"),
            common::model(),
            WorkingResourceRef::new("command-code.fixture.workspace").expect("resource"),
        ))
        .expect("session prepares");
    let run_projection = run
        .consumer_route_projection_contribution(source("command-code.projection.run"))
        .expect("run contributes");
    let session_projection = session
        .consumer_route_projection_contribution(source("command-code.projection.session"))
        .expect("session contributes");

    assert_eq!(
        run_projection.applicability().operation_shape(),
        OperationShape::StructuredRun
    );
    assert_eq!(
        session_projection.applicability().operation_shape(),
        OperationShape::InteractiveSession
    );
    assert!(identities(&run_projection).contains("feature.structured-run"));
    assert!(!identities(&run_projection).contains("feature.interactive-session"));
    assert!(identities(&session_projection).contains("feature.interactive-session"));
    assert!(!identities(&session_projection).contains("feature.structured-run"));
    assert!(identities(&run_projection).contains("control.model-selection"));
    assert!(identities(&session_projection).contains("control.model-selection"));

    let union = identities(&run_projection)
        .into_iter()
        .chain(identities(&session_projection))
        .collect::<BTreeSet<_>>();
    assert_eq!(union.len(), 9);
    let exact_tuples = [
        (&run_projection, "structured-run"),
        (&session_projection, "interactive-session"),
    ]
    .into_iter()
    .flat_map(|(projection, operation)| {
        identities(projection).into_iter().map(move |identity| {
            if identity == "control.model-selection" {
                format!("{operation}:{identity}")
            } else {
                identity.to_owned()
            }
        })
    })
    .collect::<BTreeSet<_>>();
    assert_eq!(exact_tuples.len(), 10);
    assert!(!union.contains("feature.model-catalogue"));
    assert!(!union.contains("feature.persistent-session-posture"));
    for projection in [&run_projection, &session_projection] {
        assert_exact_posture(projection);
        let model = rows(projection)
            .find(|row| {
                row.identity()
                    == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection)
            })
            .expect("exact model selection row");
        assert_eq!(model.lifecycle(), ConsumerRouteLifecycle::SelectionSummary);
        assert!(model.state_support().requested());
        assert!(model.state_support().prepared());
        assert!(!model.state_support().provider_effective());
        assert!(!model.state_support().observed());
    }

    const LEDGER: [(&str, &str, bool); 11] = [
        ("model-catalogue", "feature.model-catalogue", false),
        ("structured-run", "feature.structured-run", true),
        ("interactive-session", "feature.interactive-session", true),
        ("route-observation", "feature.streaming-events", true),
        ("route-observation", "feature.usage-evidence", true),
        (
            "route-capability",
            "feature.cancellation-or-interruption",
            true,
        ),
        ("route-capability", "feature.working-resource", true),
        ("route-capability", "feature.prepared-facade", true),
        ("route-observation", "feature.activity-observation", true),
        ("structured-run", "control.model-selection", true),
        ("interactive-session", "control.model-selection", true),
    ];
    let ledger = LEDGER
        .iter()
        .map(|(shape, semantic, _)| {
            (
                "command-code.headless".to_owned(),
                (*shape).to_owned(),
                (*semantic).to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(ledger, census_tuples());
    assert_eq!(LEDGER.iter().filter(|row| row.2).count(), 10);
    assert_eq!(LEDGER.iter().filter(|row| !row.2).count(), 1);

    let run_row = rows(&run_projection).next().expect("run row").clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        session_projection.applicability().clone(),
        [run_row.source().clone()],
        [run_row],
        [],
        [],
    )
    .expect_err("structured row cannot enter interactive applicability");
    assert_eq!(
        rejection.kind(),
        swallowtail_runtime::ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
    let session_row = rows(&session_projection)
        .next()
        .expect("session row")
        .clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        run_projection.applicability().clone(),
        [session_row.source().clone()],
        [session_row],
        [],
        [],
    )
    .expect_err("interactive row cannot enter structured applicability");
    assert_eq!(
        rejection.kind(),
        swallowtail_runtime::ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

fn assert_exact_posture(projection: &ConsumerRouteProjectionContribution) {
    for row in rows(projection) {
        assert_eq!(row.applicability(), projection.applicability());
        assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
        assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
        if row.identity()
            == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
        {
            assert_eq!(
                row.lifecycle(),
                ConsumerRouteLifecycle::PostOpenObservationOnly
            );
            assert_eq!(
                row.state_support(),
                ConsumerRouteStateSupport::descriptor_only()
            );
        }
    }
}

fn identities(projection: &ConsumerRouteProjectionContribution) -> BTreeSet<&'static str> {
    rows(projection)
        .map(|row| match row.identity() {
            ConsumerRouteRowIdentity::Feature(feature) => match feature {
                ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
                ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
                ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
                ConsumerRouteFeatureId::UsageEvidence => "feature.usage-reporting",
                ConsumerRouteFeatureId::CancellationOrInterruption => {
                    "feature.cancellation-or-interruption"
                }
                ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
                ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
                ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
                other => panic!("unexpected feature {other:?}"),
            },
            ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection) => {
                "control.model-selection"
            }
            other => panic!("unexpected row {other:?}"),
        })
        .collect()
}

fn rows(
    projection: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow> {
    projection
        .selection_rows()
        .chain(projection.session_start_rows())
        .chain(projection.active_session_rows())
}

fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source")
}

fn census_tuples() -> BTreeSet<(String, String, String)> {
    include_str!("fixtures/consumer-route-projection-census.csv")
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split(',');
            let route = fields.next()?;
            let shape = fields.next()?;
            let semantic = fields.next()?;
            (route == "command-code.headless")
                .then(|| (route.to_owned(), shape.to_owned(), semantic.to_owned()))
        })
        .collect()
}
