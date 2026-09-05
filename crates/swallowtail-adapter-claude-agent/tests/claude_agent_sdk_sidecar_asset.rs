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
    assert_eq!(spawn["env"], json!({}));
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

    let close = open_input.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true, "close response: {close}");

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
    assert!(
        !terminal.to_string().contains("fixture early input EOF"),
        "SDK error text must never cross the sidecar wire: {terminal}"
    );
    let close = early_eof.command("close-1", "close", json!({"joinBoundMs": 2_000}));
    assert_eq!(close["success"], true, "close response: {close}");
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
    assert_eq!(options["settingSources"], json!([]));
    assert_eq!(options["skills"], json!([]));
    assert_eq!(options["persistSession"], json!(false));
    assert_eq!(options["env"], json!({}));
    for forbidden in ["apiKeyHelper", "awsAuthRefresh", "gcpAuthRefresh"] {
        assert!(
            options.get(forbidden).is_none(),
            "{forbidden} must be unset"
        );
    }
    assert_eq!(options["executable"], "node");
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
