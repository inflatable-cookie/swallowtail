use semver::Version;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_opencode::{opencode_http_claim, opencode_server_binding};
use swallowtail_core::InterfaceCompatibilityAssessment;

const EXECUTION_CORPUS: &str =
    include_str!("fixtures/opencode-v1.14.48-v1.18.10/compatibility.json");
const DELETION_CORPUS: &str = include_str!("fixtures/opencode-v1.14.48-v1.18.10/deletion.json");

fn version(value: &serde_json::Value, field: &str) -> Version {
    Version::parse(value[field].as_str().expect("version field is a string"))
        .expect("frozen version is semantic")
}

fn segment_for<'a>(
    segments: &'a [serde_json::Value],
    release: &serde_json::Value,
) -> &'a serde_json::Value {
    let release = version(release, "version");
    segments
        .iter()
        .find(|segment| {
            version(segment, "minimum") <= release && release <= version(segment, "maximum")
        })
        .expect("every frozen release belongs to a deletion segment")
}

#[test]
fn deletion_closure_covers_the_unchanged_qualified_release_set() {
    let execution: serde_json::Value =
        serde_json::from_str(EXECUTION_CORPUS).expect("execution corpus parses");
    let deletion: serde_json::Value =
        serde_json::from_str(DELETION_CORPUS).expect("deletion corpus parses");
    let execution_releases = execution["releases"]
        .as_array()
        .expect("execution releases");
    let deletion_releases = deletion["releases"].as_array().expect("deletion releases");

    assert_eq!(deletion["baseline"], execution["baseline"]);
    assert_eq!(deletion["latest_qualified"], execution["latest_qualified"]);
    assert_eq!(deletion_releases.len(), 61);
    assert_eq!(deletion_releases.len(), execution_releases.len());
    assert_eq!(
        execution["selected_routes"]
            .as_array()
            .expect("execution routes")
            .len(),
        6
    );
    assert_eq!(
        execution["surface_revisions"]
            .as_array()
            .expect("execution surfaces")
            .len(),
        19
    );

    for (deletion_release, execution_release) in deletion_releases.iter().zip(execution_releases) {
        for field in ["version", "commit", "published", "openapi_sha256"] {
            assert_eq!(deletion_release[field], execution_release[field]);
        }
        assert_eq!(
            deletion_release["execution_surface"],
            execution_release["surface"]
        );
        assert_eq!(
            deletion_release["commit"]
                .as_str()
                .expect("commit is a string")
                .len(),
            40
        );
        assert_eq!(
            deletion_release["openapi_sha256"]
                .as_str()
                .expect("OpenAPI digest is a string")
                .len(),
            64
        );
        assert_eq!(
            deletion_release["delete_surface_sha256"]
                .as_str()
                .expect("delete digest is a string")
                .len(),
            64
        );
    }
}

#[test]
fn deletion_revisions_change_only_with_the_recursive_schema_closure() {
    let deletion: serde_json::Value =
        serde_json::from_str(DELETION_CORPUS).expect("deletion corpus parses");
    let releases = deletion["releases"].as_array().expect("deletion releases");
    let segments = deletion["segments"].as_array().expect("deletion segments");
    let surfaces = deletion["surface_revisions"]
        .as_array()
        .expect("deletion surfaces");
    let known_surfaces: BTreeMap<_, _> = surfaces
        .iter()
        .map(|surface| {
            (
                surface["id"].as_str().expect("surface id"),
                surface["sha256"].as_str().expect("surface digest"),
            )
        })
        .collect();

    assert_eq!(known_surfaces.len(), 2);
    assert_eq!(segments.len(), 8);
    let mut observed = BTreeSet::new();
    for release in releases {
        let surface = release["delete_surface"].as_str().expect("delete surface");
        let digest = release["delete_surface_sha256"]
            .as_str()
            .expect("delete digest");
        assert_eq!(known_surfaces.get(surface), Some(&digest));
        assert_eq!(segment_for(segments, release)["surface"], surface);
        observed.insert((surface, digest));
    }
    assert_eq!(observed.len(), 2);
    assert_eq!(releases[9]["version"], "1.15.5");
    assert_eq!(releases[9]["delete_surface"], "delete-01");
    assert_eq!(releases[10]["version"], "1.15.6");
    assert_eq!(releases[10]["delete_surface"], "delete-02");
}

#[test]
fn deletion_segments_preserve_gaps_and_unverified_newer_posture() {
    let deletion: serde_json::Value =
        serde_json::from_str(DELETION_CORPUS).expect("deletion corpus parses");
    let segments = deletion["segments"].as_array().expect("deletion segments");
    for excluded in deletion["exclusions"].as_array().expect("exclusions") {
        let excluded = Version::parse(excluded.as_str().expect("excluded version"))
            .expect("excluded version is semantic");
        assert!(
            !excluded.pre.is_empty()
                || !segments.iter().any(|segment| {
                    version(segment, "minimum") <= excluded
                        && excluded <= version(segment, "maximum")
                })
        );
    }

    assert_eq!(deletion["unverified_newer_example"], "1.18.21");
    let newer = "1.18.29";
    let binding = opencode_server_binding(newer).expect("newer example is safe");
    assert!(matches!(
        opencode_http_claim().assess(binding.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn deletion_truth_keeps_provider_and_swallowtail_boundaries_explicit() {
    let deletion: serde_json::Value =
        serde_json::from_str(DELETION_CORPUS).expect("deletion corpus parses");
    let behavior = &deletion["wire_behavior"];
    assert_eq!(behavior["success"]["status"], 200);
    assert_eq!(behavior["success"]["body"], true);
    assert_eq!(
        behavior["success"]["deletion_strength"],
        "provider_data_deleted"
    );
    assert_eq!(
        behavior["success"]["descendant_scope"],
        "provider_defined_descendants"
    );
    assert_eq!(behavior["missing_target"]["status"], 404);
    assert_eq!(behavior["missing_target"]["already_deleted_success"], false);
    assert_eq!(behavior["active_target"]["provider_busy_guard"], false);
    assert_eq!(
        behavior["active_target"]["swallowtail_requires_inactive_target"],
        true
    );
    assert_eq!(behavior["authentication"]["unauthorized_status"], 401);
    assert_eq!(
        behavior["server_error"]["provider_truth"],
        "unconfirmed_after_dispatch"
    );
    assert_eq!(deletion["source_posture"]["hard_erasure_claim"], false);

    let revisions = deletion["runtime_revisions"]
        .as_array()
        .expect("runtime revisions");
    assert_eq!(revisions.len(), 2);
    assert!(
        revisions
            .iter()
            .all(|revision| revision["recursive_descendants"] == true)
    );
    assert!(
        revisions
            .iter()
            .all(|revision| revision["busy_guard"] == false)
    );
}
