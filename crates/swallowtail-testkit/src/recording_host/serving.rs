use super::{RecordedHostCall, RecordingService};
use swallowtail_core::{EndpointAudience, ExecutionHostId, ModelArtifactBinding};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, EndpointRef, MaterializedModelArtifactRef, ModelArtifactLease,
    ModelArtifactService, ObservedServingEndpoint, RuntimeFailure, ScopeId, ServingEndpointBinding,
    ServingEndpointLease, ServingEndpointService,
};

impl ModelArtifactService for RecordingService {
    fn acquire(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        binding: ModelArtifactBinding,
    ) -> BoxFuture<'static, Result<ModelArtifactLease, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::ModelArtifactAcquire)
            .map(|()| {
                ModelArtifactLease::read_only(
                    scope,
                    execution_host_id,
                    binding,
                    MaterializedModelArtifactRef::new("/private/recording/model.gguf")
                        .expect("recording artifact is valid"),
                )
            });
        Box::pin(async move { result })
    }

    fn release(&self, _lease: ModelArtifactLease) -> BoxFuture<'static, CleanupOutcome> {
        self.state.record(RecordedHostCall::ModelArtifactRelease);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

impl ServingEndpointService for RecordingService {
    fn publish(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        audience: EndpointAudience,
        _observed: ObservedServingEndpoint,
    ) -> BoxFuture<'static, Result<ServingEndpointLease, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::ServingEndpointPublish)
            .map(|()| {
                ServingEndpointLease::new(ServingEndpointBinding::new(
                    scope,
                    execution_host_id,
                    EndpointRef::new("recording.serving-endpoint")
                        .expect("recording endpoint reference is valid"),
                    audience,
                ))
            });
        Box::pin(async move { result })
    }

    fn release(&self, _lease: ServingEndpointLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self.cleanup(RecordedHostCall::ServingEndpointRelease);
        Box::pin(async move { outcome })
    }
}
