use futures_channel::oneshot;
use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, ProcessOutputStream, ProcessRequest, RuntimeFailure,
    validate_installed_executable_discovery_services,
};

use crate::{
    AntigravityCatalogueDriver, AntigravityHeadlessDriver, antigravity_catalogue_claim,
    antigravity_headless_claim,
};

const MAX_VERSION_OUTPUT_BYTES: usize = 64;
const MAX_VERSION_STDERR_BYTES: usize = 1_024;
const MAX_SAFE_STDERR_CHARS: usize = 240;

mod probe;
use probe::{
    ProbeSignal, exit_failed, next_output, outcome, parse_version, staged_outcome, stop_and_classify,
};
pub(crate) use probe::sanitized_stderr;

impl DiscoveryDriver for AntigravityCatalogueDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.antigravity.discovery_target_required",
                "Antigravity discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_joined(
            request,
            services,
            antigravity_catalogue_claim(),
        ))
    }
}

impl DiscoveryDriver for AntigravityHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.antigravity.discovery_target_required",
                "Antigravity discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_joined(
            request,
            services,
            antigravity_headless_claim(),
        ))
    }
}

async fn probe_joined(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: swallowtail_core::InterfaceCompatibilityClaim,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    if request.target().version_axis() != claim.axis() {
        return Err(crate::failure::failure(
            "swallowtail.antigravity.discovery_axis_mismatch",
            "Antigravity discovery target uses a different release axis",
        ));
    }
    if request.cancellation().is_requested() {
        return Ok(outcome(DiscoveryStatus::Cancelled));
    }
    let task_service = services
        .task()
        .expect("validated task service is present")
        .clone();
    let scope = request.scope_id().clone();
    let (sender, receiver) = oneshot::channel();
    let task = match task_service.spawn(
        scope,
        Box::pin(async move {
            let result = probe_process(request, services, claim).await;
            let _ = sender.send(result);
        }),
    ) {
        Ok(task) => task,
        Err(_) => return Ok(staged_outcome(DiscoveryStatus::Failed, "spawn_failed")),
    };
    let result = receiver
        .await
        .unwrap_or_else(|_| Ok(staged_outcome(DiscoveryStatus::Failed, "spawn_failed")));
    if task.join().await.is_err() {
        Ok(outcome(DiscoveryStatus::CleanupFailed))
    } else {
        result
    }
}

async fn probe_process(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: swallowtail_core::InterfaceCompatibilityClaim,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    let process = match services
        .process()
        .expect("validated process service is present")
        .start(
            request.scope_id().clone(),
            ProcessRequest::new(request.target().executable().clone())
                .with_arguments(["--version".to_owned()]),
        )
        .await
    {
        Ok(process) => process,
        Err(_) => return Ok(staged_outcome(DiscoveryStatus::Failed, "spawn_failed")),
    };
    if process.close_stdin().await.is_err() {
        return Ok(stop_and_classify(process.as_ref(), exit_failed(None, &[], false)).await);
    }

    let mut deadline = services
        .time()
        .expect("validated time service is present")
        .wait_until(request.deadline());
    let mut cancelled = request.cancellation().wait_requested();
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut stderr_truncated = false;
    loop {
        match next_output(process.as_ref(), &mut deadline, &mut cancelled).await {
            ProbeSignal::Cancelled => {
                return Ok(stop_and_classify(
                    process.as_ref(),
                    outcome(DiscoveryStatus::Cancelled),
                )
                .await);
            }
            ProbeSignal::TimedOut => {
                return Ok(
                    stop_and_classify(process.as_ref(), outcome(DiscoveryStatus::TimedOut)).await,
                );
            }
            ProbeSignal::Output(Err(_)) => {
                return Ok(stop_and_classify(
                    process.as_ref(),
                    staged_outcome(DiscoveryStatus::Failed, "output_failed"),
                )
                .await);
            }
            ProbeSignal::Output(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                if stdout.len().saturating_add(chunk.bytes().len()) > MAX_VERSION_OUTPUT_BYTES {
                    return Ok(stop_and_classify(
                        process.as_ref(),
                        staged_outcome(DiscoveryStatus::Malformed, "output_limit"),
                    )
                    .await);
                }
                stdout.extend_from_slice(chunk.bytes());
            }
            ProbeSignal::Output(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stderr =>
            {
                let remaining = MAX_VERSION_STDERR_BYTES.saturating_sub(stderr.len());
                let copied = remaining.min(chunk.bytes().len());
                stderr.extend_from_slice(&chunk.bytes()[..copied]);
                stderr_truncated |= copied < chunk.bytes().len();
            }
            ProbeSignal::Output(Ok(Some(_))) => {}
            ProbeSignal::Output(Ok(None)) => break,
        }
    }
    let exit = match process.wait().await {
        Ok(exit) => exit,
        Err(_) => return Ok(outcome(DiscoveryStatus::CleanupFailed)),
    };
    if !exit.success() {
        return Ok(exit_failed(exit.code(), &stderr, stderr_truncated));
    }
    let Some(binding) = parse_version(&stdout) else {
        return Ok(outcome(DiscoveryStatus::Malformed));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        crate::failure::failure(
            "swallowtail.antigravity.discovery_classification_failed",
            "Antigravity release observation could not be classified",
        )
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}
