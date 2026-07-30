impl DiscoveryRequest {
    const fn new_inner(execution_host_id: ExecutionHostId) -> Self {
        Self { execution_host_id }
    }

    #[must_use]
    const fn execution_host_id_inner(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }
}

impl ModelCatalogRequest {
    const fn new_inner(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    #[must_use]
    const fn with_deadline_inner(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    const fn request_id_inner(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    const fn deadline_inner(&self) -> Option<Deadline> {
        self.deadline
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StructuredRunRequestState {
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
    fn new_inner(request_id: RequestId, content: OperationContent, policy: OperationPolicy) -> Self {
        Self {
            state: StructuredRunRequestState {
            request_id,
            content,
            working_resource: None,
            policy,
            deadline: None,
            attachments: Vec::new(),
            tools: Vec::new(),
            structured_output: None,
            maximum_output_tokens: None,
            },
        }
    }

    #[must_use]
    fn with_working_resource_inner(mut self, working_resource: WorkingResourceRef) -> Self {
        self.state.working_resource = Some(working_resource);
        self
    }

    #[must_use]
    const fn with_deadline_inner(mut self, deadline: Deadline) -> Self {
        self.state.deadline = Some(deadline);
        self
    }

    #[must_use]
    fn with_attachments_inner(
        mut self,
        attachments: impl IntoIterator<Item = AttachmentDescriptor>,
    ) -> Self {
        self.state.attachments = attachments.into_iter().collect();
        self
    }

    #[must_use]
    fn with_tools_inner(mut self, tools: impl IntoIterator<Item = ToolDeclaration>) -> Self {
        self.state.tools = tools.into_iter().collect();
        self
    }

    #[must_use]
    fn with_structured_output_inner(mut self, output: StructuredOutputDescriptor) -> Self {
        self.state.structured_output = Some(output);
        self
    }

    #[must_use]
    const fn with_maximum_output_tokens_inner(mut self, maximum: NonZeroU64) -> Self {
        self.state.maximum_output_tokens = Some(maximum);
        self
    }

    #[must_use]
    const fn request_id_inner(&self) -> &RequestId {
        &self.state.request_id
    }

    #[must_use]
    const fn content_inner(&self) -> &OperationContent {
        &self.state.content
    }

    #[must_use]
    const fn working_resource_inner(&self) -> Option<&WorkingResourceRef> {
        self.state.working_resource.as_ref()
    }

    #[must_use]
    const fn policy_inner(&self) -> &OperationPolicy {
        &self.state.policy
    }

    #[must_use]
    const fn deadline_inner(&self) -> Option<Deadline> {
        self.state.deadline
    }

    fn attachments_inner(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.state.attachments.iter()
    }

    fn tools_inner(&self) -> impl ExactSizeIterator<Item = &ToolDeclaration> {
        self.state.tools.iter()
    }

    #[must_use]
    const fn structured_output_inner(&self) -> Option<&StructuredOutputDescriptor> {
        self.state.structured_output.as_ref()
    }

    #[must_use]
    const fn maximum_output_tokens_inner(&self) -> Option<NonZeroU64> {
        self.state.maximum_output_tokens
    }
}
