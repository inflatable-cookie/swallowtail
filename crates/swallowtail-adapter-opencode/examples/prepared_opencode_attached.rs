use swallowtail_adapter_opencode::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodePreparedCatalogue,
    OpenCodePreparedDelete, OpenCodePreparedIntegration, OpenCodePreparedSession,
    OpenCodePreparedSessionCatalogue, OpenCodePreparedSessionImport, OpenCodeSessionCatalogueInput,
    OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
use swallowtail_core::{
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ProviderSessionCatalogueBounds,
};
use swallowtail_runtime::{
    HostServices, LoadedSession, PreparationFailure, ProviderSessionCandidate,
    ProviderSessionCatalogueId, ProviderSessionCatalogueOutcome, ProviderSessionImportOutcome,
    ProviderSessionManagementBinding, RequestId, RuntimeFailure, WorkingResourceRef,
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

pub fn session_catalogue(
    prepared: &OpenCodePreparedIntegration,
    request_id: RequestId,
    catalogue_id: ProviderSessionCatalogueId,
    working_resource: WorkingResourceRef,
    bounds: ProviderSessionCatalogueBounds,
) -> Result<OpenCodePreparedSessionCatalogue, PreparationFailure> {
    prepared.prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
        request_id,
        catalogue_id,
        working_resource,
        bounds,
    ))
}

pub async fn browse_sessions(
    catalogue: &OpenCodePreparedSessionCatalogue,
    services: HostServices,
) -> Result<ProviderSessionCatalogueOutcome, swallowtail_runtime::ProviderSessionOperationFailure> {
    catalogue.list_sessions(services).await
}

#[allow(clippy::too_many_arguments)]
pub fn import_selected_session(
    prepared: &OpenCodePreparedIntegration,
    catalogue: &OpenCodePreparedSessionCatalogue,
    candidate: ProviderSessionCandidate,
    request_id: RequestId,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
    working_resource: WorkingResourceRef,
) -> Result<OpenCodePreparedSessionImport, PreparationFailure> {
    prepared.prepare_session_import(
        catalogue,
        candidate,
        OpenCodeSessionProfileInput::new(
            request_id,
            OpenCodeModelSelection::new(route_id, route_revision, provider_id, model_id),
            working_resource,
        ),
    )
}

pub async fn revalidate_import(
    prepared: &OpenCodePreparedSessionImport,
    services: HostServices,
) -> Result<ProviderSessionImportOutcome, swallowtail_runtime::ProviderSessionOperationFailure> {
    prepared.import_session(services).await
}

pub async fn load_imported_session(
    session: &OpenCodePreparedSession,
    imported: &ProviderSessionImportOutcome,
    request_id: RequestId,
    services: HostServices,
) -> Result<LoadedSession, RuntimeFailure> {
    session
        .load_session(request_id, imported.binding().clone(), services)
        .map_err(|failure| RuntimeFailure::new(failure.diagnostic().safe().clone()))?
        .await
}

pub async fn resume_imported_session(
    session: &OpenCodePreparedSession,
    imported: &ProviderSessionImportOutcome,
    request_id: RequestId,
    services: HostServices,
) -> Result<Box<dyn swallowtail_runtime::InteractiveSessionHandle>, RuntimeFailure> {
    session
        .resume_session(request_id, imported.binding().clone(), services)
        .map_err(|failure| RuntimeFailure::new(failure.diagnostic().safe().clone()))?
        .await
}

fn main() {}
