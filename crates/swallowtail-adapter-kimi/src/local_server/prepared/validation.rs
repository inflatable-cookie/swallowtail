use super::{KimiLocalServerAttachedInput, KimiLocalServerPreparationProbe, preparation_failure};
use swallowtail_core::{
    CredentialMechanism, CredentialState, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExtensionNamespace, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{HostServices, PreparationFailure, PreparationStage};

pub(super) fn validate_input(
    input: &KimiLocalServerAttachedInput,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
    owned: bool,
) -> Result<(), PreparationFailure> {
    let required_services_present = services.execution_host_id() == &input.execution_host_id
        && services.task().is_some()
        && services.blocking_work().is_some()
        && services.time().is_some()
        && services.network().is_some()
        && services.credential().is_some()
        && services.working_resource().is_some()
        && (!owned || services.process().is_some());
    if !required_services_present {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi.local_server.preparation.host_services_rejected",
            "Kimi local-server preparation requires matching host services",
        ));
    }
    let expected = CredentialMechanism::ProviderSpecific(
        ExtensionNamespace::new("kimi-code/local-server-bearer")
            .expect("static credential namespace is valid"),
    );
    if input.access_profile.credential_mechanism() != &expected
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::LocalCompute
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.local_server.preparation.access_profile_rejected",
            "Kimi local-server requires its maintainer-supported local bearer profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.credential() != CredentialState::Ready
        || status.entitlement() != EntitlementState::Available
        || status.endpoint_authorization() != EndpointAuthorization::Allowed
        || status.runtime_readiness() != RuntimeReadiness::Ready
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.local_server.preparation.access_evidence_mismatch",
            "Kimi local-server access evidence does not match its selected profile",
        ));
    }
    if probe.cancellation.is_requested() {
        return Err(preparation_failure(
            PreparationStage::BoundedOutput,
            "swallowtail.kimi.local_server.preparation.cancelled",
            "Kimi local-server preparation was cancelled",
        ));
    }
    if services.time().expect("validated time").now() >= probe.deadline.instant() {
        return Err(preparation_failure(
            PreparationStage::BoundedOutput,
            "swallowtail.kimi.local_server.preparation.deadline_elapsed",
            "Kimi local-server preparation deadline elapsed before host work",
        ));
    }
    Ok(())
}
