mod legacy_corpus_support;

use legacy_corpus_support::{
    APP_SERVER_RELEASES, assert_release_evidence, assert_sha256, json, release_versions,
    releases_by_version, segments_by_id, string_set, strings,
};
use serde_json::Value;
use std::collections::BTreeSet;

#[test]
fn legacy_app_server_corpus_proves_v2_at_the_first_checkpoint() {
    let corpus = json(APP_SERVER_RELEASES);
    assert_eq!(corpus["facade"], "codex-app-server-v2");
    assert_eq!(corpus["profile"], "stable-read-only");
    assert_eq!(
        strings(&corpus["selected_methods"]),
        [
            "initialize",
            "model/list",
            "thread/start",
            "thread/resume",
            "turn/start",
            "turn/interrupt"
        ]
    );
    assert_eq!(
        strings(&corpus["selected_notifications"]),
        [
            "error",
            "thread/started",
            "turn/started",
            "turn/completed",
            "item/completed",
            "item/agentMessage/delta"
        ]
    );
    assert_eq!(
        string_set(&corpus["excluded_capabilities"]),
        BTreeSet::from([
            "dynamic-tools",
            "provider-requests",
            "runtime-workspace-roots",
            "workspace-write"
        ])
    );

    let first = &corpus["releases"][0];
    assert_eq!(first["version"], "0.80.0");
    assert_sha256(&first["protocol_common_source_sha256"]);
    assert_sha256(&first["protocol_v2_source_sha256"]);
    assert_eq!(first["schema"]["authority"], "source-generated");
}

#[test]
fn legacy_app_server_keeps_invocation_and_schema_authority_exact() {
    let corpus = json(APP_SERVER_RELEASES);
    assert_eq!(
        release_versions(&corpus),
        [
            "0.80.0", "0.81.0", "0.84.0", "0.94.0", "0.99.0", "0.100.0", "0.107.0", "0.110.0"
        ]
    );
    let releases = releases_by_version(&corpus);
    let segments = segments_by_id(&corpus);

    for version in ["0.80.0", "0.81.0"] {
        assert_app_release(
            releases[version],
            "legacy-default-stdio-first",
            "source-generated",
        );
    }
    assert_app_release(
        releases["0.84.0"],
        "legacy-default-stdio-second",
        "source-generated",
    );
    for version in ["0.94.0", "0.99.0"] {
        assert_app_release(
            releases[version],
            "legacy-default-stdio-second",
            "upstream-published",
        );
    }
    for version in ["0.100.0", "0.107.0"] {
        assert_app_release(
            releases[version],
            "legacy-explicit-stdio",
            "upstream-published",
        );
    }
    assert_app_release(
        releases["0.110.0"],
        "current-explicit-stdio-boundary",
        "upstream-published",
    );

    for id in ["legacy-default-stdio-first", "legacy-default-stdio-second"] {
        assert_eq!(strings(&segments[id]["invocation"]), ["app-server"]);
    }
    for id in ["legacy-explicit-stdio", "current-explicit-stdio-boundary"] {
        assert_eq!(
            strings(&segments[id]["invocation"]),
            ["app-server", "--listen", "stdio://"]
        );
    }
}

fn assert_app_release(release: &Value, segment: &str, authority: &str) {
    assert_eq!(release["segment"], segment);
    assert_release_evidence(release);
    for key in [
        "protocol_common_source_sha256",
        "protocol_v2_source_sha256",
        "cli_main_source_sha256",
        "app_server_main_source_sha256",
    ] {
        assert_sha256(&release[key]);
    }
    let schema = &release["schema"];
    assert_eq!(schema["authority"], authority);
    assert_eq!(schema["source_commit"], release["tag_commit"]);
    let artifacts = schema["aggregate_sha256"]
        .as_object()
        .expect("aggregate schema evidence is an object");
    assert_eq!(artifacts.len(), 4);
    for digest in artifacts.values() {
        assert_sha256(digest);
    }
    match authority {
        "source-generated" => {
            assert_eq!(
                schema["generation_command"],
                "cargo run --manifest-path codex-rs/Cargo.toml -p codex-app-server-protocol --bin export -- --out <output-dir>"
            );
            assert!(schema.get("source_path").is_none());
        }
        "upstream-published" => {
            assert_eq!(
                schema["source_path"],
                "codex-rs/app-server-protocol/schema/json"
            );
            assert!(schema.get("generation_command").is_none());
        }
        other => panic!("unexpected schema authority {other}"),
    }
}
