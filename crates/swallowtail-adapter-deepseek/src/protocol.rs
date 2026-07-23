use std::error::Error;
use std::fmt;

mod http;
mod private;
mod request;
mod response;
mod stream;

pub(crate) use http::{HttpRequest, HttpResponse, Method, parse_models, require_success};
pub(crate) use private::PrivateContinuation;
pub(crate) use request::{ToolSpec, encode_after_tool, encode_initial, encode_later_user};
pub(crate) use response::{
    ProviderFailureKind, ToolAttempt, Usage, classify_failure, parse_tool_attempt,
};
pub(crate) use stream::{FinalAttempt, FinalStreamParser, FinalStreamUpdate};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ProtocolFailureKind {
    InvalidStructure,
    BoundExceeded,
    ModelMismatch,
    UnknownSemanticField,
    IncompleteStream,
    ProviderFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ProtocolFailure {
    kind: ProtocolFailureKind,
}

impl ProtocolFailure {
    pub(super) const fn new(kind: ProtocolFailureKind) -> Self {
        Self { kind }
    }

    pub(super) const fn kind(&self) -> ProtocolFailureKind {
        self.kind
    }
}

impl fmt::Display for ProtocolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DeepSeek payload did not match the qualified protocol")
    }
}

impl Error for ProtocolFailure {}

#[cfg(test)]
mod tests;
