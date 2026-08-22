use crate::failure::failure;
use crate::transport::CurlTransport;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, Capability, CredentialMechanism, DriverDescriptor,
    DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape,
    PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, DebugObservationKind, EndpointRef, HostServices,
    RuntimeFailure, ScopeId,
};

const DRIVER_ID: &str = "swallowtail.openai.background";
const ROUTE: &str = "openai.background";

#[derive(Clone, Default)]
/// Low-level driver for one provider-managed OpenAI background response.
pub struct OpenAiBackgroundDriver {
    transport: CurlTransport,
}

impl OpenAiBackgroundDriver {
    #[must_use]
    /// Creates a background Responses driver using the adapter transport.
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.openai.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
            || plan.endpoint_audience().as_str() != crate::ENDPOINT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.openai.access_binding_rejected",
                "OpenAI background inference requires the exact public API-key access boundary",
            ));
        }
        let exact_facade = crate::openai_background_facade_binding();
        if plan.protocol_facade_id().as_str() != crate::OPENAI_BACKGROUND_FACADE_REVISION
            || !plan
                .interface_versions()
                .any(|binding| binding == &exact_facade)
            || !plan.assess_interface_version(&exact_facade).is_permitted()
        {
            return Err(failure(
                "swallowtail.openai.facade_binding_rejected",
                "OpenAI background inference requires the exact qualified Responses facade",
            ));
        }
        Ok(())
    }
}

#[must_use]
/// Returns the descriptor for background Responses execution and reconciliation.
pub fn openai_background_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new(crate::INTEGRATION_FAMILY).expect("static family id is valid"),
        TransportFamilyId::new("http-sse-background").expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::StructuredRun,
        DriverRole::ProviderRunReconciliation,
    ])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([
        OperationShape::StructuredRun,
        OperationShape::ProviderRunReconciliation,
    ])
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
    .with_required_host_services(
        DriverRole::ProviderRunReconciliation,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
    .with_interface_compatibility(crate::openai_background_facade_claim())
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
        let network = services.network().ok_or_else(missing_services)?;
        let credentials = services.credential().ok_or_else(missing_services)?;
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
                "swallowtail.openai.network_grant_mismatch",
                "OpenAI network grant did not match preflight",
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
                "swallowtail.openai.credential_lease_rejected",
                "OpenAI credential lease did not match preflight",
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
                "swallowtail.openai.credential_unavailable",
                "OpenAI credential was unavailable",
            )),
        }
    }

    async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.openai.credential_release_failed",
                    "OpenAI credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        }
    }
}

fn require_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    if services.task().is_none()
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        Err(missing_services())
    } else {
        Ok(())
    }
}

fn missing_services() -> RuntimeFailure {
    failure(
        "swallowtail.openai.host_service_missing",
        "OpenAI background inference requires task, blocking-work, time, network, and credential services",
    )
}

fn operation_scope(id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("openai-background:run:{id}")).map_err(|_| {
        failure(
            "swallowtail.openai.scope_invalid",
            "OpenAI operation scope was invalid",
        )
    })
}

fn requires(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}

include!("driver/validation.rs");
include!("driver/handle.rs");
include!("driver/lifecycle.rs");
include!("driver/observations.rs");
include!("driver/run.rs");
include!("driver/pump.rs");
include!("driver/reconciliation.rs");
