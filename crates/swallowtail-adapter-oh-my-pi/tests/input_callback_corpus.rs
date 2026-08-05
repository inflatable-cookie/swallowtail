use serde_json::{Value, json};

#[test]
fn exact_rpc_corpus_binds_one_bounded_inline_image() {
    let corpus: Value = serde_json::from_str(include_str!(
        "fixtures/oh-my-pi-rpc-17.2.9/input-callback-corpus.json"
    ))
    .expect("fixture parses");

    assert_eq!(corpus["version"], "17.2.9");
    assert_eq!(corpus["attachment"]["maximum_count"], 1);
    assert_eq!(corpus["attachment"]["maximum_bytes"], 1_048_576);
    assert_eq!(corpus["attachment"]["accepted_media"], json!(["image/png"]));

    let command = &corpus["attachment"]["command"];
    assert_eq!(command["id"], "req-image-1");
    assert_eq!(command["type"], "prompt");
    assert_eq!(command["images"][0]["type"], "image");
    assert_eq!(command["images"][0]["mimeType"], "image/png");
    assert!(command["images"][0]["data"].as_str().is_some());
    assert!(command.get("path").is_none());
    assert!(command.get("url").is_none());
}

#[test]
fn rpc_attachment_corpus_keeps_abort_cleanup_and_absence_exact() {
    let corpus: Value = serde_json::from_str(include_str!(
        "fixtures/oh-my-pi-rpc-17.2.9/input-callback-corpus.json"
    ))
    .expect("fixture parses");

    assert_eq!(corpus["cancellation"]["command"]["type"], "abort");
    assert_eq!(
        corpus["cleanup"],
        json!([
            "abandon_input",
            "abort_active_rpc",
            "close_stdio",
            "join_process",
            "release_attachment"
        ])
    );
    assert_eq!(corpus["consumer_tools"], "unsupported");
    assert_eq!(corpus["external_search"], "unsupported");
    assert!(
        corpus["rejections"]
            .as_array()
            .expect("rejections are an array")
            .contains(&json!("request_plan_mismatch"))
    );
}
