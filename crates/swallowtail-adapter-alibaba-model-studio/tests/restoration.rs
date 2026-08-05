mod support;

use futures_executor::block_on;
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaConversationProfileInput, EXACT_MODEL_ID, MODEL_ROUTE_ID, prepare_alibaba_model_studio,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};

#[test]
fn delete_on_close_conversation_restoration_opens_a_fresh_replacement() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("Alibaba integration prepares");
    let conversation = prepared
        .prepare_conversation(profile_input("replacement"))
        .expect("conversation prepares");
    let interrupted = RuntimeTurnId::new("alibaba-interrupted").expect("turn id");
    let restoration = conversation.prepare_working_state_restoration(interrupted.clone());
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
    let requests = fixture.requests();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].target, "/compatible-mode/v1/conversations");
    assert_eq!(requests[1].method, "GET");
    assert!(
        requests[2..6]
            .iter()
            .all(|request| request.method == "DELETE" && request.target.contains("/items/"))
    );
    assert_eq!(requests[6].method, "DELETE");
    assert_eq!(
        requests[6].target,
        "/compatible-mode/v1/conversations/conv_fixture_01"
    );
}

fn profile_input(id: &str) -> AlibabaConversationProfileInput {
    AlibabaConversationProfileInput::new(
        RequestId::new(id).expect("request id"),
        ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
        ModelRouteRevision::new("2026-07-22").expect("route revision"),
        ModelId::new(EXACT_MODEL_ID).expect("model id"),
        SessionProviderStatePolicy::DurableConversationDeleteOnClose,
    )
}
