use super::input::{
    OpenCodeCatalogueProfileInput, OpenCodeRunProfileInput, OpenCodeRunProfileParts,
    OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
use super::plan::{
    OpenCodePreparedEvidence, build_plan, failure, instance_with_capabilities,
    management_requirements, requirements, run_requirements,
};
use super::{
    OpenCodePreparedRunFuture, OpenCodePreparedSessionFuture, OpenCodePreparedSessionLoadFuture,
};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    ModelCatalogEntry, ModelRoute, PreflightPlan, ProviderId, ProviderSessionActivityEvidence,
    ProviderSessionAffectedScope, ProviderSessionBindingOrigin, ProviderSessionCancellationPosture,
    ProviderSessionDeletionStrength, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction, ReasoningMode, ResourceAccess, ResourceRepresentation,
    StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    AttachmentRole, BoxFuture, CancellationControl, CleanupOutcome, DeleteProviderSessionRequest,
    DirectContinuationTurnRequest, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, LoadSessionRequest, LoadedSession, ModelCatalogDriver,
    ModelCatalogRequest, OpenSessionRequest, OperationPolicy, PreparationFailure,
    PreparedAccessEvidence, PreparedProviderSessionManagementEvidence, ProviderRetentionPolicy,
    ProviderSessionManagementAgreement, ProviderSessionManagementBinding,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, ResumeSessionRequest, RuntimeFailure, SchemaDocument,
    SessionResumeBinding, StructuredOutputDescriptor, StructuredRunDriver, StructuredRunRequest,
    TurnHandle, TurnRequest, WorkingResourceRef,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparedRun {
    evidence: OpenCodePreparedEvidence,
    request: StructuredRunRequest,
}

impl OpenCodePreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    pub fn start_run(&self, services: HostServices) -> OpenCodePreparedRunFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OpenCodePreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
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
                ProviderSessionBindingOrigin::Created,
            )
            .await
        })
    }

    pub fn load_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        LoadSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared OpenCode session binds a working resource")
                .clone(),
            self.request.deadline(),
        )
    }

    pub fn load_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<OpenCodePreparedSessionLoadFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.load_request(request_id, binding)?;
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(async move {
            let loaded = driver.load_session(plan, request.clone(), services).await?;
            let (replay, handle) = loaded.into_parts();
            let handle = wrap_management_handle(
                handle,
                instance,
                access,
                Some(request.working_resource().clone()),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(LoadedSession::new(replay, handle))
        }))
    }

    pub fn resume_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        ResumeSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared OpenCode session binds a working resource")
                .clone(),
            self.request.deadline(),
        )
    }

    pub fn resume_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<OpenCodePreparedSessionFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.resume_request(request_id, binding)?;
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(async move {
            let handle = driver
                .resume_session(plan, request.clone(), services)
                .await?;
            wrap_management_handle(
                handle,
                instance,
                access,
                Some(request.working_resource().clone()),
                ProviderSessionBindingOrigin::Resumed,
            )
            .await
        }))
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
        self.prepare_catalogue_inner(input)
    }

    pub fn prepare_session(
        &self,
        input: OpenCodeSessionProfileInput,
    ) -> Result<OpenCodePreparedSession, PreparationFailure> {
        self.prepare_session_inner(input)
    }

    pub fn prepare_run(
        &self,
        input: OpenCodeRunProfileInput,
    ) -> Result<OpenCodePreparedRun, PreparationFailure> {
        self.prepare_run_inner(input)
    }

    pub fn prepare_delete_session(
        &self,
        input: OpenCodeSessionManagementInput,
    ) -> Result<OpenCodePreparedDelete, PreparationFailure> {
        self.prepare_delete_session_inner(input)
    }
}

include!("operations/integration.rs");
include!("operations/support.rs");
