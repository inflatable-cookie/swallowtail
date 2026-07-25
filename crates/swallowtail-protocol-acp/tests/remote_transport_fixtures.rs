use serde_json::Value;
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

#[path = "remote_transport_fixtures/http.rs"]
mod http;
#[path = "remote_transport_fixtures/websocket.rs"]
mod websocket;

const ROOT: &str = "fixtures/acp-v1-remote-transport-2.0.0";
const MAX_RECORD_BYTES: usize = 64 * 1024;
const MAX_RECORDS: usize = 64;
const MANIFEST: &str = include_str!("fixtures/acp-v1-remote-transport-2.0.0/transport.json");
const HTTP_SUCCESS: &str =
    include_str!("fixtures/acp-v1-remote-transport-2.0.0/http-sse-success.jsonl");
const HTTP_INVALID: &str =
    include_str!("fixtures/acp-v1-remote-transport-2.0.0/http-invalid-requests.jsonl");
const HTTP_DISCONNECT: &str =
    include_str!("fixtures/acp-v1-remote-transport-2.0.0/http-disconnect.sse");
const WEBSOCKET_SUCCESS: &str =
    include_str!("fixtures/acp-v1-remote-transport-2.0.0/websocket-success.jsonl");
const WEBSOCKET_DISCONNECT: &str =
    include_str!("fixtures/acp-v1-remote-transport-2.0.0/websocket-disconnect.jsonl");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FixtureError {
    TranscriptTooLarge,
    TooManyRecords,
    RecordTooLarge,
    IncompleteRecord,
    InvalidJson,
    InvalidShape,
}

fn parse_json(input: &str) -> Value {
    assert!(input.len() <= MAX_RECORD_BYTES, "fixture JSON is bounded");
    serde_json::from_str(input).expect("fixture JSON is valid")
}

fn parse_records(input: &str) -> Result<Vec<Value>, FixtureError> {
    if input.len() > MAX_RECORDS * MAX_RECORD_BYTES {
        return Err(FixtureError::TranscriptTooLarge);
    }
    if !input.is_empty() && !input.ends_with('\n') {
        return Err(FixtureError::IncompleteRecord);
    }
    let mut records = Vec::new();
    for line in input.lines() {
        if records.len() == MAX_RECORDS {
            return Err(FixtureError::TooManyRecords);
        }
        if line.len() > MAX_RECORD_BYTES {
            return Err(FixtureError::RecordTooLarge);
        }
        let record: Value = serde_json::from_str(line).map_err(|_| FixtureError::InvalidJson)?;
        if record.get("kind").and_then(Value::as_str).is_none() {
            return Err(FixtureError::InvalidShape);
        }
        records.push(record);
    }
    Ok(records)
}

fn kind(record: &Value) -> &str {
    record["kind"].as_str().expect("fixture kind is text")
}

fn body_method(record: &Value) -> Option<&str> {
    record
        .get("body")
        .and_then(|body| body.get("method"))
        .and_then(Value::as_str)
}

#[test]
fn manifest_keeps_wire_rfd_sdk_agent_and_interface_axes_separate() {
    let manifest = parse_json(MANIFEST);
    assert_eq!(manifest["fixture_schema"], 1);
    assert_eq!(manifest["protocol"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(manifest["protocol"]["transport_rfd_status"], "active");
    assert_eq!(
        manifest["protocol"]["transport_rfd_revision"],
        "observed-2026-07-24"
    );
    assert_eq!(manifest["sdk"]["transport_version"], "2.0.0");
    assert_eq!(manifest["sdk"]["core_version"], "2.0.0");
    assert_eq!(manifest["sdk"]["runtime_interface_range_claimed"], false);
    assert!(manifest.get("provider").is_none());
    assert!(manifest.get("agent").is_none());
    assert!(ROOT.contains("remote-transport-2.0.0"));
}

#[test]
fn raw_fixture_parsing_is_bounded_and_errors_carry_no_private_material() {
    assert_eq!(
        parse_records("{\"kind\":\"incomplete\"}"),
        Err(FixtureError::IncompleteRecord)
    );
    let oversized = format!(
        "{{\"kind\":\"record\",\"payload\":\"{}\"}}\n",
        "x".repeat(MAX_RECORD_BYTES)
    );
    assert_eq!(parse_records(&oversized), Err(FixtureError::RecordTooLarge));
    let one = "{\"kind\":\"record\"}\n";
    assert_eq!(
        parse_records(&one.repeat(MAX_RECORDS + 1)),
        Err(FixtureError::TooManyRecords)
    );
    let invalid = parse_records("{\"kind\":private-cookie}\n").expect_err("invalid JSON fails");
    assert_eq!(invalid, FixtureError::InvalidJson);
    assert!(!format!("{invalid:?}").contains("private-cookie"));
}

#[test]
fn manifest_disables_every_implicit_recovery_and_diagnostic_leak() {
    let manifest = parse_json(MANIFEST);
    for field in [
        "redirect",
        "retry",
        "reconnect",
        "replay",
        "resumption",
        "pooling",
        "multiplexing",
        "transport_fallback",
    ] {
        assert_eq!(manifest["recovery"][field], false);
    }
    for field in [
        "endpoint",
        "headers",
        "cookies",
        "connection_id",
        "session_id",
        "frames",
        "sdk_errors",
    ] {
        assert_eq!(manifest["diagnostics"][field], "redacted");
    }
}
