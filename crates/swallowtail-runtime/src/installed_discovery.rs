//! Shared installed-executable discovery probe, version binding parse, and
//! preparation-stage mapping.
//!
//! Provider-neutral machinery owned by the runtime so adapters stop
//! re-implementing the same bounded probe loop, semantic version binding
//! validation, and discovery failure classification. The adapter supplies
//! its claim, its exact version parser, its diagnostic-code namespace, and
//! its solution label; everything else lives here.

#![deny(missing_docs)]

use crate::{
    BoxFuture, DebugObservationKind, HostServices, InstalledExecutableDiscoveryRequest,
    PreparationFailure, PreparationStage, ProcessHandle, ProcessOutputStream, ProcessRequest,
    RuntimeFailure, validate_installed_executable_discovery_services,
};
use futures_channel::oneshot;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{
    Diagnostic, DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation,
    InterfaceCompatibilityClaim, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    SafeDiagnostic,
};

/// Maximum captured installed-executable version-output bytes.
pub const MAX_VERSION_OUTPUT_BYTES: usize = 64;

/// Maximum accepted observed version text in a semantic binding parse.
pub const MAX_VERSION_BYTES: usize = 64;

/// One adapter's static discovery diagnostic-code namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InstalledProbeCodes {
    /// Absent-target outcome code.
    pub absent: &'static str,
    /// Discovered outcome code.
    pub discovered: &'static str,
    /// Incompatible outcome code.
    pub incompatible: &'static str,
    /// Malformed outcome code.
    pub malformed: &'static str,
    /// Timed-out outcome code.
    pub timed_out: &'static str,
    /// Cancelled outcome code.
    pub cancelled: &'static str,
    /// Failed outcome code.
    pub failed: &'static str,
    /// Cleanup-failed outcome code.
    pub cleanup_failed: &'static str,
    /// Axis-mismatch preparation code.
    pub axis_mismatch: &'static str,
    /// Classification-failed preparation code.
    pub classification_failed: &'static str,
}

/// Builds one [`InstalledProbeCodes`] namespace from a literal adapter prefix.
///
/// The prefix must be a string literal such as `"swallowtail.pi"` so the
/// codes stay compile-time constants.
#[macro_export]
macro_rules! installed_probe_codes {
    ($prefix:literal) => {
        $crate::InstalledProbeCodes {
            absent: concat!($prefix, ".discovery_absent"),
            discovered: concat!($prefix, ".discovery_discovered"),
            incompatible: concat!($prefix, ".discovery_incompatible"),
            malformed: concat!($prefix, ".discovery_malformed"),
            timed_out: concat!($prefix, ".discovery_timed_out"),
            cancelled: concat!($prefix, ".discovery_cancelled"),
            failed: concat!($prefix, ".discovery_failed"),
            cleanup_failed: concat!($prefix, ".discovery_cleanup_failed"),
            axis_mismatch: concat!($prefix, ".discovery_axis_mismatch"),
            classification_failed: concat!($prefix, ".discovery_classification_failed"),
        }
    };
}

/// Parses one exact semantic-version binding for an axis.
///
/// Returns `None` for blank, oversized, trimmed, control-character, or
/// non-semantic text, so provider-observed version strings never panic a
/// caller.
#[must_use]
pub fn parse_semantic_version_binding(
    axis: &InterfaceVersionAxis,
    value: &str,
) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
        || semver::Version::parse(value).is_err()
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis.clone(),
        InterfaceVersion::new(value).ok()?,
    ))
}

/// Runs one bounded installed-executable version probe to a classified outcome.
///
/// Validates host services and the target axis, spawns the probe on the host
/// task service, captures bounded stdout under the deadline and cancellation
/// signals, stops and joins the child on every path, and classifies the
/// observation against `claim`. `parse` decodes the raw version output.
pub async fn probe_installed_executable_version(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: InterfaceCompatibilityClaim,
    parse: impl Fn(&[u8]) -> Option<InterfaceVersionBinding> + Send + 'static,
    codes: InstalledProbeCodes,
    solution: &'static str,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_installed_executable_discovery_services(&request, &services)?;
    if request.target().version_axis() != claim.axis() {
        let message = format!("{solution} discovery target uses a different version axis");
        services.emit_failure_debug(
            DebugObservationKind::InterfaceVersion,
            solution,
            "installed_discovery.axis",
            codes.axis_mismatch,
            message.as_str(),
        );
        return Err(failure(codes.axis_mismatch, message));
    }
    if request.cancellation().is_requested() {
        return Ok(outcome(
            &services,
            codes,
            solution,
            DiscoveryStatus::Cancelled,
        ));
    }
    let task_service = services.task().expect("validated task service").clone();
    let scope = request.scope_id().clone();
    let (sender, receiver) = oneshot::channel();
    let probe_services = services.clone();
    let task = match task_service.spawn(
        scope,
        Box::pin(async move {
            let result =
                probe_process(&request, probe_services, claim, parse, codes, solution).await;
            let _ = sender.send(result);
        }),
    ) {
        Ok(task) => task,
        Err(_) => {
            return Ok(outcome(
                &services,
                codes,
                solution,
                DiscoveryStatus::Failed,
            ));
        }
    };
    let result = receiver.await.unwrap_or_else(|_| {
        Ok(outcome(
            &services,
            codes,
            solution,
            DiscoveryStatus::Failed,
        ))
    });
    if task.join().await.is_err() {
        Ok(outcome(
            &services,
            codes,
            solution,
            DiscoveryStatus::CleanupFailed,
        ))
    } else {
        result
    }
}

async fn probe_process(
    request: &InstalledExecutableDiscoveryRequest,
    services: HostServices,
    claim: InterfaceCompatibilityClaim,
    parse: impl Fn(&[u8]) -> Option<InterfaceVersionBinding>,
    codes: InstalledProbeCodes,
    solution: &'static str,
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
        Err(_) => {
            return Ok(outcome(
                &services,
                codes,
                solution,
                DiscoveryStatus::Failed,
            ));
        }
    };
    if process.close_stdin().await.is_err() {
        return Ok(stop_and_classify(
            process.as_ref(),
            &services,
            codes,
            solution,
            DiscoveryStatus::Failed,
        )
        .await);
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
                return Ok(stop_and_classify(
                    process.as_ref(),
                    &services,
                    codes,
                    solution,
                    DiscoveryStatus::Cancelled,
                )
                .await);
            }
            ProbeSignal::TimedOut => {
                return Ok(stop_and_classify(
                    process.as_ref(),
                    &services,
                    codes,
                    solution,
                    DiscoveryStatus::TimedOut,
                )
                .await);
            }
            ProbeSignal::Output(Err(_)) => {
                return Ok(stop_and_classify(
                    process.as_ref(),
                    &services,
                    codes,
                    solution,
                    DiscoveryStatus::Failed,
                )
                .await);
            }
            ProbeSignal::Output(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                if stdout.len().saturating_add(chunk.bytes().len()) > MAX_VERSION_OUTPUT_BYTES {
                    return Ok(stop_and_classify(
                        process.as_ref(),
                        &services,
                        codes,
                        solution,
                        DiscoveryStatus::Malformed,
                    )
                    .await);
                }
                stdout.extend_from_slice(chunk.bytes());
            }
            ProbeSignal::Output(Ok(Some(_))) => {}
            ProbeSignal::Output(Ok(None)) => break,
        }
    }
    let exit = match process.wait().await {
        Ok(exit) => exit,
        Err(_) => {
            return Ok(outcome(
                &services,
                codes,
                solution,
                DiscoveryStatus::CleanupFailed,
            ));
        }
    };
    if !exit.success() {
        return Ok(outcome(
            &services,
            codes,
            solution,
            DiscoveryStatus::Failed,
        ));
    }
    let Some(binding) = parse(&stdout) else {
        return Ok(outcome(
            &services,
            codes,
            solution,
            DiscoveryStatus::Malformed,
        ));
    };
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        let message = format!("{solution} version observation could not be classified");
        services.emit_failure_debug(
            DebugObservationKind::InterfaceVersion,
            solution,
            "installed_discovery.classify",
            codes.classification_failed,
            message.as_str(),
        );
        failure(codes.classification_failed, message)
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}

enum ProbeSignal {
    Output(Result<Option<crate::ProcessOutputChunk>, RuntimeFailure>),
    TimedOut,
    Cancelled,
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut BoxFuture<'static, crate::DeadlineObservation>,
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
    services: &HostServices,
    codes: InstalledProbeCodes,
    solution: &'static str,
    status: DiscoveryStatus,
) -> DiscoveryOutcome {
    let graceful = process.request_stop().await;
    let forced = process.force_stop().await;
    let waited = process.wait().await;
    if graceful.is_err() || forced.is_err() || waited.is_err() {
        outcome(
            services,
            codes,
            solution,
            DiscoveryStatus::CleanupFailed,
        )
    } else {
        outcome(services, codes, solution, status)
    }
}

fn outcome(
    services: &HostServices,
    codes: InstalledProbeCodes,
    solution: &'static str,
    status: DiscoveryStatus,
) -> DiscoveryOutcome {
    let code = match status {
        DiscoveryStatus::Absent => codes.absent,
        DiscoveryStatus::Discovered => codes.discovered,
        DiscoveryStatus::Incompatible => codes.incompatible,
        DiscoveryStatus::Malformed => codes.malformed,
        DiscoveryStatus::TimedOut => codes.timed_out,
        DiscoveryStatus::Cancelled => codes.cancelled,
        DiscoveryStatus::Failed => codes.failed,
        DiscoveryStatus::CleanupFailed => codes.cleanup_failed,
    };
    let message =
        format!("{solution} installed discovery did not produce a compatible observation");
    let kind = match status {
        DiscoveryStatus::Malformed | DiscoveryStatus::Incompatible => {
            DebugObservationKind::InterfaceVersion
        }
        DiscoveryStatus::CleanupFailed => DebugObservationKind::Cleanup,
        DiscoveryStatus::Failed
        | DiscoveryStatus::TimedOut
        | DiscoveryStatus::Cancelled
        | DiscoveryStatus::Absent => DebugObservationKind::HostProcess,
        DiscoveryStatus::Discovered => DebugObservationKind::Lifecycle,
    };
    services.emit_failure_debug(
        kind,
        solution,
        "installed_discovery.probe",
        code,
        message.as_str(),
    );
    DiscoveryOutcome::new(status, Some(SafeDiagnostic::new(code, message)))
}

fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

/// Maps one probe runtime failure to its preparation stage.
#[must_use]
pub fn probe_runtime_failure(
    error: &RuntimeFailure,
    axis_mismatch_code: &'static str,
) -> PreparationFailure {
    let stage = if error.diagnostic().code() == axis_mismatch_code
        || matches!(
            error.diagnostic().code(),
            "swallowtail.installed_executable.host_services_missing"
                | "swallowtail.execution_host_mismatch"
        ) {
        PreparationStage::TargetSelection
    } else {
        PreparationStage::ProcessSpawn
    };
    PreparationFailure::new(stage, Diagnostic::new(error.diagnostic().clone()))
}

/// Maps one probe outcome to its preparation stage and failure record.
#[must_use]
pub fn probe_outcome_failure(
    outcome: &DiscoveryOutcome,
    rejected_code: &'static str,
    rejected_message: &'static str,
) -> PreparationFailure {
    let stage = match outcome.status() {
        DiscoveryStatus::Malformed => PreparationStage::VersionParse,
        DiscoveryStatus::Incompatible => PreparationStage::CompatibilityClassification,
        DiscoveryStatus::CleanupFailed => PreparationStage::Cleanup,
        DiscoveryStatus::TimedOut | DiscoveryStatus::Cancelled => PreparationStage::BoundedOutput,
        _ => PreparationStage::ProcessSpawn,
    };
    let diagnostic = outcome
        .diagnostic()
        .cloned()
        .unwrap_or_else(|| SafeDiagnostic::new(rejected_code, rejected_message));
    PreparationFailure::new(stage, Diagnostic::new(diagnostic))
}

/// Creates one preparation failure for an exact stage and safe diagnostic.
#[must_use]
pub fn preparation_failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(stage, Diagnostic::new(SafeDiagnostic::new(code, message)))
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_VERSION_BYTES, parse_semantic_version_binding, probe_outcome_failure,
        probe_runtime_failure,
    };
    use crate::{PreparationStage, RuntimeFailure};
    use swallowtail_core::{
        DiscoveryOutcome, DiscoveryStatus, InterfaceVersionAxis, SafeDiagnostic,
    };

    fn axis() -> InterfaceVersionAxis {
        InterfaceVersionAxis::new("fixture.axis").expect("axis is valid")
    }

    fn failure(code: &'static str) -> RuntimeFailure {
        RuntimeFailure::new(SafeDiagnostic::new(code, "fixture failure"))
    }

    #[test]
    fn semantic_binding_accepts_only_bare_semver_text() {
        for value in ["0.80.10", "1.2.3", "0.33.0"] {
            let binding =
                parse_semantic_version_binding(&axis(), value).expect("bare semver binds");
            assert_eq!(binding.version().as_str(), value);
        }
        let long = format!("1.0.{}", "0".repeat(MAX_VERSION_BYTES + 1));
        for value in [
            "",
            "   ",
            "0.80.10 extra",
            " 0.80.10",
            "0.80.10\n",
            "v0.80.10",
            "pi/0.80.10",
            long.as_str(),
        ] {
            assert!(
                parse_semantic_version_binding(&axis(), value).is_none(),
                "must reject {value:?}"
            );
        }
    }

    #[test]
    fn runtime_failure_maps_axis_and_host_codes_to_target_selection() {
        for code in [
            "fixture.discovery_axis_mismatch",
            "swallowtail.installed_executable.host_services_missing",
            "swallowtail.execution_host_mismatch",
        ] {
            let mapped = probe_runtime_failure(&failure(code), "fixture.discovery_axis_mismatch");
            assert_eq!(mapped.stage(), PreparationStage::TargetSelection);
        }
        let other =
            probe_runtime_failure(&failure("fixture.other"), "fixture.discovery_axis_mismatch");
        assert_eq!(other.stage(), PreparationStage::ProcessSpawn);
    }

    #[test]
    fn outcome_failure_maps_statuses_to_stages() {
        let cases = [
            (DiscoveryStatus::Malformed, PreparationStage::VersionParse),
            (
                DiscoveryStatus::Incompatible,
                PreparationStage::CompatibilityClassification,
            ),
            (DiscoveryStatus::CleanupFailed, PreparationStage::Cleanup),
            (DiscoveryStatus::TimedOut, PreparationStage::BoundedOutput),
            (DiscoveryStatus::Cancelled, PreparationStage::BoundedOutput),
            (DiscoveryStatus::Failed, PreparationStage::ProcessSpawn),
            (DiscoveryStatus::Absent, PreparationStage::ProcessSpawn),
            (DiscoveryStatus::Discovered, PreparationStage::ProcessSpawn),
        ];
        for (status, expected) in cases {
            let outcome = DiscoveryOutcome::new(status, None);
            let mapped = probe_outcome_failure(&outcome, "fixture.rejected", "fixture rejected");
            assert_eq!(mapped.stage(), expected);
            assert_eq!(
                mapped.diagnostic().safe().code(),
                "fixture.rejected",
                "fallback diagnostic is used when the outcome has none"
            );
        }
    }

    #[test]
    fn outcome_failure_keeps_the_exact_outcome_diagnostic() {
        let diagnostic = SafeDiagnostic::new("fixture.exact", "exact failure");
        let outcome = DiscoveryOutcome::new(DiscoveryStatus::Malformed, Some(diagnostic));
        let mapped = probe_outcome_failure(&outcome, "fixture.rejected", "fixture rejected");
        assert_eq!(mapped.diagnostic().safe().code(), "fixture.exact");
    }
}
