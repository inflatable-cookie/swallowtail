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
/// Prepared read-only OpenCode model-catalogue request.
pub struct OpenCodePreparedCatalogue {
    evidence: OpenCodePreparedEvidence,
    request: ModelCatalogRequest,
}

impl OpenCodePreparedCatalogue {
    /// Returns the operation's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the model-catalogue request.
    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    /// Creates the stateless low-level HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    /// Dispatches the prepared model-catalogue request.
    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }

    /// Consumes the prepared request into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(self) -> (OpenCodePreparedEvidence, PreflightPlan, ModelCatalogRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared interactive OpenCode session with separate lifecycle authority.
pub struct OpenCodePreparedSession {
    evidence: OpenCodePreparedEvidence,
    request: OpenSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared OpenCode structured run ready for explicit dispatch.
pub struct OpenCodePreparedRun {
    evidence: OpenCodePreparedEvidence,
    request: StructuredRunRequest,
}

impl OpenCodePreparedRun {
    /// Returns the operation's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the structured run request.
    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Creates the stateless low-level HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    /// Starts the prepared structured run.
    pub fn start_run(&self, services: HostServices) -> OpenCodePreparedRunFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    /// Consumes the prepared run into evidence, plan, and request.
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
    /// Returns the operation's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &OpenCodePreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the open-session request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the stateless low-level HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        OpenCodeHttpDriver::new()
    }

    /// Opens a new provider session and binds its lifecycle management handle.
    pub fn open_session(&self, services: HostServices) -> OpenCodePreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            validate_management_context(&management_instance, &access)?;
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

    /// Builds an exact retained-session load request with bounded replay.
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

    /// Loads a retained session and returns replay plus a live handle.
    pub fn load_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<OpenCodePreparedSessionLoadFuture, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        Ok(self.clone().load_prepared_session(request, services))
    }

    pub(crate) fn load_prepared_session(
        self,
        request: LoadSessionRequest,
        services: HostServices,
    ) -> OpenCodePreparedSessionLoadFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            validate_management_context(&instance, &access)?;
            let loaded = driver.load_session(plan, request.clone(), services).await?;
            let (replay, handle) = loaded.into_parts();
            let handle = wrap_management_handle(
                handle,
                instance,
                access,
                Some(
                    request
                        .working_resource()
                        .expect("prepared OpenCode load binds a working resource")
                        .clone(),
                ),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(LoadedSession::new(replay, handle))
        })
    }

    /// Builds an exact retained-session resume request without replay.
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

    /// Resumes a retained session and binds its lifecycle management handle.
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
            validate_management_context(&instance, &access)?;
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

    /// Consumes the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(self) -> (OpenCodePreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

#[derive(Clone, Debug)]
/// Prepared, separately authorized deletion of one inactive provider session.
pub struct OpenCodePreparedDelete {
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl OpenCodePreparedDelete {
    /// Returns the management preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    /// Returns the immutable management plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    /// Returns the exact delete request.
    #[must_use]
    pub const fn request(&self) -> &DeleteProviderSessionRequest {
        &self.request
    }

    /// Executes the prepared inactive-session deletion.
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
    /// Validates and prepares a model-catalogue request.
    pub fn prepare_catalogue(
        &self,
        input: OpenCodeCatalogueProfileInput,
    ) -> Result<OpenCodePreparedCatalogue, PreparationFailure> {
        self.prepare_catalogue_inner(input)
    }

    /// Validates and prepares an interactive session.
    pub fn prepare_session(
        &self,
        input: OpenCodeSessionProfileInput,
    ) -> Result<OpenCodePreparedSession, PreparationFailure> {
        self.prepare_session_inner(input)
    }

    /// Validates and prepares a structured run.
    pub fn prepare_run(
        &self,
        input: OpenCodeRunProfileInput,
    ) -> Result<OpenCodePreparedRun, PreparationFailure> {
        self.prepare_run_inner(input)
    }

    /// Validates and prepares separate inactive-session deletion authority.
    pub fn prepare_delete_session(
        &self,
        input: OpenCodeSessionManagementInput,
    ) -> Result<OpenCodePreparedDelete, PreparationFailure> {
        self.prepare_delete_session_inner(input)
    }
}

include!("operations/integration.rs");
include!("operations/support.rs");
