use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/pi-acp-negative/identity.json");
const PROTOCOL: &str = include_str!("fixtures/pi-acp-negative/protocol.json");
const CORPUS_PLAN: &str = include_str!("fixtures/pi-acp-negative/corpus-plan.json");

#[test]
fn pi_acp_closes_as_community_wrapper_over_existing_rpc() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "pi.package");
    assert_eq!(identity["route"], "pi.acp");
    assert_eq!(identity["disposition"], "negative");
    assert_eq!(
        identity["official_npm_package"],
        "@earendil-works/pi-coding-agent"
    );
    assert_eq!(identity["community_npm_package"], "pi-acp");
    assert_eq!(identity["official"]["version"], "0.84.2");
    assert_eq!(identity["official"]["native_acp_mode"], false);
    assert_eq!(identity["community"]["version"], "0.0.33");
    assert_eq!(
        identity["community"]["tarball_sha256"],
        "9fdeb8a6780c056b32c07242f359084472007308e1ab57757f3339dd9630de4b"
    );
    assert_eq!(identity["acp_registry"]["id"], "pi-acp");
    assert_eq!(identity["acp_registry"]["first_party"], false);
    assert_eq!(
        identity["host"]["executable_sha256"],
        "af302f231437eaf6f37691bce4b34234fcb626bcb5eb3910d4fc3f6519bf78ca"
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "negative-collapse-onto-existing-route");
    assert_eq!(decision["collapses_onto"], "pi.rpc");
    assert_eq!(decision["wrap_community_pi_acp"], false);
    assert_eq!(decision["add_wrapper_package"], false);
    assert_eq!(decision["start_driver_cards"], false);
    assert_eq!(decision["official_native_acp"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_acp_initialize"], false);
    assert_eq!(decision["live_rpc_session"], false);
    assert_eq!(decision["run_pi_auth"], false);
    assert_eq!(decision["claim_change_in_card"], false);
}

#[test]
fn community_adapter_spawn_is_pi_rpc_not_native_acp() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(protocol["official_acp_mode"], false);
    let modes = protocol["official_help_modes"]
        .as_array()
        .expect("official modes");
    assert!(modes.iter().any(|mode| mode == "rpc"));
    assert!(!modes.iter().any(|mode| mode == "acp"));
    assert_eq!(protocol["community_spawn"]["executable"], "pi");
    assert_eq!(
        protocol["community_spawn"]["args"],
        serde_json::json!(["--mode", "rpc", "--no-themes"])
    );
    assert_eq!(protocol["authority"]["swallowtail_wraps_pi_acp"], false);
    assert_eq!(protocol["authority"]["swallowtail_runs_pi_auth"], false);
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_rpc_session"], false);

    let plan: Value = serde_json::from_str(CORPUS_PLAN).expect("corpus plan");
    assert_eq!(plan["disposition"], "negative");
    assert_eq!(plan["collapses_onto"], "pi.rpc");
    assert_eq!(plan["first_driver_op"], "none");
    assert_eq!(plan["decoder_corpus"], serde_json::Value::Null);
    assert_eq!(plan["create_package_in_card"], serde_json::Value::Null);
    assert!(
        plan["card_283_must_cover"]
            .as_array()
            .expect("driver coverage")
            .is_empty()
    );
    assert_eq!(plan["supersede_cards"], serde_json::json!([283, 284, 285]));
}
