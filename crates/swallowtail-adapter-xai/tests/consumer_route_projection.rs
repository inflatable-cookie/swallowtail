//! Contract 061 disposition proof for the exact seventeen xAI rows.

#![allow(dead_code)]

mod support;

use std::collections::BTreeSet;
use std::num::NonZeroU64;
use support::{DriverFixture, ServerScenario, qualified_model};
use swallowtail_adapter_xai::{
    XAI_MODELS_ENDPOINT, XaiModelsPreparationInput, XaiModelsProfileInput, XaiRunProfileInput,
    XaiSessionProfileInput, prepare_xai_models, prepare_xai_responses_websocket,
    xai_responses_access_profile,
};
use swallowtail_core::{
    AccessStatus, CredentialState, EndpointAuthorization, EntitlementState, InstanceRevision,
    InstanceTargetRef, ReasoningMode, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow, ConsumerRouteRowIdentity,
    CredentialRef, OperationContent, PreparedAccessEvidence, RequestId,
};
use swallowtail_testkit::RecordingHostServices;

const ROUTE: &str = "xai.responses-websocket";
const MODELS: &str = "XaiPreparedModels";
const RUN: &str = "XaiPreparedResponsesRun[maximal]";
const SESSION: &str = "XaiPreparedResponsesSession[maximal]";
const PROFILES: [&str; 3] = [MODELS, RUN, SESSION];

struct LedgerEntry {
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
}

const XAI_TRANCHE: [LedgerEntry; 17] = [
    entry("model-catalogue", "feature.model-catalogue", &[MODELS]),
    entry("structured-run", "feature.structured-run", &[RUN]),
    entry(
        "interactive-session",
        "feature.interactive-session",
        &[SESSION],
    ),
    entry(
        "route-observation",
        "feature.streaming-events",
        &[RUN, SESSION],
    ),
    entry(
        "route-observation",
        "feature.usage-evidence",
        &[RUN, SESSION],
    ),
    entry(
        "route-observation",
        "feature.billed-cost-evidence",
        &[RUN, SESSION],
    ),
    entry("route-capability", "feature.output-token-limit", &[RUN]),
    entry(
        "route-capability",
        "feature.reasoning-selection",
        &[RUN, SESSION],
    ),
    entry(
        "route-capability",
        "feature.cancellation-or-interruption",
        &[RUN, SESSION],
    ),
    entry(
        "session-lifecycle",
        "feature.persistent-session-posture",
        &[SESSION],
    ),
    entry(
        "route-capability",
        "feature.prepared-facade",
        &[MODELS, RUN, SESSION],
    ),
    entry(
        "route-observation",
        "feature.activity-observation",
        &[RUN, SESSION],
    ),
    entry("structured-run", "control.model-selection", &[RUN]),
    entry("interactive-session", "control.model-selection", &[SESSION]),
    entry("structured-run", "control.reasoning-selection", &[RUN]),
    entry(
        "interactive-session",
        "control.reasoning-selection",
        &[SESSION],
    ),
    entry("structured-run", "control.maximum-output-tokens", &[RUN]),
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
    }
}

#[test]
fn coverage_ledger_is_exactly_seventeen_rows() {
    let identities = XAI_TRANCHE
        .iter()
        .map(|entry| (entry.operation_shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(XAI_TRANCHE.len(), 17);
    assert_eq!(identities.len(), 17);
    assert_eq!(
        XAI_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        17
    );
}

#[test]
fn every_prepared_xai_profile_matches_its_ledger_disposition() {
    let observed = [(MODELS, models()), (RUN, run()), (SESSION, session())]
        .into_iter()
        .map(|(profile, contribution)| {
            let identities = all_rows(&contribution)
                .map(|row| row_identity(row, &contribution))
                .collect::<BTreeSet<_>>();
            (profile, identities)
        })
        .collect::<Vec<_>>();

    assert_eq!(observed.len(), PROFILES.len());
    for (profile, identities) in observed {
        let expected = XAI_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.contains(&profile))
            .map(|entry| (ROUTE, entry.operation_shape, entry.semantic_id))
            .collect::<BTreeSet<_>>();
        assert_eq!(identities, expected, "{profile} disposition differs");
    }
}

#[test]
fn optional_controls_are_negative_and_run_and_session_rows_cannot_mix() {
    let fixture = DriverFixture::new(ServerScenario::OneResponse);
    let prepared =
        prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
            .expect("integration prepares");
    let minimal = prepared
        .prepare_responses_run(XaiRunProfileInput::new(
            RequestId::new("xai.projection.minimal").unwrap(),
            qualified_model("grok-4.5"),
            OperationContent::new("minimal projection run").unwrap(),
            None,
        ))
        .expect("minimal run prepares")
        .consumer_route_projection_contribution(source("xai.projection.minimal"))
        .expect("minimal run contributes");
    let semantics = all_rows(&minimal)
        .map(|row| semantic_id(row.identity()))
        .collect::<BTreeSet<_>>();
    assert!(!semantics.contains("control.reasoning-selection"));
    assert!(!semantics.contains("control.maximum-output-tokens"));
    assert!(!semantics.contains("feature.reasoning-selection"));
    assert!(!semantics.contains("feature.output-token-limit"));
    assert!(!semantics.contains("feature.persistent-session-posture"));

    let run = run();
    let session = session();
    let rejection = ConsumerRouteProjectionContribution::new(
        run.applicability().clone(),
        run.sources().cloned().collect::<Vec<_>>(),
        [all_rows(&session)
            .next()
            .expect("session publishes")
            .clone()],
        [],
        [],
    )
    .expect_err("run and session applicability cannot assemble");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

fn row_identity(
    row: &ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
) -> (&'static str, &'static str, &'static str) {
    (
        ROUTE,
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
            "feature.billed-cost-evidence" => "feature.billed-cost-evidence",
            other => panic!("unexpected xAI descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected xAI feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            other => panic!("unexpected xAI control {other:?}"),
        },
    }
}

fn census_shape(
    identity: &ConsumerRouteRowIdentity,
    operation_shape: swallowtail_core::OperationShape,
) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        assert_eq!(extension.semantic_id(), "feature.billed-cost-evidence");
        return "route-observation";
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents
            | ConsumerRouteFeatureId::UsageEvidence
            | ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            ConsumerRouteFeatureId::OutputTokenLimit
            | ConsumerRouteFeatureId::ReasoningSelection
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            ConsumerRouteFeatureId::PersistentSessionPosture => "session-lifecycle",
            other => panic!("unexpected xAI feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection | ConsumerRouteControlId::ReasoningSelection => {
                match operation_shape {
                    swallowtail_core::OperationShape::StructuredRun => "structured-run",
                    swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                    other => panic!("unexpected xAI control shape {other:?}"),
                }
            }
            ConsumerRouteControlId::MaximumOutputTokens => "structured-run",
            other => panic!("unexpected xAI control {other:?}"),
        },
    }
}

fn source(id: &str) -> swallowtail_runtime::ConsumerRouteProjectionSourceId {
    swallowtail_runtime::ConsumerRouteProjectionSourceId::new(id).expect("source id")
}

fn models() -> ConsumerRouteProjectionContribution {
    let host = RecordingHostServices::default();
    let access = xai_responses_access_profile(CredentialRef::new("xai.projection.models").unwrap());
    let prepared = prepare_xai_models(
        XaiModelsPreparationInput::new(
            InstanceRevision::new("fixture-1").unwrap(),
            host.services().execution_host_id().clone(),
            InstanceTargetRef::new(XAI_MODELS_ENDPOINT).unwrap(),
            access.clone(),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access.id().clone(),
                CredentialState::Ready,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        host.services(),
    )
    .expect("models integration prepares")
    .prepare_catalogue(XaiModelsProfileInput::new(
        RequestId::new("xai.projection.models").unwrap(),
    ))
    .expect("models catalogue prepares");
    prepared
        .consumer_route_projection_contribution(source("xai.projection.models"))
        .expect("models contributes")
}

fn run() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::OneResponse);
    let prepared =
        prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
            .expect("integration prepares")
            .prepare_responses_run(
                XaiRunProfileInput::new(
                    RequestId::new("xai.projection.run").unwrap(),
                    qualified_model("grok-4.6"),
                    OperationContent::new("maximal projection run").unwrap(),
                    None,
                )
                .with_reasoning_mode(ReasoningMode::new("xhigh").unwrap())
                .with_maximum_output_tokens(NonZeroU64::new(512).unwrap()),
            )
            .expect("run prepares");
    prepared
        .consumer_route_projection_contribution(source("xai.projection.run"))
        .expect("run contributes")
}

fn session() -> ConsumerRouteProjectionContribution {
    let fixture = DriverFixture::new(ServerScenario::OneResponse);
    let prepared =
        prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
            .expect("integration prepares")
            .prepare_responses_session(
                XaiSessionProfileInput::new(
                    RequestId::new("xai.projection.session").unwrap(),
                    qualified_model("grok-4.6"),
                    None,
                )
                .with_reasoning_mode(ReasoningMode::new("xhigh").unwrap()),
            )
            .expect("session prepares");
    prepared
        .consumer_route_projection_contribution(source("xai.projection.session"))
        .expect("session contributes")
}
