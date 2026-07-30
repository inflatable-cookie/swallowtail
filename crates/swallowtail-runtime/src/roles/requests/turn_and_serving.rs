#[derive(Clone, Debug, Eq, PartialEq)]
struct TurnRequestState {
    turn_id: RuntimeTurnId,
    content: OperationContent,
    deadline: Option<Deadline>,
    attachments: Vec<AttachmentDescriptor>,
    structured_output: Option<StructuredOutputDescriptor>,
}

impl TurnRequest {
    fn new_inner(turn_id: RuntimeTurnId, content: OperationContent) -> Self {
        Self {
            state: TurnRequestState {
            turn_id,
            content,
            deadline: None,
            attachments: Vec::new(),
            structured_output: None,
            },
        }
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
    fn with_structured_output_inner(mut self, output: StructuredOutputDescriptor) -> Self {
        self.state.structured_output = Some(output);
        self
    }

    #[must_use]
    const fn turn_id_inner(&self) -> &RuntimeTurnId {
        &self.state.turn_id
    }

    #[must_use]
    const fn content_inner(&self) -> &OperationContent {
        &self.state.content
    }

    #[must_use]
    const fn deadline_inner(&self) -> Option<Deadline> {
        self.state.deadline
    }

    fn attachments_inner(&self) -> impl ExactSizeIterator<Item = &AttachmentDescriptor> {
        self.state.attachments.iter()
    }

    #[must_use]
    const fn structured_output_inner(&self) -> Option<&StructuredOutputDescriptor> {
        self.state.structured_output.as_ref()
    }
}

impl AttachServingRequest {
    const fn new_inner(serving_instance_id: ServingInstanceId) -> Self {
        Self {
            serving_instance_id,
        }
    }

    #[must_use]
    const fn serving_instance_id_inner(&self) -> &ServingInstanceId {
        &self.serving_instance_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StartServingRequestState {
    scope: ScopeId,
    serving_instance_id: ServingInstanceId,
    artifact: ModelArtifactBinding,
    deadline: Deadline,
}

impl StartServingRequest {
    const fn new_inner(
        scope: ScopeId,
        serving_instance_id: ServingInstanceId,
        artifact: ModelArtifactBinding,
        deadline: Deadline,
    ) -> Self {
        Self {
            state: StartServingRequestState {
            scope,
            serving_instance_id,
            artifact,
            deadline,
            },
        }
    }

    #[must_use]
    const fn scope_inner(&self) -> &ScopeId {
        &self.state.scope
    }

    #[must_use]
    const fn serving_instance_id_inner(&self) -> &ServingInstanceId {
        &self.state.serving_instance_id
    }

    #[must_use]
    const fn artifact_inner(&self) -> &ModelArtifactBinding {
        &self.state.artifact
    }

    #[must_use]
    const fn deadline_inner(&self) -> Deadline {
        self.state.deadline
    }
}
