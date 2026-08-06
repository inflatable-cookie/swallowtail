use crate::{QWEN_CODE_AXIS, QwenHeadlessDriver};
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

const ACCESS_NAMESPACE: &str = "qwen-code/delegated-harness-auth";
const ENDPOINT_AUDIENCE: &str = "qwen-code";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that qualify one installed Qwen Code instance.
pub struct QwenPreparationInput {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstalledExecutableTarget,
    pub(crate) environment: EnvironmentRef,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
}

impl QwenPreparationInput {
    /// Creates preparation input for an exact installed Qwen target.
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
/// Bounded discovery request used while preparing Qwen Code.
pub struct QwenPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl QwenPreparationProbe {
    /// Creates a Qwen preparation probe.
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
/// Qualified Qwen integration ready to prepare catalogue, run, or session work.
pub struct QwenPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl QwenPreparedIntegration {
    /// Returns the approved execution environment.
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns the qualified installed-executable target.
    #[must_use]
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    /// Returns the executable observation admitted during preparation.
    #[must_use]
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the configured delegated harness-auth profile.
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Returns the exact configured provider instance.
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates over host services present when preparation succeeded.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Creates the low-level driver bound to this integration.
    #[must_use]
    pub fn low_level_driver(&self) -> QwenHeadlessDriver {
        QwenHeadlessDriver::new(self.environment.clone())
    }

    /// Rejects host or executable drift from the prepared target.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.qwen.preparation.target_drift",
                "Prepared Qwen host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers, validates, and prepares one configured Qwen Code instance.
pub async fn prepare_qwen_headless(
    input: QwenPreparationInput,
    probe: QwenPreparationProbe,
    services: HostServices,
) -> Result<QwenPreparedIntegration, PreparationFailure> {
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
    let driver = QwenHeadlessDriver::new(input.environment.clone());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &QwenPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != QWEN_CODE_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.qwen.preparation.target_axis_mismatch",
            "Qwen preparation target uses a different version axis",
        ));
    }
    let mechanism = CredentialMechanism::ProviderSpecific(
        ExtensionNamespace::new(ACCESS_NAMESPACE).expect("static Qwen namespace is valid"),
    );
    if input.access_profile.credential_mechanism() != &mechanism
        || input.access_profile.entitlement_metering() != &EntitlementMetering::Unknown
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.qwen.preparation.access_profile_rejected",
            "Qwen requires its maintainer-supported delegated harness access profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.qwen.preparation.access_evidence_mismatch",
            "Qwen access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: QwenPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<QwenPreparedIntegration, PreparationFailure> {
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
            "swallowtail.qwen.preparation.observation_mismatch",
            "Qwen discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(QwenPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}
