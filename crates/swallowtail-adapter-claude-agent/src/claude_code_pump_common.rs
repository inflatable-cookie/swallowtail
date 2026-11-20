//! Pump mechanics shared by the headless and response-only Claude Code routes.
//!
//! Both routes read one child process, decode complete lines into runtime
//! events, and end in the same three ways: a deadline, a decode or host
//! failure, or the process exiting. Only the route label, the stage names, and
//! the failure codes differ, so each route supplies one [`PumpRoute`]
//! descriptor and the mechanics live here once.

use crate::claude_code_handle::ClaudeCodeCancellation;
use std::future::poll_fn;
use std::task::{Context, Poll};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind, HostServices,
    ProcessExit, ProcessHandle, ProcessOutputChunk, RuntimeEvent, RuntimeEventSender,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
};

/// Route identity stamped on every pump observation and failure.
pub(crate) struct PumpRoute {
    /// Observer route used by debug observations.
    pub(crate) route: &'static str,
    /// Human label used in failure messages.
    pub(crate) label: &'static str,
    /// Debug stage for a stream that failed to decode.
    pub(crate) decode_stage: &'static str,
    /// Debug stage for a stream the host could not read.
    pub(crate) read_stage: &'static str,
    /// Debug stage for a stream that failed after the process ended.
    pub(crate) finish_stage: &'static str,
    /// Debug stage for a process that could not be reaped.
    pub(crate) wait_stage: &'static str,
    /// Failure code for a process that exited unsuccessfully.
    pub(crate) process_wait_failed: &'static str,
    /// Failure code for an unreleased operation-scoped resource.
    pub(crate) process_cleanup_failed: &'static str,
    /// Failure code for an event that could not reach the consumer.
    pub(crate) event_delivery_failed: &'static str,
}

/// Route identity for the structured headless route.
pub(crate) const HEADLESS_PUMP_ROUTE: PumpRoute = PumpRoute {
    route: "claude-code.headless",
    label: "Claude Code headless",
    decode_stage: "headless.pump.decode",
    read_stage: "headless.pump.read",
    finish_stage: "headless.pump.finish",
    wait_stage: "headless.pump.wait",
    process_wait_failed: "swallowtail.claude_code.headless.process_wait_failed",
    process_cleanup_failed: "swallowtail.claude_code.headless.process_cleanup_failed",
    event_delivery_failed: "swallowtail.claude_code.headless.event_delivery_failed",
};

/// Route identity for the tool-free response-only route.
pub(crate) const RESPONSE_ONLY_PUMP_ROUTE: PumpRoute = PumpRoute {
    route: "claude-code.response-only",
    label: "Claude Code response-only",
    decode_stage: "response-only.pump.decode",
    read_stage: "response-only.pump.read",
    finish_stage: "response-only.pump.finish",
    wait_stage: "response-only.pump.wait",
    process_wait_failed: "swallowtail.claude_code.response_only.process_wait_failed",
    process_cleanup_failed: "swallowtail.claude_code.response_only.process_cleanup_failed",
    event_delivery_failed: "swallowtail.claude_code.response_only.event_delivery_failed",
};

/// What one poll of the child process produced.
pub(crate) enum NextOutput {
    /// One process chunk, one clean end of output, or one host read failure.
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    /// The operation deadline elapsed before the next chunk arrived.
    Deadline,
}

/// Polls one process chunk against cancellation and the operation deadline.
///
/// A requested cancellation stops consulting the deadline, so a cancelled
/// operation drains its remaining output instead of reporting a timeout.
pub(crate) fn poll_output<'a>(
    read: &mut BoxFuture<'a, Result<Option<ProcessOutputChunk>, RuntimeFailure>>,
    cancellation: &ClaudeCodeCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
    context: &mut Context<'_>,
) -> Poll<NextOutput> {
    if !cancellation.is_requested()
        && let Some(wait) = deadline.as_mut()
        && wait.as_mut().poll(context).is_ready()
    {
        return Poll::Ready(NextOutput::Deadline);
    }
    read.as_mut().poll(context).map(NextOutput::Process)
}

/// Awaits the next process chunk, deadline, or host read failure.
pub(crate) async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &ClaudeCodeCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> NextOutput {
    let mut read = process.read_output();
    poll_fn(|context| poll_output(&mut read, cancellation, deadline, context)).await
}

/// Delivers every parsed event to the consuming stream.
pub(crate) fn send_all(
    sender: &RuntimeEventSender,
    events: impl IntoIterator<Item = RuntimeEvent>,
) -> Result<(), RuntimeFailure> {
    for event in events {
        sender.send(event)?;
    }
    Ok(())
}

/// Force-stops and reaps the child, reporting what the release achieved.
pub(crate) async fn force_cleanup(
    route: &PumpRoute,
    process: &dyn ProcessHandle,
) -> CleanupOutcome {
    let force = process.force_stop().await;
    let wait = process.wait().await;
    if force.is_err() || wait.is_err() {
        process_cleanup_failed(route)
    } else {
        CleanupOutcome::Clean
    }
}

/// Stops a child whose start already failed, discarding the outcome.
pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
    let _ = process.force_stop().await;
    let _ = process.wait().await;
}

/// Reports whether reaping the exited child released every resource.
pub(crate) fn cleanup_from_wait(
    route: &PumpRoute,
    exit: &Result<ProcessExit, RuntimeFailure>,
) -> CleanupOutcome {
    if exit.is_ok() {
        CleanupOutcome::Clean
    } else {
        process_cleanup_failed(route)
    }
}

/// Records a decode failure as a protocol-parse debug observation.
pub(crate) fn emit_protocol_debug(
    route: &PumpRoute,
    services: &HostServices,
    error: &RuntimeFailure,
    stage: &'static str,
) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::ProtocolParse,
        route.route,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

/// Records a host read failure as a host-process debug observation.
pub(crate) fn emit_host_process_debug(
    route: &PumpRoute,
    services: &HostServices,
    error: &RuntimeFailure,
    stage: &'static str,
) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        route.route,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

/// Ends the operation when an event could not reach the consumer.
pub(crate) fn event_delivery_failed(route: &PumpRoute, cleanup: CleanupOutcome) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            route.event_delivery_failed,
            format!("{} event delivery failed", route.label),
        )),
        cleanup,
    )
}

/// Ends the operation when the exited child could not be reaped.
pub(crate) fn process_wait_failed(route: &PumpRoute, services: &HostServices) -> TerminalOutcome {
    let diagnostic = SafeDiagnostic::new(
        route.process_wait_failed,
        format!("{} process wait failed", route.label),
    );
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        route.route,
        route.wait_stage,
        diagnostic.code(),
        diagnostic.message(),
    );
    TerminalOutcome::new(
        TerminalStatus::HostFailed(diagnostic),
        process_cleanup_failed(route),
    )
}

/// The cleanup outcome recorded when a required release failed.
pub(crate) fn process_cleanup_failed(route: &PumpRoute) -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        route.process_cleanup_failed,
        format!("{} process cleanup failed", route.label),
    ))
}
