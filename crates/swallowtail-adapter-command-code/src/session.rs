mod lifecycle;
mod turn;

use self::lifecycle::{ActiveSlot, SessionCancellation, close_active};
use crate::CommandCodeHeadlessDriver;
use crate::failure::{failure, unsupported};
use crate::validation::validate_session;
use std::sync::{Arc, Mutex};
use swallowtail_core::{PreflightPlan, SessionRef};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionResumeBinding, TurnHandle, TurnRequest,
};

pub(super) struct SessionState {
    pub(super) provider_session_id: Option<String>,
    pub(super) completed_turns: u32,
    pub(super) usable: bool,
}

pub(super) struct CommandCodeSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    pub(super) model: swallowtail_core::ModelId,
    pub(super) working_resource: swallowtail_runtime::WorkingResourceRef,
    pub(super) services: HostServices,
    pub(super) state: Arc<Mutex<SessionState>>,
    pub(super) active: ActiveSlot,
    cancellation: Arc<SessionCancellation>,
    pub(super) environment: swallowtail_runtime::EnvironmentRef,
    pub(super) target: swallowtail_core::InstanceTargetRef,
}

impl InteractiveSessionDriver for CommandCodeHeadlessDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session(&plan, &request, &services)?;
            let active = Arc::new(Mutex::new(None));
            let state = Arc::new(Mutex::new(SessionState {
                provider_session_id: None,
                completed_turns: 0,
                usable: true,
            }));
            let cancellation = Arc::new(SessionCancellation::new(
                Arc::clone(&active),
                Arc::clone(&state),
            ));
            let runtime_id = RuntimeSessionId::new(format!(
                "command-code-headless:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.command_code.headless.session_id_invalid",
                    "Command Code runtime session identity was invalid",
                )
            })?;
            Ok(Box::new(CommandCodeSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                model: plan
                    .model_id()
                    .cloned()
                    .expect("validated model is present"),
                working_resource: request
                    .working_resource()
                    .cloned()
                    .expect("validated working resource is present"),
                services,
                state,
                active,
                cancellation,
                environment: self.environment().clone(),
                target: plan.instance_target_ref().clone(),
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("public session resume")) })
    }
}

impl InteractiveSessionHandle for CommandCodeSessionHandle {
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
            self.state
                .lock()
                .expect("Command Code session lock poisoned")
                .usable = false;
            close_active(&self.active).await
        })
    }
}
