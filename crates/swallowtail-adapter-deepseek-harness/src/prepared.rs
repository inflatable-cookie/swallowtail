#[path = "prepared_profile.rs"]
mod profile;

pub use profile::{
    DeepSeekHarnessModelSelection, DeepSeekHarnessPreparedEvidence, DeepSeekHarnessPreparedRun,
    DeepSeekHarnessRunProfileInput,
};

use crate::DeepSeekHarnessJsonRpcDriver;
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

const PROTOCOL_FACADE_ID: &str = "deepseek-harness.sdk-jsonrpc-v1";
const POLICY_ID: &str = "deepseek-harness-prepared-read-only-run";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Route, target, environment, and host-access inputs for preparation.
pub struct DeepSeekHarnessPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl DeepSeekHarnessPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Creates explicit preparation input for one host-approved runtime-bin.
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
/// Bounded installed-executable probe inputs used during preparation.
pub struct DeepSeekHarnessPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl DeepSeekHarnessPreparationProbe {
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
/// Qualified DeepSeek Harness integration ready for one structured run.
pub struct DeepSeekHarnessPreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl DeepSeekHarnessPreparedIntegration {
    #[must_use]
    /// Returns the host-approved Cordis configuration reference.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    #[must_use]
    /// Returns the exact executable target admitted during preparation.
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    #[must_use]
    /// Returns the executable observation admitted during preparation.
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    /// Returns the host-owned, unauthenticated access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns immutable access evidence admitted during preparation.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    /// Returns the exact configured host-owned instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates over host services present when preparation succeeded.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    /// Creates the low-level driver bound to the prepared Cordis reference.
    pub fn low_level_driver(&self) -> DeepSeekHarnessJsonRpcDriver {
        DeepSeekHarnessJsonRpcDriver::new(self.environment.clone())
    }

    /// Rejects host or executable drift from the prepared binding.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.deepseek_harness.preparation.target_drift",
                "Prepared DeepSeek Harness host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers, validates, and prepares one exact DeepSeek Harness runtime-bin.
pub async fn prepare_deepseek_harness_jsonrpc(
    input: DeepSeekHarnessPreparationInput,
    probe: DeepSeekHarnessPreparationProbe,
    services: HostServices,
) -> Result<DeepSeekHarnessPreparedIntegration, PreparationFailure> {
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
    let driver = DeepSeekHarnessJsonRpcDriver::new(input.environment.clone());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &DeepSeekHarnessPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::DEEPSEEK_HARNESS_RELEASE_AXIS
        || !crate::selection::target_is_exact(input.target.executable().as_host_value())
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.deepseek_harness.preparation.target_rejected",
            "DeepSeek Harness preparation requires the exact packaged runtime target and axis",
        ));
    }
    if input.environment.as_host_value().trim().is_empty() {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.deepseek_harness.preparation.config_missing",
            "DeepSeek Harness preparation requires a host-approved Cordis configuration",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering()
            != &EntitlementMetering::SubscriptionAllowance
        || input.access_profile.endpoint_audience().as_str()
            != crate::DEEPSEEK_HARNESS_CONFIG_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.deepseek_harness.preparation.access_profile_rejected",
            "DeepSeek Harness requires its provider-supported host configuration profile",
        ));
    }
    if input.access_evidence.status().profile_id() != input.access_profile.id()
        || input.access_evidence.status().support_authority()
            != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.deepseek_harness.preparation.access_evidence_mismatch",
            "DeepSeek Harness access evidence does not match the selected profile",
        ));
    }
    Ok(())
}

fn promote(
    input: DeepSeekHarnessPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<DeepSeekHarnessPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
        || observation.version().version().as_str() != crate::DEEPSEEK_HARNESS_RELEASE_VERSION
    {
        return Err(failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.deepseek_harness.preparation.observation_mismatch",
            "DeepSeek Harness discovery does not match the prepared host and release",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(DeepSeekHarnessPreparedIntegration {
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
    input: &DeepSeekHarnessPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.deepseek_harness.preparation.target_invalid",
                "DeepSeek Harness target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::deepseek_harness_jsonrpc_descriptor()
            .identity()
            .id()
            .clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(PROTOCOL_FACADE_ID)
            .expect("static DeepSeek Harness protocol facade is valid"),
        InstancePolicyId::new(POLICY_ID).expect("static DeepSeek Harness policy is valid"),
        profile::advertised_capabilities(),
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
            "swallowtail.deepseek_harness.preparation.discovery_rejected",
            "DeepSeek Harness executable discovery was not promotable",
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

#[cfg(test)]
pub(crate) mod tests {
    use super::{
        DeepSeekHarnessPreparationInput, DeepSeekHarnessPreparedIntegration, configured_instance,
        validate_input,
    };
    use crate::{
        DEEPSEEK_HARNESS_CONFIG_AUDIENCE, DEEPSEEK_HARNESS_EXECUTABLE_BASENAME,
        DEEPSEEK_HARNESS_RELEASE_AXIS, DEEPSEEK_HARNESS_RELEASE_VERSION,
        deepseek_harness_access_profile, deepseek_harness_jsonrpc_claim,
        deepseek_harness_release_binding,
    };
    use swallowtail_core::{
        AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState,
        EndpointAuthorization, EntitlementState, ExecutionHostId, HostServiceKind,
        InstalledExecutableObservation, InstanceRevision, InterfaceVersionAxis, ModelId,
        ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness, SupportAuthority,
    };
    use swallowtail_runtime::{
        Deadline, EnvironmentRef, ExecutableRef, InstalledExecutableTarget, MonotonicInstant,
        OperationContent, RequestId, WorkingResourceRef,
    };

    fn input() -> DeepSeekHarnessPreparationInput {
        let access_id = AccessProfileId::new("deepseek-harness.fixture.access").unwrap();
        DeepSeekHarnessPreparationInput::new(
            ConfiguredInstanceId::new("deepseek-harness.fixture.instance").unwrap(),
            InstanceRevision::new("rc6").unwrap(),
            ExecutionHostId::new("deepseek-harness.fixture.host").unwrap(),
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!(
                    "/fixture/bin/{DEEPSEEK_HARNESS_EXECUTABLE_BASENAME}"
                ))
                .unwrap(),
                InterfaceVersionAxis::new(DEEPSEEK_HARNESS_RELEASE_AXIS).unwrap(),
            ),
            EnvironmentRef::new("deepseek-harness.fixture.cordis").unwrap(),
            deepseek_harness_access_profile(access_id.clone()),
            swallowtail_runtime::PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        )
    }

    #[test]
    fn preparation_requires_exact_target_and_host_configuration() {
        let mut wrong_target = input();
        wrong_target.target = InstalledExecutableTarget::new(
            ExecutableRef::new("/fixture/bin/dsh-jsonrpc-agent").unwrap(),
            InterfaceVersionAxis::new(DEEPSEEK_HARNESS_RELEASE_AXIS).unwrap(),
        );
        let error = validate_input(&wrong_target).unwrap_err();
        assert_eq!(
            error.stage(),
            swallowtail_runtime::PreparationStage::TargetSelection
        );

        assert!(EnvironmentRef::new("  ").is_err());

        assert_eq!(
            input().access_profile.endpoint_audience().as_str(),
            DEEPSEEK_HARNESS_CONFIG_AUDIENCE
        );
    }

    pub(crate) fn prepared_integration() -> DeepSeekHarnessPreparedIntegration {
        let input = input();
        let observation = InstalledExecutableObservation::classify(
            input.execution_host_id.clone(),
            deepseek_harness_release_binding(DEEPSEEK_HARNESS_RELEASE_VERSION).unwrap(),
            &deepseek_harness_jsonrpc_claim(),
        )
        .unwrap();
        let instance = configured_instance(&input, &observation).unwrap();
        DeepSeekHarnessPreparedIntegration {
            environment: input.environment,
            target: input.target,
            observation,
            access_profile: input.access_profile,
            access_evidence: input.access_evidence,
            instance,
            available_host_services: [
                HostServiceKind::Task,
                HostServiceKind::Process,
                HostServiceKind::Time,
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn prepared_run_keeps_provider_model_and_host_policy_explicit() {
        let prepared = prepared_integration();
        let run = prepared
            .prepare_run(crate::DeepSeekHarnessRunProfileInput::new(
                RequestId::new("deepseek-harness.fixture.run").unwrap(),
                crate::DeepSeekHarnessModelSelection::new(
                    ModelRouteId::new("deepseek-harness.fixture.route").unwrap(),
                    ModelRouteRevision::new("fixture-v1").unwrap(),
                    ProviderId::new("fixture-provider").unwrap(),
                    ModelId::new("fixture-model").unwrap(),
                ),
                OperationContent::new("fixture prompt").unwrap(),
                WorkingResourceRef::new("deepseek-harness.fixture.workspace").unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("explicit run prepares");
        assert_eq!(
            run.plan().provider_id().map(ProviderId::as_str),
            Some("fixture-provider")
        );
        assert_eq!(
            run.plan().model_id().map(ModelId::as_str),
            Some("fixture-model")
        );
        assert_eq!(
            run.plan().requirements().harness_isolation(),
            Some(swallowtail_core::HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            run.request()
                .working_resource()
                .map(WorkingResourceRef::as_host_value),
            Some("deepseek-harness.fixture.workspace")
        );
    }
}
