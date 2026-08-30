use crate::claude_code_events::ClaudeCodeEventParser;
use swallowtail_core::{SafeDiagnostic, WatcherCleanupCause};
use swallowtail_runtime::{
    CleanupOutcome, RuntimeEventSender, RuntimeFailure, RuntimeTurnId, TerminalOutcome,
    TerminalStatus, WatcherActivityProjection, WatcherLifecycleSubscription, WatcherSnapshot,
    project_watcher_activity,
};

use super::PumpHost;

pub(super) async fn finish_with_watchers(
    mut outcome: TerminalOutcome,
    parser: &mut ClaudeCodeEventParser,
    events: &RuntimeEventSender,
    mut host: PumpHost,
) -> TerminalOutcome {
    let Some(mut binding) = host.watcher_binding.take() else {
        return outcome;
    };
    let drain_error = drain_lifecycle(
        parser,
        events,
        host.watcher_feed.as_mut(),
        host.watcher_turn.as_ref(),
    )
    .err();
    if matches!(outcome.status(), TerminalStatus::Completed) {
        match binding.completion_gate() {
            Ok(state) if state.allows_successful_completion() => {}
            Ok(_) => {
                outcome = TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                        "swallowtail.claude_code.headless.watcher_completion_blocked",
                        "Claude Code headless cannot complete while host watchers remain active or unjoined",
                    )),
                    outcome.cleanup().clone(),
                );
            }
            Err(error) => {
                outcome = TerminalOutcome::new(
                    TerminalStatus::HostFailed(error.diagnostic().clone()),
                    outcome.cleanup().clone(),
                );
            }
        }
    }
    let watcher_cleanup = binding.close(cleanup_cause(outcome.status()));
    let outcome = replace_cleanup(outcome, watcher_cleanup);
    let later = drain_lifecycle(
        parser,
        events,
        host.watcher_feed.as_mut(),
        host.watcher_turn.as_ref(),
    );
    if let Some(error) = drain_error {
        return TerminalOutcome::new(
            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
            outcome.cleanup().clone(),
        );
    }
    if let Err(error) = later {
        return TerminalOutcome::new(
            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
            outcome.cleanup().clone(),
        );
    }
    outcome
}

pub(super) fn drain_lifecycle(
    parser: &mut ClaudeCodeEventParser,
    events: &RuntimeEventSender,
    feed: Option<&mut WatcherLifecycleSubscription>,
    turn: Option<&RuntimeTurnId>,
) -> Result<(), RuntimeFailure> {
    let Some(feed) = feed else {
        return Ok(());
    };
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match feed.poll_snapshot(&mut context) {
            std::task::Poll::Ready(Some(Ok(snapshot))) => {
                emit_lifecycle(parser, events, turn, snapshot)?;
            }
            std::task::Poll::Ready(Some(Err(error))) => return Err(error),
            std::task::Poll::Ready(None) | std::task::Poll::Pending => return Ok(()),
        }
    }
}

pub(super) fn emit_lifecycle(
    parser: &mut ClaudeCodeEventParser,
    events: &RuntimeEventSender,
    turn: Option<&RuntimeTurnId>,
    snapshot: WatcherSnapshot,
) -> Result<(), RuntimeFailure> {
    let Some(turn) = turn else {
        return Ok(());
    };
    match project_watcher_activity(turn, &snapshot) {
        Ok(WatcherActivityProjection::Activity(observation)) => {
            events.send(parser.activity_event(*observation))
        }
        Ok(WatcherActivityProjection::Joined { .. }) => Ok(()),
        Err(error) => Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.claude_code.headless.watcher_activity_projection_failed",
            error.to_string(),
        ))),
    }
}

fn cleanup_cause(status: &TerminalStatus) -> WatcherCleanupCause {
    match status {
        TerminalStatus::Cancelled => WatcherCleanupCause::Cancelled,
        TerminalStatus::TimedOut => WatcherCleanupCause::TimedOut,
        TerminalStatus::Completed => WatcherCleanupCause::Stopped,
        _ => WatcherCleanupCause::Failed,
    }
}

fn replace_cleanup(outcome: TerminalOutcome, watcher_cleanup: CleanupOutcome) -> TerminalOutcome {
    let cleanup = merge_cleanup(outcome.cleanup().clone(), watcher_cleanup);
    let rebuilt = TerminalOutcome::new(outcome.status().clone(), cleanup);
    match outcome.output().cloned() {
        Some(output) => rebuilt.with_output(output),
        None => rebuilt,
    }
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(diagnostic), _) | (_, CleanupOutcome::Failed(diagnostic)) => {
            CleanupOutcome::Failed(diagnostic)
        }
        (CleanupOutcome::Degraded(diagnostic), _) | (_, CleanupOutcome::Degraded(diagnostic)) => {
            CleanupOutcome::Degraded(diagnostic)
        }
        (CleanupOutcome::Clean, _) | (_, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}
