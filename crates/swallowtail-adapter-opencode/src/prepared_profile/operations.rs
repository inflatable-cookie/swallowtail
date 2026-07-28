use super::input::{
    OpenCodeCatalogueProfileInput, OpenCodeRunProfileInput, OpenCodeRunProfileParts,
    OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
use super::plan::{
    OpenCodePreparedEvidence, build_plan, failure, instance_with_capabilities,
    management_requirements, requirements, run_requirements,
};
use super::{OpenCodePreparedRunFuture, OpenCodePreparedSessionFuture};
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
    InteractiveSessionHandle, ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest,
    OperationPolicy, PreparationFailure, PreparedAccessEvidence,
    PreparedProviderSessionManagementEvidence, ProviderRetentionPolicy,
    ProviderSessionManagementAgreement, ProviderSessionManagementBinding,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RuntimeFailure, SchemaDocument, SessionResumeBinding,
    StructuredOutputDescriptor, StructuredRunDriver, StructuredRunRequest, TurnHandle, TurnRequest,
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
            false,
            false,
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
        let (request_id, model, working_resource, deadline, image_attachments, provider_callbacks) =
            input.into_parts();
        let capabilities = crate::prepared::all_capabilities();
        let session_capabilities = callback_resource_access(
            CapabilityProfile::new(
                capabilities
                    .iter()
                    .filter(|(capability, _)| {
                        !matches!(
                            *capability,
                            Capability::ModelCatalog | Capability::ProviderSessionDelete
                        ) && (image_attachments || *capability != Capability::Attachments)
                    })
                    .map(|(capability, constraints)| {
                        CapabilityRequirement::new(capability, constraints.iter().cloned())
                    }),
            ),
            provider_callbacks,
        );
        let instance = instance_with_capabilities(self, session_capabilities.clone());
        let (route_id, route_revision, provider_id, model_id, _) = model.into_parts();
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
            image_attachments,
            provider_callbacks,
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?;
        Ok(OpenCodePreparedSession {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
            management_instance: lifecycle_management_instance(self),
        })
    }

    pub fn prepare_run(
        &self,
        input: OpenCodeRunProfileInput,
    ) -> Result<OpenCodePreparedRun, PreparationFailure> {
        let OpenCodeRunProfileParts {
            request_id,
            model,
            content,
            working_resource,
            reasoning,
            structured_output,
            deadline,
            attachments,
            provider_callbacks,
        } = input.into_parts();
        validate_attachments(&attachments)?;
        let image_attachments = !attachments.is_empty();
        let (route_id, route_revision, provider_id, model_id, catalogue_entry) = model.into_parts();
        validate_generation_controls(
            &provider_id,
            &model_id,
            catalogue_entry.as_ref(),
            reasoning.as_ref(),
            structured_output.as_ref(),
        )?;
        let capabilities = callback_resource_access(
            run_capabilities(
                reasoning.as_ref(),
                structured_output.as_ref(),
                image_attachments,
            ),
            provider_callbacks,
        );
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = run_requirements(
            self,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            image_attachments,
            provider_callbacks,
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::TemporaryAllowed)
            .with_harness_isolation(swallowtail_core::HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(
                swallowtail_core::HarnessConfigurationPosture::Ambient,
            );
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_attachments(attachments);
        if let Some(output) = structured_output {
            request = request.with_structured_output(output);
        }
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(OpenCodePreparedRun {
            evidence: OpenCodePreparedEvidence::from_prepared(self, plan)?,
            request,
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

fn run_capabilities(
    reasoning: Option<&ReasoningMode>,
    structured_output: Option<&StructuredOutputDescriptor>,
    image_attachments: bool,
) -> CapabilityProfile {
    let mut capabilities = crate::prepared::run_capabilities()
        .iter()
        .filter(|(capability, _)| image_attachments || *capability != Capability::Attachments)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(reasoning) = reasoning {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(reasoning.clone())],
        ));
    }
    if let Some(output) = structured_output {
        capabilities.push(CapabilityRequirement::new(
            Capability::StructuredOutput,
            [
                CapabilityConstraint::SchemaDialect(output.dialect().to_owned()),
                CapabilityConstraint::StructuredOutputEnforcement(
                    StructuredOutputEnforcement::HarnessValidated,
                ),
            ],
        ));
    }
    CapabilityProfile::new(capabilities)
}

fn validate_attachments(
    attachments: &[swallowtail_runtime::AttachmentDescriptor],
) -> Result<(), PreparationFailure> {
    if attachments.len() > 1
        || attachments.iter().any(|attachment| {
            attachment.media_type() != "image/png"
                || attachment.role() != AttachmentRole::Input
                || attachment
                    .known_length()
                    .is_some_and(|length| length > 1024 * 1024)
        })
    {
        return Err(failure(
            "swallowtail.opencode.preparation.attachments_unsupported",
            "OpenCode supports one PNG attachment up to one MiB",
        ));
    }
    Ok(())
}

fn validate_generation_controls(
    provider_id: &ProviderId,
    model_id: &swallowtail_core::ModelId,
    catalogue_entry: Option<&ModelCatalogEntry>,
    reasoning: Option<&ReasoningMode>,
    structured_output: Option<&StructuredOutputDescriptor>,
) -> Result<(), PreparationFailure> {
    if reasoning.is_none() && structured_output.is_none() {
        return Ok(());
    }
    let entry = catalogue_entry.ok_or_else(|| {
        failure(
            "swallowtail.opencode.preparation.catalogue_evidence_missing",
            "OpenCode generation controls require exact selected-model catalogue evidence",
        )
    })?;
    if entry.provider_id() != Some(provider_id) || entry.id() != model_id {
        return Err(failure(
            "swallowtail.opencode.preparation.catalogue_evidence_mismatch",
            "OpenCode generation-control catalogue evidence does not match the selected model",
        ));
    }
    if let Some(reasoning) = reasoning
        && entry
            .metadata()
            .reasoning()
            .is_none_or(|metadata| !metadata.supports(reasoning))
    {
        return Err(failure(
            "swallowtail.opencode.preparation.reasoning_unsupported",
            "The selected OpenCode model does not expose the exact requested reasoning variant",
        ));
    }
    if let Some(output) = structured_output {
        let tool_calling = entry
            .metadata()
            .catalog_observations()
            .and_then(|observations| observations.tool_calling_supported());
        let valid_document = match output.document() {
            SchemaDocument::Inline(bytes) => serde_json::from_slice::<serde_json::Value>(bytes)
                .is_ok_and(|schema| schema.is_object()),
            SchemaDocument::Reference(_) => false,
        };
        if tool_calling != Some(true)
            || output.media_type() != "application/schema+json"
            || output.dialect() != "json-schema-2020-12"
            || !valid_document
        {
            return Err(failure(
                "swallowtail.opencode.preparation.schema_unsupported",
                "OpenCode structured output requires tool-capable model evidence and one inline JSON Schema 2020-12 object",
            ));
        }
    }
    Ok(())
}

fn callback_resource_access(
    capabilities: CapabilityProfile,
    provider_callbacks: bool,
) -> CapabilityProfile {
    if !provider_callbacks {
        return capabilities;
    }
    CapabilityProfile::new(capabilities.iter().map(|(capability, constraints)| {
        if capability == Capability::WorkingResource {
            CapabilityRequirement::new(
                Capability::WorkingResource,
                [
                    CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                    CapabilityConstraint::ResourceRepresentation(
                        ResourceRepresentation::Filesystem,
                    ),
                ],
            )
        } else {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }
    }))
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
