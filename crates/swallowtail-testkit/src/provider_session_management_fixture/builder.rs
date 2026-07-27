use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, Capability, CapabilityProfile,
    CapabilityRequirement, DriverDescriptor, DriverRole, ExecutionLayer, IntegrationFamilyId,
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionScheme, InterfaceVersionSegment, OperationShape,
    ProviderSessionInitialStateRequirement, ProviderSessionManagementAction, TransportFamilyId,
};

use super::{QUALIFIED_VERSION, VERSION_AXIS};

pub(super) fn driver(adapter_id: AdapterId) -> DriverDescriptor {
    driver_with(
        adapter_id,
        value(IntegrationFamilyId::new, "fixture-session"),
        value(TransportFamilyId::new, "fixture-session-rpc"),
    )
}

pub(super) fn driver_with(
    adapter_id: AdapterId,
    integration_family: IntegrationFamilyId,
    transport_family: TransportFamilyId,
) -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(adapter_id, value(AdapterVersion::new, "fixture-driver-1")),
        integration_family,
        transport_family,
    )
    .with_roles([DriverRole::ProviderSessionManagement])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::ProviderSessionManagement])
    .with_required_host_services(
        DriverRole::ProviderSessionManagement,
        [
            swallowtail_core::HostServiceKind::Task,
            swallowtail_core::HostServiceKind::Time,
            swallowtail_core::HostServiceKind::Credential,
            swallowtail_core::HostServiceKind::WorkingResource,
        ],
    )
    .with_interface_compatibility(
        InterfaceCompatibilityClaim::new(
            value(
                InterfaceCompatibilityClaimId::new,
                "fixture-session-compatibility",
            ),
            value(InterfaceVersionAxis::new, VERSION_AXIS),
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::AllowUnverified,
            [InterfaceVersionSegment::new(
                value(InterfaceVersion::new, "1.0.0"),
                value(InterfaceVersion::new, QUALIFIED_VERSION),
                value(InterfaceBehaviorRevision::new, "fixture-session-v1"),
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .expect("static compatibility claim is valid"),
    )
}

pub(super) fn capabilities(case: super::ProviderSessionManagementFixtureCase) -> CapabilityProfile {
    let capabilities = if case == super::ProviderSessionManagementFixtureCase::Unsupported {
        vec![Capability::ProviderNativeSessionClose]
    } else {
        vec![
            Capability::ProviderSessionArchive,
            Capability::ProviderSessionRestore,
            Capability::ProviderSessionDelete,
            Capability::ProviderNativeSessionClose,
        ]
    };
    CapabilityProfile::new(
        capabilities
            .into_iter()
            .map(|capability| CapabilityRequirement::new(capability, [])),
    )
}

pub(super) const fn initial_state(
    action: ProviderSessionManagementAction,
) -> ProviderSessionInitialStateRequirement {
    match action {
        ProviderSessionManagementAction::Archive => {
            ProviderSessionInitialStateRequirement::Unarchived
        }
        ProviderSessionManagementAction::Restore => {
            ProviderSessionInitialStateRequirement::Archived
        }
        ProviderSessionManagementAction::Delete(_) => {
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived
        }
    }
}

pub(super) fn value<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static provider-session fixture text is valid")
}
