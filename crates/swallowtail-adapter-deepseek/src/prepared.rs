#[path = "prepared/instance.rs"]
mod instance;

use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, AdmittedInstanceRecord, ConfigFieldId, ConfiguredInstance, ConfiguredInstanceId,
    CredentialFieldId, CredentialMechanism, EntitlementMetering, ExecutionHostId, HostServiceKind,
    InstanceRevision, InstanceTargetRef, SupportAuthority,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit host, endpoint, and API-key evidence used for preparation.
pub struct DeepSeekPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl DeepSeekPreparationInput {
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

    /// Builds preparation input from one admitted hosted-route record.
    ///
    /// The host-owned endpoint and credential references remain opaque until
    /// the selected host service resolves them during preparation or use.
    pub fn from_admitted(
        admitted: &AdmittedInstanceRecord,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        if admitted.route_id().as_str() != crate::DEEPSEEK_CONTINUATION_ADDABLE_ROUTE_ID {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.deepseek.preparation.route_mismatch",
                "DeepSeek preparation requires the admitted continuation route",
            ));
        }
        let endpoint_field_id = ConfigFieldId::new(crate::DEEPSEEK_CONTINUATION_ENDPOINT_FIELD_ID)
            .expect("static DeepSeek config field id is valid");
        let endpoint = admitted.config_ref(&endpoint_field_id).ok_or_else(|| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.deepseek.preparation.endpoint_ref_missing",
                "DeepSeek preparation requires the admitted endpoint reference",
            )
        })?;
        let credential_field_id =
            CredentialFieldId::new(crate::DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID)
                .expect("static DeepSeek credential field id is valid");
        let credential = admitted
            .credential_ref(&credential_field_id)
            .ok_or_else(|| {
                failure(
                    PreparationStage::AccessEvidence,
                    "swallowtail.deepseek.preparation.credential_ref_missing",
                    "DeepSeek preparation requires the admitted API-key reference",
                )
            })?;
        let access_profile = bind_credential_reference(access_profile, credential)?;
        Ok(Self::new(
            admitted.id().clone(),
            instance_revision,
            execution_host_id,
            InstanceTargetRef::from_config_field(endpoint),
            access_profile,
            access_evidence,
        ))
    }
}

fn bind_credential_reference(
    access_profile: AccessProfile,
    admitted: &swallowtail_core::CredentialRef,
) -> Result<AccessProfile, PreparationFailure> {
    match access_profile.credential_reference() {
        None => Ok(access_profile.with_credential_reference(admitted.clone())),
        Some(existing) if existing == admitted => Ok(access_profile),
        Some(_) => Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.deepseek.preparation.credential_ref_mismatch",
            "DeepSeek access profile does not match the admitted API-key reference",
        )),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared DeepSeek integration bound to one instance and host.
pub struct DeepSeekPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl DeepSeekPreparedIntegration {
    #[must_use]
    /// Returns the exact Open Platform API-key access profile.
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
    /// Returns the public low-level driver for advanced integration.
    pub fn low_level_driver(&self) -> crate::DeepSeekDirectDriver {
        crate::DeepSeekDirectDriver::new()
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
                "swallowtail.deepseek.preparation.target_drift",
                "Prepared DeepSeek host or endpoint target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Prepares the DeepSeek direct integration without provider effects.
pub fn prepare_deepseek_direct(
    input: DeepSeekPreparationInput,
    services: &HostServices,
) -> Result<DeepSeekPreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    Ok(DeepSeekPreparedIntegration {
        instance: configured_instance(&input),
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &DeepSeekPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id
        || input.endpoint_target.as_host_value() != crate::DEEPSEEK_ENDPOINT
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.deepseek.preparation.target_rejected",
            "DeepSeek preparation requires the exact public endpoint on the selected host",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str() != crate::DEEPSEEK_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.deepseek.preparation.access_profile_rejected",
            "DeepSeek direct preparation requires the provider-supported Open Platform API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.deepseek.preparation.access_evidence_mismatch",
            "DeepSeek access evidence does not match the selected access profile",
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
