use swallowtail_core::{
    AccessProfileId, AdapterIdentity, ConfiguredInstanceId, CredentialMechanism, DriverRole,
    ExecutionHostId, ExecutionLayer, FilesystemBoundary, InstanceRevision, ModelId, ModelRouteId,
    ModelRouteRevision, OperationShape, PreflightPlan, ProtocolFacadeId, ProviderId,
    ResourceAccess, SupportAuthority,
};

use crate::PreparedOperationEvidence;

/// Exact model binding one projection row applies to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteModelBinding {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
    provider_id: Option<ProviderId>,
}

impl ConsumerRouteModelBinding {
    #[must_use]
    /// Returns the exact model-route identity.
    pub const fn route_id(&self) -> &ModelRouteId {
        &self.route_id
    }

    #[must_use]
    /// Returns the exact model-route revision.
    pub const fn route_revision(&self) -> &ModelRouteRevision {
        &self.route_revision
    }

    #[must_use]
    /// Returns the selected model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Returns the provider identity when the model source supplied one.
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }
}

/// Exact applicability shared by one projection snapshot and each of its rows.
///
/// Applicability is descriptive. It authorizes no operation and creates no
/// route, model, or default selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteApplicability {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    driver_identity: AdapterIdentity,
    protocol_facade_id: ProtocolFacadeId,
    execution_host_id: ExecutionHostId,
    driver_role: DriverRole,
    execution_layer: ExecutionLayer,
    operation_shape: OperationShape,
    model: Option<ConsumerRouteModelBinding>,
    access_profile_id: AccessProfileId,
    credential_mechanism: CredentialMechanism,
    support_authority: SupportAuthority,
    resource_access: Option<ResourceAccess>,
    filesystem_boundary: Option<FilesystemBoundary>,
}

impl ConsumerRouteApplicability {
    #[must_use]
    /// Derives exact applicability from one immutable preflight plan.
    pub fn from_plan(plan: &PreflightPlan) -> Self {
        let model = plan
            .model_route_id()
            .map(|route_id| ConsumerRouteModelBinding {
                route_id: route_id.clone(),
                route_revision: plan
                    .model_route_revision()
                    .expect("a model route id always has a route revision")
                    .clone(),
                model_id: plan
                    .model_id()
                    .expect("a model route id always has a model id")
                    .clone(),
                provider_id: plan.provider_id().cloned(),
            });
        let policy = plan.requirements().session_access_policy();
        Self {
            instance_id: plan.instance_id().clone(),
            instance_revision: plan.instance_revision().clone(),
            driver_identity: plan.driver_identity().clone(),
            protocol_facade_id: plan.protocol_facade_id().clone(),
            execution_host_id: plan.execution_host_id().clone(),
            driver_role: plan.requirements().driver_role(),
            execution_layer: plan.requirements().execution_layer(),
            operation_shape: plan.requirements().operation_shape(),
            model,
            access_profile_id: plan.access_profile_id().clone(),
            credential_mechanism: plan.credential_mechanism().clone(),
            support_authority: plan.access_status().support_authority(),
            resource_access: policy
                .and_then(swallowtail_core::SessionAccessPolicy::resource_access),
            filesystem_boundary: policy
                .and_then(swallowtail_core::SessionAccessPolicy::filesystem_boundary),
        }
    }

    #[must_use]
    /// Derives exact applicability from one prepared-operation record.
    pub fn from_prepared_operation(evidence: &PreparedOperationEvidence) -> Self {
        Self::from_plan(evidence.plan())
    }

    #[must_use]
    /// Returns the configured-instance identity.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the exact configured-instance revision.
    pub const fn instance_revision(&self) -> &InstanceRevision {
        &self.instance_revision
    }

    #[must_use]
    /// Returns the exact adapter driver identity.
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        &self.driver_identity
    }

    #[must_use]
    /// Returns the exact protocol facade.
    pub const fn protocol_facade_id(&self) -> &ProtocolFacadeId {
        &self.protocol_facade_id
    }

    #[must_use]
    /// Returns the authoritative execution host.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    /// Returns the prepared driver role.
    pub const fn driver_role(&self) -> DriverRole {
        self.driver_role
    }

    #[must_use]
    /// Returns the prepared execution layer.
    pub const fn execution_layer(&self) -> ExecutionLayer {
        self.execution_layer
    }

    #[must_use]
    /// Returns the prepared operation shape.
    pub const fn operation_shape(&self) -> OperationShape {
        self.operation_shape
    }

    #[must_use]
    /// Returns the exact model binding when the operation fixed one.
    pub const fn model(&self) -> Option<&ConsumerRouteModelBinding> {
        self.model.as_ref()
    }

    #[must_use]
    /// Returns the exact access profile the operation was prepared against.
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        &self.access_profile_id
    }

    #[must_use]
    /// Returns the exact credential mechanism of the access boundary.
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        &self.credential_mechanism
    }

    #[must_use]
    /// Returns the support authority carried by prepared access evidence.
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }

    #[must_use]
    /// Returns the admitted resource access, when the operation bound one.
    pub const fn resource_access(&self) -> Option<ResourceAccess> {
        self.resource_access
    }

    #[must_use]
    /// Returns the admitted filesystem boundary, when the operation bound one.
    pub const fn filesystem_boundary(&self) -> Option<FilesystemBoundary> {
        self.filesystem_boundary
    }
}
