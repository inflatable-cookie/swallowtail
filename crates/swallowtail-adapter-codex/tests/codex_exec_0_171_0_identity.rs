use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_codex::{
    CODEX_LATEST_QUALIFIED_VERSION, codex_app_server_claim, codex_app_server_lifecycle_claim,
    codex_exec_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/codex-exec-0.171.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/codex-exec-0.171.0/protocol.json");
const DIST: &str = include_str!("fixtures/codex-exec-0.171.0/dist-inventory.json");
const SOURCE: &str = include_str!("fixtures/codex-exec-0.171.0/source-inventory.json");

const EXPECTED_VERSIONS: [&str; 11] = [
    "0.164.0", "0.165.0", "0.166.0", "0.167.0", "0.167.1", "0.167.2", "0.168.0", "0.169.0",
    "0.169.1", "0.170.0", "0.171.0",
];

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn assert_sha256(value: &Value) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64, "{value}");
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

#[test]
fn channels_and_host_agree_on_0_171_0_with_all_ten_hops_frozen() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["package"], "@openai/codex");
    assert_eq!(identity["routes"], serde_json::json!(["codex.exec"]));
    assert_eq!(
        identity["untouchedSiblingRoutes"],
        serde_json::json!(["codex.app-server"])
    );
    assert_eq!(identity["previousCeiling"], "0.164.0");
    assert_eq!(identity["officialLatest"], "0.171.0");
    assert_eq!(identity["channelsAgree"], true);
    assert_eq!(identity["prereleaseIgnored"], "0.173.0-alpha.4");
    assert_eq!(identity["downloadedArtifactsExecuted"], false);
    assert_eq!(identity["platformBinariesDownloaded"], true);
    assert_eq!(identity["syntheticUnverifiedNewer"], "0.171.1");

    let host = &identity["hostObservation"];
    assert_eq!(host["version"], "0.171.0");
    assert_eq!(host["cliOutput"], "codex-cli 0.171.0");
    assert_eq!(host["matchesOfficialDarwinArm64Cli"], true);
    assert_eq!(
        host["sha256"],
        "fc8d77cf7585aded2b9bc487dad29c4ee2f066123e4e76dd3b17653a8322ce42"
    );

    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), EXPECTED_VERSIONS.len());
    for (hop, expected) in hops.iter().zip(EXPECTED_VERSIONS) {
        assert_eq!(hop["version"], expected);
        assert_eq!(hop["githubTag"], format!("rust-v{expected}"));
        assert_sha256(&hop["wrapper"]["sha256"]);
        assert_sha256(&hop["wrapper"]["binCodexJsSha256"]);
        assert_sha256(&hop["darwinArm64"]["sha256"]);
        assert_sha256(&hop["darwinArm64"]["cliSha256"]);
        assert_sha256(&hop["linuxX64"]["sha256"]);
        assert_sha256(&hop["linuxX64"]["cliSha256"]);
        assert_sha256(&hop["linuxX64"]["bwrapSha256"]);
    }
    assert_eq!(
        hops.last().expect("0.171.0 hop")["darwinArm64"]["cliSha256"],
        host["sha256"]
    );
}

#[test]
fn bwrap_trusted_sigstore_provenance_and_embedded_digests_close_research_463_gate() {
    let identity = json(IDENTITY);
    let anchor = &identity["sigstoreTrustAnchor"];
    assert_eq!(
        anchor["trustedRootSha256"],
        "844a1c6de3986c9f02070266b25e0d1a2fa99ceccc89f6b9ad90aae47b62a16e"
    );
    assert_eq!(anchor["rekorBaseUrl"], "https://rekor.sigstore.dev");
    assert_eq!(
        anchor["rekorLogIds"],
        serde_json::json!([
            "c0d23d6ad406973f9559f3ba2d1ca01f84147d8ffc5b8445c224f98b9591801d",
            "cf1199155bddd051268d1f16ac5c0c75c009f6fb5a63f4177f8e18d7051e3fa0"
        ])
    );
    assert_eq!(
        anchor["fulcioRootDerSha256s"],
        serde_json::json!([
            "03a38ffb1f450100c2596d1d10b900ac4d504058006dda58199576bbeb9c73d0",
            "3ba7b6cc4e95469d4d334b49cb257ad8537076fa84b0ca87ff4ecfe6a54680c1"
        ])
    );

    let hops = identity["hops"].as_array().expect("hops");
    let bwrap = identity["bwrapProvenance"]
        .as_object()
        .expect("bwrapProvenance");
    assert_eq!(bwrap.len(), EXPECTED_VERSIONS.len());
    for (hop, v) in hops.iter().zip(EXPECTED_VERSIONS) {
        let entry = bwrap.get(v).expect("bwrap entry for version");
        assert_sha256(&entry["sha256"]);
        assert_eq!(entry["sha256"], hop["linuxX64"]["bwrapSha256"], "{v}");
        assert_sha256(&entry["sigstoreBundleSha256"]);
        assert_sha256(&entry["leafCertDerSha256"]);
        assert_eq!(
            entry["rekorLogId"], "c0d23d6ad406973f9559f3ba2d1ca01f84147d8ffc5b8445c224f98b9591801d",
            "{v}"
        );
        assert!(entry["rekorLogIndex"].as_u64().is_some(), "{v}");
        assert!(entry["rekorIntegratedTime"].as_u64().is_some(), "{v}");
        assert_eq!(entry["rekorHashMatches"], true, "{v}");
        assert_eq!(entry["rekorSetVerifiedAgainstTrustedRoot"], true, "{v}");
        assert_eq!(entry["fulcioChainVerifiedAtIntegratedTime"], true, "{v}");
        assert_eq!(entry["blobSignatureVerifiedAgainstLeafCert"], true, "{v}");
        assert_eq!(entry["embeddedInLinuxCodexBinary"], true, "{v}");
        assert_eq!(
            entry["oidcIssuer"], "https://token.actions.githubusercontent.com",
            "{v}"
        );
        assert_eq!(entry["sourceRepositoryDigest"], hop["commit"], "{v}");
        assert_eq!(entry["buildConfigDigest"], hop["commit"], "{v}");
        assert_eq!(
            entry["fulcioWorkflowRef"],
            format!(
                "https://github.com/openai/codex/.github/workflows/rust-release.yml@refs/tags/rust-v{v}"
            )
        );
    }
    assert_eq!(bwrap["0.166.0"]["sha256"], bwrap["0.167.0"]["sha256"]);
    assert_eq!(bwrap["0.166.0"]["sha256"], bwrap["0.167.1"]["sha256"]);
    assert_eq!(bwrap["0.166.0"]["sha256"], bwrap["0.168.0"]["sha256"]);
    assert_eq!(bwrap["0.169.0"]["sha256"], bwrap["0.169.1"]["sha256"]);
    assert_eq!(bwrap["0.169.0"]["sha256"], bwrap["0.170.0"]["sha256"]);
}

#[test]
fn exec_claim_extends_to_0_171_0_while_app_server_stays_at_0_164_0() {
    assert_eq!(CODEX_LATEST_QUALIFIED_VERSION, "0.171.0");

    let exec_claim = codex_exec_claim();
    for v in EXPECTED_VERSIONS {
        let InterfaceCompatibilityAssessment::Qualified(matched) = exec_claim.assess(&version(v))
        else {
            panic!("codex.exec {v} must be qualified");
        };
        assert_eq!(matched.behavior_revision().as_str(), "codex.exec.jsonl-v1");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        exec_claim.assess(&version("0.171.1"))
    else {
        panic!("0.171.1 must remain unverified newer for codex.exec");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.171.0");

    for gap in [
        "0.82.0", "0.83.0", "0.108.0", "0.109.0", "0.149.2", "0.150.2", "0.151.1", "0.152.2",
        "0.154.1", "0.155.2",
    ] {
        assert_eq!(
            exec_claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "retained gap {gap} stays incompatible"
        );
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(app_newer) =
        codex_app_server_claim().assess(&version("0.165.0"))
    else {
        panic!("codex.app-server 0.165.0 must stay unverified newer in exec-only task");
    };
    assert_eq!(app_newer.latest_qualified().as_str(), "0.164.0");

    let InterfaceCompatibilityAssessment::UnverifiedNewer(lifecycle_newer) =
        codex_app_server_lifecycle_claim().assess(&version("0.165.0"))
    else {
        panic!("codex.app-server lifecycle 0.165.0 must stay unverified newer in exec-only task");
    };
    assert_eq!(lifecycle_newer.latest_qualified().as_str(), "0.164.0");
}

#[test]
fn protocol_managed_config_and_per_hop_inventories_are_mutation_sensitive() {
    // Pin exact corpus bytes so mutating any path or digest in either inventory fails immediately.
    assert_eq!(
        sha256_hex(DIST.as_bytes()),
        "5167c78104498d1be90f1700bc69d47aea743a33fdd006d7b6c1168f2848445d"
    );
    assert_eq!(
        sha256_hex(SOURCE.as_bytes()),
        "de963fa3c51a53eb51df7459e57b3992af2ee1b97833b2e3505e5d7ae50cae35"
    );

    let protocol = json(PROTOCOL);
    assert_eq!(protocol["route"], "codex.exec");
    assert_eq!(protocol["selectedMappedSubsetUnchanged"], true);
    assert_eq!(protocol["noPrompt"], true);
    assert_eq!(protocol["noLiveSession"], true);
    assert_eq!(protocol["noHostMutation"], true);
    assert_eq!(protocol["downloadedBinariesExecuted"], false);

    let managed = &protocol["managedConfigServerClassification"];
    assert_eq!(managed["introducedInCloudConfig"], "0.165.0");
    assert_eq!(managed["wiredIntoExec"], "0.166.0");
    assert_eq!(
        managed["subsequentChanges"],
        serde_json::json!(["0.168.0", "0.171.0"])
    );
    assert_eq!(managed["nonInteractiveAuth"], true);
    assert_eq!(managed["requiresOperatorRuling"], false);
    assert_eq!(
        managed["embeddedBootstrapRetainsConfigServer"],
        "0.166.0..=0.171.0"
    );
    assert_eq!(
        managed["applicationNetworkConsumesManagedLoader"],
        "0.166.0..=0.171.0"
    );
    assert_eq!(managed["startupCacheIdentityMatched"], true);
    assert_eq!(
        managed["firstTurnEffectiveConfigProviderCheckVersions"],
        serde_json::json!(["0.166.0", "0.167.0", "0.167.1", "0.167.2"])
    );
    assert_eq!(
        managed["firstTurnManagedOnlyProviderCheckVersions"],
        serde_json::json!([
            "0.164.0", "0.165.0", "0.168.0", "0.169.0", "0.169.1", "0.170.0", "0.171.0"
        ])
    );

    let bwrap_gate = &protocol["bwrapGateResolution"];
    assert_eq!(bwrap_gate["research463GateClosed"], true);
    assert_eq!(bwrap_gate["allElevenHopsTrustedSigstoreVerified"], true);
    assert_eq!(bwrap_gate["trustedRootVerified"], true);
    assert_eq!(bwrap_gate["fulcioChainVerifiedAtIntegratedTime"], true);
    assert_eq!(bwrap_gate["rekorSetVerifiedAgainstTrustedKey"], true);
    assert_eq!(bwrap_gate["allElevenHopsEmbeddedInLinuxCodex"], true);
    assert_eq!(bwrap_gate["bwrapSourceAndVendorCByteIdentical"], true);

    let identical = protocol["byteIdenticalThroughAllTenHops"]
        .as_object()
        .expect("byteIdenticalThroughAllTenHops");
    assert_eq!(identical.len(), 11);
    assert_eq!(
        identical["codex-rs/exec/src/exec_events.rs"],
        "dafa872d7e86a099e56e28a329dcb9c03db90ed768c3b88cca8c91d46dc1d0e5"
    );
    assert_eq!(
        identical["codex-rs/linux-sandbox/src/bundled_bwrap.rs"],
        "f2825debef7272c72ae039d94d8bec7ca9e84e3ab534c3d59a9c26ae02a2ed80"
    );
    assert_eq!(
        identical["codex-rs/bwrap/src/main.rs"],
        "1fdfa3549d633f7d260c9bccc896f59a7f32275b9130b72ee796622b639081aa"
    );
    assert_eq!(
        identical["codex-rs/vendor/bubblewrap/bubblewrap.c"],
        "9bc38fb46080b6854e0c414ccb5fbd369d9d7c0230fdfa877283d31aef0c5720"
    );

    let deltas = protocol["classifiedDeltas"]
        .as_object()
        .expect("classifiedDeltas");
    assert_eq!(deltas.len(), 21);
    for key in [
        "codex-rs/exec/src/cli.rs",
        "codex-rs/exec/src/event_processor_with_jsonl_output.rs",
        "codex-rs/exec/src/event_processor.rs",
        "codex-rs/exec/src/lib.rs",
        "codex-rs/cloud-config/src/bundle_loader.rs",
        "codex-rs/cloud-config/src/config_server.rs",
        "codex-rs/cloud-config/src/service.rs",
        "codex-rs/cloud-config/src/validation.rs",
        "codex-rs/config/src/config_server.rs",
        "codex-rs/login/src/outbound_proxy.rs",
        "codex-rs/http-client/src/network_policy.rs",
        "codex-rs/app-server/src/in_process_bootstrap.rs",
        "codex-rs/app-server/src/application_network.rs",
        "codex-rs/app-server/src/config_manager.rs",
        "codex-rs/app-server/src/request_processors/turn_processor.rs",
        "codex-rs/linux-sandbox/src/bwrap.rs",
        "codex-rs/linux-sandbox/src/linux_run_main.rs",
        "codex-rs/linux-sandbox/src/launcher.rs",
        "codex-rs/sandboxing/src/bwrap.rs",
        "codex-rs/sandboxing/src/seatbelt.rs",
        "bin/codex.js",
    ] {
        assert!(deltas.contains_key(key), "missing delta {key}");
    }

    let source = json(SOURCE);
    let feeding = source["selectedFeedingFiles"]
        .as_array()
        .expect("selectedFeedingFiles");
    assert_eq!(feeding.len(), 23);
    let hashes = source["selectedFeedingFileSha256"]
        .as_object()
        .expect("selectedFeedingFileSha256");
    let per_hop = source["perHopSelectedFeedingChanges"]
        .as_object()
        .expect("perHopSelectedFeedingChanges");
    assert_eq!(per_hop.len(), 10);
    for pair in EXPECTED_VERSIONS.windows(2) {
        let hop_key = format!("{}->{}", pair[0], pair[1]);
        let expected_changed: Vec<&str> = feeding
            .iter()
            .filter_map(Value::as_str)
            .filter(|path| hashes[*path][pair[0]] != hashes[*path][pair[1]])
            .collect();
        let recorded: Vec<&str> = per_hop[&hop_key]
            .as_array()
            .expect("hop array")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(recorded, expected_changed, "hop mismatch for {hop_key}");
    }
    assert_eq!(
        per_hop["0.165.0->0.166.0"],
        serde_json::json!([
            "codex-rs/exec/src/lib.rs",
            "codex-rs/exec/src/event_processor_with_jsonl_output.rs",
            "codex-rs/cloud-config/src/bundle_loader.rs",
            "codex-rs/cloud-config/src/config_server.rs",
            "codex-rs/config/src/config_requirements.rs",
            "codex-rs/http-client/src/network_policy.rs",
            "codex-rs/app-server/src/in_process_bootstrap.rs",
            "codex-rs/app-server/src/application_network.rs",
            "codex-rs/app-server/src/config_manager.rs",
            "codex-rs/app-server/src/request_processors/turn_processor.rs",
            "codex-rs/linux-sandbox/src/bwrap.rs"
        ])
    );
    assert_eq!(
        per_hop["0.167.2->0.168.0"],
        serde_json::json!([
            "codex-rs/exec/src/lib.rs",
            "codex-rs/exec/src/event_processor.rs",
            "codex-rs/exec/src/event_processor_with_jsonl_output.rs",
            "codex-rs/cloud-config/src/bundle_loader.rs",
            "codex-rs/cloud-config/src/config_server.rs",
            "codex-rs/app-server/src/config_manager.rs",
            "codex-rs/app-server/src/request_processors/turn_processor.rs",
            "codex-rs/linux-sandbox/src/bwrap.rs",
            "codex-rs/linux-sandbox/src/linux_run_main.rs",
            "codex-rs/sandboxing/src/seatbelt.rs"
        ])
    );

    let dist = json(DIST);
    let artifact_hops = dist["artifactHops"].as_object().expect("artifactHops");
    assert_eq!(artifact_hops.len(), 10);
    for (hop, expected_wrapper, expected_bwrap_changed) in [
        ("0.164.0->0.165.0", vec!["package.json"], true),
        (
            "0.165.0->0.166.0",
            vec!["bin/codex.js", "package.json"],
            true,
        ),
        ("0.166.0->0.167.0", vec!["package.json"], false),
        ("0.167.0->0.167.1", vec!["package.json"], false),
        ("0.167.1->0.167.2", vec!["package.json"], true),
        ("0.167.2->0.168.0", vec!["package.json"], true),
        ("0.168.0->0.169.0", vec!["package.json"], true),
        ("0.169.0->0.169.1", vec!["package.json"], false),
        ("0.169.1->0.170.0", vec!["package.json"], false),
        (
            "0.170.0->0.171.0",
            vec!["bin/codex.js", "package.json"],
            true,
        ),
    ] {
        let entry = &artifact_hops[hop];
        assert_eq!(
            entry["wrapper"]["changed"],
            serde_json::json!(expected_wrapper),
            "wrapper changed mismatch for {hop}"
        );
        let lx_changed = entry["linuxX64"]["changed"]
            .as_array()
            .expect("linuxX64 changed");
        assert_eq!(
            lx_changed
                .iter()
                .any(|p| p == "vendor/x86_64-unknown-linux-musl/codex-resources/bwrap"),
            expected_bwrap_changed,
            "bwrap changed mismatch for {hop}"
        );
    }
}
