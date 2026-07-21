use super::{LlamaCppOwnedDriver, OWNED_DRIVER_ID, failure};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, ModelCatalogEntry, OperationShape, PreflightPlan,
    TransportFamilyId,
};
use swallowtail_runtime::{
    AttachServingRequest, AttachedServingHandle, BoxFuture, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, OwnedServingHandle, RunHandle, RuntimeFailure, ServingInstanceDriver,
    StartServingRequest, StructuredRunDriver, StructuredRunRequest,
};

#[must_use]
pub fn llama_cpp_owned_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(OWNED_DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("llama.cpp").expect("static family id is valid"),
        TransportFamilyId::new("process-http-sse").expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::ServingInstanceLifecycle,
        DriverRole::ModelCatalog,
        DriverRole::StructuredRun,
    ])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::ServingInstanceLifecycle,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Process,
            HostServiceKind::Network,
            HostServiceKind::ModelArtifact,
            HostServiceKind::ServingEndpoint,
        ],
    )
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
        ],
    )
}

impl ModelCatalogDriver for LlamaCppOwnedDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        self.facade.list_models(plan, request, services)
    }
}

impl StructuredRunDriver for LlamaCppOwnedDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        self.facade.start_run(plan, request, services)
    }
}

impl ServingInstanceDriver for LlamaCppOwnedDriver {
    fn attach(
        &self,
        _plan: PreflightPlan,
        _request: AttachServingRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn AttachedServingHandle>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.llama_cpp.owned_attach_rejected",
                "Owned llama.cpp lifecycle does not attach an external service",
            ))
        })
    }

    fn start(
        &self,
        plan: PreflightPlan,
        request: StartServingRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn OwnedServingHandle>, RuntimeFailure>> {
        Box::pin(self.start_owned(plan, request, services))
    }
}
