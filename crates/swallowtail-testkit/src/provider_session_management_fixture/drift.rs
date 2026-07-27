use super::builder::{driver_with, value};
use super::{ProviderSessionManagementFixture, QUALIFIED_VERSION, VERSION_AXIS};
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, Capability, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    IntegrationFamilyId, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    ProtocolFacadeId, ProviderSessionBindingOrigin, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    AccessEvidenceSourceId, InvalidProviderSessionManagementBinding, PreparedAccessEvidence,
    ProviderSessionManagementBinding, WorkingResourceRef,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderSessionManagementBindingDrift {
    ProviderSessionReference,
    DriverIdentity,
    IntegrationFamily,
    TransportFamily,
    ConfiguredInstance,
    InstanceRevision,
    ExecutionHost,
    InstanceTarget,
    ProtocolFacade,
    AccessProfile,
    InterfaceVersion,
    Capabilities,
    WorkingResource,
    Origin,
}

impl ProviderSessionManagementFixture {
    pub fn drifted_binding(
        &self,
        drift: ProviderSessionManagementBindingDrift,
    ) -> Result<ProviderSessionManagementBinding, InvalidProviderSessionManagementBinding> {
        let driver_id = if drift == ProviderSessionManagementBindingDrift::DriverIdentity {
            value(AdapterId::new, "fixture.session-management.drifted")
        } else {
            self.instance.driver_id().clone()
        };
        let integration = if drift == ProviderSessionManagementBindingDrift::IntegrationFamily {
            value(IntegrationFamilyId::new, "fixture-session-drifted")
        } else {
            self.driver.integration_family().clone()
        };
        let transport = if drift == ProviderSessionManagementBindingDrift::TransportFamily {
            value(TransportFamilyId::new, "fixture-session-rpc-drifted")
        } else {
            self.driver.transport_family().clone()
        };
        let driver = driver_with(driver_id.clone(), integration, transport);
        let access_id = if drift == ProviderSessionManagementBindingDrift::AccessProfile {
            value(AccessProfileId::new, "fixture.session-access.drifted")
        } else {
            self.instance.access_profile_id().clone()
        };
        let version = if drift == ProviderSessionManagementBindingDrift::InterfaceVersion {
            "1.1.0"
        } else {
            QUALIFIED_VERSION
        };
        let instance = ConfiguredInstance::new(
            if drift == ProviderSessionManagementBindingDrift::ConfiguredInstance {
                value(ConfiguredInstanceId::new, "fixture.instance.drifted")
            } else {
                self.instance.id().clone()
            },
            if drift == ProviderSessionManagementBindingDrift::InstanceRevision {
                value(InstanceRevision::new, "fixture-revision-drifted")
            } else {
                self.instance.revision().clone()
            },
            driver_id,
            if drift == ProviderSessionManagementBindingDrift::ExecutionHost {
                value(ExecutionHostId::new, "fixture.host.drifted")
            } else {
                self.instance.execution_host_id().clone()
            },
            if drift == ProviderSessionManagementBindingDrift::InstanceTarget {
                value(InstanceTargetRef::new, "fixture.target.drifted")
            } else {
                self.instance.target_reference().clone()
            },
            self.instance.ownership(),
            access_id.clone(),
            self.instance.support_authority(),
            if drift == ProviderSessionManagementBindingDrift::ProtocolFacade {
                value(ProtocolFacadeId::new, "fixture.session-facade.drifted")
            } else {
                self.instance.protocol_facade_id().clone()
            },
            self.instance.policy_id().clone(),
            if drift == ProviderSessionManagementBindingDrift::Capabilities {
                CapabilityProfile::new([CapabilityRequirement::new(
                    Capability::ProviderSessionArchive,
                    [],
                )])
            } else {
                self.instance.capabilities().clone()
            },
        )
        .with_interface_versions([InterfaceVersionBinding::new(
            value(InterfaceVersionAxis::new, VERSION_AXIS),
            value(InterfaceVersion::new, version),
        )]);
        let status = AccessStatus::new(
            access_id,
            self.access_status.credential(),
            self.access_status.entitlement(),
            self.access_status.endpoint_authorization(),
            self.access_status.runtime_readiness(),
            self.access_status.support_authority(),
        );
        let working_resource = if drift == ProviderSessionManagementBindingDrift::WorkingResource {
            value(WorkingResourceRef::new, "fixture.resource.drifted")
        } else {
            self.working_resource.clone()
        };
        ProviderSessionManagementBinding::from_bound_session(
            value(
                SessionRef::new,
                if drift == ProviderSessionManagementBindingDrift::ProviderSessionReference {
                    "fixture.arbitrary.provider-id"
                } else {
                    "fixture.private.provider-session"
                },
            ),
            &driver,
            &instance,
            PreparedAccessEvidence::observed(
                status,
                value(
                    AccessEvidenceSourceId::new,
                    "fixture.private.access-observation",
                ),
            ),
            Some(working_resource),
            if drift == ProviderSessionManagementBindingDrift::Origin {
                ProviderSessionBindingOrigin::Loaded
            } else {
                ProviderSessionBindingOrigin::ExplicitlyImported
            },
        )
    }
}
