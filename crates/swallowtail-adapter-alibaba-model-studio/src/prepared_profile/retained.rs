use super::input::AlibabaRetainedConversationProfileInput;
use super::plan::{
    AlibabaModelStudioPreparedEvidence, build_plan, instance_with_capabilities, model_route,
};
use crate::prepared::failure;
use crate::{AlibabaModelStudioDriver, AlibabaModelStudioPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, PreflightPlan,
    ProviderSessionBindingOrigin, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, LoadSessionRequest, LoadedSession, OpenSessionRequest,
    PreparationFailure, PreparationStage, PreparedAccessEvidence, ProviderSessionManagementBinding,
    RuntimeFailure, SessionPlanAgreement, SessionResumeBinding, TurnHandle, TurnRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Executable preserved conversation with bounded replay and resume support.
pub struct AlibabaModelStudioPreparedRetainedConversation {
    evidence: AlibabaModelStudioPreparedEvidence,
    request: OpenSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
}

impl AlibabaModelStudioPreparedRetainedConversation {
    #[must_use]
    /// Returns route-specific prepared evidence.
    pub const fn evidence(&self) -> &AlibabaModelStudioPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable retained-conversation preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived new-session request.
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    /// Returns the public low-level workspace driver.
    pub fn low_level_driver(&self) -> AlibabaModelStudioDriver {
        AlibabaModelStudioDriver::new()
    }

    #[must_use]
    pub(super) const fn management_instance(&self) -> &swallowtail_core::ConfiguredInstance {
        &self.management_instance
    }

    /// Opens a new preserved conversation and binds later management authority.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            validate_management_context(&instance, &access)?;
            let handle = driver.open_session(plan, request, services).await?;
            wrap_management_handle(
                handle,
                instance,
                access,
                ProviderSessionBindingOrigin::Created,
            )
            .await
        })
    }

    /// Builds a replaying load request from one exact resume binding.
    pub fn load_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        LoadSessionRequest::resource_free_from_plan(
            self.plan(),
            request_id,
            binding,
            self.request.deadline(),
        )
    }

    /// Loads bounded replay before returning the live retained session.
    pub fn load_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(load_retained_session(
            driver, plan, request, instance, access, services,
        )))
    }
}

pub(super) async fn load_retained_session(
    driver: AlibabaModelStudioDriver,
    plan: PreflightPlan,
    request: LoadSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
    access: PreparedAccessEvidence,
    services: HostServices,
) -> Result<LoadedSession, RuntimeFailure> {
    validate_management_context(&management_instance, &access)?;
    let loaded = driver.load_session(plan, request, services).await?;
    let (replay, handle) = loaded.into_parts();
    let handle = wrap_management_handle(
        handle,
        management_instance,
        access,
        ProviderSessionBindingOrigin::Loaded,
    )
    .await?;
    Ok(LoadedSession::new(replay, handle))
}

impl AlibabaModelStudioPreparedIntegration {
    /// Validates and prepares one preserved, loadable conversation.
    pub fn prepare_retained_conversation(
        &self,
        input: AlibabaRetainedConversationProfileInput,
    ) -> Result<AlibabaModelStudioPreparedRetainedConversation, PreparationFailure> {
        let (request_id, route_id, route_revision, model_id, deadline) = input.into_parts();
        if route_id.as_str() != crate::MODEL_ROUTE_ID || model_id.as_str() != crate::EXACT_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.route_rejected",
                "Alibaba Model Studio preparation requires the exact Singapore Qwen route",
            ));
        }
        let activity = crate::activity::profile::activity_profile();
        let base_requirements = crate::alibaba_model_studio_retained_requirements(
            self.instance().execution_host_id().clone(),
        );
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(base_requirements.capabilities().cloned()),
            &activity,
        );
        let requirements = base_requirements.with_capabilities(capabilities.iter().map(
            |(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            },
        ));
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, route_id, route_revision, model_id, capabilities);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let agreement = SessionPlanAgreement::explicit(
            SessionAccessPolicy::resource_free(),
            Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
            None,
        );
        let request = OpenSessionRequest::resource_free(request_id, deadline, agreement);
        Ok(AlibabaModelStudioPreparedRetainedConversation {
            evidence: AlibabaModelStudioPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
            management_instance: lifecycle_management_instance(self),
        })
    }
}

pub(super) fn lifecycle_management_instance(
    prepared: &AlibabaModelStudioPreparedIntegration,
) -> swallowtail_core::ConfiguredInstance {
    instance_with_capabilities(
        prepared,
        CapabilityProfile::new([CapabilityRequirement::new(
            Capability::ProviderSessionDelete,
            [],
        )]),
    )
}

async fn wrap_management_handle(
    handle: Box<dyn InteractiveSessionHandle>,
    instance: swallowtail_core::ConfiguredInstance,
    access: PreparedAccessEvidence,
    origin: ProviderSessionBindingOrigin,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    let provider_ref = handle
        .provider_session_ref()
        .cloned()
        .expect("retained-session driver guarantees provider identity");
    let binding = ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &crate::alibaba_model_studio_descriptor(),
        &instance,
        access,
        None,
        origin,
    )
    .expect("management context was validated before provider session work");
    Ok(Box::new(ManagedRetainedConversation {
        inner: handle,
        binding,
    }))
}

fn validate_management_context(
    instance: &swallowtail_core::ConfiguredInstance,
    access: &PreparedAccessEvidence,
) -> Result<(), RuntimeFailure> {
    ProviderSessionManagementBinding::validate_bound_session_context(
        &crate::alibaba_model_studio_descriptor(),
        instance,
        access,
    )
    .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))
}

struct ManagedRetainedConversation {
    inner: Box<dyn InteractiveSessionHandle>,
    binding: ProviderSessionManagementBinding,
}

impl InteractiveSessionHandle for ManagedRetainedConversation {
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
