use super::*;
use crate::{Deadline, DirectToolCallId, OperationContent, RuntimeTurnId};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    DirectAttemptTransport, DirectContinuationConfig, DirectToolSelection,
    ProviderInferenceCachePolicy,
};

fn config() -> DirectContinuationConfig {
    DirectContinuationConfig::new(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(3).unwrap(),
        NonZeroU32::new(8).unwrap(),
        NonZeroU32::new(1).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(262_144).unwrap(),
        NonZeroU64::new(1_048_576).unwrap(),
        NonZeroU32::new(4_096).unwrap(),
        NonZeroU64::new(8_192).unwrap(),
        DirectAttemptTransport::Buffered,
        DirectAttemptTransport::ServerSentEvents,
        DirectToolSelection::ProviderAutomatic,
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}

fn turn(id: &str) -> DirectContinuationTurnRequest {
    DirectContinuationTurnRequest::new(
        RuntimeTurnId::new(id).unwrap(),
        OperationContent::new("private prompt").unwrap(),
        Deadline::at(crate::MonotonicInstant::from_ticks(10)),
    )
}

#[test]
fn each_attempt_requires_a_user_turn_or_exact_tool_result_action() {
    let mut state = DirectContinuationState::new(config());
    let attempt = state.authorize_user_turn(&turn("turn-1")).unwrap();
    assert_eq!(attempt.ordinal().get(), 1);
    assert_eq!(attempt.transport(), DirectAttemptTransport::Buffered);

    let call = DirectToolCall::new(
        DirectToolCallId::new("call-1").unwrap(),
        attempt.attempt_id().clone(),
        "lookup",
        DirectToolArguments::new(br#"{"id":1}"#.to_vec(), 64).unwrap(),
    )
    .unwrap();
    state
        .pause_for_tool_calls(&attempt, std::slice::from_ref(&call))
        .unwrap();
    assert_eq!(state.pending_tool_calls(), 1);
    assert!(state.authorize_tool_results(&[]).is_err());

    let result = DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(b"private result".to_vec(), 64).unwrap(),
    );
    let continued = state.authorize_tool_results(&[result]).unwrap();
    assert_eq!(continued.ordinal().get(), 2);
    assert_eq!(
        continued.authorization(),
        DirectAttemptAuthorizationKind::CorrelatedToolResults
    );
    assert_eq!(
        continued.transport(),
        DirectAttemptTransport::ServerSentEvents
    );
    state.complete_turn().unwrap();

    let final_attempt = state.authorize_user_turn(&turn("turn-2")).unwrap();
    assert_eq!(final_attempt.ordinal().get(), 3);
    state.complete_turn().unwrap();
    assert!(state.authorize_user_turn(&turn("turn-3")).is_err());
}

#[test]
fn direct_tool_material_and_private_metadata_are_redacted_and_bounded() {
    let arguments = DirectToolArguments::new(b"private arguments".to_vec(), 64).unwrap();
    let result = DirectToolResultContent::new(b"private result".to_vec(), 64).unwrap();
    assert!(!format!("{arguments:?}").contains("private arguments"));
    assert!(!format!("{result:?}").contains("private result"));
    assert!(DirectToolArguments::new(vec![0; 65], 64).is_err());
    assert!(DirectToolResultContent::new(vec![0; 65], 64).is_err());
}
