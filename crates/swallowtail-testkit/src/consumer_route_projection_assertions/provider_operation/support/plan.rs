use std::num::{NonZeroU32, NonZeroU64};

use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityRequirement, DriverRole,
    ExecutionLayer, HarnessIsolation, HostServiceKind, InstanceOwnership, OperationRequirements,
    OperationShape, PreflightContext, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
    SessionProviderStatePolicy, preflight,
};
use swallowtail_runtime::{ProviderSessionHistoryBounds, WorkingResourceRef};

use super::ProviderOperationFixture;

impl ProviderOperationFixture {
    pub(crate) fn plan(&self) -> swallowtail_core::PreflightPlan {
        let status = self.access_evidence.status();
        let services = services(self.shape);
        let mut requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            self.shape,
            role(self.shape),
            self.instance.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([status.credential()])
                .with_entitlement_states([status.entitlement()])
                .with_endpoint_authorizations([status.endpoint_authorization()])
                .with_runtime_readiness([status.runtime_readiness()])
                .with_support_authorities([status.support_authority()]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(services.iter().copied())
        .with_capabilities(capabilities(self.shape))
        .with_interface_versions(self.instance.interface_versions().cloned());
        if self.shape != OperationShape::ProviderSessionCatalogue {
            requirements = requirements.require_model_route();
        }
        if self.shape == OperationShape::ProviderSessionHistory {
            requirements = requirements
                .with_harness_isolation(HarnessIsolation::AmbientHost)
                .with_session_access_policy(SessionAccessPolicy::ambient_harness(
                    ResourceAccess::Read,
                ))
                .with_session_provider_state_policy(
                    SessionProviderStatePolicy::DurableProviderSessionPreserved,
                );
        } else if self.shape == OperationShape::InteractiveSession {
            requirements = requirements
                .with_session_access_policy(SessionAccessPolicy::read_only())
                .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
        }
        let context = PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            status,
            services,
        );
        let context = if self.shape == OperationShape::ProviderSessionCatalogue {
            context
        } else {
            context.with_model_route(&self.route)
        };
        preflight(&context, &requirements).expect("provider-operation preflight is valid")
    }
}

pub(crate) fn role(shape: OperationShape) -> DriverRole {
    match shape {
        OperationShape::InteractiveSession => DriverRole::InteractiveSession,
        OperationShape::StructuredRun => DriverRole::StructuredRun,
        OperationShape::ProviderSessionCatalogue => DriverRole::ProviderSessionCatalogue,
        OperationShape::ProviderSessionHistory => DriverRole::ProviderSessionHistory,
        _ => panic!("fixture only supports the four asserted operation shapes"),
    }
}

pub(crate) fn capabilities(shape: OperationShape) -> Vec<CapabilityRequirement> {
    match shape {
        OperationShape::InteractiveSession => vec![
            CapabilityRequirement::new(Capability::InteractiveSession, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
        ],
        OperationShape::StructuredRun => {
            vec![CapabilityRequirement::new(Capability::StructuredRun, [])]
        }
        OperationShape::ProviderSessionCatalogue => vec![
            CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
            working_resource_requirement(),
        ],
        OperationShape::ProviderSessionHistory => vec![
            CapabilityRequirement::new(
                Capability::ProviderSessionHistory,
                [
                    CapabilityConstraint::ReplayMaximumItems(2),
                    CapabilityConstraint::ReplayMaximumBytes(64),
                ],
            ),
            CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
            working_resource_requirement(),
        ],
        _ => panic!("fixture only supports the four asserted operation shapes"),
    }
}

fn services(shape: OperationShape) -> Vec<HostServiceKind> {
    match shape {
        OperationShape::ProviderSessionCatalogue => {
            vec![HostServiceKind::Task, HostServiceKind::WorkingResource]
        }
        OperationShape::ProviderSessionHistory => vec![HostServiceKind::WorkingResource],
        _ => Vec::new(),
    }
}

fn working_resource_requirement() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

pub(crate) fn history_bounds() -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        nonzero_u32(2),
        nonzero_u64(64),
        nonzero_u32(64),
        nonzero_u32(8),
    )
}

pub(crate) fn resource() -> WorkingResourceRef {
    WorkingResourceRef::new("fixture-provider-operation-resource").expect("resource is valid")
}

pub(crate) fn nonzero_u32(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).expect("fixture bound is nonzero")
}

fn nonzero_u64(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("fixture bound is nonzero")
}
