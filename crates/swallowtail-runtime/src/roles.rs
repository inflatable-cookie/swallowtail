#![allow(unused_mut)]
#![deny(missing_docs)]

include!("roles/imports.rs");
include!("roles/drivers/provider_run_reconciliation.rs");
include!("roles/drivers/provider_recovered_resource_cleanup.rs");

/// Request to discover provider instances visible on one execution host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryRequest {
    execution_host_id: ExecutionHostId,
}

impl DiscoveryRequest {
    /// Creates discovery input bound to the supplied execution host.
    pub const fn new(execution_host_id: ExecutionHostId) -> Self {
        Self::new_inner(execution_host_id)
    }

    /// Returns the execution host within which discovery may run.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        self.execution_host_id_inner()
    }
}

/// Request to list models from one already selected driver instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogRequest {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl ModelCatalogRequest {
    /// Creates catalogue input with no deadline.
    pub const fn new(request_id: RequestId) -> Self {
        Self::new_inner(request_id)
    }

    /// Adds the absolute deadline for the catalogue operation.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.with_deadline_inner(deadline)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the absolute deadline when one was supplied.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }
}

/// Complete input for one bounded structured provider run.
///
/// Optional working resources, attachments, tools, schemas, and output bounds
/// carry only authority explicitly supplied by the consumer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuredRunRequest {
    state: StructuredRunRequestState,
}

impl StructuredRunRequest {
    /// Creates a run request from content and an admitted operation policy.
    pub fn new(request_id: RequestId, content: OperationContent, policy: OperationPolicy) -> Self {
        Self::new_inner(request_id, content, policy)
    }

    /// Binds one opaque working resource to the run.
    pub fn with_working_resource(mut self, working_resource: WorkingResourceRef) -> Self {
        self.with_working_resource_inner(working_resource)
    }

    /// Adds the absolute run deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.with_deadline_inner(deadline)
    }

    /// Replaces the attachment set with the supplied descriptors.
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.with_attachments_inner(attachments)
    }

    /// Replaces the portable tool set exposed to the provider.
    pub fn with_tools(mut self, tools: impl IntoIterator<Item = ToolDeclaration>) -> Self {
        self.with_tools_inner(tools)
    }

    /// Requests output conforming to the supplied structured descriptor.
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.with_structured_output_inner(output)
    }

    /// Adds a maximum generated-token bound.
    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.with_maximum_output_tokens_inner(maximum)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the user content sent to the run.
    pub const fn content(&self) -> &OperationContent {
        self.content_inner()
    }

    /// Returns the bound working resource when present.
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    /// Returns the admitted operation policy.
    pub const fn policy(&self) -> &OperationPolicy {
        self.policy_inner()
    }

    /// Returns the absolute deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Iterates over the requested attachments in caller order.
    pub fn attachments(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.attachments_inner()
    }

    /// Iterates over the requested portable tools in caller order.
    pub fn tools(&self) -> impl ExactSizeIterator<Item = &ToolDeclaration> {
        self.tools_inner()
    }

    /// Returns the requested structured-output descriptor when present.
    pub const fn structured_output(&self) -> Option<&StructuredOutputDescriptor> {
        self.structured_output_inner()
    }

    /// Returns the maximum generated-token bound when present.
    pub const fn maximum_output_tokens(&self) -> Option<NonZeroU64> {
        self.maximum_output_tokens_inner()
    }
}

/// Request to open a fresh interactive provider session.
///
/// The embedded plan agreement carries the preflight decisions that the
/// runtime must preserve while preparing the session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenSessionRequest {
    state: OpenSessionRequestState,
}

impl OpenSessionRequest {
    /// Creates a request bound to one working resource.
    pub fn new(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(request_id, working_resource, deadline, plan_agreement)
    }

    /// Creates a request for a route that does not require a working resource.
    pub fn resource_free(
        request_id: RequestId,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::resource_free_inner(request_id, deadline, plan_agreement)
    }

    /// Creates a resource-bound request from an admitted preflight plan.
    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, working_resource, deadline)
    }

    /// Creates a resource-free request from an admitted preflight plan.
    pub fn resource_free_from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::resource_free_from_plan_inner(plan, request_id, deadline)
    }

    /// Replaces the provider-facing session options.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the working resource when the route requires one.
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    /// Returns the absolute session-open deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Returns the requested session options.
    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    /// Returns the access policy retained from preflight.
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    /// Returns the provider-state policy retained from preflight when applicable.
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    /// Returns the harness-configuration posture retained from preflight.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    /// Returns the complete immutable preflight agreement.
    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

/// Request to resume an admitted provider session without loading replay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeSessionRequest {
    state: ResumeSessionRequestState,
}

/// Request to load bounded replay and attach to an admitted provider session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadSessionRequest {
    state: LoadSessionRequestState,
}

impl LoadSessionRequest {
    /// Creates a load request bound to one working resource.
    pub fn new(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(
            request_id,
            binding,
            working_resource,
            deadline,
            plan_agreement,
        )
    }

    /// Creates a resource-bound load request from an admitted preflight plan.
    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, binding, working_resource, deadline)
    }

    /// Creates a load request for a route without a working resource.
    #[must_use]
    pub fn resource_free(
        request_id: RequestId,
        binding: SessionResumeBinding,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::resource_free_inner(request_id, binding, deadline, plan_agreement)
    }

    /// Creates a resource-free load request from an admitted preflight plan.
    pub fn resource_free_from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::resource_free_from_plan_inner(plan, request_id, binding, deadline)
    }

    /// Replaces the provider-facing session options.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the provider-session reference carried by the resume binding.
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.provider_session_ref_inner()
    }

    /// Returns the exact resume authority supplied by the consumer.
    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        self.resume_binding_inner()
    }

    /// Returns the working resource when the route requires one.
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    /// Returns the absolute load deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Returns the requested session options.
    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    /// Returns the access policy retained from preflight.
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    /// Returns the provider-state policy retained from preflight when applicable.
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    /// Returns the harness-configuration posture retained from preflight.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    /// Returns the complete immutable preflight agreement.
    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

/// Result of loading bounded replay before returning a live session handle.
pub struct LoadedSession {
    state: LoadedSessionState,
}

impl LoadedSession {
    /// Combines ordered replay evidence with the attached live session.
    pub fn new(replay: Vec<SessionReplayItem>, session: Box<dyn InteractiveSessionHandle>) -> Self {
        Self::new_inner(replay, session)
    }

    /// Iterates over replay items in provider order.
    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.replay_inner()
    }

    /// Separates replay evidence from the live session handle.
    pub fn into_parts(self) -> (Vec<SessionReplayItem>, Box<dyn InteractiveSessionHandle>) {
        self.into_parts_inner()
    }
}

impl ResumeSessionRequest {
    /// Creates a resume request bound to the original working resource.
    pub fn new(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(
            request_id,
            binding,
            working_resource,
            deadline,
            plan_agreement,
        )
    }

    /// Creates a resume request from an admitted preflight plan.
    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, binding, working_resource, deadline)
    }

    /// Replaces the provider-facing session options.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the provider-session reference carried by the resume binding.
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.provider_session_ref_inner()
    }

    /// Returns the exact resume authority supplied by the consumer.
    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        self.resume_binding_inner()
    }

    /// Returns the working resource retained for the resumed session.
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        self.working_resource_inner()
    }

    /// Returns the absolute resume deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Returns the requested session options.
    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    /// Returns the access policy retained from preflight.
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    /// Returns the provider-state policy retained from preflight when applicable.
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    /// Returns the harness-configuration posture retained from preflight.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    /// Returns the complete immutable preflight agreement.
    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

/// Input for one turn on an already open interactive session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnRequest {
    state: TurnRequestState,
}

impl TurnRequest {
    /// Creates a turn request with no deadline, attachments, or output schema.
    pub fn new(turn_id: RuntimeTurnId, content: OperationContent) -> Self {
        Self::new_inner(turn_id, content)
    }

    /// Adds the absolute turn deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.with_deadline_inner(deadline)
    }

    /// Replaces the attachment set with the supplied descriptors.
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.with_attachments_inner(attachments)
    }

    /// Requests output conforming to the supplied structured descriptor.
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.with_structured_output_inner(output)
    }

    /// Returns the caller-assigned runtime turn identity.
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        self.turn_id_inner()
    }

    /// Returns the user content sent on this turn.
    pub const fn content(&self) -> &OperationContent {
        self.content_inner()
    }

    /// Returns the absolute turn deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Iterates over attachments in caller order.
    pub fn attachments(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.attachments_inner()
    }

    /// Returns the requested structured-output descriptor when present.
    pub const fn structured_output(&self) -> Option<&StructuredOutputDescriptor> {
        self.structured_output_inner()
    }
}

/// Request to attach to one already-running serving instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachServingRequest {
    serving_instance_id: ServingInstanceId,
}

impl AttachServingRequest {
    /// Creates an attachment request for the exact serving identity.
    pub const fn new(serving_instance_id: ServingInstanceId) -> Self {
        Self::new_inner(serving_instance_id)
    }

    /// Returns the serving instance to attach to.
    pub const fn serving_instance_id(&self) -> &ServingInstanceId {
        self.serving_instance_id_inner()
    }
}

/// Request to start one operation-owned model-serving instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartServingRequest {
    state: StartServingRequestState,
}

impl StartServingRequest {
    /// Creates a bounded start request from an admitted model artifact.
    pub const fn new(
        scope: ScopeId,
        serving_instance_id: ServingInstanceId,
        artifact: ModelArtifactBinding,
        deadline: Deadline,
    ) -> Self {
        Self::new_inner(scope, serving_instance_id, artifact, deadline)
    }

    /// Returns the operation scope that will own the instance.
    pub const fn scope(&self) -> &ScopeId {
        self.scope_inner()
    }

    /// Returns the caller-assigned serving instance identity.
    pub const fn serving_instance_id(&self) -> &ServingInstanceId {
        self.serving_instance_id_inner()
    }

    /// Returns the admitted model-artifact binding.
    pub const fn artifact(&self) -> &ModelArtifactBinding {
        self.artifact_inner()
    }

    /// Returns the absolute startup deadline.
    pub const fn deadline(&self) -> Deadline {
        self.deadline_inner()
    }
}

include!("roles/requests/basic.rs");
include!("roles/requests/sessions.rs");
include!("roles/requests/turn_and_serving.rs");

include!("roles/drivers/discovery.rs");
include!("roles/drivers/catalogue.rs");
include!("roles/drivers/structured_run.rs");
include!("roles/drivers/interactive_session.rs");
include!("roles/drivers/realtime.rs");
include!("roles/drivers/provider_session_management.rs");
include!("roles/drivers/provider_session_catalogue.rs");
include!("roles/drivers/provider_session_import.rs");
include!("roles/drivers/provider_session_reconciliation.rs");
include!("roles/drivers/serving.rs");

/// Discovers provider instances without starting an inference operation.
pub trait DiscoveryDriver: Send + Sync {
    discovery_driver_items!();
}

/// Lists models available through one prepared provider instance.
pub trait ModelCatalogDriver: Send + Sync {
    catalogue_driver_items!();
}

/// Starts one bounded provider run from an admitted preflight plan.
pub trait StructuredRunDriver: Send + Sync {
    structured_run_driver_items!();
}

/// Opens, resumes, loads, or recovers reusable provider sessions.
pub trait InteractiveSessionDriver: Send + Sync {
    interactive_session_driver_items!();
}

/// Opens one realtime duplex media session.
pub trait RealtimeMediaSessionDriver: Send + Sync {
    realtime_driver_items!();
}

/// Low-level role for one explicitly bound inactive provider session.
///
/// Implementations must finish all scoped work and preserve uncertain
/// after-dispatch truth before resolving the returned future.
pub trait ProviderSessionManagementDriver: Send + Sync {
    provider_session_management_driver_items!();
}

/// Read-only discovery of provider-owned sessions within one prepared scope.
pub trait ProviderSessionCatalogueDriver: Send + Sync {
    provider_session_catalogue_driver_items!();
}

/// Read-only revalidation and binding issue for one explicitly selected session.
pub trait ProviderSessionImportDriver: Send + Sync {
    provider_session_import_driver_items!();
}

/// Read-only observation of provider work left attached to a durable session.
///
/// This role grants no cancellation, callback, continuation, or session-import
/// authority.
pub trait ProviderSessionReconciliationDriver: Send + Sync {
    provider_session_reconciliation_driver_items!();
}

/// Read-only observation of one exact provider-owned structured run.
///
/// This role grants no create, retry, stream attachment, cancellation,
/// callback, deletion, or provider-session authority.
pub trait ProviderRunReconciliationDriver: Send + Sync {
    provider_run_reconciliation_driver_items!();
}

/// Destructive cleanup of exact inactive resources from one recovered run.
///
/// This role accepts only the separately persisted cleanup binding. It grants
/// no interruption, retry, callback, or provider-specific ordering authority.
pub trait ProviderRecoveredResourceCleanupDriver: Send + Sync {
    provider_recovered_resource_cleanup_driver_items!();
}

/// Attaches to or starts a model-serving instance.
pub trait ServingInstanceDriver: Send + Sync {
    serving_driver_items!();
}
