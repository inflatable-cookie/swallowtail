#[path = "prepared_headless/instance.rs"]
mod instance;
#[path = "prepared_headless/preparation.rs"]
mod preparation;
#[path = "prepared_headless/profile.rs"]
mod profile;

pub use profile::{KimiHeadlessPreparedEvidence, KimiHeadlessPreparedRun, KimiHeadlessRunInput};

use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, ExecutionHostId, HostServiceKind,
    InstalledExecutableObservation, InstanceRevision,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparedAccessEvidence, RequestId, ScopeId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiHeadlessPreparationInput {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstalledExecutableTarget,
    pub(crate) environment: EnvironmentRef,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
}

impl KimiHeadlessPreparationInput {
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
pub struct KimiHeadlessPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl KimiHeadlessPreparationProbe {
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
pub struct KimiHeadlessPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl KimiHeadlessPreparedIntegration {
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
    pub fn low_level_driver(&self) -> crate::KimiHeadlessDriver {
        crate::KimiHeadlessDriver::new(
            self.environment.clone(),
            self.access_profile
                .credential_reference()
                .expect("prepared Kimi headless access has one credential")
                .clone(),
        )
    }

    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation::failure(
                swallowtail_runtime::PreparationStage::TargetSelection,
                "swallowtail.kimi.headless.preparation.target_drift",
                "Prepared Kimi headless host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

pub async fn prepare_kimi_headless(
    input: KimiHeadlessPreparationInput,
    probe: KimiHeadlessPreparationProbe,
    services: HostServices,
) -> Result<KimiHeadlessPreparedIntegration, PreparationFailure> {
    preparation::validate_input(&input)?;
    let available_host_services = services.available_kinds();
    let request = InstalledExecutableDiscoveryRequest::new(
        probe.request_id,
        probe.scope_id,
        input.execution_host_id.clone(),
        input.target.clone(),
        probe.deadline,
        probe.cancellation,
    );
    let driver = crate::KimiHeadlessDriver::new(
        input.environment.clone(),
        preparation::credential_reference(&input.access_profile)?.clone(),
    );
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(preparation::discovery_runtime_failure)?;
    preparation::promote(input, outcome, available_host_services)
}
