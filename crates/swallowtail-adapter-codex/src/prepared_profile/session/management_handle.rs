use super::super::plan::instance_with_capabilities;
use crate::selection::classify_lifecycle_version;
use crate::{CodexPreparedIntegration, codex_app_server_descriptor};
use swallowtail_core::{CapabilityProfile, CapabilityRequirement, ProviderSessionBindingOrigin};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, DirectContinuationTurnRequest, HostServices,
    InteractiveSessionHandle, PreparedAccessEvidence, ProviderSessionManagementBinding,
    RuntimeFailure, SessionResumeBinding, TurnHandle, TurnRequest, WorkingResourceRef,
};

pub(super) fn lifecycle_management_instance(
    prepared: &CodexPreparedIntegration,
) -> Option<swallowtail_core::ConfiguredInstance> {
    let lifecycle = classify_lifecycle_version(prepared.observation().version().version())?;
    let capabilities = CapabilityProfile::new(
        lifecycle
            .behavior
            .capabilities()
            .iter()
            .copied()
            .map(|capability| CapabilityRequirement::new(capability, [])),
    );
    Some(instance_with_capabilities(prepared, capabilities))
}

pub(super) async fn wrap_management_handle(
    handle: Box<dyn InteractiveSessionHandle>,
    instance: Option<swallowtail_core::ConfiguredInstance>,
    access: PreparedAccessEvidence,
    working_resource: Option<WorkingResourceRef>,
    origin: ProviderSessionBindingOrigin,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    let Some(instance) = instance else {
        return Ok(handle);
    };
    let Some(provider_ref) = handle.provider_session_ref().cloned() else {
        return Ok(handle);
    };
    let binding = ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &codex_app_server_descriptor(),
        &instance,
        access,
        working_resource,
        origin,
    );
    match binding {
        Ok(binding) => Ok(Box::new(ManagedCodexSessionHandle {
            inner: handle,
            binding,
        })),
        Err(error) => {
            let _ = handle.close().await;
            Err(RuntimeFailure::new(error.diagnostic().clone()))
        }
    }
}

struct ManagedCodexSessionHandle {
    inner: Box<dyn InteractiveSessionHandle>,
    binding: ProviderSessionManagementBinding,
}

impl InteractiveSessionHandle for ManagedCodexSessionHandle {
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

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        self.inner.close()
    }
}
