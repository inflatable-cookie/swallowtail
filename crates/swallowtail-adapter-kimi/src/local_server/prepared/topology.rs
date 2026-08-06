#[path = "topology/readiness.rs"]
mod readiness;

use super::input::{KimiLocalServerOwnedInput, KimiLocalServerPreparationProbe};
use super::instance::build_prepared;
use super::preparation::{authorize_selected_endpoint, observe_server};
use super::validation::validate_input;
use super::{KimiLocalServerPreparedIntegration, preparation_failure};
use crate::failure::failure;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{InstanceOwnership, SafeDiagnostic};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, ExecutableRef, HostServices, MonotonicInstant, PreparationFailure,
    PreparationStage, ProcessHandle, ProcessRequest, RuntimeFailure,
};

use crate::local_server::transport::endpoint_port;
use readiness::observe_ready_origin;

const STOP_GRACE_TICKS: u64 = 1_000;

#[must_use = "owned Kimi local-server handles must be closed to join the child process"]
/// Owned Kimi local-server process paired with its prepared integration.
pub struct KimiLocalServerOwnedHandle {
    prepared: KimiLocalServerPreparedIntegration,
    process: Arc<dyn ProcessHandle>,
    services: HostServices,
}

impl KimiLocalServerOwnedHandle {
    /// Returns the prepared integration backed by the owned process.
    #[must_use]
    pub const fn prepared(&self) -> &KimiLocalServerPreparedIntegration {
        &self.prepared
    }

    /// Stops and joins the owned local-server process.
    pub async fn close(self) -> CleanupOutcome {
        cleanup_process(self.process.as_ref(), &self.services).await
    }
}

/// Starts, probes, and prepares one Swallowtail-owned Kimi local server.
pub async fn start_kimi_local_server_owned(
    input: KimiLocalServerOwnedInput,
    probe: KimiLocalServerPreparationProbe,
    services: HostServices,
) -> Result<KimiLocalServerOwnedHandle, PreparationFailure> {
    validate_input(&input.attached, &probe, &services, true)?;
    let available = services.available_kinds();
    let grant = authorize_selected_endpoint(&input.attached, &probe, &services).await?;
    let endpoint = grant.authorized().as_driver_value();
    let port = endpoint_port(endpoint).map_err(|error| {
        super::runtime_preparation_failure(PreparationStage::TargetSelection, error)
    })?;
    let request = ProcessRequest::new(ExecutableRef::from_instance_target(
        &input.executable_target,
    ))
    .with_arguments([
        "web".to_owned(),
        "--no-open".to_owned(),
        "--host".to_owned(),
        "127.0.0.1".to_owned(),
        "--port".to_owned(),
        port.to_string(),
        "--log-level".to_owned(),
        "info".to_owned(),
    ]);
    let process = services
        .process()
        .expect("validated process service")
        .start(probe.scope_id.clone(), request)
        .await
        .map_err(|error| {
            super::runtime_preparation_failure(PreparationStage::ProcessSpawn, error)
        })?;
    let process: Arc<dyn ProcessHandle> = Arc::from(process);
    if let Err(error) = observe_ready_origin(process.as_ref(), port, &probe, &services).await {
        return fail_after_spawn(process, &services, error).await;
    }
    let server = match observe_server(&input.attached, &probe, &services).await {
        Ok(server) => server,
        Err(error) => return fail_after_spawn(process, &services, error).await,
    };
    let prepared = build_prepared(
        input.attached,
        InstanceOwnership::HostOwnedEphemeral,
        Some(input.executable_target),
        server,
        available,
    );
    Ok(KimiLocalServerOwnedHandle {
        prepared,
        process,
        services,
    })
}

async fn fail_after_spawn<T>(
    process: Arc<dyn ProcessHandle>,
    services: &HostServices,
    cause: PreparationFailure,
) -> Result<T, PreparationFailure> {
    match cleanup_process(process.as_ref(), services).await {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => Err(cause),
        CleanupOutcome::Degraded(_) | CleanupOutcome::Failed(_) => Err(preparation_failure(
            PreparationStage::Cleanup,
            "swallowtail.kimi.local_server.preparation.cleanup_failed",
            "Owned Kimi local-server preparation cleanup could not be joined",
        )
        .with_cause(cause)),
    }
}

async fn cleanup_process(process: &dyn ProcessHandle, services: &HostServices) -> CleanupOutcome {
    match stop_and_join(process, services).await {
        Ok(()) => CleanupOutcome::Clean,
        Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.kimi.local_server.owned_cleanup_failed",
            "Owned Kimi local-server child cleanup could not be joined",
        )),
    }
}

async fn stop_and_join(
    process: &dyn ProcessHandle,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    let graceful = process.request_stop().await;
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.kimi.local_server.time_service_missing",
            "Owned Kimi local-server cleanup requires a time service",
        )
    })?;
    let grace = Deadline::at(MonotonicInstant::from_ticks(
        time.now().ticks().saturating_add(STOP_GRACE_TICKS),
    ));
    let mut wait = process.wait();
    let mut timer = time.wait_until(grace);
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
        exit?;
        return Ok(());
    }
    process.force_stop().await?;
    process.wait().await?;
    Ok(())
}
