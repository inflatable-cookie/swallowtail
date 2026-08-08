//! Shared run-loop emit and outcome helpers.
//!
//! The same small helpers recur verbatim in adapter pumps: emit one ordered
//! runtime event with the caller's sequence counter, project an activity
//! observation, and convert provider failures and cleanup results into their
//! portable terminal shapes. Adapter-specific event translation and content
//! validation stay adapter-local.

#![deny(missing_docs)]

use crate::{
    ActivityObservation, CleanupOutcome, OperationContent, RuntimeEvent, RuntimeEventKind,
    RuntimeEventSender, RuntimeFailure, TerminalStatus,
};

/// Emits one ordered event and advances the caller's sequence.
pub fn emit(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

/// Emits one ordered event carrying already-validated content.
///
/// Content validation stays adapter-local so each route keeps its exact
/// empty-content diagnostic; this helper only transports the validated value.
pub fn emit_content(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: OperationContent,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}

/// Emits one activity observation and advances the caller's sequence.
pub fn emit_activity(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    observation: Result<ActivityObservation, RuntimeFailure>,
) -> Result<(), RuntimeFailure> {
    emit(events, sequence, RuntimeEventKind::Activity(observation?))
}

/// Projects one provider runtime failure to its terminal status.
#[must_use]
pub fn provider_status(error: RuntimeFailure) -> TerminalStatus {
    TerminalStatus::ProviderFailed(error.diagnostic().clone())
}

/// Converts one cleanup attempt to its portable outcome.
#[must_use]
pub fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::{cleanup_result, emit, emit_activity, emit_content, provider_status};
    use crate::{
        ActivityKind, ActivityObservation, CleanupOutcome, OperationContent, RuntimeEvent,
        RuntimeEventKind, RuntimeEventSender, RuntimeFailure, TerminalStatus,
        runtime_event_channel,
    };
    use futures_core::Stream;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};
    use swallowtail_core::SafeDiagnostic;

    #[test]
    fn emit_appends_and_increments_sequence() {
        let (sender, mut stream) = runtime_event_channel(4).expect("capacity is valid");
        let mut sequence = 0;
        emit(&sender, &mut sequence, RuntimeEventKind::Started).expect("emit succeeds");
        emit(&sender, &mut sequence, RuntimeEventKind::OutputAvailable).expect("emit succeeds");
        assert_eq!(sequence, 2);
        let mut context = Context::from_waker(Waker::noop());
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event))) if event.sequence() == 0
        ));
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event))) if event.sequence() == 1
        ));
    }

    #[test]
    fn emit_content_carries_validated_content() {
        let (sender, mut stream) = runtime_event_channel(2).expect("capacity is valid");
        let mut sequence = 0;
        emit(&sender, &mut sequence, RuntimeEventKind::Started).expect("start emits");
        emit_content(
            &sender,
            &mut sequence,
            RuntimeEventKind::OutputDelta,
            OperationContent::new("delta").expect("content is valid"),
        )
        .expect("emit succeeds");
        let mut context = Context::from_waker(Waker::noop());
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event)))
                if event.sequence() == 0
                    && event.kind() == &RuntimeEventKind::Started
        ));
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event)))
                if event.sequence() == 1
                    && event.kind() == &RuntimeEventKind::OutputDelta
        ));
    }

    #[test]
    fn provider_failure_and_cleanup_map_to_terminal_shapes() {
        let error = RuntimeFailure::new(SafeDiagnostic::new("fixture.failure", "failed"));
        assert!(matches!(
            provider_status(error.clone()),
            TerminalStatus::ProviderFailed(diagnostic) if diagnostic.code() == "fixture.failure"
        ));
        assert_eq!(cleanup_result(Ok(())), CleanupOutcome::Clean);
        assert!(matches!(
            cleanup_result(Err(error)),
            CleanupOutcome::Failed(diagnostic) if diagnostic.code() == "fixture.failure"
        ));
    }

    #[test]
    fn emit_activity_forwards_observation_failures() {
        let (sender, _stream) = runtime_event_channel(2).expect("capacity is valid");
        let mut sequence = 0;
        let failure = RuntimeFailure::new(SafeDiagnostic::new("fixture.activity", "failed"));
        assert!(emit_activity(&sender, &mut sequence, Err(failure)).is_err());
        let _ = ActivityKind::AssistantMessage;
        let _ = ActivityObservation::new;
        let _ = RuntimeEvent::with_content;
        let _: Option<RuntimeEventSender> = None;
    }
}
