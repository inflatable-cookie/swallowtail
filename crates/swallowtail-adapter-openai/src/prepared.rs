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
/// Host, endpoint, and access evidence used to prepare background Responses.
pub struct OpenAiBackgroundPreparationInput {
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl OpenAiBackgroundPreparationInput {
    #[must_use]
    /// Creates side-effect-free preparation input for one exact public API route.
    pub const fn new(
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared OpenAI background integration bound to one instance and host.
pub struct OpenAiBackgroundPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OpenAiBackgroundPreparedIntegration {
    #[must_use]
    /// Returns the selected public API-key access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns access evidence together with its provenance.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    /// Returns the configured background Responses instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates the host services present during preparation.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    /// Returns the public low-level background driver.
    pub fn low_level_driver(&self) -> crate::OpenAiBackgroundDriver {
        crate::OpenAiBackgroundDriver::new()
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
                "swallowtail.openai.preparation.target_drift",
                "Prepared OpenAI host or public API endpoint no longer matches",
            ));
        }
        Ok(())
    }
}

/// Prepares OpenAI background Responses without issuing provider requests.
pub fn prepare_openai_background(
    input: OpenAiBackgroundPreparationInput,
    services: &HostServices,
) -> Result<OpenAiBackgroundPreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    Ok(OpenAiBackgroundPreparedIntegration {
        instance: configured_instance(&input),
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &OpenAiBackgroundPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id
        || input.endpoint_target.as_host_value() != crate::OPENAI_BACKGROUND_ENDPOINT
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.openai.preparation.target_rejected",
            "OpenAI background preparation requires the exact public API endpoint on the selected host",
        ));
    }
    if input.access_profile.id().as_str() != crate::OPENAI_BACKGROUND_ACCESS_PROFILE_ID
        || input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str()
            != crate::OPENAI_BACKGROUND_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.openai.preparation.access_profile_rejected",
            "OpenAI background preparation requires the provider-supported public API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.openai.preparation.access_evidence_mismatch",
            "OpenAI background access evidence does not match the selected public API profile",
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
