use crate::{OH_MY_PI_PACKAGE_AXIS, OhMyPiRpcDriver};
#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
pub(crate) mod instance;
use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DiscoveryOutcome, EntitlementMetering, ExecutionHostId, HostServiceKind,
    InstalledExecutableObservation, InstanceRevision, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

const ENDPOINT_AUDIENCE: &str = "oh-my-pi-harness";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OhMyPiPreparationInput {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstalledExecutableTarget,
    pub(crate) environment: EnvironmentRef,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
}

impl OhMyPiPreparationInput {
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
pub struct OhMyPiPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl OhMyPiPreparationProbe {
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
pub struct OhMyPiPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OhMyPiPreparedIntegration {
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
    pub fn low_level_driver(&self) -> OhMyPiRpcDriver {
        OhMyPiRpcDriver::new(self.environment.clone())
    }

    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.oh_my_pi.preparation.target_drift",
                "Prepared OhMyPi host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

pub async fn prepare_oh_my_pi_rpc(
    input: OhMyPiPreparationInput,
    probe: OhMyPiPreparationProbe,
    services: HostServices,
) -> Result<OhMyPiPreparedIntegration, PreparationFailure> {
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
    let driver = OhMyPiRpcDriver::new(input.environment.clone());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &OhMyPiPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != OH_MY_PI_PACKAGE_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.oh_my_pi.preparation.target_axis_mismatch",
            "OhMyPi preparation target uses a different version axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::Unknown
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.oh_my_pi.preparation.access_profile_rejected",
            "OhMyPi requires its maintainer-supported local harness access profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
        || status.credential() != CredentialState::NotRequired
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.oh_my_pi.preparation.access_evidence_mismatch",
            "OhMyPi access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: OhMyPiPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<OhMyPiPreparedIntegration, PreparationFailure> {
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
            "swallowtail.oh_my_pi.preparation.observation_mismatch",
            "OhMyPi discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(OhMyPiPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}
