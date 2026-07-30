use semver::Version;
use std::collections::BTreeMap;

const EXECUTION: &str = include_str!("fixtures/opencode-v1.14.48-v1.18.10/compatibility.json");
const CONTINUITY: &str =
    include_str!("fixtures/opencode-v1.14.48-v1.18.10/session-continuity.json");

fn version(value: &serde_json::Value, field: &str) -> Version {
    Version::parse(value[field].as_str().expect("version field is text"))
        .expect("frozen version is semantic")
}

#[test]
fn every_qualified_release_maps_to_one_continuity_segment() {
    let execution: serde_json::Value = serde_json::from_str(EXECUTION).expect("execution corpus");
    let continuity: serde_json::Value =
        serde_json::from_str(CONTINUITY).expect("continuity corpus");
    let releases = execution["releases"].as_array().expect("releases");
    let segments = continuity["segments"].as_array().expect("segments");
    assert_eq!(releases.len(), 51);
    assert_eq!(segments.len(), 12);
    assert_eq!(continuity["baseline"], execution["baseline"]);
    assert_eq!(
        continuity["latest_qualified"],
        execution["latest_qualified"]
    );

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
fn all_seven_closed_surfaces_are_referenced() {
    let continuity: serde_json::Value =
        serde_json::from_str(CONTINUITY).expect("continuity corpus");
    let surfaces: BTreeMap<_, _> = continuity["surface_revisions"]
        .as_array()
        .expect("surfaces")
        .iter()
        .map(|surface| {
            let digest = surface["sha256"].as_str().expect("digest");
            assert_eq!(digest.len(), 64);
            (surface["id"].as_str().expect("id"), digest)
        })
        .collect();
    assert_eq!(surfaces.len(), 7);
    for segment in continuity["segments"].as_array().expect("segments") {
        assert!(surfaces.contains_key(segment["surface"].as_str().expect("surface")));
    }
    assert_eq!(
        continuity["selected_routes"]
            .as_array()
            .expect("selected routes")
            .len(),
        3
    );
}

#[test]
fn load_reverses_pages_without_reversing_items() {
    let continuity: serde_json::Value =
        serde_json::from_str(CONTINUITY).expect("continuity corpus");
    let pages = continuity["representative_pages"]
        .as_array()
        .expect("representative pages");
    let replay: Vec<_> = pages
        .iter()
        .rev()
        .flat_map(|page| page["response_items"].as_array().expect("page items"))
        .map(|message| message["id"].as_str().expect("message id"))
        .collect();
    let expected: Vec<_> = continuity["expected_replay_ids"]
        .as_array()
        .expect("expected replay")
        .iter()
        .map(|id| id.as_str().expect("expected id"))
        .collect();
    assert_eq!(replay, expected);
    assert_eq!(continuity["load"]["page_wire_order"], "newest_page_first");
    assert_eq!(
        continuity["load"]["replay_order"],
        "reverse_page_sequence_preserve_item_sequence"
    );
}

#[test]
fn resume_and_cleanup_do_not_claim_replay_or_server_lifecycle() {
    let continuity: serde_json::Value =
        serde_json::from_str(CONTINUITY).expect("continuity corpus");
    assert_eq!(continuity["resume"]["message_list_called"], false);
    assert_eq!(continuity["resume"]["replay_phase"], false);
    assert_eq!(
        continuity["resume"]["continuation"],
        "session.prompt_async_exact_session"
    );
    assert_eq!(continuity["cleanup"]["server_stop"], false);
    assert_eq!(continuity["cleanup"]["session_delete"], false);
    assert_eq!(continuity["cleanup"]["detached_tasks_allowed"], false);
    assert_eq!(
        continuity["failures"]["foreign_message_session"],
        "fail_without_usable_handle"
    );
}

#[test]
fn opencode_continuity_corpus_is_bounded_and_safe() {
    let continuity: serde_json::Value =
        serde_json::from_str(CONTINUITY).expect("continuity corpus");
    for key in [
        "page_limit",
        "maximum_pages",
        "maximum_items",
        "maximum_response_bytes",
    ] {
        assert!(continuity["limits"][key].as_u64().is_some_and(|n| n > 0));
    }
    assert!(CONTINUITY.len() < 128 * 1024);
    for forbidden in [
        "/Users/",
        "OPENCODE_SERVER_PASSWORD",
        "Bearer ",
        "session-provider-private",
    ] {
        assert!(
            !CONTINUITY.contains(forbidden),
            "fixture leaked {forbidden}"
        );
    }
}
