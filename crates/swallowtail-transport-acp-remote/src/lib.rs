//! Provider-neutral remote ACP physical transport.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod config;
mod cookies;
mod correlation;
mod error;
mod http;
mod sse;
mod websocket;
mod wire;
mod worker;

#[cfg(test)]
mod transport_tests;

use config::TransportConfig;
use error::transport_error;
use futures_channel::{mpsc, oneshot};
use futures_util::{SinkExt, StreamExt};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use swallowtail_core::{PreflightPlan, SafeDiagnostic};
use swallowtail_protocol_acp::Message;
use swallowtail_runtime::{
    BlockingJob, CleanupOutcome, Deadline, DebugObservationKind, EndpointRef, HostServices,
    JoinedTask, NetworkGrant, ScopeId,
};
use worker::{ReadySignal, WorkerCommand, WorkerEvent};

pub use error::{RemoteAcpError, RemoteAcpErrorKind};

pub(crate) const ROUTE: &str = "acp.remote";

/// Exact host-bound request to open one remote ACP transport connection.
pub struct RemoteAcpConnectRequest {
    scope: ScopeId,
    endpoint: EndpointRef,
    deadline: Option<Deadline>,
}

impl RemoteAcpConnectRequest {
    /// Creates a request without a connection deadline.
    #[must_use]
    pub const fn new(scope: ScopeId, endpoint: EndpointRef) -> Self {
        Self {
            scope,
            endpoint,
            deadline: None,
        }
    }

    /// Sets the absolute connection deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Returns the operation scope that owns the connection.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the opaque endpoint reference bound during preflight.
    #[must_use]
    pub const fn endpoint(&self) -> &EndpointRef {
        &self.endpoint
    }

    /// Returns the optional absolute connection deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

/// Stateless client for opening host-authorized remote ACP connections.
#[derive(Clone, Copy, Debug, Default)]
pub struct RemoteAcpClient;

impl RemoteAcpClient {
    /// Opens a connection after validating plan, endpoint, grant, and host services.
    pub fn connect<'a>(
        &'a self,
        plan: &'a PreflightPlan,
        request: RemoteAcpConnectRequest,
        grant: NetworkGrant,
        services: HostServices,
    ) -> Pin<Box<dyn Future<Output = Result<RemoteAcpConnection, RemoteAcpError>> + Send + 'a>>
    {
        Box::pin(async move {
            services
                .require_execution_host(plan.execution_host_id())
                .map_err(|_| binding_failure())?;
            let config =
                TransportConfig::from_bound_grant(plan, &request.scope, &request.endpoint, &grant)?;
            connect_bound(config, request.scope, request.deadline, services).await
        })
    }
}

async fn connect_bound(
    config: TransportConfig,
    scope: ScopeId,
    deadline: Option<Deadline>,
    services: HostServices,
) -> Result<RemoteAcpConnection, RemoteAcpError> {
    let task = match services.task().cloned() {
        Some(task) => task,
        None => {
            let error = host_service_failure();
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            return Err(error);
        }
    };
    let blocking = match services.blocking_work().cloned() {
        Some(blocking) => blocking,
        None => {
            let error = host_service_failure();
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            return Err(error);
        }
    };
    let time = match services.time().cloned() {
        Some(time) => time,
        None => {
            let error = host_service_failure();
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            return Err(error);
        }
    };
    if services.network().is_none() {
        let error = host_service_failure();
        emit_remote_failure_debug(&services, &error, "remote.acp.connect");
        return Err(error);
    }
    if let Some(deadline) = deadline
        && time.now() >= deadline.instant()
    {
        let error = worker::cancellation_error(true);
        emit_remote_failure_debug(&services, &error, "remote.acp.connect");
        return Err(error);
    }
    let command_capacity = match usize::try_from(config.bounds.maximum_pending_requests().get()) {
        Ok(capacity) => capacity,
        Err(_) => {
            let error = error::capacity_error();
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            return Err(error);
        }
    };
    let event_capacity = match usize::try_from(
        config
            .bounds
            .maximum_connection_stream_events()
            .get()
            .saturating_add(config.bounds.maximum_session_stream_events().get()),
    ) {
        Ok(capacity) => capacity,
        Err(_) => {
            let error = error::capacity_error();
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            return Err(error);
        }
    };
    let (commands, command_rx) = mpsc::channel(command_capacity);
    let (event_tx, events) = mpsc::channel(event_capacity);
    let (ready_tx, ready_rx) = oneshot::channel();
    let ready: ReadySignal = Arc::new(Mutex::new(Some(ready_tx)));
    let ready_for_job = Arc::clone(&ready);
    let worker_time = Arc::clone(&time);
    let worker_services = services.clone();
    let job = Box::new(move || {
        worker::run(
            config,
            command_rx,
            event_tx,
            ready_for_job,
            deadline,
            Some(worker_time),
            worker_services,
        )
    }) as BlockingJob;
    let blocking_work = blocking.run(scope.clone(), job);
    let ready_for_task = Arc::clone(&ready);
    let connection_task = task
        .spawn(
            scope.clone(),
            Box::pin(async move {
                if blocking_work.await.is_err()
                    && let Some(ready) = worker::take_ready(&ready_for_task)
                {
                    let _ = ready.send(Err(transport_error()));
                }
            }),
        )
        .map_err(|_| {
            let error = host_service_failure();
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            error
        })?;

    // Start the deadline task before the connect resolves, so a hanging
    // initial connect is interruptible; the connect itself is raced against
    // the deadline inside the worker.
    let (mut deadline_done, mut deadline_task) = if let Some(deadline) = deadline {
        let (done_tx, done_rx) = oneshot::channel();
        let mut commands = commands.clone();
        let wait = time.wait_until(deadline);
        let deadline_task = task
            .spawn(
                scope,
                Box::pin(async move {
                    match futures_util::future::select(wait, Box::pin(done_rx)).await {
                        futures_util::future::Either::Left((_observation, _)) => {
                            let _ = commands.send(WorkerCommand::Deadline).await;
                        }
                        futures_util::future::Either::Right(_) => {}
                    }
                }),
            )
            .map_err(|_| {
                let error = host_service_failure();
                emit_remote_failure_debug(&services, &error, "remote.acp.connect");
                error
            })?;
        (Some(done_tx), Some(deadline_task))
    } else {
        (None, None)
    };

    let ready = match deadline {
        Some(deadline) => {
            let wait = time.wait_until(deadline);
            match futures_util::future::select(Box::pin(ready_rx), wait).await {
                futures_util::future::Either::Left((ready, _)) => {
                    ready.map_err(|_| transport_error())
                }
                futures_util::future::Either::Right(_) => Err(worker::cancellation_error(true)),
            }
        }
        None => ready_rx.await.map_err(|_| transport_error()),
    };
    match ready {
        Ok(Ok(())) => {}
        Ok(Err(error)) | Err(error) => {
            emit_remote_failure_debug(&services, &error, "remote.acp.connect");
            if let Some(done) = deadline_done.take() {
                let _ = done.send(());
            }
            if let Some(task) = deadline_task.take() {
                let _ = task.join().await;
            }
            let _ = connection_task.join().await;
            return Err(error);
        }
    }

    Ok(RemoteAcpConnection {
        commands,
        events,
        connection_task: Some(connection_task),
        deadline_task,
        deadline_done,
        // Retained so connect-scoped host services outlive worker setup.
        _services: services,
    })
}

fn emit_remote_failure_debug(services: &HostServices, error: &RemoteAcpError, stage: &'static str) {
    let diagnostic = error.diagnostic();
    let kind = match error.kind() {
        RemoteAcpErrorKind::ProtocolRejected => DebugObservationKind::ProtocolParse,
        RemoteAcpErrorKind::HostServiceMissing => DebugObservationKind::HostProcess,
        _ => DebugObservationKind::WireInbound,
    };
    services.emit_failure_debug(kind, ROUTE, stage, diagnostic.code(), diagnostic.message());
}

#[must_use = "remote ACP connections must be closed and joined"]
/// Owned remote ACP connection with bounded send and receive streams.
pub struct RemoteAcpConnection {
    commands: mpsc::Sender<WorkerCommand>,
    events: mpsc::Receiver<WorkerEvent>,
    connection_task: Option<Box<dyn JoinedTask>>,
    deadline_task: Option<Box<dyn JoinedTask>>,
    deadline_done: Option<oneshot::Sender<()>>,
    _services: HostServices,
}

impl RemoteAcpConnection {
    /// Sends one ACP message to the peer.
    pub async fn send(&mut self, message: Message) -> Result<(), RemoteAcpError> {
        self.commands
            .send(WorkerCommand::Send(message))
            .await
            .map_err(|_| transport_error())
    }

    /// Waits for the next peer message or terminal transport failure.
    pub async fn next_event(&mut self) -> Option<Result<Message, RemoteAcpError>> {
        match self.events.next().await {
            Some(WorkerEvent::Message(message)) => Some(Ok(message)),
            Some(WorkerEvent::Failed(error)) => Some(Err(error)),
            None => None,
        }
    }

    /// Requests cancellation of the connection worker.
    pub async fn cancel(&mut self) -> Result<(), RemoteAcpError> {
        self.commands
            .send(WorkerCommand::Cancel)
            .await
            .map_err(|_| transport_error())
    }

    /// Closes the transport and joins all host-owned work.
    pub async fn close(mut self) -> CleanupOutcome {
        let _ = self.commands.send(WorkerCommand::Close).await;
        if let Some(done) = self.deadline_done.take() {
            let _ = done.send(());
        }
        let connection = join(self.connection_task.take()).await;
        let deadline = join(self.deadline_task.take()).await;
        if connection && deadline {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.remote_acp.cleanup_failed",
                "Remote ACP transport cleanup failed",
            ))
        }
    }
}

async fn join(task: Option<Box<dyn JoinedTask>>) -> bool {
    match task {
        Some(task) => task.join().await.is_ok(),
        None => true,
    }
}

fn binding_failure() -> RemoteAcpError {
    error::binding_error()
}

fn host_service_failure() -> RemoteAcpError {
    RemoteAcpError::new(
        RemoteAcpErrorKind::HostServiceMissing,
        "swallowtail.remote_acp.host_service_missing",
        "Remote ACP transport requires task, blocking-work, time, and network services",
    )
}
