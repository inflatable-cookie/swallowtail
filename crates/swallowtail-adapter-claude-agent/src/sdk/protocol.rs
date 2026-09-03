//! Bounded public decoder for qualified Claude Agent SDK sidecar records.
//!
//! Consumers use this to verify a captured corpus against the exact private
//! wire without reaching the driver. It returns record categories and safe
//! failure categories only: no provider payload, credential, path, or raw SDK
//! value is exposed.

use std::error::Error;
use std::fmt;

use super::wire;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Qualified top-level record categories emitted by the sidecar.
pub enum ClaudeAgentSdkRecordKind {
    /// Correlated command response.
    Response,
    /// Turn, output, or tool-activity event.
    Event,
    /// Correlated `canUseTool` admission request awaiting a host decision.
    Callback,
    /// Unrecoverable sidecar failure; the process exits afterwards.
    Terminal,
    /// Redacted non-fatal diagnostic observation.
    Diagnostic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Safe failure categories from bounded sidecar record decoding.
pub enum ClaudeAgentSdkProtocolFailureKind {
    /// The final record was not LF-terminated.
    MissingLfDelimiter,
    /// An LF-delimited record was empty.
    EmptyRecord,
    /// A record was not valid JSON.
    MalformedJson,
    /// A record omitted its required type discriminator.
    MissingType,
    /// A top-level record type or event name was outside the qualified wire.
    UnknownRecord,
    /// A response did not match the qualified shape.
    InvalidResponse,
    /// An event did not match the qualified shape.
    InvalidEvent,
    /// A callback did not match the qualified shape.
    InvalidCallback,
    /// A terminal record did not match the qualified shape.
    InvalidTerminal,
    /// A diagnostic record did not match the qualified shape.
    InvalidDiagnostic,
    /// A single record exceeded the decoder bound.
    RecordTooLarge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Redacted protocol failure returned by the public corpus decoder.
pub struct ClaudeAgentSdkProtocolFailure {
    kind: ClaudeAgentSdkProtocolFailureKind,
}

impl ClaudeAgentSdkProtocolFailure {
    pub(crate) const fn new(kind: ClaudeAgentSdkProtocolFailureKind) -> Self {
        Self { kind }
    }

    /// Returns the safe protocol failure category.
    #[must_use]
    pub const fn kind(&self) -> ClaudeAgentSdkProtocolFailureKind {
        self.kind
    }
}

impl fmt::Display for ClaudeAgentSdkProtocolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Claude Agent SDK sidecar record did not match the qualified protocol")
    }
}

impl Error for ClaudeAgentSdkProtocolFailure {}

/// Decodes complete LF-delimited sidecar records. A partial final record
/// fails; a truncated stream is never a successful decode.
pub fn decode_records(
    bytes: &[u8],
) -> Result<Vec<ClaudeAgentSdkRecordKind>, ClaudeAgentSdkProtocolFailure> {
    if !bytes.ends_with(b"\n") {
        return Err(ClaudeAgentSdkProtocolFailure::new(
            ClaudeAgentSdkProtocolFailureKind::MissingLfDelimiter,
        ));
    }
    bytes[..bytes.len() - 1]
        .split(|byte| *byte == b'\n')
        .map(|line| wire::decode_record(line).map(|record| record.kind()))
        .collect()
}

#[cfg(test)]
mod protocol_tests;
