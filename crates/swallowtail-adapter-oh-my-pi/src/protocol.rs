use std::error::Error;
use std::fmt;

mod wire;

pub(crate) use wire::{
    OhMyPiAgentEvent, OhMyPiRpcDecoder, OhMyPiRpcRecord, OhMyPiUiDialog, OhMyPiUiDialogMethod,
    OhMyPiUiDisplay, OhMyPiUiDisplayKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OhMyPiRpcRecordKind {
    Response,
    AgentEvent,
    ExtensionUiDialog,
    ExtensionUiDisplay,
    Lifecycle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OhMyPiRpcProtocolFailureKind {
    MissingLfDelimiter,
    EmptyRecord,
    MalformedJson,
    MissingType,
    UnknownRecord,
    InvalidResponse,
    InvalidUiRequest,
    RecordTooLarge,
    InvalidChunk,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OhMyPiRpcProtocolFailure {
    kind: OhMyPiRpcProtocolFailureKind,
}

impl OhMyPiRpcProtocolFailure {
    const fn new(kind: OhMyPiRpcProtocolFailureKind) -> Self {
        Self { kind }
    }

    #[must_use]
    pub const fn kind(&self) -> OhMyPiRpcProtocolFailureKind {
        self.kind
    }
}

impl fmt::Display for OhMyPiRpcProtocolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("OhMyPi RPC record did not match the qualified protocol")
    }
}

impl Error for OhMyPiRpcProtocolFailure {}

/// Decodes complete LF-delimited stdout records. A partial final record fails.
pub fn decode_records(bytes: &[u8]) -> Result<Vec<OhMyPiRpcRecordKind>, OhMyPiRpcProtocolFailure> {
    let mut decoder = OhMyPiRpcDecoder::new();
    let records = decoder.push(bytes)?;
    decoder.finish()?;
    Ok(records.into_iter().map(|record| record.kind()).collect())
}

#[cfg(test)]
mod tests {
    use super::{OhMyPiRpcProtocolFailureKind, OhMyPiRpcRecordKind, decode_records};
    use base64::Engine as _;

    const FIXTURES: &str = "../tests/fixtures/oh-my-pi-rpc-17.2.9";

    #[test]
    fn qualified_response_event_and_ui_corpora_decode() {
        let responses = decode_records(include_bytes!(
            "../tests/fixtures/oh-my-pi-rpc-17.2.9/responses.jsonl"
        ))
        .unwrap();
        let events = decode_records(include_bytes!(
            "../tests/fixtures/oh-my-pi-rpc-17.2.9/events.jsonl"
        ))
        .unwrap();
        let ui = decode_records(include_bytes!(
            "../tests/fixtures/oh-my-pi-rpc-17.2.9/ui.jsonl"
        ))
        .unwrap();
        let provider_error = decode_records(include_bytes!(
            "../tests/fixtures/oh-my-pi-rpc-17.2.9/provider-error.jsonl"
        ))
        .unwrap();
        let close = decode_records(include_bytes!(
            "../tests/fixtures/oh-my-pi-rpc-17.2.9/close.jsonl"
        ))
        .unwrap();

        assert!(
            responses
                .iter()
                .all(|kind| *kind == OhMyPiRpcRecordKind::Response)
        );
        assert_eq!(&events[..4], [OhMyPiRpcRecordKind::Lifecycle; 4]);
        assert!(
            events[4..]
                .iter()
                .all(|kind| *kind == OhMyPiRpcRecordKind::AgentEvent)
        );
        assert!(ui.contains(&OhMyPiRpcRecordKind::ExtensionUiDialog));
        assert!(ui.contains(&OhMyPiRpcRecordKind::ExtensionUiDisplay));
        assert!(
            provider_error
                .iter()
                .all(|kind| *kind == OhMyPiRpcRecordKind::AgentEvent)
        );
        assert_eq!(close, vec![OhMyPiRpcRecordKind::AgentEvent]);
    }

    #[test]
    fn safe_unknown_is_preserved_while_malformed_partial_and_invalid_ui_fail_closed() {
        assert_eq!(
            decode_records(include_bytes!(
                "../tests/fixtures/oh-my-pi-rpc-17.2.9/unknown.jsonl"
            ))
            .expect("bounded unknown event decodes"),
            [OhMyPiRpcRecordKind::AgentEvent]
        );
        for (bytes, expected) in [
            (
                include_bytes!("../tests/fixtures/oh-my-pi-rpc-17.2.9/malformed.jsonl").as_slice(),
                OhMyPiRpcProtocolFailureKind::MalformedJson,
            ),
            (
                &include_bytes!("../tests/fixtures/oh-my-pi-rpc-17.2.9/disconnect.jsonl")
                    [..include_bytes!("../tests/fixtures/oh-my-pi-rpc-17.2.9/disconnect.jsonl")
                        .len()
                        - 1],
                OhMyPiRpcProtocolFailureKind::MissingLfDelimiter,
            ),
            (
                include_bytes!("../tests/fixtures/oh-my-pi-rpc-17.2.9/invalid-ui.jsonl").as_slice(),
                OhMyPiRpcProtocolFailureKind::InvalidUiRequest,
            ),
        ] {
            assert_eq!(decode_records(bytes).unwrap_err().kind(), expected);
        }
    }

    #[test]
    fn malformed_usage_components_fail_closed() {
        for bytes in [
            br#"{"type":"message_end","message":{"role":"assistant","usage":{"input":-1,"output":1,"cacheRead":0,"cacheWrite":0},"stopReason":"stop"}}"#.as_slice(),
            br#"{"type":"message_end","message":{"role":"assistant","usage":{"input":1.5,"output":1,"cacheRead":0,"cacheWrite":0},"stopReason":"stop"}}"#.as_slice(),
            br#"{"type":"message_end","message":{"role":"assistant","usage":{"input":1,"output":1,"cacheRead":0},"stopReason":"stop"}}"#.as_slice(),
        ] {
            let mut record = bytes.to_vec();
            record.push(b'\n');
            assert_eq!(
                decode_records(&record).expect_err("usage rejects").kind(),
                OhMyPiRpcProtocolFailureKind::UnknownRecord
            );
        }
    }

    #[test]
    fn rpc_v2_chunks_reassemble_one_bounded_logical_record() {
        let logical = serde_json::to_vec(&serde_json::json!({
            "type": "future_semantic_event",
            "payload": "x".repeat(1_100_000)
        }))
        .expect("logical frame serializes");
        let frames = chunk_frames(&logical);
        assert!(frames.lines().all(|line| line.len() < 1024 * 1024));
        assert_eq!(
            decode_records(frames.as_bytes()).expect("chunked record decodes"),
            [OhMyPiRpcRecordKind::AgentEvent]
        );

        let first = frames.lines().next().expect("first chunk");
        let incomplete = format!("{first}\n");
        assert_eq!(
            decode_records(incomplete.as_bytes())
                .expect_err("incomplete chunks fail")
                .kind(),
            OhMyPiRpcProtocolFailureKind::InvalidChunk
        );

        let mut reordered: serde_json::Value = serde_json::from_str(first).expect("chunk is JSON");
        reordered["index"] = 1.into();
        let reordered = format!("{}\n", serde_json::to_string(&reordered).unwrap());
        assert_eq!(
            decode_records(reordered.as_bytes())
                .expect_err("reordered chunks fail")
                .kind(),
            OhMyPiRpcProtocolFailureKind::InvalidChunk
        );
    }

    fn chunk_frames(logical: &[u8]) -> String {
        let chunks = logical.chunks(600_000).collect::<Vec<_>>();
        chunks
            .iter()
            .enumerate()
            .map(|(index, chunk)| {
                serde_json::json!({
                    "type": "rpc_chunk",
                    "chunkId": "fixture-chunk",
                    "index": index,
                    "count": chunks.len(),
                    "byteLength": logical.len(),
                    "data": base64::engine::general_purpose::STANDARD.encode(chunk)
                })
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    #[test]
    fn metadata_and_outbound_commands_are_valid_json_without_private_values() {
        let protocol: serde_json::Value = serde_json::from_str(include_str!(
            "../tests/fixtures/oh-my-pi-rpc-17.2.9/protocol.json"
        ))
        .unwrap();
        assert_eq!(protocol["package_version"], "17.2.9");
        assert_eq!(protocol["compatibility_claim"], "exact_qualification_point");
        assert!(
            protocol["argv"]
                .as_array()
                .unwrap()
                .contains(&"--no-session".into())
        );

        for line in include_str!("../tests/fixtures/oh-my-pi-rpc-17.2.9/commands.jsonl").lines() {
            serde_json::from_str::<serde_json::Value>(line).unwrap();
        }
        assert!(
            !include_str!("../tests/fixtures/oh-my-pi-rpc-17.2.9/README.md").contains("private")
        );
        assert!(FIXTURES.ends_with("oh-my-pi-rpc-17.2.9"));
    }
}
