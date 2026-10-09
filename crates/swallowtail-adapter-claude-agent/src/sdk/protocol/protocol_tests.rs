use super::{ClaudeAgentSdkProtocolFailureKind, ClaudeAgentSdkRecordKind, decode_records};
use crate::sdk::wire::ClaudeAgentSdkDecoder;
use crate::sdk::{
    CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_PACKAGE,
    CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE, CLAUDE_AGENT_SDK_SIDECAR_SOURCE,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_WIRE,
    ClaudeAgentSdkPackageNativePair,
};

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../../tests/fixtures/claude-agent-sdk-v1/",
            $name
        ))
    };
}

const PROTOCOL: &str = include_str!("../../../tests/fixtures/claude-agent-sdk-v1/protocol.json");

#[test]
fn qualified_corpora_decode() {
    let responses = decode_records(fixture!("responses.jsonl")).unwrap();
    assert_eq!(responses.len(), 6);
    assert!(
        responses
            .iter()
            .all(|kind| *kind == ClaudeAgentSdkRecordKind::Response)
    );
    let events = decode_records(fixture!("events.jsonl")).unwrap();
    assert_eq!(events.len(), 7);
    assert!(
        events
            .iter()
            .all(|kind| *kind == ClaudeAgentSdkRecordKind::Event)
    );
    assert_eq!(
        decode_records(fixture!("callbacks.jsonl")).unwrap(),
        [ClaudeAgentSdkRecordKind::Callback]
    );
    assert_eq!(
        decode_records(fixture!("terminal.jsonl")).unwrap(),
        [ClaudeAgentSdkRecordKind::Terminal]
    );
    assert_eq!(
        decode_records(fixture!("diagnostics.jsonl")).unwrap(),
        [
            ClaudeAgentSdkRecordKind::Diagnostic,
            ClaudeAgentSdkRecordKind::Diagnostic
        ]
    );
}

#[test]
fn unknown_malformed_and_partial_records_fail_closed() {
    for (bytes, expected) in [
        (
            fixture!("unknown.jsonl").as_slice(),
            ClaudeAgentSdkProtocolFailureKind::UnknownRecord,
        ),
        (
            fixture!("malformed.jsonl").as_slice(),
            ClaudeAgentSdkProtocolFailureKind::MalformedJson,
        ),
        (
            fixture!("disconnect.jsonl").as_slice(),
            ClaudeAgentSdkProtocolFailureKind::MissingLfDelimiter,
        ),
    ] {
        assert_eq!(decode_records(bytes).unwrap_err().kind(), expected);
    }
}

#[test]
fn streaming_decoder_bounds_oversized_and_partial_records() {
    let mut decoder = ClaudeAgentSdkDecoder::new();
    let mut oversized = vec![b'x'; 1024 * 1024 + 1];
    oversized.push(b'\n');
    assert_eq!(
        decoder.push(&oversized).err().map(|error| error.kind()),
        Some(ClaudeAgentSdkProtocolFailureKind::RecordTooLarge)
    );

    let mut decoder = ClaudeAgentSdkDecoder::new();
    decoder.push(b"{\"type\":\"event\"").unwrap();
    assert_eq!(
        decoder.finish().err().map(|error| error.kind()),
        Some(ClaudeAgentSdkProtocolFailureKind::MissingLfDelimiter)
    );

    let mut decoder = ClaudeAgentSdkDecoder::new();
    let records = decoder
        .push(b"{\"type\":\"event\",\"event\":\"progress\"}\n{\"type\":\"event\",")
        .unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0].kind(),
        ClaudeAgentSdkRecordKind::Event,
        "complete record decodes while a partial tail is held"
    );
}

#[test]
fn historical_v1_corpus_identity_stays_frozen_while_sidecar_map_extends() {
    let protocol: serde_json::Value = serde_json::from_str(PROTOCOL).unwrap();
    assert_eq!(protocol["wire"], CLAUDE_AGENT_SDK_WIRE);
    assert_eq!(protocol["behavior_revision"], CLAUDE_AGENT_SDK_BEHAVIOR);
    assert_eq!(protocol["sdk_package"], CLAUDE_AGENT_SDK_PACKAGE);
    assert_eq!(protocol["sdk_version"], "0.3.293");
    assert_eq!(protocol["native_version"], "2.1.293");
    assert_eq!(protocol["node_runtime"], "22.23.3");
    assert_eq!(
        protocol["sidecar_entry_file"],
        CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE
    );
    assert_eq!(
        protocol["compatibility_claim"],
        "qualified_only_maintained_segment"
    );
    assert_eq!(protocol["package_baseline"], "0.3.284");
    assert_eq!(protocol["native_baseline"], "2.1.284");
    let maintained_pairs: Vec<_> = ClaudeAgentSdkPackageNativePair::MAINTAINED[..10]
        .iter()
        .map(|pair| {
            serde_json::json!({
                "sdk_package_version": pair.package_version(),
                "native_version": pair.native_version(),
            })
        })
        .collect();
    assert_eq!(
        protocol["package_native_pairs"],
        serde_json::json!(maintained_pairs)
    );
    assert_eq!(
        protocol["list_sessions_params"],
        serde_json::json!([
            "cwd",
            "limit",
            "offset",
            "expectedSdkVersion",
            "expectedNativeVersion"
        ])
    );
    assert!(
        CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG
            .starts_with(protocol["sidecar_source_tag_prefix"].as_str().unwrap())
    );
    for expected in [
        CLAUDE_AGENT_SDK_WIRE,
        CLAUDE_AGENT_SDK_BEHAVIOR,
        CLAUDE_AGENT_SDK_PACKAGE,
        CLAUDE_AGENT_SDK_VERSION,
        CLAUDE_AGENT_SDK_NATIVE_VERSION,
    ] {
        assert!(
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(expected),
            "sidecar source must carry {expected}"
        );
    }
    for pair in ClaudeAgentSdkPackageNativePair::MAINTAINED {
        assert!(
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(pair.package_version()),
            "sidecar pair map must retain {}",
            pair.package_version()
        );
        assert!(
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(pair.native_version()),
            "sidecar pair map must retain {}",
            pair.native_version()
        );
    }
}

#[test]
fn outbound_corpus_covers_every_command_and_both_decisions() {
    let records: Vec<serde_json::Value> =
        include_str!("../../../tests/fixtures/claude-agent-sdk-v1/commands.jsonl")
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
    let protocol: serde_json::Value = serde_json::from_str(PROTOCOL).unwrap();
    let qualified: Vec<&str> = protocol["commands"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let commands: Vec<&str> = records
        .iter()
        .filter(|record| record["type"] == "command")
        .map(|record| {
            assert!(record["id"].as_str().unwrap().len() <= 128);
            record["command"].as_str().unwrap()
        })
        .collect();
    for command in &qualified {
        assert!(commands.contains(command), "missing command {command}");
    }
    assert_eq!(commands.len(), qualified.len());
    let decisions: Vec<&str> = records
        .iter()
        .filter(|record| record["type"] == "callback_response")
        .map(|record| record["decision"].as_str().unwrap())
        .collect();
    assert_eq!(decisions, ["allow", "deny"]);
    assert_eq!(
        records[3]["params"]["joinBoundMs"],
        protocol["join_bound_ms"]
    );
}
