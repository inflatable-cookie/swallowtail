use crate::output::{OutputState, failure};
use crate::process_reader::ReaderControl;
use std::process::{Child, ExitStatus};
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_runtime::ProcessExit;

use super::{ExitState, terminate_process_tree};

/// Bound for joining the output reader threads after the child exits.
pub(crate) const READER_JOIN_BOUND: Duration = Duration::from_secs(2);
/// Grace period between a close-stdin request and process-tree termination.
pub(crate) const GRACEFUL_STOP_BOUND: Duration = Duration::from_secs(1);
/// Poll-interval for bounded reader joins.
const JOIN_POLL_INTERVAL: Duration = Duration::from_millis(10);
/// Consecutive failed kill attempts before a requested force stop is
/// reported as failed.
const MAX_FORCE_KILL_ATTEMPTS: u32 = 100;

/// Commands sent from a process handle to its supervisor.
pub(crate) enum ChildCommand {
    RequestStop,
    ForceStop,
}

/// Reader tasks and their shared cancellation control.
pub(crate) struct ReaderSupervision {
    pub(crate) stdout: thread::JoinHandle<()>,
    pub(crate) stderr: thread::JoinHandle<()>,
    pub(crate) control: Arc<ReaderControl>,
}

/// Outcome of joining one output reader thread under a bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum JoinOutcome {
    Completed,
    TimedOut,
    Panicked,
}

/// Joins one output reader thread, cancelling it if the bound elapses.
fn join_with_bound(
    handle: thread::JoinHandle<()>,
    control: &ReaderControl,
    bound: Duration,
) -> JoinOutcome {
    let deadline = Instant::now()
        .checked_add(bound)
        .expect("reader join deadline is representable");
    loop {
        if handle.is_finished() {
            return match handle.join() {
                Ok(()) => JoinOutcome::Completed,
                Err(_) => JoinOutcome::Panicked,
            };
        }
        let now = Instant::now();
        if now >= deadline {
            control.cancel();
            return match handle.join() {
                Ok(()) => JoinOutcome::TimedOut,
                Err(_) => JoinOutcome::Panicked,
            };
        }
        thread::sleep((deadline - now).min(JOIN_POLL_INTERVAL));
    }
}

pub(crate) fn supervise_child(
    child: &mut Child,
    group_owner: &mut Option<Child>,
    commands: Receiver<ChildCommand>,
    readers: ReaderSupervision,
    output: &OutputState,
    exit: &ExitState,
) {
    let ReaderSupervision {
        stdout: stdout_reader,
        stderr: stderr_reader,
        control: reader_control,
    } = readers;
    let root_process_id = child.id();
    let mut graceful_requested = false;
    let mut graceful_signal_sent = false;
    let mut graceful_deadline = None;
    let mut force_deadline = None;
    let mut force_seen = false;
    let mut force_signal_sent = false;
    let mut failed_kills = 0_u32;
    let mut tree_error = None;
    let mut graceful_tree_termination_succeeded = false;
    let status = loop {
        match commands.recv_timeout(Duration::from_millis(10)) {
            Ok(ChildCommand::RequestStop) => {
                if !graceful_requested {
                    graceful_requested = true;
                    graceful_deadline = Instant::now().checked_add(GRACEFUL_STOP_BOUND);
                }
            }
            Ok(ChildCommand::ForceStop) => force_seen = true,
            Err(_) => {}
        }
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => {}
            Err(_) => {
                break Err(failure(
                    "swallowtail.local_process.wait_failed",
                    "Local process exit could not be observed",
                ));
            }
        }
        if graceful_requested
            && !graceful_signal_sent
            && graceful_deadline.is_some_and(|deadline| Instant::now() >= deadline)
        {
            graceful_signal_sent = true;
            force_deadline = Instant::now().checked_add(GRACEFUL_STOP_BOUND);
            match terminate_process_tree(group_owner.as_ref(), root_process_id, false) {
                Ok(()) => graceful_tree_termination_succeeded = true,
                Err(error) => {
                    tree_error.get_or_insert(error);
                    force_seen = true;
                    let _ = child.kill();
                }
            }
        } else if graceful_signal_sent
            && !force_seen
            && force_deadline.is_some_and(|deadline| Instant::now() >= deadline)
        {
            force_seen = true;
        }
        if force_seen && !force_signal_sent {
            match terminate_process_tree(group_owner.as_ref(), root_process_id, true) {
                Ok(()) => {
                    failed_kills = 0;
                    force_signal_sent = true;
                }
                Err(error) => {
                    tree_error.get_or_insert(error);
                    failed_kills = failed_kills.saturating_add(1);
                    let _ = child.kill();
                    if failed_kills >= MAX_FORCE_KILL_ATTEMPTS {
                        break Err(failure(
                            "swallowtail.local_process.force_stop_failed",
                            "Local process could not be force-stopped",
                        ));
                    }
                }
            }
        }
    };

    // The root may already be reaped. Before a graceful group signal, the
    // live owner keeps the group identity from being reused. A successful
    // graceful signal may instead have ended the entire group, making this
    // final force request's not-found result expected.
    if group_owner.is_some()
        && !force_signal_sent
        && let Err(error) = terminate_process_tree(group_owner.as_ref(), root_process_id, true)
        && !graceful_tree_termination_succeeded
    {
        tree_error.get_or_insert(error);
    }
    if status.is_err() {
        let _ = child.kill();
        if child.wait().is_err() {
            tree_error.get_or_insert(failure(
                "swallowtail.local_process.wait_failed",
                "Local process exit could not be observed",
            ));
        }
    }
    if let Some(owner) = group_owner.as_mut() {
        if tree_error.is_some() {
            let _ = owner.kill();
        }
        if owner.wait().is_err() {
            tree_error.get_or_insert(failure(
                "swallowtail.local_process.tree_join_failed",
                "Local process tree owner could not be joined",
            ));
        }
    }

    let stdout_outcome = join_with_bound(stdout_reader, &reader_control, READER_JOIN_BOUND);
    let stderr_outcome = join_with_bound(stderr_reader, &reader_control, READER_JOIN_BOUND);
    let reader_error = if matches!(stdout_outcome, JoinOutcome::Panicked)
        || matches!(stderr_outcome, JoinOutcome::Panicked)
    {
        Some(failure(
            "swallowtail.local_process.reader_panicked",
            "Local process output supervision failed",
        ))
    } else if matches!(stdout_outcome, JoinOutcome::TimedOut)
        || matches!(stderr_outcome, JoinOutcome::TimedOut)
    {
        Some(failure(
            "swallowtail.local_process.reader_join_failed",
            "Local process output readers could not be joined",
        ))
    } else {
        None
    };
    if reader_error.is_some() {
        output.fail_supervision();
    } else {
        output.complete_supervision();
    }

    let cleanup_error = tree_error.or(reader_error);
    exit.complete(match cleanup_error {
        Some(error) => Err(error),
        None => status.map(exit_record),
    });
}

fn exit_record(status: ExitStatus) -> ProcessExit {
    ProcessExit::new(status.success(), status.code())
}
