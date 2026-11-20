use super::config;
use crate::transport_tests::websocket::termination_server;
use crate::worker::{WorkerCommand, WorkerEvent};
use crate::{RemoteAcpErrorKind, config::TransportConfig};
use futures_channel::{mpsc, oneshot};
use futures_executor::block_on;
use futures_util::SinkExt;
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use swallowtail_core::{
    EndpointAudience, ExecutionHostId, RemoteAcpConnectionBounds, RemoteAcpTransport,
    SafeDiagnostic,
};
use swallowtail_protocol_acp::Message;
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome, Deadline, DeadlineObservation,
    EndpointRef, HostServices, JoinedTask, MonotonicInstant, NetworkGrant, NetworkPolicyService,
    RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Default)]
struct LifecycleCounts {
    blocking_completed: AtomicUsize,
    task_joined: AtomicUsize,
}

struct ThreadServices {
    counts: Arc<LifecycleCounts>,
}

struct ThreadTask {
    completed: oneshot::Receiver<()>,
    counts: Arc<LifecycleCounts>,
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.completed.await.map_err(|_| fixture_failure())?;
            self.counts.task_joined.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
    }
}

impl ScopedTaskService for ThreadServices {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        let (sender, completed) = oneshot::channel();
        thread::spawn(move || {
            block_on(task);
            let _ = sender.send(());
        });
        Ok(Box::new(ThreadTask {
            completed,
            counts: Arc::clone(&self.counts),
        }))
    }
}

impl BlockingWorkService for ThreadServices {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let (sender, receiver) = oneshot::channel();
        let counts = Arc::clone(&self.counts);
        thread::spawn(move || {
            let result = job();
            counts.blocking_completed.fetch_add(1, Ordering::SeqCst);
            let _ = sender.send(result);
        });
        Box::pin(async move { receiver.await.map_err(|_| fixture_failure())? })
    }
}

impl TimeService for ThreadServices {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

/// Test time service with nanosecond ticks from a local origin.
///
/// Mirrors the host-local convention so deadline tests can express absolute
/// deadlines as offsets from service creation.
pub(super) struct ElapsedTimeService {
    origin: std::time::Instant,
}

impl ElapsedTimeService {
    pub(super) fn new() -> Self {
        Self {
            origin: std::time::Instant::now(),
        }
    }

    /// Returns an absolute deadline that elapses `offset` after creation.
    pub(super) fn deadline(&self, offset: std::time::Duration) -> Deadline {
        let ticks = u64::try_from(offset.as_nanos()).expect("fixture offset fits monotonic ticks");
        Deadline::at(MonotonicInstant::from_ticks(ticks))
    }
}

impl TimeService for ElapsedTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(
            u64::try_from(self.origin.elapsed().as_nanos()).unwrap_or(u64::MAX),
        )
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let now = self.now();
        if now >= deadline.instant() {
            return Box::pin(async move { DeadlineObservation::new(deadline, now) });
        }
        let remaining = std::time::Duration::from_nanos(deadline.instant().ticks() - now.ticks());
        Box::pin(async move {
            tokio::time::sleep(remaining).await;
            DeadlineObservation::new(deadline, deadline.instant())
        })
    }
}

impl NetworkPolicyService for ThreadServices {
    fn authorize(
        &self,
        _scope: ScopeId,
        _endpoint: EndpointRef,
        _audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }
}

pub(super) async fn host_blocking_runtime_is_owned_and_joined() {
    let (endpoint, server) = termination_server(false).await;
    let counts = Arc::new(LifecycleCounts::default());
    let services = services_with_counts(
        ExecutionHostId::new("fixture.host.remote-acp").unwrap(),
        Arc::clone(&counts),
    );
    let connection = crate::connect_bound(
        config(endpoint, RemoteAcpTransport::WebSocket),
        ScopeId::new("fixture.remote-acp-operation").unwrap(),
        None,
        services,
    )
    .await
    .unwrap();

    assert_eq!(connection.close().await, CleanupOutcome::Clean);
    assert!(server.await.unwrap(), "fixture peer observed close");
    assert_eq!(counts.blocking_completed.load(Ordering::SeqCst), 1);
    assert_eq!(counts.task_joined.load(Ordering::SeqCst), 1);
}

pub(super) fn services(execution_host_id: ExecutionHostId) -> HostServices {
    services_with_counts(execution_host_id, Arc::new(LifecycleCounts::default()))
}

fn services_with_counts(
    execution_host_id: ExecutionHostId,
    counts: Arc<LifecycleCounts>,
) -> HostServices {
    let service = Arc::new(ThreadServices { counts });
    HostServices::new(execution_host_id)
        .with_task(service.clone())
        .with_blocking_work(service.clone())
        .with_time(service.clone())
        .with_network(service)
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "fixture.remote_acp.failed",
        "Remote ACP fixture failed",
    ))
}

struct FailSecondSpawnTaskService {
    spawns: AtomicUsize,
    counts: Arc<LifecycleCounts>,
}

impl ScopedTaskService for FailSecondSpawnTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        if self.spawns.fetch_add(1, Ordering::SeqCst) >= 1 {
            return Err(fixture_failure());
        }
        let (sender, completed) = oneshot::channel();
        thread::spawn(move || {
            block_on(task);
            let _ = sender.send(());
        });
        Ok(Box::new(ThreadTask {
            completed,
            counts: Arc::clone(&self.counts),
        }))
    }
}

pub(super) async fn full_events_channel_close_does_not_deadlock() {
    let (endpoint, server) = termination_server(true).await;
    let counts = Arc::new(LifecycleCounts::default());
    let services = services_with_counts(
        ExecutionHostId::new("fixture.host.full-events").unwrap(),
        Arc::clone(&counts),
    );
    // Minimum event capacity: 1 + 1 = 2.
    let small_config = TransportConfig {
        endpoint: endpoint.clone(),
        transport: RemoteAcpTransport::WebSocket,
        bounds: RemoteAcpConnectionBounds::new(
            NonZeroU64::new(64 * 1024).unwrap(),
            NonZeroU32::new(4).unwrap(),
            NonZeroU32::new(4).unwrap(),
            NonZeroU32::new(1).unwrap(),
            NonZeroU32::new(1).unwrap(),
        ),
        maximum_cookie_count: NonZeroU32::new(4).unwrap(),
        maximum_cookie_bytes: NonZeroU64::new(4096).unwrap(),
    };
    let mut connection = crate::connect_bound(
        small_config,
        ScopeId::new("fixture.full-events").unwrap(),
        None,
        services,
    )
    .await
    .unwrap();
    // Send initialize; termination_server(true) sends 1 response then closes the socket.
    // Even before reading any event, pre-filling or letting the worker fail with a full channel
    // must not deadlock close().
    connection.send(super::initialize()).await.unwrap();
    // Wait briefly for the server to respond and disconnect so worker::run reaches error reporting.
    let _ = server.await;
    let closed = tokio::time::timeout(std::time::Duration::from_secs(3), connection.close())
        .await
        .expect("close must not deadlock when events is not drained");
    assert_eq!(closed, CleanupOutcome::Clean);
    assert_eq!(counts.blocking_completed.load(Ordering::SeqCst), 1);
    assert_eq!(counts.task_joined.load(Ordering::SeqCst), 1);

    // Also prove worker::run itself never blocks when events is strictly full at capacity 1.
    let (endpoint, server2) = termination_server(false).await;
    let (mut commands, command_rx, mut event_tx, _events_kept_open, ready_tx, ready_rx) =
        mpsc::channel::<WorkerCommand>(4).pipe(|(ctx, crx)| {
            let (etx, erx) = mpsc::channel::<WorkerEvent>(1);
            let (rtx, rrx) = oneshot::channel();
            (ctx, crx, etx, erx, rtx, rrx)
        });
    event_tx
        .try_send(WorkerEvent::Message(Message::Notification {
            method: "occupied".to_owned(),
            params: serde_json::json!({}),
        }))
        .expect("fills events channel");
    let worker_services = self::services(ExecutionHostId::new("fixture.host.worker-full").unwrap());
    let worker = tokio::task::spawn_blocking(move || {
        crate::worker::run(
            config(endpoint, RemoteAcpTransport::WebSocket),
            command_rx,
            event_tx,
            Arc::new(std::sync::Mutex::new(Some(ready_tx))),
            None,
            None,
            worker_services,
        )
    });
    ready_rx.await.unwrap().unwrap();
    commands.send(WorkerCommand::Cancel).await.unwrap();
    let result = tokio::time::timeout(std::time::Duration::from_secs(3), worker)
        .await
        .expect("worker::run must not deadlock when events is full")
        .unwrap();
    assert!(result.is_err());
    let _ = server2.await;
}

pub(super) async fn deadline_task_spawn_failure_joins_connection_task() {
    let (endpoint, server) = termination_server(false).await;
    let counts = Arc::new(LifecycleCounts::default());
    let base = Arc::new(ThreadServices {
        counts: Arc::clone(&counts),
    });
    let fail_second = Arc::new(FailSecondSpawnTaskService {
        spawns: AtomicUsize::new(0),
        counts: Arc::clone(&counts),
    });
    let time = Arc::new(ElapsedTimeService::new());
    let deadline = time.deadline(std::time::Duration::from_secs(5));
    let services = HostServices::new(ExecutionHostId::new("fixture.host.spawn-fail").unwrap())
        .with_task(fail_second)
        .with_blocking_work(base.clone())
        .with_time(time)
        .with_network(base);
    let err = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        crate::connect_bound(
            config(endpoint, RemoteAcpTransport::StreamableHttpSse),
            ScopeId::new("fixture.spawn-fail").unwrap(),
            Some(deadline),
            services,
        ),
    )
    .await
    .expect("deadline_task spawn failure cleans up promptly")
    .err()
    .expect("spawn failure returns Err");
    assert_eq!(err.kind(), RemoteAcpErrorKind::HostServiceMissing);
    assert_eq!(counts.task_joined.load(Ordering::SeqCst), 1);
    assert_eq!(counts.blocking_completed.load(Ordering::SeqCst), 1);
    drop(server);
}

struct ThreadSleepTimeService {
    origin: std::time::Instant,
}

impl ThreadSleepTimeService {
    fn new() -> Self {
        Self {
            origin: std::time::Instant::now(),
        }
    }

    fn deadline(&self, offset: std::time::Duration) -> Deadline {
        let ticks = u64::try_from(offset.as_nanos()).expect("fixture offset fits monotonic ticks");
        Deadline::at(MonotonicInstant::from_ticks(ticks))
    }
}

impl TimeService for ThreadSleepTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(
            u64::try_from(self.origin.elapsed().as_nanos()).unwrap_or(u64::MAX),
        )
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let now = self.now();
        if now >= deadline.instant() {
            return Box::pin(async move { DeadlineObservation::new(deadline, now) });
        }
        let remaining = std::time::Duration::from_nanos(deadline.instant().ticks() - now.ticks());
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            thread::sleep(remaining);
            let _ = tx.send(());
        });
        Box::pin(async move {
            let _ = rx.await;
            DeadlineObservation::new(deadline, deadline.instant())
        })
    }
}

pub(super) async fn connect_timeout_signals_close_and_joins_tasks() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _open = stream;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    });
    let counts = Arc::new(LifecycleCounts::default());
    let base = Arc::new(ThreadServices {
        counts: Arc::clone(&counts),
    });
    let time = Arc::new(ThreadSleepTimeService::new());
    let deadline = time.deadline(std::time::Duration::from_millis(50));
    let services = HostServices::new(ExecutionHostId::new("fixture.host.connect-timeout").unwrap())
        .with_task(base.clone())
        .with_blocking_work(base.clone())
        .with_time(time)
        .with_network(base);
    let endpoint = url::Url::parse(&format!("ws://{address}/acp")).unwrap();
    let err = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        crate::connect_bound(
            config(endpoint, RemoteAcpTransport::WebSocket),
            ScopeId::new("fixture.connect-timeout").unwrap(),
            Some(deadline),
            services,
        ),
    )
    .await
    .expect("connect timeout completes")
    .err()
    .expect("connect timeout returns Err");
    assert_eq!(err.kind(), RemoteAcpErrorKind::DeadlineExceeded);
    assert_eq!(counts.task_joined.load(Ordering::SeqCst), 2);
    assert_eq!(counts.blocking_completed.load(Ordering::SeqCst), 1);
    let _ = server.await;
}

trait PipeExt: Sized {
    fn pipe<U>(self, f: impl FnOnce(Self) -> U) -> U {
        f(self)
    }
}

impl<T> PipeExt for T {}
