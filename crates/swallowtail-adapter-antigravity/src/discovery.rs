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

use crate::{
    AntigravityCatalogueDriver, AntigravityHeadlessDriver, antigravity_catalogue_claim,
    antigravity_headless_claim, antigravity_release_binding,
};

const MAX_VERSION_OUTPUT_BYTES: usize = 64;
const MAX_VERSION_STDERR_BYTES: usize = 1_024;
const MAX_SAFE_STDERR_CHARS: usize = 240;

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
    desired: DiscoveryOutcome,
) -> DiscoveryOutcome {
    let graceful = process.request_stop().await;
    let forced = process.force_stop().await;
    let waited = process.wait().await;
    if graceful.is_err() || forced.is_err() || waited.is_err() {
        outcome(DiscoveryStatus::CleanupFailed)
    } else {
        desired
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    antigravity_release_binding(value)
}

fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Antigravity installed discovery did not produce a compatible observation",
        )),
    )
}

fn staged_outcome(status: DiscoveryStatus, stage: &'static str) -> DiscoveryOutcome {
    let (code, message) = match stage {
        "spawn_failed" => (
            "swallowtail.antigravity.discovery_spawn_failed",
            "Antigravity version probe could not start",
        ),
        "output_failed" => (
            "swallowtail.antigravity.discovery_output_failed",
            "Antigravity version probe output could not be read",
        ),
        "output_limit" => (
            "swallowtail.antigravity.discovery_output_limit",
            "Antigravity version probe exceeded its output limit",
        ),
        _ => (
            "swallowtail.antigravity.discovery_failed",
            "Antigravity version probe failed",
        ),
    };
    DiscoveryOutcome::new(status, Some(SafeDiagnostic::new(code, message)))
}

fn exit_failed(
    exit_code: Option<i32>,
    stderr: &[u8],
    stderr_was_truncated: bool,
) -> DiscoveryOutcome {
    let mut message = match exit_code {
        Some(code) => format!("Antigravity version probe exited with status {code}"),
        None => "Antigravity version probe did not exit successfully".to_owned(),
    };
    if let Some(stderr) = sanitized_stderr(stderr, stderr_was_truncated) {
        message.push_str("; stderr: ");
        message.push_str(&stderr);
    }
    DiscoveryOutcome::new(
        DiscoveryStatus::Failed,
        Some(SafeDiagnostic::new(
            "swallowtail.antigravity.discovery_exit_failed",
            message,
        )),
    )
}

pub(crate) fn sanitized_stderr(stderr: &[u8], stderr_was_truncated: bool) -> Option<String> {
    let normalized = normalized_ascii(stderr);
    let mut excerpt = String::new();
    let mut truncated = stderr_was_truncated;
    for token in normalized.split_whitespace() {
        let token = if token_is_sensitive(token) {
            if token.contains('/') || token.contains('\\') {
                "<path>"
            } else {
                "<redacted>"
            }
        } else {
            token
        };
        let separator = usize::from(!excerpt.is_empty());
        let remaining = MAX_SAFE_STDERR_CHARS.saturating_sub(excerpt.chars().count());
        if separator + token.chars().count() > remaining {
            truncated = true;
            break;
        }
        if separator == 1 {
            excerpt.push(' ');
        }
        excerpt.push_str(token);
    }
    if excerpt.is_empty() {
        return None;
    }
    if truncated {
        excerpt.push_str(" [stderr truncated]");
    }
    Some(excerpt)
}

fn normalized_ascii(stderr: &[u8]) -> String {
    let mut normalized = String::new();
    let mut ansi_state = 0_u8;
    for character in String::from_utf8_lossy(stderr).chars() {
        match ansi_state {
            1 if character == '[' => ansi_state = 2,
            1 => ansi_state = 0,
            2 if ('@'..='~').contains(&character) => ansi_state = 0,
            2 => {}
            _ if character == '\u{1b}' => ansi_state = 1,
            _ if character.is_ascii_graphic() => normalized.push(character),
            _ if character.is_whitespace() => normalized.push(' '),
            _ => normalized.push('?'),
        }
    }
    normalized
}

fn token_is_sensitive(token: &str) -> bool {
    let token = token.trim_matches(|character: char| {
        matches!(
            character,
            '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';'
        )
    });
    let lower = token.to_ascii_lowercase();
    token.contains('/')
        || token.contains('\\')
        || token.contains('@')
        || token.contains('=')
        || token.chars().count() > 64
        || [
            "authorization",
            "api_key",
            "apikey",
            "bearer",
            "credential",
            "password",
            "secret",
            "token",
        ]
        .iter()
        .any(|shape| lower.contains(shape))
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.antigravity.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.antigravity.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.antigravity.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.antigravity.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.antigravity.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.antigravity.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.antigravity.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.antigravity.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::{exit_failed, parse_version};

    #[test]
    fn parser_accepts_only_bare_antigravity_semver() {
        assert_eq!(
            parse_version(b"1.1.9\n")
                .expect("version parses")
                .version()
                .as_str(),
            "1.1.9"
        );
        for output in [
            b"agy 1.1.9".as_slice(),
            b"1.1.9 extra".as_slice(),
            b" 1.1.9\n".as_slice(),
            b"1.1.9\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }

    #[test]
    fn exit_failure_keeps_only_status_and_sanitized_bounded_stderr() {
        let stderr = format!(
            "\u{1b}[31mwrapper failed at /Users/private/bin/agy \
             token=private user@example.com {}\u{1b}[0m",
            "detail ".repeat(80)
        );
        let outcome = exit_failed(Some(126), stderr.as_bytes(), false);
        let diagnostic = outcome.diagnostic().expect("failure is diagnosed");
        assert_eq!(
            diagnostic.code(),
            "swallowtail.antigravity.discovery_exit_failed"
        );
        assert!(diagnostic.message().contains("status 126"));
        assert!(diagnostic.message().contains("<path>"));
        assert!(diagnostic.message().contains("<redacted>"));
        assert!(diagnostic.message().contains("[stderr truncated]"));
        for private in [
            "/Users/private",
            "token=private",
            "user@example.com",
            "\u{1b}",
        ] {
            assert!(!diagnostic.message().contains(private));
        }
    }
}
