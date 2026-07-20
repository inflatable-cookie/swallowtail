use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId, InstanceTargetRef};
use swallowtail_runtime::WorkingResourceRef;

/// Provider-neutral identities for proving host placement without raw paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionTopologyFixture {
    execution_host_id: ExecutionHostId,
    configured_instance_id: ConfiguredInstanceId,
    instance_target: InstanceTargetRef,
    working_resource: WorkingResourceRef,
}

impl ExecutionTopologyFixture {
    #[must_use]
    pub fn local() -> Self {
        Self::new(
            "fixture.host.local",
            "fixture.instance.local",
            "fixture.target.local",
            "fixture.resource.local",
        )
    }

    #[must_use]
    pub fn remote_authoritative() -> Self {
        Self::new(
            "fixture.host.remote-authoritative",
            "fixture.instance.remote-authoritative",
            "fixture.target.remote-authoritative",
            "fixture.resource.remote-authoritative",
        )
    }

    fn new(host: &str, instance: &str, target: &str, resource: &str) -> Self {
        Self {
            execution_host_id: ExecutionHostId::new(host).expect("fixture host id is valid"),
            configured_instance_id: ConfiguredInstanceId::new(instance)
                .expect("fixture instance id is valid"),
            instance_target: InstanceTargetRef::new(target)
                .expect("fixture target reference is valid"),
            working_resource: WorkingResourceRef::new(resource)
                .expect("fixture working-resource reference is valid"),
        }
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn configured_instance_id(&self) -> &ConfiguredInstanceId {
        &self.configured_instance_id
    }

    #[must_use]
    pub const fn instance_target(&self) -> &InstanceTargetRef {
        &self.instance_target
    }

    #[must_use]
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionTopologyFixture;

    #[test]
    fn topologies_use_distinct_opaque_authority_references() {
        let local = ExecutionTopologyFixture::local();
        let remote = ExecutionTopologyFixture::remote_authoritative();

        assert_ne!(local.execution_host_id(), remote.execution_host_id());
        assert_ne!(
            local.configured_instance_id(),
            remote.configured_instance_id()
        );
        assert_ne!(local.instance_target(), remote.instance_target());
        assert_ne!(local.working_resource(), remote.working_resource());
        assert!(!format!("{local:?}").contains(local.instance_target().as_host_value()));
    }
}
