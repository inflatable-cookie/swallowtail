#[path = "prepared_headless/instance.rs"]
mod instance;
#[path = "prepared_headless/preparation.rs"]
mod preparation;
#[path = "prepared_headless/profile.rs"]
mod profile;

use preparation::{
    credential_reference, discovery_runtime_failure, preparation_failure, promote, validate_input,
};
pub use profile::{
    GeminiHeadlessModelSelection, GeminiHeadlessPreparedEvidence, GeminiHeadlessPreparedRun,
    GeminiHeadlessRunProfileInput, GeminiHeadlessRunRetention,
};

use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, ExecutionHostId, HostServiceKind,
    InstalledExecutableObservation, InstanceRevision,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that bind one installed Gemini CLI headless instance before discovery.
pub struct GeminiHeadlessPreparationInput {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstalledExecutableTarget,
    pub(crate) environment: EnvironmentRef,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
}

impl GeminiHeadlessPreparationInput {
    /// Creates a headless preparation input with explicit target and access evidence.
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
/// Caller-owned controls for bounded headless executable discovery.
pub struct GeminiHeadlessPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl GeminiHeadlessPreparationProbe {
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
/// Discovered and preflight-ready Gemini CLI headless integration.
pub struct GeminiHeadlessPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl GeminiHeadlessPreparedIntegration {
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

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the selected Developer API access profile.
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    /// Returns the admitted access evidence.
    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Returns the configured headless instance.
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates the host services available when preparation completed.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Reconstructs the low-level headless driver from prepared inputs.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        crate::GeminiHeadlessDriver::new(
            self.environment.clone(),
            self.access_profile
                .credential_reference()
                .expect("prepared Gemini headless access has one credential reference")
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
                "swallowtail.gemini.headless.preparation.target_drift",
                "Prepared Gemini headless host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers and prepares one exact installed Gemini CLI headless route.
pub async fn prepare_gemini_headless(
    input: GeminiHeadlessPreparationInput,
    probe: GeminiHeadlessPreparationProbe,
    services: HostServices,
) -> Result<GeminiHeadlessPreparedIntegration, PreparationFailure> {
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
    let driver = crate::GeminiHeadlessDriver::new(
        input.environment.clone(),
        credential_reference(&input.access_profile)?.clone(),
    );
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}
