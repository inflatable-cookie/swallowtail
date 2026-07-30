use crate::failure::failure;
use crate::protocol::WireRequest;
use crate::transport::CurlTransport;
use serde_json::Value;
use std::collections::BTreeSet;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, ModelCatalogEntry, ModelId,
    ModelMetadata, OperationShape, PreflightPlan, ProviderId, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};

const DRIVER_ID: &str = "swallowtail.alibaba-model-studio.deployable-models";
const MAXIMUM_MODELS: usize = 4_096;
const MAXIMUM_PAGES_PER_SOURCE: u32 = 32;
const MAXIMUM_TEXT_BYTES: usize = 512;

#[derive(Clone, Default)]
pub struct AlibabaDeployableModelsDriver {
    transport: CurlTransport,
}

impl AlibabaDeployableModelsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[must_use]
pub fn alibaba_deployable_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("alibaba-model-studio").expect("static family id is valid"),
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
    .with_interface_compatibility(crate::alibaba_deployable_models_facade_claim())
}

include!("catalogue/driver.rs");
include!("catalogue/request.rs");
include!("catalogue/protocol.rs");
include!("catalogue/tests.rs");
