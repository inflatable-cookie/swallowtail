#[path = "prepared/catalogue.rs"]
mod catalogue;
#[path = "prepared/facade.rs"]
mod facade;
#[path = "prepared/runtime.rs"]
mod runtime;

pub use catalogue::{
    BedrockCataloguePreparationInput, BedrockCataloguePreparedEvidence,
    BedrockCataloguePreparedIntegration, BedrockCatalogueProfileInput, BedrockPreparedCatalogue,
    prepare_bedrock_catalogue,
};
pub use facade::{
    BedrockCatalogueRouteInput, BedrockFacade, BedrockFacadePreparationInput,
    BedrockRuntimeRouteInput, prepare_bedrock,
};
pub use runtime::{
    BedrockModelSelection, BedrockPreparedInferenceAttempt, BedrockRuntimePreparationInput,
    BedrockRuntimePreparedEvidence, BedrockRuntimePreparedIntegration, BedrockRuntimeProfileInput,
    prepare_bedrock_runtime,
};

use swallowtail_core::{
    AccessProfile, CredentialMechanism, EntitlementMetering, ExecutionHostId, InstanceTargetRef,
    SupportAuthority,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

fn validate_preparation(
    services: &HostServices,
    execution_host: &ExecutionHostId,
    access: &AccessProfile,
    evidence: &PreparedAccessEvidence,
    required_access_id: &str,
    required_audience: &str,
    code_prefix: &'static str,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != execution_host {
        return Err(failure(
            PreparationStage::TargetSelection,
            code_prefix,
            "Bedrock preparation services belong to a different execution host",
        ));
    }
    if access.id().as_str() != required_access_id
        || access.credential_mechanism() != &CredentialMechanism::CloudProviderIdentity
        || access.credential_reference().is_none()
        || access.entitlement_metering() != &EntitlementMetering::CloudAccountBilling
        || access.support_authority() != SupportAuthority::ProviderSupported
        || access.endpoint_audience().as_str() != required_audience
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            code_prefix,
            "Bedrock preparation requires the exact provider-supported delegated cloud identity profile",
        ));
    }
    let status = evidence.status();
    if status.profile_id() != access.id()
        || status.support_authority() != access.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            code_prefix,
            "Bedrock access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn validate_execution_binding(
    prepared_host: &ExecutionHostId,
    prepared_target: &InstanceTargetRef,
    execution_host: &ExecutionHostId,
    endpoint_target: &InstanceTargetRef,
    code: &'static str,
) -> Result<(), PreparationFailure> {
    if prepared_host != execution_host || prepared_target != endpoint_target {
        return Err(failure(
            PreparationStage::TargetSelection,
            code,
            "Prepared Bedrock host or endpoint target no longer matches",
        ));
    }
    Ok(())
}

fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
