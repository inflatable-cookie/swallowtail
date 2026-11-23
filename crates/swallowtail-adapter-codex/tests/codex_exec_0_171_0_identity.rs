use serde_json::Value;
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
fn bwrap_sigstore_provenance_and_embedded_digests_close_research_463_gate() {
    let identity = json(IDENTITY);
    let bwrap = identity["bwrapProvenance"]
        .as_object()
        .expect("bwrapProvenance");
    assert_eq!(bwrap.len(), EXPECTED_VERSIONS.len());
    for v in EXPECTED_VERSIONS {
        let entry = bwrap.get(v).expect("bwrap entry for version");
        assert_sha256(&entry["sha256"]);
        assert_eq!(entry["rekorHashMatches"], true, "{v}");
        assert_eq!(entry["opensslSignatureVerified"], true, "{v}");
        assert_eq!(entry["fulcioCommitMatchesTag"], true, "{v}");
        assert_eq!(entry["embeddedInLinuxCodexBinary"], true, "{v}");
        assert_eq!(
            entry["fulcioWorkflowRef"],
            format!("https://github.com/openai/codex/.github/workflows/rust-release.yml@refs/tags/rust-v{v}")
        );
        assert!(entry["rekorLogIndex"].as_u64().is_some(), "{v}");
    }
    assert_eq!(
        bwrap["0.166.0"]["sha256"], bwrap["0.167.0"]["sha256"],
        "rustc 1.95.0 builds share identical bwrap bytes"
    );
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
fn protocol_byte_identity_and_classified_deltas_are_mutation_sensitive() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["route"], "codex.exec");
    assert_eq!(protocol["selectedMappedSubsetUnchanged"], true);
    assert_eq!(protocol["noPrompt"], true);
    assert_eq!(protocol["noLiveSession"], true);
    assert_eq!(protocol["noHostMutation"], true);
    assert_eq!(protocol["downloadedBinariesExecuted"], false);

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
    assert_eq!(deltas.len(), 10);
    for key in [
        "codex-rs/exec/src/cli.rs",
        "codex-rs/exec/src/event_processor_with_jsonl_output.rs",
        "codex-rs/exec/src/event_processor.rs",
        "codex-rs/exec/src/lib.rs",
        "codex-rs/linux-sandbox/src/bwrap.rs",
        "codex-rs/linux-sandbox/src/linux_run_main.rs",
        "codex-rs/linux-sandbox/src/launcher.rs",
        "codex-rs/sandboxing/src/bwrap.rs",
        "codex-rs/sandboxing/src/seatbelt.rs",
        "bin/codex.js",
    ] {
        assert!(deltas.contains_key(key), "missing delta {key}");
    }

    assert_eq!(
        protocol["boundedUnmappedAdditions"],
        serde_json::json!([
            "--remote",
            "--remote-auth-token-env",
            "--trust",
            "codex-rs/exec/src/remote.rs",
            "codex-rs/linux-sandbox/src/managed_tcp.rs",
            "codex-rs/linux-sandbox/src/tcp_namespace.rs",
            "codex-rs/linux-sandbox/src/tcp_net_setup.rs"
        ])
    );

    let source = json(SOURCE);
    assert_eq!(source["inventoryDigests"]["0.164.0"]["fileCount"], 9438);
    assert_eq!(
        source["inventoryDigests"]["0.165.0"]["digest"],
        "c39481bcf4c96bb514d1fe54047c78e451a66067d0e833874287f6b07f8a3eb0"
    );
    assert_eq!(source["inventoryDigests"]["0.171.0"]["fileCount"], 10460);
    assert_eq!(
        source["inventoryDigests"]["0.171.0"]["digest"],
        "7e77e00b941d589c7b1b20e054641fbf317cc385487e82c6c9b6b698697459ae"
    );
    assert_eq!(source["hopCounts"].as_object().expect("hopCounts").len(), 10);

    let dist = json(DIST);
    assert_eq!(
        dist["artifactHops"].as_object().expect("artifactHops").len(),
        10
    );
    assert_eq!(dist["packageFileCounts"]["0.171.0"]["darwinArm64"], 43);
    assert_eq!(dist["packageFileCounts"]["0.171.0"]["linuxX64"], 45);
}
