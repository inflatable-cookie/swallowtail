use crate::failure::failure;
use crate::{
    gemini_cli_acp_binding, gemini_cli_acp_claim, gemini_cli_headless_binding,
    gemini_cli_headless_claim,
};
use futures_channel::oneshot;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InstalledExecutableDiscoveryRequest, ProcessHandle,
    ProcessOutputStream, ProcessRequest, RuntimeFailure,
    validate_installed_executable_discovery_services,
};

const MAX_VERSION_OUTPUT_BYTES: usize = 64;

#[derive(Clone, Copy)]
pub(super) enum ProbeRoute {
    Acp,
    Headless,
}

impl ProbeRoute {
    fn claim(self) -> swallowtail_core::InterfaceCompatibilityClaim {
        match self {
            Self::Acp => gemini_cli_acp_claim(),
            Self::Headless => gemini_cli_headless_claim(),
        }
    }

    fn binding(self, value: &str) -> Option<swallowtail_core::InterfaceVersionBinding> {
        match self {
            Self::Acp => gemini_cli_acp_binding(value),
            Self::Headless => gemini_cli_headless_binding(value),
        }
    }
}

pub(super) async fn probe_joined(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    route: ProbeRoute,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    let claim = route.claim();
    if request.target().version_axis() != claim.axis() {
        return Err(failure(
            "swallowtail.gemini.discovery_axis_mismatch",
            "Gemini discovery target uses a different version axis",
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
            let result = probe_process(request, services, route).await;
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
    route: ProbeRoute,
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
    let Some(binding) = parse_version(&stdout, route) else {
        return Ok(outcome(DiscoveryStatus::Malformed));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &route.claim(),
    )
    .map_err(|_| {
        failure(
            "swallowtail.gemini.discovery_classification_failed",
            "Gemini version observation could not be classified",
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
    route: ProbeRoute,
) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    route.binding(value)
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Gemini installed discovery did not produce a compatible observation",
        )),
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.gemini.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.gemini.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.gemini.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.gemini.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.gemini.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.gemini.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.gemini.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.gemini.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::{ProbeRoute, parse_version};

    #[test]
    fn parser_accepts_only_the_bare_cli_semver() {
        assert_eq!(
            parse_version(b"0.51.0\n", ProbeRoute::Acp)
                .expect("version parses")
                .version()
                .as_str(),
            "0.51.0"
        );
        for output in [
            b"gemini 0.51.0".as_slice(),
            b"0.51.0 extra".as_slice(),
            b" 0.51.0\n".as_slice(),
            b"0.51.0\n\n".as_slice(),
            b"latest".as_slice(),
        ] {
            assert!(parse_version(output, ProbeRoute::Acp).is_none());
        }
        assert_eq!(
            parse_version(b"0.52.0\n", ProbeRoute::Headless)
                .expect("headless version parses")
                .version()
                .as_str(),
            "0.52.0"
        );
    }
}
