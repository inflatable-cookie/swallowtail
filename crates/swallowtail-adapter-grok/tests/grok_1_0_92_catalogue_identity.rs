//! Research 491 binds every published catalogue hop to the existing text revision.

use serde_json::{Value, json};

const IDENTITY: &str = include_str!("fixtures/grok-1.0.92-catalogue/identity.json");
const INVENTORY: &str = include_str!("fixtures/grok-1.0.92-catalogue/dist-inventory.json");
const SURFACE: &str = include_str!("fixtures/grok-1.0.92-catalogue/runtime-surface.json");
const PRIOR_IDENTITY: &str = include_str!("fixtures/grok-1.0.70-catalogue/identity.json");
const PRIOR_INVENTORY: &str = include_str!("fixtures/grok-1.0.70-catalogue/dist-inventory.json");

fn read(source: &str) -> Value {
    serde_json::from_str(source).expect("frozen catalogue corpus is valid JSON")
}

fn versions() -> Vec<String> {
    (70..=92).map(|patch| format!("1.0.{patch}")).collect()
}

fn keys(value: &Value) -> Vec<&str> {
    value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect()
}

fn assert_hex(value: &Value, digits: usize) {
    let value = value.as_str().expect("hex string");
    assert_eq!(value.len(), digits);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

#[test]
fn stable_hops_bind_the_wrapper_platform_and_host() {
    let identity = read(IDENTITY);
    assert_eq!(identity["route"], "grok-build.catalogue");
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["channel"]["selected_tag"], "latest");
    assert_eq!(
        identity["channel"]["dist_tags"],
        json!({"latest":"1.0.92", "alpha":"1.0.93"})
    );
    assert_eq!(
        identity["channel"]["first_unpublished_later_stable"],
        "1.0.94"
    );
    assert_eq!(identity["before"]["posture"], "QualifiedOnly");
    assert_eq!(
        identity["decision"],
        json!({"shape":"compatible-extension", "maintained_maximum":"1.0.92"})
    );

    let expected = versions();
    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), expected.len());
    let prior = read(PRIOR_IDENTITY);
    let boundary = prior["hops"].as_array().unwrap().last().unwrap();
    for package in ["wrapper", "darwin_arm64"] {
        for field in ["tarball_sha256", "shasum", "integrity"] {
            assert_eq!(hops[0][package][field], boundary[package][field]);
        }
    }
    assert_eq!(
        hops[0]["darwin_arm64"]["runtime_sha256"],
        boundary["darwin_arm64"]["runtime_sha256"]
    );
    for (hop, version) in hops.iter().zip(expected) {
        assert_eq!(hop["version"], version);
        assert_eq!(hop["wrapper"]["package"], "@xai-official/grok");
        assert_eq!(
            hop["darwin_arm64"]["package"],
            "@xai-official/grok-darwin-arm64"
        );
        let pins = &hop["wrapper"]["optional_platform_versions"];
        assert_eq!(
            keys(pins),
            [
                "@xai-official/grok-darwin-arm64",
                "@xai-official/grok-darwin-x64",
                "@xai-official/grok-linux-arm64",
                "@xai-official/grok-linux-x64",
                "@xai-official/grok-win32-arm64",
                "@xai-official/grok-win32-x64"
            ]
        );
        assert!(
            pins.as_object()
                .unwrap()
                .values()
                .all(|pin| pin == &version)
        );
        for artifact in [&hop["wrapper"], &hop["darwin_arm64"]] {
            assert!(
                artifact["tarball"]
                    .as_str()
                    .unwrap()
                    .ends_with(&format!("-{version}.tgz"))
            );
            assert!(
                artifact["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert!(artifact["source_git_head"].is_null());
            assert_hex(&artifact["shasum"], 40);
            assert_hex(&artifact["tarball_sha256"], 64);
        }
        let runtime = &hop["darwin_arm64"];
        assert_hex(&runtime["runtime_sha256"], 64);
        assert_hex(&runtime["runtime_build_id"], 12);
        if identity["host"]["version"] == version {
            for field in ["runtime_bytes", "runtime_sha256", "runtime_build_id"] {
                assert_eq!(identity["host"][field], runtime[field]);
            }
        }
    }
    assert_eq!(identity["host"]["version"], "1.0.89");
}

#[test]
fn the_complete_package_trees_only_change_manifests_and_the_payload() {
    let inventory = read(INVENTORY);
    let expected = versions();
    assert_eq!(keys(&inventory["trees"]), expected);
    assert_eq!(keys(&inventory["adjacent_deltas"]), expected[1..]);

    let prior = read(PRIOR_INVENTORY);
    for (package, unchanged, changed) in [
        (
            "wrapper",
            &[
                "README.md",
                "bin/grok",
                "bin/grok-bootstrap.js",
                "bin/postinstall.js",
            ][..],
            &["package.json"][..],
        ),
        (
            "darwin_arm64",
            &["README.md", "THIRD_PARTY_NOTICES.md"][..],
            &["bin/grok.br", "package.json"][..],
        ),
    ] {
        assert_eq!(
            inventory["trees"]["1.0.70"][package],
            prior["files"][package]["1.0.70"]
        );
        for adjacent in expected.windows(2) {
            let current = &inventory["trees"][&adjacent[1]][package];
            let previous = &inventory["trees"][&adjacent[0]][package];
            assert_eq!(
                current.as_object().unwrap().len(),
                unchanged.len() + changed.len()
            );
            for name in unchanged {
                assert_eq!(current[*name], previous[*name]);
                assert_hex(&current[*name]["sha256"], 64);
            }
            for name in changed {
                assert_ne!(current[*name]["sha256"], previous[*name]["sha256"]);
                assert_hex(&current[*name]["sha256"], 64);
            }
            assert_eq!(
                inventory["adjacent_deltas"][&adjacent[1]][package],
                json!({"added":[], "removed":[], "changed":changed, "identical":unchanged}),
            );
        }
    }
}

#[test]
fn the_selected_text_and_help_stay_on_the_existing_revision() {
    let surface = read(SURFACE);
    assert_eq!(
        surface["invocation_help"]["arguments"],
        json!(["--no-auto-update", "models", "--help"])
    );
    assert_eq!(
        surface["invocation_help"]["usage"],
        "Usage: grok models [OPTIONS]"
    );
    assert_eq!(
        surface["format_pieces"]["header"],
        json!(["Default model: ", "", "\n\nAvailable models:", "\n"])
    );
    assert_eq!(
        surface["format_pieces"]["default_row"],
        json!(["\n  * ", " (default)"])
    );
    assert_eq!(surface["format_pieces"]["plain_row"], json!(["\n  - "]));
    assert_eq!(surface["note_literal"], " (not in the list below)");

    let expected = versions();
    assert_eq!(keys(&surface["hops"]), expected);
    for (patch, version) in (70..=92).zip(expected) {
        let hop = &surface["hops"][&version];
        assert_eq!(hop["format_argument_count"], 3);
        for refs in [
            "note_references",
            "default_row_references",
            "plain_row_references",
            "request_references",
            "update_flag_references",
        ] {
            assert_eq!(hop[refs].as_array().unwrap().len(), 2, "{version} {refs}");
        }
        let help_digest = if patch < 77 {
            "7c4b7362337c5112a21e57fa4cbbef85e0bfbd3039d3bdd25052c78997287396"
        } else {
            "9db07ad36b5186156edec4aaac73e6ffc06510fe39022c58da3010be29e3822d"
        };
        assert_eq!(
            hop["cli_help"],
            json!({"exit_status":0, "stderr_bytes":0, "stdout_sha256": help_digest})
        );
        let docs = hop["embedded_documents"].as_array().unwrap();
        assert_eq!(docs.len(), 2);
        let doc_digest = if patch < 81 {
            "c7b26d2f4a4fc6f479b4ffa6c880eaec5eca1721701faa6d970d1cb4a51d5b7a"
        } else {
            "da2d31772775ae693a6b2e6cea5a920483f3002e98ddd026c43f9a1fcebe1c1c"
        };
        for doc in docs {
            assert_eq!(doc["sha256"], doc_digest);
            assert_eq!(doc["default"], "grok-4.6");
            assert_eq!(doc["model_ids"], json!(["grok-4.6", "grok-4.5"]));
        }
        let other_tables = if patch == 74 {
            json!(["0x10a421d90", "0x10a421d50", "0x10a421dd0", "0x10a421dd0"])
        } else {
            json!([])
        };
        assert_eq!(hop["additional_default_header_tables"], other_tables);
    }
}
