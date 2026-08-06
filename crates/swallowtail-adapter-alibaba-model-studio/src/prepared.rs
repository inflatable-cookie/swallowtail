#[path = "prepared/instance.rs"]
mod instance;

use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, CredentialMechanism, EntitlementMetering, ExecutionHostId,
    HostServiceKind, InstanceRevision, InstanceTargetRef, SupportAuthority,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Host, workspace endpoint, and access evidence used for route preparation.
pub struct AlibabaModelStudioPreparationInput {
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl AlibabaModelStudioPreparationInput {
    #[must_use]
    /// Creates workspace preparation input without provider effects.
    pub const fn new(
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_revision,
            execution_host_id,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Model Studio workspace integration bound to one instance and host.
pub struct AlibabaModelStudioPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl AlibabaModelStudioPreparedIntegration {
    #[must_use]
    /// Returns the selected workspace API-key access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns access evidence together with its provenance.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    /// Returns the configured workspace instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates the host services present during preparation.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    /// Returns the public low-level workspace driver.
    pub fn low_level_driver(&self) -> crate::AlibabaModelStudioDriver {
        crate::AlibabaModelStudioDriver::new()
    }

    /// Rejects execution-host or workspace-endpoint drift.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        endpoint_target: &InstanceTargetRef,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.instance.execution_host_id()
            || endpoint_target != self.instance.target_reference()
        {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.alibaba_model_studio.preparation.target_drift",
                "Prepared Alibaba Model Studio host or workspace endpoint no longer matches",
            ));
        }
        Ok(())
    }
}

/// Prepares a Model Studio workspace integration without provider effects.
pub fn prepare_alibaba_model_studio(
    input: AlibabaModelStudioPreparationInput,
    services: &HostServices,
) -> Result<AlibabaModelStudioPreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    Ok(AlibabaModelStudioPreparedIntegration {
        instance: configured_instance(&input),
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &AlibabaModelStudioPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.alibaba_model_studio.preparation.host_mismatch",
            "Alibaba Model Studio preparation services belong to a different execution host",
        ));
    }
    if input.access_profile.id().as_str() != crate::ACCESS_PROFILE_ID
        || input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str() != crate::ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.alibaba_model_studio.preparation.access_profile_rejected",
            "Alibaba Model Studio preparation requires the Singapore workspace general API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.alibaba_model_studio.preparation.access_evidence_mismatch",
            "Alibaba Model Studio access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

pub(crate) fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
