use super::{cleanup_failure, failure};
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, HostServices, ModelArtifactLease, MonotonicInstant, ProcessExit,
    ProcessHandle, RuntimeFailure, ServingEndpointLease,
};

const STOP_GRACE_TICKS: u64 = 1_000;

pub(super) struct OwnedState {
    pub services: HostServices,
    pub process: Option<Arc<dyn ProcessHandle>>,
    pub endpoint: Option<ServingEndpointLease>,
    pub artifact: Option<ModelArtifactLease>,
}

impl OwnedState {
    pub fn new(services: HostServices) -> Self {
        Self {
            services,
            process: None,
            endpoint: None,
            artifact: None,
        }
    }

    pub async fn fail<T>(self, original: RuntimeFailure) -> Result<T, RuntimeFailure> {
        match self.cleanup().await {
            CleanupOutcome::Clean | CleanupOutcome::NotApplicable => Err(original),
            CleanupOutcome::Degraded(_) | CleanupOutcome::Failed(_) => Err(cleanup_failure()),
        }
    }

    pub async fn cleanup(mut self) -> CleanupOutcome {
        let mut failed = false;
        if let Some(process) = self.process.take()
            && stop_and_join(process.as_ref(), &self.services)
                .await
                .is_err()
        {
            failed = true;
        }
        if let Some(endpoint) = self.endpoint.take() {
            let outcome = self
                .services
                .serving_endpoint()
                .expect("owned state retains endpoint service")
                .release(endpoint)
                .await;
            failed |= matches!(
                outcome,
                CleanupOutcome::Degraded(_) | CleanupOutcome::Failed(_)
            );
        }
        if let Some(artifact) = self.artifact.take() {
            let outcome = self
                .services
                .model_artifact()
                .expect("owned state retains artifact service")
                .release(artifact)
                .await;
            failed |= matches!(
                outcome,
                CleanupOutcome::Degraded(_) | CleanupOutcome::Failed(_)
            );
        }
        if failed {
            CleanupOutcome::Failed(cleanup_failure().diagnostic().clone())
        } else {
            CleanupOutcome::Clean
        }
    }
}

async fn stop_and_join(
    process: &dyn ProcessHandle,
    services: &HostServices,
) -> Result<ProcessExit, RuntimeFailure> {
    let graceful = process.request_stop().await;
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.llama_cpp.owned_service_missing",
            "Owned llama.cpp time service is unavailable during cleanup",
        )
    })?;
    let grace = Deadline::at(MonotonicInstant::from_ticks(
        time.now().ticks().saturating_add(STOP_GRACE_TICKS),
    ));
    let wait = process.wait();
    let timer = time.wait_until(grace);
    let mut wait = Box::pin(wait);
    let mut timer = Box::pin(timer);
    let joined = poll_fn(|context| {
        if let Poll::Ready(exit) = wait.as_mut().poll(context) {
            return Poll::Ready(Some(exit));
        }
        if timer.as_mut().poll(context).is_ready() {
            return Poll::Ready(None);
        }
        Poll::Pending
    })
    .await;
    if let Some(exit) = joined {
        graceful?;
        return exit;
    }
    process.force_stop().await?;
    process.wait().await
}
