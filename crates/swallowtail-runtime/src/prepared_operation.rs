use crate::{PreparationFailure, PreparationStage, PreparedAccessEvidence};
use swallowtail_core::{
    AdapterIdentity, ConfiguredInstanceId, Diagnostic, DriverRole, ExecutionHostId, ExecutionLayer,
    InstanceRevision, InstanceTargetRef, InterfaceCompatibilityAssessment, InterfaceVersionBinding,
    ObservableActivityProfile, OperationShape, PreflightPlan, ProtocolFacadeId, SafeDiagnostic,
    TransportFamilyId,
};

mod activity_profile;

use activity_profile::prepare_activity_profile;

/// Safe identity evidence shared by every adapter-local prepared operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedOperationBinding {
    driver_identity: AdapterIdentity,
    transport_family: TransportFamilyId,
    driver_role: DriverRole,
    execution_layer: ExecutionLayer,
    operation_shape: OperationShape,
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    instance_target: InstanceTargetRef,
    protocol_facade_id: ProtocolFacadeId,
}

impl PreparedOperationBinding {
    fn from_plan(plan: &PreflightPlan) -> Self {
        Self {
            driver_identity: plan.driver_identity().clone(),
            transport_family: plan.transport_family().clone(),
            driver_role: plan.requirements().driver_role(),
            execution_layer: plan.requirements().execution_layer(),
            operation_shape: plan.requirements().operation_shape(),
            instance_id: plan.instance_id().clone(),
            instance_revision: plan.instance_revision().clone(),
            execution_host_id: plan.execution_host_id().clone(),
            instance_target: plan.instance_target_ref().clone(),
            protocol_facade_id: plan.protocol_facade_id().clone(),
        }
    }

    #[must_use]
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        &self.driver_identity
    }

    #[must_use]
    pub const fn transport_family(&self) -> &TransportFamilyId {
        &self.transport_family
    }

    #[must_use]
    pub const fn driver_role(&self) -> DriverRole {
        self.driver_role
    }

    #[must_use]
    pub const fn execution_layer(&self) -> ExecutionLayer {
        self.execution_layer
    }

    #[must_use]
    pub const fn operation_shape(&self) -> OperationShape {
        self.operation_shape
    }

    #[must_use]
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
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
}

/// One exact interface binding and its visible compatibility assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedInterfaceCompatibility {
    binding: InterfaceVersionBinding,
    assessment: InterfaceCompatibilityAssessment,
}

impl PreparedInterfaceCompatibility {
    fn from_plan(plan: &PreflightPlan, binding: InterfaceVersionBinding) -> Self {
        let assessment = plan.assess_interface_version(&binding);
        Self {
            binding,
            assessment,
        }
    }

    #[must_use]
    pub const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    #[must_use]
    pub const fn assessment(&self) -> &InterfaceCompatibilityAssessment {
        &self.assessment
    }
}

/// Provider-neutral evidence retained by an adapter-local prepared operation.
///
/// This record owns the immutable expanded plan. It does not own an operation
/// request, driver, provider handle, credential, or routing decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedOperationEvidence {
    binding: PreparedOperationBinding,
    plan: PreflightPlan,
    access: PreparedAccessEvidence,
    interface_compatibility: Vec<PreparedInterfaceCompatibility>,
    observable_activity: ObservableActivityProfile,
}

impl PreparedOperationEvidence {
    pub fn from_plan(
        plan: PreflightPlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        Self::prepare(plan, access, None)
    }

    pub fn from_plan_with_activity_profile(
        plan: PreflightPlan,
        access: PreparedAccessEvidence,
        observable_activity: ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Self::prepare(plan, access, Some(observable_activity))
    }

    fn prepare(
        plan: PreflightPlan,
        access: PreparedAccessEvidence,
        supplied_activity: Option<ObservableActivityProfile>,
    ) -> Result<Self, PreparationFailure> {
        if access.status() != plan.access_status() {
            return Err(PreparationFailure::new(
                PreparationStage::AccessEvidence,
                Diagnostic::new(SafeDiagnostic::new(
                    "swallowtail.prepared_operation.access_mismatch",
                    "Prepared operation access evidence does not match its immutable plan",
                )),
            ));
        }
        let binding = PreparedOperationBinding::from_plan(&plan);
        let interface_compatibility = plan
            .interface_versions()
            .cloned()
            .map(|binding| PreparedInterfaceCompatibility::from_plan(&plan, binding))
            .collect::<Vec<_>>();
        let observable_activity =
            prepare_activity_profile(&plan, &binding, &interface_compatibility, supplied_activity)?;
        Ok(Self {
            binding,
            plan,
            access,
            interface_compatibility,
            observable_activity,
        })
    }

    #[must_use]
    pub const fn binding(&self) -> &PreparedOperationBinding {
        &self.binding
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        &self.plan
    }

    #[must_use]
    pub const fn access(&self) -> &PreparedAccessEvidence {
        &self.access
    }

    pub fn interface_compatibility(
        &self,
    ) -> impl ExactSizeIterator<Item = &PreparedInterfaceCompatibility> {
        self.interface_compatibility.iter()
    }

    #[must_use]
    pub const fn observable_activity(&self) -> &ObservableActivityProfile {
        &self.observable_activity
    }

    #[must_use]
    pub fn matches_plan(&self, plan: &PreflightPlan) -> bool {
        self.plan == *plan
    }

    #[must_use]
    pub fn into_plan(self) -> PreflightPlan {
        self.plan
    }
}

#[cfg(test)]
#[path = "prepared_operation/tests.rs"]
mod tests;
