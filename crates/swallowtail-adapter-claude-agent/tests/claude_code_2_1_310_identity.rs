use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const HEADLESS_IDENTITY: &str = include_str!("fixtures/claude-code-2.1.310/headless-identity.json");
const RESPONSE_IDENTITY: &str =
    include_str!("fixtures/claude-code-2.1.310/response-only-identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.310/dist-inventory.json");
const MARKERS: &str = include_str!("fixtures/claude-code-2.1.310/marker-ledger.json");
const HEADLESS_PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.310/headless-protocol.json");
const RESPONSE_PROTOCOL: &str =
    include_str!("fixtures/claude-code-2.1.310/response-only-protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn versions() -> Vec<String> {
    (293..=310)
        .filter(|patch| !matches!(patch, 297 | 298))
        .map(|patch| format!("2.1.{patch}"))
        .collect()
}

fn published_after(start: u32) -> Vec<String> {
    ((start + 1)..=310)
        .filter(|patch| !matches!(patch, 297 | 298))
        .map(|patch| format!("2.1.{patch}"))
        .collect()
}

#[test]
fn official_stable_is_2_1_310_and_the_two_claims_stay_separate() {
    let headless = fixture(HEADLESS_IDENTITY);
    let response = fixture(RESPONSE_IDENTITY);
    assert_ne!(headless["axis"], response["axis"]);
    for identity in [&headless, &response] {
        assert_eq!(identity["official_latest_at_observation"], "2.1.310");
        assert_eq!(identity["observed_at_utc"], "2026-10-26T10:32:43Z");
        assert_eq!(
            identity["npm"]["dist_tags"],
            json!({"stable": "2.1.308", "next": "2.1.310", "latest": "2.1.310"})
        );
        assert_eq!(identity["npm"]["stable_dist_tag_is_delayed_channel"], true);
        assert_eq!(identity["github"]["latest_non_prerelease"], "v2.1.310");
        assert_eq!(
            identity["github"]["tag_commits"]["2.1.310"],
            "b70dc4113ad72e0ee44f7b1b95afaf510b53ee98"
        );
        assert_eq!(
            identity["unpublished_between"],
            json!(["2.1.297", "2.1.298"])
        );
        assert_eq!(identity["first_unpublished_after_latest"], "2.1.311");
        assert_eq!(identity["npm_2_1_297_http_status"], 404);
        assert_eq!(identity["npm_2_1_298_http_status"], 404);
        assert_eq!(identity["npm_2_1_311_http_status"], 404);
        assert_eq!(identity["github_tag_v2_1_297_present"], false);
        assert_eq!(identity["github_tag_v2_1_298_present"], false);
        assert_eq!(identity["github_tag_v2_1_311_present"], false);
        assert_eq!(identity["host"]["present"], false);
        assert_eq!(identity["identity_decision"], "compatible-extension");
        assert_eq!(identity["segment_shape"], "compatible-extension");
        assert_eq!(identity["downloaded_artifacts_executed"], false);
        assert_eq!(identity["provider_prompt_sent"], false);
        assert_eq!(identity["live_session"], false);
        assert_eq!(identity["channels_agree_on_official_stable"], true);
    }
    assert_eq!(headless["previous_ceiling"], "2.1.294");
    assert_eq!(
        headless["claim_at_observation"]["latest_qualified"],
        "2.1.294"
    );
    assert_eq!(
        headless["claim_at_observation"]["claim_id"],
        "claude-code.headless.window-1"
    );
    assert_eq!(response["previous_ceiling"], "2.1.293");
    assert_eq!(
        response["claim_at_observation"]["latest_qualified"],
        "2.1.293"
    );
    assert_eq!(
        response["claim_at_observation"]["claim_id"],
        "claude-code.response-only.window-1"
    );
    assert_eq!(
        headless["published_hops_after_previous_ceiling"],
        json!(published_after(294))
    );
    assert_eq!(
        response["published_hops_after_previous_ceiling"],
        json!(published_after(293))
    );
}

#[test]
fn both_claims_extend_through_2_1_310_and_keep_their_behavior() {
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.310");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.310"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &[
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
            "2.1.279", "2.1.297", "2.1.298",
        ]
    );
    let headless = claude_code_headless_claim();
    let response = claude_code_response_only_claim();
    assert_eq!(headless.latest_qualified().as_str(), "2.1.310");
    assert_eq!(response.latest_qualified().as_str(), "2.1.310");
    assert_ne!(headless.axis(), response.axis());
    for patch in 295..=310 {
        let version = InterfaceVersion::new(format!("2.1.{patch}")).expect("version");
        if matches!(patch, 297 | 298) {
            assert!(!headless.permits(&version), "2.1.{patch}");
            assert!(!response.permits(&version), "2.1.{patch}");
            continue;
        }
        assert!(
            matches!(
                headless.assess(&version),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "claude-code.headless.stream-json.v2"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "2.1.{patch}"
        );
        assert!(
            matches!(
                response.assess(&version),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str()
                        == "claude-code.response-only.stream-json.v3"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "2.1.{patch}"
        );
    }
    let newer = InterfaceVersion::new("2.1.311").expect("version");
    assert!(matches!(
        headless.assess(&newer),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        response.assess(&newer),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(response.supports(&InterfaceVersion::new("2.1.294").expect("version")));
}

#[test]
fn package_trees_change_only_version_metadata_and_the_runtime() {
    let inventory = fixture(INVENTORY);
    let versions = versions();
    assert_eq!(inventory["compared"], json!(versions));
    assert_eq!(
        inventory["unpublished_between"],
        json!(["2.1.297", "2.1.298"])
    );
    let packages = [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ];
    let sdk_change_hops = [
        "2.1.294_to_2.1.295",
        "2.1.295_to_2.1.296",
        "2.1.296_to_2.1.299",
        "2.1.301_to_2.1.302",
    ];
    for package in packages {
        for version in &versions {
            let count = inventory["packages"][package][version]["files"]
                .as_object()
                .expect("files")
                .len();
            assert_eq!(
                inventory["package_file_counts"][package][version], count,
                "{package} {version}"
            );
        }
        let mut previous = &versions[0];
        for version in versions.iter().skip(1) {
            let hop = format!("{previous}_to_{version}");
            let delta = &inventory["file_delta"][package][&hop];
            assert_eq!(delta["added"], json!([]), "{package} {hop}");
            assert_eq!(delta["removed"], json!([]), "{package} {hop}");
            if package == "claude-code" {
                let mut changed = vec!["package.json"];
                if sdk_change_hops.contains(&hop.as_str()) {
                    changed.push("sdk-tools.d.ts");
                }
                assert_eq!(delta["changed"], json!(changed), "{hop}");
            } else {
                assert_eq!(
                    delta["changed"],
                    json!(["claude", "package.json"]),
                    "{package} {hop}"
                );
            }
            previous = version;
        }
    }
    let wrapper = &inventory["packages"]["claude-code"];
    for name in [
        "bin/claude.exe",
        "cli-wrapper.cjs",
        "install.cjs",
        "LICENSE.md",
        "README.md",
    ] {
        let first = &wrapper["2.1.294"]["files"][name];
        for version in versions
            .iter()
            .filter(|version| version.as_str() != "2.1.293")
        {
            assert_eq!(&wrapper[version]["files"][name], first, "{name} {version}");
        }
    }
    assert_eq!(
        wrapper["2.1.310"]["tarball_sha256"],
        "d696314076114df499ba88e117000a24a9d7ab4f502dcaefba52c83fd22acfbd"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["2.1.310"]["files"]["claude"],
        "b73676430af12f97316a798b8371f81d640f90a58bdcb8f3ce56a835bb47df56"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.310"]["files"]["claude"],
        "ffb64331bc0ee1b69efb7c2c428bd3ab516bc31b75bf60886b356c97c1c25ffd"
    );
}

#[test]
fn hook_and_stream_markers_hold_and_no_hop_is_a_selected_change() {
    let inventory = fixture(INVENTORY);
    let markers = fixture(MARKERS);
    let headless = fixture(HEADLESS_PROTOCOL);
    let response = fixture(RESPONSE_PROTOCOL);
    assert_eq!(headless["decision"], "compatible-extension");
    assert_eq!(response["decision"], "compatible-extension");
    assert_eq!(
        headless["behavior_revision"],
        "claude-code.headless.stream-json.v2"
    );
    assert_eq!(
        response["behavior_revision"],
        "claude-code.response-only.stream-json.v3"
    );
    assert_eq!(
        headless["selected_behavior_boundary"]["permission_mode"],
        "plan"
    );
    assert_eq!(
        response["selected_behavior_boundary"]["permission_mode_expected_on_init"],
        "default"
    );
    assert_eq!(response["selected_behavior_boundary"]["safe_mode"], true);
    assert_eq!(headless["selected_behavior_boundary"]["safe_mode"], false);
    let scanned = published_after(294);
    assert_eq!(markers["versions_scanned"], json!(scanned));
    for version in &scanned {
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let row = &markers["versions"][version][package];
            assert_eq!(row["all_markers_present"], true, "{package} {version}");
            assert_eq!(row["help_lines_present"], true, "{package} {version}");
            assert_eq!(
                row["artifact_sha256"],
                inventory["packages"][package][version]["files"]["claude"]
            );
        }
    }
    let headless_hops = headless["per_hop_classification"].as_array().unwrap();
    let response_hops = response["per_hop_classification"].as_array().unwrap();
    assert_eq!(headless_hops.len(), scanned.len());
    assert_eq!(response_hops.len(), scanned.len() + 1);
    assert_eq!(response_hops[0]["hop"], "2.1.293_to_2.1.294");
    for (index, version) in scanned.iter().enumerate() {
        let previous = if index == 0 {
            "2.1.294".to_owned()
        } else {
            scanned[index - 1].clone()
        };
        let expected = format!("{previous}_to_{version}");
        for hop in [&headless_hops[index], &response_hops[index + 1]] {
            assert_eq!(hop["hop"], expected);
            assert_eq!(hop["selected_mapped_change"], false, "{expected}");
            assert_eq!(hop["decision"], "compatible-extension", "{expected}");
            assert!(!hop["release_note_review"].as_array().unwrap().is_empty());
        }
    }
    for hop in response_hops {
        assert_eq!(hop["selected_mapped_change"], false);
    }
}
