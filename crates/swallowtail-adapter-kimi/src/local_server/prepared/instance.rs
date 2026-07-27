use super::input::KimiLocalServerAttachedInput;
use super::operation::lifecycle_capabilities;
use super::{KimiLocalServerObservation, KimiLocalServerPreparedIntegration};
use std::collections::BTreeSet;
use swallowtail_core::{
    ConfiguredInstance, HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceTargetRef,
    ProtocolFacadeId,
};

pub(super) fn build_prepared(
    input: KimiLocalServerAttachedInput,
    ownership: InstanceOwnership,
    executable_target: Option<InstanceTargetRef>,
    server: KimiLocalServerObservation,
    available_host_services: BTreeSet<HostServiceKind>,
) -> KimiLocalServerPreparedIntegration {
    let instance = ConfiguredInstance::new(
        input.instance_id,
        input.instance_revision,
        crate::kimi_local_server_descriptor()
            .identity()
            .id()
            .clone(),
        input.execution_host_id,
        input.endpoint_target,
        ownership,
        input.access_profile.id().clone(),
        input.access_profile.support_authority(),
        ProtocolFacadeId::new("kimi-local-server-rest-ws-v2")
            .expect("static protocol facade is valid"),
        InstancePolicyId::new("authenticated-loopback-provider-session-management")
            .expect("static policy id is valid"),
        lifecycle_capabilities(),
    )
    .with_interface_versions([server.binding().clone()]);
    KimiLocalServerPreparedIntegration {
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        server,
        state_root: input.state_root,
        executable_target,
        available_host_services,
    }
}
