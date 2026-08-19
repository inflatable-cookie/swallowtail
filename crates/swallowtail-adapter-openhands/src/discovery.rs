use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, DebugObservationKind, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, ProcessHandle, ProcessOutputStream,
    ProcessRequest, RuntimeFailure, installed_probe_codes,
    validate_installed_executable_discovery_services,
};

use crate::command::discovery_arguments;
use crate::{OpenHandsAgentServerDriver, openhands_agent_server_claim};

const SWALLOWTAIL_OPENHANDS_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.openhands.agent_server");
const SOLUTION: &str = "OpenHands Agent Server";
const MAX_OUTPUT: usize = 4096;

impl DiscoveryDriver for OpenHandsAgentServerDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.openhands.agent_server.discovery_target_required",
                "OpenHands Agent Server discovery requires one explicit host-approved interpreter",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_package_version(request, services))
    }
}

async fn probe_package_version(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    let claim = openhands_agent_server_claim();
    if request.target().version_axis() != claim.axis() {
        return Err(crate::failure::failure(
            SWALLOWTAIL_OPENHANDS_PROBE_CODES.axis_mismatch,
            "OpenHands Agent Server discovery target uses a different version axis",
        ));
    }
    let process = match services
        .process()
        .expect("validated process service")
        .start(
            request.scope_id().clone(),
            ProcessRequest::new(request.target().executable().clone())
                .with_arguments(discovery_arguments()),
        )
        .await
    {
        Ok(process) => process,
        Err(_) => return Ok(failed(&services, DiscoveryStatus::Failed)),
    };
    if process.close_stdin().await.is_err() {
        return Ok(stop(&*process, &services, DiscoveryStatus::Failed).await);
    }
    let mut deadline = services
        .time()
        .expect("validated time service")
        .wait_until(request.deadline());
    let mut cancelled = request.cancellation().wait_requested();
    let mut stdout = Vec::new();
    loop {
        match next_output(process.as_ref(), &mut deadline, &mut cancelled).await {
            Probe::Cancelled => {
                return Ok(stop(&*process, &services, DiscoveryStatus::Cancelled).await);
            }
            Probe::TimedOut => {
                return Ok(stop(&*process, &services, DiscoveryStatus::TimedOut).await);
            }
            Probe::Output(Err(_)) => {
                return Ok(stop(&*process, &services, DiscoveryStatus::Failed).await);
            }
            Probe::Output(Ok(Some(chunk))) => {
                if chunk.stream() == ProcessOutputStream::Stdout {
                    if stdout.len().saturating_add(chunk.bytes().len()) > MAX_OUTPUT {
                        return Ok(stop(&*process, &services, DiscoveryStatus::Malformed).await);
                    }
                    stdout.extend_from_slice(chunk.bytes());
                }
            }
            Probe::Output(Ok(None)) => break,
        }
    }
    let exit = match process.wait().await {
        Ok(exit) => exit,
        Err(_) => return Ok(failed(&services, DiscoveryStatus::CleanupFailed)),
    };
    if !exit.success() {
        return Ok(failed(&services, DiscoveryStatus::Failed));
    }
    let Some(binding) = crate::selection::parse_openhands_version_output(&stdout) else {
        return Ok(failed(&services, DiscoveryStatus::Malformed));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        crate::failure::failure(
            SWALLOWTAIL_OPENHANDS_PROBE_CODES.classification_failed,
            "OpenHands Agent Server version observation could not be classified",
        )
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}

enum Probe {
    Output(Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure>),
    TimedOut,
    Cancelled,
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut swallowtail_runtime::BoxFuture<
        'static,
        swallowtail_runtime::DeadlineObservation,
    >,
    cancelled: &mut swallowtail_runtime::BoxFuture<'static, ()>,
) -> Probe {
    let mut output = process.read_output();
    poll_fn(|context| {
        if cancelled.as_mut().poll(context).is_ready() {
            return Poll::Ready(Probe::Cancelled);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Probe::TimedOut);
        }
        output.as_mut().poll(context).map(Probe::Output)
    })
    .await
}

async fn stop(
    process: &dyn ProcessHandle,
    services: &HostServices,
    status: DiscoveryStatus,
) -> DiscoveryOutcome {
    let graceful = process.request_stop().await;
    let forced = process.force_stop().await;
    let waited = process.wait().await;
    if graceful.is_err() || forced.is_err() || waited.is_err() {
        failed(services, DiscoveryStatus::CleanupFailed)
    } else {
        failed(services, status)
    }
}

fn failed(services: &HostServices, status: DiscoveryStatus) -> DiscoveryOutcome {
    let code = match status {
        DiscoveryStatus::Malformed => SWALLOWTAIL_OPENHANDS_PROBE_CODES.malformed,
        DiscoveryStatus::TimedOut => SWALLOWTAIL_OPENHANDS_PROBE_CODES.timed_out,
        DiscoveryStatus::Cancelled => SWALLOWTAIL_OPENHANDS_PROBE_CODES.cancelled,
        DiscoveryStatus::CleanupFailed => SWALLOWTAIL_OPENHANDS_PROBE_CODES.cleanup_failed,
        _ => SWALLOWTAIL_OPENHANDS_PROBE_CODES.failed,
    };
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        SOLUTION,
        "installed_discovery.probe",
        code,
        "OpenHands Agent Server installed discovery did not produce a compatible observation",
    );
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            code,
            "OpenHands Agent Server installed discovery did not produce a compatible observation",
        )),
    )
}
