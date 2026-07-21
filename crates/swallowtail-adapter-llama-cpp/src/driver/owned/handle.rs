use super::OwnedState;
use swallowtail_core::{ExecutionHostId, InstanceOwnership};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, OwnedServingHandle, ServingEndpointBinding, ServingInstanceId,
};

pub(super) struct LlamaCppOwnedHandle {
    serving_instance_id: ServingInstanceId,
    execution_host_id: ExecutionHostId,
    endpoint: ServingEndpointBinding,
    state: OwnedState,
}

impl LlamaCppOwnedHandle {
    pub fn new(
        serving_instance_id: ServingInstanceId,
        execution_host_id: ExecutionHostId,
        endpoint: ServingEndpointBinding,
        state: OwnedState,
    ) -> Self {
        Self {
            serving_instance_id,
            execution_host_id,
            endpoint,
            state,
        }
    }
}

impl OwnedServingHandle for LlamaCppOwnedHandle {
    fn serving_instance_id(&self) -> &ServingInstanceId {
        &self.serving_instance_id
    }

    fn ownership(&self) -> InstanceOwnership {
        InstanceOwnership::HostOwnedEphemeral
    }

    fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    fn endpoint_binding(&self) -> &ServingEndpointBinding {
        &self.endpoint
    }

    fn stop(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move { self.state.cleanup().await })
    }
}
