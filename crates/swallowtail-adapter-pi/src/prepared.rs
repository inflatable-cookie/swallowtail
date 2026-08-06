use crate::{PI_PACKAGE_AXIS, PiRpcDriver};
#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
pub(crate) mod instance;
use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, DiscoveryOutcome,
    EntitlementMetering, ExecutionHostId, ExtensionNamespace, HostServiceKind,
    InstalledExecutableObservation, InstanceRevision, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

const ACCESS_NAMESPACE: &str = "pi/delegated-harness-auth";
const ENDPOINT_AUDIENCE: &str = "pi-harness";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that bind one installed Pi package before discovery.
pub struct PiPreparationInput {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstalledExecutableTarget,
    pub(crate) environment: EnvironmentRef,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
}

impl PiPreparationInput {
    /// Creates explicit target, environment, and local-access inputs.
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
/// Caller-owned identity, deadline, and cancellation controls for discovery.
pub struct PiPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl PiPreparationProbe {
    /// Creates one bounded installed-executable discovery probe.
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
/// Discovered Pi integration before operation-specific preflight.
pub struct PiPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl PiPreparedIntegration {
    /// Returns the host-private launch environment.
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns the exact executable discovery target.
    #[must_use]
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    /// Returns the qualified installed-package observation.
    #[must_use]
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the local harness access profile.
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    /// Returns the admitted access evidence.
    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Returns the configured RPC instance.
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates host services available when preparation completed.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Reconstructs the low-level RPC driver from prepared inputs.
    #[must_use]
    pub fn low_level_driver(&self) -> PiRpcDriver {
        PiRpcDriver::new(
            self.environment.clone(),
            self.access_profile
                .credential_reference()
                .expect("prepared Pi access has one credential reference")
                .clone(),
        )
    }

    /// Rejects execution after the selected host or target has drifted.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.pi.preparation.target_drift",
                "Prepared Pi host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers and prepares one exact installed Pi RPC route.
pub async fn prepare_pi_rpc(
    input: PiPreparationInput,
    probe: PiPreparationProbe,
    services: HostServices,
) -> Result<PiPreparedIntegration, PreparationFailure> {
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
    let driver = PiRpcDriver::new(
        input.environment.clone(),
        credential_reference(&input.access_profile)?.clone(),
    );
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &PiPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != PI_PACKAGE_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.pi.preparation.target_axis_mismatch",
            "Pi preparation target uses a different version axis",
        ));
    }
    let mechanism = CredentialMechanism::ProviderSpecific(
        ExtensionNamespace::new(ACCESS_NAMESPACE).expect("static Pi namespace is valid"),
    );
    if input.access_profile.credential_mechanism() != &mechanism
        || input.access_profile.entitlement_metering() != &EntitlementMetering::Unknown
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.pi.preparation.access_profile_rejected",
            "Pi requires its maintainer-supported delegated harness access profile",
        ));
    }
    let _ = credential_reference(&input.access_profile)?;
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.pi.preparation.access_evidence_mismatch",
            "Pi access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: PiPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<PiPreparedIntegration, PreparationFailure> {
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
            "swallowtail.pi.preparation.observation_mismatch",
            "Pi discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(PiPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}

fn credential_reference(
    profile: &AccessProfile,
) -> Result<&swallowtail_core::CredentialRef, PreparationFailure> {
    profile.credential_reference().ok_or_else(|| {
        preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.pi.preparation.credential_reference_missing",
            "Pi requires one delegated harness credential reference",
        )
    })
}
