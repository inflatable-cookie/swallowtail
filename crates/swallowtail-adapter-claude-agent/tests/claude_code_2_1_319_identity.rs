use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.319/identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.319/dist-inventory.json");
const SURFACES: &str = include_str!("fixtures/claude-code-2.1.319/surface-ledger.json");
const ROUTES: &str = include_str!("fixtures/claude-code-2.1.319/route-comparison.json");

fn fixture(text: &str) -> Value {
    serde_json::from_str(text).expect("frozen JSON is valid")
}

fn version(text: &str) -> InterfaceVersion {
    InterfaceVersion::new(text).expect("version is valid")
}

fn keys(value: &Value) -> Vec<String> {
    let mut keys = value
        .as_object()
        .expect("object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

#[test]
fn npm_and_github_identity_is_frozen_for_both_published_hops() {
    let identity = fixture(IDENTITY);
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["official_stable"], "2.1.319");
    assert_eq!(
        identity["channel_at_observation"],
        json!({"stable":"2.1.311", "latest":"2.1.319", "next":"2.1.319"})
    );
    assert_eq!(
        json!(keys(&identity["versions"])),
        json!(["2.1.317", "2.1.318", "2.1.319"])
    );
    for (point, published, sha, tag, commit) in [
        (
            "2.1.318",
            "2026-11-03T21:55:17.247Z",
            "d86c0189406af48b393f658ea1c0ba9a8fd10dc3c461e0ad50887b465a18b83c",
            "v2.1.318",
            "646b74189c735abbccf78ef5cb7ddd38a07a36eb",
        ),
        (
            "2.1.319",
            "2026-11-06T00:58:56.366Z",
            "733a926c19df1c84f35a8e4285a637edce7bf2de13672cfe13a1397efd0bf823",
            "v2.1.319",
            "0ed94698aa0aeee471465fc9c867c2e8434c4d68",
        ),
    ] {
        let hop = &identity["versions"][point];
        assert_eq!(hop["npm_published"], published);
        assert_eq!(hop["npm_tarball_sha256"], sha);
        assert_eq!(hop["github_tag"], tag);
        assert_eq!(hop["github_commit"], commit);
    }
    assert_eq!(identity["first_unpublished_after_latest"], "2.1.320");
    assert_eq!(identity["checked_unpublished"]["2.1.320"]["npm_http_status"], 404);
    assert_eq!(identity["checked_unpublished"]["2.1.320"]["github_tag_present"], false);
}

#[test]
fn frozen_package_inventories_cover_exact_files_and_hops() {
    let inventory = fixture(INVENTORY);
    let package_names = [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ];
    assert_eq!(json!(keys(&inventory["packages"])), json!(package_names));
    assert_eq!(inventory["versions"], json!(["2.1.317", "2.1.318", "2.1.319"]));
    for package in package_names {
        assert_eq!(
            json!(keys(&inventory["packages"][package])),
            inventory["versions"]
        );
        for point in ["2.1.317", "2.1.318", "2.1.319"] {
            assert_eq!(
                inventory["packages"][package][point]["files"].as_object().unwrap().len(),
                if package == "claude-code" { 7 } else { 4 },
                "{package}@{point}"
            );
            assert_eq!(
                inventory["packages"][package][point]["tarball_sha256"]
                    .as_str()
                    .unwrap()
                    .len(),
                64
            );
            assert!(inventory["packages"][package][point]["integrity"].is_string());
        }
    }
    assert_eq!(
        inventory["hop_file_changes"]["claude-code"]["2.1.317_to_2.1.318"]["changed"],
        json!(["package.json", "sdk-tools.d.ts"])
    );
    assert_eq!(
        inventory["hop_file_changes"]["claude-code"]["2.1.318_to_2.1.319"]["changed"],
        json!(["cli-wrapper.cjs", "install.cjs", "package.json", "sdk-tools.d.ts"])
    );
    assert_eq!(
        inventory["changed_file_classification"]["claude-code:2.1.318_to_2.1.319"]["cli-wrapper.cjs"],
        "runtime launcher rejects duplicate or unreadable platform package manifests; failure-only protection for ambiguous or damaged installs, with exact package argv unchanged"
    );
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        for hop in ["2.1.317_to_2.1.318", "2.1.318_to_2.1.319"] {
            assert_eq!(
                inventory["hop_file_changes"][package][hop]["changed"],
                json!(["claude", "package.json"])
            );
            assert_eq!(inventory["hop_file_changes"][package][hop]["added"], json!([]));
            assert_eq!(inventory["hop_file_changes"][package][hop]["removed"], json!([]));
        }
    }
}

#[test]
fn permission_setting_stops_headless_but_does_not_reach_response_only() {
    let surfaces = fixture(SURFACES);
    let routes = fixture(ROUTES);
    assert_eq!(surfaces["versions_scanned"], json!(["2.1.317", "2.1.318", "2.1.319"]));
    assert_eq!(surfaces["permission_setting_delta"]["key"], "permissions.denyInsteadOfPrompting");
    assert_eq!(surfaces["permission_setting_delta"]["absent_in"], json!(["2.1.317", "2.1.318"]));
    assert_eq!(surfaces["permission_setting_delta"]["present_in"], json!(["2.1.319"]));
    assert_eq!(routes["headless"]["hops"]["2.1.319"], "stop-selected-permission-flow");
    assert_eq!(routes["response_only"]["hops"]["2.1.319"], "compatible-extension");
    assert_eq!(routes["response_only"]["qualified_ceiling"], "2.1.319");
    for entry in surfaces["marker_presence_and_counts"].as_array().unwrap() {
        if entry["version"] == "2.1.318" || entry["version"] == "2.1.319" {
            assert_eq!(entry["selected_stream_json_help_byte_identical"], true);
        }
    }

    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.317");
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, "2.1.322");
    let headless = claude_code_headless_claim();
    let response_only = claude_code_response_only_claim();
    assert!(matches!(
        headless.assess(&version("2.1.319")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    for point in ["2.1.318", "2.1.319", "2.1.321", "2.1.322"] {
        assert!(matches!(
            response_only.assess(&version(point)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.behavior_revision().as_str()
                    == "claude-code.response-only.stream-json.v3"
                    && matched.support_status() == InterfaceSupportStatus::Maintained
        ));
    }
    assert!(matches!(
        response_only.assess(&version("2.1.320")),
        InterfaceCompatibilityAssessment::Incompatible
    ));
    assert!(matches!(
        response_only.assess(&version("2.1.323")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
