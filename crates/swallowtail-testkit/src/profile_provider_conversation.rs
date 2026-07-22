use crate::{ProviderConversationPreflightCase, ProviderConversationPreflightFixture};
use swallowtail_core::{OwnedRemoteResourceKind, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, ProviderCancellationOutcome, RemoteResourceDeletionOutcome, TerminalOutcome,
    TerminalStatus, validate_session_provider_state_plan,
};

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
        SessionProviderStatePolicy::DurableConversationDeleteOnClose
    );
    validate_session_provider_state_plan(&plan, request.provider_state_policy())
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
