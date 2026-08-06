use crate::{ProviderConversationPreflightCase, ProviderConversationPreflightFixture};
use swallowtail_core::{OwnedRemoteResourceKind, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, ProviderCancellationOutcome, RemoteResourceDeletionOutcome,
    SessionResumeBinding, TerminalOutcome, TerminalStatus, WorkingResourceRef,
    validate_session_plan_agreement, validate_session_provider_state_plan,
};

/// Runs common ephemeral provider-conversation boundary assertions.
pub fn run_provider_conversation_boundary_assertions() {
    let fixture = ProviderConversationPreflightFixture::for_case(
        ProviderConversationPreflightCase::Canonical,
    );
    let plan = fixture
        .preflight()
        .expect("conversation preflight succeeds");
    let request = fixture.open_request();
    assert!(request.working_resource().is_none());
    assert_eq!(
        request.provider_state_policy(),
        Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose)
    );
    validate_session_plan_agreement(&plan, request.plan_agreement())
        .expect("request policy matches preflight");
    assert!(
        validate_session_provider_state_plan(&plan, SessionProviderStatePolicy::Prohibited)
            .is_err()
    );

    let mut turns = SerialTurnGate::default();
    turns.start().expect("first turn starts");
    assert!(turns.start().is_err());
    turns.complete();
    turns.start().expect("second turn starts");
    turns.complete();
    assert!(turns.start().is_err());

    let success = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::ConversationItems,
            RemoteResourceDeletionOutcome::Confirmed,
        )
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Conversation,
            RemoteResourceDeletionOutcome::Confirmed,
        );
    assert_eq!(
        success
            .remote_resource_deletions()
            .collect::<Vec<_>>()
            .len(),
        2
    );

    let raced = TerminalOutcome::new(
        TerminalStatus::Cancelled,
        CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
            "fixture.remote_turn_race",
            "Remote turn state remained uncertain during cleanup",
        )),
    )
    .with_provider_cancellation(ProviderCancellationOutcome::Unconfirmed)
    .with_remote_resource_deletion(
        OwnedRemoteResourceKind::ConversationItems,
        RemoteResourceDeletionOutcome::Unconfirmed,
    )
    .with_remote_resource_deletion(
        OwnedRemoteResourceKind::Conversation,
        RemoteResourceDeletionOutcome::Unconfirmed,
    );
    assert_eq!(
        raced.provider_cancellation(),
        Some(ProviderCancellationOutcome::Unconfirmed)
    );
    assert_eq!(raced.status(), &TerminalStatus::Cancelled);
}

/// Runs common retained provider-conversation lifecycle assertions.
pub fn run_retained_provider_conversation_boundary_assertions() {
    let fixture = ProviderConversationPreflightFixture::for_case(
        ProviderConversationPreflightCase::CanonicalRetained,
    );
    let plan = fixture
        .preflight()
        .expect("retained conversation preflight succeeds");
    let open = fixture.open_request();
    assert!(open.working_resource().is_none());
    assert_eq!(
        open.provider_state_policy(),
        Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    );

    let binding = fixture.retained_binding();
    assert!(binding.is_resource_free());
    assert!(binding.matches_resource_free_attachment(&plan, open.access_policy()));
    let persisted = binding
        .export_persisted(&plan)
        .expect("exact retained binding persists");
    let restored = SessionResumeBinding::restore_persisted_resource_free(
        &persisted,
        &plan,
        open.access_policy(),
    )
    .expect("exact retained binding restores");
    assert_eq!(restored, binding);

    let resource = WorkingResourceRef::new("fixture.unrelated-resource")
        .expect("static working resource is valid");
    assert!(
        SessionResumeBinding::restore_persisted(
            &persisted,
            &plan,
            &resource,
            open.access_policy(),
        )
        .is_err()
    );

    let load = fixture.retained_load_request();
    assert!(load.working_resource().is_none());
    assert_eq!(load.resume_binding(), &binding);
    validate_session_plan_agreement(&plan, load.plan_agreement())
        .expect("retained load agreement matches preflight");
    assert!(
        validate_session_provider_state_plan(
            &plan,
            SessionProviderStatePolicy::DurableConversationDeleteOnClose,
        )
        .is_err()
    );
}

#[derive(Default)]
struct SerialTurnGate {
    active: bool,
    completed: u8,
}

impl SerialTurnGate {
    fn start(&mut self) -> Result<(), ()> {
        if self.active || self.completed >= 2 {
            Err(())
        } else {
            self.active = true;
            Ok(())
        }
    }

    fn complete(&mut self) {
        assert!(self.active);
        self.active = false;
        self.completed += 1;
    }
}
