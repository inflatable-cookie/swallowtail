#![deny(missing_docs)]

//! Prepared evidence and route-local operation facades for DeepSeek Harness
//! Web `/api`.
//!
//! This module intentionally does not share the JSON-RPC prepared types. The
//! Web route has a different driver identity, protocol facade, loopback
//! endpoint, method allowlist, and provider-session surface.

use crate::web::{
    DeepSeekHarnessWebDriver, DeepSeekHarnessWebModel, method_allowlist, require_loopback_endpoint,
};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    InstalledExecutableCompatibility, InstalledExecutableObservation, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, ObservableActivityProfile, OperationShape, ProtocolFacadeId, ProviderId,
    ProviderSessionActivityEvidence, ProviderSessionAffectedScope, ProviderSessionBindingOrigin,
    ProviderSessionCancellationPosture, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority,
};
use swallowtail_core::{ProviderSessionCatalogueBounds, SessionRef};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, Deadline, DiscoveryCancellation, DiscoveryDriver,
    EnvironmentRef, HostServices, InstalledExecutableDiscoveryRequest, InstalledExecutableTarget,
    OperationContent, PreparationFailure, PreparationStage, PreparedAccessEvidence,
    PreparedOperationEvidence, ProviderSessionCatalogueAgreement, ProviderSessionCatalogueDriver,
    ProviderSessionCatalogueId, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope,
    ProviderSessionHistoryAgreement, ProviderSessionHistoryBounds, ProviderSessionHistoryCursor,
    ProviderSessionHistoryDriver, ProviderSessionHistoryId, ProviderSessionHistoryPage,
    ProviderSessionHistoryPlan, ProviderSessionHistoryRequest, ProviderSessionManagementBinding,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, ProviderSessionOperationFailure, RequestId, RunHandle,
    RuntimeFailure, SessionResumeBinding, StructuredRunDriver, StructuredRunRequest,
    WorkingResourceRef, base_requirements, build_plan, instance_with_capabilities,
};
use swallowtail_runtime::{
    PreparedProviderSessionCatalogueEvidence, PreparedProviderSessionHistoryEvidence,
    PreparedProviderSessionManagementEvidence,
};

const WEB_PROTOCOL_FACADE_ID: &str = "deepseek-harness.apiproxy-v1";
const WEB_POLICY_ID: &str = "deepseek-harness-web-prepared-read-only";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for preparing one exact DeepSeek Harness Web runtime-bin.
pub struct DeepSeekHarnessWebPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    endpoint: String,
}

impl DeepSeekHarnessWebPreparationInput {
    #[must_use]
    /// Creates Web preparation input with the fixed local-server default.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        target: InstalledExecutableTarget,
        environment: EnvironmentRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            target,
            environment,
            access_profile,
            access_evidence,
            endpoint: String::new(),
        }
    }

    #[must_use]
    /// Replaces the endpoint while retaining preparation-time loopback checks.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }
}

#[derive(Clone, Debug)]
/// Bounded discovery inputs for Web preparation.
pub struct DeepSeekHarnessWebPreparationProbe {
    request_id: RequestId,
    scope_id: swallowtail_runtime::ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl DeepSeekHarnessWebPreparationProbe {
    #[must_use]
    /// Creates a cancellable, deadline-bound Web discovery probe.
    pub const fn new(
        request_id: RequestId,
        scope_id: swallowtail_runtime::ScopeId,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            request_id,
            scope_id,
            deadline,
            cancellation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Qualified Web integration and immutable host-approved route evidence.
pub struct DeepSeekHarnessWebPreparedIntegration {
    environment: EnvironmentRef,
    endpoint: String,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl DeepSeekHarnessWebPreparedIntegration {
    #[must_use]
    /// Returns the host-approved Cordis configuration reference.
    pub fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    #[must_use]
    /// Returns the exact loopback HTTP endpoint admitted for this route.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    #[must_use]
    /// Returns the exact host-approved `dsh` executable target.
    pub fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    #[must_use]
    /// Returns the exact observed Web release binding.
    pub fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    /// Returns the host-owned local-unauthenticated access profile.
    pub fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    /// Returns immutable access evidence admitted during preparation.
    pub fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    /// Returns the configured Web route instance.
    pub fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    #[must_use]
    /// Returns the Web compatibility axis.
    pub const fn release_axis(&self) -> &'static str {
        crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_AXIS
    }

    #[must_use]
    /// Returns the exact npm release admitted by this route.
    pub const fn release_version(&self) -> &'static str {
        crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_VERSION
    }

    #[must_use]
    /// Returns the exact executable basename admitted by this route.
    pub const fn executable_basename(&self) -> &'static str {
        "dsh"
    }

    #[must_use]
    /// Returns the protocol facade identity for the Web API route.
    pub const fn protocol_facade_id(&self) -> &'static str {
        WEB_PROTOCOL_FACADE_ID
    }

    #[must_use]
    /// Returns the exact allowlisted Web method names.
    pub fn allowlisted_methods(&self) -> &'static [&'static str] {
        method_allowlist()
    }

    /// Iterates over host services present when preparation succeeded.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    /// Creates the Web driver bound to the prepared endpoint and environment.
    pub fn low_level_driver(&self) -> DeepSeekHarnessWebDriver {
        DeepSeekHarnessWebDriver::new(self.environment.clone())
            .with_endpoint(self.endpoint.clone())
            .expect("prepared Web endpoint already passed loopback validation")
    }

    /// Creates management authority for one exact provider session.
    pub fn management_binding(
        &self,
        provider_session_ref: SessionRef,
        working_resource: Option<WorkingResourceRef>,
        origin: ProviderSessionBindingOrigin,
    ) -> Result<ProviderSessionManagementBinding, PreparationFailure> {
        ProviderSessionManagementBinding::from_bound_session(
            provider_session_ref,
            &crate::web::deepseek_harness_web_descriptor(),
            &self.instance,
            self.access_evidence.clone(),
            working_resource,
            origin,
        )
        .map_err(|error| {
            preparation_failure(
                "management binding is not valid for Web route",
                error.diagnostic(),
            )
        })
    }

    /// Validates and prepares one structured Web prompt operation.
    pub fn prepare_run(
        &self,
        input: DeepSeekHarnessWebRunProfileInput,
    ) -> Result<DeepSeekHarnessWebPreparedRun, PreparationFailure> {
        let activity = web_activity_profile(&self.observation)?;
        let capabilities = with_activity(run_capabilities(), &activity);
        let instance = web_instance_with_capabilities(&self.instance, capabilities.clone());
        let route = ModelRoute::new(
            input.model.route_id,
            input.model.route_revision,
            instance.id().clone(),
            input.model.model_id,
            capabilities.clone(),
        )
        .with_provider_id(input.model.provider_id);
        let descriptor = crate::web::deepseek_harness_web_descriptor();
        let requirements = base_requirements(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            &instance,
            &self.access_profile,
            [CredentialState::NotRequired],
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        )
        .with_host_services(descriptor.required_host_services(DriverRole::StructuredRun))
        .with_interface_versions([self.observation.version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let plan = build_plan(
            &descriptor,
            &instance,
            Some(&route),
            &requirements,
            &self.access_profile,
            self.access_evidence.status(),
            self.available_host_services(),
        )?;
        let request =
            StructuredRunRequest::new(input.request_id, input.content, web_operation_policy())
                .with_working_resource(input.working_resource)
                .with_deadline(input.deadline);
        Ok(DeepSeekHarnessWebPreparedRun {
            evidence: DeepSeekHarnessWebPreparedEvidence::from_plan(self, plan, activity)?,
            request,
        })
    }

    /// Prepares a bounded working-resource-scoped provider-session catalogue.
    pub fn prepare_session_catalogue(
        &self,
        input: DeepSeekHarnessWebSessionCatalogueInput,
    ) -> Result<DeepSeekHarnessWebPreparedSessionCatalogue, PreparationFailure> {
        let descriptor = crate::web::deepseek_harness_web_descriptor();
        let catalogue = CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []);
        let resource = read_only_working_resource_capability();
        let capabilities = CapabilityProfile::new([catalogue.clone(), resource.clone()]);
        let instance = web_instance_with_capabilities(&self.instance, capabilities);
        let requirements = base_requirements(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionCatalogue,
            DriverRole::ProviderSessionCatalogue,
            &instance,
            &self.access_profile,
            [CredentialState::NotRequired],
            [catalogue, resource],
        )
        .with_host_services(descriptor.required_host_services(DriverRole::ProviderSessionCatalogue))
        .with_interface_versions([self.observation.version().clone()])
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let plan = build_plan(
            &descriptor,
            &instance,
            None,
            &requirements,
            &self.access_profile,
            self.access_evidence.status(),
            self.available_host_services(),
        )?;
        let plan = ProviderSessionCataloguePlan::new(
            plan,
            ProviderSessionCatalogueAgreement::new(
                input.catalogue_id,
                ProviderSessionCatalogueScope::working_resource(input.working_resource),
                input.bounds,
                input.deadline,
            ),
        )
        .map_err(|error| preparation_failure("catalogue plan is invalid", error.diagnostic()))?;
        let request = ProviderSessionCatalogueRequest::from_plan(input.request_id, &plan, None)
            .map_err(|error| {
                preparation_failure("catalogue request is invalid", error.diagnostic())
            })?;
        let evidence = PreparedProviderSessionCatalogueEvidence::from_plan(
            plan,
            self.access_evidence.clone(),
        )?;
        Ok(DeepSeekHarnessWebPreparedSessionCatalogue {
            integration: self.clone(),
            evidence,
            request,
        })
    }

    /// Prepares control-free newest-first history for one bound Web session.
    pub fn prepare_session_history(
        &self,
        input: DeepSeekHarnessWebSessionHistoryInput,
    ) -> Result<DeepSeekHarnessWebPreparedSessionHistory, PreparationFailure> {
        if input.binding.model_route_id() != Some(&input.model.route_id)
            || input.binding.model_id() != Some(&input.model.model_id)
        {
            return Err(preparation_failure_message(
                "history model does not match its durable Web session binding",
            ));
        }
        let history = CapabilityRequirement::new(
            Capability::ProviderSessionHistory,
            [
                CapabilityConstraint::ReplayMaximumItems(input.bounds.maximum_page_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(input.bounds.maximum_page_bytes().get()),
            ],
        );
        let resource = read_only_working_resource_capability();
        let durable = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let capabilities = CapabilityProfile::new([history.clone(), resource.clone(), durable]);
        let instance = web_instance_with_capabilities(&self.instance, capabilities.clone());
        let route = ModelRoute::new(
            input.model.route_id.clone(),
            input.model.route_revision.clone(),
            instance.id().clone(),
            input.model.model_id.clone(),
            capabilities.clone(),
        )
        .with_provider_id(input.model.provider_id.clone());
        let descriptor = crate::web::deepseek_harness_web_descriptor();
        let requirements = base_requirements(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionHistory,
            DriverRole::ProviderSessionHistory,
            &instance,
            &self.access_profile,
            [CredentialState::NotRequired],
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        )
        .with_host_services(descriptor.required_host_services(DriverRole::ProviderSessionHistory))
        .with_interface_versions([self.observation.version().clone()])
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        )
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let plan = build_plan(
            &descriptor,
            &instance,
            Some(&route),
            &requirements,
            &self.access_profile,
            self.access_evidence.status(),
            self.available_host_services(),
        )?;
        let plan = ProviderSessionHistoryPlan::new(
            plan,
            ProviderSessionHistoryAgreement::new(
                input.history_id,
                input.binding,
                input.bounds,
                input.deadline,
            ),
        )
        .map_err(|error| preparation_failure("history plan is invalid", error.diagnostic()))?;
        let request = ProviderSessionHistoryRequest::from_plan(input.request_id, &plan, None)
            .map_err(|error| {
                preparation_failure("history request is invalid", error.diagnostic())
            })?;
        let evidence =
            PreparedProviderSessionHistoryEvidence::from_plan(plan, self.access_evidence.clone())?;
        Ok(DeepSeekHarnessWebPreparedSessionHistory {
            integration: self.clone(),
            evidence,
            request,
        })
    }

    /// Prepares target-only archival for one inactive Web session.
    pub fn prepare_archive_session(
        &self,
        input: DeepSeekHarnessWebSessionManagementInput,
    ) -> Result<DeepSeekHarnessWebPreparedArchive, PreparationFailure> {
        let capability = CapabilityRequirement::new(Capability::ProviderSessionArchive, []);
        let instance = web_instance_with_capabilities(
            &self.instance,
            CapabilityProfile::new([capability.clone()]),
        );
        let descriptor = crate::web::deepseek_harness_web_descriptor();
        let requirements = base_requirements(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionManagement,
            DriverRole::ProviderSessionManagement,
            &instance,
            &self.access_profile,
            [CredentialState::NotRequired],
            [capability],
        )
        .with_host_services(
            descriptor.required_host_services(DriverRole::ProviderSessionManagement),
        )
        .with_interface_versions([self.observation.version().clone()])
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let preflight = build_plan(
            &descriptor,
            &instance,
            None,
            &requirements,
            &self.access_profile,
            self.access_evidence.status(),
            self.available_host_services(),
        )?;
        let agreement = swallowtail_runtime::ProviderSessionManagementAgreement::new(
            input.binding,
            ProviderSessionManagementAction::Archive,
            ProviderSessionInitialStateRequirement::Unarchived,
            ProviderSessionAffectedScope::TargetOnly,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            input.deadline,
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement)
            .map_err(|error| preparation_failure("archive plan is invalid", error.diagnostic()))?;
        let request =
            ArchiveProviderSessionRequest::from_plan(input.request_id, &plan).map_err(|error| {
                preparation_failure("archive request is invalid", error.diagnostic())
            })?;
        let evidence = PreparedProviderSessionManagementEvidence::from_plan(plan)?;
        Ok(DeepSeekHarnessWebPreparedArchive {
            integration: self.clone(),
            evidence,
            request,
        })
    }
}

/// Discovers and prepares one exact Web route without minting credentials.
pub async fn prepare_deepseek_harness_web(
    input: DeepSeekHarnessWebPreparationInput,
    probe: DeepSeekHarnessWebPreparationProbe,
    services: HostServices,
) -> Result<DeepSeekHarnessWebPreparedIntegration, PreparationFailure> {
    validate_input(&input)?;
    let available_host_services = services.available_kinds();
    let request = InstalledExecutableDiscoveryRequest::new(
        probe.request_id,
        probe.scope_id,
        input.execution_host_id.clone(),
        input.target.clone(),
        probe.deadline,
        probe.cancellation,
    );
    let driver = DeepSeekHarnessWebDriver::new(input.environment.clone());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(|error| {
            preparation_failure("Web executable discovery failed", error.diagnostic())
        })?;
    promote(input, outcome, available_host_services)
}

fn validate_input(input: &DeepSeekHarnessWebPreparationInput) -> Result<(), PreparationFailure> {
    if input.target.version_axis().as_str() != crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_AXIS
        || !crate::web::target_is_exact(input.target.executable().as_host_value())
    {
        return Err(preparation_failure_message(
            "Web preparation requires the exact dsh target and Web version axis",
        ));
    }
    if input.environment.as_host_value().trim().is_empty() {
        return Err(preparation_failure_message(
            "Web preparation requires a host-approved Cordis configuration",
        ));
    }
    if input.endpoint.is_empty() {
        // Empty means the documented local-server default; keeping this
        // normalization in preparation avoids a hidden endpoint at execute.
    } else if let Err(error) = require_loopback_endpoint(&input.endpoint) {
        return Err(preparation_failure(
            "Web endpoint is not an allowed loopback bind",
            error.diagnostic(),
        ));
    }
    let profile = &input.access_profile;
    let status = input.access_evidence.status();
    if profile.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || profile.credential_reference().is_some()
        || profile.entitlement_metering() != &EntitlementMetering::SubscriptionAllowance
        || profile.endpoint_audience().as_str() != crate::DEEPSEEK_HARNESS_CONFIG_AUDIENCE
        || profile.support_authority() != SupportAuthority::ProviderSupported
        || status.profile_id() != profile.id()
        || status.credential() != CredentialState::NotRequired
        || status.entitlement() != EntitlementState::Available
        || status.endpoint_authorization() != EndpointAuthorization::Allowed
        || status.runtime_readiness() != RuntimeReadiness::Ready
        || status.support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(preparation_failure_message(
            "Web preparation requires provider-supported local unauthenticated access evidence",
        ));
    }
    Ok(())
}

fn promote(
    input: DeepSeekHarnessWebPreparationInput,
    outcome: swallowtail_core::DiscoveryOutcome,
    available_host_services: BTreeSet<HostServiceKind>,
) -> Result<DeepSeekHarnessWebPreparedIntegration, PreparationFailure> {
    let observation = outcome
        .installed_executable_observation()
        .filter(|observation| observation.is_permitted())
        .cloned()
        .ok_or_else(|| {
            preparation_failure_message("Web discovery did not produce a qualified executable")
        })?;
    if observation.execution_host_id() != &input.execution_host_id
        || observation.version().axis() != input.target.version_axis()
        || observation.version().version().as_str()
            != crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_VERSION
    {
        return Err(preparation_failure_message(
            "Web discovery does not match the prepared host and release",
        ));
    }
    let endpoint = if input.endpoint.is_empty() {
        "http://127.0.0.1:3080".to_owned()
    } else {
        input.endpoint.clone()
    };
    let instance = configured_instance(&input, &observation)?;
    Ok(DeepSeekHarnessWebPreparedIntegration {
        environment: input.environment,
        endpoint,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services,
    })
}

fn configured_instance(
    input: &DeepSeekHarnessWebPreparationInput,
    observation: &InstalledExecutableObservation,
) -> Result<ConfiguredInstance, PreparationFailure> {
    let target = InstanceTargetRef::new(input.target.executable().as_host_value())
        .map_err(|_| preparation_failure_message("Web target cannot be bound to an instance"))?;
    Ok(ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision.clone(),
        crate::web::deepseek_harness_web_descriptor()
            .identity()
            .id()
            .clone(),
        input.execution_host_id.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(WEB_PROTOCOL_FACADE_ID).expect("static Web protocol facade is valid"),
        InstancePolicyId::new(WEB_POLICY_ID).expect("static Web policy is valid"),
        advertised_capabilities(),
    )
    .with_interface_versions([observation.version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient))
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit provider, model, and route selection for one Web prompt.
pub struct DeepSeekHarnessWebModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl DeepSeekHarnessWebModelSelection {
    #[must_use]
    /// Creates an explicit provider and model route selection.
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider_id: ProviderId,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            provider_id,
            model_id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prompt, model, working resource, and deadline for one Web run.
pub struct DeepSeekHarnessWebRunProfileInput {
    request_id: RequestId,
    model: DeepSeekHarnessWebModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
}

impl DeepSeekHarnessWebRunProfileInput {
    #[must_use]
    /// Creates an explicit structured Web prompt profile.
    pub const fn new(
        request_id: RequestId,
        model: DeepSeekHarnessWebModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable evidence for one prepared Web structured run.
pub struct DeepSeekHarnessWebPreparedEvidence {
    observation: InstalledExecutableObservation,
    environment: EnvironmentRef,
    endpoint: String,
    operation: PreparedOperationEvidence,
}

impl DeepSeekHarnessWebPreparedEvidence {
    fn from_plan(
        integration: &DeepSeekHarnessWebPreparedIntegration,
        plan: swallowtail_core::PreflightPlan,
        activity: ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: integration.observation.clone(),
            environment: integration.environment.clone(),
            endpoint: integration.endpoint.clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                integration.access_evidence.clone(),
                activity,
            )?,
        })
    }

    #[must_use]
    /// Returns the exact Web executable observation.
    pub fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    /// Returns the prepared access evidence.
    pub fn access(&self) -> &PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    /// Returns the complete prepared-operation evidence.
    pub fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the Web endpoint bound to this prepared operation.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    #[must_use]
    /// Returns the admitted activity profile.
    pub fn observable_activity(&self) -> &ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the validated preflight plan.
    pub fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    /// Creates the low-level Web driver bound to this operation.
    pub fn low_level_driver(&self) -> DeepSeekHarnessWebDriver {
        DeepSeekHarnessWebDriver::new(self.environment.clone())
            .with_endpoint(self.endpoint.clone())
            .expect("prepared Web endpoint already passed loopback validation")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot structured Web prompt.
pub struct DeepSeekHarnessWebPreparedRun {
    evidence: DeepSeekHarnessWebPreparedEvidence,
    request: StructuredRunRequest,
}

impl DeepSeekHarnessWebPreparedRun {
    #[must_use]
    /// Returns immutable Web operation evidence.
    pub fn evidence(&self) -> &DeepSeekHarnessWebPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the validated Web preflight plan.
    pub fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the bound structured-run request.
    pub fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Creates the low-level Web driver bound to this run.
    pub fn low_level_driver(&self) -> DeepSeekHarnessWebDriver {
        self.evidence.low_level_driver()
    }

    /// Starts the prepared Web run with caller-supplied host services.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for one bounded Web provider-session catalogue.
pub struct DeepSeekHarnessWebSessionCatalogueInput {
    request_id: RequestId,
    catalogue_id: ProviderSessionCatalogueId,
    working_resource: WorkingResourceRef,
    bounds: ProviderSessionCatalogueBounds,
    deadline: Option<Deadline>,
}

impl DeepSeekHarnessWebSessionCatalogueInput {
    #[must_use]
    /// Creates a Web catalogue bounded to one working resource.
    pub const fn new(
        request_id: RequestId,
        catalogue_id: ProviderSessionCatalogueId,
        working_resource: WorkingResourceRef,
        bounds: ProviderSessionCatalogueBounds,
    ) -> Self {
        Self {
            request_id,
            catalogue_id,
            working_resource,
            bounds,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds a deadline to catalogue traversal.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug)]
/// Prepared Web provider-session catalogue and native search/model helpers.
pub struct DeepSeekHarnessWebPreparedSessionCatalogue {
    integration: DeepSeekHarnessWebPreparedIntegration,
    evidence: PreparedProviderSessionCatalogueEvidence,
    request: ProviderSessionCatalogueRequest,
}

impl DeepSeekHarnessWebPreparedSessionCatalogue {
    #[must_use]
    /// Returns portable catalogue evidence.
    pub fn evidence(&self) -> &PreparedProviderSessionCatalogueEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the exact catalogue plan.
    pub fn plan(&self) -> &ProviderSessionCataloguePlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the initial catalogue request.
    pub fn request(&self) -> &ProviderSessionCatalogueRequest {
        &self.request
    }

    #[must_use]
    /// Returns the prepared route evidence used by this catalogue.
    pub fn integration(&self) -> &DeepSeekHarnessWebPreparedIntegration {
        &self.integration
    }

    #[must_use]
    /// Creates the low-level Web driver bound to this catalogue.
    pub fn low_level_driver(&self) -> DeepSeekHarnessWebDriver {
        self.integration.low_level_driver()
    }

    /// Lists the initial bounded page of provider sessions.
    pub fn list_sessions(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        self.list_page(self.request.clone(), services)
    }

    /// Lists one explicitly supplied catalogue page.
    pub fn list_page(
        &self,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        Box::pin(async move { driver.list_provider_sessions(plan, request, services).await })
    }

    /// Builds a continuation request from the provider cursor.
    pub fn next_page_request(
        &self,
        request_id: RequestId,
        cursor: swallowtail_runtime::ProviderSessionCursor,
    ) -> Result<ProviderSessionCatalogueRequest, PreparationFailure> {
        ProviderSessionCatalogueRequest::from_plan(request_id, self.plan(), Some(cursor)).map_err(
            |error| preparation_failure("catalogue continuation is invalid", error.diagnostic()),
        )
    }

    /// Searches provider-owned sessions through the prepared Web route.
    pub fn search_sessions(
        &self,
        query: &str,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<(SessionRef, String)>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().preflight().clone();
        let request_id = self.request.request_id().clone();
        let deadline = self.plan().agreement().deadline();
        let query = query.to_owned();
        Box::pin(async move {
            driver
                .search_sessions(&plan, &request_id, &query, &services, deadline)
                .await
        })
    }

    /// Lists models available to one provider-owned session.
    pub fn list_models(
        &self,
        session: SessionRef,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<DeepSeekHarnessWebModel>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().preflight().clone();
        let request_id = self.request.request_id().clone();
        let deadline = self.plan().agreement().deadline();
        Box::pin(async move {
            driver
                .list_session_models(&plan, &request_id, &session, &services, deadline)
                .await
        })
    }

    /// Prepares native Web fork authority without issuing resume authority.
    pub fn prepare_fork(
        &self,
        input: DeepSeekHarnessWebForkInput,
    ) -> DeepSeekHarnessWebPreparedFork {
        DeepSeekHarnessWebPreparedFork {
            catalogue: self.clone(),
            input,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for the native Web session-fork method.
pub struct DeepSeekHarnessWebForkInput {
    request_id: RequestId,
    session: SessionRef,
    at_sequence: Option<u64>,
    deadline: Option<Deadline>,
}

impl DeepSeekHarnessWebForkInput {
    #[must_use]
    /// Creates a native fork request for one provider session.
    pub const fn new(request_id: RequestId, session: SessionRef) -> Self {
        Self {
            request_id,
            session,
            at_sequence: None,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds an optional provider event sequence at which to fork.
    pub const fn at_sequence(mut self, sequence: u64) -> Self {
        self.at_sequence = Some(sequence);
        self
    }

    #[must_use]
    /// Adds a deadline to the native fork request.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug)]
/// Prepared route-local native Web fork operation.
pub struct DeepSeekHarnessWebPreparedFork {
    catalogue: DeepSeekHarnessWebPreparedSessionCatalogue,
    input: DeepSeekHarnessWebForkInput,
}

impl DeepSeekHarnessWebPreparedFork {
    #[must_use]
    /// Returns the catalogue-bound preflight plan used for native fork.
    pub fn plan(&self) -> &ProviderSessionCataloguePlan {
        self.catalogue.plan()
    }

    #[must_use]
    /// Returns the target Web session for this fork.
    pub fn session(&self) -> &SessionRef {
        &self.input.session
    }

    /// Executes the native fork method.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<SessionRef, RuntimeFailure>> {
        let driver = self.catalogue.low_level_driver();
        let plan = self.catalogue.plan().preflight().clone();
        let request_id = self.input.request_id.clone();
        let session = self.input.session.clone();
        let at_sequence = self.input.at_sequence;
        let deadline = self
            .input
            .deadline
            .or(self.catalogue.plan().agreement().deadline());
        Box::pin(async move {
            driver
                .fork_session(
                    &plan,
                    &request_id,
                    &session,
                    at_sequence,
                    &services,
                    deadline,
                )
                .await
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for control-free newest-first history of one Web session.
pub struct DeepSeekHarnessWebSessionHistoryInput {
    request_id: RequestId,
    history_id: ProviderSessionHistoryId,
    model: DeepSeekHarnessWebModelSelection,
    binding: SessionResumeBinding,
    bounds: ProviderSessionHistoryBounds,
    deadline: Option<Deadline>,
}

impl DeepSeekHarnessWebSessionHistoryInput {
    #[must_use]
    /// Creates history input from an exact route-bound session binding.
    pub const fn new(
        request_id: RequestId,
        history_id: ProviderSessionHistoryId,
        model: DeepSeekHarnessWebModelSelection,
        binding: SessionResumeBinding,
        bounds: ProviderSessionHistoryBounds,
    ) -> Self {
        Self {
            request_id,
            history_id,
            model,
            binding,
            bounds,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds a deadline to history paging.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug)]
/// Prepared read-only Web history operation; it exposes no resume handle.
pub struct DeepSeekHarnessWebPreparedSessionHistory {
    integration: DeepSeekHarnessWebPreparedIntegration,
    evidence: PreparedProviderSessionHistoryEvidence,
    request: ProviderSessionHistoryRequest,
}

impl DeepSeekHarnessWebPreparedSessionHistory {
    #[must_use]
    /// Returns portable history evidence.
    pub fn evidence(&self) -> &PreparedProviderSessionHistoryEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the exact history plan.
    pub fn plan(&self) -> &ProviderSessionHistoryPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the initial newest-page request.
    pub fn request(&self) -> &ProviderSessionHistoryRequest {
        &self.request
    }

    #[must_use]
    /// Returns the prepared Web route evidence.
    pub fn integration(&self) -> &DeepSeekHarnessWebPreparedIntegration {
        &self.integration
    }

    #[must_use]
    /// Creates the low-level Web driver bound to history.
    pub fn low_level_driver(&self) -> DeepSeekHarnessWebDriver {
        self.integration.low_level_driver()
    }

    /// Reads the newest bounded history page without resuming the session.
    pub fn page_history(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
        self.page(self.request.clone(), services)
    }

    /// Reads one explicitly supplied history page.
    pub fn page(
        &self,
        request: ProviderSessionHistoryRequest,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        Box::pin(async move {
            driver
                .page_provider_session_history(plan, request, services)
                .await
        })
    }

    /// Builds a continuation request from an opaque older-page cursor.
    pub fn older_page_request(
        &self,
        request_id: RequestId,
        cursor: ProviderSessionHistoryCursor,
    ) -> Result<ProviderSessionHistoryRequest, PreparationFailure> {
        ProviderSessionHistoryRequest::from_plan(request_id, self.plan(), Some(cursor)).map_err(
            |error| preparation_failure("history continuation is invalid", error.diagnostic()),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for target-only Web archive authority.
pub struct DeepSeekHarnessWebSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Option<Deadline>,
}

impl DeepSeekHarnessWebSessionManagementInput {
    #[must_use]
    /// Creates inactive-session management input from an exact binding.
    pub const fn new(request_id: RequestId, binding: ProviderSessionManagementBinding) -> Self {
        Self {
            request_id,
            binding,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds a deadline to archive dispatch.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug)]
/// Prepared target-only Web archive operation.
pub struct DeepSeekHarnessWebPreparedArchive {
    integration: DeepSeekHarnessWebPreparedIntegration,
    evidence: PreparedProviderSessionManagementEvidence,
    request: ArchiveProviderSessionRequest,
}

impl DeepSeekHarnessWebPreparedArchive {
    #[must_use]
    /// Returns portable management evidence.
    pub fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the exact archive plan.
    pub fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the bound archive request.
    pub fn request(&self) -> &ArchiveProviderSessionRequest {
        &self.request
    }

    #[must_use]
    /// Returns the prepared Web route evidence.
    pub fn integration(&self) -> &DeepSeekHarnessWebPreparedIntegration {
        &self.integration
    }

    /// Executes target-only Web archive with caller-supplied host services.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = self.integration.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.archive_session(plan, request, services).await })
    }
}

fn advertised_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        read_only_working_resource_capability(),
        CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
        CapabilityRequirement::new(Capability::ProviderSessionHistory, []),
        CapabilityRequirement::new(Capability::ProviderSessionArchive, []),
    ])
}

fn web_instance_with_capabilities(
    base: &ConfiguredInstance,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    instance_with_capabilities(base, capabilities)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

fn run_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        read_only_working_resource_capability(),
    ])
}

fn read_only_working_resource_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

fn web_operation_policy() -> swallowtail_runtime::OperationPolicy {
    swallowtail_runtime::OperationPolicy::offline()
        .with_provider_retention(swallowtail_runtime::ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

fn web_activity_profile(
    observation: &InstalledExecutableObservation,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior = match observation.compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        _ => {
            return Err(preparation_failure_message(
                "Web activity requires the exact qualified runtime-bin",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            observation.version().axis().clone(),
            behavior,
        )],
        [
            activity_kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [ActivityContentStream::FinalAnswerText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            activity_kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            activity_kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| preparation_failure_message("Web activity profile is invalid"))
}

fn with_activity(
    capabilities: CapabilityProfile,
    activity: &ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .filter(|(capability, _)| *capability != Capability::ObservableActivity)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        activity
            .capability_requirement()
            .expect("qualified Web activity profile is available"),
    );
    CapabilityProfile::new(requirements)
}

fn activity_kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, [])
        .map_err(|_| preparation_failure_message("Web activity kind is invalid"))
}

fn preparation_failure_message(message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.deepseek_harness.web.preparation.invalid",
            message,
        )),
    )
}

fn preparation_failure(
    message: &'static str,
    diagnostic: &swallowtail_core::SafeDiagnostic,
) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(
            diagnostic.code(),
            message,
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{deepseek_harness_access_profile, deepseek_harness_web_claim};
    use std::num::{NonZeroU32, NonZeroU64};
    use swallowtail_core::{
        AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId, EndpointAuthorization,
        InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, ModelId, ModelRouteId,
        ModelRouteRevision, ProviderId,
    };
    use swallowtail_runtime::MonotonicInstant;

    fn prepared() -> DeepSeekHarnessWebPreparedIntegration {
        let access_id = AccessProfileId::new("deepseek-harness.web.fixture.access").unwrap();
        let input = DeepSeekHarnessWebPreparationInput::new(
            ConfiguredInstanceId::new("deepseek-harness.web.fixture.instance").unwrap(),
            InstanceRevision::new("rc6").unwrap(),
            ExecutionHostId::new("deepseek-harness.web.fixture.host").unwrap(),
            InstalledExecutableTarget::new(
                swallowtail_runtime::ExecutableRef::new("/fixture/bin/dsh").unwrap(),
                InterfaceVersionAxis::new(crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_AXIS).unwrap(),
            ),
            EnvironmentRef::new("deepseek-harness.web.fixture.cordis").unwrap(),
            deepseek_harness_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        )
        .with_endpoint("http://127.0.0.1:3080");
        let observation = InstalledExecutableObservation::classify(
            input.execution_host_id.clone(),
            InterfaceVersionBinding::new(
                InterfaceVersionAxis::new(crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_AXIS).unwrap(),
                InterfaceVersion::new(crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_VERSION).unwrap(),
            ),
            &deepseek_harness_web_claim(),
        )
        .unwrap();
        let instance = configured_instance(&input, &observation).unwrap();
        let descriptor = crate::web::deepseek_harness_web_descriptor();
        let mut services = BTreeSet::new();
        for role in [
            DriverRole::Discovery,
            DriverRole::StructuredRun,
            DriverRole::ProviderSessionCatalogue,
            DriverRole::ProviderSessionHistory,
            DriverRole::ProviderSessionManagement,
        ] {
            services.extend(descriptor.required_host_services(role));
        }
        DeepSeekHarnessWebPreparedIntegration {
            environment: input.environment,
            endpoint: input.endpoint,
            target: input.target,
            observation,
            access_profile: input.access_profile,
            access_evidence: input.access_evidence,
            instance,
            available_host_services: services,
        }
    }

    fn catalogue_bounds() -> ProviderSessionCatalogueBounds {
        ProviderSessionCatalogueBounds::new(
            NonZeroU32::new(8).unwrap(),
            NonZeroU32::new(32).unwrap(),
            NonZeroU32::new(128).unwrap(),
            NonZeroU32::new(512).unwrap(),
            NonZeroU32::new(256).unwrap(),
        )
        .unwrap()
    }

    fn history_bounds() -> ProviderSessionHistoryBounds {
        ProviderSessionHistoryBounds::new(
            NonZeroU32::new(8).unwrap(),
            NonZeroU64::new(8_192).unwrap(),
            NonZeroU32::new(128).unwrap(),
            NonZeroU32::new(32).unwrap(),
        )
    }

    #[test]
    fn prepared_web_facades_keep_route_endpoint_allowlist_and_access_explicit() {
        let prepared = prepared();
        assert_eq!(prepared.release_axis(), "deepseek-harness.web");
        assert_eq!(prepared.release_version(), "0.1.0-rc.6");
        assert_eq!(prepared.endpoint(), "http://127.0.0.1:3080");
        assert_eq!(prepared.allowlisted_methods().len(), 11);
        assert_eq!(
            prepared.access_evidence().status().credential(),
            CredentialState::NotRequired
        );

        let run = prepared
            .prepare_run(DeepSeekHarnessWebRunProfileInput::new(
                RequestId::new("deepseek-harness.web.fixture.run").unwrap(),
                DeepSeekHarnessWebModelSelection::new(
                    ModelRouteId::new("deepseek-harness.web.fixture.route").unwrap(),
                    ModelRouteRevision::new("fixture-v1").unwrap(),
                    ProviderId::new("fixture-provider").unwrap(),
                    ModelId::new("fixture-model").unwrap(),
                ),
                OperationContent::new("fixture prompt").unwrap(),
                WorkingResourceRef::new("deepseek-harness.web.fixture.workspace").unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(10_000)),
            ))
            .unwrap();
        assert_eq!(
            run.plan().driver_identity().id().as_str(),
            "swallowtail.deepseek-harness.local-server"
        );
        assert_eq!(
            run.plan().transport_family().as_str(),
            "deepseek-harness-local-server-http-ws-v1"
        );
        assert_eq!(
            run.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );

        let catalogue = prepared
            .prepare_session_catalogue(DeepSeekHarnessWebSessionCatalogueInput::new(
                RequestId::new("deepseek-harness.web.fixture.catalogue").unwrap(),
                ProviderSessionCatalogueId::new("deepseek-harness.web.fixture.catalogue-id")
                    .unwrap(),
                WorkingResourceRef::new("deepseek-harness.web.fixture.workspace").unwrap(),
                catalogue_bounds(),
            ))
            .unwrap();
        let session = SessionRef::new("deepseek-harness.web.fixture.session").unwrap();
        let fork = catalogue.prepare_fork(
            DeepSeekHarnessWebForkInput::new(
                RequestId::new("deepseek-harness.web.fixture.fork").unwrap(),
                session.clone(),
            )
            .at_sequence(4),
        );
        assert_eq!(
            fork.plan().preflight().requirements().driver_role(),
            DriverRole::ProviderSessionCatalogue
        );

        let model = DeepSeekHarnessWebModelSelection::new(
            ModelRouteId::new("deepseek-harness.web.fixture.route").unwrap(),
            ModelRouteRevision::new("fixture-v1").unwrap(),
            ProviderId::new("fixture-provider").unwrap(),
            ModelId::new("fixture-model").unwrap(),
        );
        let binding = SessionResumeBinding::new(
            session.clone(),
            prepared.instance.id().clone(),
            prepared.instance.execution_host_id().clone(),
            ModelRouteId::new("deepseek-harness.web.fixture.route").unwrap(),
            ModelId::new("fixture-model").unwrap(),
            WorkingResourceRef::new("deepseek-harness.web.fixture.workspace").unwrap(),
            SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
        );
        let history = prepared
            .prepare_session_history(DeepSeekHarnessWebSessionHistoryInput::new(
                RequestId::new("deepseek-harness.web.fixture.history").unwrap(),
                ProviderSessionHistoryId::new("deepseek-harness.web.fixture.history-id").unwrap(),
                model,
                binding,
                history_bounds(),
            ))
            .unwrap();
        assert!(
            !history
                .plan()
                .preflight()
                .requirements()
                .capabilities()
                .any(|required| required.capability() == Capability::Resume)
        );
        assert_eq!(
            history
                .plan()
                .preflight()
                .requirements()
                .session_access_policy(),
            Some(&SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        );

        let management_binding = prepared
            .management_binding(
                session,
                Some(WorkingResourceRef::new("deepseek-harness.web.fixture.workspace").unwrap()),
                ProviderSessionBindingOrigin::Loaded,
            )
            .unwrap();
        let archive = prepared
            .prepare_archive_session(DeepSeekHarnessWebSessionManagementInput::new(
                RequestId::new("deepseek-harness.web.fixture.archive").unwrap(),
                management_binding,
            ))
            .unwrap();
        assert_eq!(
            archive.plan().agreement().action(),
            ProviderSessionManagementAction::Archive
        );
        assert_eq!(
            archive.plan().preflight().requirements().driver_role(),
            DriverRole::ProviderSessionManagement
        );
    }

    #[test]
    fn invalid_prepared_web_endpoint_is_rejected_before_route_admission() {
        let access_id = AccessProfileId::new("deepseek-harness.web.fixture.invalid").unwrap();
        let input = DeepSeekHarnessWebPreparationInput::new(
            ConfiguredInstanceId::new("deepseek-harness.web.fixture.invalid-instance").unwrap(),
            InstanceRevision::new("rc6").unwrap(),
            ExecutionHostId::new("deepseek-harness.web.fixture.invalid-host").unwrap(),
            InstalledExecutableTarget::new(
                swallowtail_runtime::ExecutableRef::new("/fixture/bin/dsh").unwrap(),
                InterfaceVersionAxis::new(crate::web::DEEPSEEK_HARNESS_WEB_RELEASE_AXIS).unwrap(),
            ),
            EnvironmentRef::new("deepseek-harness.web.fixture.cordis").unwrap(),
            deepseek_harness_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        )
        .with_endpoint("https://127.0.0.1:3080");
        assert!(validate_input(&input).is_err());
    }
}
