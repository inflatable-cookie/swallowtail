#[allow(dead_code)]
#[path = "driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_MODEL_ID, DeepSeekCatalogueProfileInput, DeepSeekModelSelection,
    DeepSeekRunProfileInput, DeepSeekSessionProfileInput, prepare_deepseek_direct,
};
use swallowtail_core::{
    ModelId, ModelRouteId, ModelRouteRevision, ProviderInferenceCachePolicy, ReasoningMode,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionSourceId,
    ConsumerRouteRowIdentity, OperationContent, RequestId, SchemaDocument, ToolDeclaration,
};

use std::collections::{BTreeMap, BTreeSet};
use swallowtail_testkit::{
    ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade,
};

#[test]
fn candidate_i_projection_ledger_is_exact_and_provider_free() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let catalogue = prepared
        .prepare_catalogue(DeepSeekCatalogueProfileInput::new(
            RequestId::new("projection-catalogue").expect("request id"),
        ))
        .expect("catalogue prepares")
        .consumer_route_projection_contribution(source("deepseek.projection.catalogue"))
        .expect("catalogue contribution");
    assert_rows(
        &catalogue,
        [
            feature(ConsumerRouteFeatureId::ModelCatalogue),
            feature(ConsumerRouteFeatureId::PreparedFacade),
        ],
    );

    let run = prepared
        .prepare_run(DeepSeekRunProfileInput::new(
            RequestId::new("projection-run").expect("request id"),
            model(),
            OperationContent::new("projection only").expect("content"),
            ReasoningMode::new("high").expect("reasoning"),
            std::num::NonZeroU64::new(512).expect("maximum"),
            ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
        ))
        .expect("run prepares")
        .consumer_route_projection_contribution(source("deepseek.projection.run"))
        .expect("run contribution");
    assert_rows(
        &run,
        [
            feature(ConsumerRouteFeatureId::StructuredRun),
            feature(ConsumerRouteFeatureId::StreamingEvents),
            feature(ConsumerRouteFeatureId::UsageEvidence),
            feature(ConsumerRouteFeatureId::OutputTokenLimit),
            feature(ConsumerRouteFeatureId::ReasoningSelection),
            feature(ConsumerRouteFeatureId::CancellationOrInterruption),
            feature(ConsumerRouteFeatureId::PreparedFacade),
            feature(ConsumerRouteFeatureId::ActivityObservation),
            control(ConsumerRouteControlId::ModelSelection),
            control(ConsumerRouteControlId::ReasoningSelection),
            control(ConsumerRouteControlId::MaximumOutputTokens),
            namespaced("control.inference-cache-policy"),
        ],
    );

    let session = prepared
        .prepare_session(DeepSeekSessionProfileInput::new(
            RequestId::new("projection-session").expect("request id"),
            model(),
            ReasoningMode::new("high").expect("reasoning"),
            [tool()],
            ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
        ))
        .expect("session prepares")
        .consumer_route_projection_contribution(source("deepseek.projection.session"))
        .expect("session contribution");
    assert_rows(
        &session,
        [
            feature(ConsumerRouteFeatureId::InteractiveSession),
            feature(ConsumerRouteFeatureId::StreamingEvents),
            feature(ConsumerRouteFeatureId::UsageEvidence),
            feature(ConsumerRouteFeatureId::OutputTokenLimit),
            feature(ConsumerRouteFeatureId::ReasoningSelection),
            feature(ConsumerRouteFeatureId::ConsumerToolExchange),
            feature(ConsumerRouteFeatureId::CancellationOrInterruption),
            feature(ConsumerRouteFeatureId::PreparedFacade),
            feature(ConsumerRouteFeatureId::ActivityObservation),
            control(ConsumerRouteControlId::ModelSelection),
            control(ConsumerRouteControlId::ReasoningSelection),
            namespaced("control.inference-cache-policy"),
            control(ConsumerRouteControlId::ToolDeclarations),
        ],
    );

    let emitted = rows(&catalogue)
        .chain(rows(&run))
        .chain(rows(&session))
        .collect::<BTreeSet<_>>();
    assert_eq!(emitted.len(), 16);
    for withheld in [
        feature(ConsumerRouteFeatureId::PersistentSessionPosture),
        feature(ConsumerRouteFeatureId::ProviderSessionCatalogue),
        feature(ConsumerRouteFeatureId::ProviderSessionHistory),
    ] {
        assert!(!emitted.contains(&withheld));
    }
    // Model, reasoning, and cache controls each occupy both run and session
    // census tuples. The actual contributions above therefore prove 16 unique
    // identities and exactly 19 operation-scoped ledger rows.
    assert_eq!(emitted.len() + 3, 19);
    const ROUTE: &str = "deepseek.continuation";
    const CATALOGUE: &str = "catalogue";
    const RUN: &str = "run";
    const SESSION: &str = "session";
    const LEDGER: [(&str, &str, &[&str]); 22] = [
        ("model-catalogue", "feature.model-catalogue", &[CATALOGUE]),
        (
            "route-capability",
            "feature.prepared-facade",
            &[CATALOGUE, RUN, SESSION],
        ),
        ("structured-run", "feature.structured-run", &[RUN]),
        (
            "interactive-session",
            "feature.interactive-session",
            &[SESSION],
        ),
        (
            "route-observation",
            "feature.streaming-events",
            &[RUN, SESSION],
        ),
        (
            "route-observation",
            "feature.usage-evidence",
            &[RUN, SESSION],
        ),
        (
            "route-capability",
            "feature.output-token-limit",
            &[RUN, SESSION],
        ),
        (
            "route-capability",
            "feature.reasoning-selection",
            &[RUN, SESSION],
        ),
        (
            "route-capability",
            "feature.consumer-tool-exchange",
            &[SESSION],
        ),
        (
            "route-capability",
            "feature.cancellation-or-interruption",
            &[RUN, SESSION],
        ),
        (
            "route-observation",
            "feature.activity-observation",
            &[RUN, SESSION],
        ),
        ("structured-run", "control.model-selection", &[RUN]),
        ("interactive-session", "control.model-selection", &[SESSION]),
        ("structured-run", "control.reasoning-selection", &[RUN]),
        (
            "interactive-session",
            "control.reasoning-selection",
            &[SESSION],
        ),
        ("structured-run", "control.maximum-output-tokens", &[RUN]),
        ("structured-run", "control.inference-cache-policy", &[RUN]),
        (
            "interactive-session",
            "control.inference-cache-policy",
            &[SESSION],
        ),
        (
            "interactive-session",
            "control.tool-declarations",
            &[SESSION],
        ),
        (
            "session-lifecycle",
            "feature.persistent-session-posture",
            &[],
        ),
        (
            "session-lifecycle",
            "feature.provider-session-catalogue",
            &[],
        ),
        ("session-lifecycle", "feature.provider-session-history", &[]),
    ];
    let observed = BTreeMap::from([
        (CATALOGUE, observed_tuples(ROUTE, &catalogue)),
        (RUN, observed_tuples(ROUTE, &run)),
        (SESSION, observed_tuples(ROUTE, &session)),
    ]);
    assert_consumer_route_ledger_emitted_by_facade(
        &observed,
        LEDGER
            .iter()
            .map(|(shape, semantic, emitted_by)| ConsumerRouteLedgerClaim {
                identity: (ROUTE, *shape, *semantic),
                emitted_by,
            }),
    );
    assert!(fixture.server.requests().is_empty());
}

fn observed_tuples(
    route: &'static str,
    contribution: &swallowtail_runtime::ConsumerRouteProjectionContribution,
) -> BTreeSet<(&'static str, &'static str, &'static str)> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| {
            let (shape, semantic) = match row.identity() {
                ConsumerRouteRowIdentity::Feature(feature) => match feature {
                    ConsumerRouteFeatureId::ModelCatalogue => {
                        ("model-catalogue", "feature.model-catalogue")
                    }
                    ConsumerRouteFeatureId::PreparedFacade => {
                        ("route-capability", "feature.prepared-facade")
                    }
                    ConsumerRouteFeatureId::StructuredRun => {
                        ("structured-run", "feature.structured-run")
                    }
                    ConsumerRouteFeatureId::InteractiveSession => {
                        ("interactive-session", "feature.interactive-session")
                    }
                    ConsumerRouteFeatureId::StreamingEvents => {
                        ("route-observation", "feature.streaming-events")
                    }
                    ConsumerRouteFeatureId::UsageEvidence => {
                        ("route-observation", "feature.usage-evidence")
                    }
                    ConsumerRouteFeatureId::OutputTokenLimit => {
                        ("route-capability", "feature.output-token-limit")
                    }
                    ConsumerRouteFeatureId::ReasoningSelection => {
                        ("route-capability", "feature.reasoning-selection")
                    }
                    ConsumerRouteFeatureId::ConsumerToolExchange => {
                        ("route-capability", "feature.consumer-tool-exchange")
                    }
                    ConsumerRouteFeatureId::CancellationOrInterruption => {
                        ("route-capability", "feature.cancellation-or-interruption")
                    }
                    ConsumerRouteFeatureId::ActivityObservation => {
                        ("route-observation", "feature.activity-observation")
                    }
                    other => panic!("unexpected DeepSeek feature {other:?}"),
                },
                ConsumerRouteRowIdentity::Control(control) => {
                    let shape = match contribution.applicability().operation_shape() {
                        swallowtail_core::OperationShape::StructuredRun => "structured-run",
                        swallowtail_core::OperationShape::InteractiveSession => {
                            "interactive-session"
                        }
                        other => panic!("unexpected DeepSeek control shape {other:?}"),
                    };
                    let semantic = match control {
                        ConsumerRouteControlId::ModelSelection => "control.model-selection",
                        ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
                        ConsumerRouteControlId::MaximumOutputTokens => {
                            "control.maximum-output-tokens"
                        }
                        ConsumerRouteControlId::ToolDeclarations => "control.tool-declarations",
                        ConsumerRouteControlId::Namespaced(extension)
                            if extension.semantic_id() == "control.inference-cache-policy" =>
                        {
                            "control.inference-cache-policy"
                        }
                        other => panic!("unexpected DeepSeek control {other:?}"),
                    };
                    (shape, semantic)
                }
            };
            (route, shape, semantic)
        })
        .collect()
}

fn assert_rows(
    contribution: &swallowtail_runtime::ConsumerRouteProjectionContribution,
    expected: impl IntoIterator<Item = ConsumerRouteRowIdentity>,
) {
    assert_eq!(
        rows(contribution).collect::<BTreeSet<_>>(),
        expected.into_iter().collect()
    );
}

fn rows(
    contribution: &swallowtail_runtime::ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = ConsumerRouteRowIdentity> + '_ {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| row.identity().clone())
}

fn feature(id: ConsumerRouteFeatureId) -> ConsumerRouteRowIdentity {
    ConsumerRouteRowIdentity::Feature(id)
}

fn control(id: ConsumerRouteControlId) -> ConsumerRouteRowIdentity {
    ConsumerRouteRowIdentity::Control(id)
}

fn namespaced(semantic: &str) -> ConsumerRouteRowIdentity {
    control(ConsumerRouteControlId::Namespaced(
        swallowtail_runtime::ConsumerRouteNamespacedExtension::new(
            "deepseek.continuation",
            swallowtail_adapter_deepseek::DEEPSEEK_FACADE_REVISION,
            semantic,
        )
        .expect("namespaced identity"),
    ))
}

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("source id")
}

fn model() -> DeepSeekModelSelection {
    DeepSeekModelSelection::new(
        ModelRouteId::new("deepseek.prepared.v4-pro").expect("route id"),
        ModelRouteRevision::new("2026-07-22").expect("route revision"),
        ModelId::new(DEEPSEEK_MODEL_ID).expect("model id"),
    )
}

fn tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_weather",
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1_024).expect("schema"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool")
}
