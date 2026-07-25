use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, CredentialMechanism, EntitlementMetering, ExecutionHostId,
    HostServiceKind, InstanceRevision, InstanceTargetRef, SupportAuthority,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiRealtimePreparationInput {
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl OpenAiRealtimePreparationInput {
    #[must_use]
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
pub struct OpenAiRealtimePreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OpenAiRealtimePreparedIntegration {
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
    pub fn low_level_driver(&self) -> crate::OpenAiRealtimeDriver {
        crate::OpenAiRealtimeDriver::new()
    }

    pub fn validate_execution_binding(
        &self,
        host: &ExecutionHostId,
        target: &InstanceTargetRef,
    ) -> Result<(), PreparationFailure> {
        if host != self.instance.execution_host_id() || target != self.instance.target_reference() {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.openai.realtime_preparation.target_drift",
                "Prepared OpenAI Realtime host or endpoint no longer matches",
            ));
        }
        Ok(())
    }
}

pub fn prepare_openai_realtime(
    input: OpenAiRealtimePreparationInput,
    services: &HostServices,
) -> Result<OpenAiRealtimePreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    let instance = crate::openai_realtime_instance(
        input.instance_revision,
        input.execution_host_id,
        input.endpoint_target,
        input.access_profile.id().clone(),
    );
    Ok(OpenAiRealtimePreparedIntegration {
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &OpenAiRealtimePreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.openai.realtime_preparation.host_mismatch",
            "OpenAI Realtime preparation services belong to a different execution host",
        ));
    }
    if input.access_profile.id().as_str() != crate::OPENAI_REALTIME_ACCESS_PROFILE_ID
        || input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str()
            != crate::OPENAI_REALTIME_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.openai.realtime_preparation.access_profile_rejected",
            "OpenAI Realtime preparation requires the provider-supported public API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.openai.realtime_preparation.access_evidence_mismatch",
            "OpenAI Realtime access evidence does not match the selected profile",
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
