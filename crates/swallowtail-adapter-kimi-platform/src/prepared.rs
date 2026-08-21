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
pub struct KimiPlatformPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl KimiPlatformPreparationInput {
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
        if admitted.route_id().as_str() != crate::KIMI_PLATFORM_CHAT_ADDABLE_ROUTE_ID {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.kimi_platform.preparation.route_mismatch",
                "Kimi Platform preparation requires the admitted Chat route",
            ));
        }
        let endpoint_field_id = ConfigFieldId::new(crate::KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID)
            .expect("static Kimi Platform config field id is valid");
        let endpoint = admitted.config_ref(&endpoint_field_id).ok_or_else(|| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.kimi_platform.preparation.endpoint_ref_missing",
                "Kimi Platform preparation requires the admitted endpoint reference",
            )
        })?;
        let credential_field_id =
            CredentialFieldId::new(crate::KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
                .expect("static Kimi Platform credential field id is valid");
        let credential = admitted
            .credential_ref(&credential_field_id)
            .ok_or_else(|| {
                failure(
                    PreparationStage::AccessEvidence,
                    "swallowtail.kimi_platform.preparation.credential_ref_missing",
                    "Kimi Platform preparation requires the admitted API-key reference",
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
            "swallowtail.kimi_platform.preparation.credential_ref_mismatch",
            "Kimi Platform access profile does not match the admitted API-key reference",
        )),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Kimi Platform integration bound to one instance and host.
pub struct KimiPlatformPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl KimiPlatformPreparedIntegration {
    #[must_use]
    /// Returns the exact public-platform API-key access profile.
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
    pub fn low_level_driver(&self) -> crate::KimiPlatformDirectDriver {
        crate::KimiPlatformDirectDriver::new()
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
                "swallowtail.kimi_platform.preparation.target_drift",
                "Prepared Kimi Platform host or endpoint target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Prepares the Kimi Platform direct integration without provider effects.
pub fn prepare_kimi_platform_direct(
    input: KimiPlatformPreparationInput,
    services: &HostServices,
) -> Result<KimiPlatformPreparedIntegration, PreparationFailure> {
    validate_input(&input, services)?;
    Ok(KimiPlatformPreparedIntegration {
        instance: configured_instance(&input),
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services: services.available_kinds(),
    })
}

fn validate_input(
    input: &KimiPlatformPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi_platform.preparation.host_mismatch",
            "Kimi Platform preparation services belong to a different execution host",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str()
            != crate::KIMI_PLATFORM_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi_platform.preparation.access_profile_rejected",
            "Kimi Platform direct preparation requires the provider-supported Platform API-key profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi_platform.preparation.access_evidence_mismatch",
            "Kimi Platform access evidence does not match the selected access profile",
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
