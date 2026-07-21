use crate::failure::{failure, unsupported};
use crate::protocol::{
    ATTACHED_VERSION, ChatTemplateCapabilities, DeploymentEvidence, Event, ObservedVersion,
    Readiness, Request, parse_event, parse_health, parse_models, parse_properties,
};
use crate::transport::{CurlTransport, Subscription};
use std::future::Future;
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    Capability, CredentialMechanism, ExternalNetworkPolicy, ExternalSearchPolicy,
    InstanceOwnership, ModelCatalogEntry, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, EndpointRef, HostServices,
    ModelCatalogDriver, ModelCatalogRequest, OperationContent, ProviderObservation, RunHandle,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId, StructuredRunDriver,
    StructuredRunRequest, TerminalOutcome, TerminalStatus, TokenUsage, runtime_event_channel,
    terminal_outcome_channel,
};

const DRIVER_ID: &str = "swallowtail.llama-cpp.attached-openai-chat";
const EVENT_CAPACITY: usize = 64;

mod owned;
pub use owned::{LlamaCppOwnedDriver, llama_cpp_owned_descriptor};
mod descriptor;
pub use descriptor::llama_cpp_attached_descriptor;

#[derive(Clone)]
pub struct LlamaCppAttachedDriver {
    transport: CurlTransport,
    driver_id: &'static str,
    version: ObservedVersion,
    run_id_prefix: &'static str,
}

impl Default for LlamaCppAttachedDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl LlamaCppAttachedDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::for_facade(DRIVER_ID, ATTACHED_VERSION, "llama-cpp-attached")
    }

    fn for_facade(
        driver_id: &'static str,
        version: ObservedVersion,
        run_id_prefix: &'static str,
    ) -> Self {
        Self {
            transport: CurlTransport,
            driver_id,
            version,
            run_id_prefix,
        }
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != self.driver_id {
            return Err(failure(
                "swallowtail.llama_cpp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.ownership() != InstanceOwnership::ExternalAttached
            || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
        {
            return Err(failure(
                "swallowtail.llama_cpp.access_binding_rejected",
                "llama.cpp requires an attached unauthenticated deployment binding",
            ));
        }
        Ok(())
    }

    async fn observe(
        &self,
        scope: ScopeId,
        endpoint: &str,
        expected_model: Option<&str>,
        deadline: Option<Deadline>,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<DeploymentEvidence, RuntimeFailure> {
        let health = complete_before_deadline(
            self.transport.request(
                scope.clone(),
                endpoint.to_owned(),
                Request::health(),
                services,
                Arc::clone(&cancelled),
            ),
            deadline,
            services,
            Arc::clone(&cancelled),
        )
        .await?;
        if parse_health(&health)? != Readiness::Ready {
            return Err(failure(
                "swallowtail.llama_cpp.deployment_loading",
                "llama.cpp deployment is still loading",
            ));
        }
        let properties = complete_before_deadline(
            self.transport.request(
                scope,
                endpoint.to_owned(),
                Request::properties(),
                services,
                Arc::clone(&cancelled),
            ),
            deadline,
            services,
            cancelled,
        )
        .await?;
        let evidence = parse_properties(&properties, self.version)?;
        validate_evidence(&evidence, expected_model)?;
        Ok(evidence)
    }
}

impl ModelCatalogDriver for LlamaCppAttachedDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = operation_scope("catalog", request.request_id().as_str())?;
            let endpoint = authorize_endpoint(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let evidence = self
                .observe(
                    scope.clone(),
                    &endpoint,
                    None,
                    request.deadline(),
                    &services,
                    Arc::clone(&cancelled),
                )
                .await?;
            let response = complete_before_deadline(
                self.transport.request(
                    scope,
                    endpoint,
                    Request::models(),
                    &services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                &services,
                cancelled,
            )
            .await?;
            let models = parse_models(&response)?;
            if models[0].id().as_str() != evidence.model_alias {
                return Err(failure(
                    "swallowtail.llama_cpp.catalogue_identity_mismatch",
                    "llama.cpp catalogue did not match deployment properties",
                ));
            }
            Ok(models)
        })
    }
}

async fn authorize_endpoint(
    plan: &PreflightPlan,
    scope: ScopeId,
    services: &HostServices,
) -> Result<String, RuntimeFailure> {
    let network = services.network().ok_or_else(|| missing("network"))?;
    let endpoint = EndpointRef::from_instance_target(plan.instance_target_ref());
    let grant = network
        .authorize(
            scope.clone(),
            endpoint.clone(),
            plan.endpoint_audience().clone(),
        )
        .await?;
    if grant.scope() != &scope
        || grant.endpoint() != &endpoint
        || grant.audience() != plan.endpoint_audience()
    {
        return Err(failure(
            "swallowtail.llama_cpp.network_grant_mismatch",
            "llama.cpp network grant did not match preflight",
        ));
    }
    Ok(grant.authorized().as_driver_value().to_owned())
}

fn require_services(services: &HostServices, run: bool) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || (run && services.task().is_none())
    {
        Err(missing("required host"))
    } else {
        Ok(())
    }
}

fn missing(service: &str) -> RuntimeFailure {
    failure(
        "swallowtail.llama_cpp.host_service_missing",
        format!("llama.cpp attached inference requires the {service} service"),
    )
}

fn operation_scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("llama-cpp-attached:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.llama_cpp.scope_invalid",
            "llama.cpp operation scope was invalid",
        )
    })
}

include!("driver/lifecycle.rs");
include!("driver/validation.rs");
include!("driver/run.rs");
include!("driver/pump.rs");
include!("driver/handle.rs");
