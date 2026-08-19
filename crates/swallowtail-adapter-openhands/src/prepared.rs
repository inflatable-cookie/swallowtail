//! Adapter-local prepared constructor for `openhands.agent-server`.

#[path = "prepared/activity.rs"]
mod activity;
#[path = "prepared/run.rs"]
mod run;

pub use run::{OpenHandsAgentServerPreparedRun, OpenHandsAgentServerRunProfileInput};

use crate::OpenHandsAgentServerDriver;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, DiscoveryOutcome,
    DiscoveryStatus, EntitlementMetering, ExecutionHostId, HarnessConfigurationPosture,
    HostServiceKind, InstalledExecutableObservation, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ProtocolFacadeId, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

const PROTOCOL_FACADE_ID: &str = "openhands-agent-server-http-ws-v1";
const POLICY_ID: &str = "openhands-prepared-ambient-agent-server";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Host, target, environment, and local-loopback inputs for OpenHands preparation.
pub struct OpenHandsAgentServerPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl OpenHandsAgentServerPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Creates explicit preparation input for one exact OpenHands package.
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
/// Bounded installed-executable probe used during OpenHands Agent Server preparation.
pub struct OpenHandsAgentServerPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl OpenHandsAgentServerPreparationProbe {
    #[must_use]
    /// Creates a cancellable, deadline-bound discovery probe.
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
/// Qualified OpenHands Agent Server integration ready to prepare one owned loopback run.
pub struct OpenHandsAgentServerPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OpenHandsAgentServerPreparedIntegration {
    #[must_use]
    /// Returns the approved execution environment.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    #[must_use]
    /// Returns the exact installed target admitted during preparation.
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    #[must_use]
    /// Returns the executable observation admitted during preparation.
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    /// Returns the host-owned local-loopback access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns immutable access evidence admitted during preparation.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    /// Returns the exact configured provider instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates over host services present when preparation succeeded.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    /// Creates the low-level Agent Server driver as an explicit escape hatch.
    pub fn low_level_driver(&self) -> OpenHandsAgentServerDriver {
        OpenHandsAgentServerDriver::new(self.environment.clone())
    }

    /// Rejects host or executable drift from the prepared target.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.openhands.agent_server.preparation.target_drift",
                "Prepared OpenHands host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers and prepares exactly one qualified OpenHands Agent Server package.
pub async fn prepare_openhands_agent_server(
    input: OpenHandsAgentServerPreparationInput,
    probe: OpenHandsAgentServerPreparationProbe,
    services: HostServices,
) -> Result<OpenHandsAgentServerPreparedIntegration, PreparationFailure> {
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
    let driver = OpenHandsAgentServerDriver::new(input.environment.clone());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &OpenHandsAgentServerPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::OPENHANDS_PACKAGE_AXIS {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.openhands.agent_server.preparation.target_axis_mismatch",
            "OpenHands Agent Server preparation target uses a different package axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::Unknown
        || input.access_profile.endpoint_audience().as_str()
            != crate::OPENHANDS_LOCAL_ACCOUNT_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.openhands.agent_server.preparation.access_profile_rejected",
            "OpenHands Agent Server requires its provider-supported local-loopback profile",
        ));
    }
    if input.access_evidence.status().profile_id() != input.access_profile.id()
        || input.access_evidence.status().support_authority()
            != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.openhands.agent_server.preparation.access_evidence_mismatch",
            "OpenHands access evidence does not match the selected profile",
        ));
    }
    Ok(())
}

fn promote(
    input: OpenHandsAgentServerPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<OpenHandsAgentServerPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
        || observation.version().version().as_str() != crate::OPENHANDS_PACKAGE_VERSION
    {
        return Err(failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.openhands.agent_server.preparation.observation_mismatch",
            "OpenHands discovery observation does not match the prepared host and package",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(OpenHandsAgentServerPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}

fn configured_instance(
    input: &OpenHandsAgentServerPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.openhands.agent_server.preparation.target_invalid",
                "OpenHands Agent Server target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::openhands_agent_server_descriptor()
            .identity()
            .id()
            .clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(PROTOCOL_FACADE_ID)
            .expect("static OpenHands Agent Server facade is valid"),
        InstancePolicyId::new(POLICY_ID).expect("static OpenHands Agent Server policy is valid"),
        run::advertised_capabilities(),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

fn discovery_runtime_failure(error: swallowtail_runtime::RuntimeFailure) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::ProcessSpawn,
        swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
    )
}

fn discovery_outcome_failure(outcome: &DiscoveryOutcome) -> PreparationFailure {
    let stage = match outcome.status() {
        DiscoveryStatus::Malformed => PreparationStage::VersionParse,
        DiscoveryStatus::Incompatible => PreparationStage::CompatibilityClassification,
        DiscoveryStatus::CleanupFailed => PreparationStage::Cleanup,
        DiscoveryStatus::TimedOut | DiscoveryStatus::Cancelled => PreparationStage::BoundedOutput,
        _ => PreparationStage::ProcessSpawn,
    };
    let diagnostic = outcome.diagnostic().cloned().unwrap_or_else(|| {
        swallowtail_core::SafeDiagnostic::new(
            "swallowtail.openhands.agent_server.preparation.discovery_rejected",
            "OpenHands interpreter discovery was not promotable",
        )
    });
    PreparationFailure::new(stage, swallowtail_core::Diagnostic::new(diagnostic))
}

fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
