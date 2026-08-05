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
use crate::{OhMyPiRpcDriver, oh_my_pi_package_binding, oh_my_pi_rpc_claim};

const MAX_VERSION_OUTPUT_BYTES: usize = 64;

impl DiscoveryDriver for OhMyPiRpcDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.oh_my_pi.discovery_target_required",
                "OhMyPi discovery requires one explicit host-approved executable target",
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
    if request.target().version_axis() != oh_my_pi_rpc_claim().axis() {
        return Err(failure(
            "swallowtail.oh_my_pi.discovery_axis_mismatch",
            "OhMyPi discovery target uses a different version axis",
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
        &oh_my_pi_rpc_claim(),
    )
    .map_err(|_| {
        failure(
            "swallowtail.oh_my_pi.discovery_classification_failed",
            "OhMyPi version observation could not be classified",
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
    oh_my_pi_package_binding(value.strip_prefix("omp/")?)
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "OhMyPi installed discovery did not produce a compatible observation",
        )),
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.oh_my_pi.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.oh_my_pi.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.oh_my_pi.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.oh_my_pi.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.oh_my_pi.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.oh_my_pi.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.oh_my_pi.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.oh_my_pi.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_the_exact_omp_version_banner() {
        for candidate in ["17.2.9", "17.3.0"] {
            assert_eq!(
                parse_version(format!("omp/{candidate}\n").as_bytes())
                    .expect("version parses")
                    .version()
                    .as_str(),
                candidate
            );
        }
        for output in [
            b"17.2.9".as_slice(),
            b"omp 17.2.9".as_slice(),
            b"pi/17.2.9".as_slice(),
            b"17.2.9 extra".as_slice(),
            b" omp/17.2.9\n".as_slice(),
            b"omp/17.2.9\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
