use super::{GeminiHeadlessPreparationInput, GeminiHeadlessPreparedIntegration, instance};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, CredentialMechanism, DiscoveryOutcome, DiscoveryStatus, EntitlementMetering,
    HostServiceKind, SupportAuthority,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

const ENDPOINT_AUDIENCE: &str = "gemini-developer-api";

pub(super) fn validate_input(
    input: &GeminiHeadlessPreparationInput,
) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::GEMINI_CLI_HEADLESS_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.gemini.headless.preparation.target_axis_mismatch",
            "Gemini headless preparation target uses a different version axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.gemini.headless.preparation.access_profile_rejected",
            "Gemini headless requires its provider-supported Developer API-key profile",
        ));
    }
    let _ = credential_reference(&input.access_profile)?;
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.gemini.headless.preparation.access_evidence_mismatch",
            "Gemini headless access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

pub(super) fn promote(
    input: GeminiHeadlessPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<GeminiHeadlessPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
    {
        return Err(preparation_failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.gemini.headless.preparation.observation_mismatch",
            "Gemini headless discovery observation does not match the prepared target",
        ));
    }
    let instance = instance::configured_instance(&input, &observation)?;
    Ok(GeminiHeadlessPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}

pub(super) fn credential_reference(
    profile: &AccessProfile,
) -> Result<&swallowtail_core::CredentialRef, PreparationFailure> {
    profile.credential_reference().ok_or_else(|| {
        preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.gemini.headless.preparation.credential_reference_missing",
            "Gemini headless requires one Developer API-key credential reference",
        )
    })
}

pub(super) fn discovery_runtime_failure(
    error: swallowtail_runtime::RuntimeFailure,
) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.gemini.discovery_axis_mismatch"
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
            "swallowtail.gemini.headless.preparation.discovery_rejected",
            "Gemini headless executable discovery was not promotable",
        )
    });
    PreparationFailure::new(stage, swallowtail_core::Diagnostic::new(diagnostic))
}

pub(super) fn preparation_failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
