use crate::selection::{codex_app_server_claim, codex_exec_claim};
use crate::{CodexAppServerDriver, CodexExecDriver};
use futures_channel::oneshot;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation, InterfaceCompatibilityClaim,
    InterfaceVersion, InterfaceVersionBinding, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, ProcessHandle, ProcessOutputStream, ProcessRequest,
    RuntimeFailure, validate_installed_executable_discovery_services,
};

const MAX_VERSION_OUTPUT_BYTES: usize = 128;

impl DiscoveryDriver for CodexExecDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        explicit_target_required()
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_joined(request, services, codex_exec_claim()))
    }
}

impl DiscoveryDriver for CodexAppServerDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        explicit_target_required()
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_joined(request, services, codex_app_server_claim()))
    }
}

fn explicit_target_required<T>() -> BoxFuture<'static, Result<T, RuntimeFailure>> {
    Box::pin(async {
        Err(failure(
            "swallowtail.codex.discovery_target_required",
            "Codex discovery requires one explicit host-approved executable target",
        ))
    })
}

async fn probe_joined(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: InterfaceCompatibilityClaim,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    if request.target().version_axis() != claim.axis() {
        return Err(failure(
            "swallowtail.codex.discovery_axis_mismatch",
            "Codex discovery target uses a different version axis",
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
        Err(_) => return Ok(outcome(DiscoveryStatus::Failed)),
    };
    let result = receiver
        .await
        .unwrap_or_else(|_| Ok(outcome(DiscoveryStatus::Failed)));
    if task.join().await.is_err() {
        Ok(outcome(DiscoveryStatus::CleanupFailed))
    } else {
        result
    }
}

async fn probe_process(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: InterfaceCompatibilityClaim,
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
        Err(_) => return Ok(outcome(DiscoveryStatus::Failed)),
    };
    if process.close_stdin().await.is_err() {
        return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::Failed).await);
    }

    let mut deadline = services
        .time()
        .expect("validated time service is present")
        .wait_until(request.deadline());
    let mut cancelled = request.cancellation().wait_requested();
    let mut stdout = Vec::new();
    loop {
        match next_output(process.as_ref(), &mut deadline, &mut cancelled).await {
            ProbeSignal::Cancelled => {
                return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::Cancelled).await);
            }
            ProbeSignal::TimedOut => {
                return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::TimedOut).await);
            }
            ProbeSignal::Output(Err(_)) => {
                return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::Failed).await);
            }
            ProbeSignal::Output(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                if stdout.len().saturating_add(chunk.bytes().len()) > MAX_VERSION_OUTPUT_BYTES {
                    return Ok(
                        stop_and_classify(process.as_ref(), DiscoveryStatus::Malformed).await,
                    );
                }
                stdout.extend_from_slice(chunk.bytes());
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
        return Ok(outcome(DiscoveryStatus::Failed));
    }
    let Some(binding) = parse_version(&stdout, claim.axis().clone()) else {
        return Ok(outcome(DiscoveryStatus::Malformed));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        failure(
            "swallowtail.codex.discovery_classification_failed",
            "Codex version observation could not be classified",
        )
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}

enum ProbeSignal {
    Output(Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure>),
    TimedOut,
    Cancelled,
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancelled: &mut BoxFuture<'static, ()>,
) -> ProbeSignal {
    let mut output = process.read_output();
    poll_fn(|context| {
        if cancelled.as_mut().poll(context).is_ready() {
            return Poll::Ready(ProbeSignal::Cancelled);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(ProbeSignal::TimedOut);
        }
        output.as_mut().poll(context).map(ProbeSignal::Output)
    })
    .await
}

async fn stop_and_classify(
    process: &dyn ProcessHandle,
    status: DiscoveryStatus,
) -> DiscoveryOutcome {
    let graceful = process.request_stop().await;
    let forced = process.force_stop().await;
    let waited = process.wait().await;
    if graceful.is_err() || forced.is_err() || waited.is_err() {
        outcome(DiscoveryStatus::CleanupFailed)
    } else {
        outcome(status)
    }
}

fn parse_version(
    output: &[u8],
    axis: swallowtail_core::InterfaceVersionAxis,
) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?.trim();
    let version = output.strip_prefix("codex-cli ")?;
    if version.is_empty()
        || version.bytes().any(|byte| byte.is_ascii_whitespace())
        || semver::Version::parse(version).is_err()
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis,
        InterfaceVersion::new(version).ok()?,
    ))
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Codex installed executable discovery did not produce a compatible observation",
        )),
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.codex.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.codex.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.codex.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.codex.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.codex.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.codex.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.codex.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.codex.discovery_cleanup_failed",
    }
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
#[path = "discovery_tests.rs"]
mod tests;
