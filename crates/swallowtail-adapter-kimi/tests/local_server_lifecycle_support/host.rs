#[path = "host/process.rs"]
mod process;

use super::FixtureServer;
use futures_executor::block_on;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::thread::JoinHandle;
use swallowtail_core::{EndpointAudience, ExecutionHostId, ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::{
    AuthorizedEndpoint, BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome,
    CredentialLease, CredentialRef, CredentialService, Deadline, DeadlineObservation, EndpointRef,
    HostServices, JoinedTask, MonotonicInstant, NetworkGrant, NetworkPolicyService, ProcessRequest,
    ResourceLease, RuntimeFailure, ScopeId, ScopedTaskService, SecretLease, WorkingResourceRef,
    WorkingResourceService,
};

#[derive(Clone)]
pub struct FixtureHost {
    endpoint: String,
    pub(super) ready_endpoint: Arc<Mutex<String>>,
    now: Arc<AtomicU64>,
    deadline_waker: Arc<Mutex<Option<Waker>>>,
    pub(super) process_request: Arc<Mutex<Option<ProcessRequest>>>,
    pub(super) process_stopped: Arc<AtomicBool>,
    pub(super) process_waited: Arc<AtomicBool>,
    credential_releases: Arc<AtomicU64>,
}

impl FixtureHost {
    pub fn new(server: &FixtureServer) -> Self {
        Self::for_endpoint(server.endpoint())
    }

    pub fn for_endpoint(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
            ready_endpoint: Arc::new(Mutex::new(endpoint.to_owned())),
            now: Arc::new(AtomicU64::new(0)),
            deadline_waker: Arc::new(Mutex::new(None)),
            process_request: Arc::new(Mutex::new(None)),
            process_stopped: Arc::new(AtomicBool::new(false)),
            process_waited: Arc::new(AtomicBool::new(false)),
            credential_releases: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn services(&self, host: ExecutionHostId, process: bool) -> HostServices {
        let services = HostServices::new(host)
            .with_task(Arc::new(FixtureTaskService))
            .with_blocking_work(Arc::new(FixtureBlockingService))
            .with_time(Arc::new(self.clone()))
            .with_network(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()));
        if process {
            services.with_process(Arc::new(self.clone()))
        } else {
            services
        }
    }

    pub fn process_arguments(&self) -> Option<Vec<String>> {
        self.process_request
            .lock()
            .expect("fixture process request lock is not poisoned")
            .as_ref()
            .map(|request| request.arguments().map(str::to_owned).collect())
    }

    pub fn process_stopped_and_joined(&self) -> bool {
        self.process_stopped.load(Ordering::SeqCst) && self.process_waited.load(Ordering::SeqCst)
    }

    pub fn credential_releases(&self) -> u64 {
        self.credential_releases.load(Ordering::SeqCst)
    }

    pub fn set_now(&self, ticks: u64) {
        self.now.store(ticks, Ordering::SeqCst);
        if let Some(waker) = self
            .deadline_waker
            .lock()
            .expect("fixture deadline lock is not poisoned")
            .take()
        {
            waker.wake();
        }
    }

    pub fn set_ready_endpoint(&self, endpoint: impl Into<String>) {
        *self
            .ready_endpoint
            .lock()
            .expect("fixture ready-endpoint lock is not poisoned") = endpoint.into();
    }
}

struct FixtureBlockingService;

impl BlockingWorkService for FixtureBlockingService {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let (sender, receiver) = futures_channel::oneshot::channel();
        let thread = std::thread::spawn(move || {
            let result = job();
            let _ = sender.send(result);
        });
        Box::pin(async move {
            let result = receiver.await.map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.kimi.blocking_result_missing",
                    "Fixture blocking work did not return",
                ))
            })?;
            thread.join().map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.kimi.blocking_join_failed",
                    "Fixture blocking work did not join",
                ))
            })?;
            result
        })
    }
}

struct FixtureTaskService;
struct FixtureTask(Option<JoinHandle<()>>);

impl ScopedTaskService for FixtureTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(FixtureTask(Some(std::thread::spawn(move || {
            block_on(task);
        })))))
    }
}

impl JoinedTask for FixtureTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let thread = self.0.take().expect("fixture task joins once");
        Box::pin(async move {
            thread.join().map_err(|_| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.kimi.task_failed",
                    "Fixture task failed",
                ))
            })
        })
    }
}

impl swallowtail_runtime::TimeService for FixtureHost {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.now.load(Ordering::SeqCst))
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let now = Arc::clone(&self.now);
        let waker = Arc::clone(&self.deadline_waker);
        Box::pin(std::future::poll_fn(move |context| {
            let observed = MonotonicInstant::from_ticks(now.load(Ordering::SeqCst));
            if observed >= deadline.instant() {
                std::task::Poll::Ready(DeadlineObservation::new(deadline, observed))
            } else {
                *waker.lock().expect("fixture deadline lock is not poisoned") =
                    Some(context.waker().clone());
                std::task::Poll::Pending
            }
        }))
    }
}

impl NetworkPolicyService for FixtureHost {
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        let authorized = AuthorizedEndpoint::new(self.endpoint.clone())
            .expect("fixture authorized endpoint is valid");
        Box::pin(async move { Ok(NetworkGrant::new(scope, endpoint, audience, authorized)) })
    }
}

impl CredentialService for FixtureHost {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        Box::pin(async move {
            Ok(CredentialLease::Secret(SecretLease::new(
                scope,
                reference,
                FixtureServer::token().to_vec(),
                audience,
            )))
        })
    }

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        self.credential_releases.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move {
            drop(lease);
            CleanupOutcome::Clean
        })
    }
}

impl WorkingResourceService for FixtureHost {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async move {
            Ok(
                ResourceLease::consumer_owned(scope, reference, access, representation)
                    .with_filesystem(
                        swallowtail_runtime::MaterializedResourceRef::new("fixture.kimi.workspace")
                            .expect("fixture workspace is valid"),
                    ),
            )
        })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.kimi.temporary_rejected",
                "Fixture does not create temporary resources",
            )))
        })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::Clean })
    }
}
