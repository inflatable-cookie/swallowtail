use super::DeepSeekDirectDriver;
use super::access::AccessLeases;
use super::catalogue::{operation_scope, require_services};
use super::history::SessionHistory;
use super::lifecycle::{ActiveSlot, SessionCancellation, close_active, merge_cleanup};
use crate::failure::{failure, unsupported};
use crate::protocol::ToolSpec;
use crate::selection::validate_deepseek_request_plan;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{PreflightPlan, SessionRef};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, DirectContinuationBinding,
    DirectContinuationState, DirectContinuationTurnRequest, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenDirectContinuationSessionRequest, OpenSessionRequest,
    ProviderPrivateContinuationRecord, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionResumeBinding, TurnHandle, TurnRequest,
};

pub(super) struct DeepSeekSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) plan: PreflightPlan,
    pub(super) scope: swallowtail_runtime::ScopeId,
    pub(super) services: HostServices,
    pub(super) transport: crate::transport::CurlTransport,
    pub(super) endpoint: String,
    pub(super) access: Option<AccessLeases>,
    pub(super) tools: Arc<Vec<ToolSpec>>,
    pub(super) state: Arc<Mutex<DirectContinuationState>>,
    pub(super) history: Arc<Mutex<SessionHistory>>,
    pub(super) private_records: Arc<Mutex<Vec<ProviderPrivateContinuationRecord>>>,
    pub(super) binding: DirectContinuationBinding,
    pub(super) usable: Arc<AtomicBool>,
    pub(super) active: ActiveSlot,
    pub(super) cancellation: Arc<SessionCancellation>,
}

impl InteractiveSessionDriver for DeepSeekDirectDriver {
    fn open_session(
        &self,
        _plan: PreflightPlan,
        _request: OpenSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("the generic session-open request")) })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
    }

    fn open_direct_continuation_session(
        &self,
        plan: PreflightPlan,
        request: OpenDirectContinuationSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_deepseek_request_plan(&plan, &request)?;
            require_services(&services, true)?;
            let tools = tool_specs(&request)?;
            if tools.is_empty() {
                return Err(failure(
                    "swallowtail.deepseek.tools_missing",
                    "DeepSeek continuation requires at least one declared consumer tool",
                ));
            }
            let scope = operation_scope("session", request.request_id().as_str())?;
            let runtime_id =
                RuntimeSessionId::new(format!("deepseek-direct:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.deepseek.session_id_invalid",
                            "DeepSeek runtime session identity was invalid",
                        )
                    })?;
            let access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let endpoint = access.endpoint.clone();
            let config = request.config().clone();
            let binding = DirectContinuationBinding::from_plan(&plan, runtime_id.clone())?;
            let active = Arc::new(Mutex::new(None));
            let usable = Arc::new(AtomicBool::new(true));
            let cancellation = Arc::new(SessionCancellation::new(
                Arc::clone(&active),
                Arc::clone(&usable),
            ));
            Ok(Box::new(DeepSeekSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                plan,
                scope,
                services,
                transport: self.transport.clone(),
                endpoint,
                access: Some(access),
                tools: Arc::new(tools),
                state: Arc::new(Mutex::new(DirectContinuationState::new(config.clone()))),
                history: Arc::new(Mutex::new(SessionHistory::new(&config))),
                private_records: Arc::new(Mutex::new(Vec::new())),
                binding,
                usable,
                active,
                cancellation,
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }
}

impl InteractiveSessionHandle for DeepSeekSessionHandle {
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
        _request: TurnRequest,
        _services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("the generic turn request")) })
    }

    fn start_direct_continuation_turn<'a>(
        &'a mut self,
        request: DirectContinuationTurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start_direct_turn(request, services).await })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            self.usable.store(false, Ordering::SeqCst);
            self.state
                .lock()
                .expect("continuation state lock poisoned")
                .invalidate();
            let active = close_active(&self.active).await;
            *self.history.lock().expect("history lock poisoned") = SessionHistory::new(
                self.plan
                    .requirements()
                    .direct_continuation()
                    .expect("validated continuation")
                    .config(),
            );
            self.private_records
                .lock()
                .expect("private record lock poisoned")
                .clear();
            let credential = match self.access.as_mut() {
                Some(access) => access.release(&self.services).await,
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(active, credential)
        })
    }
}

fn tool_specs(
    request: &OpenDirectContinuationSessionRequest,
) -> Result<Vec<ToolSpec>, RuntimeFailure> {
    request
        .options()
        .tools()
        .map(|tool| {
            if tool.schema_media_type() != "application/schema+json"
                || tool.schema_dialect() != "json-schema-2020-12"
            {
                return Err(unsupported("non-JSON-Schema tool declarations"));
            }
            let bytes = tool
                .input_schema()
                .inline_bytes()
                .ok_or_else(|| unsupported("referenced tool schemas"))?;
            let parameters = serde_json::from_slice(bytes).map_err(|_| {
                failure(
                    "swallowtail.deepseek.tool_schema_invalid",
                    "DeepSeek tool schema was not valid JSON",
                )
            })?;
            Ok(ToolSpec {
                name: tool.name().to_owned(),
                description: tool
                    .description()
                    .map_or_else(String::new, |description| description.as_str().to_owned()),
                parameters,
            })
        })
        .collect()
}
