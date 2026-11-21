use crate::config::TransportConfig;
use crate::error::{RemoteAcpError, RemoteAcpErrorKind, transport_error};
use crate::{http, websocket};
use futures_channel::mpsc;
use std::future::Future;
use std::sync::{Arc, Mutex};
use swallowtail_core::RemoteAcpTransport;
use swallowtail_protocol_acp::Message;
use swallowtail_runtime::{
    Deadline, DebugObservationKind, HostServices, RuntimeFailure, TimeService,
};

pub(crate) enum WorkerCommand {
    Send(Message),
    Cancel,
    Deadline,
    Close,
}

pub(crate) enum WorkerEvent {
    Message(Message),
}

pub(crate) type ReadySignal =
    Arc<Mutex<Option<futures_channel::oneshot::Sender<Result<(), RemoteAcpError>>>>>;

pub(crate) type TerminalErrorSlot = Arc<Mutex<Option<RemoteAcpError>>>;

/// Bounded grace window for sending a close frame or DELETE after the
/// connection deadline has elapsed.
const CLOSE_GRACE: std::time::Duration = std::time::Duration::from_secs(2);

#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    config: TransportConfig,
    commands: mpsc::Receiver<WorkerCommand>,
    events: mpsc::Sender<WorkerEvent>,
    ready: ReadySignal,
    terminal_error: TerminalErrorSlot,
    deadline: Option<Deadline>,
    time: Option<Arc<dyn TimeService>>,
    services: HostServices,
) -> Result<(), RuntimeFailure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| runtime_failure(transport_error()))?;
    runtime.block_on(async move {
        let ready = take_ready(&ready).ok_or_else(|| runtime_failure(transport_error()))?;
        let result = match config.transport {
            RemoteAcpTransport::StreamableHttpSse => {
                http::run(config, commands, events.clone(), ready, deadline, time).await
            }
            RemoteAcpTransport::WebSocket => {
                websocket::run(config, commands, events.clone(), ready, deadline, time).await
            }
        };
        if let Err(error) = &result {
            emit_worker_failure_debug(&services, error);
            *terminal_error
                .lock()
                .expect("remote ACP terminal error lock poisoned") = Some(error.clone());
        }
        drop(events);
        result.map_err(runtime_failure)
    })
}

fn emit_worker_failure_debug(services: &HostServices, error: &RemoteAcpError) {
    let diagnostic = error.diagnostic();
    let kind = match error.kind() {
        RemoteAcpErrorKind::ProtocolRejected => DebugObservationKind::ProtocolParse,
        RemoteAcpErrorKind::HostServiceMissing => DebugObservationKind::HostProcess,
        _ => DebugObservationKind::WireInbound,
    };
    services.emit_failure_debug(
        kind,
        crate::ROUTE,
        "remote.acp.worker",
        diagnostic.code(),
        diagnostic.message(),
    );
}

/// Races a worker future against the connection deadline.
///
/// The host time service owns the deadline semantics, so this crate never
/// assumes a tick rate. When the deadline elapses first, the future is
/// dropped and a deadline failure is returned; a non-responding peer cannot
/// outlive the connection deadline at any await point.
pub(crate) async fn race_deadline<F, T>(
    deadline: Option<Deadline>,
    time: Option<&dyn TimeService>,
    future: F,
) -> Result<T, RemoteAcpError>
where
    F: Future<Output = T>,
{
    match (deadline, time) {
        (Some(deadline), Some(time)) => {
            let wait = time.wait_until(deadline);
            tokio::select! {
                value = future => Ok(value),
                _ = wait => Err(cancellation_error(true)),
            }
        }
        _ => Ok(future.await),
    }
}

/// Races a post-deadline close handshake against a bounded grace period rather
/// than the already-elapsed connection deadline.
pub(crate) async fn race_close_grace<F, T>(
    time: Option<&dyn TimeService>,
    future: F,
) -> Result<T, RemoteAcpError>
where
    F: Future<Output = T>,
{
    match time {
        Some(time) => {
            let grace_deadline = Deadline::at(time.now().saturating_add_duration(CLOSE_GRACE));
            race_deadline(Some(grace_deadline), Some(time), future).await
        }
        None => tokio::time::timeout(CLOSE_GRACE, future)
            .await
            .map_err(|_| cancellation_error(true)),
    }
}

pub(crate) fn take_ready(
    ready: &ReadySignal,
) -> Option<futures_channel::oneshot::Sender<Result<(), RemoteAcpError>>> {
    ready.lock().expect("remote ACP ready lock poisoned").take()
}

fn runtime_failure(error: RemoteAcpError) -> RuntimeFailure {
    RuntimeFailure::new(error.diagnostic().clone())
}

pub(crate) fn cancellation_error(deadline: bool) -> RemoteAcpError {
    if deadline {
        RemoteAcpError::new(
            RemoteAcpErrorKind::DeadlineExceeded,
            "swallowtail.remote_acp.deadline_exceeded",
            "Remote ACP connection deadline elapsed",
        )
    } else {
        RemoteAcpError::new(
            RemoteAcpErrorKind::Cancelled,
            "swallowtail.remote_acp.cancelled",
            "Remote ACP connection was cancelled",
        )
    }
}
