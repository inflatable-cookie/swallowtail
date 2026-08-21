use std::error::Error;
use std::fmt;

use super::wire;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Qualified top-level record categories emitted by the Pi SDK sidecar.
pub enum PiSdkSidecarRecordKind {
    /// Correlated command response.
    Response,
    /// Turn, message, tool, usage, progress, or replay event.
    Event,
    /// Unrecoverable sidecar failure; the process exits afterwards.
    Terminal,
    /// Redacted non-fatal diagnostic observation.
    Diagnostic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Safe failure categories from bounded sidecar record decoding.
pub enum PiSdkSidecarProtocolFailureKind {
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
    /// A terminal record did not match the qualified shape.
    InvalidTerminal,
    /// A diagnostic record did not match the qualified shape.
    InvalidDiagnostic,
    /// A single record exceeded the decoder bound.
    RecordTooLarge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Redacted protocol failure returned by the public corpus decoder.
pub struct PiSdkSidecarProtocolFailure {
    kind: PiSdkSidecarProtocolFailureKind,
}

impl PiSdkSidecarProtocolFailure {
    pub(crate) const fn new(kind: PiSdkSidecarProtocolFailureKind) -> Self {
        Self { kind }
    }

    /// Returns the safe protocol failure category.
    #[must_use]
    pub const fn kind(&self) -> PiSdkSidecarProtocolFailureKind {
        self.kind
    }
}

impl fmt::Display for PiSdkSidecarProtocolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Pi SDK sidecar record did not match the qualified protocol")
    }
}

impl Error for PiSdkSidecarProtocolFailure {}

/// Decodes complete LF-delimited sidecar records. A partial final record
/// fails.
pub fn decode_records(
    bytes: &[u8],
) -> Result<Vec<PiSdkSidecarRecordKind>, PiSdkSidecarProtocolFailure> {
    if !bytes.ends_with(b"\n") {
        return Err(PiSdkSidecarProtocolFailure::new(
            PiSdkSidecarProtocolFailureKind::MissingLfDelimiter,
        ));
    }
    bytes[..bytes.len() - 1]
        .split(|byte| *byte == b'\n')
        .map(|line| wire::decode_record(line).map(|record| record.kind()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{PiSdkSidecarProtocolFailureKind, PiSdkSidecarRecordKind, decode_records};
    use crate::sidecar::{
        PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_ENTRY_FILE, PI_SDK_SIDECAR_NODE_RUNTIME,
        PI_SDK_SIDECAR_SDK_PACKAGE, PI_SDK_SIDECAR_SDK_VERSION, PI_SDK_SIDECAR_SOURCE,
        PI_SDK_SIDECAR_SOURCE_TAG, PI_SDK_SIDECAR_WIRE,
    };

    const FIXTURES: &str = "../../tests/fixtures/pi-sdk-sidecar-v1";

    macro_rules! fixture {
        ($name:literal) => {
            include_bytes!(concat!("../../tests/fixtures/pi-sdk-sidecar-v1/", $name))
        };
    }

    #[test]
    fn qualified_corpora_decode() {
        let responses = decode_records(fixture!("responses.jsonl")).unwrap();
        assert_eq!(responses.len(), 9);
        assert!(
            responses
                .iter()
                .all(|kind| *kind == PiSdkSidecarRecordKind::Response)
        );
        let events = decode_records(fixture!("events.jsonl")).unwrap();
        assert!(
            events
                .iter()
                .all(|kind| *kind == PiSdkSidecarRecordKind::Event)
        );
        let replay = decode_records(fixture!("replay.jsonl")).unwrap();
        assert_eq!(replay.len(), 4);
        assert!(
            replay
                .iter()
                .all(|kind| *kind == PiSdkSidecarRecordKind::Event)
        );
        assert_eq!(
            decode_records(fixture!("terminal.jsonl")).unwrap(),
            [PiSdkSidecarRecordKind::Terminal]
        );
        assert_eq!(
            decode_records(fixture!("diagnostics.jsonl")).unwrap(),
            [
                PiSdkSidecarRecordKind::Diagnostic,
                PiSdkSidecarRecordKind::Diagnostic
            ]
        );
    }

    #[test]
    fn unknown_malformed_and_partial_records_fail_closed() {
        for (bytes, expected) in [
            (
                fixture!("unknown.jsonl").as_slice(),
                PiSdkSidecarProtocolFailureKind::UnknownRecord,
            ),
            (
                fixture!("malformed.jsonl").as_slice(),
                PiSdkSidecarProtocolFailureKind::MalformedJson,
            ),
            (
                fixture!("disconnect.jsonl").as_slice(),
                PiSdkSidecarProtocolFailureKind::MissingLfDelimiter,
            ),
        ] {
            assert_eq!(decode_records(bytes).unwrap_err().kind(), expected);
        }
    }

    #[test]
    fn streaming_decoder_bounds_oversized_and_partial_records() {
        use super::wire::PiSdkSidecarDecoder;

        let mut decoder = PiSdkSidecarDecoder::new();
        let mut oversized = vec![b'x'; 1024 * 1024 + 1];
        oversized.push(b'\n');
        let oversized_kind = match decoder.push(&oversized) {
            Err(failure) => failure.kind(),
            Ok(_) => panic!("oversized record decoded unexpectedly"),
        };
        assert_eq!(
            oversized_kind,
            PiSdkSidecarProtocolFailureKind::RecordTooLarge
        );

        let mut decoder = PiSdkSidecarDecoder::new();
        decoder.push(b"{\"type\":\"event\"").unwrap();
        let partial_kind = match decoder.finish() {
            Err(failure) => failure.kind(),
            Ok(()) => panic!("partial record finished unexpectedly"),
        };
        assert_eq!(
            partial_kind,
            PiSdkSidecarProtocolFailureKind::MissingLfDelimiter
        );

        let mut decoder = PiSdkSidecarDecoder::new();
        let records = decoder
            .push(b"{\"type\":\"event\",\"event\":\"progress\"}\n{\"type\":\"event\",")
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].kind(),
            PiSdkSidecarRecordKind::Event,
            "complete record decodes while a partial tail is held"
        );
    }

    #[test]
    fn corpus_identity_matches_the_frozen_sidecar_identity() {
        let protocol: serde_json::Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/pi-sdk-sidecar-v1/protocol.json"
        ))
        .unwrap();
        assert_eq!(protocol["wire"], PI_SDK_SIDECAR_WIRE);
        assert_eq!(protocol["behavior_revision"], PI_SDK_SIDECAR_BEHAVIOR);
        assert_eq!(protocol["sdk_package"], PI_SDK_SIDECAR_SDK_PACKAGE);
        assert_eq!(protocol["sdk_version"], PI_SDK_SIDECAR_SDK_VERSION);
        assert_eq!(protocol["node_runtime"], PI_SDK_SIDECAR_NODE_RUNTIME);
        assert_eq!(protocol["sidecar_entry_file"], PI_SDK_SIDECAR_ENTRY_FILE);
        assert_eq!(protocol["compatibility_claim"], "qualified_only_one_point");
        assert!(
            PI_SDK_SIDECAR_SOURCE_TAG
                .starts_with(protocol["sidecar_source_tag_prefix"].as_str().unwrap())
        );
        assert!(PI_SDK_SIDECAR_SOURCE.contains(PI_SDK_SIDECAR_WIRE));
        assert!(PI_SDK_SIDECAR_SOURCE.contains(PI_SDK_SIDECAR_BEHAVIOR));
        assert!(PI_SDK_SIDECAR_SOURCE.contains(PI_SDK_SIDECAR_SDK_VERSION));
        assert!(FIXTURES.ends_with("pi-sdk-sidecar-v1"));
    }

    #[test]
    fn outbound_command_corpus_is_valid_and_covers_every_command() {
        let commands: Vec<serde_json::Value> =
            include_str!("../../tests/fixtures/pi-sdk-sidecar-v1/commands.jsonl")
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
        let names: Vec<&str> = commands
            .iter()
            .map(|command| {
                assert_eq!(command["type"], "command");
                assert!(command["id"].as_str().unwrap().len() <= 128);
                command["command"].as_str().unwrap()
            })
            .collect();
        for qualified in [
            "bootstrap",
            "session_new",
            "session_switch",
            "session_replay",
            "prompt",
            "steer",
            "follow_up",
            "abort",
            "state",
            "close",
        ] {
            assert!(names.contains(&qualified), "missing command {qualified}");
        }
        assert_eq!(commands.len(), 12);
        let bootstrap = &commands[0];
        assert_eq!(bootstrap["command"], "bootstrap");
        let params: Vec<&str> = bootstrap["params"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(params, ["cwd", "model", "provider", "thinkingLevel"]);
        let catalogue = &commands[1];
        assert_eq!(catalogue["command"], "bootstrap");
        assert_eq!(catalogue["params"]["catalogueOnly"], true);
        let protocol: serde_json::Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/pi-sdk-sidecar-v1/protocol.json"
        ))
        .unwrap();
        assert_eq!(
            protocol["bootstrap_params"].as_array().unwrap(),
            &["cwd", "provider", "model", "thinkingLevel", "catalogueOnly"]
        );
        assert_eq!(
            protocol["environment"]["sdk_module"],
            "PI_SDK_SIDECAR_SDK_MODULE"
        );
        assert_eq!(
            protocol["environment"]["agent_dir"],
            "PI_SDK_SIDECAR_AGENT_DIR"
        );
        assert_eq!(
            protocol["environment"]["session_dir"],
            "PI_SDK_SIDECAR_SESSION_DIR"
        );
        assert_eq!(protocol["bounds"]["catalogue_models"], 256);
        assert_eq!(protocol["bounds"]["catalogue_text_bytes"], 256);
        assert_eq!(
            protocol["catalogue_bootstrap_params"].as_array().unwrap(),
            &["catalogueOnly"]
        );
    }
}
