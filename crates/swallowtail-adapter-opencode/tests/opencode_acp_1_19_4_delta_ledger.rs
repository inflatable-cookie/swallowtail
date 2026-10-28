use std::collections::BTreeSet;

use serde_json::{Map, Value};
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_LATEST_QUALIFIED_VERSION, opencode_acp_binding, opencode_acp_claim,
    opencode_http_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

const IDENTITY: &str = include_str!("fixtures/opencode-acp-1.19.4/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-acp-1.19.4/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-acp-1.19.4/protocol.json");

const VERSIONS: [&str; 6] = ["1.18.35", "1.19.0", "1.19.1", "1.19.2", "1.19.3", "1.19.4"];

type Hop = (
    &'static str,
    u64,
    u64,
    u64,
    u64,
    &'static str,
    &'static str,
    &'static str,
);

const HOPS: [Hop; 5] = [
    (
        "1.18.35_to_1.19.0",
        6,
        0,
        132,
        6496,
        "6e2952483d2a3ea4c0c42a35a49d97ec8cad5f8ddbac313e7cb5501ed3c24912",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "f8e4b4bcc49d049e9ca902922c6849fa14a06cc4f194d68865d94be76f16312d",
    ),
    (
        "1.19.0_to_1.19.1",
        0,
        2,
        66,
        6566,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "48ac85681e814d6834baf01b23e3a7468c20a28e32c371d04d0992aceb71ffaf",
        "dbda759d04dfe29ed66ce8a705bf0167d91e0d821e7666a4e7a9f534ad88a461",
    ),
    (
        "1.19.1_to_1.19.2",
        2,
        0,
        41,
        6591,
        "98ffaff23d851bdaf783266b51c93d3ecf2477202c93e21b033c9b653066b82e",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "1a62c90b489ccd1b2f5dcb11898572c8113656d8dc19912f74b993fcdf2ba542",
    ),
    (
        "1.19.2_to_1.19.3",
        0,
        0,
        36,
        6598,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "0bfa26392c13bbec41d0f8ae6eeeff75d855ec60b6744bc8cf77c39ef0844816",
    ),
    (
        "1.19.3_to_1.19.4",
        2,
        0,
        46,
        6588,
        "139e7cec0a30bd65dbdf806a803ba7462672317eae3522470c09411c21130023",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "4d2fd9cf80c08b925f2a0878a9b843f060f3f84740104e2e64985f132db584a0",
    ),
];

#[test]
fn acp_identity_keeps_a_separate_v2_extension() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let claim = opencode_acp_claim();
    let http = opencode_http_claim();
    let window = &identity["qualification_window"];
    assert_eq!(identity["family"], "opencode.acp");
    assert_eq!(identity["research"], 453);
    assert_eq!(
        identity["official_channels"]["npm_dist_tag_latest"],
        "1.19.4"
    );
    assert_eq!(
        identity["official_channels"]["github_latest_stable_tag"],
        "v1.19.4"
    );
    assert_eq!(identity["official_channels"]["channels_agree"], true);
    assert_eq!(
        identity["official_channels"]["next_stable_not_published"],
        "1.19.5"
    );
    assert_eq!(window["previous_ceiling"], "1.18.35");
    assert_eq!(window["new_ceiling"], "1.19.4");
    assert_eq!(window["unpublished_gap"], "1.18.36");
    assert_eq!(window["v1_segment"], "1.18.18..=1.18.30");
    assert_exact_strings(
        &window["v2_segments"],
        &["1.18.31..=1.18.35", "1.19.0..=1.19.4"],
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["behavior"],
        "opencode.acp-v1.client-mcp-servers-v2"
    );
    assert_eq!(
        identity["identity_decision"]["flatten_onto_opencode_http"],
        false
    );
    assert_eq!(protocol["acp_sdk_pin"], "0.21.0");
    assert_eq!(
        protocol["source_slice"]["route_behavior_files_byte_identical"],
        true
    );
    assert_eq!(
        protocol["http_mcp_live_evidence"]["qualified_exact_version"],
        "1.18.18"
    );
    assert_eq!(
        protocol["http_mcp_live_evidence"]["transfer_to_1_19_0_through_1_19_4"],
        false
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(claim.id().as_str(), "opencode.acp.executable-window-1");
    assert_eq!(claim.axis().as_str(), "opencode.executable");
    assert_ne!(claim.axis().as_str(), http.axis().as_str());
    assert_ne!(claim.id().as_str(), http.id().as_str());
    assert_eq!(
        claim.latest_qualified().as_str(),
        OPENCODE_ACP_LATEST_QUALIFIED_VERSION
    );

    let milestones: Vec<_> = claim.milestones().collect();
    assert_eq!(milestones.len(), 3);
    assert_eq!(milestones[0].minimum().as_str(), "1.18.18");
    assert_eq!(milestones[0].maximum().as_str(), "1.18.30");
    assert_eq!(
        milestones[0].support_status(),
        InterfaceSupportStatus::Deprecated
    );
    assert_eq!(milestones[1].minimum().as_str(), "1.18.31");
    assert_eq!(milestones[1].maximum().as_str(), "1.18.35");
    assert_eq!(milestones[2].minimum().as_str(), "1.19.0");
    assert_eq!(
        milestones[2].maximum().as_str(),
        OPENCODE_ACP_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(
        milestones[1].behavior_revision().as_str(),
        milestones[2].behavior_revision().as_str()
    );
    assert_eq!(
        milestones[1].support_status(),
        InterfaceSupportStatus::Maintained
    );
    assert_eq!(
        milestones[2].support_status(),
        InterfaceSupportStatus::Maintained
    );

    for version in ["1.18.35", "1.19.0", "1.19.4"] {
        let binding = opencode_acp_binding(version).unwrap();
        assert!(matches!(
            claim.assess(binding.version()),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    let gap = opencode_acp_binding("1.18.36").unwrap();
    assert!(matches!(
        claim.assess(gap.version()),
        InterfaceCompatibilityAssessment::Incompatible
    ));
    let later = opencode_acp_binding("1.19.6").unwrap();
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn acp_behavior_sources_stay_byte_identical_across_the_hops() {
    let inventory = json(INVENTORY);
    let protocol = json(PROTOCOL);
    let hops = inventory["repository_hops"].as_array().unwrap();
    assert_eq!(hops.len(), HOPS.len());
    for (hop, expected) in hops.iter().zip(HOPS) {
        assert_eq!(hop["hop"], expected.0);
        assert_eq!(hop["repository_delta_counts"]["added"], expected.1);
        assert_eq!(hop["repository_delta_counts"]["removed"], expected.2);
        assert_eq!(hop["repository_delta_counts"]["changed"], expected.3);
        assert_eq!(hop["repository_delta_counts"]["identical"], expected.4);
        assert_eq!(hop["repository_path_sha256"]["added"], expected.5);
        assert_eq!(hop["repository_path_sha256"]["removed"], expected.6);
        assert_eq!(hop["repository_path_sha256"]["changed"], expected.7);
        assert_exact_strings(
            &hop["changed_selected_files"],
            &["packages/opencode/package.json"],
        );
    }

    let selected = inventory["selected_file_sha256"].as_object().unwrap();
    let behavior = protocol["source_slice"]["route_behavior_files"]
        .as_array()
        .unwrap();
    for path in behavior {
        let path = path.as_str().unwrap();
        let versions = selected[path].as_object().expect(path);
        assert_exact_object_keys(versions, &VERSIONS);
        let first = &versions["1.18.35"];
        for version in VERSIONS {
            assert_eq!(&versions[version], first, "{path} changed at {version}");
        }
    }
    let manifest = selected["packages/opencode/package.json"]
        .as_object()
        .unwrap();
    let mut previous = None::<String>;
    for version in VERSIONS {
        let digest = manifest[version].as_str().unwrap().to_owned();
        assert_ne!(Some(digest.as_str()), previous.as_deref());
        previous = Some(digest);
    }

    for package in ["opencode-ai", "opencode-linux-x64"] {
        let records = &inventory["npm_packages"][package]["hops"];
        for (name, _, _, _, _, _, _, _) in HOPS {
            let hop = &records[name];
            if package == "opencode-ai" {
                assert_exact_strings(&hop["changed"], &["package.json"]);
                assert_exact_strings(
                    &hop["identical"],
                    &["LICENSE", "bin/opencode.exe", "postinstall.mjs"],
                );
            } else {
                assert_exact_strings(&hop["changed"], &["bin/opencode", "package.json"]);
                assert_exact_strings(&hop["identical"], &[]);
            }
            assert_exact_strings(&hop["added"], &[]);
            assert_exact_strings(&hop["removed"], &[]);
        }
    }
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual
        .as_array()
        .expect("string array")
        .iter()
        .map(|value| value.as_str().expect("string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}
