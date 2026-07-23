use crate::{
    AttachedServingHandle, AttachmentDescriptor, BoxFuture, Deadline, HostServices,
    InstalledExecutableDiscoveryRequest, InteractiveSessionHandle, ModelArtifactBinding,
    OpenDirectContinuationSessionRequest, OpenRealtimeMediaSessionRequest, OperationContent,
    OperationPolicy, OwnedServingHandle, RealtimeMediaSessionHandle, RequestId, RunHandle,
    RuntimeFailure, RuntimeTurnId, ScopeId, ServingInstanceId, SessionAccessPolicy, SessionOptions,
    SessionReplayItem, SessionResumeBinding, StructuredOutputDescriptor, ToolDeclaration,
    WorkingResourceRef,
};
use std::num::NonZeroU64;
use swallowtail_core::{
    DiscoveryOutcome, ExecutionHostId, ModelCatalogEntry, PreflightPlan, SessionRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryRequest {
    execution_host_id: ExecutionHostId,
}

impl DiscoveryRequest {
    #[must_use]
    pub const fn new(execution_host_id: ExecutionHostId) -> Self {
        Self { execution_host_id }
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogRequest {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl ModelCatalogRequest {
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuredRunRequest {
    request_id: RequestId,
    content: OperationContent,
    working_resource: Option<WorkingResourceRef>,
    policy: OperationPolicy,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    tools: Vec<ToolDeclaration>,
    structured_output: Option<StructuredOutputDescriptor>,
    maximum_output_tokens: Option<NonZeroU64>,
}

impl StructuredRunRequest {
    #[must_use]
    pub fn new(request_id: RequestId, content: OperationContent, policy: OperationPolicy) -> Self {
        Self {
            request_id,
            content,
            working_resource: None,
            policy,
            deadline: None,
            attachments: Vec::new(),
            tools: Vec::new(),
            structured_output: None,
            maximum_output_tokens: None,
        }
    }

    #[must_use]
    pub fn with_working_resource(mut self, working_resource: WorkingResourceRef) -> Self {
        self.working_resource = Some(working_resource);
        self
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.attachments = attachments.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_tools(mut self, tools: impl IntoIterator<Item = ToolDeclaration>) -> Self {
        self.tools = tools.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }

    #[must_use]
    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.maximum_output_tokens = Some(maximum);
        self
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }

    #[must_use]
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource.as_ref()
    }

    #[must_use]
    pub const fn policy(&self) -> &OperationPolicy {
        &self.policy
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    pub fn attachments(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.attachments.iter()
    }

    pub fn tools(&self) -> impl ExactSizeIterator<Item = &ToolDeclaration> {
        self.tools.iter()
    }

    #[must_use]
    pub const fn structured_output(&self) -> Option<&StructuredOutputDescriptor> {
        self.structured_output.as_ref()
    }

    #[must_use]
    pub const fn maximum_output_tokens(&self) -> Option<NonZeroU64> {
        self.maximum_output_tokens
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenSessionRequest {
    request_id: RequestId,
    working_resource: Option<WorkingResourceRef>,
    deadline: Option<Deadline>,
    options: SessionOptions,
    access_policy: SessionAccessPolicy,
    provider_state_policy: swallowtail_core::SessionProviderStatePolicy,
}

impl OpenSessionRequest {
    #[must_use]
    pub fn new(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            working_resource: Some(working_resource),
            deadline,
            options: SessionOptions::default(),
            access_policy: SessionAccessPolicy::default(),
            provider_state_policy: swallowtail_core::SessionProviderStatePolicy::default(),
        }
    }

    #[must_use]
    pub fn resource_free(request_id: RequestId, deadline: Option<Deadline>) -> Self {
        Self {
            request_id,
            working_resource: None,
            deadline,
            options: SessionOptions::default(),
            access_policy: SessionAccessPolicy::resource_free(),
            provider_state_policy: swallowtail_core::SessionProviderStatePolicy::default(),
        }
    }

    #[must_use]
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn with_access_policy(mut self, policy: SessionAccessPolicy) -> Self {
        self.access_policy = policy;
        self
    }

    #[must_use]
    pub const fn with_provider_state_policy(
        mut self,
        policy: swallowtail_core::SessionProviderStatePolicy,
    ) -> Self {
        self.provider_state_policy = policy;
        self
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource.as_ref()
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    pub const fn options(&self) -> &SessionOptions {
        &self.options
    }

    #[must_use]
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        &self.access_policy
    }

    #[must_use]
    pub const fn provider_state_policy(&self) -> swallowtail_core::SessionProviderStatePolicy {
        self.provider_state_policy
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeSessionRequest {
    request_id: RequestId,
    binding: SessionResumeBinding,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    options: SessionOptions,
    access_policy: SessionAccessPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadSessionRequest {
    request_id: RequestId,
    binding: SessionResumeBinding,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    options: SessionOptions,
    access_policy: SessionAccessPolicy,
}

impl LoadSessionRequest {
    #[must_use]
    pub fn new(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            binding,
            working_resource,
            deadline,
            options: SessionOptions::default(),
            access_policy: SessionAccessPolicy::default(),
        }
    }

    #[must_use]
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn with_access_policy(mut self, policy: SessionAccessPolicy) -> Self {
        self.access_policy = policy;
        self
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }
    #[must_use]
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.binding.provider_session_ref()
    }
    #[must_use]
    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        &self.binding
    }
    #[must_use]
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }
    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
    #[must_use]
    pub const fn options(&self) -> &SessionOptions {
        &self.options
    }
    #[must_use]
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        &self.access_policy
    }
}

pub struct LoadedSession {
    replay: Vec<SessionReplayItem>,
    session: Box<dyn InteractiveSessionHandle>,
}

impl LoadedSession {
    #[must_use]
    pub fn new(replay: Vec<SessionReplayItem>, session: Box<dyn InteractiveSessionHandle>) -> Self {
        Self { replay, session }
    }

    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.replay.iter()
    }

    #[must_use]
    pub fn into_parts(self) -> (Vec<SessionReplayItem>, Box<dyn InteractiveSessionHandle>) {
        (self.replay, self.session)
    }
}

impl ResumeSessionRequest {
    #[must_use]
    pub fn new(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            binding,
            working_resource,
            deadline,
            options: SessionOptions::default(),
            access_policy: SessionAccessPolicy::default(),
        }
    }

    #[must_use]
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn with_access_policy(mut self, policy: SessionAccessPolicy) -> Self {
        self.access_policy = policy;
        self
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.binding.provider_session_ref()
    }

    #[must_use]
    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        &self.binding
    }

    #[must_use]
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    pub const fn options(&self) -> &SessionOptions {
        &self.options
    }

    #[must_use]
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        &self.access_policy
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnRequest {
    turn_id: RuntimeTurnId,
    content: OperationContent,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    structured_output: Option<StructuredOutputDescriptor>,
}

impl TurnRequest {
    #[must_use]
    pub fn new(turn_id: RuntimeTurnId, content: OperationContent) -> Self {
        Self {
            turn_id,
            content,
            deadline: None,
            attachments: Vec::new(),
            structured_output: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.attachments = attachments.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_structured_output(mut self, output: StructuredOutputDescriptor) -> Self {
        self.structured_output = Some(output);
        self
    }

    #[must_use]
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    pub fn attachments(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.attachments.iter()
    }

    #[must_use]
    pub const fn structured_output(&self) -> Option<&StructuredOutputDescriptor> {
        self.structured_output.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachServingRequest {
    serving_instance_id: ServingInstanceId,
}

impl AttachServingRequest {
    #[must_use]
    pub const fn new(serving_instance_id: ServingInstanceId) -> Self {
        Self {
            serving_instance_id,
        }
    }

    #[must_use]
    pub const fn serving_instance_id(&self) -> &ServingInstanceId {
        &self.serving_instance_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartServingRequest {
    scope: ScopeId,
    serving_instance_id: ServingInstanceId,
    artifact: ModelArtifactBinding,
    deadline: Deadline,
}

impl StartServingRequest {
    #[must_use]
    pub const fn new(
        scope: ScopeId,
        serving_instance_id: ServingInstanceId,
        artifact: ModelArtifactBinding,
        deadline: Deadline,
    ) -> Self {
        Self {
            scope,
            serving_instance_id,
            artifact,
            deadline,
        }
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn serving_instance_id(&self) -> &ServingInstanceId {
        &self.serving_instance_id
    }

    #[must_use]
    pub const fn artifact(&self) -> &ModelArtifactBinding {
        &self.artifact
    }

    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

pub trait DiscoveryDriver: Send + Sync {
    fn discover(
        &self,
        request: DiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>>;

    fn discover_installed_executable(
        &self,
        _request: InstalledExecutableDiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.installed_executable.discovery_unsupported",
                "Driver does not support installed executable discovery",
            )))
        })
    }
}

pub trait ModelCatalogDriver: Send + Sync {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>>;
}

pub trait StructuredRunDriver: Send + Sync {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>>;
}

pub trait InteractiveSessionDriver: Send + Sync {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;

    fn load_session(
        &self,
        _plan: PreflightPlan,
        _request: LoadSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.session_load_unsupported",
                "Driver does not support provider session load",
            )))
        })
    }

    fn open_direct_continuation_session(
        &self,
        _plan: PreflightPlan,
        _request: OpenDirectContinuationSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.direct_continuation.unsupported",
                "Driver does not support locally continued direct sessions",
            )))
        })
    }
}

pub trait RealtimeMediaSessionDriver: Send + Sync {
    fn open_realtime_media_session(
        &self,
        plan: PreflightPlan,
        request: OpenRealtimeMediaSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>>;
}

pub trait ServingInstanceDriver: Send + Sync {
    fn attach(
        &self,
        plan: PreflightPlan,
        request: AttachServingRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn AttachedServingHandle>, RuntimeFailure>>;

    fn start(
        &self,
        plan: PreflightPlan,
        request: StartServingRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn OwnedServingHandle>, RuntimeFailure>>;
}
