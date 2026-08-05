use super::fixture::PreparedFixture;
use futures_executor::block_on;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_opencode::{
    OpenCodeSessionProfileInput, OpenCodeSessionReconciliationInput,
};
use swallowtail_runtime::{
    ProviderSessionReconciliationBounds, RequestId, RuntimeTurnId, SessionResumeBinding,
    SettledSessionAttachment, SettledSessionAttachmentKind, SettledSessionRestorationOutcome,
};

#[test]
fn reconciliation_loads_only_after_inactive_observation() {
    let fixture = PreparedFixture::new("opencode.reconciliation.then.load", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("settled-session-plan").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let plan = session.plan();
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        session.request().access_policy().clone(),
    );
    let reconciliation = prepared
        .prepare_session_reconciliation(OpenCodeSessionReconciliationInput::new(
            RequestId::new("settled-reconciliation").unwrap(),
            fixture.model(),
            binding,
            RuntimeTurnId::new("settled-interrupted-turn").unwrap(),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(8).unwrap(),
                NonZeroU64::new(4096).unwrap(),
            ),
        ))
        .expect("reconciliation prepares");
    let restoration = reconciliation
        .prepare_settled_session_restoration(session, RequestId::new("settled-load").unwrap())
        .expect("settled restoration prepares");
    assert_eq!(
        restoration.attachment_kind(),
        SettledSessionAttachmentKind::Load
    );

    let restored = block_on(restoration.restore(fixture.services()))
        .expect("inactive session reconciles then loads");
    let SettledSessionRestorationOutcome::Attached(attached) = restored else {
        panic!("inactive OpenCode session must attach");
    };
    assert_eq!(
        attached.reconciliation().state(),
        swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
    );
    let SettledSessionAttachment::Loaded(loaded) = attached.attachment() else {
        panic!("OpenCode settled attachment must preserve load truth");
    };
    assert_eq!(loaded.replay().count(), 4);
}

#[test]
fn preparation_rejects_a_foreign_attachment_plan() {
    let source = PreparedFixture::new("opencode.settled.source", "1.18.10");
    let source_prepared = source.prepared();
    let source_session = source_prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("settled-source-session").unwrap(),
            source.model(),
            source.resource.clone(),
        ))
        .expect("source session prepares");
    let plan = source_session.plan();
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        source.resource.clone(),
        source_session.request().access_policy().clone(),
    );
    let reconciliation = source_prepared
        .prepare_session_reconciliation(OpenCodeSessionReconciliationInput::new(
            RequestId::new("settled-source-reconciliation").unwrap(),
            source.model(),
            binding,
            RuntimeTurnId::new("settled-source-turn").unwrap(),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(8).unwrap(),
                NonZeroU64::new(4096).unwrap(),
            ),
        ))
        .expect("source reconciliation prepares");

    let foreign = PreparedFixture::new("opencode.settled.foreign", "1.18.10");
    let foreign_session = foreign
        .prepared()
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("settled-foreign-session").unwrap(),
            foreign.model(),
            foreign.resource.clone(),
        ))
        .expect("foreign session prepares");
    let failure = match reconciliation.prepare_settled_session_restoration(
        foreign_session,
        RequestId::new("settled-foreign-load").unwrap(),
    ) {
        Ok(_) => panic!("cross-host attachment must reject before provider work"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.settled_session_binding_mismatch"
    );
}
