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
    AdapterId, AdapterIdentity, AdapterVersion, CatalogObservation, CredentialMechanism,
    DriverDescriptor, DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId,
    ModelCatalogEntry, ModelCatalogObservations, ModelId, ModelMetadata, ModelModality,
    OperationShape, PreflightPlan, ProviderCatalogValue, ProviderId, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};
use url::Url;

const DRIVER_ID: &str = "swallowtail.xai.models";
const MAXIMUM_MODELS: usize = 2_048;
const MAXIMUM_BODY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_TEXT_BYTES: usize = 512;

#[derive(Clone, Default)]
/// Low-level read-only xAI language-model catalogue driver.
pub struct XaiModelsDriver;

impl XaiModelsDriver {
    #[must_use]
    /// Creates an xAI Models driver.
    pub fn new() -> Self {
        Self
    }
}

#[must_use]
/// Returns the exact descriptor for the xAI Models route.
pub fn xai_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("xai").expect("static family id is valid"),
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
    .with_interface_compatibility(crate::xai_models_facade_claim())
}

include!("catalogue/driver.rs");
include!("catalogue/request.rs");
include!("catalogue/protocol.rs");
include!("catalogue/tests.rs");
