#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
mod instance;
pub(crate) use instance::{run_capabilities, session_capabilities};

use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DiscoveryOutcome, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, HostServiceKind, InstalledExecutableObservation, InstanceRevision,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

use crate::{
    GROK_BUILD_ACP_AXIS, GROK_BUILD_SUBSCRIPTION_ACCESS_PROFILE_ID,
    GROK_BUILD_SUBSCRIPTION_AUDIENCE, GrokAcpDriver,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl GrokPreparationInput {
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        target: InstalledExecutableTarget,
        environment: EnvironmentRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            target,
            environment,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GrokPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl GrokPreparationProbe {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        scope_id: ScopeId,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            request_id,
            scope_id,
            deadline,
            cancellation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl GrokPreparedIntegration {
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    #[must_use]
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    #[must_use]
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

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
    pub fn low_level_driver(&self) -> GrokAcpDriver {
        GrokAcpDriver::new(
            self.environment.clone(),
            self.access_profile
                .credential_reference()
                .expect("prepared Grok access has one credential reference")
                .clone(),
        )
    }

    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.grok.preparation.target_drift",
                "Prepared Grok host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

pub async fn prepare_grok_build(
    input: GrokPreparationInput,
    probe: GrokPreparationProbe,
    services: HostServices,
) -> Result<GrokPreparedIntegration, PreparationFailure> {
    validate_input(&input)?;
    let available_host_services = services.available_kinds();
    let request = InstalledExecutableDiscoveryRequest::new(
        probe.request_id,
        probe.scope_id,
        input.execution_host_id.clone(),
        input.target.clone(),
        probe.deadline,
        probe.cancellation,
    );
    let driver = GrokAcpDriver::new(
        input.environment.clone(),
        credential_reference(&input.access_profile)?.clone(),
    );
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &GrokPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != GROK_BUILD_ACP_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.grok.preparation.target_axis_mismatch",
            "Grok preparation target uses a different version axis",
        ));
    }
    let access = &input.access_profile;
    if access.id().as_str() != GROK_BUILD_SUBSCRIPTION_ACCESS_PROFILE_ID
        || access.credential_mechanism() != &CredentialMechanism::InteractiveOauth
        || access.entitlement_metering() != &EntitlementMetering::SubscriptionAllowance
        || access.endpoint_audience().as_str() != GROK_BUILD_SUBSCRIPTION_AUDIENCE
        || access.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.grok.preparation.access_profile_mismatch",
            "Grok preparation requires its delegated subscription OAuth profile",
        ));
    }
    let _ = credential_reference(access)?;
    let status = input.access_evidence.status();
    if status.profile_id() != access.id()
        || status.credential() != CredentialState::Ready
        || status.entitlement() != EntitlementState::Available
        || status.endpoint_authorization() != EndpointAuthorization::Allowed
        || status.runtime_readiness() != RuntimeReadiness::Ready
        || status.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.grok.preparation.access_evidence_mismatch",
            "Grok access evidence does not prove the selected subscription profile ready",
        ));
    }
    Ok(())
}

fn promote(
    input: GrokPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<GrokPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
    {
        return Err(preparation_failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.grok.preparation.observation_mismatch",
            "Grok discovery observation does not match the prepared target",
        ));
    }
    let configured = instance::configured_instance(&input, &observation)?;
    Ok(GrokPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance: configured,
        available_host_services,
    })
}

fn credential_reference(
    profile: &AccessProfile,
) -> Result<&swallowtail_core::CredentialRef, PreparationFailure> {
    profile.credential_reference().ok_or_else(|| {
        preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.grok.preparation.credential_reference_missing",
            "Grok subscription access requires one opaque credential reference",
        )
    })
}
