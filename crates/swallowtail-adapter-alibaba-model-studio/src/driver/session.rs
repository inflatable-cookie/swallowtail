use super::AlibabaModelStudioDriver;
use super::access::AccessLeases;
use super::lifecycle::{
    ActiveSlot, SessionCancellation, close_active, complete_before_deadline, merge_cleanup,
};
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{ConversationRef, WireRequest, parse_conversation};
use crate::transport::CurlTransport;
use cleanup::{CleanupAccess, cleanup_conversation};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{PreflightPlan, SessionRef};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionAccessPolicy, SessionResumeBinding, TurnHandle, TurnRequest,
    validate_session_access_plan, validate_session_provider_state_plan,
};

mod cleanup;

pub(super) struct AlibabaSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) scope: ScopeId,
    pub(super) services: HostServices,
    pub(super) transport: CurlTransport,
    pub(super) conversation: ConversationRef,
    pub(super) access: Option<AccessLeases>,
    pub(super) completed_turns: Arc<AtomicU8>,
    pub(super) usable: Arc<AtomicBool>,
    pub(super) remote_uncertain: Arc<AtomicBool>,
    pub(super) active: ActiveSlot,
    pub(super) cancellation: Arc<SessionCancellation>,
}

impl InteractiveSessionDriver for AlibabaModelStudioDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "alibaba-model-studio:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.alibaba_model_studio.scope_invalid",
                    "Alibaba Model Studio session scope was invalid",
                )
            })?;
            let runtime_id = RuntimeSessionId::new(format!(
                "alibaba-model-studio:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.alibaba_model_studio.session_id_invalid",
                    "Alibaba Model Studio runtime session identity was invalid",
                )
            })?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let response = complete_before_deadline(
                self.transport.request(
                    scope.clone(),
                    access.endpoint.clone(),
                    access.secret()?,
                    WireRequest::create_conversation(),
                    &services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                &services,
                cancelled,
            )
            .await;
            let conversation = match response
                .and_then(|response| parse_conversation(&response.body).map_err(protocol))
            {
                Ok(conversation) => conversation,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let active = Arc::new(Mutex::new(None));
            let usable = Arc::new(AtomicBool::new(true));
            let cancellation = Arc::new(SessionCancellation::new(
                Arc::clone(&active),
                Arc::clone(&usable),
            ));
            Ok(Box::new(AlibabaSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                scope,
                services,
                transport: self.transport.clone(),
                conversation,
                access: Some(access),
                completed_turns: Arc::new(AtomicU8::new(0)),
                usable,
                remote_uncertain: Arc::new(AtomicBool::new(false)),
                active,
                cancellation,
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("conversation resume")) })
    }
}

impl InteractiveSessionHandle for AlibabaSessionHandle {
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

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let active = close_active(&self.active).await;
            let cleanup_access = self.access.as_ref().map(CleanupAccess::acquire);
            let remote = match cleanup_access {
                Some(Ok(access)) => {
                    cleanup_conversation(
                        &self.transport,
                        &self.scope,
                        &self.services,
                        &access,
                        &self.conversation,
                        self.remote_uncertain.load(Ordering::SeqCst),
                    )
                    .await
                }
                Some(Err(error)) => CleanupOutcome::Failed(error.diagnostic().clone()),
                None => CleanupOutcome::NotApplicable,
            };
            let credential = match self.access.as_mut() {
                Some(access) => access.release(&self.services).await,
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(merge_cleanup(active, remote), credential)
        })
    }
}

fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if services.task().is_none()
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.host_services_missing",
            "Alibaba Model Studio required host services are unavailable",
        ));
    }
    validate_session_access_plan(plan, request.access_policy())?;
    validate_session_provider_state_plan(plan, request.provider_state_policy())?;
    if request.working_resource().is_some()
        || request.access_policy() != &SessionAccessPolicy::resource_free()
    {
        return Err(unsupported("a working resource"));
    }
    if !request.options().is_empty() {
        return Err(unsupported("session options"));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.deadline_elapsed",
            "Alibaba Model Studio session deadline elapsed before provider work",
        ));
    }
    Ok(())
}
