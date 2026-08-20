use crate::{CODEX_CLI_AXIS, CodexAppServerDriver, CodexExecDriver};
#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
mod instance;
use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, AdmittedInstanceRecord, ConfigFieldId, ConfiguredInstance, ConfiguredInstanceId,
    DiscoveryOutcome, ExecutionHostId, HostServiceKind, InstalledExecutableObservation,
    InstanceRevision, InterfaceVersionAxis,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

/// The Codex transport selected before preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodexPreparedDriver {
    /// One-shot structured `codex exec` route.
    StructuredExec,
    /// Interactive and retained-thread app-server route.
    AppServer,
}

/// Adapter-local facts required to prepare one exact Codex installation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexPreparationInput {
    driver: CodexPreparedDriver,
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl CodexPreparationInput {
    /// Creates preparation input for one exact installed Codex route.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        driver: CodexPreparedDriver,
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        target: InstalledExecutableTarget,
        environment: EnvironmentRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            driver,
            instance_id,
            instance_revision,
            execution_host_id,
            target,
            environment,
            access_profile,
            access_evidence,
        }
    }

    /// Builds the addable app-server preparation input from one admitted record.
    ///
    /// The host-owned binary path and environment remain opaque until the
    /// selected process service resolves them during discovery.
    pub fn from_admitted(
        admitted: &AdmittedInstanceRecord,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        if admitted.route_id().as_str() != crate::CODEX_APP_SERVER_ADDABLE_ROUTE_ID {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.codex.preparation.route_mismatch",
                "Codex preparation requires the admitted app-server route",
            ));
        }
        let binary_field_id = ConfigFieldId::new(crate::CODEX_APP_SERVER_BINARY_PATH_FIELD_ID)
            .expect("static Codex config field id is valid");
        let binary = admitted.config_ref(&binary_field_id).ok_or_else(|| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.codex.preparation.binary_path_ref_missing",
                "Codex preparation requires the admitted binary-path reference",
            )
        })?;
        let environment_field_id = ConfigFieldId::new(crate::CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID)
            .expect("static Codex config field id is valid");
        let environment = admitted.config_ref(&environment_field_id).ok_or_else(|| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.codex.preparation.environment_ref_missing",
                "Codex preparation requires the admitted environment reference",
            )
        })?;
        Ok(Self::new(
            CodexPreparedDriver::AppServer,
            admitted.id().clone(),
            instance_revision,
            execution_host_id,
            InstalledExecutableTarget::new(
                ExecutableRef::from_config_field(binary),
                InterfaceVersionAxis::new(crate::CODEX_CLI_AXIS)
                    .expect("static Codex version axis is valid"),
            ),
            EnvironmentRef::from_config_field(environment),
            access_profile,
            access_evidence,
        ))
    }
}

/// Caller-owned lifecycle controls for the bounded installed-version probe.
#[derive(Clone, Debug)]
pub struct CodexPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl CodexPreparationProbe {
    /// Creates a bounded installed-version probe.
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

/// One prepared Codex driver bound to one host-approved executable observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexPreparedIntegration {
    driver: CodexPreparedDriver,
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl CodexPreparedIntegration {
    /// Returns the exact prepared Codex route.
    #[must_use]
    pub const fn driver(&self) -> CodexPreparedDriver {
        self.driver
    }

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

    /// Returns the configured ChatGPT subscription access profile.
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

    /// Rejects host registry or approved-target drift before provider effects.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.codex.preparation.target_drift",
                "Prepared Codex host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers and binds one exact Codex executable without selecting operation intent.
pub async fn prepare_codex(
    input: CodexPreparationInput,
    probe: CodexPreparationProbe,
    services: HostServices,
) -> Result<CodexPreparedIntegration, PreparationFailure> {
    validate_input(&input)?;
    let available_host_services = services.available_kinds();
    let probe = InstalledExecutableDiscoveryRequest::new(
        probe.request_id,
        probe.scope_id,
        input.execution_host_id.clone(),
        input.target.clone(),
        probe.deadline,
        probe.cancellation,
    );
    let outcome = match input.driver {
        CodexPreparedDriver::StructuredExec => {
            CodexExecDriver::new(input.environment.clone())
                .discover_installed_executable(probe, services)
                .await
        }
        CodexPreparedDriver::AppServer => {
            CodexAppServerDriver::new(input.environment.clone())
                .discover_installed_executable(probe, services)
                .await
        }
    }
    .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &CodexPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != CODEX_CLI_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.codex.preparation.target_axis_mismatch",
            "Codex preparation target uses a different version axis",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.codex.preparation.access_evidence_mismatch",
            "Codex access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: CodexPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<CodexPreparedIntegration, PreparationFailure> {
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
            "swallowtail.codex.preparation.observation_mismatch",
            "Codex discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(CodexPreparedIntegration {
        driver: input.driver,
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}
