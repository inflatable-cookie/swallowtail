use crate::{GEMINI_CLI_ACP_AXIS, GeminiAcpDriver};
#[path = "prepared/failure.rs"]
mod failure;
#[path = "prepared/instance.rs"]
pub(crate) mod instance;
use failure::{discovery_outcome_failure, discovery_runtime_failure, preparation_failure};
use instance::configured_instance;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, DiscoveryOutcome,
    EntitlementMetering, ExecutionHostId, HostServiceKind, InstalledExecutableObservation,
    InstanceRevision, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

const ENDPOINT_AUDIENCE: &str = "gemini-developer-api";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that bind one installed Gemini CLI ACP instance before discovery.
pub struct GeminiPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl GeminiPreparationInput {
    /// Creates an ACP preparation input with explicit host, target, and access evidence.
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
pub struct GeminiPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl GeminiPreparationProbe {
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
/// Discovered and preflight-ready Gemini CLI ACP integration.
pub struct GeminiPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl GeminiPreparedIntegration {
    /// Returns the host-private environment used to launch Gemini CLI.
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

    /// Returns the admitted access evidence used during preparation.
    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Returns the configured ACP instance created from discovery evidence.
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates the host services available when preparation completed.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Reconstructs the low-level ACP driver bound to prepared inputs.
    #[must_use]
    pub fn low_level_driver(&self) -> GeminiAcpDriver {
        GeminiAcpDriver::new(
            self.environment.clone(),
            self.access_profile
                .credential_reference()
                .expect("prepared Gemini access has one credential reference")
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
                "swallowtail.gemini.preparation.target_drift",
                "Prepared Gemini host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers and prepares one exact installed Gemini CLI ACP route.
pub async fn prepare_gemini_acp(
    input: GeminiPreparationInput,
    probe: GeminiPreparationProbe,
    services: HostServices,
) -> Result<GeminiPreparedIntegration, PreparationFailure> {
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
    let driver = GeminiAcpDriver::new(
        input.environment.clone(),
        credential_reference(&input.access_profile)?.clone(),
    );
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &GeminiPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != GEMINI_CLI_ACP_AXIS {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.gemini.preparation.target_axis_mismatch",
            "Gemini preparation target uses a different version axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.gemini.preparation.access_profile_rejected",
            "Gemini ACP requires its provider-supported Developer API-key profile",
        ));
    }
    let _ = credential_reference(&input.access_profile)?;
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.gemini.preparation.access_evidence_mismatch",
            "Gemini access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: GeminiPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<GeminiPreparedIntegration, PreparationFailure> {
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
            "swallowtail.gemini.preparation.observation_mismatch",
            "Gemini discovery observation does not match the prepared target",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(GeminiPreparedIntegration {
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
            "swallowtail.gemini.preparation.credential_reference_missing",
            "Gemini ACP requires one Developer API-key credential reference",
        )
    })
}
