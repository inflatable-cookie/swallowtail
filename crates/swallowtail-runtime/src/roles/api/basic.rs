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

