//! Exercises the fake-backed native-proof harness for `claude-agent.sdk`.
//!
//! Every case runs the shipped sidecar under real Node against a fake SDK that
//! speaks the native stdio wire, a fake native process, and a loopback endpoint.
//! No package is installed, no credential exists, no provider is contacted and
//! no vendor artifact is executed.
//!
//! What these tests prove is the harness: exact identity admission, the native
//! argv and environment, the initialize handshake, streamed input, result and
//! usage projection, the permission round trip, pre-callback denial, interrupt,
//! native exit, joined cleanup and egress silence. They prove nothing about the
//! real native; that is what the disabled original mode in the plan is for.

mod native_proof_support;
mod sdk_support;
// Every integration binary compiles the shared fixture tree but uses a subset.
#[allow(dead_code)]
mod sidecar_asset_support;

use native_proof_support::{
    FixtureOptions, Mode, ModeRefusal, NativeProofFixture, PLAN_SCHEMA, SELECTED_FLOW, arg_value,
    arg_with_value, argv_strings, forbidden_environment_hits, hops, plan, resolve_mode,
    verify_plan_against_fixtures,
};
use sdk_support::{SdkFixtureHost, SdkScenario, cleanup_request, preparation};
use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_PACKAGE_AXIS, ClaudeAgentSdkPackageNativePair,
    prepare_claude_agent_sdk_session,
};
use swallowtail_runtime::InteractiveSessionHandle;

/// The only pair the prepared driver and the shipped sidecar admit today.
const ACCEPTED_SDK: &str = "0.3.317";
const ACCEPTED_NATIVE: &str = "2.1.317";

fn fixture(scenario: &str) -> NativeProofFixture {
    NativeProofFixture::start(&FixtureOptions::new(
        scenario,
        ACCEPTED_SDK,
        ACCEPTED_NATIVE,
    ))
}

fn open(fixture: &mut NativeProofFixture) -> Value {
    let cwd = fixture.sidecar().cwd();
    fixture.sidecar().command(
        "open-1",
        "open",
        json!({
            "cwd": cwd,
            "model": "claude-sonnet-5-20250929",
            "permissionMode": "default",
            "expectedSdkVersion": ACCEPTED_SDK,
            "expectedNativeVersion": ACCEPTED_NATIVE,
        }),
    )
}

fn close(fixture: &mut NativeProofFixture) -> Value {
    fixture
        .sidecar()
        .command("close-1", "close", json!({"joinBoundMs": 2_000}))
}

/// Closes the session, then reads the native's own exit record. The native only
/// exits when the sidecar ends input at close, which is the joined-cleanup
/// contract this harness observes.
fn exit_after_close(fixture: &mut NativeProofFixture) -> Value {
    let closed = close(fixture);
    assert_eq!(closed["success"], true, "close joins the session: {closed}");
    fixture.wait_for_native_exit()
}

#[test]
fn every_planned_digest_matches_the_committed_identity_ledgers() {
    verify_plan_against_fixtures().expect("the probe plan agrees with every fixture");
    let planned = hops();
    assert_eq!(
        planned.len(),
        5,
        "the plan covers the accepted base and every published hop: {planned:?}"
    );
    // The committed classification covers the same hops, and the schema pins
    // the tag the plan carries.
    let flow: Value = serde_json::from_str(SELECTED_FLOW).expect("selected flow is JSON");
    let classified: Vec<&str> = flow["versions"]
        .as_array()
        .expect("classified versions")
        .iter()
        .map(|value| value.as_str().expect("version"))
        .collect();
    for hop in &planned {
        assert!(
            classified.contains(&hop.sdk_version.as_str()),
            "hop {} has a selected-flow classification",
            hop.sdk_version
        );
    }
    let schema: Value = serde_json::from_str(PLAN_SCHEMA).expect("plan schema is JSON");
    assert_eq!(
        schema["properties"]["schema"]["const"],
        plan()["schema"],
        "the schema pins the plan's schema tag"
    );
    for hop in &planned {
        let platforms: Vec<&str> = hop
            .platforms
            .iter()
            .map(|row| row.platform.as_str())
            .collect();
        assert_eq!(
            platforms,
            vec!["darwin-arm64", "linux-x64"],
            "hop {} names both selected platforms",
            hop.sdk_version
        );
        for row in &hop.platforms {
            assert_eq!(
                row.binary_sha256.len(),
                64,
                "digest is a SHA-256 hex string"
            );
            assert!(row.binary_bytes > 0, "the artifact size is recorded");
        }
    }
}

#[test]
fn the_plan_keeps_original_execution_disabled_behind_an_explicit_authority() {
    let plan = plan();
    assert_eq!(
        plan["authorized_stage"]["original_artifact_execution"]["enabled"],
        json!(false)
    );
    assert_eq!(
        resolve_mode(&plan, None, None),
        Ok(Mode::Fake),
        "no request means the fake harness"
    );
    assert_eq!(resolve_mode(&plan, Some("fake"), None), Ok(Mode::Fake));
    assert_eq!(
        resolve_mode(&plan, Some("original"), Some("anything")),
        Err(ModeRefusal::Disabled),
        "a token cannot enable a plan that disables the stage"
    );
    assert_eq!(
        resolve_mode(&plan, Some("original"), None),
        Err(ModeRefusal::Disabled)
    );

    let mut enabled = plan.clone();
    enabled["authorized_stage"]["original_artifact_execution"]["enabled"] = json!(true);
    let required = enabled["authorized_stage"]["original_artifact_execution"]["required_authority"]
        .as_str()
        .expect("plan names the required authority")
        .to_owned();
    assert_eq!(
        resolve_mode(&enabled, Some("original"), None),
        Err(ModeRefusal::AuthorityMissing)
    );
    assert_eq!(
        resolve_mode(&enabled, Some("original"), Some("wrong-token")),
        Err(ModeRefusal::AuthorityMissing)
    );
    assert_eq!(
        resolve_mode(&enabled, Some("original"), Some(&required)),
        Ok(Mode::Original),
        "the gate opens only for the named authority"
    );
    assert_eq!(
        resolve_mode(&plan, Some("orignal"), None),
        Err(ModeRefusal::UnknownMode("orignal".to_owned()))
    );
}

#[test]
fn the_prepared_driver_and_sidecar_still_refuse_every_unqualified_hop() {
    let maintained: Vec<&str> = ClaudeAgentSdkPackageNativePair::MAINTAINED
        .iter()
        .map(|pair| pair.package_version())
        .collect();
    for hop in hops() {
        if hop.sdk_version == ACCEPTED_SDK {
            assert!(
                maintained.contains(&ACCEPTED_SDK),
                "the accepted pair stays admitted"
            );
            continue;
        }
        assert!(
            !maintained.contains(&hop.sdk_version.as_str()),
            "hop {} must not be admitted on claim evidence alone",
            hop.sdk_version
        );
        let host = swallowtail_core::ExecutionHostId::new(format!(
            "claude-agent-sdk.native-proof.{}",
            hop.sdk_version
        ))
        .expect("fixture host id is valid");
        assert!(
            prepare_claude_agent_sdk_session(
                preparation(host)
                    .with_package_native_pair(ClaudeAgentSdkPackageNativePair::MAINTAINED[0]),
                swallowtail_runtime::SessionOptions::default(),
            )
            .is_ok(),
            "the accepted pair still prepares"
        );
    }

    // The sidecar enforces the same ceiling: a planned hop above it fails
    // before the SDK module loads and before any native process starts.
    let mut sidecar = NativeProofFixture::start(&FixtureOptions::new("ok", "0.3.322", "2.1.322"));
    let cwd = sidecar.sidecar().cwd();
    let refusal = sidecar.sidecar().command(
        "open-1",
        "open",
        json!({
            "cwd": cwd,
            "model": "claude-sonnet-5-20250929",
            "expectedSdkVersion": "0.3.322",
            "expectedNativeVersion": "2.1.322",
        }),
    );
    assert_eq!(refusal["success"], false, "the hop is refused: {refusal}");
    assert_eq!(refusal["failure"]["code"], "sdk_version_mismatch");
    assert!(!sidecar.sidecar().sdk_was_constructed());
    assert!(
        !sidecar.root().join("native-observations.json").exists(),
        "no native process ran for a refused pair"
    );
}

#[test]
fn the_prepared_driver_launch_reaches_the_real_sidecar_and_native_wire() {
    let host_id = swallowtail_core::ExecutionHostId::new("claude-agent-sdk.native-proof.driver")
        .expect("fixture host id is valid");
    let pair = ClaudeAgentSdkPackageNativePair::MAINTAINED
        .iter()
        .copied()
        .find(|pair| pair.package_version() == ACCEPTED_SDK)
        .expect("the accepted pair is maintained");
    let prepared = prepare_claude_agent_sdk_session(
        preparation(host_id.clone()).with_package_native_pair(pair),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("accepted pair prepares");
    let versions: Vec<(&str, &str)> = prepared
        .plan()
        .interface_versions()
        .map(|binding| (binding.axis().as_str(), binding.version().as_str()))
        .collect();
    assert!(versions.contains(&(CLAUDE_AGENT_SDK_PACKAGE_AXIS, ACCEPTED_SDK)));

    let fixture_host = SdkFixtureHost::new(SdkScenario::Complete);
    let services = fixture_host.services(host_id);
    let session = futures_executor::block_on(prepared.open_route_session(services.clone()))
        .expect("prepared facade opens the accepted pair");
    let launch = fixture_host
        .inputs()
        .into_iter()
        .find(|record| record["command"] == "open")
        .expect("the driver emitted its open launch record");
    let cleanup = futures_executor::block_on(Box::new(session).close(cleanup_request(), services));
    assert!(matches!(
        cleanup,
        swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::Degraded(_)
    ));

    // The same driver-produced launch parameters now go to the real sidecar,
    // which drives the fake SDK over the native wire.
    let mut fixture = fixture("ok");
    // The driver supplies the workspace; the probe runs it in the fixture's own
    // isolated directory, so the launch shape is the driver's and the paths are
    // the harness's.
    let mut params = launch["params"].clone();
    params["cwd"] = json!(fixture.root().join("workspace").to_string_lossy());
    let opened = fixture.sidecar().command("open-1", "open", params);
    assert_eq!(opened["success"], true, "sidecar opens the pair: {opened}");
    assert_eq!(opened["data"]["sdkVersion"], ACCEPTED_SDK);
    assert_eq!(opened["data"]["nativeVersion"], ACCEPTED_NATIVE);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    let terminal = fixture.sidecar().wait_for_turn_end_record();
    assert_eq!(terminal["subtype"], "success");
    let exit = exit_after_close(&mut fixture);
    assert_eq!(exit["exitCode"], 0);
}

#[test]
fn the_native_receives_the_selected_argv_and_none_of_the_deselected_flags() {
    let mut fixture = fixture("ok");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    fixture.sidecar().wait_for_turn_end();
    let observations = exit_after_close(&mut fixture);
    let argv = argv_strings(&observations);
    assert!(argv.contains(&json!("--output-format")));
    assert!(argv.contains(&json!("stream-json")));
    assert!(argv.contains(&json!("--input-format")));
    assert!(argv.contains(&json!("--verbose")));
    assert_eq!(
        arg_value(&argv, "--model"),
        Some(&json!("claude-sonnet-5-20250929"))
    );
    assert_eq!(
        arg_value(&argv, "--permission-mode"),
        Some(&json!("default"))
    );
    assert!(
        arg_with_value(&argv, "--permission-prompt-tool").is_some(),
        "the callback is transported over stdio: {argv:?}"
    );
    assert_eq!(
        arg_value(&argv, "--setting-sources"),
        Some(&json!("")),
        "the empty setting sources reach the native as an empty list"
    );
    assert!(argv.contains(&json!("--strict-mcp-config")));
    assert!(
        arg_value(&argv, "--tools").is_some() || arg_with_value(&argv, "--tools").is_some(),
        "the admitted tool set is explicit"
    );
    for deselected in ["--await-initialize", "--managed-settings", "--allowedTools"] {
        assert!(
            arg_with_value(&argv, deselected).is_none(),
            "{deselected} must not be emitted on this route: {argv:?}"
        );
    }
    assert!(arg_with_value(&argv, "--include-partial-messages").is_none());
}

#[test]
fn the_native_initialize_handshake_and_streamed_input_are_observed() {
    let mut fixture = fixture("ok");
    let opened = open(&mut fixture);
    assert_eq!(
        opened["success"], true,
        "open handshake completes: {opened}"
    );
    let observations = fixture.native_observations();
    assert_eq!(
        observations["initializeRequests"].as_array().map(Vec::len),
        Some(1),
        "exactly one initialize request crosses the wire"
    );
    assert_eq!(
        observations["initializeRequests"][0]["request_id"],
        json!("fixture-sdk-0")
    );

    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    let terminal = fixture.sidecar().wait_for_turn_end_record();
    assert_eq!(terminal["subtype"], "success");
    let observations = exit_after_close(&mut fixture);
    assert_eq!(
        observations["userMessages"].as_array().map(Vec::len),
        Some(1),
        "one streamed user message reached the native"
    );
    assert_eq!(
        observations["userMessages"][0]["contentTypes"],
        json!(["text"])
    );
    assert_eq!(observations["interruptRequests"], 0);
}

#[test]
fn result_and_usage_frames_cross_the_native_wire_and_project() {
    let mut fixture = fixture("ok");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    let terminal = fixture.sidecar().wait_for_turn_end_record();
    assert_eq!(terminal["subtype"], "success");
    assert_eq!(terminal["isError"], false);
    assert_eq!(terminal["numTurns"], 1);
    assert_eq!(terminal["durationMs"], 7);
    assert_eq!(
        terminal["usage"],
        json!({
            "inputTokens": 21,
            "outputTokens": 5,
            "cacheReadInputTokens": 8,
            "cacheWriteInputTokens": 9,
        }),
        "the native usage snapshot is projected with each turn: {terminal}"
    );
    let observations = exit_after_close(&mut fixture);
    assert_eq!(
        observations["events"]
            .as_array()
            .expect("native events")
            .iter()
            .filter(|event| event["event"] == "user_received")
            .count(),
        1
    );
}

#[test]
fn a_failure_result_stays_typed_and_carries_no_provider_text() {
    let mut fixture = fixture("error-result");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    let terminal = fixture.sidecar().wait_for_turn_end_record();
    assert_eq!(terminal["subtype"], "error_during_execution");
    assert_eq!(terminal["isError"], true);
    assert!(
        !serde_json::to_string(&terminal)
            .expect("terminal record serializes")
            .contains("fixture provider detail"),
        "provider text never crosses the wire: {terminal}"
    );
    assert_eq!(close(&mut fixture)["success"], true);
}

#[test]
fn a_permission_callback_round_trip_reaches_the_native() {
    let mut fixture = fixture("permission");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "write a file"}));
    let callback = fixture.sidecar().next_callback();
    assert_eq!(callback["toolName"], "Read");
    assert_eq!(
        fixture.native_observations()["permissionRequestsSent"],
        json!(1),
        "the native asked before the callback answered"
    );
    let identifier = callback["id"].as_str().expect("callback id").to_owned();
    fixture.sidecar().respond_callback(&identifier, "allow");
    fixture.sidecar().wait_for_turn_end();
    let observations = exit_after_close(&mut fixture);
    assert_eq!(observations["permissionRequestsSent"], json!(1));
    assert_eq!(
        observations["permissionAnswers"].as_array().map(Vec::len),
        Some(1),
        "the callback decision was written back to the native"
    );
    let sdk = fixture.sdk_observations();
    assert_eq!(sdk["permissionAnswers"].as_array().map(Vec::len), Some(1));
}

#[test]
fn a_pre_callback_denial_fabricates_no_permission_request() {
    let mut fixture = fixture("managed-deny");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "write a file"}));
    let terminal = fixture.sidecar().wait_for_turn_end_record();
    assert_eq!(terminal["subtype"], "success");
    assert_eq!(
        fixture.sidecar().callback_tool_names(),
        Vec::<String>::new(),
        "a pre-callback denial never becomes a host permission request"
    );
    let observations = exit_after_close(&mut fixture);
    assert_eq!(observations["permissionRequestsSent"], json!(0));
    assert_eq!(
        observations["permissionAnswers"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn interrupt_reaches_the_native_and_the_turn_settles() {
    let mut fixture = fixture("abortable-turn");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "a long turn"}));
    let response = fixture
        .sidecar()
        .command("interrupt-1", "interrupt", json!({}));
    assert_eq!(
        response["success"], true,
        "interrupt is answered: {response}"
    );
    assert_eq!(response["data"]["interrupted"], true);
    let terminal = fixture.sidecar().wait_for_turn_end_record();
    assert_eq!(
        terminal["terminalReason"], "aborted_streaming",
        "the native's abort reason reaches the projection: {terminal}"
    );
    let observations = exit_after_close(&mut fixture);
    assert_eq!(
        observations["interruptRequests"],
        json!(1),
        "the native received one interrupt control request"
    );
    assert_eq!(
        fixture.wait_for_sdk_native_exit()["interruptCalls"],
        json!(1)
    );
}

#[test]
fn a_native_that_dies_mid_session_is_observed_and_the_session_turns_terminal() {
    let mut fixture = fixture("crash-after-turn");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    let observations = fixture.wait_for_native_exit();
    assert_eq!(
        observations["exitCode"],
        json!(3),
        "the native boundary records the real exit code: {observations}"
    );
    assert!(
        observations["events"]
            .as_array()
            .expect("native events")
            .iter()
            .any(|event| event["event"] == "crash"),
        "the peer recorded its own death"
    );
    assert_eq!(
        fixture.wait_for_sdk_native_exit()["nativeExit"]["code"],
        json!(3),
        "the SDK also observed the native's exit"
    );

    // The session must not keep pretending it is usable: a later command either
    // fails, produces a terminal record, or finds the wire closed. It never
    // answers as if nothing happened.
    fixture.sidecar().send_without_reply(json!({
        "type": "command",
        "id": "query-2",
        "command": "query",
        "params": {"text": "second turn"},
    }));
    let mut terminal = false;
    while let Some(record) = fixture
        .sidecar()
        .next_record_within(std::time::Duration::from_secs(10))
    {
        if record["type"] == "terminal" {
            terminal = true;
            break;
        }
        if record["type"] == "response" && record["id"] == "query-2" {
            assert_eq!(
                record["success"], false,
                "a dead native never yields a successful turn: {record}"
            );
            terminal = true;
            break;
        }
    }
    let closed_wire = fixture
        .sidecar()
        .next_record_within(std::time::Duration::from_secs(2))
        .is_none();
    assert!(
        terminal || closed_wire,
        "a dead native ends the session instead of leaving it silent"
    );
}

#[test]
fn identity_substitution_fails_closed_before_any_peer_runs() {
    let mut package = NativeProofFixture::start(&FixtureOptions {
        scenario: "ok",
        sdk_version: ACCEPTED_SDK,
        native_version: ACCEPTED_NATIVE,
        declared_sdk_version: Some("0.3.316"),
        declared_native_version: None,
    });
    let refused = open(&mut package);
    assert_eq!(
        refused["success"], false,
        "a substituted package fails: {refused}"
    );
    assert_eq!(refused["failure"]["code"], "sdk_version_mismatch");
    assert!(!package.sidecar().sdk_was_constructed());
    assert!(!package.root().join("native-observations.json").exists());

    let mut native = NativeProofFixture::start(&FixtureOptions {
        scenario: "ok",
        sdk_version: ACCEPTED_SDK,
        native_version: ACCEPTED_NATIVE,
        declared_sdk_version: None,
        declared_native_version: Some("2.1.316"),
    });
    let refused = open(&mut native);
    assert_eq!(
        refused["success"], false,
        "a substituted native fails: {refused}"
    );
    assert_eq!(refused["failure"]["code"], "native_version_mismatch");
    assert!(!native.root().join("native-observations.json").exists());
    assert_eq!(native.endpoint().contacts(), 0);
}

#[test]
fn the_fixture_environment_carries_no_credential_and_no_forward_proxy() {
    let mut fixture = fixture("ok");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    fixture.sidecar().wait_for_turn_end();
    let observations = exit_after_close(&mut fixture);
    let keys = fixture.native_environment_keys();
    assert_eq!(
        keys,
        observations["envKeys"]
            .as_array()
            .expect("native environment keys")
            .iter()
            .map(|value| value.as_str().expect("environment key").to_owned())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        forbidden_environment_hits(&keys),
        Vec::<String>::new(),
        "no provider credential or proxy variable reaches a peer: {keys:?}"
    );
    let home = observations["envKeys"]
        .as_array()
        .expect("native environment keys")
        .iter()
        .any(|value| value == "HOME");
    assert!(home, "the isolated HOME is present");
    let endpoint = fixture.endpoint();
    assert_eq!(
        endpoint.contacts(),
        1,
        "the only egress the fixture allows is its own loopback endpoint"
    );
    assert_eq!(
        endpoint.requests(),
        vec!["POST /v1/messages HTTP/1.1".to_owned()],
        "one controlled loopback request, nothing else"
    );
    assert!(
        endpoint
            .requests()
            .iter()
            .all(|request| !request.contains("Authorization")),
        "no credential header is sent"
    );
}

#[test]
fn the_harness_records_a_joined_cleanup_without_a_surviving_peer() {
    let mut fixture = fixture("ok");
    assert_eq!(open(&mut fixture)["success"], true);
    fixture
        .sidecar()
        .command("query-1", "query", json!({"text": "first turn"}));
    fixture.sidecar().wait_for_turn_end();
    let closed = close(&mut fixture);
    assert_eq!(closed["success"], true);
    let exit = fixture.wait_for_native_exit();
    assert_eq!(exit["exitCode"], 0, "the native's own exit is observed");
    let events = exit["events"].as_array().expect("native events");
    assert!(
        events.last().is_some_and(|event| event["event"] == "exit"),
        "the native recorded its own exit last: {events:?}"
    );
    assert!(
        fixture
            .sidecar()
            .fixture_directory()
            .join("native-observations.json")
            .exists(),
        "the observation file survives the run for the reviewer"
    );
}

// ---------------------------------------------------------------------------
// Temporary probe-only admission
// ---------------------------------------------------------------------------

use native_proof_support::{
    PROBE_ADMISSION_SCHEMA, ProbeFixture, probe_admission, probe_options, probe_request,
    selected_options_digest, verify_probe_admission,
};

#[test]
fn the_probe_admission_binds_every_tuple_and_cannot_admit_production() {
    verify_probe_admission().expect("the probe admission binds every digest it names");
    let admission = probe_admission();
    assert_eq!(admission["mode"], "probe-only");
    assert_eq!(admission["production_admission"], json!(false));
    assert_eq!(
        admission["authority"]["original_run_authorised"],
        json!(false)
    );
    assert_eq!(admission["authority"]["original_run_token"], Value::Null);
    let tuples = admission["tuples"].as_array().expect("admitted tuples");
    let planned = hops();
    assert_eq!(
        tuples.len(),
        planned.len(),
        "every planned artifact is admitted, including the accepted control"
    );
    let proposed: Vec<&str> = planned
        .iter()
        .map(|hop| hop.sdk_version.as_str())
        .filter(|version| *version != ACCEPTED_SDK)
        .collect();
    assert_eq!(proposed.len(), 4, "four proposed hops are admitted");
    let schema: Value = serde_json::from_str(PROBE_ADMISSION_SCHEMA).expect("admission schema");
    assert_eq!(schema["properties"]["mode"]["const"], "probe-only");
    assert_eq!(schema["properties"]["production_admission"]["const"], false);

    // Production admission stays closed for every admitted tuple: the claim
    // table does not contain them and the shipped sidecar refuses each one.
    let maintained: Vec<&str> = ClaudeAgentSdkPackageNativePair::MAINTAINED
        .iter()
        .map(|pair| pair.package_version())
        .collect();
    for tuple in tuples {
        let sdk_version = tuple["sdk_version"].as_str().expect("tuple sdk version");
        if sdk_version == ACCEPTED_SDK {
            assert!(
                maintained.contains(&sdk_version),
                "the admitted control stays the maintained pair"
            );
            continue;
        }
        assert!(
            !maintained.contains(&sdk_version),
            "the probe admission must not enter the maintained pair map"
        );
        let mut fixture = NativeProofFixture::start(&FixtureOptions::new(
            "ok",
            sdk_version,
            tuple["native_version"]
                .as_str()
                .expect("tuple native version"),
        ));
        let cwd = fixture.sidecar().cwd();
        let refusal = fixture.sidecar().command(
            "open-1",
            "open",
            json!({
                "cwd": cwd,
                "model": "claude-sonnet-5-20250929",
                "expectedSdkVersion": sdk_version,
                "expectedNativeVersion": tuple["native_version"],
            }),
        );
        assert_eq!(
            refusal["success"], false,
            "{sdk_version} must stay refused in production: {refusal}"
        );
        assert_eq!(refusal["failure"]["code"], "sdk_version_mismatch");
        assert!(!fixture.sidecar().sdk_was_constructed());
    }
}

#[test]
fn every_admitted_tuple_runs_a_positive_fake_probe() {
    let admission = probe_admission();
    for tuple in admission["tuples"].as_array().expect("admitted tuples") {
        let sdk_version = tuple["sdk_version"].as_str().expect("tuple sdk version");
        let native_version = tuple["native_version"]
            .as_str()
            .expect("tuple native version");
        for platform in ["darwin-arm64", "linux-x64"] {
            let fixture = ProbeFixture::start("ok", &|_| {});
            fixture.bind_peer(sdk_version, native_version);
            let run = fixture.run(sdk_version, platform, "ok", None, 120_000, &[]);
            assert_eq!(
                run.status, 0,
                "probe {sdk_version}/{platform} succeeds: {} {}",
                run.stdout, run.stderr
            );
            let evidence = run.evidence.expect("probe evidence is written");
            assert_eq!(evidence["probeOnly"], true);
            assert_eq!(evidence["productionAdmission"], false);
            assert_eq!(evidence["tuple"]["sdk_version"], sdk_version);
            assert_eq!(evidence["tuple"]["native_version"], native_version);
            assert_eq!(evidence["platform"], platform);
            assert_eq!(evidence["nativeExit"]["join"], "exited");
            assert_eq!(evidence["failure"], Value::Null);
            let records = evidence["records"].as_array().expect("probe records");
            assert!(
                records
                    .iter()
                    .any(|record| record["event"] == "initialized"),
                "the native initialize handshake is observed: {evidence}"
            );
            assert!(
                records
                    .iter()
                    .any(|record| record["event"] == "message" && record["type"] == "result"),
                "the native result frame is observed: {evidence}"
            );
            let native = fixture.native_observations();
            assert_eq!(native["exitCode"], json!(0));
            assert_eq!(
                native["userMessages"].as_array().map(Vec::len),
                Some(1),
                "one streamed user message reached the peer"
            );
        }
    }
}

#[test]
fn the_probe_option_surface_equals_the_shipped_sidecar_surface() {
    let mut fixture = fixture("ok");
    assert_eq!(open(&mut fixture)["success"], true);
    let sidecar_surface = fixture.sidecar().observed_options();
    assert_eq!(
        selected_options_digest(&sidecar_surface),
        probe_admission()["selected_options_digest"],
        "the probe template is exactly the sidecar's selected option surface: {sidecar_surface}"
    );
    assert_eq!(
        selected_options_digest(&probe_options()),
        probe_admission()["selected_options_digest"]
    );
    close(&mut fixture);
}

#[test]
fn a_substituted_probe_identity_is_refused_before_any_run() {
    let fixture = ProbeFixture::start("ok", &|_| {});
    // The admitted tuple is 0.3.322/2.1.322; the peer claims the accepted pair.
    fixture.bind_peer("0.3.317", "2.1.322");
    let run = fixture.run("0.3.322", "linux-x64", "ok", None, 30_000, &[]);
    assert_eq!(
        run.status, 2,
        "a substituted peer is refused: {}",
        run.stderr
    );
    assert!(
        run.stderr.contains("identity_substitution"),
        "the refusal names identity substitution: {}",
        run.stderr
    );
    assert!(run.evidence.is_none(), "no run is recorded");
    assert!(
        !fixture
            .root()
            .join("peer/native-observations.json")
            .exists(),
        "no native peer started"
    );

    let native = ProbeFixture::start("ok", &|_| {});
    native.bind_peer("0.3.322", "2.1.317");
    let run = native.run("0.3.322", "linux-x64", "ok", None, 30_000, &[]);
    assert_eq!(
        run.status, 2,
        "a substituted native is refused: {}",
        run.stderr
    );
    assert!(run.stderr.contains("identity_substitution"));
    assert!(!native.root().join("peer/native-observations.json").exists());
}

/// One refusal case for the probe-admission gate: the ledger override, the
/// extra runner arguments and the refusal code the run must name.
type RefusalCase = (
    &'static str,
    fn(&mut Value),
    &'static [&'static str],
    &'static str,
);

#[test]
fn a_ledger_that_claims_production_admission_is_refused() {
    let cases: [RefusalCase; 5] = [
        (
            "production admission",
            |admission: &mut Value| admission["production_admission"] = json!(true),
            &["--mode", "original", "--authority-token", "fixture-token"],
            "production_admission_refused",
        ),
        (
            "production mode",
            |admission: &mut Value| admission["mode"] = json!("production"),
            &["--mode", "original", "--authority-token", "fixture-token"],
            "production_admission_refused",
        ),
        (
            "foreign authority",
            |admission: &mut Value| admission["authority"]["kind"] = json!("operator-authority"),
            &[],
            "authority_missing",
        ),
        (
            "authority without a token",
            |admission: &mut Value| {
                admission["authority"]["original_run_authorised"] = json!(true);
                admission["authority"]["original_run_token"] = json!("fixture-token");
            },
            &["--mode", "original"],
            "authority_missing",
        ),
        (
            "authority with the wrong token",
            |admission: &mut Value| {
                admission["authority"]["original_run_authorised"] = json!(true);
                admission["authority"]["original_run_token"] = json!("fixture-token");
            },
            &["--mode", "original", "--authority-token", "another-token"],
            "authority_missing",
        ),
    ];
    for (label, overrides, extra, expected) in cases {
        let fixture = ProbeFixture::start("ok", &overrides);
        fixture.bind_peer("0.3.322", "2.1.322");
        let run = fixture.run("0.3.322", "darwin-arm64", "ok", None, 30_000, extra);
        assert_eq!(
            run.status, 2,
            "the {label} ledger is refused: {} {}",
            run.stdout, run.stderr
        );
        assert!(
            run.stderr.contains(expected),
            "the {label} refusal names {expected}: {}",
            run.stderr
        );
        assert!(run.evidence.is_none(), "no run is recorded for {label}");
    }

    // The gate is a real gate, not a stub: an authorised ledger with its exact
    // token opens original mode, which is why the committed ledger keeps both
    // fields closed and `verify_probe_admission` refuses if they drift.
    let authorised = ProbeFixture::start("ok", &|admission: &mut Value| {
        admission["authority"]["original_run_authorised"] = json!(true);
        admission["authority"]["original_run_token"] = json!("fixture-token");
    });
    authorised.bind_peer("0.3.322", "2.1.322");
    let run = authorised.run(
        "0.3.322",
        "linux-x64",
        "ok",
        None,
        30_000,
        &["--mode", "original", "--authority-token", "fixture-token"],
    );
    assert_eq!(
        run.status, 0,
        "the authority gate opens only for the exact token: {} {}",
        run.stdout, run.stderr
    );
    assert_eq!(
        run.evidence.expect("evidence")["mode"],
        "original",
        "the record truthfully identifies the mode that ran"
    );
}

#[test]
fn a_non_loopback_probe_endpoint_is_refused_before_spawn() {
    let fixture = ProbeFixture::start("ok", &|_| {});
    fixture.bind_peer("0.3.321", "2.1.321");
    let run = fixture.run(
        "0.3.321",
        "linux-x64",
        "ok",
        Some("example.invalid:443"),
        30_000,
        &[],
    );
    assert_eq!(
        run.status, 2,
        "a non-loopback endpoint is refused: {}",
        run.stderr
    );
    assert!(run.stderr.contains("non_loopback_endpoint"));
    assert!(
        !fixture
            .root()
            .join("peer/native-observations.json")
            .exists()
    );

    // The loopback endpoint is admitted and recorded as the only egress.
    let loopback = ProbeFixture::start("ok", &|_| {});
    loopback.bind_peer("0.3.318", "2.1.318");
    let run = loopback.run(
        "0.3.318",
        "linux-x64",
        "ok",
        Some("127.0.0.1:1"),
        120_000,
        &[],
    );
    assert_eq!(
        run.status, 0,
        "loopback is the admitted endpoint: {}",
        run.stderr
    );
    let evidence = run.evidence.expect("evidence");
    assert_eq!(evidence["endpoint"], "127.0.0.1:1");
}

#[test]
fn a_peer_that_outlives_the_bound_fails_the_probe_closed() {
    let fixture = ProbeFixture::start("never-exit", &|_| {});
    fixture.bind_peer("0.3.319", "2.1.319");
    let run = fixture.run("0.3.319", "darwin-arm64", "never-exit", None, 1_500, &[]);
    assert_eq!(
        run.status, 2,
        "an unjoined peer fails the probe: {}",
        run.stderr
    );
    assert!(
        run.stderr.contains("cleanup_failed"),
        "the refusal names the cleanup failure: {}",
        run.stderr
    );
    let evidence = run.evidence.expect("the probe still writes evidence");
    assert_eq!(
        evidence["nativeExit"]["join"], "killed",
        "the unjoined peer is killed, not assumed gone: {evidence}"
    );
    assert_eq!(evidence["cleanup"], "killed");
}

#[test]
fn the_probe_request_names_live_digests_and_the_exact_artifacts() {
    let request = probe_request();
    let digests = request["source_digests"]
        .as_object()
        .expect("request names source digests");
    assert!(
        digests.len() >= 5,
        "the request names every probe source: {digests:?}"
    );
    assert!(
        request["command"]
            .as_str()
            .expect("request names a command")
            .contains("probe-runner.mjs")
    );
    assert!(
        request["original_command_suffix"]
            .as_str()
            .expect("request names the original suffix")
            .contains("--authority-token")
    );
    let artifacts = request["artifacts"].as_array().expect("request artifacts");
    assert_eq!(
        artifacts.len(),
        5,
        "the request names the accepted control and every proposed hop"
    );
    for artifact in artifacts {
        for platform in ["darwin-arm64", "linux-x64"] {
            assert!(
                artifact["platforms"][platform]["binary_sha256"]
                    .as_str()
                    .is_some_and(|digest| digest.len() == 64),
                "every artifact names its platform digest"
            );
        }
    }
    // The request's bound digests are the live ones; verify_probe_admission
    // recomputes them and would have failed above if they had drifted.
    verify_probe_admission().expect("request digests are live");
}
