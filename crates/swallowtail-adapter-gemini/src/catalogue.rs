use crate::failure::failure;
use curl::easy::{Easy, List, WriteError};
use futures_channel::oneshot;
use serde_json::Value;
use std::collections::BTreeSet;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use std::time::Duration;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, ModelCatalogEntry,
    ModelCatalogObservations, ModelId, ModelMetadata, ModelTokenLimits, OperationShape,
    PreflightPlan, ProviderId, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};
use url::Url;

const DRIVER_ID: &str = "swallowtail.gemini.models";
const MAXIMUM_MODELS: usize = 4_096;
const MAXIMUM_PAGES: usize = 32;
const MAXIMUM_BODY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_TEXT_BYTES: usize = 512;

#[derive(Clone, Default)]
pub struct GeminiModelsDriver;

impl GeminiModelsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[must_use]
pub fn gemini_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("gemini").expect("static family id is valid"),
        TransportFamilyId::new("https-json-models").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
    .with_interface_compatibility(crate::gemini_models_facade_claim())
}

impl ModelCatalogDriver for GeminiModelsDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!("gemini-models:{}", request.request_id().as_str()))
                .map_err(|_| protocol_failure())?;
            let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
            let audience = plan.endpoint_audience().clone();
            let grant = services
                .network()
                .expect("validated network service")
                .authorize(scope.clone(), endpoint_ref.clone(), audience.clone())
                .await?;
            if grant.scope() != &scope
                || grant.endpoint() != &endpoint_ref
                || grant.audience() != &audience
            {
                return Err(failure(
                    "swallowtail.gemini.models.network_grant_mismatch",
                    "Gemini Models network grant does not match its immutable plan",
                ));
            }
            let credential_service = services
                .credential()
                .cloned()
                .expect("validated credential service");
            let reference = plan
                .credential_reference()
                .expect("validated credential")
                .clone();
            let mut lease = Some(
                credential_service
                    .acquire(scope.clone(), reference.clone(), audience.clone())
                    .await?,
            );
            let secret = match lease.as_ref().expect("credential was acquired") {
                CredentialLease::Secret(secret)
                    if secret.scope() == &scope
                        && secret.reference() == &reference
                        && secret.audience() == &audience =>
                {
                    secret.expose_secret().to_vec()
                }
                CredentialLease::Secret(_) | CredentialLease::Delegated(_) => {
                    let cleanup = credential_service
                        .release(lease.take().expect("credential was acquired"))
                        .await;
                    return if matches!(
                        cleanup,
                        CleanupOutcome::Clean | CleanupOutcome::NotApplicable
                    ) {
                        Err(failure(
                            "swallowtail.gemini.models.credential_lease_rejected",
                            "Gemini Models requires a matching API-key secret lease",
                        ))
                    } else {
                        Err(cleanup_failure())
                    };
                }
            };
            let result = list_pages(
                scope,
                grant.authorized().as_driver_value(),
                secret,
                &request,
                &services,
            )
            .await;
            let cleanup = credential_service
                .release(lease.take().expect("credential was acquired"))
                .await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Err(error),
                _ => Err(cleanup_failure()),
            }
        })
    }
}

include!("catalogue/pages.rs");
include!("catalogue/transport.rs");
include!("catalogue/validation.rs");
include!("catalogue/tests.rs");
