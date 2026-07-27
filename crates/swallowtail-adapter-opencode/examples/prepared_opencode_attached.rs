use swallowtail_adapter_opencode::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodePreparedCatalogue,
    OpenCodePreparedDelete, OpenCodePreparedIntegration, OpenCodePreparedSession,
    OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, ProviderId};
use swallowtail_runtime::{
    PreparationFailure, ProviderSessionManagementBinding, RequestId, WorkingResourceRef,
};

pub fn catalogue(
    prepared: &OpenCodePreparedIntegration,
    request_id: RequestId,
) -> Result<OpenCodePreparedCatalogue, PreparationFailure> {
    prepared.prepare_catalogue(OpenCodeCatalogueProfileInput::new(request_id))
}

pub fn read_only_session(
    prepared: &OpenCodePreparedIntegration,
    request_id: RequestId,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
    working_resource: WorkingResourceRef,
) -> Result<OpenCodePreparedSession, PreparationFailure> {
    prepared.prepare_session(OpenCodeSessionProfileInput::new(
        request_id,
        OpenCodeModelSelection::new(route_id, route_revision, provider_id, model_id),
        working_resource,
    ))
}

pub fn delete_inactive_session(
    prepared: &OpenCodePreparedIntegration,
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
) -> Result<OpenCodePreparedDelete, PreparationFailure> {
    prepared.prepare_delete_session(OpenCodeSessionManagementInput::new(request_id, binding))
}

fn main() {}
