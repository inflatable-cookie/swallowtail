use futures_channel::oneshot;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{
    CredentialRef, DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation,
    InterfaceCompatibilityClaim, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, ProcessHandle, ProcessOutputStream, ProcessRequest,
    RuntimeFailure, validate_installed_executable_discovery_services,
};

use crate::failure::failure;
use crate::{grok_build_acp_binding, grok_build_acp_claim};

const MAX_VERSION_OUTPUT_BYTES: usize = 96;
const QUALIFIED_SOURCE_REVISIONS: [(&str, &str); 6] = [
    ("0.2.114", "0c785038798"),
    ("0.2.115", "dd16b5eb7d50"),
    ("0.2.116", "99b387d2cc0e"),
    ("0.2.117", "f1c06093089f"),
    ("1.0.4", "d846eb93d94d"),
    ("1.0.5", "5115b46bc909"),
];

/// Low-level installed discovery and ACP operation driver for Grok Build.
pub struct GrokAcpDriver {
    ambient_environment: EnvironmentRef,
    credential: CredentialRef,
}

impl GrokAcpDriver {
    /// Creates a driver with its approved environment and opaque OAuth credential.
    #[must_use]
    pub const fn new(ambient_environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            ambient_environment,
            credential,
        }
    }

    /// Returns the approved ambient execution environment.
    #[must_use]
    pub const fn ambient_environment(&self) -> &EnvironmentRef {
        &self.ambient_environment
    }

    /// Returns the opaque credential reference used for token activation.
    #[must_use]
    pub const fn credential(&self) -> &CredentialRef {
        &self.credential
    }
}

impl DiscoveryDriver for GrokAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.grok.discovery_target_required",
                "Grok discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe_joined(request, services, grok_build_acp_claim()))
    }
}

async fn probe_joined(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: InterfaceCompatibilityClaim,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    if request.target().version_axis() != claim.axis() {
        return Err(failure(
            "swallowtail.grok.discovery_axis_mismatch",
            "Grok discovery target uses a different version axis",
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
                .with_arguments(["--no-auto-update".to_owned(), "--version".to_owned()]),
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
    let Some(binding) = parse_version(&stdout) else {
        return Ok(outcome(DiscoveryStatus::Malformed));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        failure(
            "swallowtail.grok.discovery_classification_failed",
            "Grok version observation could not be classified",
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
    let output = output.strip_suffix('\n').unwrap_or(output);
    let prefix = "grok ";
    let suffix = " [stable]";
    let core = output.strip_prefix(prefix)?.strip_suffix(suffix)?;
    let (version, revision) = core.split_once(" (")?;
    let revision = revision.strip_suffix(')')?;
    if !(7..=40).contains(&revision.len()) || !revision.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return None;
    }
    if qualified_source_revision(version).is_some_and(|expected| revision != expected) {
        return None;
    }
    grok_build_acp_binding(version)
}

fn qualified_source_revision(version: &str) -> Option<&'static str> {
    QUALIFIED_SOURCE_REVISIONS
        .iter()
        .find_map(|(candidate, revision)| (*candidate == version).then_some(*revision))
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Grok installed executable discovery did not produce a compatible observation",
        )),
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.grok.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.grok.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.grok.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.grok.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.grok.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.grok.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.grok.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.grok.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_requires_stable_channel_and_every_exact_qualified_revision() {
        for (output, version) in [
            ("grok 0.2.114 (0c785038798) [stable]\n", "0.2.114"),
            ("grok 0.2.115 (dd16b5eb7d50) [stable]\n", "0.2.115"),
            ("grok 0.2.116 (99b387d2cc0e) [stable]\n", "0.2.116"),
            ("grok 0.2.117 (f1c06093089f) [stable]\n", "0.2.117"),
            ("grok 1.0.4 (d846eb93d94d) [stable]\n", "1.0.4"),
            ("grok 1.0.5 (5115b46bc909) [stable]\n", "1.0.5"),
            ("grok 0.2.118 (123456789abc) [stable]\n", "0.2.118"),
            ("grok 1.0.6 (abcdef123456) [stable]\n", "1.0.6"),
        ] {
            assert_eq!(
                parse_version(output.as_bytes())
                    .expect("exact release parses")
                    .version()
                    .as_str(),
                version
            );
        }
        for output in [
            b"grok 0.2.114 (wrongsource) [stable]\n".as_slice(),
            b"grok 0.2.115 (0c785038798) [stable]\n".as_slice(),
            b"grok 0.2.116 (dd16b5eb7d50) [stable]\n".as_slice(),
            b"grok 0.2.117 (99b387d2cc0e) [stable]\n".as_slice(),
            b"grok 1.0.4 (f1c06093089f) [stable]\n".as_slice(),
            b"grok 1.0.5 (d846eb93d94d) [stable]\n".as_slice(),
            b"grok 0.2.114 (0c785038798) [alpha]\n".as_slice(),
            b"grok 0.2.114 (0c785038798)\n".as_slice(),
            b"0.2.114\n".as_slice(),
            b"grok 0.2.114-alpha.1 (123456789ab) [stable]\n".as_slice(),
            b"grok 0.2.114 (0c785038798) [stable]\nprivate".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
