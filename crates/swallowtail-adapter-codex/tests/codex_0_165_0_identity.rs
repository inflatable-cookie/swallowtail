use std::collections::BTreeMap;

use serde_json::Value;
use swallowtail_adapter_codex::{
    codex_app_server_claim, codex_app_server_lifecycle_claim, codex_exec_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/codex-cli-0.165.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/codex-cli-0.165.0/protocol.json");
const DIST: &str = include_str!("fixtures/codex-cli-0.165.0/dist-inventory.json");
const SOURCE: &str = include_str!("fixtures/codex-cli-0.165.0/source-inventory.json");
const STATUS: &str = include_str!("fixtures/codex-cli-0.165.0/source-name-status.tsv");

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
fn channels_agree_on_0_165_0_and_ignore_alpha() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["package"], "@openai/codex");
    assert_eq!(
        identity["routes"],
        serde_json::json!(["codex.exec", "codex.app-server"])
    );
    assert_eq!(identity["previousCeiling"], "0.164.0");
    assert_eq!(identity["officialLatest"], "0.165.0");
    assert_eq!(identity["channelsAgree"], true);
    assert_eq!(identity["prereleaseIgnored"], "0.166.0-alpha.5");
    assert_eq!(identity["downloadedArtifactsExecuted"], false);
    assert_eq!(identity["platformBinariesDownloaded"], true);
    assert_eq!(identity["hostObservation"], Value::Null);
    assert_eq!(identity["syntheticUnverifiedNewer"], "0.165.1");
    assert_eq!(identity["sourceFileCounts"]["0.164.0"], 9438);
    assert_eq!(identity["sourceFileCounts"]["0.165.0"], 9507);

    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), 2);
    assert_eq!(hops[0]["version"], "0.164.0");
    assert_eq!(hops[0]["role"], "previous-ceiling");
    assert_eq!(
        hops[0]["commit"],
        "6aaac694803004a63c6f393d153c96d22521a6b9"
    );
    assert_eq!(hops[1]["version"], "0.165.0");
    assert_eq!(hops[1]["role"], "official-latest");
    assert_eq!(hops[1]["githubTag"], "rust-v0.165.0");
    assert_eq!(
        hops[1]["commit"],
        "360319d8dee62d87e2d59fc0c21805d79519cfed"
    );
    assert_eq!(
        hops[1]["tagObject"],
        "e374836b2728d9b7aa981c2902649e640a2edaa8"
    );
    assert_eq!(hops[1]["npmPublishedAt"], "2026-10-27T20:14:25.378Z");
    assert_eq!(hops[1]["githubPublishedAt"], "2026-10-27T20:09:38Z");
    assert_sha256(&hops[1]["wrapper"]["sha256"]);
    assert_eq!(
        hops[1]["wrapper"]["sha256"],
        "cb890818146db2401af6962bd1c2b4cb72a1b2b5df99f10ff2b36ae3d1c37f45"
    );
    assert_eq!(
        hops[1]["darwinArm64"]["sha256"],
        "a991b0abcb6b6d2ae4d0221b55dd74afe8b87847044483ec68d290908a5a0f97"
    );
    assert_eq!(
        hops[1]["linuxX64"]["sha256"],
        "f997cd6ef5987126e19cd3c82e847a4a6461270f32f97340c5e99a65fb6f9b8a"
    );
    assert_eq!(
        hops[1]["darwinArm64"]["cliSha256"],
        "a45099154d7fb4a5ed8a56f40966a95deb938e508bab23e6985035c709e9a85a"
    );
    assert_eq!(
        hops[1]["linuxX64"]["cliSha256"],
        "c06a67fd8ac76508dc78bd99ce20c9f317b8d568beda59485f3f4134df9962d5"
    );
    assert_eq!(hops[1]["darwinArm64"]["files"], 43);
    assert_eq!(hops[1]["linuxX64"]["files"], 45);
    assert_eq!(
        hops[1]["darwinArm64"]["provenance"],
        "hashed-unexecuted-tarball"
    );
    assert_eq!(
        hops[1]["linuxX64"]["provenance"],
        "hashed-unexecuted-tarball"
    );
}

#[test]
fn decision_retains_previous_claims_until_gaps_close() {
    let identity = json(IDENTITY);
    let before = &identity["claimAtObservation"];
    assert_eq!(before["exec"]["latest"], "0.164.0");
    assert_eq!(before["appServer"]["latest"], "0.164.0");
    assert_eq!(before["lifecycle"]["latest"], "0.164.0");
    assert_ne!(before["exec"]["behavior"], before["appServer"]["behavior"]);

    let decision = &identity["identityDecision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(decision["raiseLatestQualifiedTo"], Value::Null);
    assert_eq!(decision["flattenClaims"], false);
    assert_eq!(decision["newMilestone"], false);
    assert_eq!(decision["newPublicOperation"], false);
    assert_eq!(decision["keepClaimIds"], true);
    assert_eq!(decision["execBehavior"], "codex.exec.jsonl-v1");
    assert_eq!(
        decision["appServerBehavior"],
        "codex.app-server.v2.protected-aws-workspace-roots"
    );
    assert_eq!(
        decision["lifecycleBehavior"],
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
    );

    {
        let point = "0.164.0";
        let exec = codex_exec_claim().assess(&version(point));
        let InterfaceCompatibilityAssessment::Qualified(matched) = exec else {
            panic!("{point} must be qualified exec");
        };
        assert_eq!(matched.behavior_revision().as_str(), "codex.exec.jsonl-v1");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);

        let app = codex_app_server_claim().assess(&version(point));
        let InterfaceCompatibilityAssessment::Qualified(matched) = app else {
            panic!("{point} must be qualified app-server");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "codex.app-server.v2.protected-aws-workspace-roots"
        );

        let lifecycle = codex_app_server_lifecycle_claim().assess(&version(point));
        let InterfaceCompatibilityAssessment::Qualified(matched) = lifecycle else {
            panic!("{point} must be qualified lifecycle");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
        );
    }

    assert_eq!(
        codex_exec_claim().assess(&version("0.155.2")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_exec_claim().assess(&version("0.165.0"))
    else {
        panic!("0.165.0 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.164.0");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_app_server_claim().assess(&version("0.165.0"))
    else {
        panic!("app-server 0.165.0 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.164.0");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_app_server_lifecycle_claim().assess(&version("0.165.0"))
    else {
        panic!("lifecycle 0.165.0 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.164.0");
}

#[test]
fn selected_files_and_dispatch_are_classified() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selectedMappedSubsetUnchanged"], false);
    assert_eq!(protocol["noPrompt"], true);
    assert_eq!(protocol["noLiveSession"], true);
    assert_eq!(protocol["downloadedBinariesExecuted"], false);
    let identical = protocol["byteIdenticalThroughHop"]
        .as_object()
        .expect("identical files");
    assert_eq!(identical.len(), 15);
    assert_eq!(
        identical["codex-rs/exec/src/cli.rs"],
        "66643e8b3d5964b70cb8f231fb309312376272fb3ea87bcb6849f44cf112f8f6"
    );
    assert_eq!(
        identical["codex-rs/linux-sandbox/src/bwrap.rs"],
        "69a23ea16d525de5938c1fd47561c90d6047eb58f9864ca47749d696e5d9fa76"
    );
    assert_eq!(
        identical["codex-rs/app-server-protocol/src/protocol/v2/model.rs"],
        "a25e1cf1aaf5350b593ebc0ecc811e4f24fa5da3f73abb29ce380336bff599d9"
    );

    let deltas = protocol["classifiedDeltas"].as_array().expect("deltas");
    let classes: BTreeMap<&str, &str> = deltas
        .iter()
        .map(|delta| {
            (
                delta["path"].as_str().expect("path"),
                delta["class"].as_str().expect("class"),
            )
        })
        .collect();
    assert_eq!(
        classes["codex-rs/app-server/src/request_processors/thread_delete.rs"],
        "selected-lifecycle-guard-for-unmapped-attached-threads"
    );
    assert_eq!(
        classes["codex-rs/app-server/src/message_processor.rs"],
        "bounded-new-dispatch"
    );
    assert_eq!(
        classes["codex-rs/app-server-protocol/src/protocol/v2/thread.rs"],
        "additive-optional"
    );
    assert_eq!(
        classes["codex-rs/linux-sandbox/src/bwrap.rs"],
        "byte-identical"
    );
    assert_eq!(deltas.len(), 18);
    for path in [
        "codex-rs/thread-store/src/local/delete_thread.rs",
        "codex-rs/rollout/src/rollout_reference_index.rs",
        "codex-rs/thread-store/src/store.rs",
        "codex-rs/thread-store/src/local/list_threads.rs",
        "codex-rs/state/migrations/0062_threads_attached_parent.sql",
    ] {
        assert_eq!(classes[path], "selected-lifecycle-attached-child");
    }
    assert!(protocol["handlerTraces"]["threadDelete"]["externalParentLimitation"].is_string());
    assert!(protocol["execReachability"]["linuxSandboxHelper"]["provenanceGap"].is_string());

    let delete =
        &protocol["contentSha256"]["codex-rs/app-server/src/request_processors/thread_delete.rs"];
    assert_eq!(
        delete["0.164.0"],
        "44391901784847c1b62463739df39fdac408d78e300c0a58f39c2f7804400751"
    );
    assert_eq!(
        delete["0.165.0"],
        "5090073a863a47189dba2ff697b3a0cfd39de9d1797cd3a6e1b84039753b1275"
    );
    assert_ne!(delete["0.164.0"], delete["0.165.0"]);

    let dispatch = &protocol["handlerTraces"]["messageProcessorDispatch"];
    assert_eq!(
        dispatch["newUnmappedArms"]["0.165.0"],
        serde_json::json!(["ThreadMemoryRead"])
    );
    assert_eq!(
        dispatch["archiveDeleteUnarchiveStillCall"],
        "thread_processor"
    );
    for method in [
        "Initialize",
        "ConfigRead",
        "ModelList",
        "ThreadStart",
        "TurnStart",
        "ThreadArchive",
        "ThreadDelete",
        "ThreadUnarchive",
    ] {
        assert!(
            dispatch["selectedMethodArmsRemain"]
                .as_array()
                .unwrap()
                .iter()
                .any(|arm| arm.as_str() == Some(method)),
            "{method}"
        );
    }
    assert_eq!(
        protocol["handlerTraces"]["threadDelete"]["adapterSendsAttachToSource"],
        false
    );
    assert_eq!(
        protocol["schemaBoundaries"]["threadDeleteResponseByteIdentical"],
        true
    );
    assert_eq!(
        protocol["execReachability"]["selectedArgv"],
        serde_json::json!(["exec", "--json", "--sandbox", "read-only"])
    );
    let bwrap = &protocol["execReachability"]["linuxSandboxHelper"]["shippedBwrapSha256"];
    assert_ne!(bwrap["0.164.0"], bwrap["0.165.0"]);
    assert_eq!(
        protocol["execReachability"]["linuxSandboxHelper"]["bwrapSourceByteIdentical"],
        true
    );
}

#[test]
fn source_and_artifact_ledgers_are_mutation_sensitive() {
    let source = json(SOURCE);
    assert_eq!(
        source["inventoryDigests"]["0.165.0"]["digest"],
        "c39481bcf4c96bb514d1fe54047c78e451a66067d0e833874287f6b07f8a3eb0"
    );
    assert_eq!(source["inventoryDigests"]["0.165.0"]["fileCount"], 9507);
    assert_eq!(source["hopCounts"]["0.164.0->0.165.0"]["added"], 80);
    assert_eq!(source["hopCounts"]["0.164.0->0.165.0"]["removed"], 11);
    assert_eq!(source["hopCounts"]["0.164.0->0.165.0"]["changed"], 594);
    assert_eq!(source["hopCounts"]["0.164.0->0.165.0"]["identical"], 8833);

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for line in STATUS.lines().skip(1).filter(|line| !line.is_empty()) {
        let mut parts = line.split('\t');
        let hop = parts.next().expect("hop");
        let status = parts.next().expect("status");
        let path = parts.next().expect("path");
        assert!(parts.next().is_none(), "{line}");
        assert_eq!(hop, "0.164.0-0.165.0");
        assert!(matches!(status, "A" | "D" | "M"), "{status}");
        assert!(!path.is_empty());
        *counts.entry(status).or_insert(0) += 1;
    }
    assert_eq!(counts["A"], 80);
    assert_eq!(counts["D"], 11);
    assert_eq!(counts["M"], 594);
    assert!(STATUS.contains(
        "0.164.0-0.165.0\tM\tcodex-rs/app-server/src/request_processors/thread_delete.rs"
    ));
    assert!(STATUS.contains(
        "0.164.0-0.165.0\tA\tcodex-rs/app-server/src/request_processors/thread_memory.rs"
    ));

    let dist = json(DIST);
    assert_eq!(
        dist["wrapperCodexJsSha256"],
        "61b0194f3bb6534439c8d26a3ed57d0805f84b884588b761795323eeb92fcf70"
    );
    assert_eq!(dist["downloadedBinariesExecuted"], false);
    let hop = &dist["artifactHops"]["0.164.0->0.165.0"];
    assert_eq!(
        hop["wrapper"]["changed"],
        serde_json::json!(["package.json"])
    );
    assert_eq!(hop["wrapper"]["added"], serde_json::json!([]));
    assert_eq!(hop["wrapper"]["removed"], serde_json::json!([]));
    assert!(
        hop["wrapper"]["identical"]
            .as_array()
            .unwrap()
            .iter()
            .any(|path| path.as_str() == Some("bin/codex.js"))
    );
    assert!(hop["linux-x64"]["changed"].as_array().unwrap().iter().any(
        |path| path.as_str() == Some("vendor/x86_64-unknown-linux-musl/codex-resources/bwrap")
    ));
    assert!(
        hop["linux-x64"]["identical"]
            .as_array()
            .unwrap()
            .iter()
            .any(|path| path.as_str() == Some("vendor/x86_64-unknown-linux-musl/codex-path/rg"))
    );
    assert_eq!(
        dist["cliSha256"]["0.165.0"]["linux-x64"],
        "c06a67fd8ac76508dc78bd99ce20c9f317b8d568beda59485f3f4134df9962d5"
    );

    let inventory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/codex-cli-0.165.0/artifact-inventory");
    for (name, count) in [
        ("0.165.0-wrapper.sha256", 3),
        ("0.165.0-darwin-arm64.sha256", 43),
        ("0.165.0-linux-x64.sha256", 45),
    ] {
        let listing = std::fs::read_to_string(inventory.join(name)).expect(name);
        let lines = listing.lines().filter(|line| !line.is_empty()).count();
        assert_eq!(lines, count, "{name}");
    }
}
