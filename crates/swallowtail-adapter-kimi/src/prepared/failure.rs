use swallowtail_core::{Diagnostic, DiscoveryOutcome, DiscoveryStatus, SafeDiagnostic};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

pub(super) fn discovery_runtime_failure(error: RuntimeFailure) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.kimi.discovery_axis_mismatch"
        | "swallowtail.installed_executable.host_services_missing"
        | "swallowtail.execution_host_mismatch" => PreparationStage::TargetSelection,
        _ => PreparationStage::ProcessSpawn,
    };
    PreparationFailure::new(stage, Diagnostic::new(error.diagnostic().clone()))
}

pub(super) fn discovery_outcome_failure(outcome: &DiscoveryOutcome) -> PreparationFailure {
    let stage = match outcome
        .diagnostic()
        .map(SafeDiagnostic::code)
        .unwrap_or_default()
    {
        "swallowtail.kimi.discovery_timed_out" | "swallowtail.kimi.discovery_cancelled" => {
            PreparationStage::BoundedOutput
        }
        "swallowtail.kimi.discovery_malformed" => PreparationStage::VersionParse,
        "swallowtail.kimi.discovery_incompatible" => PreparationStage::CompatibilityClassification,
        "swallowtail.kimi.discovery_cleanup_failed" => PreparationStage::Cleanup,
        _ => match outcome.status() {
            DiscoveryStatus::Malformed => PreparationStage::VersionParse,
            DiscoveryStatus::Incompatible => PreparationStage::CompatibilityClassification,
            DiscoveryStatus::CleanupFailed => PreparationStage::Cleanup,
            DiscoveryStatus::TimedOut | DiscoveryStatus::Cancelled => {
                PreparationStage::BoundedOutput
            }
            _ => PreparationStage::ProcessSpawn,
        },
    };
    let diagnostic = outcome.diagnostic().cloned().unwrap_or_else(|| {
        SafeDiagnostic::new(
            "swallowtail.kimi.preparation.discovery_rejected",
            "Kimi installed executable discovery was not promotable",
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
