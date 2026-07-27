use futures_channel::oneshot;
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_core::{EndpointAudience, ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, Deadline, DeadlineObservation, EndpointRef,
    HostServices, JoinedTask, MonotonicInstant, NetworkGrant, NetworkPolicyService, RuntimeFailure,
    ScopeId, ScopedTaskService, TimeService,
};

struct ThreadServices;

struct ThreadTask(Option<std::thread::JoinHandle<()>>);

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.0.take().expect("fixture task joins once");
        Box::pin(async move { handle.join().map_err(|_| failure()) })
    }
}

impl ScopedTaskService for ThreadServices {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Some(std::thread::spawn(move || {
            block_on(task);
        })))))
    }
}

impl BlockingWorkService for ThreadServices {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let (sender, receiver) = oneshot::channel();
        std::thread::spawn(move || {
            let _ = sender.send(job());
        });
        Box::pin(async move { receiver.await.map_err(|_| failure())? })
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
        Box::pin(async { Err(failure()) })
    }
}

pub fn services(host_id: ExecutionHostId) -> HostServices {
    let threads = Arc::new(ThreadServices);
    HostServices::new(host_id)
        .with_task(threads.clone())
        .with_blocking_work(threads.clone())
        .with_time(threads.clone())
        .with_network(threads)
}

fn failure() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "fixture.claude_lifecycle_remote.failed",
        "Claude lifecycle remote fixture failed",
    ))
}
