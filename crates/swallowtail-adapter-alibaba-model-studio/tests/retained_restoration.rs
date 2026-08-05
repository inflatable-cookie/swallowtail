mod support;

use futures_executor::block_on;
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaRetainedConversationProfileInput, EXACT_MODEL_ID, MODEL_ROUTE_ID,
    prepare_alibaba_model_studio,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};

#[test]
fn retained_conversation_maps_to_common_continuation_recovery() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let retained = prepared
        .prepare_retained_conversation(AlibabaRetainedConversationProfileInput::new(
            RequestId::new("restoration-source").expect("request id"),
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
            ModelRouteRevision::new("2026-08-05").expect("route revision"),
            ModelId::new(EXACT_MODEL_ID).expect("model id"),
        ))
        .expect("retained conversation prepares");
    let session = block_on(retained.open_session(fixture.services())).expect("session opens");
    let binding = session.resume_binding().expect("resume binding").clone();
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let interrupted_turn_id = RuntimeTurnId::new("interrupted-retained-turn").expect("turn id");
    let restoration = retained
        .prepare_working_state_restoration(
            RequestId::new("restore-retained").expect("request id"),
            binding,
            interrupted_turn_id.clone(),
        )
        .expect("restoration prepares");
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::ProviderSessionContinuationRecovery
    );

    let outcome = block_on(restoration.restore(fixture.services())).expect("restoration succeeds");
    assert_eq!(
        outcome.method(),
        WorkingStateRestorationMethod::ProviderSessionContinuationRecovery
    );
    let WorkingStateRestorationOutcome::SessionRecovered(recovered) = outcome else {
        panic!("retained conversation must recover a loaded session");
    };
    assert_eq!(recovered.interrupted_turn_id(), &interrupted_turn_id);
    assert_eq!(recovered.replay().len(), 4);
    let (_, loaded) = recovered.into_parts();
    let (_, session) = loaded.into_parts();
    assert!(session.management_binding().is_some());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(
        fixture
            .requests()
            .iter()
            .all(|request| request.method != "DELETE")
    );
}
