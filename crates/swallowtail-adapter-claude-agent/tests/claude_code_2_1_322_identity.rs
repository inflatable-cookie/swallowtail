use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_response_only_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.322/identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.322/dist-inventory.json");
const SURFACES: &str = include_str!("fixtures/claude-code-2.1.322/surface-ledger.json");
const ROUTE: &str = include_str!("fixtures/claude-code-2.1.322/route-comparison.json");

fn fixture(text: &str) -> Value {
    serde_json::from_str(text).expect("frozen JSON is valid")
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

fn version(text: &str) -> InterfaceVersion {
    InterfaceVersion::new(text).expect("version is valid")
}

#[test]
fn official_stable_and_unpublished_points_are_frozen() {
    let identity = fixture(IDENTITY);
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["channel_at_observation"], json!({"stable":"2.1.311","latest":"2.1.322","next":"2.1.322"}));
    assert_eq!(identity["official_stable"], "2.1.322");
    assert_eq!(identity["first_unpublished_after_latest"], "2.1.323");
    assert_eq!(identity["unpublished_gap"], "2.1.320");
    assert_eq!(json!(keys(&identity["versions"])), json!(["2.1.319", "2.1.321", "2.1.322"]));
    for (point, published, wrapper_hash, wrapper_integrity, github_commit) in [
        ("2.1.321", "2026-11-09T19:02:47.641Z", "00ae37b8e165be25ffc4068ba698317b07fa2d3bf56146b21fb82ac950dbf161", "sha512-g9IMBKV0t092/KXM1iHE3LrsPzDaOqeYO6nHtXkztwP42Rf/w+pg/G4h5QVU51p3Nnkf/RyR0xi3V/1GEqmRsA==", "a4fcccc70a89df6607e66cc30682d551269d0680"),
        ("2.1.322", "2026-11-10T01:58:41.811Z", "39f9d22aa2938cf276494d46b0c2bb593b4ef923323d02bf3cc5805b9d9a71c9", "sha512-oP7oW4vM5DREWfL1jn4JSGlmLcrJ5QmppgADrVNJBUPiC1mQ5YvRrke0on+YZ/3QhKIaCJtdr6jPvehcAQIHLQ==", "6e1e44f6433bfa17d198e932bff53373aa229eea"),
    ] {
        let hop = &identity["versions"][point];
        assert_eq!(hop["npm_published"], published);
        assert_eq!(hop["npm_tarball_sha256"], wrapper_hash);
        assert_eq!(hop["npm_integrity"], wrapper_integrity);
        assert_eq!(hop["github_tag"], format!("v{point}"));
        assert_eq!(hop["github_commit"], github_commit);
    }
    for (point, platform, tarball_hash, integrity, binary_hash, binary_size) in [
        ("2.1.321", "claude-code-darwin-arm64", "c2a561c2f090d2875d5b7bfac7c65c9050db3416559e7f8b40979b3ca615d5a3", "sha512-Cq4TmU0aidUvmnVMhmmAqNSG7jMZ60F5APqhyU/V3G9wv+65+H632FQTO8DSFQjQlNCuX8YJT8pm2dLXT5g9Aw==", "abc9feeeca8091d51b623291e044cf0528c2f6b74e0c1d6ef671d52c73f7cd3c", 323686048),
        ("2.1.321", "claude-code-linux-x64", "249f294c2f6bfc1e6503f0780b610e880f7e9d7ae15ab34dd4ebdf23adf7664a", "sha512-lW+MhUThnU2hr6QnQ9xgD8c6OWAJbYNpiMzIQ4OJaLxcByTs53v7wV/2IzGspBbcMhnxFox1vvoltuWym/h6uw==", "426c33815ca6645a772f722fbff10dbf2cbdb124a8aeab733c1365dbf8bbf61f", 341910032),
        ("2.1.322", "claude-code-darwin-arm64", "906bde9402dfe0d2a38e931f582b7c831addef31b02b4dfa5bf05047075f4768", "sha512-OAx5Y7V7tGCi0tWuDV3lwcDWmIua512NgpClLbqvwC9m+mbLiaytgVLXf4unTXBjMf+QbQiIUO1Kus90PklDhA==", "a9d237949db816a0b7afd88a9a8e1df36b3a597a30f5a41d1c0539eb180a3450", 323702464),
        ("2.1.322", "claude-code-linux-x64", "3cc175d82980c20ab66057d345bd2d913244f8cf1791d10722a1225214b57219", "sha512-0DoaNAXLWCYYIGachWBCkTUsS8dOOgvk4R2fFKdrLejO9XlRTwaWUMD9ONdDmv0csdpwxj4qLUN6zWMaG2Sz6g==", "bfb3703ad5380242328459bc62ada4bde8ae7ba57c6e1eeadb9c00e08874c478", 341914128),
    ] {
        let package = &identity["versions"][point]["platform_packages"][platform];
        assert_eq!(package["tarball_sha256"], tarball_hash);
        assert_eq!(package["integrity"], integrity);
        assert_eq!(package["binary_sha256"], binary_hash);
        assert_eq!(package["binary_size"], binary_size);
    }
    for point in ["2.1.320", "2.1.323"] {
        assert_eq!(identity["checked_unpublished"][point]["npm_http_status"], 404);
        assert_eq!(identity["checked_unpublished"][point]["github_tag_present"], false);
    }
}

#[test]
fn complete_package_inventories_and_hop_deltas_are_exact() {
    let inventory = fixture(INVENTORY);
    let packages = ["claude-code", "claude-code-darwin-arm64", "claude-code-linux-x64"];
    assert_eq!(inventory["versions"], json!(["2.1.319", "2.1.321", "2.1.322"]));
    assert_eq!(json!(keys(&inventory["packages"])), json!(packages));
    for package in packages {
        assert_eq!(json!(keys(&inventory["packages"][package])), inventory["versions"]);
        for point in ["2.1.319", "2.1.321", "2.1.322"] {
            let files = &inventory["packages"][package][point]["files"];
            let expected_files = if package == "claude-code" {
                json!(["LICENSE.md", "README.md", "bin/claude.exe", "cli-wrapper.cjs", "install.cjs", "package.json", "sdk-tools.d.ts"])
            } else {
                json!(["LICENSE.md", "README.md", "claude", "package.json"])
            };
            assert_eq!(json!(keys(files)), expected_files, "{package}@{point}");
            assert_eq!(inventory["packages"][package][point]["tarball_sha256"].as_str().unwrap().len(), 64);
            assert!(inventory["packages"][package][point]["integrity"].is_string());
            let manifest = files.as_object().unwrap().iter().map(|(path, hash)| {
                format!("{}  {path}\n", hash.as_str().expect("file digest is text"))
            }).collect::<String>();
            let digest = format!("{:x}", Sha256::digest(manifest.as_bytes()));
            assert_eq!(inventory["packages"][package][point]["files_manifest_sha256"], digest, "{package}@{point}");
        }
    }
    assert_eq!(inventory["hop_file_changes"]["claude-code"]["2.1.319_to_2.1.321"]["changed"], json!(["package.json", "sdk-tools.d.ts"]));
    assert_eq!(inventory["hop_file_changes"]["claude-code"]["2.1.321_to_2.1.322"]["changed"], json!(["package.json"]));
    for hop in ["2.1.319_to_2.1.321", "2.1.321_to_2.1.322"] {
        assert_eq!(inventory["hop_file_changes"]["claude-code"][hop]["added"], json!([]));
        assert_eq!(inventory["hop_file_changes"]["claude-code"][hop]["removed"], json!([]));
        for file in ["bin/claude.exe", "cli-wrapper.cjs", "install.cjs"] {
            assert!(inventory["hop_file_changes"]["claude-code"][hop]["identical"].as_array().unwrap().iter().any(|entry| entry == file));
        }
    }
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        for hop in ["2.1.319_to_2.1.321", "2.1.321_to_2.1.322"] {
            assert_eq!(inventory["hop_file_changes"][package][hop]["changed"], json!(["claude", "package.json"]));
            assert_eq!(inventory["hop_file_changes"][package][hop]["added"], json!([]));
            assert_eq!(inventory["hop_file_changes"][package][hop]["removed"], json!([]));
            assert_eq!(inventory["hop_file_changes"][package][hop]["identical"], json!(["LICENSE.md", "README.md"]));
        }
    }
    assert_eq!(inventory["packages"]["claude-code"]["2.1.321"]["files"]["cli-wrapper.cjs"], inventory["packages"]["claude-code"]["2.1.319"]["files"]["cli-wrapper.cjs"]);
    assert_eq!(inventory["packages"]["claude-code"]["2.1.322"]["files"]["install.cjs"], inventory["packages"]["claude-code"]["2.1.319"]["files"]["install.cjs"]);
    for (package, point, tarball_hash) in [
        ("claude-code", "2.1.321", "00ae37b8e165be25ffc4068ba698317b07fa2d3bf56146b21fb82ac950dbf161"),
        ("claude-code", "2.1.322", "39f9d22aa2938cf276494d46b0c2bb593b4ef923323d02bf3cc5805b9d9a71c9"),
        ("claude-code-darwin-arm64", "2.1.321", "c2a561c2f090d2875d5b7bfac7c65c9050db3416559e7f8b40979b3ca615d5a3"),
        ("claude-code-darwin-arm64", "2.1.322", "906bde9402dfe0d2a38e931f582b7c831addef31b02b4dfa5bf05047075f4768"),
        ("claude-code-linux-x64", "2.1.321", "249f294c2f6bfc1e6503f0780b610e880f7e9d7ae15ab34dd4ebdf23adf7664a"),
        ("claude-code-linux-x64", "2.1.322", "3cc175d82980c20ab66057d345bd2d913244f8cf1791d10722a1225214b57219"),
    ] {
        assert_eq!(inventory["packages"][package][point]["tarball_sha256"], tarball_hash);
    }
    for (package, point, manifest_hash) in [
        ("claude-code", "2.1.321", "9e2320a4cd8d282467241bc122aacdde998dabc7d6684e7a95bfe69ba925454c"),
        ("claude-code", "2.1.322", "ed009e303063be3f6cbc850ed981f91197d38190ad6d57338e052ba0c11d0635"),
        ("claude-code-darwin-arm64", "2.1.321", "756da7121e992fa725231848c11124ac1655952fc25f43f8483d63ea83ada3db"),
        ("claude-code-darwin-arm64", "2.1.322", "2a699855b30c51fb5361649f0d9e1fc7be8602e726afd7f94c4ba73afdeeba25"),
        ("claude-code-linux-x64", "2.1.321", "f9f64eaecda3edd222077c6a897f08c364ac9958490fe45abfa8c2ed64d447a1"),
        ("claude-code-linux-x64", "2.1.322", "686e81ec68c30d85ee457c4ee643bbf7abb43c0b0b994e0a96d73506042d88cb"),
    ] {
        assert_eq!(inventory["packages"][package][point]["files_manifest_sha256"], manifest_hash);
    }
    assert_eq!(json!(keys(&inventory["changed_file_classification"]["claude-code:2.1.319_to_2.1.321"])), json!(["package.json", "sdk-tools.d.ts"]));
    assert_eq!(json!(keys(&inventory["changed_file_classification"]["claude-code:2.1.321_to_2.1.322"])), json!(["package.json"]));
    assert_eq!(
        json!(keys(&inventory["changed_file_classification"])),
        json!([
            "claude-code-darwin-arm64:2.1.319_to_2.1.321",
            "claude-code-darwin-arm64:2.1.321_to_2.1.322",
            "claude-code-linux-x64:2.1.319_to_2.1.321",
            "claude-code-linux-x64:2.1.321_to_2.1.322",
            "claude-code:2.1.319_to_2.1.321",
            "claude-code:2.1.321_to_2.1.322",
        ])
    );
    let classification_bytes =
        serde_json::to_vec(&inventory["changed_file_classification"]).expect("classification serializes");
    assert_eq!(
        format!("{:x}", Sha256::digest(&classification_bytes)),
        "6a6e602c047cdb6005f7cc931b62942bbfd0a94b871e83df0a9efc0d036c578f"
    );
}

#[test]
fn selected_behavior_stop_and_route_ledger_are_frozen() {
    let surface = fixture(SURFACES);
    let route = fixture(ROUTE);
    assert_eq!(surface["versions_scanned"], json!(["2.1.319", "2.1.321", "2.1.322"]));
    assert_eq!(surface["platforms"], json!(["claude-code-darwin-arm64", "claude-code-linux-x64"]));
    assert_eq!(surface["selected_help_lines"].as_array().unwrap().len(), 2);
    let rows = surface["marker_presence_and_counts"].as_array().unwrap();
    assert_eq!(json!(keys(rows.iter().find(|row| row["version"] == "2.1.321" && row["package"] == "claude-code-darwin-arm64").unwrap())), json!(["binary_sha256", "binary_size", "denyInsteadOfPrompting_count", "marker_counts", "package", "selected_stream_json_help_byte_identical", "version"]));
    let selected_markers = ["--input-format", "--output-format", "--permission-mode", "--setting-sources", "--strict-mcp-config", "--no-session-persistence", "--disable-slash-commands", "--safe-mode", "--mcp-config", "--include-hook-events", "--no-chrome", "--prompt-suggestions", "--tools", "stream-json", "permissionDecision", "stop_hook_active", "CLAUDE_CODE_STOP_HOOK_BLOCK_CAP", "parent_tool_use_id", "PreToolUse", "SubagentStop", "onFailure", "prompt.context", "session.start", "fs.ancestors", "instructionFiles", "AGENTS_NAMES", "CLAUDE_NAMES", "projectDirOf", "registerPlugin", "UserPromptSubmit", "updatedInput", "type:\"result\""];
    let mut sorted_selected_markers = selected_markers.to_vec();
    sorted_selected_markers.sort_unstable();
    let row = rows.iter().find(|row| row["version"] == "2.1.321" && row["package"] == "claude-code-darwin-arm64").unwrap();
    assert_eq!(json!(keys(&row["marker_counts"])), json!(sorted_selected_markers));
    let ledger_bytes = serde_json::to_vec(&surface["marker_presence_and_counts"]).expect("ledger serializes");
    assert_eq!(format!("{:x}", Sha256::digest(&ledger_bytes)), "c2e3a3743324b99f87497220c38a27e73e6de87bea2f1bc9a58ff0d7906def7a");
    for point in ["2.1.321", "2.1.322"] {
        for platform in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let row = rows.iter().find(|row| row["version"] == point && row["package"] == platform).expect("platform marker row");
            assert_eq!(row["selected_stream_json_help_byte_identical"], true);
            assert_eq!(row["marker_counts"]["--input-format"], 29);
            assert_eq!(row["marker_counts"]["--output-format"], 42);
            assert_eq!(row["marker_counts"]["--tools"], 83);
            assert_eq!(row["marker_counts"]["--safe-mode"], 14);
            assert_eq!(row["marker_counts"]["--strict-mcp-config"], 24);
            assert_eq!(row["marker_counts"]["type:\"result\""], 12);
            assert_eq!(row["denyInsteadOfPrompting_count"], if platform == "claude-code-darwin-arm64" { 35 } else { 34 });
            assert_eq!(row["binary_sha256"].as_str().unwrap().len(), 64);
        }
    }
    assert_eq!(route["behavior_revision"], "claude-code.response-only.stream-json.v3");
    assert_eq!(route["previous_ceiling"], "2.1.319");
    assert_eq!(route["hops"], json!({
        "2.1.321": "selected-behavior-unresolved",
        "2.1.322": "selected-behavior-unresolved"
    }));
    assert_eq!(route["selected_command"], json!([
        "-p", "--input-format", "text", "--output-format", "stream-json", "--verbose",
        "--no-session-persistence", "--tools", "", "--safe-mode", "--disable-slash-commands",
        "--no-chrome", "--prompt-suggestions", "false", "--mcp-config", "{\"mcpServers\":{}}",
        "--strict-mcp-config"
    ]));
    assert_eq!(route["selected_fields"], json!([
        "init", "assistant", "content_block_delta", "result", "is_error", "permissionMode",
        "tools", "mcp_servers", "num_turns"
    ]));
    assert_eq!(route["unchanged"], json!([
        "tool-free behavior", "safe-mode behavior", "strict empty MCP", "init validation",
        "typed response and failure mapping", "cancellation and joined process cleanup"
    ]));
    assert_eq!(route["qualified_ceiling"], "2.1.319");
    assert_eq!(route["preserved_gap"], "2.1.320");
    assert_eq!(route["first_unpublished_after_latest"], "2.1.323");
    assert_eq!(route["decision"], "Preserve the 2.1.319 ceiling. Exact wrapper and native package identities are frozen, but selected native behavior at both published hops is unresolved: the native binaries changed and available markers/release notes do not prove control flow or reachability. Keep 2.1.321 and 2.1.322 UnverifiedNewer, the unpublished 2.1.320 gap explicit, and 2.1.323 UnverifiedNewer.");

    assert_eq!(surface["selected_mapping"], json!({
        "argv": "The npm wrapper launch binary, `cli-wrapper.cjs`, and `install.cjs` are byte-identical to 2.1.319, so the wrapper-side argument construction is unchanged. The native executables changed at both published hops; these artifacts do not establish how the changed native code consumes `--input-format text`, `--output-format stream-json`, an empty value following `--tools`, `--safe-mode`, or strict empty MCP.",
        "stream_result_failure": "Static marker counts and printable help strings do not establish native stream parsing, init/result/failure serialization, or selected control flow. The 2.1.321 Linux NFS release note mentions text and JSON output but does not prove whether the correction reaches this stream-json route or how failures are serialized.",
        "hooks_settings": "The route passes an empty tool set and no route-configured hooks, but binary markers do not prove whether hook or settings paths are reachable from native startup. The 2.1.321 hook-counter release note and 2.1.319 denyInsteadOfPrompting markers cannot settle that question for changed executables.",
        "cancellation_cleanup": "Swallowtail adapter cancellation and joined child cleanup are covered by local process tests, but unchanged adapter tests do not establish native shutdown behavior for the changed executables."
    }));
    assert_eq!(surface["release_note_classification"], json!({
        "2.1.321": "Both native executables changed. Marker-count deltas include `--tools` 85→83, `--mcp-config` 93→94, `permissionDecision` 177→181, `parent_tool_use_id` 130→131, `PreToolUse` 350→352, and `updatedInput` 562→580 on both platforms; Linux `onFailure` remains 151 while Darwin remains 159. The release note's Linux NFS correction does not identify stream-json framing. Selected behavior and reachability therefore remain unresolved on Darwin arm64 and Linux x64.",
        "2.1.322": "Both native executables changed again (Darwin arm64 SHA-256 `a9d237949db816a0b7afd88a9a8e1df36b3a597a30f5a41d1c0539eb180a3450`; Linux x64 SHA-256 `bfb3703ad5380242328459bc62ada4bde8ae7ba57c6e1eeadb9c00e08874c478`) while the measured marker counts remain the same as 2.1.321. The Vertex fast-mode release note describes a server/API path but does not establish the changed native binaries' selected behavior."
    }));
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, "2.1.319");
    let claim = claude_code_response_only_claim();
    for point in ["2.1.318", "2.1.319"] {
        assert!(matches!(claim.assess(&version(point)), InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "claude-code.response-only.stream-json.v3"
                && matched.support_status() == InterfaceSupportStatus::Maintained));
    }
    for point in ["2.1.321", "2.1.322", "2.1.323"] {
        assert!(matches!(claim.assess(&version(point)), InterfaceCompatibilityAssessment::UnverifiedNewer(_)));
    }
    assert_eq!(claim.assess(&version("2.1.320")), InterfaceCompatibilityAssessment::Incompatible);
    assert!(matches!(claim.assess(&version("2.1.323")), InterfaceCompatibilityAssessment::UnverifiedNewer(_)));
}
