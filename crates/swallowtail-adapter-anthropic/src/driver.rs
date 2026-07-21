use crate::failure::{failure, unsupported};
use crate::protocol::{PROVIDER_ID, Request, parse_model_page};
use crate::transport::CurlTransport;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, Capability, CredentialMechanism, DriverDescriptor,
    DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId, ModelCatalogEntry,
    OperationShape, PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};

const DRIVER_ID: &str = "swallowtail.anthropic.direct";
const MAX_CATALOGUE_PAGES: usize = 8;

#[derive(Clone, Default)]
pub struct AnthropicDirectDriver {
    transport: CurlTransport,
}

impl AnthropicDirectDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.anthropic.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
        {
            return Err(failure(
                "swallowtail.anthropic.credential_binding_rejected",
                "Anthropic direct inference requires a bound public API-key credential",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn anthropic_direct_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("anthropic").expect("static family id is valid"),
        TransportFamilyId::new("http-sse").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog, DriverRole::StructuredRun])
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
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
}

impl ModelCatalogDriver for AnthropicDirectDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = operation_scope("catalog", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let endpoint = access.endpoint.clone();
            let credential = access.secret()?.to_vec();
            let result = async {
                let mut models = Vec::new();
                let mut cursor = None;
                for _ in 0..MAX_CATALOGUE_PAGES {
                    let response = complete_before_deadline(
                        self.transport.request(
                            scope.clone(),
                            endpoint.clone(),
                            credential.clone(),
                            Request::models(cursor.as_deref()),
                            &services,
                            Arc::clone(&cancelled),
                        ),
                        request.deadline(),
                        &services,
                        Arc::clone(&cancelled),
                    )
                    .await?;
                    let (page, next) = parse_model_page(&response)?;
                    models.extend(page);
                    cursor = next;
                    if cursor.is_none() {
                        return Ok(models);
                    }
                }
                Err(failure(
                    "swallowtail.anthropic.catalogue_page_limit",
                    "Anthropic model catalogue exceeded its bounded page limit",
                ))
            }
            .await;
            let cleanup = access.release(&services).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.anthropic.catalogue_cleanup_failed",
                    "Anthropic catalogue credential cleanup failed",
                )),
            }
        })
    }
}

struct AccessLeases {
    endpoint: String,
    credential: Option<CredentialLease>,
}

impl AccessLeases {
    async fn acquire(
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let network = services.network().ok_or_else(|| missing("network"))?;
        let credentials = services.credential().ok_or_else(|| missing("credential"))?;
        let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = network
            .authorize(
                scope.clone(),
                endpoint_ref.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint_ref
            || grant.audience() != plan.endpoint_audience()
        {
            return Err(failure(
                "swallowtail.anthropic.network_grant_mismatch",
                "Anthropic network grant did not match preflight",
            ));
        }
        let reference = plan.credential_reference().expect("validated").clone();
        let credential = credentials
            .acquire(
                scope.clone(),
                reference.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if credential.scope() != &scope
            || credential.reference() != &reference
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Secret(_))
        {
            let _ = credentials.release(credential).await;
            return Err(failure(
                "swallowtail.anthropic.credential_lease_rejected",
                "Anthropic credential lease did not match preflight",
            ));
        }
        Ok(Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            credential: Some(credential),
        })
    }

    fn secret(&self) -> Result<&[u8], RuntimeFailure> {
        match self.credential.as_ref() {
            Some(CredentialLease::Secret(secret)) => Ok(secret.expose_secret()),
            _ => Err(failure(
                "swallowtail.anthropic.credential_unavailable",
                "Anthropic credential was unavailable",
            )),
        }
    }

    async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.anthropic.credential_release_failed",
                    "Anthropic credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        }
    }
}

fn require_services(services: &HostServices, run: bool) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
        || (run && services.task().is_none())
    {
        Err(missing("required host"))
    } else {
        Ok(())
    }
}

fn missing(service: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.host_service_missing",
        format!("Anthropic direct inference requires the {service} service"),
    )
}

fn operation_scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("anthropic-direct:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.anthropic.scope_invalid",
            "Anthropic operation scope was invalid",
        )
    })
}

include!("driver/lifecycle.rs");
include!("driver/run.rs");
include!("driver/handle.rs");
