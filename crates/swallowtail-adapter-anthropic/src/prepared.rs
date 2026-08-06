#[path = "prepared/instance.rs"]
mod instance;

use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    EntitlementMetering, ExecutionHostId, HostServiceKind, InstanceRevision, InstanceTargetRef,
    SupportAuthority,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

pub(crate) const ENDPOINT_AUDIENCE: &str = "api.anthropic.com";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit host, endpoint, and API-key evidence used for Messages preparation.
pub struct AnthropicPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl AnthropicPreparationInput {
    #[must_use]
    /// Creates preparation input without performing provider work.
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Anthropic Messages integration bound to one instance and host.
pub struct AnthropicPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl AnthropicPreparedIntegration {
    #[must_use]
    /// Returns the exact public API-key access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns the access evidence and its provenance.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    /// Returns the prepared configured instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates the host services present during preparation.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    /// Returns the public low-level Messages driver.
    pub fn low_level_driver(&self) -> crate::AnthropicDirectDriver {
        crate::AnthropicDirectDriver::new()
    }

    /// Rejects execution-host or endpoint drift from the prepared binding.
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
                "swallowtail.anthropic.preparation.target_drift",
                "Prepared Anthropic host or endpoint target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Prepares Anthropic Messages integration without provider effects.
pub fn prepare_anthropic_direct(
    input: AnthropicPreparationInput,
    services: &HostServices,
) -> Result<AnthropicPreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    Ok(AnthropicPreparedIntegration {
        instance: configured_instance(&input),
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &AnthropicPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.anthropic.preparation.host_mismatch",
            "Anthropic preparation services belong to a different execution host",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.anthropic.preparation.access_profile_rejected",
            "Anthropic direct preparation requires the provider-supported public API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.anthropic.preparation.access_evidence_mismatch",
            "Anthropic access evidence does not match the selected access profile",
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
