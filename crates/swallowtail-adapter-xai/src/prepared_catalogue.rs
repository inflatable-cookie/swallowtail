use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, AccessRequirement, Capability, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState, Diagnostic,
    DriverRole, EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId,
    ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ProtocolFacadeId, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, PreparedOperationEvidence, RequestId, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit host, endpoint, and API-key evidence for xAI Models preparation.
pub struct XaiModelsPreparationInput {
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl XaiModelsPreparationInput {
    #[must_use]
    /// Creates Models preparation input without provider work.
    pub const fn new(
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared read-only xAI Models integration.
pub struct XaiModelsPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer input for an xAI Models catalogue observation.
pub struct XaiModelsProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl XaiModelsProfileInput {
    #[must_use]
    /// Creates catalogue input without a deadline.
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds an exact host-monotonic deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared xAI Models catalogue operation.
pub struct XaiPreparedModels {
    evidence: PreparedOperationEvidence,
    request: ModelCatalogRequest,
}

impl XaiModelsPreparedIntegration {
    #[must_use]
    /// Returns the prepared configured instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    #[must_use]
    /// Returns the exact public API-key access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns the access evidence and provenance.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Iterates the host services present during preparation.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Prepares a bound read-only model-catalogue observation.
    pub fn prepare_catalogue(
        &self,
        input: XaiModelsProfileInput,
    ) -> Result<XaiPreparedModels, PreparationFailure> {
        let descriptor = crate::xai_models_descriptor();
        let requirements = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            DriverRole::ModelCatalog,
            self.instance.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::ProviderSupported]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(descriptor.required_host_services(DriverRole::ModelCatalog))
        .with_capabilities([CapabilityRequirement::new(Capability::ModelCatalog, [])])
        .with_interface_versions([crate::xai_models_facade_binding()]);
        let plan = preflight(
            &PreflightContext::new(
                &descriptor,
                &self.instance,
                &self.access_profile,
                self.access_evidence.status(),
                self.available_host_services(),
            ),
            &requirements,
        )
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request = match input.deadline {
            Some(deadline) => ModelCatalogRequest::new(input.request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(input.request_id),
        };
        Ok(XaiPreparedModels {
            evidence: PreparedOperationEvidence::from_plan(plan, self.access_evidence.clone())?,
            request,
        })
    }
}

impl XaiPreparedModels {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable catalogue plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived catalogue request.
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    /// Executes the bound model-catalogue observation.
    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        let driver = crate::XaiModelsDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }
}

/// Prepares the xAI Models integration without provider effects.
pub fn prepare_xai_models(
    input: XaiModelsPreparationInput,
    services: &HostServices,
) -> Result<XaiModelsPreparedIntegration, PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id
        || input.endpoint_target.as_host_value() != crate::XAI_MODELS_ENDPOINT
    {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.xai.models.preparation.target_rejected",
            "xAI Models preparation requires the exact public Models API origin",
        ));
    }
    if input.access_profile.id().as_str() != crate::XAI_MODELS_ACCESS_PROFILE_ID
        || input.access_profile.credential_mechanism() != &CredentialMechanism::ApiKey
        || input.access_profile.credential_reference().is_none()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::PayAsYouGo
        || input.access_profile.support_authority() != SupportAuthority::ProviderSupported
        || input.access_profile.endpoint_audience().as_str() != crate::XAI_MODELS_ENDPOINT_AUDIENCE
        || input.access_evidence.status().profile_id() != input.access_profile.id()
    {
        return Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.xai.models.preparation.access_rejected",
            "xAI Models preparation requires the provider-supported public API-key profile",
        ));
    }
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new(crate::XAI_MODELS_CONFIGURED_INSTANCE_ID)
            .expect("static instance id is valid"),
        input.instance_revision,
        crate::xai_models_descriptor().identity().id().clone(),
        input.execution_host_id,
        input.endpoint_target,
        InstanceOwnership::ExternalAttached,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(crate::XAI_MODELS_FACADE_REVISION)
            .expect("static facade id is valid"),
        InstancePolicyId::new("xai-public-language-models-read-only")
            .expect("static policy id is valid"),
        CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]),
    )
    .with_interface_versions([crate::xai_models_facade_binding()]);
    Ok(XaiModelsPreparedIntegration {
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services: services.available_kinds(),
    })
}

fn preparation_failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
