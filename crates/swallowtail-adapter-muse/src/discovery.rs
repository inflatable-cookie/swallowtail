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

use crate::{MuseHeadlessDriver, muse_code_release_binding, muse_headless_claim};

const MAXIMUM_VERSION_OUTPUT_BYTES: usize = 64;

impl DiscoveryDriver for MuseHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.muse_code.discovery_target_required",
                "Muse Code discovery requires one explicit signed payload target",
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
    if request.target().version_axis().as_str() != crate::MUSE_CODE_RELEASE_AXIS {
        return Err(crate::failure::failure(
            "swallowtail.muse_code.discovery_axis_mismatch",
            "Muse Code discovery target uses a different signed payload axis",
        ));
    }
    if !crate::selection::is_versioned_payload_target(request.target().executable().as_host_value())
    {
        return Err(crate::failure::failure(
            "swallowtail.muse_code.discovery_launcher_rejected",
            "Muse Code discovery requires the exact versioned payload target",
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
        return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::CleanupFailed).await);
    }
    let mut deadline = services
        .time()
        .expect("validated time service")
        .wait_until(request.deadline());
    let mut cancelled = request.cancellation().wait_requested();
    let mut stdout = Vec::new();
    loop {
        match next_output(process.as_ref(), &mut deadline, &mut cancelled).await {
            Signal::Cancelled => {
                return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::Cancelled).await);
            }
            Signal::TimedOut => {
                return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::TimedOut).await);
            }
            Signal::Output(Err(_)) => {
                return Ok(stop_and_classify(process.as_ref(), DiscoveryStatus::Failed).await);
            }
            Signal::Output(Ok(Some(chunk))) if chunk.stream() == ProcessOutputStream::Stdout => {
                if stdout.len().saturating_add(chunk.bytes().len()) > MAXIMUM_VERSION_OUTPUT_BYTES {
                    return Ok(
                        stop_and_classify(process.as_ref(), DiscoveryStatus::Malformed).await,
                    );
                }
                stdout.extend_from_slice(chunk.bytes());
            }
            Signal::Output(Ok(Some(_))) => {}
            Signal::Output(Ok(None)) => break,
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
        &muse_headless_claim(),
    )
    .map_err(|_| {
        crate::failure::failure(
            "swallowtail.muse_code.discovery_classification_failed",
            "Muse Code payload observation could not be classified",
        )
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}

enum Signal {
    Output(Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure>),
    TimedOut,
    Cancelled,
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancelled: &mut BoxFuture<'static, ()>,
) -> Signal {
    let mut output = process.read_output();
    poll_fn(|context| {
        if cancelled.as_mut().poll(context).is_ready() {
            return Poll::Ready(Signal::Cancelled);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Signal::TimedOut);
        }
        output.as_mut().poll(context).map(Signal::Output)
    })
    .await
}

async fn stop_and_classify(
    process: &dyn ProcessHandle,
    desired: DiscoveryStatus,
) -> DiscoveryOutcome {
    let stop = process.force_stop().await;
    let wait = process.wait().await;
    if stop.is_err() || wait.is_err() {
        outcome(DiscoveryStatus::CleanupFailed)
    } else {
        outcome(desired)
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let revision = exact.strip_prefix("Muse Code 0.1.0 (")?.strip_suffix(')')?;
    muse_code_release_binding(revision)
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Muse Code discovery did not produce the exact signed payload observation",
        )),
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.muse_code.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.muse_code.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.muse_code.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.muse_code.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.muse_code.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.muse_code.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.muse_code.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.muse_code.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_requires_the_exact_direct_payload_version_line() {
        assert_eq!(
            parse_version(b"Muse Code 0.1.0 (0.1.0-R708.1)\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "0.1.0-R708.1"
        );
        for rejected in [
            b"Muse Code 0.1.0 (0.1.0-R708.2)\n".as_slice(),
            b"muse 0.1.0-R708.1\n".as_slice(),
            b"Muse Code 0.1.0 (0.1.0-R708.1) extra\n".as_slice(),
        ] {
            assert!(parse_version(rejected).is_none());
        }
    }
}
