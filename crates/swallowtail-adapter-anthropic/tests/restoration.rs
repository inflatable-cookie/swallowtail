mod support;

#[allow(dead_code)]
#[path = "prepared_facade/fixtures.rs"]
mod fixtures;

use fixtures::PreparedFixture;
use futures_executor::block_on;
use swallowtail_adapter_anthropic::{AnthropicModelSelection, AnthropicSessionProfileInput};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, SchemaDocument, ToolDeclaration,
    WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};

#[test]
fn direct_session_restoration_opens_a_fresh_session_with_explicit_context_loss() {
    let fixture = PreparedFixture::new(ExecutionHostId::new("anthropic.restoration").unwrap());
    let session = fixture
        .prepared()
        .prepare_session(AnthropicSessionProfileInput::new(
            RequestId::new("anthropic-restoration").unwrap(),
            model("claude-fixture-primary"),
            [fixture_tool()],
        ))
        .expect("direct session prepares");
    let interrupted = RuntimeTurnId::new("anthropic-interrupted").unwrap();
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
    assert_eq!(fixture.server.inference_attempts(), 0);
}

fn model(id: &str) -> AnthropicModelSelection {
    AnthropicModelSelection::new(
        ModelRouteId::new(format!("anthropic.{id}")).unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new(id).unwrap(),
    )
}

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_customer",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"customer_id":{"type":"string"}},"required":["customer_id"],"additionalProperties":false}"#.to_vec(),
            4096,
        )
        .unwrap(),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap()
}
