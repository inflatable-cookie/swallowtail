use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const LIFECYCLE_RELEASES: &str =
    include_str!("fixtures/compatibility/app-server-lifecycle-releases.json");

#[test]
fn lifecycle_corpus_freezes_every_method_and_behavior_boundary() {
    let corpus = json(LIFECYCLE_RELEASES);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["facade"], "codex-app-server-v2");
    assert_eq!(corpus["claim"], "codex.app-server.lifecycle-window-1");
    assert_eq!(corpus["qualified_range"], "0.80.0..=0.149.0");

    let boundaries = &corpus["method_boundaries"];
    assert_eq!(boundaries["thread/archive"]["present"], "0.80.0");
    assert!(boundaries["thread/archive"]["prior"].is_null());
    assert_boundary(boundaries, "thread/unarchive", "0.91.0", "0.92.0");
    assert_boundary(
        boundaries,
        "thread/archived-notification",
        "0.103.0",
        "0.104.0",
    );
    assert_boundary(
        boundaries,
        "thread/unarchived-notification",
        "0.103.0",
        "0.104.0",
    );
    assert_boundary(
        boundaries,
        "best-effort-descendant-archive",
        "0.122.0",
        "0.123.0",
    );
    assert_boundary(boundaries, "thread/delete", "0.139.0", "0.140.0");
}

#[test]
fn lifecycle_segments_keep_capabilities_independent() {
    let corpus = json(LIFECYCLE_RELEASES);
    let segments = corpus["segments"]
        .as_array()
        .expect("segments are an array");
    assert_eq!(segments.len(), 7);

    assert_segment(&segments[0], "0.80.0..=0.81.0", &["archive"], "none");
    assert_segment(&segments[1], "0.84.0..=0.91.0", &["archive"], "none");
    assert_segment(
        &segments[2],
        "0.92.0..=0.103.0",
        &["archive", "restore"],
        "none",
    );
    assert_segment(
        &segments[3],
        "0.104.0..=0.107.0",
        &["archive", "restore"],
        "none",
    );
    assert_segment(
        &segments[4],
        "0.110.0..=0.122.0",
        &["archive", "restore"],
        "none",
    );
    assert_segment(
        &segments[5],
        "0.123.0..=0.139.0",
        &["archive", "restore"],
        "best-effort",
    );
    assert_segment(
        &segments[6],
        "0.140.0..=0.149.0",
        &["archive", "delete", "restore"],
        "best-effort",
    );

    assert_eq!(strings(&segments[2]["notifications"]), BTreeSet::new());
    assert_eq!(
        strings(&segments[3]["notifications"]),
        BTreeSet::from(["thread/archived", "thread/unarchived"])
    );
    assert_eq!(
        strings(&segments[6]["notifications"]),
        BTreeSet::from(["thread/archived", "thread/deleted", "thread/unarchived"])
    );
}

#[test]
fn wire_contract_does_not_overclaim_idempotency_or_descendant_archive() {
    let corpus = json(LIFECYCLE_RELEASES);
    let wire = &corpus["wire_contract"];

    assert_eq!(wire["archive"]["response"], serde_json::json!({}));
    assert_eq!(wire["archive"]["guaranteed_scope"], "target-only");
    assert_eq!(wire["archive"]["already_archived_target"], "error");
    assert_eq!(wire["archive"]["missing_target"], "error");

    assert_eq!(wire["restore"]["guaranteed_scope"], "target-only");
    assert_eq!(wire["restore"]["unarchived_target"], "error");
    assert_eq!(wire["restore"]["descendants"], "not-restored");

    assert_eq!(wire["delete"]["strength"], "provider-hard-deleted");
    assert_eq!(
        wire["delete"]["guaranteed_scope"],
        "provider-defined-descendants"
    );
    assert_eq!(wire["delete"]["order"], "descendants-before-root");
    assert_eq!(
        wire["delete"]["missing_rollout_after_validated_target"],
        "tolerated"
    );
    assert_eq!(wire["delete"]["unknown_target"], "error");
    assert_eq!(wire["delete"]["repeated_fully_deleted_target"], "error");
    assert_eq!(wire["cancellation"], "before-dispatch-only");
}

#[test]
fn release_evidence_covers_existing_and_lifecycle_milestones() {
    let corpus = json(LIFECYCLE_RELEASES);
    let releases = corpus["releases"]
        .as_array()
        .expect("releases are an array");
    let expected = [
        "0.80.0", "0.81.0", "0.84.0", "0.91.0", "0.92.0", "0.94.0", "0.99.0", "0.100.0", "0.103.0",
        "0.104.0", "0.107.0", "0.110.0", "0.122.0", "0.123.0", "0.130.0", "0.131.0", "0.139.0",
        "0.140.0", "0.144.6", "0.145.0", "0.146.0", "0.147.0", "0.148.0", "0.149.0",
    ];
    assert_eq!(
        releases
            .iter()
            .map(|release| release["version"].as_str().expect("version is text"))
            .collect::<Vec<_>>(),
        expected
    );

    for release in releases {
        assert_eq!(
            release["tag_commit"]
                .as_str()
                .expect("tag commit is text")
                .len(),
            40
        );
        assert!(
            release["published_at"]
                .as_str()
                .expect("publication date is text")
                .ends_with('Z')
        );
        assert!(matches!(
            release["schema_authority"].as_str(),
            Some("source-generated" | "upstream-published")
        ));
        let hashes = release["aggregate_schema_sha256"]
            .as_array()
            .expect("schema hashes are an array");
        assert_eq!(hashes.len(), 4);
        for hash in hashes {
            let hash = hash.as_str().expect("schema hash is text");
            assert_eq!(hash.len(), 64);
            assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
    }

    let by_version: BTreeMap<_, _> = releases
        .iter()
        .map(|release| {
            (
                release["version"].as_str().expect("version is text"),
                release,
            )
        })
        .collect();
    for version in ["0.80.0", "0.81.0", "0.84.0", "0.91.0", "0.92.0"] {
        assert_eq!(by_version[version]["schema_authority"], "source-generated");
    }
    assert_eq!(
        corpus["current_main_observation"]["projected_into_qualified_range"],
        false
    );
}

#[test]
fn lifecycle_exclusions_preserve_existing_app_server_window() {
    let corpus = json(LIFECYCLE_RELEASES);
    let exclusions: BTreeMap<_, _> = corpus["exclusions"]
        .as_array()
        .expect("exclusions are an array")
        .iter()
        .map(|entry| {
            (
                entry["range"].as_str().expect("range is text"),
                entry["reason"].as_str().expect("reason is text"),
            )
        })
        .collect();
    assert_eq!(
        exclusions["0.82.0..=0.83.0"],
        "not-qualified-by-existing-app-server-claim"
    );
    assert_eq!(
        exclusions["0.108.0..=0.109.0"],
        "source-tags-without-published-npm-releases"
    );
    assert_eq!(exclusions["0.146.0-alpha.4"], "prerelease");
    assert_eq!(corpus["unverified_newer"]["example"], "0.149.1");
    assert_eq!(
        corpus["unverified_newer"]["execution"],
        "permitted-with-explicit-unverified-status"
    );
    assert_eq!(corpus["unverified_newer"]["guaranteed"], false);
}

fn assert_boundary(boundaries: &Value, method: &str, absent: &str, present: &str) {
    assert_eq!(boundaries[method]["absent"], absent);
    assert_eq!(boundaries[method]["present"], present);
}

fn assert_segment(segment: &Value, range: &str, capabilities: &[&str], descendants: &str) {
    assert_eq!(segment["range"], range);
    assert_eq!(
        strings(&segment["capabilities"]),
        capabilities.iter().copied().collect()
    );
    assert_eq!(segment["archive_descendants"], descendants);
}

fn strings(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|entry| entry.as_str().expect("entry is text"))
        .collect()
}

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("fixture is valid JSON")
}
