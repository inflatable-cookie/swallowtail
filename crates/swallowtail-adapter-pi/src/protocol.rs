use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PiRpcRecordKind {
    Response,
    AgentEvent,
    ExtensionUiDialog,
    ExtensionUiDisplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PiRpcProtocolFailureKind {
    MissingLfDelimiter,
    EmptyRecord,
    MalformedJson,
    MissingType,
    UnknownRecord,
    InvalidResponse,
    InvalidUiRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PiRpcProtocolFailure {
    kind: PiRpcProtocolFailureKind,
}

impl PiRpcProtocolFailure {
    const fn new(kind: PiRpcProtocolFailureKind) -> Self {
        Self { kind }
    }

    #[must_use]
    pub const fn kind(&self) -> PiRpcProtocolFailureKind {
        self.kind
    }
}

impl fmt::Display for PiRpcProtocolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Pi RPC record did not match the qualified protocol")
    }
}

impl Error for PiRpcProtocolFailure {}

/// Decodes complete LF-delimited stdout records. A partial final record fails.
pub fn decode_records(bytes: &[u8]) -> Result<Vec<PiRpcRecordKind>, PiRpcProtocolFailure> {
    if !bytes.ends_with(b"\n") {
        return Err(PiRpcProtocolFailure::new(
            PiRpcProtocolFailureKind::MissingLfDelimiter,
        ));
    }
    bytes[..bytes.len() - 1]
        .split(|byte| *byte == b'\n')
        .map(decode_record)
        .collect()
}

fn decode_record(bytes: &[u8]) -> Result<PiRpcRecordKind, PiRpcProtocolFailure> {
    if bytes.is_empty() {
        return Err(PiRpcProtocolFailure::new(
            PiRpcProtocolFailureKind::EmptyRecord,
        ));
    }
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| PiRpcProtocolFailure::new(PiRpcProtocolFailureKind::MalformedJson))?;
    let record_type = value
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| PiRpcProtocolFailure::new(PiRpcProtocolFailureKind::MissingType))?;
    match record_type {
        "response" => decode_response(&value),
        "extension_ui_request" => decode_ui_request(&value),
        "agent_start"
        | "agent_end"
        | "agent_settled"
        | "turn_start"
        | "turn_end"
        | "message_start"
        | "message_update"
        | "message_end"
        | "tool_execution_start"
        | "tool_execution_update"
        | "tool_execution_end"
        | "queue_update"
        | "auto_retry_start"
        | "auto_retry_end"
        | "extension_error" => Ok(PiRpcRecordKind::AgentEvent),
        _ => Err(PiRpcProtocolFailure::new(
            PiRpcProtocolFailureKind::UnknownRecord,
        )),
    }
}

fn decode_response(value: &Value) -> Result<PiRpcRecordKind, PiRpcProtocolFailure> {
    if value.get("command").and_then(Value::as_str).is_none()
        || value.get("success").and_then(Value::as_bool).is_none()
    {
        return Err(PiRpcProtocolFailure::new(
            PiRpcProtocolFailureKind::InvalidResponse,
        ));
    }
    Ok(PiRpcRecordKind::Response)
}

fn decode_ui_request(value: &Value) -> Result<PiRpcRecordKind, PiRpcProtocolFailure> {
    if value.get("id").and_then(Value::as_str).is_none() {
        return Err(PiRpcProtocolFailure::new(
            PiRpcProtocolFailureKind::InvalidUiRequest,
        ));
    }
    match value.get("method").and_then(Value::as_str) {
        Some("select" | "confirm" | "input" | "editor") => Ok(PiRpcRecordKind::ExtensionUiDialog),
        Some("notify" | "setStatus" | "setWidget" | "setTitle" | "set_editor_text") => {
            Ok(PiRpcRecordKind::ExtensionUiDisplay)
        }
        _ => Err(PiRpcProtocolFailure::new(
            PiRpcProtocolFailureKind::InvalidUiRequest,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{PiRpcProtocolFailureKind, PiRpcRecordKind, decode_records};

    const FIXTURES: &str = "../tests/fixtures/pi-rpc-0.80.10";

    #[test]
    fn qualified_response_event_and_ui_corpora_decode() {
        let responses = decode_records(include_bytes!(
            "../tests/fixtures/pi-rpc-0.80.10/responses.jsonl"
        ))
        .unwrap();
        let events = decode_records(include_bytes!(
            "../tests/fixtures/pi-rpc-0.80.10/events.jsonl"
        ))
        .unwrap();
        let ui =
            decode_records(include_bytes!("../tests/fixtures/pi-rpc-0.80.10/ui.jsonl")).unwrap();
        let provider_error = decode_records(include_bytes!(
            "../tests/fixtures/pi-rpc-0.80.10/provider-error.jsonl"
        ))
        .unwrap();
        let close = decode_records(include_bytes!(
            "../tests/fixtures/pi-rpc-0.80.10/close.jsonl"
        ))
        .unwrap();

        assert!(
            responses
                .iter()
                .all(|kind| *kind == PiRpcRecordKind::Response)
        );
        assert!(
            events
                .iter()
                .all(|kind| *kind == PiRpcRecordKind::AgentEvent)
        );
        assert!(ui.contains(&PiRpcRecordKind::ExtensionUiDialog));
        assert!(ui.contains(&PiRpcRecordKind::ExtensionUiDisplay));
        assert!(
            provider_error
                .iter()
                .all(|kind| *kind == PiRpcRecordKind::AgentEvent)
        );
        assert_eq!(close, vec![PiRpcRecordKind::AgentEvent]);
    }

    #[test]
    fn malformed_unknown_partial_and_invalid_ui_fail_closed() {
        for (bytes, expected) in [
            (
                include_bytes!("../tests/fixtures/pi-rpc-0.80.10/malformed.jsonl").as_slice(),
                PiRpcProtocolFailureKind::MalformedJson,
            ),
            (
                include_bytes!("../tests/fixtures/pi-rpc-0.80.10/unknown.jsonl").as_slice(),
                PiRpcProtocolFailureKind::UnknownRecord,
            ),
            (
                &include_bytes!("../tests/fixtures/pi-rpc-0.80.10/disconnect.jsonl")
                    [..include_bytes!("../tests/fixtures/pi-rpc-0.80.10/disconnect.jsonl").len()
                        - 1],
                PiRpcProtocolFailureKind::MissingLfDelimiter,
            ),
            (
                include_bytes!("../tests/fixtures/pi-rpc-0.80.10/invalid-ui.jsonl").as_slice(),
                PiRpcProtocolFailureKind::InvalidUiRequest,
            ),
        ] {
            assert_eq!(decode_records(bytes).unwrap_err().kind(), expected);
        }
    }

    #[test]
    fn metadata_and_outbound_commands_are_valid_json_without_private_values() {
        let protocol: serde_json::Value = serde_json::from_str(include_str!(
            "../tests/fixtures/pi-rpc-0.80.10/protocol.json"
        ))
        .unwrap();
        assert_eq!(protocol["package_version"], "0.80.10");
        assert_eq!(protocol["compatibility_claim"], "exact_qualification_point");
        assert!(
            protocol["argv"]
                .as_array()
                .unwrap()
                .contains(&"--offline".into())
        );

        for line in include_str!("../tests/fixtures/pi-rpc-0.80.10/commands.jsonl").lines() {
            serde_json::from_str::<serde_json::Value>(line).unwrap();
        }
        assert!(!include_str!("../tests/fixtures/pi-rpc-0.80.10/README.md").contains("private"));
        assert!(FIXTURES.ends_with("pi-rpc-0.80.10"));
    }
}
