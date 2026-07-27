use super::OpenCodePreparedSessionFuture;
use super::input::{
    OpenCodeCatalogueProfileInput, OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
use super::plan::{
    OpenCodePreparedEvidence, build_plan, failure, instance_with_capabilities,
    management_requirements, requirements,
};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, ModelCatalogEntry,
    ModelRoute, PreflightPlan, ProviderSessionActivityEvidence, ProviderSessionAffectedScope,
    ProviderSessionBindingOrigin, ProviderSessionCancellationPosture,
    ProviderSessionDeletionStrength, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, DeleteProviderSessionRequest,
    DirectContinuationTurnRequest, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest,
    PreparationFailure, PreparedAccessEvidence, PreparedProviderSessionManagementEvidence,
    ProviderSessionManagementAgreement, ProviderSessionManagementBinding,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RuntimeFailure, SessionResumeBinding, TurnHandle, TurnRequest,
    WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparedCatalogue {
    evidence: OpenCodePreparedEvidence,
    request: ModelCatalogRequest,
}

impl OpenCodePreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(self) -> (OpenCodePreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparedSession {
    evidence: OpenCodePreparedEvidence,
    request: OpenSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
}

impl OpenCodePreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    pub fn open_session(&self, services: HostServices) -> OpenCodePreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            let handle = driver.open_session(plan, request.clone(), services).await?;
            wrap_management_handle(
                handle,
                management_instance,
                access,
                request.working_resource().cloned(),
            )
            .await
        })
    }

    #[must_use]
    pub fn into_parts(self) -> (OpenCodePreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

#[derive(Clone, Debug)]
pub struct OpenCodePreparedDelete {
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl OpenCodePreparedDelete {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &DeleteProviderSessionRequest {
        &self.request
    }

    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = OpenCodeHttpDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.delete_session(plan, request, services).await })
    }
}

impl OpenCodePreparedIntegration {
    pub fn prepare_catalogue(
        &self,
        input: OpenCodeCatalogueProfileInput,
    ) -> Result<OpenCodePreparedCatalogue, PreparationFailure> {
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(
            self,
            DriverRole::ModelCatalog,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, None, &requirements)?;
        let (request_id, deadline) = input.into_parts();
        let request = match deadline {
            Some(deadline) => ModelCatalogRequest::new(request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(request_id),
        };
        Ok(OpenCodePreparedCatalogue {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }

    pub fn prepare_session(
        &self,
        input: OpenCodeSessionProfileInput,
    ) -> Result<OpenCodePreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline) = input.into_parts();
        let capabilities = crate::prepared::all_capabilities();
        let session_capabilities = CapabilityProfile::new(
            capabilities
                .iter()
                .filter(|(capability, _)| {
                    !matches!(
                        *capability,
                        Capability::ModelCatalog | Capability::ProviderSessionDelete
                    )
                })
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
        );
        let instance = instance_with_capabilities(self, session_capabilities.clone());
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            session_capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = requirements(
            self,
            DriverRole::InteractiveSession,
            session_capabilities
                .iter()
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?;
        Ok(OpenCodePreparedSession {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
            management_instance: lifecycle_management_instance(self),
        })
    }

    pub fn prepare_delete_session(
        &self,
        input: OpenCodeSessionManagementInput,
    ) -> Result<OpenCodePreparedDelete, PreparationFailure> {
        let (request_id, binding, deadline, allow_unverified_newer) = input.into_parts();
        if !self.server().is_qualified() && !allow_unverified_newer {
            return Err(failure(
                "swallowtail.opencode.preparation.lifecycle_unverified_newer",
                "Newer unverified OpenCode deletion requires explicit acceptance",
            ));
        }
        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );
        let capability = CapabilityRequirement::new(Capability::ProviderSessionDelete, []);
        let instance =
            instance_with_capabilities(self, CapabilityProfile::new([capability.clone()]));
        let requirements = management_requirements(self, [capability]);
        let preflight = build_plan(self, &instance, None, &requirements)?;
        let agreement = ProviderSessionManagementAgreement::new(
            binding,
            action,
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            deadline,
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.lifecycle_binding_mismatch",
                "OpenCode session-management binding does not match this prepared integration",
            )
        })?;
        let request = DeleteProviderSessionRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.lifecycle_request_invalid",
                "OpenCode delete request could not be prepared",
            )
        })?;
        Ok(OpenCodePreparedDelete {
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }
}

fn lifecycle_management_instance(
    prepared: &OpenCodePreparedIntegration,
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
    working_resource: Option<WorkingResourceRef>,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    let Some(provider_ref) = handle.provider_session_ref().cloned() else {
        return Ok(handle);
    };
    match ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &crate::opencode_http_descriptor(),
        &instance,
        access,
        working_resource,
        ProviderSessionBindingOrigin::Created,
    ) {
        Ok(binding) => Ok(Box::new(ManagedOpenCodeSessionHandle {
            inner: handle,
            binding,
        })),
        Err(error) => {
            let _ = handle.close().await;
            Err(RuntimeFailure::new(error.diagnostic().clone()))
        }
    }
}

struct ManagedOpenCodeSessionHandle {
    inner: Box<dyn InteractiveSessionHandle>,
    binding: ProviderSessionManagementBinding,
}

impl InteractiveSessionHandle for ManagedOpenCodeSessionHandle {
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
