use futures_channel::oneshot;
use futures_executor::block_on;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use swallowtail_core::EndpointAudience;
use swallowtail_host_local::LocalProcessHost;
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    CredentialService, Deadline, DeadlineObservation, EndpointRef, JoinedTask, MonotonicInstant,
    NetworkGrant, NetworkPolicyService, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Call {
    TaskSpawn,
    TaskJoin,
    BlockingStart,
    BlockingJoin,
    NetworkAuthorize,
    CredentialAcquire,
    CredentialRelease,
    TimerStart,
    TimerComplete,
    TimerDrop,
}

#[derive(Clone, Default)]
pub struct CallLog(Arc<Mutex<Vec<Call>>>);

impl CallLog {
    fn record(&self, call: Call) {
        self.0.lock().expect("call log lock poisoned").push(call);
    }

    pub fn calls(&self) -> Vec<Call> {
        self.0.lock().expect("call log lock poisoned").clone()
    }

    pub fn count(&self, call: Call) -> usize {
        self.calls().iter().filter(|seen| **seen == call).count()
    }
}

#[derive(Clone, Copy)]
pub enum TimeMode {
    Pending,
    Delayed,
}

#[derive(Clone)]
pub struct ThreadServices {
    origin: Instant,
    mode: TimeMode,
    calls: CallLog,
}

impl ThreadServices {
    pub fn new(mode: TimeMode, calls: CallLog) -> Self {
        Self {
            origin: Instant::now(),
            mode,
            calls,
        }
    }
}

struct Task {
    thread: JoinHandle<()>,
    calls: CallLog,
}

impl JoinedTask for Task {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.thread.join().map_err(|_| fixture_failure())?;
            self.calls.record(Call::TaskJoin);
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
        self.calls.record(Call::TaskSpawn);
        Ok(Box::new(Task {
            thread: thread::spawn(move || block_on(task)),
            calls: self.calls.clone(),
        }))
    }
}

impl BlockingWorkService for ThreadServices {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.calls.record(Call::BlockingStart);
        let calls = self.calls.clone();
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            let result = job();
            calls.record(Call::BlockingJoin);
            let _ = sender.send(result);
        });
        Box::pin(async move { receiver.await.map_err(|_| fixture_failure())? })
    }
}

impl TimeService for ThreadServices {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.origin.elapsed().as_millis() as u64)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        self.calls.record(Call::TimerStart);
        let inner: BoxFuture<'static, DeadlineObservation> = match self.mode {
            TimeMode::Pending => Box::pin(std::future::pending()),
            TimeMode::Delayed => {
                let (sender, receiver) = oneshot::channel();
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(20));
                    let _ = sender.send(DeadlineObservation::new(deadline, deadline.instant()));
                });
                Box::pin(async move { receiver.await.expect("fixture timer sender lives") })
            }
        };
        Box::pin(TrackedTimer {
            inner,
            calls: self.calls.clone(),
            completed: false,
        })
    }
}

struct TrackedTimer {
    inner: BoxFuture<'static, DeadlineObservation>,
    calls: CallLog,
    completed: bool,
}

impl Future for TrackedTimer {
    type Output = DeadlineObservation;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let result = self.inner.as_mut().poll(context);
        if result.is_ready() && !self.completed {
            self.completed = true;
            self.calls.record(Call::TimerComplete);
        }
        result
    }
}

impl Drop for TrackedTimer {
    fn drop(&mut self) {
        self.calls.record(Call::TimerDrop);
    }
}

pub struct TrackingNetwork {
    pub inner: LocalProcessHost,
    pub calls: CallLog,
}

impl NetworkPolicyService for TrackingNetwork {
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        self.calls.record(Call::NetworkAuthorize);
        self.inner.authorize(scope, endpoint, audience)
    }
}

pub struct TrackingCredential {
    pub inner: LocalProcessHost,
    pub calls: CallLog,
    pub fail_release: bool,
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.calls.record(Call::CredentialAcquire);
        self.inner.acquire(scope, reference, audience)
    }

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release(lease);
        let calls = self.calls.clone();
        let fail_release = self.fail_release;
        Box::pin(async move {
            let outcome = release.await;
            calls.record(Call::CredentialRelease);
            if fail_release && outcome == CleanupOutcome::Clean {
                CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "fixture.credential_release_failed",
                    "Fixture credential release failed",
                ))
            } else {
                outcome
            }
        })
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.thread_failed",
        "Fixture thread failed",
    ))
}
