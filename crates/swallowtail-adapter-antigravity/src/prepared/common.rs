use super::AntigravityPreparedDriver;
use swallowtail_core::{
    AccessProfile, AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, DriverDescriptor,
    EndpointAuthorization, EntitlementState, HarnessConfigurationPosture, InstanceOwnership,
    PreflightPlan, ResourceAccess, ResourceRepresentation, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedAccessEvidence};

pub(super) fn route_instance_shape(
    driver: AntigravityPreparedDriver,
) -> (
    DriverDescriptor,
    &'static str,
    &'static str,
    CapabilityProfile,
) {
    match driver {
        AntigravityPreparedDriver::Catalogue => (
            crate::antigravity_catalogue_descriptor(),
            "antigravity-models-v1",
            "antigravity-prepared-catalogue",
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]),
        ),
        AntigravityPreparedDriver::Headless => (
            crate::antigravity_headless_descriptor(),
            "antigravity-stream-json-v1",
            "antigravity-prepared-explicit-run",
            run_capabilities(ResourceAccess::Read, None, false),
        ),
        AntigravityPreparedDriver::Continuation => (
            crate::antigravity_headless_descriptor(),
            "antigravity-stream-json-v1",
            "antigravity-prepared-exact-continuation",
            continuation_capabilities(),
        ),
    }
}

pub(super) fn run_capabilities(
    access: ResourceAccess,
    effort: Option<&swallowtail_core::ReasoningMode>,
    schema: bool,
) -> CapabilityProfile {
    let mut capabilities = vec![
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
        working_resource(access),
    ];
    if let Some(effort) = effort {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(effort.clone())],
        ));
    }
    if schema {
        capabilities.push(CapabilityRequirement::new(
            Capability::StructuredOutput,
            [
                CapabilityConstraint::SchemaDialect("json-schema-2020-12".to_owned()),
                CapabilityConstraint::StructuredOutputEnforcement(
                    swallowtail_core::StructuredOutputEnforcement::ProviderNative,
                ),
            ],
        ));
    }
    CapabilityProfile::new(capabilities)
}

pub(super) fn continuation_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [CapabilityConstraint::MaximumTurns(24)],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        working_resource(ResourceAccess::Read),
    ])
}

fn working_resource(access: ResourceAccess) -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(access),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

pub(super) fn instance_with_capabilities(
    base: &ConfiguredInstance,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(base, capabilities)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(super) fn access_requirement(profile: &AccessProfile) -> AccessRequirement {
    AccessRequirement::new(profile.id().clone())
        .with_credential_states([CredentialState::NotRequired])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported])
}

pub(super) fn build_plan(
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
            "swallowtail.antigravity.preparation.ownership_rejected",
            "Antigravity prepared operations require host-owned ephemeral execution",
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
