use swallowtail_core::ProviderSessionBindingOrigin;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, DirectContinuationTurnRequest, HostServices,
    InteractiveSessionHandle, PreparedAccessEvidence, ProviderSessionManagementBinding,
    RuntimeFailure, SessionResumeBinding, TurnHandle, TurnRequest, WorkingResourceRef,
};

pub(super) fn validate_management_context(
    instance: &swallowtail_core::ConfiguredInstance,
    access: &PreparedAccessEvidence,
) -> Result<(), RuntimeFailure> {
    ProviderSessionManagementBinding::validate_bound_session_context(
        &crate::claude_agent_acp_descriptor(),
        instance,
        access,
    )
    .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))
}

pub(super) async fn wrap_management_handle(
    handle: Box<dyn InteractiveSessionHandle>,
    instance: swallowtail_core::ConfiguredInstance,
    access: PreparedAccessEvidence,
    working_resource: Option<WorkingResourceRef>,
    origin: ProviderSessionBindingOrigin,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    let Some(provider_ref) = handle.provider_session_ref().cloned() else {
        return Ok(handle);
    };
    let binding = ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &crate::claude_agent_acp_descriptor(),
        &instance,
        access,
        working_resource,
        origin,
    )
    .expect("management context was validated before provider session work");
    Ok(Box::new(ManagedClaudeAgentSessionHandle {
        inner: handle,
        binding,
    }))
}

struct ManagedClaudeAgentSessionHandle {
    inner: Box<dyn InteractiveSessionHandle>,
    binding: ProviderSessionManagementBinding,
}

impl InteractiveSessionHandle for ManagedClaudeAgentSessionHandle {
    fn request_id(&self) -> &swallowtail_runtime::RequestId {
        self.inner.request_id()
    }

    fn session_id(&self) -> &swallowtail_runtime::RuntimeSessionId {
        self.inner.session_id()
    }

    fn provider_session_ref(&self) -> Option<&swallowtail_core::SessionRef> {
        self.inner.provider_session_ref()
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        self.inner.resume_binding()
    }

    fn negotiated_model_options(
        &self,
    ) -> Option<&swallowtail_runtime::NegotiatedSessionModelOptions> {
        self.inner.negotiated_model_options()
    }

    fn management_binding(&self) -> Option<&ProviderSessionManagementBinding> {
        Some(&self.binding)
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        self.inner.start_turn(request, services)
    }

    fn start_direct_continuation_turn<'a>(
        &'a mut self,
        request: DirectContinuationTurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        self.inner.start_direct_continuation_turn(request, services)
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.inner.cancellation()
    }

    fn close(
        self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        self.inner.close(request, services)
    }
}
