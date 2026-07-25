#[path = "prepared/attached.rs"]
mod attached;
#[path = "prepared/owned.rs"]
mod owned;

pub use attached::{
    LlamaCppAttachedPreparationInput, LlamaCppAttachedPreparedEvidence,
    LlamaCppAttachedPreparedIntegration, LlamaCppCatalogueProfileInput,
    LlamaCppInferenceProfileInput, LlamaCppModelSelection, LlamaCppPreparedCatalogue,
    LlamaCppPreparedInferenceAttempt, prepare_llama_cpp_attached,
};
pub use owned::{
    LlamaCppOwnedPreparationInput, LlamaCppOwnedPreparedEvidence, LlamaCppOwnedPreparedIntegration,
    LlamaCppOwnedServingSelection, LlamaCppPreparedServingStart, prepare_llama_cpp_owned,
};

use swallowtail_core::{
    AccessProfile, CredentialMechanism, EntitlementMetering, ExecutionHostId, SupportAuthority,
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
    code: &'static str,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != execution_host {
        return Err(failure(
            PreparationStage::TargetSelection,
            code,
            "llama.cpp preparation services belong to a different execution host",
        ));
    }
    if access.id().as_str() != required_access_id
        || access.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || access.credential_reference().is_some()
        || access.entitlement_metering() != &EntitlementMetering::LocalCompute
        || access.support_authority() != SupportAuthority::IntegrationMaintainerSupported
        || access.endpoint_audience().as_str() != required_audience
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            code,
            "llama.cpp preparation requires the exact local unauthenticated access profile",
        ));
    }
    if evidence.status().profile_id() != access.id()
        || evidence.status().support_authority() != access.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            code,
            "llama.cpp access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn preflight_failure(error: swallowtail_core::PreflightFailure) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
    )
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
