use crate::{PreparedAccessEvidence, WorkingResourceRef};
use std::error::Error;
use std::fmt;
use swallowtail_core::{
    AdapterIdentity, Capability, CapabilityManifest, ConfiguredInstance, ConfiguredInstanceId,
    DriverDescriptor, ExecutionHostId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    PreflightPlan, ProtocolFacadeId, ProviderSessionBindingOrigin,
    ProviderSessionInterfaceCompatibility, SafeDiagnostic, SessionRef, TransportFamilyId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvalidProviderSessionManagementBindingKind {
    DriverMismatch,
    AccessProfileMismatch,
    MissingInterfaceVersion,
    IncompatibleInterfaceVersion,
    MissingManagementCapability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidProviderSessionManagementBinding {
    kind: InvalidProviderSessionManagementBindingKind,
    diagnostic: SafeDiagnostic,
}

impl InvalidProviderSessionManagementBinding {
    fn new(kind: InvalidProviderSessionManagementBindingKind, message: &'static str) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.provider_session_management_binding_invalid",
                message,
            ),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> InvalidProviderSessionManagementBindingKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidProviderSessionManagementBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidProviderSessionManagementBinding {}

/// Opaque authority to manage one inactive persistent provider session.
///
/// A raw `SessionRef` is insufficient. Construction also requires the exact
/// observed driver, transport, configured instance, access evidence, interface
/// versions, and management capabilities for the selected route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionManagementBinding {
    provider_session_ref: SessionRef,
    driver_identity: AdapterIdentity,
    integration_family: IntegrationFamilyId,
    transport_family: TransportFamilyId,
    configured_instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    instance_target: InstanceTargetRef,
    protocol_facade_id: ProtocolFacadeId,
    access: PreparedAccessEvidence,
    interface_compatibility: Vec<ProviderSessionInterfaceCompatibility>,
    capabilities: CapabilityManifest,
    working_resource: Option<WorkingResourceRef>,
    origin: ProviderSessionBindingOrigin,
}

impl ProviderSessionManagementBinding {
    pub fn from_bound_session(
        provider_session_ref: SessionRef,
        driver: &DriverDescriptor,
        instance: &ConfiguredInstance,
        access: PreparedAccessEvidence,
        working_resource: Option<WorkingResourceRef>,
        origin: ProviderSessionBindingOrigin,
    ) -> Result<Self, InvalidProviderSessionManagementBinding> {
        if driver.identity().id() != instance.driver_id() {
            return Err(InvalidProviderSessionManagementBinding::new(
                InvalidProviderSessionManagementBindingKind::DriverMismatch,
                "Provider-session driver does not match its configured instance",
            ));
        }
        if access.status().profile_id() != instance.access_profile_id() {
            return Err(InvalidProviderSessionManagementBinding::new(
                InvalidProviderSessionManagementBindingKind::AccessProfileMismatch,
                "Provider-session access evidence does not match its configured instance",
            ));
        }

        let interface_compatibility: Vec<_> = instance
            .interface_versions()
            .cloned()
            .map(|binding| {
                let assessment = driver.assess_interface_version(&binding);
                ProviderSessionInterfaceCompatibility::new(binding, assessment)
            })
            .collect();
        if interface_compatibility.is_empty() {
            return Err(InvalidProviderSessionManagementBinding::new(
                InvalidProviderSessionManagementBindingKind::MissingInterfaceVersion,
                "Provider-session management requires an exact interface version",
            ));
        }
        if interface_compatibility
            .iter()
            .any(|evidence| !evidence.assessment().is_permitted())
        {
            return Err(InvalidProviderSessionManagementBinding::new(
                InvalidProviderSessionManagementBindingKind::IncompatibleInterfaceVersion,
                "Provider-session interface version is incompatible",
            ));
        }

        let capabilities = CapabilityManifest::new(
            instance
                .capabilities()
                .iter()
                .map(|(capability, _)| capability)
                .filter(|capability| is_session_lifecycle_capability(*capability)),
        );
        if ![
            Capability::ProviderSessionArchive,
            Capability::ProviderSessionRestore,
            Capability::ProviderSessionDelete,
        ]
        .into_iter()
        .any(|capability| capabilities.supports(capability))
        {
            return Err(InvalidProviderSessionManagementBinding::new(
                InvalidProviderSessionManagementBindingKind::MissingManagementCapability,
                "Provider-session route does not advertise a management action",
            ));
        }

        Ok(Self {
            provider_session_ref,
            driver_identity: driver.identity().clone(),
            integration_family: driver.integration_family().clone(),
            transport_family: driver.transport_family().clone(),
            configured_instance_id: instance.id().clone(),
            instance_revision: instance.revision().clone(),
            execution_host_id: instance.execution_host_id().clone(),
            instance_target: instance.target_reference().clone(),
            protocol_facade_id: instance.protocol_facade_id().clone(),
            access,
            interface_compatibility,
            capabilities,
            working_resource,
            origin,
        })
    }

    #[must_use]
    pub const fn provider_session_ref(&self) -> &SessionRef {
        &self.provider_session_ref
    }

    #[must_use]
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        &self.driver_identity
    }

    #[must_use]
    pub const fn integration_family(&self) -> &IntegrationFamilyId {
        &self.integration_family
    }

    #[must_use]
    pub const fn transport_family(&self) -> &TransportFamilyId {
        &self.transport_family
    }

    #[must_use]
    pub const fn configured_instance_id(&self) -> &ConfiguredInstanceId {
        &self.configured_instance_id
    }

    #[must_use]
    pub const fn instance_revision(&self) -> &InstanceRevision {
        &self.instance_revision
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn instance_target(&self) -> &InstanceTargetRef {
        &self.instance_target
    }

    #[must_use]
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        &self.protocol_facade_id
    }

    #[must_use]
    pub const fn access(&self) -> &PreparedAccessEvidence {
        &self.access
    }

    pub fn interface_compatibility(
        &self,
    ) -> impl ExactSizeIterator<Item = &ProviderSessionInterfaceCompatibility> {
        self.interface_compatibility.iter()
    }

    #[must_use]
    pub fn supports(&self, capability: Capability) -> bool {
        self.capabilities.supports(capability)
    }

    pub fn capabilities(&self) -> impl ExactSizeIterator<Item = Capability> + '_ {
        self.capabilities.iter()
    }

    #[must_use]
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource.as_ref()
    }

    #[must_use]
    pub const fn origin(&self) -> ProviderSessionBindingOrigin {
        self.origin
    }

    #[must_use]
    pub fn matches_preflight_plan(&self, plan: &PreflightPlan) -> bool {
        &self.driver_identity == plan.driver_identity()
            && &self.integration_family == plan.integration_family()
            && &self.transport_family == plan.transport_family()
            && &self.configured_instance_id == plan.instance_id()
            && &self.instance_revision == plan.instance_revision()
            && &self.execution_host_id == plan.execution_host_id()
            && &self.instance_target == plan.instance_target_ref()
            && &self.protocol_facade_id == plan.protocol_facade_id()
            && self.access.status() == plan.access_status()
            && self.interface_compatibility
                == plan
                    .interface_versions()
                    .cloned()
                    .map(|binding| {
                        let assessment = plan.assess_interface_version(&binding);
                        ProviderSessionInterfaceCompatibility::new(binding, assessment)
                    })
                    .collect::<Vec<_>>()
    }
}

const fn is_session_lifecycle_capability(capability: Capability) -> bool {
    matches!(
        capability,
        Capability::ProviderSessionArchive
            | Capability::ProviderSessionRestore
            | Capability::ProviderSessionDelete
            | Capability::ProviderNativeSessionClose
    )
}

include!("provider_session_management/tests.rs");
