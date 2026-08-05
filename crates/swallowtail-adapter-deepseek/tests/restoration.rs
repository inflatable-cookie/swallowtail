#[allow(dead_code)]
#[path = "driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_MODEL_ID, DeepSeekModelSelection, DeepSeekSessionProfileInput, prepare_deepseek_direct,
};
use swallowtail_core::{
    ModelId, ModelRouteId, ModelRouteRevision, ProviderInferenceCachePolicy, ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, SchemaDocument, ToolDeclaration,
    WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};

#[test]
fn direct_session_restoration_opens_a_fresh_session_with_explicit_context_loss() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let session = prepared
        .prepare_session(session_input("restoration"))
        .expect("session prepares");
    let interrupted = RuntimeTurnId::new("deepseek-interrupted").expect("turn id");
    let restoration = session.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
        panic!("fresh session replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let (_, replacement) = replacement.into_parts();
    assert!(replacement.provider_session_ref().is_none());
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.server.attempts(), 0);
}

fn session_input(id: &str) -> DeepSeekSessionProfileInput {
    DeepSeekSessionProfileInput::new(
        RequestId::new(id).expect("request id"),
        DeepSeekModelSelection::new(
            ModelRouteId::new("deepseek.prepared.v4-pro").expect("route id"),
            ModelRouteRevision::new("2026-07-22").expect("route revision"),
            ModelId::new(DEEPSEEK_MODEL_ID).expect("model id"),
        ),
        ReasoningMode::new("high").expect("reasoning"),
        [ToolDeclaration::new(
            "lookup_weather",
            SchemaDocument::inline(
                br#"{"type":"object","properties":{"city":{"type":"string"}},"required":["city"],"additionalProperties":false}"#.to_vec(),
                1_024,
            )
            .expect("schema"),
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("tool")],
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}
