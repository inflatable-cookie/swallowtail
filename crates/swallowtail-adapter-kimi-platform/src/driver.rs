use crate::failure::{failure, unsupported};
use crate::protocol::{AUDIENCE, Request, parse_models};
use crate::transport::CurlTransport;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, InstanceOwnership, IntegrationFamilyId, ModelCatalogEntry,
    OperationShape, PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};

const DRIVER_ID: &str = "swallowtail.kimi-platform.direct-chat";

#[derive(Clone, Default)]
pub struct KimiPlatformDirectDriver {
    transport: CurlTransport,
}

impl KimiPlatformDirectDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID
            || plan.requirements().execution_layer() != ExecutionLayer::DirectModelInference
            || plan.ownership() != InstanceOwnership::ExternalAttached
        {
            return Err(failure(
                "swallowtail.kimi_platform.plan_binding_rejected",
                "Preflight plan is not bound to the Kimi Platform direct driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
            || plan.endpoint_audience().as_str() != AUDIENCE
        {
            return Err(failure(
                "swallowtail.kimi_platform.access_binding_rejected",
                "Kimi Platform requires the public-platform API-key access profile",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn kimi_platform_direct_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("kimi-platform").expect("static family id is valid"),
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

impl ModelCatalogDriver for KimiPlatformDirectDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            if plan.requirements().driver_role() != DriverRole::ModelCatalog {
                return Err(failure(
                    "swallowtail.kimi_platform.role_mismatch",
                    "Kimi Platform catalogue requires a catalogue preflight plan",
                ));
            }
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = operation_scope("catalog", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let endpoint = access.endpoint.clone();
            let credential = access.secret()?.to_vec();
            let result = complete_before_deadline(
                self.transport.request(
                    scope,
                    endpoint,
                    credential,
                    Request::models(),
                    &services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                &services,
                cancelled,
            )
            .await
            .and_then(|response| parse_models(&response));
            let cleanup = access.release(&services).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.kimi_platform.catalogue_cleanup_failed",
                    "Kimi Platform catalogue credential cleanup failed",
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
                "swallowtail.kimi_platform.network_grant_mismatch",
                "Kimi Platform network grant did not match preflight",
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
                "swallowtail.kimi_platform.credential_lease_rejected",
                "Kimi Platform credential lease did not match preflight",
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
                "swallowtail.kimi_platform.credential_unavailable",
                "Kimi Platform credential was unavailable",
            )),
        }
    }

    async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.kimi_platform.credential_release_failed",
                    "Kimi Platform credential service disappeared during cleanup",
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
        "swallowtail.kimi_platform.host_service_missing",
        format!("Kimi Platform direct inference requires the {service} service"),
    )
}

fn operation_scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("kimi-platform-direct:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.kimi_platform.scope_invalid",
            "Kimi Platform operation scope was invalid",
        )
    })
}

include!("driver/lifecycle.rs");
include!("driver/run.rs");
include!("driver/handle.rs");
