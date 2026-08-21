use semver::Version;
use std::collections::{BTreeMap, BTreeSet};

const EXECUTION: &str = include_str!("fixtures/opencode-v1.14.48-v1.18.10/compatibility.json");
const IMPORT: &str = include_str!("fixtures/opencode-v1.14.48-v1.18.10/session-import.json");
const SUCCESS: &str =
    include_str!("fixtures/opencode-v1.14.48-v1.18.10/session-import-success.json");
const FAILURES: &str =
    include_str!("fixtures/opencode-v1.14.48-v1.18.10/session-import-failures.json");
const RECONCILIATION: &str =
    include_str!("fixtures/opencode-v1.14.48-v1.18.10/session-reconciliation.json");

fn version(value: &serde_json::Value, field: &str) -> Version {
    Version::parse(value[field].as_str().expect("version field is text"))
        .expect("frozen version is semantic")
}

#[test]
fn every_qualified_release_maps_to_one_complete_import_surface() {
    let execution: serde_json::Value = serde_json::from_str(EXECUTION).expect("execution corpus");
    let import: serde_json::Value = serde_json::from_str(IMPORT).expect("import corpus");
    let releases = execution["releases"].as_array().expect("release records");
    let segments = import["segments"].as_array().expect("import segments");
    assert_eq!(releases.len(), 61);
    assert_eq!(segments.len(), 12);
    assert_eq!(import["baseline"], execution["baseline"]);
    assert_eq!(import["latest_qualified"], execution["latest_qualified"]);
    for release in releases {
        let release_version = version(release, "version");
        assert_eq!(
            segments
                .iter()
                .filter(|segment| {
                    version(segment, "minimum") <= release_version
                        && release_version <= version(segment, "maximum")
                })
                .count(),
            1,
            "{release_version} maps exactly once"
        );
    }
}

#[test]
fn exact_five_operation_closure_has_seven_revisions() {
    let import: serde_json::Value = serde_json::from_str(IMPORT).expect("import corpus");
    let routes = import["selected_routes"].as_array().expect("routes");
    assert_eq!(
        routes
            .iter()
            .map(|route| route["operation_id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "session.list",
            "session.status",
            "session.get",
            "session.messages",
            "session.prompt_async"
        ]
    );
    let revisions: BTreeMap<_, _> = import["surface_revisions"]
        .as_array()
        .expect("surface revisions")
        .iter()
        .map(|revision| {
            let digest = revision["sha256"].as_str().expect("digest");
            assert_eq!(digest.len(), 64);
            (
                revision["id"].as_str().expect("revision id"),
                revision["reference_count"]
                    .as_u64()
                    .expect("reference count"),
            )
        })
        .collect();
    assert_eq!(revisions.len(), 7);
    for segment in import["segments"].as_array().expect("segments") {
        assert!(revisions.contains_key(segment["surface"].as_str().unwrap()));
    }
}

#[test]
fn directory_pagination_status_and_child_policy_are_explicit() {
    let import: serde_json::Value = serde_json::from_str(IMPORT).expect("import corpus");
    let success: serde_json::Value = serde_json::from_str(SUCCESS).expect("success fixture");
    assert_eq!(
        import["binding"]["selected_parameters"],
        serde_json::json!(["directory", "start", "limit"])
    );
    let starts = success["pages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|page| page["start"].as_u64().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(starts, [0, 2, 4]);
    assert!(
        success["pages"][2]["sessions"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        import["projection"]["child_session"],
        "visible_unavailable_provider_reported"
    );
    assert_eq!(import["projection"]["busy_or_retry"], "active_unavailable");
    let candidates = success["expected_candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| candidate["id"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        candidates,
        BTreeSet::from(["ses_busy", "ses_child", "ses_root"])
    );
    assert_eq!(success["excluded"], serde_json::json!(["ses_foreign"]));
}

#[test]
fn history_and_failure_fixtures_remain_bounded_and_fail_closed() {
    let import: serde_json::Value = serde_json::from_str(IMPORT).expect("import corpus");
    let success: serde_json::Value = serde_json::from_str(SUCCESS).expect("success fixture");
    let failures: serde_json::Value = serde_json::from_str(FAILURES).expect("failure fixture");
    assert_eq!(
        success["expected_history_ids"],
        serde_json::json!(["message-1", "message-2", "message-3", "message-4"])
    );
    for key in ["maximum_pages", "maximum_items", "maximum_response_bytes"] {
        assert!(
            import["history"][key]
                .as_u64()
                .is_some_and(|bound| bound > 0)
        );
    }
    for group in ["malformed", "stale", "unsupported"] {
        assert!(!failures[group].as_array().unwrap().is_empty());
    }
    for stale in failures["stale"].as_array().unwrap() {
        assert_eq!(stale["result"], "no_binding");
    }
    for fixture in [IMPORT, SUCCESS, FAILURES] {
        assert!(fixture.len() < 128 * 1024);
        for forbidden in [
            "OPENCODE_SERVER_PASSWORD",
            "Authorization: Basic",
            "/Users/",
        ] {
            assert!(!fixture.contains(forbidden), "fixture leaked {forbidden}");
        }
    }
}

#[test]
fn reconciliation_reuses_only_the_read_only_qualified_surface() {
    let reconciliation: serde_json::Value =
        serde_json::from_str(RECONCILIATION).expect("reconciliation corpus");
    assert_eq!(reconciliation["qualified_release_count"], 61);
    assert_eq!(reconciliation["surface_segment_count"], 12);
    assert_eq!(
        reconciliation["operations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|operation| operation["method"].as_str().unwrap())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["GET"])
    );
    assert_eq!(reconciliation["attribution"]["kind"], "provider-session");
    assert_eq!(reconciliation["attribution"]["terminal_states"], false);
    for mutation in reconciliation["forbidden_mutations"].as_array().unwrap() {
        assert!(!mutation.as_str().unwrap().trim().is_empty());
    }
}
