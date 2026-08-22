mod lifecycle;
mod turn;

use self::lifecycle::{ActiveSlot, SessionCancellation, close_active};
use super::*;
use crate::failure::unsupported;
use crate::protocol::ChatMessage;
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, OperationShape,
    SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    CancellationControl, InteractiveSessionDriver, InteractiveSessionHandle, OpenSessionRequest,
    RequestId, ResumeSessionRequest, RuntimeSessionId, SessionAccessPolicy, SessionResumeBinding,
    TurnHandle, TurnRequest, validate_session_plan_agreement,
};

pub(super) struct SessionState {
    pub(super) history: Vec<ChatMessage>,
    pub(super) completed_turns: u32,
    pub(super) usable: bool,
}

pub(super) struct OllamaSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    pub(super) model: String,
    pub(super) context_window: Option<crate::OllamaContextWindow>,
    pub(super) plan: PreflightPlan,
    pub(super) endpoint: String,
    pub(super) services: HostServices,
    pub(super) state: Arc<Mutex<SessionState>>,
    pub(super) active: ActiveSlot,
    cancellation: Arc<SessionCancellation>,
    pub(super) transport: CurlTransport,
}

impl InteractiveSessionDriver for OllamaNativeAttachedDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            self.validate_prepared_dispatch(&plan, request.request_id())?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, true)?;
            validate_open(&plan, &request, &services)?;
            let scope = operation_scope("session", request.request_id().as_str())?;
            let endpoint = authorize_endpoint(&plan, scope, &services).await?;
            let active = Arc::new(Mutex::new(None));
            let state = Arc::new(Mutex::new(SessionState {
                history: Vec::new(),
                completed_turns: 0,
                usable: true,
            }));
            let cancellation = Arc::new(SessionCancellation::new(
                Arc::clone(&active),
                Arc::clone(&state),
            ));
            let runtime_id =
                RuntimeSessionId::new(format!("ollama-native:{}", request.request_id().as_str()))
                    .map_err(|_| {
                    failure(
                        "swallowtail.ollama.session_id_invalid",
                        "Ollama runtime session identity was invalid",
                    )
                })?;
            let model = plan
                .requirements()
                .attached_runtime()
                .expect("validated Ollama attached requirements")
                .model_tag()
                .as_str()
                .to_owned();
            Ok(Box::new(OllamaSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                model,
                context_window: self.context_window(),
                plan,
                endpoint,
                services,
                state,
                active,
                cancellation,
                transport: self.transport.clone(),
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
    }
}

impl InteractiveSessionHandle for OllamaSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        None
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        None
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start_turn_inner(request, services).await })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let cleanup = close_active(&self.active).await;
            let mut state = self.state.lock().expect("Ollama session lock poisoned");
            state.usable = false;
            state.history.clear();
            cleanup
        })
    }
}

fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if plan.requirements().operation_shape() != OperationShape::InteractiveSession
        || plan.requirements().driver_role() != DriverRole::InteractiveSession
        || plan.model_id().is_none()
        || plan.provider_id().is_some()
    {
        return Err(failure(
            "swallowtail.ollama.session_binding_rejected",
            "Ollama interactive session requires one exact prepared model route",
        ));
    }
    let required = |capability| {
        plan.requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == capability)
    };
    for capability in [
        Capability::InteractiveSession,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::OutputTokenLimit,
        Capability::Interruption,
    ] {
        if required(capability).is_none() {
            return Err(failure(
                "swallowtail.ollama.session_capability_rejected",
                "Ollama interactive session capabilities were incomplete",
            ));
        }
    }
    let exact_constraint = |capability, constraint| {
        required(capability)
            .is_some_and(|requirement| requirement.constraints().eq(std::iter::once(&constraint)))
    };
    let interactive_constraints = required(Capability::InteractiveSession)
        .expect("validated Ollama interactive capability")
        .constraints()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    if interactive_constraints
        != std::collections::BTreeSet::from([
            CapabilityConstraint::MaximumTurns(24),
            CapabilityConstraint::PrivateHistoryMaximumBytes(1_048_576),
        ])
        || !exact_constraint(
            Capability::StreamingEvents,
            CapabilityConstraint::StreamRecordMaximumCount(4096),
        )
        || !exact_constraint(
            Capability::Interruption,
            CapabilityConstraint::CancellationScope(CancellationScope::ActiveTurn),
        )
        || !exact_constraint(
            Capability::OutputTokenLimit,
            CapabilityConstraint::OutputTokenMaximum(8),
        )
    {
        return Err(failure(
            "swallowtail.ollama.session_bound_rejected",
            "Ollama interactive session bounds did not match the qualified profile",
        ));
    }
    if request.working_resource().is_some()
        || request.access_policy() != &SessionAccessPolicy::resource_free()
        || request.provider_state_policy() != Some(SessionProviderStatePolicy::Prohibited)
        || !request.options().is_empty()
    {
        return Err(unsupported(
            "working resource, provider state, or session options",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated Ollama time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.ollama.deadline_elapsed",
            "Ollama session deadline elapsed before opening",
        ));
    }
    Ok(())
}
