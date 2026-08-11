use super::{ClaudeCodeResponsePreparationInput, ClaudeCodeResponsePreparedIntegration, instance};
use std::collections::BTreeSet;
use swallowtail_core::{
    CredentialMechanism, CredentialState, DiscoveryOutcome, DiscoveryStatus, EntitlementMetering,
    HostServiceKind, SupportAuthority,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn validate_input(
    input: &ClaudeCodeResponsePreparationInput,
) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::CLAUDE_CODE_RESPONSE_ONLY_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.claude_code.response_only.preparation.target_axis_mismatch",
            "Claude Code response-only preparation target uses a different version axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.entitlement_metering()
            != &EntitlementMetering::SubscriptionAllowance
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.endpoint_audience().as_str()
            != crate::claude_code::ENDPOINT_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.claude_code.response_only.preparation.access_profile_rejected",
            "Claude Code response-only requires provider-supported local subscription access",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.credential() != CredentialState::NotRequired
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.claude_code.response_only.preparation.access_evidence_mismatch",
            "Claude Code response-only access evidence does not match the selected profile",
        ));
    }
    Ok(())
}

pub(super) fn promote(
    input: ClaudeCodeResponsePreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<ClaudeCodeResponsePreparedIntegration, PreparationFailure> {
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
            "swallowtail.claude_code.response_only.preparation.observation_mismatch",
            "Claude Code response-only discovery does not match the prepared target",
        ));
    }
    let configured = instance::configured_instance(&input, &observation)?;
    Ok(ClaudeCodeResponsePreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance: configured,
        available_host_services,
    })
}

pub(super) fn discovery_runtime_failure(
    error: swallowtail_runtime::RuntimeFailure,
) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.claude_code.response_only.discovery_axis_mismatch"
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
            "swallowtail.claude_code.response_only.preparation.discovery_rejected",
            "Claude Code response-only executable discovery was not promotable",
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
