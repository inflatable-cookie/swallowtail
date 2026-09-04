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

#[test]
fn candidate_i_projection_ledger_is_exact_and_provider_free() {
    assert_eq!(19 + 22 + 6, 47);
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
    assert!(catalogue.selection_rows().any(|row| row.identity()
        == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ModelCatalogue)));

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
    assert!(run.selection_rows().any(|row| row.identity()
        == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection)));
    assert!(run.session_start_rows().any(|row| row.identity()
        == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::MaximumOutputTokens)));

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
    assert!(session.session_start_rows().any(|row| row.identity()
        == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ToolDeclarations)));
    assert!(fixture.server.requests().is_empty());
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
