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
    origin: ProviderSessionBindingOrigin,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    let Some(provider_ref) = handle.provider_session_ref().cloned() else {
        return Ok(handle);
    };
    let binding = ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &crate::opencode_http_descriptor(),
        &instance,
        access,
        working_resource,
        origin,
    )
    .expect("management context was validated before provider session work");
    Ok(Box::new(ManagedOpenCodeSessionHandle {
        inner: handle,
        binding,
    }))
}

pub(super) fn validate_management_context(
    instance: &swallowtail_core::ConfiguredInstance,
    access: &PreparedAccessEvidence,
) -> Result<(), RuntimeFailure> {
    ProviderSessionManagementBinding::validate_bound_session_context(
        &crate::opencode_http_descriptor(),
        instance,
        access,
    )
    .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))
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

    fn close(
        self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        self.inner.close(request, services)
    }
}
