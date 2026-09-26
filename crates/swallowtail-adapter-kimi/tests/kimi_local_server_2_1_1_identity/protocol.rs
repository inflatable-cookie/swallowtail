use super::support::{PROTOCOL, json};

#[test]
fn owned_argv_is_still_loopback_web() {
    let executing = &json(PROTOCOL)["executing_path"];
    assert_eq!(
        executing["owned_argv"],
        serde_json::json!(["web", "--no-open", "--host", "127.0.0.1"])
    );
    assert_eq!(executing["process_request_sets_cwd"], false);
    assert_eq!(executing["never_passes_remote_control"], true);
}

#[test]
fn selected_wire_blobs_hold_then_new_hops_classify() {
    let source = &json(PROTOCOL)["selected_local_server_source"];
    assert_eq!(
        source["auth_ts_unchanged_0_43_0_through_2_1_1"],
        "0bb9ea42f8437c6a45deafe8cb84529d8005525c"
    );
    assert_eq!(
        source["rest_session_ts_unchanged_0_43_0_through_2_1_1"],
        "41d037b82aa949eed0ca97e35c3a29c8281b9531"
    );
    assert_eq!(
        source["rest_model_catalog_ts"]["0.43.0..=0.43.1_and_2.0.0"],
        "1931153d18587ffda30fa6799e21e2ece56f8ccc"
    );
    assert_eq!(
        source["rest_model_catalog_ts"]["2.0.1..=2.1.1"],
        "42c7a93e4b37bb6271c8659696c202761cba7615"
    );
    assert_eq!(
        source["web_run_ts"]["0.43.0..=2.0.2"],
        "b10a666d9cd72e4adf92a9707f1c9135bc3aa670"
    );
    assert_eq!(
        source["web_run_ts"]["2.1.0..=2.1.1"],
        "e1d28ee07126fbba55a0fcb0815f7e4d44e06aac"
    );
    assert_eq!(
        source["prompts_ts"]["disabled_tools_applied_after_setModel"],
        true
    );
}

#[test]
fn every_new_hop_stays_stopped() {
    let verdict = &json(PROTOCOL)["per_hop_verdict"];
    assert!(
        verdict["0.43.1_to_2.0.0"]
            .as_str()
            .expect("verdict is text")
            .starts_with("major-line reset")
    );
    for hop in [
        "0.43.0_to_0.43.1",
        "2.0.0_to_2.0.1",
        "2.0.1_to_2.0.2",
        "2.0.2_to_2.1.0",
        "2.1.0_to_2.1.1",
    ] {
        assert!(
            verdict[hop]
                .as_str()
                .expect("verdict is text")
                .starts_with("still stopped:"),
            "{hop} must stay stopped"
        );
    }
    assert_eq!(
        json(PROTOCOL)["containment_trace"]["restored_containment_at_any_later_point"],
        false
    );
}

#[test]
fn decoder_corpus_stays_on_the_existing_local_server_specimens() {
    assert_eq!(
        json(PROTOCOL)["decoder_corpus"],
        "kimi-local-server-0.28.1-0.29.0"
    );
}
