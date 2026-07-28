use super::{KimiHeadlessPreparationInput, KimiHeadlessPreparedIntegration, instance};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, CredentialMechanism, DiscoveryOutcome, DiscoveryStatus, HostServiceKind,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn validate_input(
    input: &KimiHeadlessPreparationInput,
) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::KIMI_CODE_AXIS {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi.headless.preparation.target_axis_mismatch",
            "Kimi headless preparation target uses a different version axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::InteractiveOauth {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.headless.preparation.access_profile_rejected",
            "Kimi headless requires delegated membership OAuth access",
        ));
    }
    let _ = credential_reference(&input.access_profile)?;
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.headless.preparation.access_evidence_mismatch",
            "Kimi headless access evidence does not match the selected profile",
        ));
    }
    Ok(())
}

pub(super) fn promote(
    input: KimiHeadlessPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<KimiHeadlessPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
    {
        return Err(failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.kimi.headless.preparation.observation_mismatch",
            "Kimi headless discovery observation does not match the prepared target",
        ));
    }
    let configured = instance::configured_instance(&input, &observation)?;
    Ok(KimiHeadlessPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance: configured,
        available_host_services,
    })
}

pub(super) fn credential_reference(
    profile: &AccessProfile,
) -> Result<&swallowtail_core::CredentialRef, PreparationFailure> {
    profile.credential_reference().ok_or_else(|| {
        failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.headless.preparation.credential_reference_missing",
            "Kimi headless requires one delegated credential reference",
        )
    })
}

pub(super) fn discovery_runtime_failure(
    error: swallowtail_runtime::RuntimeFailure,
) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.kimi.discovery_axis_mismatch"
        | "swallowtail.installed_executable.host_services_missing"
        | "swallowtail.execution_host_mismatch" => PreparationStage::TargetSelection,
        _ => PreparationStage::ProcessSpawn,
    };
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
    )
}

fn discovery_outcome_failure(outcome: &DiscoveryOutcome) -> PreparationFailure {
    let stage = match outcome.status() {
        DiscoveryStatus::Malformed => PreparationStage::VersionParse,
        DiscoveryStatus::Incompatible => PreparationStage::CompatibilityClassification,
        DiscoveryStatus::CleanupFailed => PreparationStage::Cleanup,
        DiscoveryStatus::TimedOut | DiscoveryStatus::Cancelled => PreparationStage::BoundedOutput,
        _ => PreparationStage::ProcessSpawn,
    };
    let diagnostic = outcome.diagnostic().cloned().unwrap_or_else(|| {
        swallowtail_core::SafeDiagnostic::new(
            "swallowtail.kimi.headless.preparation.discovery_rejected",
            "Kimi headless executable discovery was not promotable",
        )
    });
    PreparationFailure::new(stage, swallowtail_core::Diagnostic::new(diagnostic))
}

pub(super) fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
