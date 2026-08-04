#![allow(unused_mut)]
include!("roles/imports.rs");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryRequest {
    execution_host_id: ExecutionHostId,
}

impl DiscoveryRequest {
    pub const fn new(execution_host_id: ExecutionHostId) -> Self {
        Self::new_inner(execution_host_id)
    }

    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        self.execution_host_id_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogRequest {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl ModelCatalogRequest {
    pub const fn new(request_id: RequestId) -> Self {
        Self::new_inner(request_id)
    }

    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.with_deadline_inner(deadline)
    }

    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuredRunRequest {
    state: StructuredRunRequestState,
}

impl StructuredRunRequest {
    pub fn new(request_id: RequestId, content: OperationContent, policy: OperationPolicy) -> Self {
        Self::new_inner(request_id, content, policy)
    }

    pub fn with_working_resource(mut self, working_resource: WorkingResourceRef) -> Self {
        self.with_working_resource_inner(working_resource)
    }

    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.with_deadline_inner(deadline)
    }

    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.with_attachments_inner(attachments)
    }

    pub fn with_tools(mut self, tools: impl IntoIterator<Item = ToolDeclaration>) -> Self {
        self.with_tools_inner(tools)
    }

    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.with_structured_output_inner(output)
    }

    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.with_maximum_output_tokens_inner(maximum)
    }

    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    pub const fn content(&self) -> &OperationContent {
        self.content_inner()
    }

    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    pub const fn policy(&self) -> &OperationPolicy {
        self.policy_inner()
    }

    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    pub fn attachments(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.attachments_inner()
    }

    pub fn tools(&self) -> impl ExactSizeIterator<Item = &ToolDeclaration> {
        self.tools_inner()
    }

    pub const fn structured_output(&self) -> Option<&StructuredOutputDescriptor> {
        self.structured_output_inner()
    }

    pub const fn maximum_output_tokens(&self) -> Option<NonZeroU64> {
        self.maximum_output_tokens_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenSessionRequest {
    state: OpenSessionRequestState,
}

impl OpenSessionRequest {
    pub fn new(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(request_id, working_resource, deadline, plan_agreement)
    }

    pub fn resource_free(
        request_id: RequestId,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::resource_free_inner(request_id, deadline, plan_agreement)
    }

    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, working_resource, deadline)
    }

    pub fn resource_free_from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::resource_free_from_plan_inner(plan, request_id, deadline)
    }

    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeSessionRequest {
    state: ResumeSessionRequestState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadSessionRequest {
    state: LoadSessionRequestState,
}

impl LoadSessionRequest {
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

    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, binding, working_resource, deadline)
    }

    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.provider_session_ref_inner()
    }

    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        self.resume_binding_inner()
    }

    pub const fn working_resource(&self) -> &WorkingResourceRef {
        self.working_resource_inner()
    }

    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

pub struct LoadedSession {
    state: LoadedSessionState,
}

impl LoadedSession {
    pub fn new(replay: Vec<SessionReplayItem>, session: Box<dyn InteractiveSessionHandle>) -> Self {
        Self::new_inner(replay, session)
    }

    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.replay_inner()
    }

    pub fn into_parts(self) -> (Vec<SessionReplayItem>, Box<dyn InteractiveSessionHandle>) {
        self.into_parts_inner()
    }
}

impl ResumeSessionRequest {
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

    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, binding, working_resource, deadline)
    }

    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.provider_session_ref_inner()
    }

    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        self.resume_binding_inner()
    }

    pub const fn working_resource(&self) -> &WorkingResourceRef {
        self.working_resource_inner()
    }

    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnRequest {
    state: TurnRequestState,
}

impl TurnRequest {
    pub fn new(turn_id: RuntimeTurnId, content: OperationContent) -> Self {
        Self::new_inner(turn_id, content)
    }

    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.with_deadline_inner(deadline)
    }

    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.with_attachments_inner(attachments)
    }

    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.with_structured_output_inner(output)
    }

    pub const fn turn_id(&self) -> &RuntimeTurnId {
        self.turn_id_inner()
    }

    pub const fn content(&self) -> &OperationContent {
        self.content_inner()
    }

    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    pub fn attachments(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.attachments_inner()
    }

    pub const fn structured_output(&self) -> Option<&StructuredOutputDescriptor> {
        self.structured_output_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachServingRequest {
    serving_instance_id: ServingInstanceId,
}

impl AttachServingRequest {
    pub const fn new(serving_instance_id: ServingInstanceId) -> Self {
        Self::new_inner(serving_instance_id)
    }

    pub const fn serving_instance_id(&self) -> &ServingInstanceId {
        self.serving_instance_id_inner()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartServingRequest {
    state: StartServingRequestState,
}

impl StartServingRequest {
    pub const fn new(
        scope: ScopeId,
        serving_instance_id: ServingInstanceId,
        artifact: ModelArtifactBinding,
        deadline: Deadline,
    ) -> Self {
        Self::new_inner(scope, serving_instance_id, artifact, deadline)
    }

    pub const fn scope(&self) -> &ScopeId {
        self.scope_inner()
    }

    pub const fn serving_instance_id(&self) -> &ServingInstanceId {
        self.serving_instance_id_inner()
    }

    pub const fn artifact(&self) -> &ModelArtifactBinding {
        self.artifact_inner()
    }

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

pub trait DiscoveryDriver: Send + Sync {
    discovery_driver_items!();
}

pub trait ModelCatalogDriver: Send + Sync {
    catalogue_driver_items!();
}

pub trait StructuredRunDriver: Send + Sync {
    structured_run_driver_items!();
}

pub trait InteractiveSessionDriver: Send + Sync {
    interactive_session_driver_items!();
}

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

pub trait ServingInstanceDriver: Send + Sync {
    serving_driver_items!();
}
