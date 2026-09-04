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

use std::collections::BTreeSet;

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
    assert!(fixture.server.requests().is_empty());
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
