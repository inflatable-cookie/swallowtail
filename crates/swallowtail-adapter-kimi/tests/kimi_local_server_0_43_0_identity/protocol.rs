use super::support::{FROZEN_0_41_0_PROTOCOL, PROTOCOL, assert_sha1, json};

const UNCHANGED_APPROVAL: &str = "bb7e354bce8eb15bf4634f0785abf4c2a63d7d01";
const UNCHANGED_QUESTION: &str = "3584025ac68ccf3478e0fa216fee62a626368330";
const AUTH_0_38_0: &str = "9fedc57a6b98ea3b59a6b0eed959fb6bc1883092";
const AUTH_0_39_0: &str = "0bb9ea42f8437c6a45deafe8cb84529d8005525c";
const CATALOG_0_38_0: &str = "f3f7105b5b85903f4fe215b86614c08f5d7beb01";
const CATALOG_0_39_0: &str = "ad58d6be8d26b89da757ee6a0d83732a1d978d0b";
const WS_0_38_0: &str = "1245774552dd363ab166ed549fb58b6abe95da9d";
const WS_0_39_0: &str = "546984d23ab3340d396eadb1570b804fc6494333";
const SESSION_0_38_0: &str = "f1cc7d5132e813598b423b5366030e193fa450cd";
const SESSION_0_39_0: &str = "1abc32e6b9fadf365ae7250573895af4aadcaf65";
const TERMINALS_0_38_0: &str = "251ad90cac3d8334120f8bcadd53c4d5a6e8ef66";
const TERMINALS_0_42_0: &str = "3dcd07b0ca1aa97ad295efc2b3305fa33eb85072";
const CATALOG_0_42_0: &str = "1931153d18587ffda30fa6799e21e2ece56f8ccc";
const WS_0_42_0: &str = "f71b3700496a2377951c6025d339406c494c06e6";
const SESSION_0_42_0: &str = "41d037b82aa949eed0ca97e35c3a29c8281b9531";
const WEB_INDEX_0_42_0: &str = "30240210b239ebf2573090a82b697e2c4ff823a3";
const WEB_RUN_0_42_0: &str = "878ab3cfc60eb31f67dc0f8c4b7184971612f85f";
const WEB_RUN_0_43_0: &str = "b10a666d9cd72e4adf92a9707f1c9135bc3aa670";
const WEB_SHARED_0_39_0: &str = "91fede6c07bbdacea36243865beeaea5cdd8a5e0";
const BASH_0_38_0: &str = "4668f87694928c1cf557b5e7624c239ba5436625";
const BASH_0_39_0: &str = "41090010ce96a7c66586fda8dc17b3a19d62ceaf";
const RWV_0_38_0: &str = "06ccea0050af00057e45b5a128ca7a679878524a";
const RWV_0_39_0: &str = "03edbdba4b1587e91ed7e9df076d36e1cc79330a";
const RWV_0_40_0: &str = "01db1bbf8f51cc1b16178d3d4e07817d4b63d372";

#[test]
fn owned_argv_is_loopback_web_without_bypass_or_remote_control() {
    let executing = &json(PROTOCOL)["executing_path"];
    assert_eq!(
        executing["owned_argv"],
        serde_json::json!(["web", "--no-open", "--host", "127.0.0.1"])
    );
    assert_eq!(executing["process_request_sets_cwd"], false);
    assert_eq!(executing["never_passes_allow_remote_terminals"], true);
    assert_eq!(executing["never_passes_dangerous_bypass_auth"], true);
    assert_eq!(executing["never_passes_remote_control"], true);
}

#[test]
fn safe_prefix_wire_blobs_hold_then_new_hops_classify() {
    let source = &json(PROTOCOL)["selected_local_server_source"];
    assert_eq!(source["mapped_wire_schemas_comment_only_at_0_39_0"], true);
    assert_eq!(
        source["mapped_wire_blobs_byte_identical_0_39_0_through_0_41_0"],
        true
    );
    assert_eq!(
        source["heartbeat_ping_schema_unchanged_0_38_0_through_0_43_0"],
        true
    );

    assert_sha1(&source["auth_ts"]["0.38.0"], AUTH_0_38_0);
    assert_sha1(&source["auth_ts"]["0.39.0..=0.43.0"], AUTH_0_39_0);
    assert_eq!(source["auth_ts"]["0.38.0_to_0.39.0"], "comment-only");
    assert_ne!(AUTH_0_38_0, AUTH_0_39_0);

    assert_sha1(&source["rest_model_catalog_ts"]["0.38.0"], CATALOG_0_38_0);
    assert_sha1(
        &source["rest_model_catalog_ts"]["0.39.0..=0.41.0"],
        CATALOG_0_39_0,
    );
    assert_sha1(
        &source["rest_model_catalog_ts"]["0.42.0..=0.43.0"],
        CATALOG_0_42_0,
    );

    assert_sha1(&source["ws_control_ts"]["0.38.0"], WS_0_38_0);
    assert_sha1(&source["ws_control_ts"]["0.39.0..=0.41.0"], WS_0_39_0);
    assert_sha1(&source["ws_control_ts"]["0.42.0..=0.43.0"], WS_0_42_0);

    assert_sha1(&source["rest_session_ts"]["0.38.0"], SESSION_0_38_0);
    assert_sha1(
        &source["rest_session_ts"]["0.39.0..=0.41.0"],
        SESSION_0_39_0,
    );
    assert_sha1(
        &source["rest_session_ts"]["0.42.0..=0.43.0"],
        SESSION_0_42_0,
    );

    assert_sha1(
        &source["rest_approval_ts_unchanged_0_38_0_through_0_43_0"],
        UNCHANGED_APPROVAL,
    );
    assert_sha1(
        &source["rest_question_ts_unchanged_0_38_0_through_0_43_0"],
        UNCHANGED_QUESTION,
    );

    assert_sha1(&source["terminals_ts"]["0.38.0..=0.41.0"], TERMINALS_0_38_0);
    assert_sha1(&source["terminals_ts"]["0.42.0..=0.43.0"], TERMINALS_0_42_0);

    assert_sha1(&source["web_index_ts"]["0.42.0..=0.43.0"], WEB_INDEX_0_42_0);
    assert_sha1(&source["web_run_ts"]["0.42.0"], WEB_RUN_0_42_0);
    assert_sha1(&source["web_run_ts"]["0.43.0"], WEB_RUN_0_43_0);
    assert_ne!(WEB_RUN_0_42_0, WEB_RUN_0_43_0);
    assert_sha1(
        &source["web_shared_ts_unchanged_0_39_0_through_0_43_0"],
        WEB_SHARED_0_39_0,
    );
}

#[test]
fn bash_authority_blobs_pin_the_safe_prefix_and_the_uncontained_gap() {
    let authority = &json(PROTOCOL)["bash_cwd_authority"];
    assert_eq!(authority["first_published"], "0.40.0");
    assert_eq!(authority["holds_through"], "0.43.0");
    assert_eq!(
        authority["bash_tool_call"],
        "view.resolve(args.cwd ?? view.workDir)"
    );
    assert_sha1(&authority["bash_tool_ts"]["0.38.0"], BASH_0_38_0);
    assert_sha1(&authority["bash_tool_ts"]["0.39.0..=0.43.0"], BASH_0_39_0);
    assert_sha1(
        &authority["runtime_workspace_view_ts"]["0.38.0"],
        RWV_0_38_0,
    );
    assert_sha1(
        &authority["runtime_workspace_view_ts"]["0.39.0..=0.39.1"],
        RWV_0_39_0,
    );
    assert_sha1(
        &authority["runtime_workspace_view_ts"]["0.40.0..=0.43.0"],
        RWV_0_40_0,
    );
    assert_ne!(RWV_0_39_0, RWV_0_40_0);
}

#[test]
fn every_hop_carries_a_verdict_with_no_restored_containment() {
    let verdict = &json(PROTOCOL)["per_hop_verdict"];
    assert_eq!(
        verdict["0.38.0_to_0.39.0"],
        "safe prefix: wire comment-only, Bash assertion preserved"
    );
    assert_eq!(
        verdict["0.39.0_to_0.39.1"],
        "safe prefix: every selected blob byte-identical"
    );
    assert!(
        verdict["0.39.1_to_0.40.0"]
            .as_str()
            .expect("verdict is text")
            .starts_with("stop:")
    );
    for hop in [
        "0.40.0_to_0.40.1",
        "0.40.1_to_0.41.0",
        "0.41.0_to_0.42.0",
        "0.42.0_to_0.43.0",
    ] {
        assert!(
            verdict[hop]
                .as_str()
                .expect("verdict is text")
                .starts_with("still stopped:")
        );
    }
    assert_eq!(
        json(PROTOCOL)["containment_trace"]["restored_containment_at_any_later_point"],
        false
    );
}

#[test]
fn remote_control_and_watch_fs_deltas_stay_unmapped() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        protocol["web_cli_deltas"]["remote_control_de_experimentalized_at_0_42_0"],
        true
    );
    assert_eq!(
        protocol["web_cli_deltas"]["swallowtail_never_passes_remote_control"],
        true
    );
    assert_eq!(protocol["web_cli_deltas"]["classification"], "unmapped");
    let unmapped = protocol["unmapped_deltas"]
        .as_array()
        .expect("unmapped deltas are an array");
    for required in [
        "watch_fs",
        "deleteSessionResponseSchema",
        "runtime_id",
        "localServerToken",
    ] {
        assert!(
            unmapped
                .iter()
                .any(|delta| delta.as_str().expect("delta is text").contains(required)),
            "missing unmapped delta {required}"
        );
    }
}

#[test]
fn decoder_corpus_stays_on_the_existing_local_server_specimens() {
    assert_eq!(
        json(PROTOCOL)["decoder_corpus"],
        "kimi-local-server-0.28.1-0.29.0"
    );
}

#[test]
fn the_0_41_0_ledger_values_carry_forward_unchanged() {
    let frozen_protocol = json(FROZEN_0_41_0_PROTOCOL);
    let frozen = &frozen_protocol["selected_local_server_source"];
    let live_protocol = json(PROTOCOL);
    let source = &live_protocol["selected_local_server_source"];
    assert_eq!(source["auth_ts"]["0.38.0"], frozen["auth_ts"]["0.38.0"]);
    assert_eq!(
        source["auth_ts"]["0.39.0..=0.43.0"],
        frozen["auth_ts"]["0.39.0..=0.41.0"]
    );
    assert_eq!(
        source["rest_approval_ts_unchanged_0_38_0_through_0_43_0"],
        frozen["rest_approval_ts_unchanged_0_38_0_through_0_41_0"]
    );
    assert_eq!(
        source["rest_question_ts_unchanged_0_38_0_through_0_43_0"],
        frozen["rest_question_ts_unchanged_0_38_0_through_0_41_0"]
    );
    assert_eq!(
        source["terminals_ts"]["0.38.0..=0.41.0"],
        frozen["terminals_ts_unchanged_0_38_0_through_0_41_0"]
    );
}
