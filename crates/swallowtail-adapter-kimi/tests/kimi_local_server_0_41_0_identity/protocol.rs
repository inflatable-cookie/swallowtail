use super::support::{PROTOCOL, assert_sha1, json, strings};

const UNCHANGED_APPROVAL: &str = "bb7e354bce8eb15bf4634f0785abf4c2a63d7d01";
const UNCHANGED_QUESTION: &str = "3584025ac68ccf3478e0fa216fee62a626368330";
const UNCHANGED_TERMINALS: &str = "251ad90cac3d8334120f8bcadd53c4d5a6e8ef66";
const AUTH_0_38_0: &str = "9fedc57a6b98ea3b59a6b0eed959fb6bc1883092";
const AUTH_0_39_0: &str = "0bb9ea42f8437c6a45deafe8cb84529d8005525c";
const CATALOG_0_38_0: &str = "f3f7105b5b85903f4fe215b86614c08f5d7beb01";
const CATALOG_0_39_0: &str = "ad58d6be8d26b89da757ee6a0d83732a1d978d0b";
const WS_0_38_0: &str = "1245774552dd363ab166ed549fb58b6abe95da9d";
const WS_0_39_0: &str = "546984d23ab3340d396eadb1570b804fc6494333";

#[test]
fn owned_argv_is_loopback_web_without_bypass_or_remote_control() {
    let executing = &json(PROTOCOL)["executing_path"];
    assert_eq!(
        strings(&executing["owned_argv"]),
        [
            "web",
            "--no-open",
            "--host",
            "127.0.0.1",
            "--port",
            "<port>",
            "--log-level",
            "info"
        ]
    );
    assert_eq!(executing["process_request_sets_cwd"], false);
    assert_eq!(executing["never_passes_allow_remote_terminals"], true);
    assert_eq!(executing["never_passes_dangerous_bypass_auth"], true);
    assert_eq!(executing["never_passes_remote_control"], true);
}

#[test]
fn mapped_protocol_blobs_are_comment_only_then_hold() {
    let source = &json(PROTOCOL)["selected_local_server_source"];
    assert_eq!(source["mapped_wire_schemas_comment_only_at_0_39_0"], true);
    assert_eq!(
        source["mapped_wire_blobs_byte_identical_0_39_0_through_0_41_0"],
        true
    );
    assert_eq!(source["heartbeat_ping_schema_unchanged"], true);
    assert_eq!(
        source["archive_restore_schema_comment_only_at_0_39_0"],
        true
    );

    assert_sha1(&source["auth_ts"]["0.38.0"], AUTH_0_38_0);
    assert_sha1(&source["auth_ts"]["0.39.0..=0.41.0"], AUTH_0_39_0);
    assert_eq!(source["auth_ts"]["0.38.0_to_0.39.0"], "comment-only");
    assert_ne!(AUTH_0_38_0, AUTH_0_39_0);

    assert_sha1(&source["rest_model_catalog_ts"]["0.38.0"], CATALOG_0_38_0);
    assert_sha1(
        &source["rest_model_catalog_ts"]["0.39.0..=0.41.0"],
        CATALOG_0_39_0,
    );
    assert_eq!(
        source["rest_model_catalog_ts"]["0.38.0_to_0.39.0"],
        "comment-only"
    );

    assert_sha1(&source["ws_control_ts"]["0.38.0"], WS_0_38_0);
    assert_sha1(&source["ws_control_ts"]["0.39.0..=0.41.0"], WS_0_39_0);
    assert_eq!(source["ws_control_ts"]["0.38.0_to_0.39.0"], "comment-only");

    assert_sha1(
        &source["rest_approval_ts_unchanged_0_38_0_through_0_41_0"],
        UNCHANGED_APPROVAL,
    );
    assert_sha1(
        &source["rest_question_ts_unchanged_0_38_0_through_0_41_0"],
        UNCHANGED_QUESTION,
    );
    assert_sha1(
        &source["terminals_ts_unchanged_0_38_0_through_0_41_0"],
        UNCHANGED_TERMINALS,
    );
}

#[test]
fn remote_terminal_flag_removal_stays_unmapped() {
    let web = &json(PROTOCOL)["web_cli_deltas"];
    assert_eq!(web["removed_at_0_39_0"], "--allow-remote-terminals");
    assert_eq!(web["pty_terminals_loopback_only_from_0_39_0"], true);
    assert_eq!(
        web["swallowtail_never_selected_allow_remote_terminals"],
        true
    );
    assert_eq!(web["swallowtail_owned_bind_is_loopback"], true);
    assert_eq!(web["classification"], "unmapped");
    assert_eq!(
        strings(&web["added_unmapped_remote_control"]),
        [
            "apps/kimi-code/src/cli/sub/web/remote-control.ts",
            "apps/kimi-code/src/cli/sub/web/remote-control-lock.ts"
        ]
    );
}

#[test]
fn decoder_corpus_stays_on_the_existing_local_server_specimens() {
    assert_eq!(
        json(PROTOCOL)["decoder_corpus"],
        "kimi-local-server-0.28.1-0.29.0"
    );
}
