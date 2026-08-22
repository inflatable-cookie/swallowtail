use crate::failure::failure;
use crate::protocol::{
    ObservationBinding, Request, parse_inventory, parse_model_detail, parse_version,
};
use crate::selection::{OLLAMA_DRIVER_ID, OLLAMA_NATIVE_FACADE};
use crate::transport::{CurlTransport, Subscription};
use std::collections::BTreeMap;
use std::future::Future;
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    AttachedModelObservation, AttachedModelObservationScope, Capability, CapabilityConstraint,
    CredentialMechanism, ExternalNetworkPolicy, ExternalSearchPolicy, InstanceOwnership,
    ModelCatalogEntry, ModelId, ModelMetadata, PreflightPlan, StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, DebugObservationKind, EndpointRef,
    HostServices, ModelCatalogDriver, ModelCatalogRequest, OperationContent, ProviderObservation,
    RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus, TokenUsage,
    runtime_event_channel, terminal_outcome_channel,
};

mod session;

const EVENT_CAPACITY: usize = 64;
const ROUTE: &str = "ollama.attached";

#[derive(Clone, Default)]
/// Low-level driver for one externally managed Ollama native HTTP runtime.
pub struct OllamaNativeAttachedDriver {
    transport: CurlTransport,
    prepared_dispatch_binding:
        Option<crate::prepared_dispatch_binding::OllamaPreparedDispatchBinding>,
}

impl OllamaNativeAttachedDriver {
    /// Creates a driver using the adapter's bounded HTTP transport.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a driver bound to the exact context selection in prepared evidence.
    #[must_use]
    pub fn bound_to_prepared_evidence(evidence: &crate::OllamaPreparedEvidence) -> Self {
        Self {
            transport: CurlTransport,
            prepared_dispatch_binding: Some(
                crate::prepared_dispatch_binding::OllamaPreparedDispatchBinding::from_evidence(
                    evidence,
                ),
            ),
        }
    }

    pub(super) fn context_window(&self) -> Option<crate::OllamaContextWindow> {
        self.prepared_dispatch_binding.as_ref().and_then(
            crate::prepared_dispatch_binding::OllamaPreparedDispatchBinding::context_window,
        )
    }

    pub(crate) fn from_transport(transport: CurlTransport) -> Self {
        Self {
            transport,
            prepared_dispatch_binding: None,
        }
    }

    /// Validates that this driver matches one prepared evidence snapshot.
    pub fn validate_against_prepared_evidence(
        &self,
        evidence: &crate::OllamaPreparedEvidence,
    ) -> Result<(), RuntimeFailure> {
        let expected =
            crate::prepared_dispatch_binding::OllamaPreparedDispatchBinding::from_evidence(
                evidence,
            );
        match &self.prepared_dispatch_binding {
            Some(binding) if binding == &expected => Ok(()),
            _ => Err(failure(
                "swallowtail.ollama.context_window_binding_mismatch",
                "Ollama prepared dispatch binding did not match prepared evidence",
            )),
        }
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != OLLAMA_DRIVER_ID {
            return Err(failure(
                "swallowtail.ollama.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.ownership() != InstanceOwnership::ExternalAttached
            || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
        {
            return Err(failure(
                "swallowtail.ollama.access_binding_rejected",
                "Ollama requires an attached local-unauthenticated runtime binding",
            ));
        }
        if plan.protocol_facade_id().as_str() != OLLAMA_NATIVE_FACADE {
            return Err(failure(
                "swallowtail.ollama.facade_binding_rejected",
                "Ollama requires the qualified native text facade",
            ));
        }
        let requirements = plan.requirements().attached_runtime().ok_or_else(|| {
            failure(
                "swallowtail.ollama.attached_binding_missing",
                "Ollama requires exact attached-runtime requirements",
            )
        })?;
        let observation = plan.attached_model_observation().ok_or_else(|| {
            failure(
                "swallowtail.ollama.model_observation_missing",
                "Ollama requires selected-model detail evidence",
            )
        })?;
        if !plan
            .assess_interface_version(requirements.runtime_version())
            .is_permitted()
            || plan.model_id() != Some(requirements.model_id())
            || plan.provider_id().is_some()
            || observation.scope() != AttachedModelObservationScope::SelectedModelDetail
            || observation.runtime_version() != requirements.runtime_version()
            || observation.model_tag() != requirements.model_tag()
            || observation.manifest_digest() != Some(requirements.manifest_digest())
        {
            return Err(failure(
                "swallowtail.ollama.attached_binding_mismatch",
                "Ollama runtime, route, tag, or digest binding did not match preflight",
            ));
        }
        if let Some(binding) = &self.prepared_dispatch_binding {
            binding.validate_against(plan)?;
        }
        Ok(())
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
            "swallowtail.ollama.network_grant_mismatch",
            "Ollama network grant did not match preflight",
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
        "swallowtail.ollama.host_service_missing",
        format!("Ollama native attached inference requires the {service} service"),
    )
}

fn operation_scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("ollama-native-attached:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.ollama.scope_invalid",
            "Ollama operation scope was invalid",
        )
    })
}

include!("driver/lifecycle.rs");
include!("driver/catalogue.rs");
include!("driver/validation.rs");
include!("driver/run.rs");
include!("driver/pump.rs");
include!("driver/handle.rs");
