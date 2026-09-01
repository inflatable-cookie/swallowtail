use super::support::{
    FROZEN_0_152_0_APP_SERVER_HELP_SHA256, FROZEN_0_152_0_EXEC_HELP_SHA256,
    FROZEN_0_152_0_PROTOCOL, PROTOCOL, assert_exact_string_set, assert_sha256, json, strings,
};

#[test]
fn upstream_published_schema_digests_equal_the_frozen_0_152_0_corpus() {
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_0_152_0_PROTOCOL);
    assert_sha256(
        &protocol["schema"]["v2_bundle_sha256"],
        frozen["schema"]["v2_bundle_sha256"]
            .as_str()
            .expect("digest"),
    );
    assert_sha256(
        &protocol["schema"]["v2_bundle_sha256"],
        "08fa1b1072c5d8a889f00fdd96d1c853084e288d89d246552c1c47c23142adbb",
    );
    assert_sha256(
        &protocol["schema"]["experimental_bundle_sha256"],
        frozen["schema"]["experimental_bundle_sha256"]
            .as_str()
            .expect("digest"),
    );
    assert_sha256(
        &protocol["schema"]["experimental_bundle_sha256"],
        "d8faa38d5f00aa7ddfe635a2d374ee5f871ffd217d4d175c72fbe7f009f4f669",
    );
    assert_sha256(
        &protocol["schema"]["model_list_params_sha256"],
        frozen["schema"]["model_list_params_sha256"]
            .as_str()
            .expect("digest"),
    );
    assert_sha256(
        &protocol["schema"]["model_list_params_sha256"],
        "de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a",
    );
    assert_sha256(
        &protocol["schema"]["thread_resume_params_sha256"],
        frozen["schema"]["thread_resume_params_sha256"]
            .as_str()
            .expect("digest"),
    );
    assert_sha256(
        &protocol["schema"]["thread_resume_params_sha256"],
        "8ac68582a81d60940b10b330be8546123f56bfe246b56f8a4f121da00f347cf2",
    );
    assert_sha256(
        &protocol["schema"]["turn_start_params_sha256"],
        "a3835e8c1e942e4b358e1a670939b89918b16c4d13105a579899892b7ade6dea",
    );
}

#[test]
fn selected_flags_and_methods_match_the_frozen_0_152_0_corpus() {
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_0_152_0_PROTOCOL);
    assert_eq!(
        protocol["exec_selected_flags_present"],
        frozen["exec_selected_flags_present"]
    );
    assert_exact_string_set(
        &protocol["app_server_selected_flags_present"],
        &["app-server", "--listen", "stdio://"],
    );
    assert_eq!(
        protocol["app_server_selected_flags_present"],
        frozen["app_server_selected_flags_present"]
    );
    assert_eq!(
        protocol["schema"]["methods_present"],
        frozen["schema"]["methods_present"]
    );
    assert_eq!(
        protocol["schema"]["thread_resume_required"],
        frozen["schema"]["thread_resume_required"]
    );
    assert_eq!(
        protocol["schema"]["thread_resume_properties"],
        frozen["schema"]["thread_resume_properties"]
    );
    let methods = strings(&protocol["schema"]["methods_present"]);
    assert_eq!(methods.len(), 15);
    let properties = strings(&protocol["schema"]["thread_resume_properties"]);
    assert!(properties.contains(&"excludeTurns"));
    assert!(properties.contains(&"threadId"));
    assert_eq!(
        protocol["schema"]["thread_resume_exclude_turns"],
        "already-selected-mapped"
    );
}

#[test]
fn frozen_help_digests_carry_over_with_byte_identical_sources() {
    let protocol = json(PROTOCOL);
    assert_sha256(
        &protocol["frozen_0_152_0_corpus"]["exec_help_sha256"],
        FROZEN_0_152_0_EXEC_HELP_SHA256,
    );
    assert_sha256(
        &protocol["frozen_0_152_0_corpus"]["app_server_help_sha256"],
        FROZEN_0_152_0_APP_SERVER_HELP_SHA256,
    );
    assert_eq!(
        json(FROZEN_0_152_0_PROTOCOL)["exec_help_sha256"],
        serde_json::json!(FROZEN_0_152_0_EXEC_HELP_SHA256)
    );
    assert_eq!(
        protocol["decoder_corpus"],
        "existing-codex-jsonl-and-app-server-v2"
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_session"], false);
    assert_eq!(protocol["host_install_changed"], false);
}

#[test]
fn unmapped_deltas_stay_bounded_with_the_new_guardian_field() {
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_0_152_0_PROTOCOL);
    let unused = strings(&protocol["unused_deltas"]);
    let frozen_unused = strings(&frozen["unused_deltas"]);
    assert!(
        unused.len() > frozen_unused.len(),
        "0.152.1 records the frozen set plus one new entry"
    );
    let new_entries: Vec<&str> = unused
        .iter()
        .copied()
        .filter(|entry| !frozen_unused.contains(entry))
        .collect();
    assert_eq!(
        new_entries,
        vec!["Guardian AutoReviewMessages optional node_repl_policy"]
    );
    let source = &protocol["github_source_delta"];
    assert_exact_string_set(&source["added"], &[]);
    assert_exact_string_set(&source["removed"], &[]);
    assert_exact_string_set(&source["mapped_feeding_changed_files"], &[]);
    let classifications = source["classifications"]
        .as_array()
        .expect("classifications");
    assert_eq!(classifications.len(), 12);
    assert_eq!(
        protocol["schema"]["authority"],
        "upstream-published checked-in schema files"
    );
    let byte_identical = &source["byte_identical_mapped_sources"];
    assert_eq!(
        byte_identical["codex-rs/app-server-protocol/schema/json/ClientRequest.json"],
        "e92a06a1c476d62e6dc814c39d8ed3bcdfe3b865"
    );
    assert_eq!(
        byte_identical["codex-rs/exec/src/cli.rs"],
        "7e2f35e2af406bbe1debd8253ebeb1e41b320d9c"
    );
    assert_eq!(
        byte_identical["codex-rs/app-server/src/main.rs"],
        "4d5ab3f122bf836faf8729d39e946da0065ec466"
    );
    assert_eq!(
        byte_identical["codex-rs/cli/src/main.rs"],
        "0bfea04498c6c3ca11e64b859d1ae50d67caf2f4"
    );
}
