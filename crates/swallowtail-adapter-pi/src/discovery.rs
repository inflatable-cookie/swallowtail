use futures_channel::oneshot;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, ProcessHandle, ProcessOutputStream, ProcessRequest,
    RuntimeFailure, validate_installed_executable_discovery_services,
};

use crate::failure::failure;
use crate::{PiRpcDriver, pi_package_binding, pi_rpc_claim};

const MAX_VERSION_OUTPUT_BYTES: usize = 64;

impl DiscoveryDriver for PiRpcDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.pi.discovery_target_required",
                "Pi discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_joined(request, services))
    }
}

async fn probe_joined(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    if request.target().version_axis() != pi_rpc_claim().axis() {
        return Err(failure(
            "swallowtail.pi.discovery_axis_mismatch",
            "Pi discovery target uses a different version axis",
        ));
    }
    if request.cancellation().is_requested() {
        return Ok(outcome(DiscoveryStatus::Cancelled));
    }
    let task_service = services.task().expect("validated task service").clone();
    let scope = request.scope_id().clone();
    let (sender, receiver) = oneshot::channel();
    let task = match task_service.spawn(
        scope,
        Box::pin(async move {
            let result = probe_process(request, services).await;
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
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    let process = match services
        .process()
        .expect("validated process service")
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
        .expect("validated time service")
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
    let Some(binding) = parse_version(&stdout) else {
        return Ok(outcome(DiscoveryStatus::Malformed));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &pi_rpc_claim(),
    )
    .map_err(|_| {
        failure(
            "swallowtail.pi.discovery_classification_failed",
            "Pi version observation could not be classified",
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

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    pi_package_binding(value)
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Pi installed discovery did not produce a compatible observation",
        )),
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.pi.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.pi.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.pi.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.pi.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.pi.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.pi.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.pi.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.pi.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_bare_pi_semver() {
        for candidate in ["0.80.10", "0.83.0"] {
            assert_eq!(
                parse_version(format!("{candidate}\n").as_bytes())
                    .expect("version parses")
                    .version()
                    .as_str(),
                candidate
            );
        }
        for output in [
            b"pi 0.80.10".as_slice(),
            b"0.80.10 extra".as_slice(),
            b" 0.80.10\n".as_slice(),
            b"0.80.10\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
