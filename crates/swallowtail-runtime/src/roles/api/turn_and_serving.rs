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

