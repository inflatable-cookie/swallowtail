use swallowtail_core::{Diagnostic, DiscoveryOutcome, DiscoveryStatus, SafeDiagnostic};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

pub(super) fn discovery_runtime_failure(error: RuntimeFailure) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.claude_agent.discovery_axis_mismatch"
        | "swallowtail.installed_executable.host_services_missing"
        | "swallowtail.execution_host_mismatch" => PreparationStage::TargetSelection,
        _ => PreparationStage::ProcessSpawn,
    };
    PreparationFailure::new(stage, Diagnostic::new(error.diagnostic().clone()))
}

pub(super) fn discovery_outcome_failure(outcome: &DiscoveryOutcome) -> PreparationFailure {
    let stage = match outcome.status() {
        DiscoveryStatus::Malformed => PreparationStage::VersionParse,
        DiscoveryStatus::Incompatible => PreparationStage::CompatibilityClassification,
        DiscoveryStatus::CleanupFailed => PreparationStage::Cleanup,
        DiscoveryStatus::TimedOut | DiscoveryStatus::Cancelled => PreparationStage::BoundedOutput,
        _ => PreparationStage::ProcessSpawn,
    };
    let diagnostic = outcome.diagnostic().cloned().unwrap_or_else(|| {
        SafeDiagnostic::new(
            "swallowtail.claude_agent.preparation.discovery_rejected",
            "Claude Agent installed executable discovery was not promotable",
        )
    });
    PreparationFailure::new(stage, Diagnostic::new(diagnostic))
}

pub(super) fn preparation_failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(stage, Diagnostic::new(SafeDiagnostic::new(code, message)))
}
