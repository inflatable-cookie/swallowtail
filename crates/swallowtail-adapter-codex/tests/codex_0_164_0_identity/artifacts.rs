use super::support::{HOPS, IDENTITY, PROTOCOL, assert_sha256, json};

#[test]
fn official_channels_freeze_every_stable_hop_after_0_161_0() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["package"], "@openai/codex");
    assert_eq!(identity["previousCeiling"], "0.161.0");
    assert_eq!(identity["officialLatest"], "0.164.0");
    assert_eq!(identity["channelsAgree"], true);
    assert_eq!(identity["prereleaseIgnored"], "0.165.0-alpha.1");
    assert_eq!(identity["downloadedArtifactsExecuted"], false);
    assert_eq!(identity["platformBinariesDownloaded"], true);
    assert_eq!(
        identity["behaviorAuthority"],
        "public-source-tags-and-hashed-native-artifacts"
    );
    assert_eq!(identity["syntheticUnverifiedNewer"], "0.164.1");
    assert_eq!(
        identity["nativeSelection"]["darwinArm64"],
        "optionalDependencies alias npm:@openai/codex@VER-darwin-arm64"
    );
    assert_eq!(
        identity["nativeSelection"]["linuxX64"],
        "optionalDependencies alias npm:@openai/codex@VER-linux-x64"
    );

    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), HOPS.len());
    for (hop, expected) in hops.iter().zip(HOPS) {
        assert_eq!(hop["version"], *expected);
        assert_eq!(hop["githubTag"], format!("rust-v{expected}"));
        assert_eq!(hop["tagObject"].as_str().unwrap().len(), 40);
        assert_eq!(hop["commit"].as_str().unwrap().len(), 40);
        assert_sha256(&hop["wrapper"]["sha256"]);
        assert!(
            hop["wrapper"]["integrity"]
                .as_str()
                .unwrap()
                .starts_with("sha512-")
        );
        for native in ["darwinArm64", "linuxX64"] {
            assert_sha256(&hop[native]["sha256"]);
            assert!(
                hop[native]["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert!(hop[native]["files"].as_u64().unwrap() > 0);
            assert_sha256(&hop[native]["cliSha256"]);
            assert_eq!(
                hop[native]["packageVersion"],
                format!("{expected}-{}", native_suffix(native))
            );
            assert_eq!(
                hop[native]["npmAlias"],
                format!("npm:@openai/codex@{expected}-{}", native_suffix(native))
            );
        }
        assert!(identity["sourceFileCounts"][expected].as_u64().unwrap() > 0);
    }
    assert_eq!(
        hops[0]["commit"],
        "979011409de0a60b52f179721948e65531d26144"
    );
    assert_eq!(
        hops[4]["commit"],
        "6aaac694803004a63c6f393d153c96d22521a6b9"
    );
    assert_eq!(
        hops[4]["wrapper"]["sha256"],
        "afb117ec0d382744583dd13154ba0ca6b0f1980f8ed6a23bffb3aaadb6226e30"
    );
    assert_eq!(
        hops[4]["darwinArm64"]["sha256"],
        "9b5a35be1975ceb332eaf274032f375b017ab644ff0165737ead03277fae77cd"
    );
    assert_eq!(
        hops[4]["linuxX64"]["sha256"],
        "62659fefd9a8efa21a496402fde3c11c025bbeee735a582968f8ffd14cb5f79c"
    );
    assert_eq!(
        hops[0]["darwinArm64"]["sha256"],
        "21c99b848b25ed91b92afadabe3751cc33878d2684f3e2d01cc1782e6d0abc7a"
    );
    assert_eq!(
        hops[0]["linuxX64"]["sha256"],
        "d50e02c2755be551b576527b2b13c6b6a4af3340149afdf792d5491e5d899978"
    );
    assert_eq!(identity["sourceFileCounts"]["0.161.0"], 8899);
    assert_eq!(identity["sourceFileCounts"]["0.164.0"], 9438);
}

#[test]
fn downloaded_natives_were_hashed_and_never_executed() {
    let identity = json(IDENTITY);
    assert_eq!(identity["downloadedArtifactsExecuted"], false);
    assert_eq!(identity["platformBinariesDownloaded"], true);
    assert_eq!(json(PROTOCOL)["downloadedBinariesExecuted"], false);
    assert_eq!(json(PROTOCOL)["platformBinariesHashed"], true);
    assert_eq!(identity["hostObservation"], serde_json::Value::Null);
    for hop in identity["hops"].as_array().expect("hops") {
        let provenance = hop["darwinArm64"]["provenance"].as_str().unwrap();
        assert!(
            provenance == "hashed-unexecuted-tarball" || provenance == "research-370",
            "{}",
            provenance
        );
    }
}

fn native_suffix(native: &str) -> &'static str {
    match native {
        "darwinArm64" => "darwin-arm64",
        "linuxX64" => "linux-x64",
        other => panic!("unknown native key {other}"),
    }
}
