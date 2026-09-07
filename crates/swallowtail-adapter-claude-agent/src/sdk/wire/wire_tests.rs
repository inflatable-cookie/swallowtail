use super::{
    ClaudeAgentSdkCommand, ClaudeAgentSdkDiagnosticLevel, ClaudeAgentSdkEvent,
    ClaudeAgentSdkFailureCode, ClaudeAgentSdkRecord, ClaudeAgentSdkToolDecision, decode_record,
    encode_callback_response, encode_command,
};
use crate::sdk::protocol::ClaudeAgentSdkProtocolFailureKind;
use serde_json::{Value, json};
use std::collections::BTreeSet;

const MODEL_EVIDENCE_BOUNDS: &str =
    include_str!("../../../tests/fixtures/claude-agent-sdk/model-evidence-bounds.json");

#[test]
fn commands_and_callback_responses_encode_as_lf_terminated_records() {
    let bytes = encode_command("open-1", ClaudeAgentSdkCommand::Open, json!({"cwd": "/w"}))
        .expect("command encodes");
    assert!(bytes.ends_with(b"\n"));
    let value: serde_json::Value =
        serde_json::from_slice(&bytes[..bytes.len() - 1]).expect("command is JSON");
    assert_eq!(value["type"], "command");
    assert_eq!(value["command"], "open");
    assert_eq!(value["params"]["cwd"], "/w");

    let bytes = encode_callback_response("cb-1", ClaudeAgentSdkToolDecision::Deny)
        .expect("callback response encodes");
    let value: serde_json::Value =
        serde_json::from_slice(&bytes[..bytes.len() - 1]).expect("response is JSON");
    assert_eq!(value["type"], "callback_response");
    assert_eq!(value["decision"], "deny");
}

#[test]
fn responses_must_correlate_a_qualified_command_and_one_outcome() {
    for invalid in [
        json!({"type": "response", "id": "a", "command": "explode", "success": true}),
        json!({"type": "response", "id": "", "command": "open", "success": true}),
        json!({"type": "response", "id": "a", "command": "open"}),
        json!({"type": "response", "id": "a", "command": "open", "success": true, "failure": {"code": "x", "message": "y"}}),
        json!({"type": "response", "id": "a", "command": "open", "success": false}),
        json!({"type": "response", "id": "a", "command": "open", "success": true, "data": 7}),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidResponse),
            "record {invalid} must fail closed"
        );
    }
}

#[test]
fn command_rejection_keeps_only_its_fixed_code() {
    let bytes = serde_json::to_vec(&json!({
        "type": "response",
        "id": "open-1",
        "command": "open",
        "success": false,
        "failure": {
            "code": "construction_failed",
            "message": "provider path and account details stay discarded"
        }
    }))
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Response(response) =
        decode_record(&bytes).expect("rejection decodes")
    else {
        panic!("response expected");
    };
    assert_eq!(
        response.failure_code,
        Some(ClaudeAgentSdkFailureCode::ConstructionFailed)
    );
    assert_eq!(response.original_failure_code, None);
    assert!(response.data.is_none());

    let unknown = serde_json::to_vec(&json!({
        "type": "response",
        "id": "open-1",
        "command": "open",
        "success": false,
        "failure": {"code": "provider_secret", "message": "discard me"}
    }))
    .expect("fixture serializes");
    assert_eq!(
        decode_record(&unknown).err().map(|error| error.kind()),
        Some(ClaudeAgentSdkProtocolFailureKind::InvalidResponse)
    );
}

#[test]
fn terminal_session_rejection_keeps_only_a_bounded_original_code() {
    let bytes = serde_json::to_vec(&json!({
        "type": "response",
        "id": "query-2",
        "command": "query",
        "success": false,
        "failure": {
            "code": "session_rejected_terminal",
            "message": "sidecar command failed: session_rejected_terminal",
            "originalCode": "supported_model_rejected"
        }
    }))
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Response(response) =
        decode_record(&bytes).expect("terminal rejection decodes")
    else {
        panic!("response expected");
    };
    assert_eq!(
        response.failure_code,
        Some(ClaudeAgentSdkFailureCode::SessionRejectedTerminal)
    );
    assert_eq!(
        response.original_failure_code,
        Some(ClaudeAgentSdkFailureCode::SupportedModelRejected)
    );

    for original_code in [None, Some("provider_secret")] {
        let mut failure = json!({
            "code": "session_rejected_terminal",
            "message": "sidecar command failed: session_rejected_terminal"
        });
        if let Some(original_code) = original_code {
            failure["originalCode"] = json!(original_code);
        }
        let invalid = json!({
            "type": "response",
            "id": "query-2",
            "command": "query",
            "success": false,
            "failure": failure
        });
        assert_eq!(
            decode_record(&serde_json::to_vec(&invalid).expect("fixture serializes"))
                .err()
                .map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidResponse)
        );
    }
}

#[test]
fn sidecar_command_failure_codes_match_the_rust_enumeration() {
    let source = crate::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE;
    let (_, body) = source
        .split_once("const COMMAND_FAILURE_CODES = new Set([")
        .expect("sidecar declares its command failure code set");
    let (body, _) = body
        .split_once("]);")
        .expect("sidecar command failure code set is closed");
    let sidecar_codes: BTreeSet<_> = body
        .split('"')
        .filter(|value| {
            !value.is_empty()
                && value
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character == '_')
        })
        .collect();
    let rust_codes: BTreeSet<_> = [
        ClaudeAgentSdkFailureCode::MissingEnvironment,
        ClaudeAgentSdkFailureCode::InvalidCommand,
        ClaudeAgentSdkFailureCode::ToolsInvalid,
        ClaudeAgentSdkFailureCode::PermissionModeInvalid,
        ClaudeAgentSdkFailureCode::PermissionModeRejected,
        ClaudeAgentSdkFailureCode::SdkUnavailable,
        ClaudeAgentSdkFailureCode::SdkExportMissing,
        ClaudeAgentSdkFailureCode::SdkVersionMismatch,
        ClaudeAgentSdkFailureCode::SdkIdentityUnverifiable,
        ClaudeAgentSdkFailureCode::NativeManifestUnavailable,
        ClaudeAgentSdkFailureCode::NativeVersionMismatch,
        ClaudeAgentSdkFailureCode::CapabilitiesOverflow,
        ClaudeAgentSdkFailureCode::CapabilitiesInvalid,
        ClaudeAgentSdkFailureCode::AccountNotFirstParty,
        ClaudeAgentSdkFailureCode::AccountNotSubscription,
        ClaudeAgentSdkFailureCode::AlreadyOpen,
        ClaudeAgentSdkFailureCode::NodeRuntimeUnsupported,
        ClaudeAgentSdkFailureCode::ConstructionFailed,
        ClaudeAgentSdkFailureCode::InitializationFailed,
        ClaudeAgentSdkFailureCode::InitMissing,
        ClaudeAgentSdkFailureCode::SessionRejectedTerminal,
        ClaudeAgentSdkFailureCode::CwdMismatch,
        ClaudeAgentSdkFailureCode::ModelMismatch,
        ClaudeAgentSdkFailureCode::ModelMissing,
        ClaudeAgentSdkFailureCode::SupportedModelRejected,
        ClaudeAgentSdkFailureCode::AccountUnavailable,
        ClaudeAgentSdkFailureCode::ResumeCwdMismatch,
        ClaudeAgentSdkFailureCode::ResumeAccountMismatch,
        ClaudeAgentSdkFailureCode::ResumeSessionUnknown,
        ClaudeAgentSdkFailureCode::ResumeBoundaryInvalid,
        ClaudeAgentSdkFailureCode::ResumePersistenceDisabled,
        ClaudeAgentSdkFailureCode::ListingFailed,
        ClaudeAgentSdkFailureCode::ListingInvalid,
        ClaudeAgentSdkFailureCode::NativeChildUnavailable,
        ClaudeAgentSdkFailureCode::NotOpen,
        ClaudeAgentSdkFailureCode::TurnActive,
        ClaudeAgentSdkFailureCode::PromptTooLarge,
        ClaudeAgentSdkFailureCode::InterruptFailed,
        ClaudeAgentSdkFailureCode::PermissionModeUnsupported,
        ClaudeAgentSdkFailureCode::PermissionModeFailed,
        ClaudeAgentSdkFailureCode::PermissionModeUnconfirmed,
        ClaudeAgentSdkFailureCode::ModelChangeUnsupported,
        ClaudeAgentSdkFailureCode::ModelChangeFailed,
        ClaudeAgentSdkFailureCode::ModelChangeUnconfirmed,
        ClaudeAgentSdkFailureCode::EffortUnconfirmed,
        ClaudeAgentSdkFailureCode::McpServersInvalid,
        ClaudeAgentSdkFailureCode::McpServerUndeclared,
        ClaudeAgentSdkFailureCode::McpServerFailed,
        ClaudeAgentSdkFailureCode::McpServerNeedsAuth,
        ClaudeAgentSdkFailureCode::McpStatusInvalid,
        ClaudeAgentSdkFailureCode::UnknownCommand,
        ClaudeAgentSdkFailureCode::CommandFailed,
    ]
    .into_iter()
    .map(ClaudeAgentSdkFailureCode::as_str)
    .collect();
    assert_eq!(sidecar_codes, rust_codes);
}

#[test]
fn callbacks_admit_only_the_qualified_tool_admission_shape() {
    let bytes = serde_json::to_vec(
        &json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool", "toolName": "Read"}),
    )
    .expect("fixture serializes");
    let record = decode_record(&bytes).expect("qualified callback decodes");
    let ClaudeAgentSdkRecord::Callback(callback) = record else {
        panic!("callback expected");
    };
    assert_eq!(callback.id, "cb-1");
    assert_eq!(callback.tool_name, "Read");

    for invalid in [
        json!({"type": "callback", "id": "cb-1", "callback": "request_user_dialog", "toolName": "Read"}),
        json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool"}),
        json!({"type": "callback", "callback": "can_use_tool", "toolName": "Read"}),
        json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool", "toolName": "Read", "command": "pwd"}),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidCallback)
        );
    }
}

#[test]
fn bash_callbacks_carry_a_bounded_truncation_flagged_command_view() {
    let command = "x".repeat(128);
    let description = "d".repeat(128);
    let bytes = serde_json::to_vec(&json!({
        "type": "callback",
        "id": "cb-1",
        "callback": "can_use_tool",
        "toolName": "Bash",
        "command": command,
        "commandByteLength": 256,
        "description": description,
        "truncated": true
    }))
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Callback(callback) =
        decode_record(&bytes).expect("Bash callback decodes")
    else {
        panic!("callback expected");
    };
    let view = callback
        .bash_command
        .expect("Bash carries its command view");
    assert_eq!(view.command.len(), 128);
    assert_eq!(view.command_byte_length, 256);
    assert_eq!(view.description.len(), 128);
    assert!(view.truncated);

    let invalid = serde_json::to_vec(&json!({
        "type": "callback",
        "id": "cb-1",
        "callback": "can_use_tool",
        "toolName": "Bash",
        "command": "pwd",
        "commandByteLength": 256,
        "description": "run it",
        "truncated": false
    }))
    .expect("fixture serializes");
    assert_eq!(
        decode_record(&invalid).err().map(|error| error.kind()),
        Some(ClaudeAgentSdkProtocolFailureKind::InvalidCallback)
    );
}

#[test]
fn events_decode_their_qualified_payloads_and_reject_the_rest() {
    let bytes = serde_json::to_vec(
        &json!({"type": "event", "event": "tool_ended", "toolCallId": "t-1", "isError": true}),
    )
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Event(ClaudeAgentSdkEvent::ToolEnded { call_id, failed }) =
        decode_record(&bytes).expect("qualified event decodes")
    else {
        panic!("tool_ended expected");
    };
    assert_eq!(call_id, "t-1");
    assert!(failed);

    for (invalid, expected) in [
        (
            json!({"type": "event", "event": "tool_ended", "toolCallId": "t-1"}),
            ClaudeAgentSdkProtocolFailureKind::InvalidEvent,
        ),
        (
            json!({"type": "event", "event": "usage_report"}),
            ClaudeAgentSdkProtocolFailureKind::UnknownRecord,
        ),
        (
            json!({"type": "event"}),
            ClaudeAgentSdkProtocolFailureKind::MissingType,
        ),
        (
            json!({"type": "shell", "event": "turn_started"}),
            ClaudeAgentSdkProtocolFailureKind::UnknownRecord,
        ),
        (
            json!({"event": "turn_started"}),
            ClaudeAgentSdkProtocolFailureKind::MissingType,
        ),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(expected),
            "record {invalid} must fail closed"
        );
    }
}

#[test]
fn turn_end_decodes_every_sanitized_result_observation_without_result_text() {
    let bytes = serde_json::to_vec(&json!({
        "type": "event",
        "event": "turn_ended",
        "subtype": "error_during_execution",
        "stopReason": "error_during_execution",
        "isError": true,
        "numTurns": 2,
        "durationMs": 41,
        "errorTextPresent": true,
        "errorTextType": "string",
        "resultFieldPresence": {
            "error": true,
            "is_error": true,
            "num_turns": true,
            "subtype": true
        }
    }))
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Event(ClaudeAgentSdkEvent::TurnEnded {
        stop_reason,
        failed,
        subtype,
        num_turns,
        duration_ms,
        error_text_present,
        error_text_type,
        result_field_presence,
    }) = decode_record(&bytes).expect("turn end decodes")
    else {
        panic!("turn_ended expected");
    };
    assert_eq!(stop_reason, "error_during_execution");
    assert!(failed);
    assert_eq!(subtype.as_deref(), Some("error_during_execution"));
    assert_eq!(num_turns, Some(2));
    assert_eq!(duration_ms, Some(41));
    assert!(error_text_present);
    assert_eq!(error_text_type, "string");
    assert!(result_field_presence["error"]);
    assert!(result_field_presence["num_turns"]);

    let null_metadata = serde_json::to_vec(&json!({
        "type": "event", "event": "turn_ended", "subtype": null,
        "stopReason": "", "isError": false, "numTurns": null,
        "durationMs": null, "errorTextPresent": false,
        "errorTextType": "absent", "resultFieldPresence": {}
    }))
    .expect("fixture serializes");
    assert!(matches!(
        decode_record(&null_metadata),
        Ok(ClaudeAgentSdkRecord::Event(
            ClaudeAgentSdkEvent::TurnEnded {
                subtype: None,
                num_turns: None,
                duration_ms: None,
                ..
            }
        ))
    ));

    let invalid = json!({
        "type": "event", "event": "turn_ended", "subtype": "provider error",
        "stopReason": "provider error", "isError": true, "numTurns": null,
        "durationMs": null, "errorTextPresent": true,
        "errorTextType": "string", "resultFieldPresence": {}
    });
    let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
    assert_eq!(
        decode_record(&bytes).err().map(|error| error.kind()),
        Some(ClaudeAgentSdkProtocolFailureKind::InvalidEvent),
        "record {invalid} must fail closed"
    );
}

#[test]
fn terminal_and_diagnostic_payloads_stay_bounded() {
    let bytes = serde_json::to_vec(&json!({
        "type": "diagnostic",
        "level": "error",
        "code": "sdk_close_failed",
        "message": "sidecar diagnostic: sdk_close_failed"
    }))
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Diagnostic(diagnostic) =
        decode_record(&bytes).expect("qualified diagnostic decodes")
    else {
        panic!("diagnostic expected");
    };
    assert_eq!(diagnostic.level, ClaudeAgentSdkDiagnosticLevel::Error);

    let oversized = "x".repeat(600);
    for (invalid, expected) in [
        (
            json!({"type": "diagnostic", "level": "trace", "code": "a", "message": "b"}),
            ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic,
        ),
        (
            json!({"type": "diagnostic", "level": "info", "code": "a", "message": oversized}),
            ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic,
        ),
        (
            json!({"type": "terminal"}),
            ClaudeAgentSdkProtocolFailureKind::InvalidTerminal,
        ),
        (
            json!({"type": "terminal", "failure": {"code": "a"}}),
            ClaudeAgentSdkProtocolFailureKind::InvalidTerminal,
        ),
        (
            json!({"type": "terminal", "failure": {
                "code": "session_rejected_terminal",
                "message": "sidecar command failed: session_rejected_terminal",
                "originalCode": "supported_model_rejected"
            }}),
            ClaudeAgentSdkProtocolFailureKind::InvalidTerminal,
        ),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(expected)
        );
    }
}

fn model_qualification_record() -> Value {
    json!({
        "type": "diagnostic",
        "level": "error",
        "code": "supported_model_rejected",
        "message": "sidecar diagnostic: supported_model_rejected",
        "evidence": {
            "requestedModel": "claude-sonnet-5",
            "effectiveModel": "claude-sonnet-5-20250929",
            "catalogueSize": 1,
            "catalogueDigest": "sha256:4b07d9a517e35f5852d8385b4a192070",
            "requestedMembership": true,
            "effectiveMembership": false,
            "querySource": "sdk.query",
            "phase": "first-turn-model-qualification",
            "declaredSdkVersion": "0.3.259",
            "loadedSdkVersion": "0.3.259",
            "nativeVersion": "2.1.259"
        }
    })
}

fn bad_digest(record: &mut Value) {
    record["evidence"]["catalogueDigest"] = json!("sha256:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
}

fn oversized_model_id(record: &mut Value) {
    record["evidence"]["requestedModel"] = json!("x".repeat(129));
}

fn oversized_catalogue_count(record: &mut Value) {
    record["evidence"]["catalogueSize"] = json!(65);
}

fn extra_evidence_field(record: &mut Value) {
    record["evidence"]["unexpected"] = json!(true);
}

fn missing_evidence_field(record: &mut Value) {
    record["evidence"]
        .as_object_mut()
        .expect("evidence object")
        .remove("phase");
}

fn wrong_query_source(record: &mut Value) {
    record["evidence"]["querySource"] = json!("sdk.initialization");
}

fn wrong_phase(record: &mut Value) {
    record["evidence"]["phase"] = json!("open");
}

fn evidence_on_wrong_diagnostic_code(record: &mut Value) {
    record["code"] = json!("query_rejected");
}

type EvidenceMutation = (&'static str, fn(&mut Value));

#[test]
fn model_qualification_evidence_rejects_contract_drift() {
    let cases: [EvidenceMutation; 8] = [
        ("bad digest", bad_digest),
        ("model id over 128 bytes", oversized_model_id),
        ("catalogue count over 64", oversized_catalogue_count),
        ("extra evidence field", extra_evidence_field),
        ("missing evidence field", missing_evidence_field),
        ("wrong query source", wrong_query_source),
        ("wrong phase", wrong_phase),
        (
            "evidence on wrong diagnostic code",
            evidence_on_wrong_diagnostic_code,
        ),
    ];
    for (label, mutate) in cases {
        let mut record = model_qualification_record();
        mutate(&mut record);
        let bytes = serde_json::to_vec(&record).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic),
            "{label} must fail closed"
        );
    }
}

#[test]
fn model_qualification_ids_match_the_shared_boundary_and_control_table() {
    let cases: Value = serde_json::from_str(MODEL_EVIDENCE_BOUNDS)
        .expect("model evidence bounds fixture is valid JSON");
    for case in cases
        .as_array()
        .expect("model evidence fixture is an array")
    {
        let name = case["name"].as_str().expect("fixture case name");
        let unit = case["unit"].as_str().expect("fixture unit");
        let repeat = case["repeat"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .expect("fixture repeat is a usize");
        let value = unit.repeat(repeat);
        let valid = case["valid"].as_bool().expect("fixture validity");

        for field in ["requestedModel", "effectiveModel"] {
            let mut record = model_qualification_record();
            record["evidence"][field] = json!(value);
            let decoded = decode_record(&serde_json::to_vec(&record).expect("fixture serializes"));
            assert_eq!(decoded.is_ok(), valid, "Rust predicate for {name}/{field}");
        }
    }
}

#[test]
fn sdk_identity_mismatch_evidence_is_bounded_and_typed() {
    let record = json!({
        "type": "diagnostic",
        "level": "error",
        "code": "sdk_version_mismatch",
        "message": "sidecar diagnostic: sdk_version_mismatch",
        "evidence": {
            "declaredSdkPackage": "@anthropic-ai/claude-agent-sdk",
            "declaredSdkVersion": "0.3.259",
            "loadedSdkPackage": "@anthropic-ai/claude-agent-sdk",
            "loadedSdkVersion": "0.3.258"
        }
    });
    assert!(matches!(
        decode_record(&serde_json::to_vec(&record).expect("fixture serializes")),
        Ok(ClaudeAgentSdkRecord::Diagnostic(_))
    ));

    let mutations: [fn(&mut Value); 3] = [
        |record: &mut Value| record["evidence"]["loadedSdkVersion"] = json!("x".repeat(129)),
        |record: &mut Value| record["evidence"]["unexpected"] = json!(true),
        |record: &mut Value| record["code"] = json!("query_rejected"),
    ];
    for mutation in mutations {
        let mut invalid = record.clone();
        mutation(&mut invalid);
        assert_eq!(
            decode_record(&serde_json::to_vec(&invalid).expect("fixture serializes"))
                .err()
                .map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic)
        );
    }
}
