#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CallbackOperationId {
    Run(RuntimeRunId),
    Turn(RuntimeTurnId),
}

#[derive(Clone, Eq, PartialEq)]
pub struct CallbackPayload(Vec<u8>);

impl CallbackPayload {
    pub fn new(
        bytes: impl Into<Vec<u8>>,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let bytes = bytes.into();
        if bytes.len() > maximum_bytes {
            Err(InputLimitExceeded::new(
                "callback payload",
                maximum_bytes,
                bytes.len(),
            ))
        } else {
            Ok(Self(bytes))
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for CallbackPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CallbackPayload")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CallbackRequestKind {
    ToolCall {
        tool_name: String,
        arguments: CallbackPayload,
    },
    Extension(ProviderExtension),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackRequest {
    callback_id: CallbackId,
    operation_id: CallbackOperationId,
    event_sequence: u64,
    deadline: Option<Deadline>,
    provider_request_ref: Option<ProviderRequestRef>,
    kind: CallbackRequestKind,
}

impl CallbackRequest {
    pub fn tool_call(
        callback_id: CallbackId,
        turn_id: RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        tool_name: impl Into<String>,
        arguments: CallbackPayload,
    ) -> Result<Self, crate::InputValueRequired> {
        Self::tool_call_for(
            callback_id,
            CallbackOperationId::Turn(turn_id),
            event_sequence,
            deadline,
            tool_name,
            arguments,
        )
    }

    pub fn run_tool_call(
        callback_id: CallbackId,
        run_id: RuntimeRunId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        tool_name: impl Into<String>,
        arguments: CallbackPayload,
    ) -> Result<Self, crate::InputValueRequired> {
        Self::tool_call_for(
            callback_id,
            CallbackOperationId::Run(run_id),
            event_sequence,
            deadline,
            tool_name,
            arguments,
        )
    }

    fn tool_call_for(
        callback_id: CallbackId,
        operation_id: CallbackOperationId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        tool_name: impl Into<String>,
        arguments: CallbackPayload,
    ) -> Result<Self, crate::InputValueRequired> {
        Ok(Self {
            callback_id,
            operation_id,
            event_sequence,
            deadline,
            provider_request_ref: None,
            kind: CallbackRequestKind::ToolCall {
                tool_name: crate::input::required_text("callback tool name", tool_name)?,
                arguments,
            },
        })
    }

    pub fn extension(
        callback_id: CallbackId,
        turn_id: RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        extension: ProviderExtension,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        if extension.payload().len() > maximum_bytes {
            return Err(InputLimitExceeded::new(
                "callback extension payload",
                maximum_bytes,
                extension.payload().len(),
            ));
        }
        Ok(Self {
            callback_id,
            operation_id: CallbackOperationId::Turn(turn_id),
            event_sequence,
            deadline,
            provider_request_ref: None,
            kind: CallbackRequestKind::Extension(extension),
        })
    }

    #[must_use]
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> &CallbackOperationId {
        &self.operation_id
    }

    #[must_use]
    pub const fn turn_id(&self) -> Option<&RuntimeTurnId> {
        match &self.operation_id {
            CallbackOperationId::Turn(turn_id) => Some(turn_id),
            CallbackOperationId::Run(_) => None,
        }
    }

    #[must_use]
    pub const fn run_id(&self) -> Option<&RuntimeRunId> {
        match &self.operation_id {
            CallbackOperationId::Run(run_id) => Some(run_id),
            CallbackOperationId::Turn(_) => None,
        }
    }

    #[must_use]
    pub const fn event_sequence(&self) -> u64 {
        self.event_sequence
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    pub const fn kind(&self) -> &CallbackRequestKind {
        &self.kind
    }

    #[must_use]
    pub fn with_provider_request_ref(mut self, provider_request_ref: ProviderRequestRef) -> Self {
        self.provider_request_ref = Some(provider_request_ref);
        self
    }

    #[must_use]
    pub const fn provider_request_ref(&self) -> Option<&ProviderRequestRef> {
        self.provider_request_ref.as_ref()
    }
}
