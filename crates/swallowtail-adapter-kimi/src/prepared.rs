use crate::{KIMI_CODE_AXIS, KimiAcpDriver};
#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
pub(crate) mod instance;
use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, DiscoveryOutcome,
    ExecutionHostId, HostServiceKind, InstalledExecutableObservation, InstanceRevision,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    state_root: Option<WorkingResourceRef>,
}

impl KimiPreparationInput {
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
            state_root: None,
        }
    }

    /// Binds the opaque Kimi state root needed for later cross-transport
    /// provider-session authority checks.
    #[must_use]
    pub fn with_state_root(mut self, state_root: WorkingResourceRef) -> Self {
        self.state_root = Some(state_root);
        self
    }
}

#[derive(Clone, Debug)]
pub struct KimiPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl KimiPreparationProbe {
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
pub struct KimiPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    state_root: Option<WorkingResourceRef>,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl KimiPreparedIntegration {
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

    #[must_use]
    pub const fn state_root(&self) -> Option<&WorkingResourceRef> {
        self.state_root.as_ref()
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> KimiAcpDriver {
        KimiAcpDriver::new(
            self.environment.clone(),
            self.access_profile
                .credential_reference()
                .expect("prepared Kimi access has one credential reference")
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
                "swallowtail.kimi.preparation.target_drift",
                "Prepared Kimi host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

pub async fn prepare_kimi(
    input: KimiPreparationInput,
    probe: KimiPreparationProbe,
    services: HostServices,
) -> Result<KimiPreparedIntegration, PreparationFailure> {
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
    let driver = KimiAcpDriver::new(
        input.environment.clone(),
        credential_reference(&input.access_profile)?.clone(),
    );
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &KimiPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != KIMI_CODE_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi.preparation.target_axis_mismatch",
            "Kimi preparation target uses a different version axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::InteractiveOauth {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.preparation.credential_mechanism_mismatch",
            "Kimi Code requires delegated membership OAuth access",
        ));
    }
    let _ = credential_reference(&input.access_profile)?;
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.preparation.access_evidence_mismatch",
            "Kimi access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: KimiPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<KimiPreparedIntegration, PreparationFailure> {
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
            "swallowtail.kimi.preparation.observation_mismatch",
            "Kimi discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(KimiPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        state_root: input.state_root,
        available_host_services,
    })
}

fn credential_reference(
    profile: &AccessProfile,
) -> Result<&swallowtail_core::CredentialRef, PreparationFailure> {
    profile.credential_reference().ok_or_else(|| {
        preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.preparation.credential_reference_missing",
            "Kimi Code requires one delegated credential reference",
        )
    })
}
