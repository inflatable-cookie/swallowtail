#[path = "prepared/activity.rs"]
mod activity;
#[path = "prepared/catalogue.rs"]
mod catalogue;
#[path = "prepared/common.rs"]
mod common;
#[path = "prepared/run.rs"]
mod run;
#[path = "prepared/session.rs"]
mod session;

pub use catalogue::{AntigravityCatalogueProfileInput, AntigravityPreparedCatalogue};
pub use run::{
    AntigravityHeadlessModelSelection, AntigravityHeadlessRunProfileInput,
    AntigravityPreparedHeadlessRun,
};
pub use session::{AntigravityContinuationProfileInput, AntigravityPreparedContinuation};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AntigravityPreparedDriver {
    Catalogue,
    Headless,
    Continuation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntigravityPreparationInput {
    driver: AntigravityPreparedDriver,
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl AntigravityPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        driver: AntigravityPreparedDriver,
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

    #[must_use]
    pub const fn driver(&self) -> AntigravityPreparedDriver {
        self.driver
    }
}

#[derive(Clone, Debug)]
pub struct AntigravityPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl AntigravityPreparationProbe {
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
pub enum AntigravityPreparedIntegration {
    Catalogue(AntigravityPreparedCatalogueIntegration),
    Headless(AntigravityPreparedHeadlessIntegration),
    Continuation(AntigravityPreparedContinuationIntegration),
}

impl AntigravityPreparedIntegration {
    #[must_use]
    pub const fn driver(&self) -> AntigravityPreparedDriver {
        match self {
            Self::Catalogue(_) => AntigravityPreparedDriver::Catalogue,
            Self::Headless(_) => AntigravityPreparedDriver::Headless,
            Self::Continuation(_) => AntigravityPreparedDriver::Continuation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntigravityPreparedCatalogueIntegration(PreparedState);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntigravityPreparedHeadlessIntegration(PreparedState);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntigravityPreparedContinuationIntegration(PreparedState);

macro_rules! integration_accessors {
    ($type:ty) => {
        impl $type {
            #[must_use]
            pub const fn environment(&self) -> &EnvironmentRef {
                &self.0.environment
            }

            #[must_use]
            pub const fn observation(&self) -> &InstalledExecutableObservation {
                &self.0.observation
            }

            #[must_use]
            pub const fn access_profile(&self) -> &AccessProfile {
                &self.0.access_profile
            }

            #[must_use]
            pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
                &self.0.access_evidence
            }

            #[must_use]
            pub const fn instance(&self) -> &ConfiguredInstance {
                &self.0.instance
            }

            pub fn available_host_services(
                &self,
            ) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
                self.0.available_host_services.iter().copied()
            }
        }
    };
}

integration_accessors!(AntigravityPreparedCatalogueIntegration);
integration_accessors!(AntigravityPreparedHeadlessIntegration);
integration_accessors!(AntigravityPreparedContinuationIntegration);

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreparedState {
    environment: EnvironmentRef,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

pub async fn prepare_antigravity(
    input: AntigravityPreparationInput,
    probe: AntigravityPreparationProbe,
    services: HostServices,
) -> Result<AntigravityPreparedIntegration, PreparationFailure> {
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
    let outcome = match input.driver {
        AntigravityPreparedDriver::Catalogue => {
            crate::AntigravityCatalogueDriver::new(input.environment.clone())
                .discover_installed_executable(request, services)
                .await
        }
        AntigravityPreparedDriver::Headless | AntigravityPreparedDriver::Continuation => {
            crate::AntigravityHeadlessDriver::new(input.environment.clone())
                .discover_installed_executable(request, services)
                .await
        }
    }
    .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &AntigravityPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::ANTIGRAVITY_RELEASE_AXIS {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.antigravity.preparation.target_axis_mismatch",
            "Antigravity preparation target uses a different release axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering()
            != &EntitlementMetering::SubscriptionAllowance
        || input.access_profile.endpoint_audience().as_str()
            != crate::ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.antigravity.preparation.access_profile_rejected",
            "Antigravity requires its provider-supported personal Google profile",
        ));
    }
    if input.access_evidence.status().profile_id() != input.access_profile.id()
        || input.access_evidence.status().support_authority()
            != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.antigravity.preparation.access_evidence_mismatch",
            "Antigravity access evidence does not match the selected profile",
        ));
    }
    Ok(())
}

fn promote(
    input: AntigravityPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<AntigravityPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| discovery_outcome_failure(&outcome))?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
    {
        return Err(failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.antigravity.preparation.observation_mismatch",
            "Antigravity discovery observation does not match the prepared target",
        ));
    }
    let state = PreparedState {
        instance: configured_instance(&input, &observation)?,
        environment: input.environment,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services,
    };
    Ok(match input.driver {
        AntigravityPreparedDriver::Catalogue => AntigravityPreparedIntegration::Catalogue(
            AntigravityPreparedCatalogueIntegration(state),
        ),
        AntigravityPreparedDriver::Headless => {
            AntigravityPreparedIntegration::Headless(AntigravityPreparedHeadlessIntegration(state))
        }
        AntigravityPreparedDriver::Continuation => AntigravityPreparedIntegration::Continuation(
            AntigravityPreparedContinuationIntegration(state),
        ),
    })
}

fn configured_instance(
    input: &AntigravityPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.antigravity.preparation.target_invalid",
                "Antigravity target could not be bound to the configured instance",
            )
        })?;
    let (descriptor, facade, policy, capabilities) = common::route_instance_shape(input.driver);
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        descriptor.identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(facade).expect("static facade is valid"),
        InstancePolicyId::new(policy).expect("static policy is valid"),
        capabilities,
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
            "swallowtail.antigravity.preparation.discovery_rejected",
            "Antigravity executable discovery was not promotable",
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
