#[path = "prepared/acp.rs"]
mod acp;
#[path = "prepared/activity.rs"]
pub(crate) mod activity;
#[path = "prepared/catalogue.rs"]
mod catalogue;
#[path = "prepared/headless.rs"]
mod headless;
#[path = "prepared/plan.rs"]
pub(crate) mod plan;

pub use acp::{CursorAcpSessionProfileInput, CursorPreparedAcpSession};
pub use catalogue::{CursorCatalogueProfileInput, CursorPreparedCatalogue};
pub use headless::{
    CursorHeadlessModelSelection, CursorHeadlessRunProfileInput, CursorPreparedHeadlessRun,
};

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
pub enum CursorPreparedDriver {
    Catalogue,
    Acp,
    Headless,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPreparationInput {
    driver: CursorPreparedDriver,
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl CursorPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        driver: CursorPreparedDriver,
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
    pub const fn driver(&self) -> CursorPreparedDriver {
        self.driver
    }
}

#[derive(Clone, Debug)]
pub struct CursorPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl CursorPreparationProbe {
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
pub enum CursorPreparedIntegration {
    Catalogue(CursorPreparedCatalogueIntegration),
    Acp(CursorPreparedAcpIntegration),
    Headless(CursorPreparedHeadlessIntegration),
}

impl CursorPreparedIntegration {
    #[must_use]
    pub const fn driver(&self) -> CursorPreparedDriver {
        match self {
            Self::Catalogue(_) => CursorPreparedDriver::Catalogue,
            Self::Acp(_) => CursorPreparedDriver::Acp,
            Self::Headless(_) => CursorPreparedDriver::Headless,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPreparedCatalogueIntegration(PreparedState);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPreparedAcpIntegration(PreparedState);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPreparedHeadlessIntegration(PreparedState);

macro_rules! integration_accessors {
    ($type:ty) => {
        impl $type {
            #[must_use]
            pub const fn environment(&self) -> &EnvironmentRef {
                &self.0.environment
            }

            #[must_use]
            pub const fn target(&self) -> &InstalledExecutableTarget {
                &self.0.target
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

integration_accessors!(CursorPreparedCatalogueIntegration);
integration_accessors!(CursorPreparedAcpIntegration);
integration_accessors!(CursorPreparedHeadlessIntegration);

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreparedState {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

pub async fn prepare_cursor(
    input: CursorPreparationInput,
    probe: CursorPreparationProbe,
    services: HostServices,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
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
        CursorPreparedDriver::Catalogue => {
            crate::CursorCatalogueDriver::new(input.environment.clone())
                .discover_installed_executable(request, services)
                .await
        }
        CursorPreparedDriver::Acp => {
            crate::CursorAcpDriver::new(input.environment.clone())
                .discover_installed_executable(request, services)
                .await
        }
        CursorPreparedDriver::Headless => {
            crate::CursorHeadlessDriver::new(input.environment.clone())
                .discover_installed_executable(request, services)
                .await
        }
    }
    .map_err(discovery_runtime_failure)?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &CursorPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::CURSOR_AGENT_RELEASE_AXIS {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.cursor.preparation.target_axis_mismatch",
            "Cursor preparation target uses a different release axis",
        ));
    }
    if input.access_profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering()
            != &EntitlementMetering::SubscriptionAllowance
        || input.access_profile.endpoint_audience().as_str() != crate::CURSOR_SUBSCRIPTION_AUDIENCE
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.cursor.preparation.access_profile_rejected",
            "Cursor requires its provider-supported delegated subscription profile",
        ));
    }
    if input.access_evidence.status().profile_id() != input.access_profile.id()
        || input.access_evidence.status().support_authority()
            != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.cursor.preparation.access_evidence_mismatch",
            "Cursor access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn promote(
    input: CursorPreparationInput,
    outcome: DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
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
            "swallowtail.cursor.preparation.observation_mismatch",
            "Cursor discovery observation does not match the prepared target",
        ));
    }
    let state = PreparedState {
        instance: configured_instance(&input, &observation)?,
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        available_host_services,
    };
    Ok(match input.driver {
        CursorPreparedDriver::Catalogue => {
            CursorPreparedIntegration::Catalogue(CursorPreparedCatalogueIntegration(state))
        }
        CursorPreparedDriver::Acp => {
            CursorPreparedIntegration::Acp(CursorPreparedAcpIntegration(state))
        }
        CursorPreparedDriver::Headless => {
            CursorPreparedIntegration::Headless(CursorPreparedHeadlessIntegration(state))
        }
    })
}

fn configured_instance(
    input: &CursorPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target =
        InstanceTargetRef::new(input.target.executable().as_host_value()).map_err(|_| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.cursor.preparation.target_invalid",
                "Cursor approved target could not be bound to the configured instance",
            )
        })?;
    let (descriptor, facade, policy, capabilities) = plan::route_instance_shape(input.driver);
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        descriptor.identity().id().clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(facade).expect("static Cursor facade is valid"),
        InstancePolicyId::new(policy).expect("static Cursor policy is valid"),
        capabilities,
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

fn discovery_runtime_failure(error: swallowtail_runtime::RuntimeFailure) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.cursor.discovery_axis_mismatch"
        | "swallowtail.installed_executable.host_services_missing"
        | "swallowtail.execution_host_mismatch" => PreparationStage::TargetSelection,
        _ => PreparationStage::ProcessSpawn,
    };
    PreparationFailure::new(
        stage,
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
            "swallowtail.cursor.preparation.discovery_rejected",
            "Cursor executable discovery was not promotable",
        )
    });
    PreparationFailure::new(stage, swallowtail_core::Diagnostic::new(diagnostic))
}

pub(crate) fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
