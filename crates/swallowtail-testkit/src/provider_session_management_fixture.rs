use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId,
    CapabilityRequirement, ConfiguredInstance, CredentialMechanism, CredentialRef, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    OperationRequirements, OperationShape, PreflightContext, PreflightFailure, PreflightPlan,
    ProtocolFacadeId, ProviderSessionActivityEvidence, ProviderSessionAffectedScope,
    ProviderSessionBindingOrigin, ProviderSessionCancellationPosture,
    ProviderSessionManagementAction, RuntimeReadiness, SessionRef, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    AccessEvidenceSourceId, Deadline, InvalidProviderSessionManagementBinding,
    PreparedAccessEvidence, ProviderSessionManagementAgreement, ProviderSessionManagementBinding,
    ProviderSessionManagementPlan,
};

use crate::ExecutionTopologyFixture;

mod builder;
mod drift;

use builder::{capabilities, driver, initial_state, value};
pub use drift::ProviderSessionManagementBindingDrift;

const VERSION_AXIS: &str = "fixture.session-rpc";
const QUALIFIED_VERSION: &str = "1.2.0";
const UNVERIFIED_VERSION: &str = "1.3.0";
const INCOMPATIBLE_VERSION: &str = "0.9.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Interface-compatibility scenario selected by the management fixture.
pub enum ProviderSessionManagementFixtureCase {
    /// A qualified, supported interface version.
    Qualified,
    /// A newer version without qualification evidence.
    UnverifiedNewer,
    /// An interface version outside the compatible window.
    Incompatible,
    /// The requested management action is not advertised.
    Unsupported,
}

/// Composable provider-neutral management fixture for one exact host topology.
pub struct ProviderSessionManagementFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    working_resource: swallowtail_runtime::WorkingResourceRef,
    action: ProviderSessionManagementAction,
}

impl ProviderSessionManagementFixture {
    /// Builds a fixture for a local execution topology.
    #[must_use]
    pub fn local(
        case: ProviderSessionManagementFixtureCase,
        action: ProviderSessionManagementAction,
    ) -> Self {
        Self::for_topology(ExecutionTopologyFixture::local(), case, action)
    }

    /// Builds a fixture for an authoritative remote execution topology.
    #[must_use]
    pub fn remote_authoritative(
        case: ProviderSessionManagementFixtureCase,
        action: ProviderSessionManagementAction,
    ) -> Self {
        Self::for_topology(
            ExecutionTopologyFixture::remote_authoritative(),
            case,
            action,
        )
    }

    /// Builds a fixture for an explicit execution topology.
    #[must_use]
    pub fn for_topology(
        topology: ExecutionTopologyFixture,
        case: ProviderSessionManagementFixtureCase,
        action: ProviderSessionManagementAction,
    ) -> Self {
        let adapter_id = value(AdapterId::new, "fixture.session-management");
        let access_id = value(AccessProfileId::new, "fixture.session-access");
        let version = match case {
            ProviderSessionManagementFixtureCase::Qualified
            | ProviderSessionManagementFixtureCase::Unsupported => QUALIFIED_VERSION,
            ProviderSessionManagementFixtureCase::UnverifiedNewer => UNVERIFIED_VERSION,
            ProviderSessionManagementFixtureCase::Incompatible => INCOMPATIBLE_VERSION,
        };
        let driver = driver(adapter_id.clone());
        let capabilities = capabilities(case);
        let instance = ConfiguredInstance::new(
            topology.configured_instance_id().clone(),
            value(InstanceRevision::new, "fixture-revision-1"),
            adapter_id,
            topology.execution_host_id().clone(),
            topology.instance_target().clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            value(ProtocolFacadeId::new, "fixture.session-facade"),
            value(InstancePolicyId::new, "fixture.session-policy"),
            capabilities,
        )
        .with_interface_versions([InterfaceVersionBinding::new(
            value(InterfaceVersionAxis::new, VERSION_AXIS),
            value(InterfaceVersion::new, version),
        )]);
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::AutomationToken,
            EntitlementMetering::SubscriptionAllowance,
            value(EndpointAudience::new, "fixture.session-audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(value(CredentialRef::new, "fixture.private.credential"));
        let access_status = AccessStatus::new(
            access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        );
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionManagement,
            DriverRole::ProviderSessionManagement,
            topology.execution_host_id().clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ])
        .with_capabilities([CapabilityRequirement::new(action.required_capability(), [])])
        .with_interface_versions(instance.interface_versions().cloned());

        Self {
            driver,
            instance,
            access_profile,
            access_status,
            requirements,
            working_resource: topology.working_resource().clone(),
            action,
        }
    }

    /// Runs provider-neutral preflight for the management operation.
    pub fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                &self.access_status,
                [
                    HostServiceKind::Task,
                    HostServiceKind::Time,
                    HostServiceKind::Credential,
                    HostServiceKind::WorkingResource,
                ],
            ),
            &self.requirements,
        )
    }

    /// Builds the retained provider-session binding used by the operation.
    pub fn binding(
        &self,
    ) -> Result<ProviderSessionManagementBinding, InvalidProviderSessionManagementBinding> {
        ProviderSessionManagementBinding::from_bound_session(
            value(SessionRef::new, "fixture.private.provider-session"),
            &self.driver,
            &self.instance,
            PreparedAccessEvidence::observed(
                self.access_status.clone(),
                value(
                    AccessEvidenceSourceId::new,
                    "fixture.private.access-observation",
                ),
            ),
            Some(self.working_resource.clone()),
            ProviderSessionBindingOrigin::ExplicitlyImported,
        )
    }

    /// Builds a prepared management plan with the optional deadline.
    pub fn plan(
        &self,
        deadline: Option<Deadline>,
    ) -> Result<ProviderSessionManagementPlan, swallowtail_runtime::RuntimeFailure> {
        let preflight = self.preflight().map_err(|failure| {
            swallowtail_runtime::RuntimeFailure::new(failure.diagnostic().clone())
        })?;
        let binding = self.binding().map_err(|failure| {
            swallowtail_runtime::RuntimeFailure::new(failure.diagnostic().clone())
        })?;
        ProviderSessionManagementPlan::new(
            preflight,
            ProviderSessionManagementAgreement::new(
                binding,
                self.action,
                initial_state(self.action),
                ProviderSessionAffectedScope::TargetOnly,
                ProviderSessionActivityEvidence::CallerAssertedInactive,
                ProviderSessionCancellationPosture::BeforeDispatchOnly,
                deadline,
            ),
        )
    }

    #[must_use]
    /// Returns the fixture driver descriptor.
    pub const fn driver(&self) -> &DriverDescriptor {
        &self.driver
    }

    #[must_use]
    /// Returns the configured provider instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    #[must_use]
    /// Returns the access profile bound to the instance.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns the observed access status.
    pub const fn access_status(&self) -> &AccessStatus {
        &self.access_status
    }

    #[must_use]
    /// Returns the operation requirements used by preflight.
    pub const fn requirements(&self) -> &OperationRequirements {
        &self.requirements
    }
}
