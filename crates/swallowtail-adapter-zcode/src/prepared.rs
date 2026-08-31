use crate::{ZcodeAppServerDriver, ZcodeAppServerMode};
use std::collections::BTreeSet;
use std::path::Path;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, DiscoveryOutcome,
    DiscoveryStatus, EntitlementMetering, ExecutionHostId, HarnessConfigurationPosture,
    HostServiceKind, InstalledExecutableObservation, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ProtocolFacadeId, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

const PROTOCOL_FACADE_ID: &str = "zcode.protocol-stdio-v1";
const POLICY_ID: &str = "zcode-prepared-app-server-run";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Route, interpreter, target, config, and host-access inputs for preparation.
pub struct ZcodePreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    interpreter: ExecutableRef,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl ZcodePreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Creates explicit preparation input for one host-approved Node and `zcode.cjs`.
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        interpreter: ExecutableRef,
        target: InstalledExecutableTarget,
        environment: EnvironmentRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            interpreter,
            target,
            environment,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug)]
/// Bounded installed-executable probe inputs used during preparation.
pub struct ZcodePreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl ZcodePreparationProbe {
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
/// Qualified ZCode integration ready for one structured run.
pub struct ZcodePreparedIntegration {
    interpreter: ExecutableRef,
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl ZcodePreparedIntegration {
    #[must_use]
    /// Returns the host-approved Node interpreter.
    pub const fn interpreter(&self) -> &ExecutableRef {
        &self.interpreter
    }

    #[must_use]
    /// Returns the host-approved settings configuration reference.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    #[must_use]
    /// Returns the exact `zcode.cjs` target admitted during preparation.
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
    /// Creates the low-level driver bound to the prepared settings and mode.
    pub fn low_level_driver(&self, mode: ZcodeAppServerMode) -> ZcodeAppServerDriver {
        ZcodeAppServerDriver::new(self.environment.clone(), mode)
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
                "swallowtail.zcode.app_server.preparation.target_drift",
                "Prepared ZCode host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers, validates, and prepares one exact ZCode app-server payload.
pub async fn prepare_zcode_app_server(
    input: ZcodePreparationInput,
    probe: ZcodePreparationProbe,
    services: HostServices,
) -> Result<ZcodePreparedIntegration, PreparationFailure> {
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
    let driver = ZcodeAppServerDriver::new(input.environment.clone(), ZcodeAppServerMode::plan());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &ZcodePreparationInput) -> Result<(), PreparationFailure> {
    if !interpreter_is_node(input.interpreter.as_host_value()) {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.zcode.app_server.preparation.interpreter_rejected",
            "ZCode preparation requires a host-approved Node interpreter",
        ));
    }
    if input.target.version_axis().as_str() != crate::ZCODE_RELEASE_AXIS
        || !crate::selection::target_is_exact(input.target.executable().as_host_value())
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.zcode.app_server.preparation.target_rejected",
            "ZCode preparation requires the exact packaged runtime target and axis",
        ));
    }
    if input.environment.as_host_value().trim().is_empty() {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.zcode.app_server.preparation.config_missing",
            "ZCode preparation requires a host-approved settings configuration",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering()
            != &EntitlementMetering::SubscriptionAllowance
        || input.access_profile.endpoint_audience().as_str() != crate::ZCODE_CONFIG_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.zcode.app_server.preparation.access_profile_rejected",
            "ZCode requires its provider-supported host configuration profile",
        ));
    }
    if input.access_evidence.status().profile_id() != input.access_profile.id()
        || input.access_evidence.status().support_authority()
            != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.zcode.app_server.preparation.access_evidence_mismatch",
            "ZCode access evidence does not match the selected profile",
        ));
    }
    Ok(())
}

fn interpreter_is_node(value: &str) -> bool {
    matches!(
        Path::new(value)
            .file_name()
            .and_then(std::ffi::OsStr::to_str),
        Some("node" | "node.exe")
    )
}

fn promote(
    input: ZcodePreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<ZcodePreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
        || observation.version().version().as_str() != crate::ZCODE_RELEASE_VERSION
    {
        return Err(failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.zcode.app_server.preparation.observation_mismatch",
            "ZCode discovery does not match the prepared host and release",
        ));
    }
    let instance = configured_instance(&input, &observation)?;
    Ok(ZcodePreparedIntegration {
        interpreter: input.interpreter,
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
    input: &ZcodePreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.zcode.app_server.preparation.target_invalid",
                "ZCode target could not be bound to the configured instance",
            )
        })?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::zcode_app_server_descriptor().identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(PROTOCOL_FACADE_ID).expect("static ZCode protocol facade is valid"),
        InstancePolicyId::new(POLICY_ID).expect("static ZCode policy is valid"),
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
            "swallowtail.zcode.app_server.preparation.discovery_rejected",
            "ZCode executable discovery was not promotable",
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

#[path = "prepared_profile.rs"]
mod profile;
#[cfg(test)]
#[path = "prepared/projection_fixture.rs"]
pub(crate) mod projection_fixture;

pub use profile::{
    ZcodeModelSelection, ZcodePreparedEvidence, ZcodePreparedRun, ZcodeRunProfileInput,
};

#[cfg(test)]
mod tests {
    use super::{
        ZcodePreparationInput, ZcodePreparedIntegration, configured_instance, validate_input,
    };
    use crate::{
        ZCODE_CONFIG_AUDIENCE, ZCODE_EXECUTABLE_BASENAME, ZCODE_RELEASE_AXIS,
        ZCODE_RELEASE_VERSION, ZcodeAppServerMode, zcode_access_profile, zcode_app_server_claim,
        zcode_release_binding,
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

    fn input() -> ZcodePreparationInput {
        let access_id = AccessProfileId::new("zcode.fixture.access").unwrap();
        ZcodePreparationInput::new(
            ConfiguredInstanceId::new("zcode.fixture.instance").unwrap(),
            InstanceRevision::new("0.16.3").unwrap(),
            ExecutionHostId::new("zcode.fixture.host").unwrap(),
            ExecutableRef::new("/fixture/bin/node").unwrap(),
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!("/fixture/vendor/{ZCODE_EXECUTABLE_BASENAME}")).unwrap(),
                InterfaceVersionAxis::new(ZCODE_RELEASE_AXIS).unwrap(),
            ),
            EnvironmentRef::new("/fixture/settings.json").unwrap(),
            zcode_access_profile(access_id.clone()),
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
    fn preparation_requires_node_payload_and_host_configuration() {
        let mut wrong_interpreter = input();
        wrong_interpreter.interpreter = ExecutableRef::new("/fixture/bin/zcode.js").unwrap();
        let error = validate_input(&wrong_interpreter).unwrap_err();
        assert_eq!(
            error.stage(),
            swallowtail_runtime::PreparationStage::TargetSelection
        );
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.zcode.app_server.preparation.interpreter_rejected"
        );

        let mut wrong_target = input();
        wrong_target.target = InstalledExecutableTarget::new(
            ExecutableRef::new("/fixture/bin/zcode.js").unwrap(),
            InterfaceVersionAxis::new(ZCODE_RELEASE_AXIS).unwrap(),
        );
        let error = validate_input(&wrong_target).unwrap_err();
        assert_eq!(
            error.stage(),
            swallowtail_runtime::PreparationStage::TargetSelection
        );

        assert!(EnvironmentRef::new("  ").is_err());
        assert_eq!(
            input().access_profile.endpoint_audience().as_str(),
            ZCODE_CONFIG_AUDIENCE
        );
        assert!(ZcodeAppServerMode::new("yolo").is_none());
    }

    fn prepared_integration() -> ZcodePreparedIntegration {
        let input = input();
        let observation = InstalledExecutableObservation::classify(
            input.execution_host_id.clone(),
            zcode_release_binding(ZCODE_RELEASE_VERSION).unwrap(),
            &zcode_app_server_claim(),
        )
        .unwrap();
        let instance = configured_instance(&input, &observation).unwrap();
        ZcodePreparedIntegration {
            interpreter: input.interpreter,
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
    fn prepared_run_keeps_mode_provider_model_and_cwd_explicit() {
        let prepared = prepared_integration();
        let run = prepared
            .prepare_run(crate::ZcodeRunProfileInput::new(
                RequestId::new("zcode.fixture.run").unwrap(),
                crate::ZcodeModelSelection::new(
                    ModelRouteId::new("zcode.fixture.route").unwrap(),
                    ModelRouteRevision::new("fixture-v1").unwrap(),
                    ProviderId::new("zai").unwrap(),
                    ModelId::new("fixture-model").unwrap(),
                ),
                ZcodeAppServerMode::plan(),
                OperationContent::new("fixture prompt").unwrap(),
                WorkingResourceRef::new("zcode.fixture.workspace").unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("explicit run prepares");
        assert_eq!(run.mode().as_str(), "plan");
        assert_eq!(
            run.plan().provider_id().map(ProviderId::as_str),
            Some("zai")
        );
        assert_eq!(
            run.plan().model_id().map(ModelId::as_str),
            Some("fixture-model")
        );
        assert_eq!(
            run.request()
                .working_resource()
                .map(WorkingResourceRef::as_host_value),
            Some("zcode.fixture.workspace")
        );
        assert_eq!(run.low_level_driver().mode().as_str(), "plan");
        assert_eq!(prepared.interpreter().as_host_value(), "/fixture/bin/node");
        assert!(!run.mode().as_str().eq_ignore_ascii_case("yolo"));
    }
}
