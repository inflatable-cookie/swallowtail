use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    EntitlementMetering, ExecutionHostId, HostServiceKind, InstanceRevision, InstanceTargetRef,
    ProviderAgentBinding, SupportAuthority,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    provider_agent: ProviderAgentBinding,
}

impl AnthropicManagedPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
        provider_agent: ProviderAgentBinding,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
            provider_agent,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl AnthropicManagedPreparedIntegration {
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> crate::AnthropicManagedAgentDriver {
        crate::AnthropicManagedAgentDriver::new()
    }

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
                "swallowtail.anthropic.managed.preparation.target_drift",
                "Prepared Anthropic Managed Agents host or endpoint target no longer matches",
            ));
        }
        Ok(())
    }
}

pub fn prepare_anthropic_managed_agent(
    input: AnthropicManagedPreparationInput,
    services: &HostServices,
) -> Result<AnthropicManagedPreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    let instance = crate::anthropic_managed_instance(
        input.instance_id,
        input.instance_revision,
        input.execution_host_id,
        input.endpoint_target,
        input.access_profile.id().clone(),
        input.provider_agent,
    );
    Ok(AnthropicManagedPreparedIntegration {
        instance,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &AnthropicManagedPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.anthropic.managed.preparation.host_mismatch",
            "Anthropic Managed Agents preparation services belong to a different execution host",
        ));
    }
    if input
        .provider_agent
        .version()
        .as_str()
        .parse::<u64>()
        .is_err()
    {
        return Err(failure(
            PreparationStage::Preflight,
            "swallowtail.anthropic.managed.preparation.agent_version_rejected",
            "Anthropic Managed Agents requires one exact numeric operator-owned agent version",
        ));
    }
    if input.access_profile.id().as_str() != crate::ANTHROPIC_MANAGED_ACCESS_PROFILE_ID
        || input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str()
            != crate::ANTHROPIC_MANAGED_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.anthropic.managed.preparation.access_profile_rejected",
            "Anthropic Managed Agents preparation requires the provider-supported public API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.anthropic.managed.preparation.access_evidence_mismatch",
            "Anthropic Managed Agents access evidence does not match the selected public API profile",
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
