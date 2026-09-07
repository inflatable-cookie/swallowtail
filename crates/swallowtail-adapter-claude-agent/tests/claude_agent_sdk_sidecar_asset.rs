//! Sidecar-level falsification of the shipped Claude Agent SDK asset.
//!
//! This runs the real `claude-agent-sdk-sidecar.mjs` under Node against a fake
//! SDK module and a fake native child. It is provider-free by construction:
//! nothing is installed, the official package is never present, no credential
//! exists, and no provider session is opened. What it proves is exactly the
//! part a Rust-side fake cannot: how the asset drives the SDK's own option
//! surface and `canUseTool` contract.

mod sidecar_asset_support;

use serde_json::{Value, json};
use sidecar_asset_support::SidecarProcess;

const EVIDENCE_BOUNDS: &str = include_str!("fixtures/claude-agent-sdk/model-evidence-bounds.json");

#[test]
fn matching_sdk_package_identity_is_verified_and_reported_at_open() {
    let mut sidecar = SidecarProcess::start_scenario("sdk-identity-match");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "matching identity opens: {open}");
    assert_eq!(open["data"]["sdkPackage"], "@anthropic-ai/claude-agent-sdk");
    assert_eq!(open["data"]["sdkVersion"], "0.3.259");
    assert!(sidecar.sdk_was_constructed());
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true);
}

#[test]
fn mismatching_sdk_package_identity_fails_before_sdk_construction_with_bounded_evidence() {
    let mut sidecar = SidecarProcess::start_scenario("sdk-identity-mismatch");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], false, "mismatch must reject: {open}");
    assert_eq!(open["failure"]["code"], "sdk_version_mismatch");
    assert!(!sidecar.sdk_was_constructed());
    assert_eq!(
        sidecar.next_diagnostic()["evidence"],
        json!({
            "declaredSdkPackage": "@anthropic-ai/claude-agent-sdk",
            "declaredSdkVersion": "0.3.259",
            "loadedSdkPackage": "@anthropic-ai/claude-agent-sdk",
            "loadedSdkVersion": "0.3.258"
        })
    );
}

#[test]
fn sdk_identity_fields_match_the_shared_boundary_and_control_table() {
    let cases: Value =
        serde_json::from_str(EVIDENCE_BOUNDS).expect("evidence bounds fixture is valid JSON");
    for case in cases
        .as_array()
        .expect("evidence bounds fixture is an array")
    {
        let name = case["name"].as_str().expect("fixture case name");
        let unit = case["unit"].as_str().expect("fixture unit");
        let repeat = case["repeat"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .expect("fixture repeat is a usize");
        let value = unit.repeat(repeat);
        let valid = case["valid"].as_bool().expect("fixture validity");

        for field in ["loadedSdkPackage", "loadedSdkVersion"] {
            let mut sidecar = SidecarProcess::start_sdk_identity_case(field, &value);
            let open = sidecar.command(
                "open-1",
                "open",
                json!({"cwd": sidecar.cwd(), "model": "m-1"}),
            );
            let expected_code = if valid {
                "sdk_version_mismatch"
            } else {
                "sdk_identity_unverifiable"
            };
            assert_eq!(
                open["failure"]["code"], expected_code,
                "sidecar identity classification for {name}/{field}: {open}"
            );
            assert!(
                !sidecar.sdk_was_constructed(),
                "SDK query ran for {name}/{field}"
            );
            if valid {
                assert_eq!(
                    sidecar.next_diagnostic()["evidence"][field],
                    json!(value.clone()),
                    "sidecar identity evidence for {name}/{field}"
                );
            }
        }
    }
}

#[test]
fn missing_sdk_package_identity_is_typed_before_sdk_construction() {
    let mut sidecar = SidecarProcess::start_scenario("sdk-identity-missing");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(
        open["success"], false,
        "missing identity must reject: {open}"
    );
    assert_eq!(open["failure"]["code"], "sdk_identity_unverifiable");
    assert!(!sidecar.sdk_was_constructed());
}

#[test]
fn package_identity_stays_bound_to_the_nested_sdk_package_root() {
    let mut sidecar = SidecarProcess::start_scenario("sdk-identity-nested-match");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(
        open["success"], true,
        "nested package identity opens: {open}"
    );
    assert_eq!(open["data"]["sdkPackage"], "@anthropic-ai/claude-agent-sdk");
    assert_eq!(open["data"]["sdkVersion"], "0.3.259");
    assert!(sidecar.sdk_was_constructed());
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true);
}

#[test]
fn unrelated_ancestor_identity_is_unverifiable_before_sdk_construction() {
    for scenario in [
        "sdk-identity-unrelated-ancestor",
        "sdk-identity-malformed",
        "sdk-identity-unreadable",
        "sdk-identity-ambiguous",
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let open = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        assert_eq!(
            open["failure"]["code"], "sdk_identity_unverifiable",
            "identity failure classification for {scenario}: {open}"
        );
        assert!(
            !sidecar.sdk_was_constructed(),
            "SDK query ran for {scenario}"
        );
    }
}

#[test]
fn model_evidence_bounds_and_controls_match_the_shared_fixture_table() {
    let cases: Value =
        serde_json::from_str(EVIDENCE_BOUNDS).expect("model evidence bounds fixture is valid JSON");
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

        for scenario in ["model-evidence-requested", "model-evidence-effective"] {
            let mut sidecar = SidecarProcess::start_model_evidence_case(scenario, &value);
            let requested_model = if scenario == "model-evidence-requested" {
                value.clone()
            } else {
                "fixture-requested".to_owned()
            };
            let open = sidecar.command(
                "open-1",
                "open",
                json!({"cwd": sidecar.cwd(), "model": requested_model}),
            );
            assert_eq!(open["success"], true, "fixture open for {name}/{scenario}");
            let response = sidecar.command("query-1", "query", json!({"text": "first turn"}));
            assert_eq!(response["failure"]["code"], "supported_model_rejected");
            let diagnostic = sidecar.next_diagnostic();
            assert_eq!(
                diagnostic.get("evidence").is_some(),
                valid,
                "sidecar predicate for {name}/{scenario}"
            );
            let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
            assert_eq!(close["success"], true);
        }
    }
}

#[test]
fn the_fake_sdk_calls_spawn_with_one_spawn_options_object() {
    let mut sidecar = SidecarProcess::start();
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(
        open["success"], true,
        "object-form spawn hook must construct: {open}"
    );
    assert_eq!(open["data"]["readiness"], "requested-with-supported-list");
    assert_eq!(open["data"]["requestedModel"], "m-1");
    assert!(open["data"].get("model").is_none());
    assert_eq!(
        sidecar.observed_control_calls(),
        vec![
            "initializationResult".to_owned(),
            "supportedModels".to_owned(),
            "accountInfo".to_owned(),
        ]
    );
    assert!(!sidecar.first_input_consumed());
    // This is the argument the fake SDK actually received, not the query
    // options passed into `query()`. A positional callback receives the
    // object in the wrong slot and fails before this response is produced.
    let spawn = sidecar.observed_spawn_hook_argument();
    assert_eq!(
        sidecar.observed_spawn_hook_argument_count(),
        1_usize,
        "the SDK invokes spawnClaudeCodeProcess with one object argument"
    );
    let mut keys = spawn
        .as_object()
        .expect("spawn hook shape is an object")
        .keys()
        .collect::<Vec<_>>();
    keys.sort();
    assert_eq!(keys, vec!["args", "command", "cwd", "env", "signal"]);
    assert!(spawn["command"].is_string());
    assert_eq!(spawn["args"][0], "-e");
    assert_eq!(spawn["cwd"], sidecar.cwd());
    assert_eq!(
        spawn["env"]["keys"],
        json!([
            "COLORTERM",
            "COMMAND_MODE",
            "HOME",
            "LANG",
            "LC_ALL",
            "LC_CTYPE",
            "MallocNanoZone",
            "PATH",
            "SHELL",
            "TERM",
            "TMPDIR",
            "USER",
            "XPC_FLAGS",
            "XPC_SERVICE_NAME",
            "__CF_USER_TEXT_ENCODING"
        ])
    );
    assert_eq!(
        spawn["signal"], true,
        "SpawnOptions.signal must be preserved"
    );
}

#[test]
fn session_input_stays_open_until_close_and_early_eof_is_an_error_result() {
    let mut open_input = SidecarProcess::start_scenario("input-stream-lifetime");
    let open = open_input.command(
        "open-1",
        "open",
        json!({"cwd": open_input.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "open response: {open}");
    open_input.command("query-1", "query", json!({"text": "first turn"}));
    let terminal = open_input.wait_for_turn_end_record();
    assert_eq!(
        open_input.observed_prompt_stream_state().as_deref(),
        Some("open")
    );
    assert_eq!(terminal["subtype"], "success");
    assert_eq!(terminal["isError"], false);
    assert_eq!(terminal["numTurns"], 1);
    assert_eq!(terminal["durationMs"], 7);
    assert_eq!(terminal["errorTextPresent"], false);
    assert_eq!(terminal["errorTextType"], "absent");
    for field in ["duration_ms", "is_error", "num_turns", "subtype", "type"] {
        assert_eq!(
            terminal["resultFieldPresence"][field], true,
            "{field} present"
        );
    }
    for field in ["duration_api_ms", "result", "errors", "uuid", "session_id"] {
        assert_eq!(
            terminal["resultFieldPresence"][field], false,
            "{field} absent"
        );
    }

    let close = open_input.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true, "close response: {close}");
    assert_eq!(
        close["data"]["closeTimeline"],
        json!([
            "close_requested",
            "session_input_closed",
            "sdk_transport_close_ran",
            "native_join_exited"
        ])
    );

    let mut early_eof = SidecarProcess::start_scenario("early-input-eof");
    let open = early_eof.command(
        "open-1",
        "open",
        json!({"cwd": early_eof.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "open response: {open}");
    early_eof.command("query-1", "query", json!({"text": "first turn"}));
    let terminal = early_eof.wait_for_turn_end_record();
    assert_eq!(
        early_eof.observed_prompt_stream_state().as_deref(),
        Some("early-eof")
    );
    assert_eq!(terminal["subtype"], "error_during_execution");
    assert_eq!(terminal["isError"], true);
    assert_eq!(terminal["numTurns"], 1);
    assert_eq!(terminal["durationMs"], 7);
    assert_eq!(terminal["errorTextPresent"], true);
    assert_eq!(terminal["errorTextType"], "string");
    for field in [
        "duration_ms",
        "error",
        "is_error",
        "num_turns",
        "subtype",
        "type",
    ] {
        assert_eq!(
            terminal["resultFieldPresence"][field], true,
            "{field} present"
        );
    }
    for field in ["duration_api_ms", "result", "errors", "uuid", "session_id"] {
        assert_eq!(
            terminal["resultFieldPresence"][field], false,
            "{field} absent"
        );
    }
    assert!(
        !terminal.to_string().contains("fixture early input EOF"),
        "SDK error text must never cross the sidecar wire: {terminal}"
    );
    let close = early_eof.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true, "close response: {close}");
}

#[test]
fn post_init_oversized_prompt_rejection_keeps_sidecar_usable_without_replay() {
    let mut sidecar = SidecarProcess::start_scenario("between-turns");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "open response: {open}");

    let first = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(first["success"], true, "first query response: {first}");
    sidecar.wait_for_turn_end();
    let barrier = sidecar.command(
        "barrier-1",
        "set_permission_mode",
        json!({"mode": "default"}),
    );
    assert_eq!(
        barrier["success"], true,
        "first-turn drain barrier: {barrier}"
    );
    let input_calls_before_rejection = sidecar.observed_query_input_calls();

    let rejection = sidecar.command(
        "query-2",
        "query",
        json!({"text": "x".repeat(256 * 1024 + 1)}),
    );
    assert_eq!(
        rejection["success"], false,
        "oversized prompt response: {rejection}"
    );
    assert_eq!(rejection["failure"]["code"], "prompt_too_large");
    assert_eq!(
        sidecar.observed_query_input_calls(),
        input_calls_before_rejection,
        "sidecar rejected the oversized prompt before SDK input consumption"
    );

    let next = sidecar.command("query-3", "query", json!({"text": "next turn"}));
    assert_eq!(next["success"], true, "next query response: {next}");
    sidecar.wait_for_turn_end();
    assert!(
        sidecar.observed_query_input_calls() > input_calls_before_rejection,
        "the valid next turn reached the SDK exactly after the local rejection"
    );

    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true, "close response: {close}");
}

#[test]
fn an_unmapped_message_stays_terminal_without_crossing_its_type_or_error_text() {
    let mut sidecar = SidecarProcess::start_scenario("unknown-message");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(
        open["success"], true,
        "unknown-message fixture opens: {open}"
    );
    let terminal = sidecar.terminal_after_query("query-1", json!({"text": "first turn"}));
    assert_eq!(terminal["failure"]["code"], "unknown_message");
    assert_eq!(
        terminal["failure"]["message"],
        "sidecar terminated: unknown_message"
    );
    assert!(!terminal.to_string().contains("fixture_unknown_message"));
}

#[test]
fn pinned_advisory_rate_limits_allow_reply_and_successful_result() {
    for scenario in ["rate-allowed", "rate-allowed_warning"] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let open = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        assert_eq!(open["success"], true);
        sidecar.command("query-1", "query", json!({"text": "first turn"}));
        assert_eq!(sidecar.next_event()["event"], "turn_started");
        assert_eq!(
            sidecar.next_event(),
            json!({"type": "event", "event": "progress"})
        );
        let reply = sidecar.next_event();
        assert_eq!(reply["event"], "output_delta");
        assert_eq!(reply["delta"], "fixture reply");
        let ended = sidecar.next_event();
        assert_eq!(ended["event"], "turn_ended");
        assert_eq!(ended["isError"], false);
        assert_eq!(ended["stopReason"], "success");
        let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
        assert_eq!(close["success"], true);
        assert_eq!(close["data"]["sdkTransportCloseRan"], true);
        assert_eq!(close["data"]["nativeExitObserved"], true);
    }
}

#[test]
fn idle_rate_limit_notifications_do_not_emit_turn_events_and_next_turn_succeeds() {
    let mut sidecar = SidecarProcess::start_scenario("between-turns");
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(sidecar.next_event()["event"], "turn_started");
    assert_eq!(sidecar.next_event()["event"], "output_delta");
    assert_eq!(sidecar.next_event()["event"], "turn_ended");

    // Same-mode control is a fixture barrier: its SDK method waits until the
    // session iterator has projected all three notices while no turn is active.
    let barrier = sidecar.command(
        "idle-barrier",
        "set_permission_mode",
        json!({"mode": "default"}),
    );
    assert_eq!(barrier["success"], true);
    sidecar.command("query-2", "query", json!({"text": "second turn"}));
    // Any idle progress record would be held ahead of this turn_started.
    assert_eq!(sidecar.next_event()["event"], "turn_started");
    let reply = sidecar.next_event();
    assert_eq!(reply["event"], "output_delta");
    assert_eq!(reply["delta"], "fixture reply");
    let ended = sidecar.next_event();
    assert_eq!(ended["event"], "turn_ended");
    assert_eq!(ended["isError"], false);
    assert_eq!(ended["stopReason"], "success");
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true);
    assert_eq!(close["data"]["nativeExitObserved"], true);
}

#[test]
fn idle_malformed_and_unknown_notifications_still_terminate() {
    for scenario in ["between-malformed", "between-unknown"] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        let terminal = sidecar.terminal_after_query("query-1", json!({"text": "first turn"}));
        assert_eq!(terminal["failure"]["code"], "unknown_message");
    }
}

#[test]
fn rejected_rate_limit_preserves_the_provider_error_result() {
    let mut sidecar = SidecarProcess::start_scenario("rate-rejected");
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(sidecar.next_event()["event"], "turn_started");
    assert_eq!(
        sidecar.next_event(),
        json!({"type": "event", "event": "progress"})
    );
    let ended = sidecar.next_event();
    assert_eq!(ended["event"], "turn_ended");
    assert_eq!(ended["isError"], true);
    assert_eq!(ended["errorTextPresent"], true);
    assert!(!ended.to_string().contains("private rejection detail"));
    assert_eq!(
        sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}))["success"],
        true
    );
}

#[test]
fn malformed_rate_limits_remain_terminal() {
    for scenario in [
        "rate-missing-info",
        "rate-null-info",
        "rate-array-info",
        "rate-missing-status",
        "rate-numeric-status",
        "rate-future-status",
        "rate-missing-session",
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        let terminal = sidecar.terminal_after_query("query-1", json!({"text": "first turn"}));
        assert_eq!(terminal["failure"]["code"], "unknown_message");
        assert!(!terminal.to_string().contains("private-session"));
    }
}

#[test]
fn rate_limit_progress_preserves_interrupt_and_close() {
    let mut sidecar = SidecarProcess::start_scenario("rate-cancel");
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(sidecar.next_event()["event"], "turn_started");
    assert_eq!(sidecar.next_event()["event"], "progress");
    let interrupt = sidecar.command("interrupt-1", "interrupt", json!({}));
    assert_eq!(interrupt["success"], true);
    assert_eq!(sidecar.wait_for_turn_end_record()["event"], "turn_ended");
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true);
    assert_eq!(close["data"]["sdkTransportCloseRan"], true);
    assert_eq!(close["data"]["nativeExitObserved"], true);
}

#[test]
fn pinned_result_error_fields_report_presence_without_provider_text() {
    for (scenario, field, kind) in [
        ("pinned-error-result", "errors", "array"),
        ("pinned-success-error", "result", "string"),
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let open = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        assert_eq!(open["success"], true);
        sidecar.command("query-1", "query", json!({"text": "first turn"}));
        let ended = sidecar.wait_for_turn_end_record();
        assert_eq!(ended["isError"], true);
        assert_eq!(ended["errorTextPresent"], true);
        assert_eq!(ended["errorTextType"], kind);
        assert_eq!(ended["resultFieldPresence"][field], true);
        assert!(!ended.to_string().contains("private provider detail"));
        sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    }
}

#[test]
fn open_rejections_expose_only_the_fixed_sidecar_code() {
    for (scenario, expected) in [("account-not-first-party", "account_not_first_party")] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let response = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        assert_eq!(
            response["success"], false,
            "{scenario} must reject: {response}"
        );
        assert_eq!(response["failure"]["code"], expected);
        let failure = response["failure"].to_string();
        for forbidden in ["/fixture/", "@example", "token", "organization"] {
            assert!(
                !failure.contains(forbidden),
                "{scenario} leaked {forbidden}: {failure}"
            );
        }
    }
}

#[test]
fn first_party_account_fields_are_labelled_observations_not_gates() {
    for (scenario, subscription, token, api_key) in [
        ("read-only", true, false, false),
        ("account-not-subscription", false, false, false),
        ("account-token-source", true, true, false),
        ("account-api-key-source", true, false, true),
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let open = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        assert_eq!(open["success"], true, "{scenario} must open: {open}");
        assert_eq!(open["data"]["account"]["apiProvider"], "firstParty");
        assert_eq!(
            open["data"]["account"]["subscriptionTypePresent"],
            subscription
        );
        assert_eq!(open["data"]["account"]["tokenSourcePresent"], token);
        assert_eq!(open["data"]["account"]["apiKeySourcePresent"], api_key);
    }
}

#[test]
fn an_empty_supported_model_list_is_unavailable() {
    let mut sidecar = SidecarProcess::start_scenario("empty-supported-models");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "empty list is unavailable: {open}");
    assert_eq!(open["data"]["supportedModels"], json!([]));
    let first_turn = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(
        first_turn["success"], true,
        "empty list must not reject the effective model: {first_turn}"
    );
    assert_eq!(first_turn["data"]["model"], "m-1");
}

#[test]
fn first_turn_init_rejections_expose_their_fixed_sidecar_code() {
    for (scenario, expected) in [
        ("init-missing", "init_missing"),
        ("init-not-first", "init_missing"),
        ("init-throws", "initialization_failed"),
        ("cwd-mismatch", "cwd_mismatch"),
        ("missing-model", "model_missing"),
        ("unsupported-model", "supported_model_rejected"),
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let open = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        assert_eq!(
            open["success"], true,
            "{scenario} must pass initialize: {open}"
        );
        let response = sidecar.command("query-1", "query", json!({"text": "first turn"}));
        assert_eq!(
            response["success"], false,
            "{scenario} must reject: {response}"
        );
        assert_eq!(response["failure"]["code"], expected);
    }
}

#[test]
fn first_turn_rejection_makes_retry_terminal_without_replaying_sdk_input() {
    let mut sidecar = SidecarProcess::start_scenario("unsupported-model");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true);

    let first_turn = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(first_turn["success"], false);
    assert_eq!(first_turn["failure"]["code"], "supported_model_rejected");
    assert_eq!(
        sidecar.next_diagnostic()["code"],
        "supported_model_rejected"
    );
    assert_eq!(sidecar.observed_query_input_calls(), 1);

    let retry = sidecar.command("query-2", "query", json!({"text": "retry"}));
    assert_eq!(retry["success"], false);
    assert_eq!(retry["failure"]["code"], "session_rejected_terminal");
    assert_eq!(retry["failure"]["originalCode"], "supported_model_rejected");
    assert_eq!(
        retry["failure"]["message"],
        "sidecar command failed: session_rejected_terminal"
    );
    assert_eq!(sidecar.observed_query_input_calls(), 1);

    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(
        close["success"], true,
        "terminal rejection remains closable"
    );
    assert_eq!(close["data"]["nativeExitObserved"], true);
    assert_eq!(close["data"]["sdkTransportCloseRan"], true);
    assert_eq!(sidecar.observed_close_calls(), 1);
}

#[test]
fn first_turn_rejection_can_close_without_retry_or_replay() {
    let mut sidecar = SidecarProcess::start_scenario("init-missing");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true);

    let first_turn = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(first_turn["success"], false);
    assert_eq!(first_turn["failure"]["code"], "init_missing");
    assert_eq!(sidecar.observed_query_input_calls(), 1);

    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(
        close["success"], true,
        "init shape rejection remains closable"
    );
    assert_eq!(close["data"]["nativeExitObserved"], true);
    assert_eq!(close["data"]["sdkTransportCloseRan"], true);
    assert_eq!(sidecar.observed_query_input_calls(), 1);
    assert_eq!(sidecar.observed_close_calls(), 1);
}

#[test]
fn model_rejection_evidence_is_bounded_and_does_not_change_the_failure_response() {
    for (scenario, digest, requested_membership, effective_membership) in [
        (
            "alias-only",
            "sha256:a461b472cb41a9ec3dee5c90cf8e4a78",
            true,
            false,
        ),
        (
            "neither-ids",
            "sha256:f9ef6eedd766691f041bc3387784d8dd",
            false,
            false,
        ),
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        sidecar.command(
            "open-1",
            "open",
            json!({"cwd": sidecar.cwd(), "model": "m-1"}),
        );
        let response = sidecar.command("query-1", "query", json!({"text": "first turn"}));
        assert_eq!(
            response,
            json!({
                "type": "response",
                "id": "query-1",
                "command": "query",
                "success": false,
                "failure": {
                    "code": "supported_model_rejected",
                    "message": "sidecar command failed: supported_model_rejected"
                }
            }),
            "the rejection response remains byte-identical for {scenario}"
        );
        let diagnostic = sidecar.next_diagnostic();
        assert_eq!(
            diagnostic["code"], "supported_model_rejected",
            "diagnostic code for {scenario}"
        );
        assert_eq!(
            diagnostic["message"],
            "sidecar diagnostic: supported_model_rejected"
        );
        assert_eq!(
            diagnostic["evidence"],
            json!({
                "requestedModel": "m-1",
                "effectiveModel": "claude-sonnet-5-20250929",
                "catalogueSize": 1,
                "catalogueDigest": digest,
                "requestedMembership": requested_membership,
                "effectiveMembership": effective_membership,
                "querySource": "sdk.query",
                "phase": "first-turn-model-qualification",
                "declaredSdkVersion": "0.3.259",
                "loadedSdkVersion": "0.3.259",
                "nativeVersion": "2.1.259"
            })
        );
        let wire = diagnostic.to_string();
        assert!(!wire.contains("first turn"));
        assert!(!wire.contains("fixture-secret"));
        assert!(!wire.contains(&sidecar.cwd()));
        assert!(!wire.contains("Error("));
        let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
        assert_eq!(close["success"], true);
    }
}

#[test]
fn diagnostic_write_failure_keeps_the_supported_model_rejection() {
    let mut sidecar = SidecarProcess::start_scenario("diagnostic-write-failure");
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    let response = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(
        response,
        json!({
            "type": "response",
            "id": "query-1",
            "command": "query",
            "success": false,
            "failure": {
                "code": "supported_model_rejected",
                "message": "sidecar command failed: supported_model_rejected"
            }
        })
    );
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true);
}

#[test]
fn canonicalized_first_turn_cwd_is_accepted_but_a_different_path_is_rejected() {
    let mut sidecar = SidecarProcess::start_scenario("canonical-cwd");
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "canonical cwd open: {open}");
    let first_turn = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(
        first_turn["success"], true,
        "canonical cwd init: {first_turn}"
    );
    assert_eq!(first_turn["data"]["cwd"], sidecar.cwd());

    let mut different = SidecarProcess::start_scenario("cwd-mismatch");
    let open = different.command(
        "open-1",
        "open",
        json!({"cwd": different.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "different cwd open: {open}");
    let first_turn = different.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(
        first_turn["success"], false,
        "different cwd must reject: {first_turn}"
    );
    assert_eq!(first_turn["failure"]["code"], "cwd_mismatch");
}

#[test]
fn canonical_effective_model_is_accepted_and_published() {
    let mut sidecar = SidecarProcess::start_scenario("canonical-model");
    let response = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "claude-sonnet-5"}),
    );
    assert_eq!(
        response["success"], true,
        "canonical model must open: {response}"
    );
    assert_eq!(
        response["data"]["readiness"],
        "requested-with-supported-list"
    );
    assert_eq!(response["data"]["requestedModel"], "claude-sonnet-5");
    assert!(response["data"].get("model").is_none());
    let first_turn = sidecar.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(first_turn["success"], true, "first-turn init: {first_turn}");
    assert_eq!(first_turn["data"]["readiness"], "confirmed");
    assert_eq!(first_turn["data"]["model"], "claude-sonnet-5-20250929");
}

#[test]
fn supported_models_drive_confirmed_and_unconfirmed_model_changes() {
    let mut confirmed = SidecarProcess::start_scenario("model-change-confirmed");
    confirmed.command(
        "open-1",
        "open",
        json!({"cwd": confirmed.cwd(), "model": "m-1"}),
    );
    confirmed.command("query-1", "query", json!({"text": "first turn"}));
    let changed = confirmed.command("model-1", "set_model", json!({"model": "claude-opus-5"}));
    assert_eq!(
        changed["success"], true,
        "confirmed model response: {changed}"
    );
    assert_eq!(changed["data"]["model"], "claude-opus-5");
    assert_eq!(confirmed.observed_model_set_calls(), vec!["claude-opus-5"]);

    let mut unconfirmed = SidecarProcess::start_scenario("model-change-unconfirmed");
    unconfirmed.command(
        "open-1",
        "open",
        json!({"cwd": unconfirmed.cwd(), "model": "m-1"}),
    );
    unconfirmed.command("query-1", "query", json!({"text": "first turn"}));
    let response = unconfirmed.command("model-1", "set_model", json!({"model": "claude-opus-5"}));
    assert_eq!(
        response["success"], false,
        "unconfirmed model response: {response}"
    );
    assert_eq!(response["failure"]["code"], "model_change_unconfirmed");
    assert_eq!(
        unconfirmed.observed_model_set_calls(),
        vec!["claude-opus-5"]
    );
}

#[test]
fn a_fake_sdk_rejection_is_typed_unconfirmed_after_query_set_model() {
    let mut rejected = SidecarProcess::start_scenario("model-change-rejected");
    rejected.command(
        "open-1",
        "open",
        json!({"cwd": rejected.cwd(), "model": "m-1"}),
    );
    rejected.command("query-1", "query", json!({"text": "first turn"}));
    let response = rejected.command("model-1", "set_model", json!({"model": "claude-opus-5"}));
    assert_eq!(
        response["success"], false,
        "provider rejection is not confirmation: {response}"
    );
    assert_eq!(response["failure"]["code"], "model_change_unconfirmed");
    assert_eq!(rejected.observed_model_set_calls(), vec!["claude-opus-5"]);
}

#[test]
fn the_sidecar_rejects_a_model_before_query_set_model() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    sidecar.command("query-1", "query", json!({"text": "first turn"}));
    let response = sidecar.command("model-1", "set_model", json!({"model": "claude-opus-5"}));
    assert_eq!(response["success"], false);
    assert_eq!(response["failure"]["code"], "supported_model_rejected");
    assert!(sidecar.observed_model_set_calls().is_empty());
}

#[test]
fn effort_is_optional_at_open_and_init_confirmation_is_preserved() {
    let mut confirmed = SidecarProcess::start_scenario("effort-confirmed");
    let open = confirmed.command(
        "open-1",
        "open",
        json!({"cwd": confirmed.cwd(), "model": "m-1", "effort": "xhigh"}),
    );
    assert_eq!(open["data"]["requestedEffort"], "xhigh");
    assert_eq!(confirmed.observed_options()["effort"], "xhigh");
    let query = confirmed.command("query-1", "query", json!({"text": "first turn"}));
    assert_eq!(query["data"]["effort"], "xhigh");

    let mut requested_only = SidecarProcess::start();
    let open = requested_only.command(
        "open-1",
        "open",
        json!({"cwd": requested_only.cwd(), "model": "m-1", "effort": "max"}),
    );
    assert_eq!(open["data"]["requestedEffort"], "max");
    let query = requested_only.command("query-1", "query", json!({"text": "first turn"}));
    assert!(query["data"].get("effort").is_none());
}

#[test]
fn every_allowed_invocation_crosses_the_callback_with_its_input_intact() {
    let mut sidecar = SidecarProcess::start();
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "open response: {open}");
    assert_eq!(open["data"]["readiness"], "requested-with-supported-list");

    let query = sidecar.command("query-1", "query", json!({"text": "read it"}));
    assert_eq!(query["success"], true, "query response: {query}");
    assert_eq!(query["data"]["readiness"], "confirmed");

    // The allowed tool reaches the host as a bounded callback carrying the
    // tool name and nothing else: no input, no path, no provider payload.
    let request = sidecar.next_callback();
    assert_eq!(request["callback"], "can_use_tool");
    assert_eq!(request["toolName"], "Read");
    assert_eq!(
        request.as_object().expect("callback is an object").len(),
        4,
        "callback carries only type, id, callback, and toolName: {request}"
    );
    assert!(request.get("input").is_none());

    sidecar.respond_callback(request["id"].as_str().expect("callback id"), "allow");
    let observations = sidecar.admissions(&["Read"]);

    // An allowed decision returns the provider's own input unchanged. An empty
    // `updatedInput` would silently destroy the path the tool needs.
    let allowed = &observations["Read"];
    assert_eq!(allowed["behavior"], "allow");
    assert_eq!(
        allowed["updatedInput"],
        json!({"file_path": "/fixture/read-me.txt"}),
        "allowed input must survive the round trip"
    );
}

#[test]
fn a_tool_outside_the_read_only_set_is_denied_without_asking_the_host() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    sidecar.command("query-1", "query", json!({"text": "run it"}));

    let request = sidecar.next_callback();
    assert_eq!(
        request["toolName"], "Read",
        "only allowed tools are offered"
    );
    sidecar.respond_callback(request["id"].as_str().expect("callback id"), "allow");

    let observations = sidecar.admissions(&["Read", "Bash"]);
    let denied = &observations["Bash"];
    assert_eq!(denied["behavior"], "deny");
    assert!(
        denied.get("updatedInput").is_none(),
        "a denial never returns tool input"
    );
    // The unadmitted tool produced no consumer round trip at all.
    assert_eq!(
        sidecar.callback_tool_names(),
        vec!["Read".to_owned()],
        "an unadmitted tool must never reach the host"
    );
}

#[test]
fn the_asset_restricts_availability_without_auto_allowing_anything() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    let options = sidecar.observed_options();

    // `tools` restricts what exists; `allowedTools` would auto-allow without
    // prompting and must never be set by this route.
    assert_eq!(options["tools"], json!(["Read", "Glob", "Grep"]));
    assert!(
        options.get("allowedTools").is_none(),
        "allowedTools bypasses per-use admission: {options}"
    );
    assert_eq!(options["model"], "m-1");
    assert!(
        options.get("effort").is_none(),
        "the default profile must omit Options.effort: {options}"
    );
    assert_eq!(options["settingSources"], json!([]));
    assert_eq!(options["skills"], json!([]));
    assert_eq!(options["persistSession"], json!(false));
    assert_eq!(options["mcpServers"], json!({}));
    assert_eq!(
        options["strictMcpConfig"], true,
        "strict MCP config stays on even when no servers are declared"
    );
    assert_eq!(
        options["env"]["keys"],
        json!([
            "COLORTERM",
            "COMMAND_MODE",
            "HOME",
            "LANG",
            "LC_ALL",
            "LC_CTYPE",
            "MallocNanoZone",
            "PATH",
            "SHELL",
            "TERM",
            "TMPDIR",
            "USER",
            "XPC_FLAGS",
            "XPC_SERVICE_NAME",
            "__CF_USER_TEXT_ENCODING"
        ])
    );
    for forbidden in [
        "ANTHROPIC_API_KEY",
        "CLAUDE_CONFIG_DIR",
        "CLAUDE_CODE_USE_BEDROCK",
        "OPENAI_API_KEY",
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "GOOGLE_API_KEY",
        "RANDOM_UNRELATED",
    ] {
        assert!(
            !options["env"]["keys"]
                .as_array()
                .unwrap()
                .iter()
                .any(|key| key == forbidden),
            "forbidden environment key was forwarded: {forbidden}"
        );
    }
    for forbidden in ["apiKeyHelper", "awsAuthRefresh", "gcpAuthRefresh"] {
        assert!(
            options.get(forbidden).is_none(),
            "{forbidden} must be unset"
        );
    }
    assert_eq!(options["executable"], "node");
}

#[test]
fn resume_options_are_forwarded_without_replaying_a_transcript() {
    let mut sidecar = SidecarProcess::start();
    let open = sidecar.command(
        "open-1",
        "open",
        json!({
            "cwd": sidecar.cwd(),
            "model": "m-1",
            "persistSession": true,
            "resume": "session-1",
            "resumeSessionAt": "message-boundary-1"
        }),
    );
    assert_eq!(open["success"], true, "resume open response: {open}");
    let options = sidecar.observed_options();
    assert_eq!(options["persistSession"], true);
    assert_eq!(options["resume"], "session-1");
    assert_eq!(options["resumeSessionAt"], "message-boundary-1");

    let query = sidecar.command("query-1", "query", json!({"text": "continue"}));
    assert_eq!(query["success"], true, "resumed query response: {query}");
    assert_eq!(query["data"]["sessionId"], "session-1");
    assert_eq!(query["data"]["accountVerified"], true);
    assert_eq!(query["data"]["account"]["apiProvider"], "firstParty");
    assert!(sidecar.first_input_consumed());
}

#[test]
fn fake_sdk_resume_mismatch_scenarios_fail_on_the_first_turn() {
    for (scenario, expected) in [
        ("resume-session-unknown", "resume_session_unknown"),
        ("resume-cwd-mismatch", "resume_cwd_mismatch"),
        ("resume-account-mismatch", "resume_account_mismatch"),
    ] {
        let mut sidecar = SidecarProcess::start_scenario(scenario);
        let cwd = sidecar.cwd();
        let open = sidecar.command(
            "open-1",
            "open",
            json!({
                "cwd": cwd,
                "model": "m-1",
                "persistSession": true,
                "resume": "session-1"
            }),
        );
        assert_eq!(open["success"], true, "resume open response: {open}");

        let query = sidecar.command("query-1", "query", json!({"text": "continue"}));
        assert_eq!(query["success"], false, "resume mismatch response: {query}");
        assert_eq!(query["failure"]["code"], expected);
        assert!(
            query.get("data").is_none(),
            "a failed resume must not accept a turn: {query}"
        );
    }
}

#[test]
fn listing_forwards_only_the_bounded_provider_metadata_request() {
    let mut sidecar = SidecarProcess::start();
    let response = sidecar.command(
        "list-1",
        "list_sessions",
        json!({"cwd": sidecar.cwd(), "limit": 1000, "offset": 0}),
    );
    assert_eq!(response["success"], true, "listing response: {response}");
    assert_eq!(response["data"]["cwd"], sidecar.cwd());
    assert_eq!(
        response["data"]["sessions"],
        json!([{
            "sessionId": "session-1",
            "cwd": sidecar.cwd(),
            "createdAt": 100,
            "lastModified": 200,
            "title": "Fixture title"
        }])
    );
    assert_eq!(
        sidecar.observed_list_sessions(),
        json!({
            "dir": sidecar.cwd(),
            "limit": 1000,
            "offset": 0,
            "includeWorktrees": false,
            "includeProgrammatic": true
        })
    );
}

const WRITE_TOOLS: [&str; 6] = ["Read", "Glob", "Grep", "Edit", "Write", "MultiEdit"];
const BASH_TOOLS: [&str; 7] = ["Read", "Glob", "Grep", "Edit", "Write", "MultiEdit", "Bash"];

fn open_bash(sidecar: &mut SidecarProcess, permission_mode: &str) -> serde_json::Value {
    let cwd = sidecar.cwd();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": cwd, "model": "m-1", "tools": BASH_TOOLS,
               "permissionMode": permission_mode}),
    )
}

fn next_bash_request(sidecar: &mut SidecarProcess) -> serde_json::Value {
    let request = sidecar.next_callback();
    assert_eq!(request["toolName"], "Bash");
    request
}

#[test]
fn bash_is_host_mediated_in_every_permission_mode_with_bounded_views() {
    for permission_mode in ["default", "plan", "acceptEdits"] {
        let mut sidecar = SidecarProcess::start_bash();
        let open = open_bash(&mut sidecar, permission_mode);
        assert_eq!(open["success"], true, "open response: {open}");
        assert_eq!(open["data"]["tools"], json!(BASH_TOOLS));
        assert_eq!(open["data"]["permissionMode"], permission_mode);

        sidecar.command("query-1", "query", json!({"text": "run it"}));
        let denied = next_bash_request(&mut sidecar);
        assert_eq!(
            denied["command"],
            "node -e \"require('fs').writeFileSync('denied.txt','denied')\""
        );
        assert_eq!(denied["description"], "write a denied marker");
        assert_eq!(denied["commandByteLength"], 60);
        assert_eq!(denied["truncated"], false);
        sidecar.respond_callback(denied["id"].as_str().expect("callback id"), "deny");

        let allowed = next_bash_request(&mut sidecar);
        assert_eq!(allowed["command"].as_str().expect("command").len(), 128);
        assert!(
            allowed["commandByteLength"]
                .as_u64()
                .expect("command length")
                > 128
        );
        assert_eq!(
            allowed["description"].as_str().expect("description").len(),
            128
        );
        assert_eq!(allowed["truncated"], true);
        sidecar.respond_callback(allowed["id"].as_str().expect("callback id"), "allow");

        let outcomes = sidecar.bash_outcomes(2);
        assert_eq!(outcomes[0]["allowed"], false);
        assert_eq!(outcomes[0]["ran"], false);
        assert_eq!(outcomes[1]["allowed"], true);
        assert_eq!(outcomes[1]["inputUnchanged"], true);
        assert_eq!(outcomes[1]["ran"], true);
        assert_eq!(outcomes[1]["exitStatus"], 0);
        assert!(outcomes[1]["command"].as_str().expect("full command").len() > 128);
        assert_eq!(
            outcomes[1]["description"]
                .as_str()
                .expect("full description")
                .len(),
            180
        );
        assert_eq!(sidecar.file_under_cwd("denied.txt"), None);
        assert_eq!(
            sidecar.file_under_cwd("allowed.txt").as_deref(),
            Some("allowed")
        );
        assert_eq!(
            sidecar.callback_tool_names(),
            vec!["Bash".to_owned(), "Bash".to_owned()]
        );
    }
}

fn open_editing(sidecar: &mut SidecarProcess, permission_mode: &str) -> serde_json::Value {
    let cwd = sidecar.cwd();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": cwd, "model": "m-1", "tools": WRITE_TOOLS,
               "permissionMode": permission_mode}),
    )
}

/// Answers the read the editing fixture always makes, then returns the write
/// request the host must decide.
fn next_write_request(sidecar: &mut SidecarProcess) -> serde_json::Value {
    let read = sidecar.next_callback();
    assert_eq!(read["toolName"], "Read");
    sidecar.respond_callback(read["id"].as_str().expect("callback id"), "allow");
    let write = sidecar.next_callback();
    assert_eq!(write["toolName"], "Write");
    write
}

#[test]
fn a_two_turn_editing_session_writes_only_what_the_host_admitted() {
    let mut sidecar = SidecarProcess::start_editing();
    let open = open_editing(&mut sidecar, "default");
    assert_eq!(open["success"], true, "open response: {open}");
    assert_eq!(open["data"]["tools"], json!(WRITE_TOOLS));
    assert_eq!(open["data"]["permissionMode"], "default");

    sidecar.command("query-1", "query", json!({"text": "edit it"}));
    let first = next_write_request(&mut sidecar);
    sidecar.respond_callback(first["id"].as_str().expect("callback id"), "allow");
    sidecar.wait_for_turn_end();

    sidecar.command("query-2", "query", json!({"text": "edit it again"}));
    let second = next_write_request(&mut sidecar);
    sidecar.respond_callback(second["id"].as_str().expect("callback id"), "deny");

    let writes = sidecar.writes(2);
    assert_eq!(writes[0]["admitted"], "allowed");
    assert_eq!(writes[1]["admitted"], "denied");
    // The filesystem is the evidence: the admitted write landed, the denied
    // one never touched disk.
    assert_eq!(
        sidecar.file_under_cwd("turn-1.txt").as_deref(),
        Some("turn 1\n")
    );
    assert_eq!(sidecar.file_under_cwd("turn-2.txt"), None);
    // Every write crossed the consumer boundary first, in both turns.
    assert_eq!(
        sidecar.callback_tool_names(),
        vec![
            "Read".to_owned(),
            "Write".to_owned(),
            "Read".to_owned(),
            "Write".to_owned()
        ]
    );
}

#[test]
fn accept_edits_skips_admission_for_edits_and_nothing_else() {
    let mut sidecar = SidecarProcess::start_editing();
    let open = open_editing(&mut sidecar, "acceptEdits");
    assert_eq!(open["data"]["permissionMode"], "acceptEdits");

    sidecar.command("query-1", "query", json!({"text": "edit it"}));
    // The read is still mediated; the edit is not offered at all.
    let read = sidecar.next_callback();
    assert_eq!(read["toolName"], "Read");
    sidecar.respond_callback(read["id"].as_str().expect("callback id"), "allow");

    let writes = sidecar.writes(1);
    assert_eq!(writes[0]["admitted"], "skipped");
    assert_eq!(
        sidecar.file_under_cwd("turn-1.txt").as_deref(),
        Some("turn 1\n")
    );
    assert_eq!(sidecar.callback_tool_names(), vec!["Read".to_owned()]);
}

#[test]
fn a_mid_session_permission_mode_change_round_trips_the_confirmed_mode() {
    let mut sidecar = SidecarProcess::start_editing();
    open_editing(&mut sidecar, "default");

    let planned = sidecar.command("mode-1", "set_permission_mode", json!({"mode": "plan"}));
    assert_eq!(planned["success"], true, "mode response: {planned}");
    assert_eq!(planned["data"]["permissionMode"], "plan");

    let restored = sidecar.command("mode-2", "set_permission_mode", json!({"mode": "default"}));
    assert_eq!(restored["data"]["permissionMode"], "default");
    assert_eq!(
        sidecar.observed_permission_modes(),
        vec!["plan".to_owned(), "default".to_owned()],
        "the SDK saw exactly the two requested changes"
    );
}

#[test]
fn an_auto_approving_mode_never_reaches_the_sdk() {
    for mode in ["bypassPermissions", "auto", "dontAsk"] {
        let mut sidecar = SidecarProcess::start_editing();
        let cwd = sidecar.cwd();
        let open = sidecar.command(
            "open-1",
            "open",
            json!({"cwd": cwd, "model": "m-1", "tools": WRITE_TOOLS, "permissionMode": mode}),
        );
        assert_eq!(open["success"], false, "{mode} must be refused: {open}");
        assert_eq!(open["failure"]["code"], "permission_mode_rejected");
        // Refused before construction: the SDK was never even loaded.
        assert!(
            !sidecar.sdk_was_constructed(),
            "{mode} must be refused before the SDK is constructed"
        );
    }
}

#[test]
fn an_unadmitted_tool_name_is_refused_before_the_sdk_is_constructed() {
    let mut sidecar = SidecarProcess::start_editing();
    let cwd = sidecar.cwd();
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": cwd, "model": "m-1", "tools": ["Read", "BashOutput"],
               "permissionMode": "default"}),
    );
    assert_eq!(open["success"], false, "open response: {open}");
    assert_eq!(open["failure"]["code"], "tools_invalid");
    assert!(!sidecar.sdk_was_constructed());
}

#[test]
fn a_write_profile_restricts_availability_without_auto_allowing_anything() {
    let mut sidecar = SidecarProcess::start_editing();
    open_editing(&mut sidecar, "acceptEdits");
    let options = sidecar.observed_options();

    // Even with writes admitted, `allowedTools` stays unset: the consumer's
    // decision is the only thing that can allow a call.
    assert_eq!(options["tools"], json!(WRITE_TOOLS));
    assert!(
        options.get("allowedTools").is_none(),
        "allowedTools bypasses per-use admission: {options}"
    );
    assert_eq!(options["permissionMode"], "acceptEdits");
    // Bash stays disallowed when this editing profile withholds it; terminal
    // and other later-card tools remain outside the route entirely.
    for forbidden in ["Bash", "BashOutput", "KillShell", "NotebookEdit", "Task"] {
        assert!(
            options["disallowedTools"]
                .as_array()
                .expect("disallowed tools are listed")
                .iter()
                .any(|tool| tool == forbidden),
            "{forbidden} must stay disallowed: {options}"
        );
    }
}

#[test]
fn an_admissible_tool_the_host_withheld_is_disallowed() {
    let mut sidecar = SidecarProcess::start();
    let cwd = sidecar.cwd();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": cwd, "model": "m-1", "tools": ["Read", "Glob", "Grep"],
               "permissionMode": "default"}),
    );
    let options = sidecar.observed_options();
    for withheld in ["Edit", "Write", "MultiEdit"] {
        assert!(
            options["disallowedTools"]
                .as_array()
                .expect("disallowed tools are listed")
                .iter()
                .any(|tool| tool == withheld),
            "{withheld} was not admitted, so it must be disallowed: {options}"
        );
    }
}

#[test]
fn close_reports_the_native_exit_it_actually_observed() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );

    // The fake native child exits on its own well inside the declared bound.
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2000}));
    assert_eq!(close["success"], true, "close response: {close}");
    assert_eq!(close["data"]["nativeJoin"], "exited");
    assert_eq!(close["data"]["nativeExitObserved"], true);
    assert_eq!(close["data"]["nativeExitEvent"], "exit");
    assert_eq!(close["data"]["nativeExitCode"], 0);
    assert_eq!(close["data"]["nativeExitSignal"], Value::Null);
    assert_eq!(close["data"]["sdkTransportCloseRan"], true);
    assert_eq!(close["data"]["joinBoundMs"], 2000);
    assert_eq!(sidecar.observed_close_calls(), 1);
}

#[test]
fn a_native_child_alive_at_the_bound_is_reported_as_a_survivor() {
    let mut sidecar = SidecarProcess::start_with_surviving_native_child();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );

    // The child outlives the declared bound. The retained handle still shows it
    // running, which is a positive survivor observation the host turns into
    // cleanup failure, never an absence of news.
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 300}));
    assert_eq!(close["data"]["nativeJoin"], "survivor");
    assert_eq!(close["data"]["nativeExitObserved"], false);
    assert_eq!(close["data"]["nativeExitEvent"], Value::Null);
    assert_eq!(close["data"]["nativeExitCode"], Value::Null);
    assert_eq!(close["data"]["nativeExitSignal"], Value::Null);
    assert_eq!(close["data"]["sdkTransportCloseRan"], true);
    assert_eq!(sidecar.observed_close_calls(), 1);
}

fn fixture_mcp_server() -> Value {
    json!({
        "name": "fixture",
        "command": "/usr/bin/node",
        "args": ["server.mjs"],
        "envAllowlistKeys": ["PATH", "HOME"],
        "tools": ["search"],
        "optional": false
    })
}

fn fixture_mcp_open(cwd: &str, extra_tools: &[&str], optional: bool) -> Value {
    let mut server = fixture_mcp_server();
    server["optional"] = json!(optional);
    let mut tools = vec!["Read", "Glob", "Grep"];
    tools.extend_from_slice(extra_tools);
    json!({
        "cwd": cwd,
        "model": "m-1",
        "tools": tools,
        "permissionMode": "default",
        "mcpServers": [server]
    })
}

#[test]
fn a_declared_stdio_mcp_server_connects_and_its_tool_is_mediated() {
    let mut sidecar = SidecarProcess::start_scenario("mcp");
    let cwd = sidecar.cwd();
    let open = sidecar.command(
        "open-1",
        "open",
        fixture_mcp_open(&cwd, &["mcp__fixture__search"], false),
    );
    assert_eq!(open["success"], true, "MCP open response: {open}");
    assert_eq!(
        open["data"]["mcpServerStatus"],
        json!([{"name": "fixture", "status": "connected"}])
    );
    assert_eq!(
        open["data"]["tools"],
        json!(["Read", "Glob", "Grep", "mcp__fixture__search"])
    );
    let options = sidecar.observed_options();
    assert_eq!(options["strictMcpConfig"], true);
    assert!(
        options.get("allowedTools").is_none(),
        "MCP must never be auto-allowed: {options}"
    );
    let server = &options["mcpServers"]["fixture"];
    assert_eq!(server["type"], "stdio");
    assert_eq!(server["command"], "/usr/bin/node");
    assert_eq!(server["alwaysLoad"], true);
    assert!(
        server["env"].is_object(),
        "MCP env must be an explicit object, never inherited process.env: {server}"
    );
    let env_keys: Vec<_> = server["env"]
        .as_object()
        .expect("MCP env is an object")
        .keys()
        .cloned()
        .collect();
    for key in ["PATH", "HOME"] {
        assert!(
            env_keys.iter().any(|existing| existing == key),
            "{key} must be copied into the explicit MCP env"
        );
    }
    assert!(
        !env_keys.iter().any(|key| key == "ANTHROPIC_API_KEY"),
        "credentials must not reach MCP env: {server}"
    );
    assert!(
        sidecar
            .observed_control_calls()
            .contains(&"mcpServerStatus".to_owned())
    );

    sidecar.command("query-1", "query", json!({"text": "search it"}));
    let request = sidecar.next_callback();
    assert_eq!(request["toolName"], "mcp__fixture__search");
    sidecar.respond_callback(request["id"].as_str().expect("callback id"), "allow");
    let admissions = sidecar.admissions(&["mcp__fixture__search", "mcp__fixture__echo"]);
    assert_eq!(admissions["mcp__fixture__search"]["behavior"], "allow");
    assert_eq!(admissions["mcp__fixture__echo"]["behavior"], "deny");
    assert_eq!(
        sidecar.callback_tool_names(),
        vec!["mcp__fixture__search".to_owned()],
        "an unadmitted MCP tool must never reach the host"
    );
    assert_eq!(
        sidecar.observed_mcp_hits(),
        vec!["mcp__fixture__search".to_owned()],
        "a denied MCP call must never reach the server"
    );
}

#[test]
fn an_undeclared_mcp_server_name_is_rejected_before_the_sdk_is_constructed() {
    let mut sidecar = SidecarProcess::start();
    let cwd = sidecar.cwd();
    let open = sidecar.command(
        "open-1",
        "open",
        json!({
            "cwd": cwd,
            "model": "m-1",
            "tools": ["Read", "Glob", "Grep", "mcp__secret__search"],
            "permissionMode": "default"
        }),
    );
    assert_eq!(open["success"], false, "undeclared MCP open: {open}");
    assert_eq!(open["failure"]["code"], "mcp_server_undeclared");
    assert!(!sidecar.sdk_was_constructed());
}

#[test]
fn a_failing_required_mcp_server_fails_open() {
    let mut sidecar = SidecarProcess::start_scenario("mcp-required-fail");
    let cwd = sidecar.cwd();
    let open = sidecar.command(
        "open-1",
        "open",
        fixture_mcp_open(&cwd, &["mcp__fixture__search"], false),
    );
    assert_eq!(open["success"], false, "required MCP failure: {open}");
    assert_eq!(open["failure"]["code"], "mcp_server_failed");
}

#[test]
fn an_optional_mcp_server_failure_is_recorded_without_failing_open() {
    let mut sidecar = SidecarProcess::start_scenario("mcp-optional-fail");
    let cwd = sidecar.cwd();
    let open = sidecar.command(
        "open-1",
        "open",
        fixture_mcp_open(&cwd, &["mcp__fixture__search"], true),
    );
    assert_eq!(open["success"], true, "optional MCP failure open: {open}");
    assert_eq!(
        open["data"]["mcpServerStatus"],
        json!([{
            "name": "fixture",
            "status": "failed",
            "failureCode": "mcp_server_failed"
        }])
    );
    let options = sidecar.observed_options();
    assert_eq!(options["mcpServers"]["fixture"]["alwaysLoad"], false);
}
