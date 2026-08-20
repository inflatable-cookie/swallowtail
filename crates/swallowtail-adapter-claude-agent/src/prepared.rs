use crate::{CLAUDE_AGENT_ACP_AXIS, ClaudeAgentAcpDriver};
#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
pub(crate) mod instance;
use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, AdmittedInstanceRecord, ConfigFieldId, ConfiguredInstance, ConfiguredInstanceId,
    CredentialMechanism, CredentialState, DiscoveryOutcome, EntitlementMetering, ExecutionHostId,
    HostServiceKind, InstalledExecutableObservation, InstanceRevision, InterfaceVersionAxis,
    SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

const ENDPOINT_AUDIENCE: &str = "api.anthropic.com";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that qualify one installed Claude Agent ACP instance.
pub struct ClaudeAgentPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl ClaudeAgentPreparationInput {
    /// Creates preparation input for an exact Claude Agent ACP target.
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

    /// Builds the addable ACP preparation input from one admitted record.
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
        if admitted.route_id().as_str() != crate::CLAUDE_AGENT_ACP_ADDABLE_ROUTE_ID {
            return Err(preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.claude_agent.preparation.route_mismatch",
                "Claude Agent preparation requires the admitted ACP route",
            ));
        }
        let binary_field_id = ConfigFieldId::new(crate::CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID)
            .expect("static Claude Agent config field id is valid");
        let binary = admitted.config_ref(&binary_field_id).ok_or_else(|| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.claude_agent.preparation.binary_path_ref_missing",
                "Claude Agent preparation requires the admitted binary-path reference",
            )
        })?;
        let environment_field_id = ConfigFieldId::new(crate::CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID)
            .expect("static Claude Agent config field id is valid");
        let environment = admitted.config_ref(&environment_field_id).ok_or_else(|| {
            preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.claude_agent.preparation.environment_ref_missing",
                "Claude Agent preparation requires the admitted environment reference",
            )
        })?;
        Ok(Self::new(
            admitted.id().clone(),
            instance_revision,
            execution_host_id,
            InstalledExecutableTarget::new(
                ExecutableRef::from_config_field(binary),
                InterfaceVersionAxis::new(crate::CLAUDE_AGENT_ACP_AXIS)
                    .expect("static Claude Agent version axis is valid"),
            ),
            EnvironmentRef::from_config_field(environment),
            access_profile,
            access_evidence,
        ))
    }
}

#[derive(Clone, Debug)]
/// Bounded discovery request used while preparing Claude Agent ACP.
pub struct ClaudeAgentPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl ClaudeAgentPreparationProbe {
    /// Creates a Claude Agent ACP preparation probe.
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
/// Qualified Claude Agent ACP integration ready to prepare operations.
pub struct ClaudeAgentPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl ClaudeAgentPreparedIntegration {
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

    /// Returns the configured API-key or provider-owned local-auth profile.
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

    /// Creates the low-level ACP driver bound to this integration and access posture.
    #[must_use]
    pub fn low_level_driver(&self) -> ClaudeAgentAcpDriver {
        driver_for_access(self.environment.clone(), &self.access_profile)
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
                "swallowtail.claude_agent.preparation.target_drift",
                "Prepared Claude Agent host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers, validates, and prepares one Claude Agent ACP instance.
pub async fn prepare_claude_agent(
    input: ClaudeAgentPreparationInput,
    probe: ClaudeAgentPreparationProbe,
    services: HostServices,
) -> Result<ClaudeAgentPreparedIntegration, PreparationFailure> {
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
    let driver = driver_for_access(input.environment.clone(), &input.access_profile);
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &ClaudeAgentPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != CLAUDE_AGENT_ACP_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.claude_agent.preparation.target_axis_mismatch",
            "Claude Agent preparation target uses a different version axis",
        ));
    }
    let expected_credential_state = match input.access_profile.credential_mechanism() {
        CredentialMechanism::ApiKey
            if input.access_profile.entitlement_metering() == &EntitlementMetering::PayAsYouGo
                && input.access_profile.credential_reference().is_some() =>
        {
            CredentialState::Ready
        }
        CredentialMechanism::LocalUnauthenticated
            if input.access_profile.entitlement_metering()
                == &EntitlementMetering::SubscriptionAllowance
                && input.access_profile.credential_reference().is_none() =>
        {
            CredentialState::NotRequired
        }
        _ => {
            return Err(preparation_failure(
                PreparationStage::AccessEvidence,
                "swallowtail.claude_agent.preparation.access_profile_rejected",
                "Claude Agent requires API-key billing or a locally authenticated subscription",
            ));
        }
    };
    if input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.claude_agent.preparation.access_profile_rejected",
            "Claude Agent requires its maintainer-supported Anthropic access profile",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.credential() != expected_credential_state
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.claude_agent.preparation.access_evidence_mismatch",
            "Claude Agent access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: ClaudeAgentPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<ClaudeAgentPreparedIntegration, PreparationFailure> {
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
            "swallowtail.claude_agent.preparation.observation_mismatch",
            "Claude Agent discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(ClaudeAgentPreparedIntegration {
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}

fn driver_for_access(environment: EnvironmentRef, profile: &AccessProfile) -> ClaudeAgentAcpDriver {
    match profile.credential_reference() {
        Some(credential) => ClaudeAgentAcpDriver::new(environment, credential.clone()),
        None => ClaudeAgentAcpDriver::with_local_auth(environment),
    }
}
