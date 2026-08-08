use super::CursorPreparedDriver;
use swallowtail_core::{
    AccessProfile, AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, DriverDescriptor,
    EndpointAuthorization, EntitlementState, HarnessConfigurationPosture, InstanceOwnership,
    PreflightPlan, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionProviderStatePolicy, SupportAuthority,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedAccessEvidence};

pub(crate) fn route_instance_shape(
    driver: CursorPreparedDriver,
) -> (
    DriverDescriptor,
    &'static str,
    &'static str,
    CapabilityProfile,
) {
    match driver {
        CursorPreparedDriver::Catalogue => (
            crate::cursor_catalogue_descriptor(),
            "cursor-agent-models-v1",
            "cursor-prepared-catalogue",
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]),
        ),
        CursorPreparedDriver::Acp => (
            crate::cursor_acp_descriptor(),
            "acp-v1",
            "cursor-prepared-ambient-read-write",
            acp_capabilities(),
        ),
        CursorPreparedDriver::Headless => (
            crate::cursor_headless_descriptor(),
            "cursor-stream-json-v1",
            "cursor-prepared-ambient-explicit-access",
            headless_capabilities(ResourceAccess::Read),
        ),
    }
}

pub(crate) fn acp_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::ProviderSessionAttachmentRecovery,
            [
                CapabilityConstraint::ReplayMaximumItems(
                    crate::MAXIMUM_ATTACHMENT_RECOVERY_UPDATES as u32,
                ),
                CapabilityConstraint::ReplayMaximumBytes(
                    crate::MAXIMUM_ATTACHMENT_RECOVERY_BYTES as u64,
                ),
            ],
        ),
    ])
}

pub(crate) fn headless_capabilities(access: ResourceAccess) -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ])
}

pub(crate) fn instance_with_capabilities(
    base: &ConfiguredInstance,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(base, capabilities)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(crate) fn access_requirement(profile: &AccessProfile) -> AccessRequirement {
    AccessRequirement::new(profile.id().clone())
        .with_credential_states([CredentialState::NotRequired])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported])
}

pub(crate) fn build_plan(
    descriptor: &DriverDescriptor,
    instance: &ConfiguredInstance,
    access_profile: &AccessProfile,
    access_evidence: &PreparedAccessEvidence,
    available_services: impl IntoIterator<Item = swallowtail_core::HostServiceKind>,
    requirements: &swallowtail_core::OperationRequirements,
    route: Option<&swallowtail_core::ModelRoute>,
) -> Result<PreflightPlan, PreparationFailure> {
    if instance.ownership() != InstanceOwnership::HostOwnedEphemeral {
        return Err(super::failure(
            PreparationStage::Preflight,
            "swallowtail.cursor.preparation.ownership_rejected",
            "Cursor prepared operations require host-owned ephemeral execution",
        ));
    }
    swallowtail_runtime::build_plan(
        descriptor,
        instance,
        route,
        requirements,
        access_profile,
        access_evidence.status(),
        available_services,
    )
}

pub(crate) const ACP_PROVIDER_STATE: SessionProviderStatePolicy =
    SessionProviderStatePolicy::DurableProviderSessionPreserved;
