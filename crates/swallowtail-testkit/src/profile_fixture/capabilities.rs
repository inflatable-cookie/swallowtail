use crate::SyntheticProfile;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ResourceAccess, ResourceRepresentation,
};

pub(super) fn profile(profile: SyntheticProfile) -> CapabilityProfile {
    CapabilityProfile::new(requirements(profile))
}

pub(super) fn requirements(profile: SyntheticProfile) -> Vec<CapabilityRequirement> {
    let interruption_scope = match profile {
        SyntheticProfile::LongLivedRpcHarness
        | SyntheticProfile::LongLivedAcpHarness
        | SyntheticProfile::PersistentAcpHarness
        | SyntheticProfile::AttachedNetworkHarness
        | SyntheticProfile::ConnectionScopedDirectSession
        | SyntheticProfile::LocallyContinuedDirectSession => CancellationScope::ActiveTurn,
        SyntheticProfile::RealtimeMediaDirectSession => CancellationScope::ActiveResponse,
        SyntheticProfile::OwnedSelfHosted => CancellationScope::OwnedServingInstance,
        _ => CancellationScope::StructuredRun,
    };
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(interruption_scope)],
        ),
    ];
    match profile {
        SyntheticProfile::LongLivedRpcHarness => {
            capabilities.push(CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(Capability::Resume, []));
        }
        SyntheticProfile::LongLivedAcpHarness => {
            capabilities.push(CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(
                Capability::WorkingResource,
                [
                    CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                    CapabilityConstraint::ResourceRepresentation(
                        ResourceRepresentation::Filesystem,
                    ),
                ],
            ));
        }
        SyntheticProfile::PersistentAcpHarness => {
            capabilities.extend(crate::profile_persistent_acp_shape::capabilities());
        }
        SyntheticProfile::ConnectionScopedDirectSession => {
            capabilities.push(CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(Capability::UsageReporting, []));
            capabilities.push(CapabilityRequirement::new(
                Capability::BilledCostReporting,
                [],
            ));
        }
        SyntheticProfile::LocallyContinuedDirectSession => {
            capabilities.push(CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(Capability::ToolCalls, []));
            capabilities.push(CapabilityRequirement::new(Capability::UsageReporting, []));
            capabilities.push(CapabilityRequirement::new(Capability::OutputTokenLimit, []));
            capabilities
                .extend(crate::direct_continuation_fixture::config().capability_requirements());
        }
        SyntheticProfile::RealtimeMediaDirectSession => {
            capabilities.push(
                crate::realtime_media_fixture::realtime_media_config().capability_requirement(),
            );
            capabilities.push(CapabilityRequirement::new(Capability::UsageReporting, []));
        }
        SyntheticProfile::AttachedNetworkHarness => {
            capabilities.push(CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            ));
        }
        SyntheticProfile::AttachedSelfHosted | SyntheticProfile::OwnedSelfHosted => {}
        SyntheticProfile::OneShotStructuredCli | SyntheticProfile::HostedDirectApi => {
            capabilities.push(CapabilityRequirement::new(Capability::StructuredRun, []));
        }
        SyntheticProfile::ProviderManagedRemoteHarness => {
            capabilities.extend(super::managed::capabilities());
        }
    }
    capabilities
}
