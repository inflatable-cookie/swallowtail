use super::{OpenCodePreparationInput, OpenCodePreparationProbe, failure, runtime_failure};
use swallowtail_core::{
    CredentialMechanism, EntitlementMetering, ExtensionNamespace, SupportAuthority,
};
use swallowtail_runtime::{HostServices, PreparationFailure, PreparationStage, RuntimeFailure};

pub(super) fn validate_input(
    input: &OpenCodePreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.opencode.preparation.host_services_rejected",
            "OpenCode preparation requires matching network, credential, time, and blocking services",
        ));
    }
    let delegated = CredentialMechanism::ProviderSpecific(
        ExtensionNamespace::new("opencode/delegated-auth")
            .expect("static OpenCode credential namespace is valid"),
    );
    if input.access_profile.credential_mechanism() != &delegated
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::Unknown
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.opencode.preparation.access_profile_rejected",
            "OpenCode preparation requires the maintainer-supported delegated-auth profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.opencode.preparation.access_evidence_mismatch",
            "OpenCode access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

pub(super) fn validate_probe(
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if probe.cancellation.is_requested() {
        return Err(failure(
            PreparationStage::BoundedOutput,
            "swallowtail.opencode.preparation.cancelled",
            "OpenCode preparation was cancelled",
        ));
    }
    if services.time().expect("validated time service").now() >= probe.deadline.instant() {
        return Err(failure(
            PreparationStage::BoundedOutput,
            "swallowtail.opencode.preparation.deadline_elapsed",
            "OpenCode preparation deadline elapsed before endpoint work",
        ));
    }
    Ok(())
}

pub(super) fn health_failure(error: RuntimeFailure) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.opencode.version_invalid" => PreparationStage::VersionParse,
        "swallowtail.opencode.version_unsupported" => PreparationStage::CompatibilityClassification,
        _ => PreparationStage::BoundedOutput,
    };
    runtime_failure(stage, error)
}
