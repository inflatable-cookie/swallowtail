use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.293/identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.293/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.293/protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn object_keys(value: &Value) -> Vec<&str> {
    value
        .as_object()
        .expect("fixture object")
        .keys()
        .map(String::as_str)
        .collect()
}

fn assert_exact_strings(value: &Value, expected: &[&str]) {
    let mut actual = value
        .as_array()
        .expect("fixture string array")
        .iter()
        .map(|entry| entry.as_str().expect("fixture string"))
        .collect::<Vec<_>>();
    let mut expected = expected.to_vec();
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let mut actual = object_keys(value);
    let mut expected = expected.to_vec();
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

fn versions() -> Vec<String> {
    (281..=293).map(|patch| format!("2.1.{patch}")).collect()
}

fn hops() -> Vec<String> {
    versions()
        .windows(2)
        .map(|pair| format!("{}_to_{}", pair[0], pair[1]))
        .collect()
}

#[test]
fn official_identity_stops_without_changing_the_headless_claim() {
    let identity = fixture(IDENTITY);
    assert_eq!(identity["axis"], "claude-code.headless-stream-json");
    assert_eq!(identity["previous_ceiling"], "2.1.281");
    assert_eq!(identity["official_latest_at_observation"], "2.1.293");
    assert_eq!(identity["npm"]["dist_tags"]["latest"], "2.1.293");
    assert_eq!(identity["npm"]["dist_tags"]["stable"], "2.1.285");
    assert_eq!(identity["github"]["latest_non_prerelease"], "v2.1.293");
    assert_eq!(identity["first_unpublished_after_latest"], "2.1.294");
    assert_eq!(identity["npm_2.1.294_http_status"], 404);
    assert_eq!(identity["github_tag_v2.1.294_present"], false);
    assert_eq!(identity["published_hops"], json!(versions()[1..]));
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "2.1.281"
    );
    assert_eq!(
        identity["claim_at_observation"]["claim_id"],
        "claude-code.headless.window-1"
    );
    assert_eq!(
        identity["claim_at_observation"]["behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(identity["identity_decision"], "stop");
    assert_eq!(identity["segment_shape"], "stop");
    assert_eq!(
        identity["stops"].as_array().unwrap().len(),
        2,
        "both selected-surface stops remain recorded"
    );
    assert_eq!(identity["stops"][0]["version"], "2.1.287");
    assert_eq!(identity["stops"][1]["version"], "2.1.290");

    let source_tags = object_keys(&identity["github"]["tag_commits"]);
    let expected_tags = versions();
    let expected_tags = expected_tags.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(source_tags, expected_tags);

    let claim = claude_code_headless_claim();
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.294");
    assert_eq!(claim.latest_qualified().as_str(), "2.1.294");
    assert!(claim.supports(&InterfaceVersion::new("2.1.281").unwrap()));
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("2.1.294").unwrap()),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "claude-code.headless.stream-json.v2"
    ));
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("2.1.295").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.behavior_revision().as_str() == "claude-code.headless.stream-json.v2"
    ));
    for excluded in [
        "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
        "2.1.279",
    ] {
        assert!(!claim.permits(&InterfaceVersion::new(excluded).unwrap()));
    }
}

#[test]
fn every_published_package_tree_and_hop_file_set_is_frozen() {
    let inventory = fixture(INVENTORY);
    let versions = versions();
    let expected_versions = versions.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(inventory["compared"], json!(expected_versions));
    let package_names = [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ];
    assert_exact_strings(
        &Value::Array(
            object_keys(&inventory["packages"])
                .into_iter()
                .map(|name| Value::String(name.to_owned()))
                .collect(),
        ),
        &package_names,
    );

    let wrapper_files = [
        "LICENSE.md",
        "README.md",
        "bin/claude.exe",
        "cli-wrapper.cjs",
        "install.cjs",
        "package.json",
        "sdk-tools.d.ts",
    ];
    let platform_files = ["LICENSE.md", "README.md", "claude", "package.json"];
    for package in package_names {
        assert_eq!(
            object_keys(&inventory["packages"][package]),
            expected_versions
        );
        assert_eq!(
            object_keys(&inventory["file_delta"][package]),
            hops().iter().map(String::as_str).collect::<Vec<_>>()
        );
        for version in &versions {
            let record = &inventory["packages"][package][version];
            let expected_files: &[&str] = if package == "claude-code" {
                &wrapper_files
            } else {
                &platform_files
            };
            assert_exact_keys(&record["files"], expected_files);
            assert_exact_keys(&record["file_sizes_bytes"], expected_files);
            assert!(
                record["tarball_integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert_eq!(record["tarball_sha256"].as_str().unwrap().len(), 64);
            for digest in record["files"].as_object().unwrap().values() {
                assert_eq!(digest.as_str().unwrap().len(), 64);
            }
        }

        for (index, pair) in versions.windows(2).enumerate() {
            let hop = format!("{}_to_{}", pair[0], pair[1]);
            let delta = &inventory["file_delta"][package][&hop];
            assert_eq!(delta["added"], json!([]), "{package} {hop}");
            assert_eq!(delta["removed"], json!([]), "{package} {hop}");
            let expected_changed: &[&str] = if package == "claude-code" {
                match index {
                    2 | 3 => &["package.json", "sdk-tools.d.ts"],
                    6 => &["install.cjs", "package.json"],
                    8 | 10 => &["package.json", "sdk-tools.d.ts"],
                    _ => &["package.json"],
                }
            } else {
                &["claude", "package.json"]
            };
            assert_exact_strings(&delta["changed"], expected_changed);
            let expected_identical: &[&str] = if package == "claude-code" {
                match index {
                    2 | 3 => &[
                        "LICENSE.md",
                        "README.md",
                        "bin/claude.exe",
                        "cli-wrapper.cjs",
                        "install.cjs",
                    ],
                    6 => &[
                        "LICENSE.md",
                        "README.md",
                        "bin/claude.exe",
                        "cli-wrapper.cjs",
                        "sdk-tools.d.ts",
                    ],
                    8 | 10 => &[
                        "LICENSE.md",
                        "README.md",
                        "bin/claude.exe",
                        "cli-wrapper.cjs",
                        "install.cjs",
                    ],
                    _ => &[
                        "LICENSE.md",
                        "README.md",
                        "bin/claude.exe",
                        "cli-wrapper.cjs",
                        "install.cjs",
                        "sdk-tools.d.ts",
                    ],
                }
            } else {
                &["LICENSE.md", "README.md"]
            };
            assert_exact_strings(&delta["identical"], expected_identical);
        }
    }
}

#[test]
fn selected_command_and_every_changed_file_classification_are_exact() {
    let protocol = fixture(PROTOCOL);
    assert_eq!(
        protocol["behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(protocol["decision"], "stop");
    assert_eq!(
        protocol["selected_command"]["arguments"],
        json!([
            "-p",
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "<caller-selected-model>",
            "--permission-mode",
            "plan",
            "--tools",
            "Read,Glob,Grep",
            "--setting-sources",
            "user,project,local",
            "--mcp-config",
            "{\"mcpServers\":{}}",
            "--strict-mcp-config"
        ])
    );
    assert_eq!(
        protocol["selected_behavior_boundary"]["slash_commands_disabled"],
        false
    );
    assert_eq!(
        protocol["selected_behavior_boundary"]["session_persistence"],
        false
    );
    assert_eq!(protocol["static_string_inspection"]["binaries_checked"], 26);
    assert_eq!(protocol["static_string_inspection"]["missing"], json!([]));
    assert_eq!(
        protocol["per_hop_classification"].as_array().unwrap().len(),
        12
    );

    let inventory = fixture(INVENTORY);
    let mut stop_hops = Vec::new();
    for (index, hop_record) in protocol["per_hop_classification"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let hop = hops()[index].clone();
        assert_eq!(hop_record["hop"], hop);
        let mut inventory_changed = Vec::new();
        for package in [
            "claude-code",
            "claude-code-darwin-arm64",
            "claude-code-linux-x64",
        ] {
            for file in inventory["file_delta"][package][&hop]["changed"]
                .as_array()
                .unwrap()
            {
                inventory_changed.push(format!("{package}/{}", file.as_str().unwrap()));
            }
        }
        inventory_changed.sort();
        let mut classified = hop_record["changed_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|file| {
                assert!(!file["classification"].as_str().unwrap().is_empty());
                file["path"].as_str().unwrap().to_owned()
            })
            .collect::<Vec<_>>();
        classified.sort();
        assert_eq!(classified, inventory_changed, "{hop} file classifications");
        if hop_record["selected_mapped_change"] == true {
            stop_hops.push(hop);
            assert_eq!(hop_record["decision"], "stop");
        } else {
            assert_eq!(hop_record["decision"], "no-selected-change-identified");
        }
    }
    assert_eq!(stop_hops, ["2.1.286_to_2.1.287", "2.1.289_to_2.1.290"]);
}
