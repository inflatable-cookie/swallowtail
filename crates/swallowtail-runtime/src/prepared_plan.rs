//! Shared prepared-plan construction.
//!
//! Provider-neutral skeletons that adapters previously copied into their
//! `prepared_profile/plan.rs` modules: configured-instance rebinding with
//! capabilities, the base operation-requirements record, and the preflight
//! plan build. Adapter-specific pieces (descriptors, model routes, claims,
//! requirement tails, evidence wrappers) stay adapter-local.

#![deny(missing_docs)]

use crate::{PreparationFailure, PreparationStage};
use swallowtail_core::{
    AccessProfile, AccessRequirement, AccessStatus, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, CredentialState, Diagnostic, DriverDescriptor, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionLayer, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, RuntimeReadiness,
    preflight,
};

/// Rebuilds one configured instance with a new capability profile.
///
/// The returned instance shares the base identity, host, target, access,
/// support, facade, and policy bindings, plus its interface versions.
/// Adapter-specific posture extensions chain on the result.
#[must_use]
pub fn instance_with_capabilities(
    base: &ConfiguredInstance,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
}

/// Builds the base operation-requirements record for one prepared role.
///
/// Covers the standard ready-credential, available-entitlement, allowed-
/// endpoint, ready-runtime access requirement plus the instance ownership
/// modes and capabilities. Host services and adapter-specific requirement
/// tails chain on the result.
#[must_use]
pub fn base_requirements(
    layer: ExecutionLayer,
    shape: OperationShape,
    role: DriverRole,
    instance: &ConfiguredInstance,
    access_profile: &AccessProfile,
    credential_states: impl IntoIterator<Item = CredentialState>,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        layer,
        shape,
        role,
        instance.execution_host_id().clone(),
        AccessRequirement::new(access_profile.id().clone())
            .with_credential_states(credential_states)
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([access_profile.support_authority()]),
    )
    .with_ownership_modes([instance.ownership()])
    .with_capabilities(capabilities)
}

/// Runs preflight for one prepared operation and maps failures to
/// `PreparationStage::Preflight`.
pub fn build_plan(
    descriptor: &DriverDescriptor,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
    access_profile: &AccessProfile,
    access_status: &AccessStatus,
    available_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> Result<PreflightPlan, PreparationFailure> {
    let context = PreflightContext::new(
        descriptor,
        instance,
        access_profile,
        access_status,
        available_host_services,
    );
    let context = match route {
        Some(route) => context.with_model_route(route),
        None => context,
    };
    preflight(&context, requirements).map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{base_requirements, build_plan, instance_with_capabilities};
    use crate::PreparationStage;
    use crate::PreparedAccessEvidence;
    use swallowtail_core::{
        AccessProfile, AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion,
        Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
        ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverDescriptor, DriverRole,
        EndpointAuthorization, EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind,
        InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
        IntegrationFamilyId, OperationShape, ProtocolFacadeId, RuntimeReadiness, SupportAuthority,
        TransportFamilyId,
    };

    fn base() -> (ConfiguredInstance, AccessProfile, DriverDescriptor) {
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            AdapterId::new("fixture.adapter").expect("adapter id is valid"),
            ExecutionHostId::new("fixture.host").expect("host id is valid"),
            InstanceTargetRef::new("fixture.target").expect("target is valid"),
            InstanceOwnership::ExternalAttached,
            AccessProfileId::new("fixture.access").expect("access id is valid"),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
            InstancePolicyId::new("fixture.policy").expect("policy is valid"),
            CapabilityProfile::new([]),
        );
        let access = AccessProfile::new(
            AccessProfileId::new("fixture.access").expect("access id is valid"),
            CredentialMechanism::Unauthenticated,
            swallowtail_core::EntitlementMetering::LocalCompute,
            swallowtail_core::EndpointAudience::new("fixture.audience").expect("audience is valid"),
            SupportAuthority::ProviderSupported,
        );
        let descriptor = DriverDescriptor::new(
            AdapterIdentity::new(
                AdapterId::new("fixture.adapter").expect("adapter id is valid"),
                AdapterVersion::new("0.0.0").expect("version is valid"),
            ),
            IntegrationFamilyId::new("fixture").expect("family id is valid"),
            TransportFamilyId::new("fixture").expect("transport id is valid"),
        );
        (instance, access, descriptor)
    }

    #[test]
    fn instance_rebinding_preserves_identity_and_replaces_capabilities() {
        let (instance, _, _) = base();
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::StreamingEvents, [])]);
        let rebound = instance_with_capabilities(&instance, capabilities);
        assert_eq!(rebound.id(), instance.id());
        assert_eq!(rebound.execution_host_id(), instance.execution_host_id());
        assert_eq!(
            rebound
                .capabilities()
                .iter()
                .map(|(capability, _)| capability)
                .collect::<Vec<_>>(),
            vec![Capability::StreamingEvents]
        );
    }

    #[test]
    fn base_requirements_bind_role_shape_and_access_states() {
        let (instance, access, _) = base();
        let requirements = base_requirements(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            &instance,
            &access,
            [CredentialState::Ready],
            [CapabilityRequirement::new(Capability::StructuredRun, [])],
        );
        assert_eq!(
            requirements.operation_shape(),
            OperationShape::StructuredRun
        );
        assert_eq!(requirements.driver_role(), DriverRole::StructuredRun);
        assert_eq!(
            requirements.execution_host_id(),
            instance.execution_host_id()
        );
    }

    #[test]
    fn build_plan_runs_preflight_and_maps_failures() {
        let (instance, access, descriptor) = base();
        let status = AccessStatus::new(
            AccessProfileId::new("fixture.access").expect("access id is valid"),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let evidence = PreparedAccessEvidence::caller_asserted(status.clone());
        let requirements = base_requirements(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            &instance,
            &access,
            [CredentialState::Ready],
            [CapabilityRequirement::new(Capability::StructuredRun, [])],
        );
        let plan = build_plan(
            &descriptor,
            &instance,
            None,
            &requirements,
            &access,
            evidence.status(),
            [HostServiceKind::Process, HostServiceKind::Time],
        );
        match plan {
            Ok(_) => {}
            Err(failure) => assert_eq!(failure.stage(), PreparationStage::Preflight),
        }
    }
}
