use super::access::AccessLeases;
use super::lifecycle::{ActiveSlot, SessionCancellation, close_active, merge_cleanup};
use super::{PROVIDER_ID, XaiWebSocketDriver};
use crate::failure::{failure, unsupported};
use crate::transport::Connection;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, PreflightPlan, SessionRef,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionAccessPolicy, SessionResumeBinding, TurnHandle, TurnRequest,
    validate_session_access_plan,
};

pub(super) struct XaiSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) model: String,
    pub(super) model_route_id: swallowtail_core::ModelRouteId,
    pub(super) access_profile_id: swallowtail_core::AccessProfileId,
    pub(super) scope: ScopeId,
    pub(super) services: HostServices,
    pub(super) connection: Connection,
    pub(super) access: Option<AccessLeases>,
    pub(super) continuation: Arc<Mutex<Option<String>>>,
    pub(super) chain_valid: Arc<AtomicBool>,
    pub(super) active: ActiveSlot,
    pub(super) cancellation: Arc<SessionCancellation>,
}

impl InteractiveSessionDriver for XaiWebSocketDriver {
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
            let model = plan
                .model_id()
                .expect("validated model")
                .as_str()
                .to_owned();
            let model_route_id = plan.model_route_id().expect("validated route").clone();
            let access_profile_id = plan.access_profile_id().clone();
            let scope = ScopeId::new(format!(
                "xai-websocket:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.xai.scope_invalid",
                    "xAI session scope was invalid",
                )
            })?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let connection = match access.connect(scope.clone(), &services).await {
                Ok(connection) => connection,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let active = Arc::new(Mutex::new(None));
            let cancellation = Arc::new(SessionCancellation::new(
                connection.closer(),
                Arc::clone(&active),
            ));
            let runtime_id =
                RuntimeSessionId::new(format!("xai-websocket:{}", request.request_id().as_str()))
                    .map_err(|_| {
                    failure(
                        "swallowtail.xai.session_id_invalid",
                        "xAI runtime session identity was invalid",
                    )
                })?;
            Ok(Box::new(XaiSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                model,
                model_route_id,
                access_profile_id,
                scope,
                services,
                connection,
                access: Some(access),
                continuation: Arc::new(Mutex::new(None)),
                chain_valid: Arc::new(AtomicBool::new(true)),
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
        Box::pin(async { Err(unsupported("session resume")) })
    }
}

impl InteractiveSessionHandle for XaiSessionHandle {
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
            let connection = self.connection.clone();
            let connection_cleanup = match self.services.blocking_work() {
                Some(blocking) => {
                    let work =
                        blocking.run(self.scope.clone(), Box::new(move || connection.close()));
                    super::lifecycle::cleanup_from_result(work.await)
                }
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.xai.blocking_service_missing",
                    "xAI blocking-work service disappeared during cleanup",
                )),
            };
            let credential = match self.access.as_mut() {
                Some(access) => access.release(&self.services).await,
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(merge_cleanup(active, connection_cleanup), credential)
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
            "swallowtail.xai.host_services_missing",
            "xAI WebSocket required host services are unavailable",
        ));
    }
    if plan
        .provider_id()
        .is_none_or(|id| id.as_str() != PROVIDER_ID)
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(failure(
            "swallowtail.xai.model_binding_rejected",
            "xAI WebSocket requires one exact xAI model route",
        ));
    }
    for capability in [
        Capability::InteractiveSession,
        Capability::StreamingEvents,
        Capability::Interruption,
        Capability::UsageReporting,
        Capability::BilledCostReporting,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == capability)
        {
            return Err(failure(
                "swallowtail.xai.capability_binding_rejected",
                "xAI WebSocket capability requirements were incomplete",
            ));
        }
    }
    let interruption = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::Interruption)
        .expect("validated interruption capability");
    let constraints: Vec<_> = interruption.constraints().collect();
    if constraints
        != [&CapabilityConstraint::CancellationScope(
            CancellationScope::ActiveTurn,
        )]
    {
        return Err(failure(
            "swallowtail.xai.interruption_binding_rejected",
            "xAI WebSocket interruption must be bound to the active turn",
        ));
    }
    validate_session_access_plan(plan, request.access_policy())?;
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
            "swallowtail.xai.deadline_elapsed",
            "xAI session deadline elapsed before provider work",
        ));
    }
    Ok(())
}
