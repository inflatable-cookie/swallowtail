use super::support::{PROTOCOL, json, strings};

#[test]
fn executing_bundle_tree_ledger_is_exact_and_unmapped() {
    let protocol = json(PROTOCOL);
    let packaging = &protocol["packaging_deltas"];
    assert_eq!(
        packaging["classification"],
        "unmapped; twelve provider-transport chunks plus a main-application-chunk rehash whose only RPC dispatch delta is additive clear_queue"
    );
    assert_eq!(packaging["executing_bundle_entry"], "dist/bundle/index.js");
    assert_eq!(packaging["index_js_digest_changed"], true);
    assert_eq!(packaging["cli_js_digest_changed"], true);
    assert_eq!(packaging["rpc_entry_js_digest_changed"], true);
    let added = strings(&packaging["added_chunks"]);
    let removed = strings(&packaging["removed_chunks"]);
    assert_eq!(added.len(), 7);
    assert_eq!(removed.len(), 7);
    assert_eq!(
        added,
        [
            "chunk-2KVJKXS2",
            "chunk-OMWWHBTG",
            "google-generative-ai-YMUPJBKR",
            "google-vertex-MPWMV4OF",
            "https-proxy-agent-2VXB7436",
            "mistral-conversations-Q3AWZJAZ",
            "openai-completions-ERMU2SS7",
        ]
    );
    assert_eq!(
        removed,
        [
            "chunk-E5KXRMZK",
            "chunk-GPPBJGBU",
            "dist-RDWOYWHR",
            "google-generative-ai-XDKMGBCJ",
            "google-vertex-D5FGEO3Y",
            "mistral-conversations-YK73UAOZ",
            "openai-completions-JD4WAC3R",
        ]
    );
    for chunk in &added {
        assert!(
            !removed.contains(chunk),
            "added chunk {chunk} must not remain in the removed ledger"
        );
    }
    let main = &packaging["main_application_chunks"];
    let transport = &packaging["provider_transport_chunks"];
    assert_eq!(strings(&main["added"]), ["chunk-OMWWHBTG"]);
    assert_eq!(strings(&main["removed"]), ["chunk-E5KXRMZK"]);
    assert_eq!(main["rpc_dispatch_command_count_from"], 32);
    assert_eq!(main["rpc_dispatch_command_count_to"], 33);
    assert_eq!(
        strings(&main["rpc_dispatch_added_commands"]),
        ["clear_queue"]
    );
    assert_eq!(main["rpc_dispatch_removed_commands"], serde_json::json!([]));
    assert_eq!(
        main["rpc_dispatch_delta_is_only_additive_clear_queue"],
        true
    );
    let transport_added = strings(&transport["added"]);
    let transport_removed = strings(&transport["removed"]);
    assert_eq!(transport_added.len(), 6);
    assert_eq!(transport_removed.len(), 6);
    assert_eq!(
        transport_added,
        [
            "chunk-2KVJKXS2",
            "google-generative-ai-YMUPJBKR",
            "google-vertex-MPWMV4OF",
            "https-proxy-agent-2VXB7436",
            "mistral-conversations-Q3AWZJAZ",
            "openai-completions-ERMU2SS7",
        ]
    );
    assert_eq!(
        transport_removed,
        [
            "chunk-GPPBJGBU",
            "dist-RDWOYWHR",
            "google-generative-ai-XDKMGBCJ",
            "google-vertex-D5FGEO3Y",
            "mistral-conversations-YK73UAOZ",
            "openai-completions-JD4WAC3R",
        ]
    );
    let mut categorized_added = transport_added.clone();
    categorized_added.extend(strings(&main["added"]));
    categorized_added.sort_unstable();
    let mut added_sorted = added.clone();
    added_sorted.sort_unstable();
    assert_eq!(categorized_added, added_sorted);
    let mut categorized_removed = transport_removed.clone();
    categorized_removed.extend(strings(&main["removed"]));
    categorized_removed.sort_unstable();
    let mut removed_sorted = removed.clone();
    removed_sorted.sort_unstable();
    assert_eq!(categorized_removed, removed_sorted);
}

#[test]
fn changelog_and_chunk_ledger_lengths_are_load_bearing() {
    let protocol = json(PROTOCOL);
    assert_eq!(strings(&protocol["unmapped_changelog"]).len(), 8);
    assert_eq!(
        strings(&protocol["packaging_deltas"]["added_chunks"]).len(),
        7
    );
    assert_eq!(
        strings(&protocol["packaging_deltas"]["removed_chunks"]).len(),
        7
    );
    assert_eq!(
        strings(&protocol["packaging_deltas"]["main_application_chunks"]["added"]).len()
            + strings(&protocol["packaging_deltas"]["provider_transport_chunks"]["added"]).len(),
        7
    );
    assert_eq!(
        strings(&protocol["packaging_deltas"]["main_application_chunks"]["removed"]).len()
            + strings(&protocol["packaging_deltas"]["provider_transport_chunks"]["removed"]).len(),
        7
    );
    assert_eq!(
        protocol["unmapped_changelog_inert_reasons"]
            .as_object()
            .expect("inert reasons are an object")
            .len(),
        2
    );
}
