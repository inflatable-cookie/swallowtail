use crate::support;

use serde_json::Value;
use support::{Direction, methods, parse_json, parse_transcript};
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

const RANGE: &str = include_str!("fixtures/acp-v1-grok-build-range-2026-07-24/release-corpus.json");
const BASE_PROTOCOL: &str = include_str!("fixtures/acp-v1-grok-build-0.2.0/protocol.json");
const BASE_INITIALIZE: &str = include_str!("fixtures/acp-v1-grok-build-0.2.0/initialize.ndjson");
const BASE_AUTH_REQUIRED: &str =
    include_str!("fixtures/acp-v1-grok-build-0.2.0/auth-required.ndjson");
const LATEST_PROTOCOL: &str = include_str!("fixtures/acp-v1-grok-build-0.2.111/protocol.json");
const LATEST_INITIALIZE: &str =
    include_str!("fixtures/acp-v1-grok-build-0.2.111/initialize.ndjson");
const LATEST_AUTH_REQUIRED: &str =
    include_str!("fixtures/acp-v1-grok-build-0.2.111/auth-required.ndjson");
const QUALIFIED_RANGE: &str =
    include_str!("fixtures/acp-v1-grok-build-range-2026-07-30/release-corpus.json");
const QUALIFIED_PROTOCOL: &str = include_str!("fixtures/acp-v1-grok-build-0.2.114/protocol.json");
const QUALIFIED_INITIALIZE: &str =
    include_str!("fixtures/acp-v1-grok-build-0.2.114/initialize.ndjson");
const QUALIFIED_ACTIVATE: &str = include_str!("fixtures/acp-v1-grok-build-0.2.114/activate.ndjson");

#[test]
fn release_snapshot_has_two_exact_runs_and_no_qualified_range() {
    let corpus = parse_json(RANGE);
    let publication = &corpus["publication"];
    assert_eq!(publication["first"], "0.2.0");
    assert_eq!(publication["last"], "0.2.111");
    assert_eq!(publication["count"], 111);
    assert_eq!(publication["missing"], serde_json::json!(["0.2.48"]));

    let runs = publication["published_patch_runs"]
        .as_array()
        .expect("published runs");
    let count: u64 = runs
        .iter()
        .map(|run| {
            run["last"].as_u64().expect("run end") - run["first"].as_u64().expect("run start") + 1
        })
        .sum();
    assert_eq!(count, 111);

    assert_eq!(corpus["distribution_tags"]["launcher"]["latest"], "0.2.111");
    assert_eq!(corpus["distribution_tags"]["platform"]["latest"], "0.1.220");
    assert_eq!(corpus["distribution_tags"]["platform"]["alpha"], "0.2.111");
    assert_eq!(corpus["qualification"]["segments"], serde_json::json!([]));
    assert_eq!(
        corpus["qualification"]["qualified_releases"],
        serde_json::json!([])
    );
    assert_eq!(corpus["qualification"]["unverified_newer"], false);
}

#[test]
fn exact_artifact_sdk_wire_and_version_axes_stay_separate() {
    let base = parse_json(BASE_PROTOCOL);
    let latest = parse_json(LATEST_PROTOCOL);

    assert_release(
        &base,
        "0.2.0",
        "d89b1a2fa7a",
        "0.6.0",
        Some("0.5.0"),
        "be4db9c6dd288dce2c5d8f130421769872046e5208b6c6457679e692286dfd57",
    );
    assert_release(
        &latest,
        "0.2.111",
        "94172f2aa4e5",
        "0.10.4",
        None,
        "e1fafdfffe14f339460befaf194360e8f90bfd02efe8a4f24cfa1c7aea657ffe",
    );
    assert_eq!(base["artifacts"]["codesign_team"], "5Y6N3AJ54S");
    assert_eq!(latest["artifacts"]["codesign_team"], "5Y6N3AJ54S");
    assert_eq!(
        base["invocation"]["direct_executable_argv"],
        serde_json::json!(["--no-auto-update", "agent", "stdio"])
    );
    assert_eq!(
        latest["invocation"]["direct_executable_argv"],
        serde_json::json!(["--no-auto-update", "agent", "stdio"])
    );
}

#[test]
fn only_the_latest_inspected_short_version_probe_is_side_effect_free() {
    let base = parse_json(BASE_PROTOCOL);
    let latest = parse_json(LATEST_PROTOCOL);

    assert_eq!(base["version_observation"]["state_files_created"], 21);
    assert_eq!(base["version_observation"]["safe_for_contract_032"], false);
    assert_eq!(latest["version_observation"]["state_files_created"], 0);
    assert_eq!(
        latest["version_observation"]["stdout"],
        "grok 0.2.111 (94172f2aa4e5)"
    );
    assert_eq!(latest["version_observation"]["safe_for_contract_032"], true);
    for fixture in [&base, &latest] {
        assert_eq!(
            fixture["invocation"]["npm_launcher_may_materialize_executable_in_grok_home"],
            true
        );
    }
}

#[test]
fn both_exact_agents_accept_read_only_client_capabilities_but_require_auth() {
    for (version, initialize, auth_required) in [
        ("0.2.0", BASE_INITIALIZE, BASE_AUTH_REQUIRED),
        ("0.2.111", LATEST_INITIALIZE, LATEST_AUTH_REQUIRED),
    ] {
        let init = parse_transcript(initialize).expect("initialize transcript parses");
        assert_eq!(methods(&init), ["initialize"]);
        assert_eq!(init[0].direction(), Direction::ClientToAgent);
        assert_eq!(init[0].id(), init[1].id());
        assert_eq!(
            init[0].message()["params"]["protocolVersion"],
            ACP_PROTOCOL_VERSION
        );
        assert_eq!(
            init[0].message()["params"]["clientCapabilities"]["fs"]["readTextFile"],
            true
        );
        assert_eq!(
            init[0].message()["params"]["clientCapabilities"]["fs"]["writeTextFile"],
            false
        );
        assert_eq!(
            init[1].message()["result"]["_meta"]["agentVersion"],
            version
        );
        assert_eq!(
            init[1].message()["result"]["authMethods"][0]["id"],
            "grok.com"
        );

        let auth = parse_transcript(auth_required).expect("auth transcript parses");
        assert_eq!(methods(&auth), ["session/new"]);
        assert_eq!(auth[0].id(), auth[1].id());
        assert_eq!(auth[1].message()["error"]["code"], -32000);
        assert_eq!(
            auth[1].message()["error"]["message"],
            "Authentication required"
        );
    }
}

#[test]
fn model_and_reasoning_drift_prevent_one_inferred_behavior_segment() {
    let base = parse_json(BASE_PROTOCOL);
    let latest = parse_json(LATEST_PROTOCOL);

    assert_eq!(base["model"]["current"], "grok-build-latest");
    assert_eq!(base["model"]["reasoning_efforts"], serde_json::json!([]));
    assert_eq!(latest["model"]["current"], "grok-4.5");
    assert_eq!(
        latest["model"]["reasoning_efforts"],
        serde_json::json!(["low", "medium", "high"])
    );
    assert!(
        parse_json(RANGE)["observed_milestones"]
            .as_array()
            .expect("milestones")
            .iter()
            .all(|milestone| milestone["exact_transition"].is_null())
    );
}

#[test]
fn current_auth_and_permission_evidence_stops_production_qualification() {
    let latest = parse_json(LATEST_PROTOCOL);

    assert_eq!(
        latest["authentication"]["advertised_without_credential"],
        serde_json::json!(["grok.com"])
    );
    assert_eq!(
        latest["authentication"]["public_example_expected"],
        serde_json::json!(["xai.api_key", "cached_token"])
    );
    assert_eq!(
        latest["authentication"]["authentication_request_observed"],
        false
    );
    assert_eq!(latest["authentication"]["qualified"], false);
    assert_eq!(latest["configuration"]["posture"], "ambient");
    assert_eq!(
        latest["configuration"]["plan_mode_allows_shell_and_subagent_bypass"],
        true
    );
    assert_eq!(latest["configuration"]["hooks_may_fail_open"], true);
    assert_eq!(latest["configuration"]["bounded_read_only_claim"], false);
}

#[test]
fn exact_authenticated_release_defines_one_qualified_segment() {
    let range = parse_json(QUALIFIED_RANGE);
    let protocol = parse_json(QUALIFIED_PROTOCOL);

    assert_eq!(
        range["qualification"]["qualified_releases"],
        serde_json::json!(["0.2.114"])
    );
    assert_eq!(
        range["qualification"]["segments"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        range["qualification"]["uninspected_older"],
        serde_json::json!(["0.2.112", "0.2.113"])
    );
    assert_eq!(range["qualification"]["unverified_newer_allowed"], true);
    assert_eq!(range["qualification"]["prerelease_allowed"], false);
    assert_eq!(protocol["qualification"]["guaranteed_baseline"], "0.2.114");
    assert_eq!(protocol["qualification"]["latest_qualified"], "0.2.114");
}

#[test]
fn qualified_release_activates_only_the_existing_cached_token() {
    let initialize = parse_transcript(QUALIFIED_INITIALIZE).expect("initialize parses");
    let activate = parse_transcript(QUALIFIED_ACTIVATE).expect("activation parses");
    let protocol = parse_json(QUALIFIED_PROTOCOL);

    assert_eq!(methods(&initialize), ["initialize"]);
    assert_eq!(
        initialize[1].message()["result"]["_meta"]["agentVersion"],
        "0.2.114"
    );
    assert_eq!(
        initialize[1].message()["result"]["_meta"]["defaultAuthMethodId"],
        "cached_token"
    );
    let auth_methods = initialize[1].message()["result"]["authMethods"]
        .as_array()
        .unwrap()
        .iter()
        .map(|method| method["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(auth_methods, ["cached_token", "grok.com"]);

    assert_eq!(methods(&activate), ["authenticate"]);
    assert_eq!(activate[0].message()["params"]["methodId"], "cached_token");
    assert_eq!(activate[0].message()["params"]["_meta"]["headless"], true);
    assert_eq!(activate[1].message()["result"], serde_json::json!({}));

    assert_eq!(protocol["authentication"]["credential_file_changed"], false);
    assert_eq!(
        protocol["authentication"]["provider_private_response_metadata_discarded"],
        true
    );
    assert_eq!(
        protocol["authentication"]["login_or_api_key_fallback"],
        false
    );
}

#[test]
fn qualified_artifact_matches_the_installed_signed_executable() {
    let protocol = parse_json(QUALIFIED_PROTOCOL);
    assert_eq!(protocol["release"]["source_revision"], "0c785038798");
    assert_eq!(protocol["release"]["channel"], "stable");
    assert_eq!(
        protocol["artifacts"]["executable_sha256"],
        "e715f57f9018a1737c1a64ef1cb260ac2a5045dfa6a1a0e1c7a7cbe193a083b2"
    );
    assert_eq!(
        protocol["version_observation"]["stdout"],
        "grok 0.2.114 (0c785038798) [stable]"
    );
    assert_eq!(
        protocol["invocation"]["direct_executable_argv"],
        serde_json::json!(["--no-auto-update", "agent", "stdio"])
    );
}

#[test]
fn normalized_corpus_contains_no_host_or_credential_material() {
    for fixture in [
        RANGE,
        BASE_PROTOCOL,
        BASE_INITIALIZE,
        BASE_AUTH_REQUIRED,
        LATEST_PROTOCOL,
        LATEST_INITIALIZE,
        LATEST_AUTH_REQUIRED,
        QUALIFIED_RANGE,
        QUALIFIED_PROTOCOL,
        QUALIFIED_INITIALIZE,
        QUALIFIED_ACTIVATE,
    ] {
        for forbidden in [
            "Toms-MacBook-Pro",
            "/Users/",
            "auth.json",
            "@inflatablecookie",
            "team_id",
            "subscription_tier",
            "XAI_API_KEY=",
            "Bearer ",
            "xai-secret-",
        ] {
            assert!(!fixture.contains(forbidden), "fixture leaked {forbidden}");
        }
    }
}

fn assert_release(
    fixture: &Value,
    version: &str,
    source: &str,
    sdk: &str,
    schema: Option<&str>,
    executable_sha256: &str,
) {
    assert_eq!(fixture["release"]["package_version"], version);
    assert_eq!(fixture["release"]["executable_version"], version);
    assert_eq!(fixture["release"]["source_revision"], source);
    assert_eq!(fixture["release"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(fixture["release"]["bundled_acp_sdk"], sdk);
    match schema {
        Some(schema) => assert_eq!(fixture["release"]["bundled_acp_schema"], schema),
        None => assert!(fixture["release"]["bundled_acp_schema"].is_null()),
    }
    assert_eq!(fixture["artifacts"]["executable_sha256"], executable_sha256);
}
