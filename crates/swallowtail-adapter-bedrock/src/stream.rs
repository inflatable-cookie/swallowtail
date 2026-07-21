use aws_sdk_bedrockruntime::operation::converse_stream::ConverseStreamError;
use aws_sdk_bedrockruntime::types::error::ConverseStreamOutputError;
use aws_sdk_bedrockruntime::types::{ContentBlockDelta, ConverseStreamOutput, StopReason};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopKind {
    EndTurn,
    MaximumTokens,
    StopSequence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenUsage {
    pub input: u64,
    pub output: u64,
    pub total: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamUpdate {
    MessageStarted,
    TextDelta(String),
    ContentBlockStopped,
    MessageStopped(StopKind),
    Usage(TokenUsage),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeFailure {
    EventOutOfOrder,
    MissingRequiredField,
    UnsupportedSemanticEvent,
    UnknownSdkVariant,
    UsageOutOfRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderFailureKind {
    AuthenticationOrPermissionDenied,
    InvalidRequest,
    ModelUnavailable,
    ModelTimedOut,
    RateLimited,
    ResourceNotFound,
    ProviderOverloaded,
    ProviderFailed,
    ProtocolFailed,
    TransportFailed,
}

#[derive(Default)]
pub struct StreamDecoder {
    started: bool,
    content_stopped: bool,
    message_stopped: bool,
    usage_seen: bool,
}

impl StreamDecoder {
    pub fn push(&mut self, event: ConverseStreamOutput) -> Result<StreamUpdate, DecodeFailure> {
        if self.usage_seen {
            return Err(DecodeFailure::EventOutOfOrder);
        }
        if event.is_unknown() {
            return Err(DecodeFailure::UnknownSdkVariant);
        }
        match event {
            ConverseStreamOutput::MessageStart(_) if !self.started => {
                self.started = true;
                Ok(StreamUpdate::MessageStarted)
            }
            ConverseStreamOutput::ContentBlockDelta(event)
                if self.started && !self.content_stopped =>
            {
                let delta = event.delta().ok_or(DecodeFailure::MissingRequiredField)?;
                if delta.is_unknown() {
                    return Err(DecodeFailure::UnknownSdkVariant);
                }
                match delta {
                    ContentBlockDelta::Text(text) => Ok(StreamUpdate::TextDelta(text.clone())),
                    ContentBlockDelta::Citation(_)
                    | ContentBlockDelta::Image(_)
                    | ContentBlockDelta::ReasoningContent(_)
                    | ContentBlockDelta::ToolResult(_)
                    | ContentBlockDelta::ToolUse(_) => Err(DecodeFailure::UnsupportedSemanticEvent),
                    _ => Err(DecodeFailure::UnknownSdkVariant),
                }
            }
            ConverseStreamOutput::ContentBlockStop(_) if self.started && !self.content_stopped => {
                self.content_stopped = true;
                Ok(StreamUpdate::ContentBlockStopped)
            }
            ConverseStreamOutput::MessageStop(event)
                if self.content_stopped && !self.message_stopped =>
            {
                let reason = event.stop_reason();
                if !StopReason::values().contains(&reason.as_str()) {
                    return Err(DecodeFailure::UnknownSdkVariant);
                }
                let stop = match reason {
                    StopReason::EndTurn => StopKind::EndTurn,
                    StopReason::MaxTokens => StopKind::MaximumTokens,
                    StopReason::StopSequence => StopKind::StopSequence,
                    _ => return Err(DecodeFailure::UnsupportedSemanticEvent),
                };
                self.message_stopped = true;
                Ok(StreamUpdate::MessageStopped(stop))
            }
            ConverseStreamOutput::Metadata(event) if self.message_stopped => {
                let usage = event.usage().ok_or(DecodeFailure::MissingRequiredField)?;
                let input = u64::try_from(usage.input_tokens())
                    .map_err(|_| DecodeFailure::UsageOutOfRange)?;
                let output = u64::try_from(usage.output_tokens())
                    .map_err(|_| DecodeFailure::UsageOutOfRange)?;
                let total = u64::try_from(usage.total_tokens())
                    .map_err(|_| DecodeFailure::UsageOutOfRange)?;
                self.usage_seen = true;
                Ok(StreamUpdate::Usage(TokenUsage {
                    input,
                    output,
                    total,
                }))
            }
            ConverseStreamOutput::ContentBlockStart(_) => {
                Err(DecodeFailure::UnsupportedSemanticEvent)
            }
            ConverseStreamOutput::ContentBlockDelta(_)
            | ConverseStreamOutput::ContentBlockStop(_)
            | ConverseStreamOutput::MessageStart(_)
            | ConverseStreamOutput::MessageStop(_)
            | ConverseStreamOutput::Metadata(_) => Err(DecodeFailure::EventOutOfOrder),
            _ => Err(DecodeFailure::UnknownSdkVariant),
        }
    }

    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.usage_seen
    }
}

#[must_use]
pub fn classify_converse_failure(error: &ConverseStreamError) -> ProviderFailureKind {
    match error {
        ConverseStreamError::AccessDeniedException(_) => {
            ProviderFailureKind::AuthenticationOrPermissionDenied
        }
        ConverseStreamError::ValidationException(_) => ProviderFailureKind::InvalidRequest,
        ConverseStreamError::ModelNotReadyException(_)
        | ConverseStreamError::ModelErrorException(_) => ProviderFailureKind::ModelUnavailable,
        ConverseStreamError::ModelTimeoutException(_) => ProviderFailureKind::ModelTimedOut,
        ConverseStreamError::ThrottlingException(_) => ProviderFailureKind::RateLimited,
        ConverseStreamError::ResourceNotFoundException(_) => ProviderFailureKind::ResourceNotFound,
        ConverseStreamError::ServiceUnavailableException(_) => {
            ProviderFailureKind::ProviderOverloaded
        }
        ConverseStreamError::InternalServerException(_)
        | ConverseStreamError::ModelStreamErrorException(_) => ProviderFailureKind::ProviderFailed,
        _ => ProviderFailureKind::ProtocolFailed,
    }
}

#[must_use]
pub fn classify_output_failure(error: &ConverseStreamOutputError) -> ProviderFailureKind {
    match error {
        ConverseStreamOutputError::ValidationException(_) => ProviderFailureKind::InvalidRequest,
        ConverseStreamOutputError::ThrottlingException(_) => ProviderFailureKind::RateLimited,
        ConverseStreamOutputError::ServiceUnavailableException(_) => {
            ProviderFailureKind::ProviderOverloaded
        }
        ConverseStreamOutputError::InternalServerException(_)
        | ConverseStreamOutputError::ModelStreamErrorException(_) => {
            ProviderFailureKind::ProviderFailed
        }
        _ => ProviderFailureKind::ProtocolFailed,
    }
}
