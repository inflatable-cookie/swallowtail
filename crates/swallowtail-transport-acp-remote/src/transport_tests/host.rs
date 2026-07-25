use super::config;
use crate::transport_tests::websocket::termination_server;
use futures_channel::oneshot;
use futures_executor::block_on;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use swallowtail_core::{EndpointAudience, ExecutionHostId, RemoteAcpTransport, SafeDiagnostic};
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
