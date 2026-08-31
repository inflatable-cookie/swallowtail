use swallowtail_core::{
    AccessProfileId, AdapterIdentity, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverRole, EndpointAuthorization, EntitlementState, ExecutionHostId, ExecutionLayer,
    FilesystemBoundary, InstancePolicyId, InstanceRevision, OperationShape, PreflightPlan,
    ProtocolFacadeId, ResourceAccess, RuntimeReadiness, SupportAuthority,
};

use super::model_binding::ConsumerRouteModelBinding;
use crate::PreparedOperationEvidence;

/// Exact applicability shared by one projection snapshot and each of its rows.
///
/// Applicability is descriptive. It authorizes no operation and creates no
/// route, model, or default selection. The five access dimensions stay
/// independently observable; no aggregate availability may replace them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteApplicability {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    instance_policy_id: InstancePolicyId,
    driver_identity: AdapterIdentity,
    protocol_facade_id: ProtocolFacadeId,
    execution_host_id: ExecutionHostId,
    driver_role: DriverRole,
    execution_layer: ExecutionLayer,
    operation_shape: OperationShape,
    model: Option<ConsumerRouteModelBinding>,
    access_profile_id: AccessProfileId,
    credential_mechanism: CredentialMechanism,
    credential_state: CredentialState,
    entitlement_state: EntitlementState,
    endpoint_authorization: EndpointAuthorization,
    runtime_readiness: RuntimeReadiness,
    support_authority: SupportAuthority,
    resource_access: Option<ResourceAccess>,
    filesystem_boundary: Option<FilesystemBoundary>,
}

impl ConsumerRouteApplicability {
    #[must_use]
    /// Derives exact applicability from one immutable preflight plan.
    pub fn from_plan(plan: &PreflightPlan) -> Self {
        let policy = plan.requirements().session_access_policy();
        let status = plan.access_status();
        Self {
            instance_id: plan.instance_id().clone(),
            instance_revision: plan.instance_revision().clone(),
            instance_policy_id: plan.instance_policy_id().clone(),
            driver_identity: plan.driver_identity().clone(),
            protocol_facade_id: plan.protocol_facade_id().clone(),
            execution_host_id: plan.execution_host_id().clone(),
            driver_role: plan.requirements().driver_role(),
            execution_layer: plan.requirements().execution_layer(),
            operation_shape: plan.requirements().operation_shape(),
            model: ConsumerRouteModelBinding::from_plan(plan),
            access_profile_id: plan.access_profile_id().clone(),
            credential_mechanism: plan.credential_mechanism().clone(),
            credential_state: status.credential(),
            entitlement_state: status.entitlement(),
            endpoint_authorization: status.endpoint_authorization(),
            runtime_readiness: status.runtime_readiness(),
            support_authority: status.support_authority(),
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
    /// Returns the exact configured instance policy.
    pub const fn instance_policy_id(&self) -> &InstancePolicyId {
        &self.instance_policy_id
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
    /// Returns observed credential readiness as its own access dimension.
    pub const fn credential_state(&self) -> CredentialState {
        self.credential_state
    }

    #[must_use]
    /// Returns observed entitlement state as its own access dimension.
    pub const fn entitlement_state(&self) -> EntitlementState {
        self.entitlement_state
    }

    #[must_use]
    /// Returns observed endpoint authorization as its own access dimension.
    pub const fn endpoint_authorization(&self) -> EndpointAuthorization {
        self.endpoint_authorization
    }

    #[must_use]
    /// Returns observed provider or runtime readiness as its own dimension.
    pub const fn runtime_readiness(&self) -> RuntimeReadiness {
        self.runtime_readiness
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
