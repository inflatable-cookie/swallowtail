use serde_json::Value;
use swallowtail_adapter_codex::{
    codex_app_server_claim, codex_app_server_lifecycle_claim, codex_exec_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/codex-cli-0.164.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/codex-cli-0.164.0/protocol.json");

const HOPS: &[&str] = &["0.161.0", "0.162.0", "0.162.1", "0.163.0", "0.164.0"];

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen identity JSON is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

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
    assert_eq!(identity["platformBinariesDownloaded"], false);
    assert_eq!(identity["syntheticUnverifiedNewer"], "0.164.1");

    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), HOPS.len());
    for (hop, expected) in hops.iter().zip(HOPS) {
        assert_eq!(hop["version"], *expected);
        assert_eq!(hop["githubTag"], format!("rust-v{expected}"));
        assert_eq!(hop["tagObject"].as_str().unwrap().len(), 40);
        assert_eq!(hop["commit"].as_str().unwrap().len(), 40);
        assert_eq!(hop["wrapper"]["sha256"].as_str().unwrap().len(), 64);
        assert!(
            hop["wrapper"]["integrity"]
                .as_str()
                .unwrap()
                .starts_with("sha512-")
        );
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
    assert_eq!(identity["sourceFileCounts"]["0.161.0"], 8899);
    assert_eq!(identity["sourceFileCounts"]["0.164.0"], 9438);
}

#[test]
fn selected_mapped_files_stay_byte_identical_and_deltas_stay_classified() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selectedMappedSubsetUnchanged"], true);
    assert_eq!(protocol["noPrompt"], true);
    assert_eq!(protocol["noLiveSession"], true);
    let identical = protocol["byteIdenticalThroughAllHops"]
        .as_object()
        .expect("identical files");
    assert_eq!(identical.len(), 9);
    for (path, digest) in identical {
        let digest = digest.as_str().expect("digest");
        assert_eq!(digest.len(), 64, "{path}");
        assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
    }
    assert_eq!(
        identical["codex-rs/exec/src/cli.rs"],
        "66643e8b3d5964b70cb8f231fb309312376272fb3ea87bcb6849f44cf112f8f6"
    );
    assert_eq!(
        identical["codex-rs/app-server-protocol/src/protocol/v2/permissions.rs"],
        "fb60ae2f9babce22ca326b457c22b41e6fa24f71ad05429a8921074cc0ca5598"
    );
    let deltas = protocol["classifiedDeltas"].as_array().expect("deltas");
    assert_eq!(deltas.len(), 8);
    for delta in deltas {
        assert!(matches!(
            delta["class"].as_str(),
            Some("unselected" | "additive-optional")
        ));
        assert!(!delta["reason"].as_str().unwrap().is_empty());
    }
}

#[test]
fn decision_extends_both_claims_without_flattening() {
    let identity = json(IDENTITY);
    let before = &identity["claimAtObservation"];
    assert_eq!(before["exec"]["id"], "codex.exec.cli-window-2");
    assert_eq!(before["appServer"]["id"], "codex.app-server.cli-window-2");
    assert_eq!(before["exec"]["latest"], "0.161.0");
    assert_eq!(before["appServer"]["latest"], "0.161.0");
    assert_ne!(
        before["exec"]["behavior"],
        before["appServer"]["behavior"]
    );

    let decision = &identity["identityDecision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["raiseLatestQualifiedTo"], "0.164.0");
    assert_eq!(decision["flattenClaims"], false);
    assert_eq!(decision["newMilestone"], false);
    assert_eq!(decision["newPublicOperation"], false);
    assert_eq!(decision["execBehavior"], "codex.exec.jsonl-v1");
    assert_eq!(
        decision["appServerBehavior"],
        "codex.app-server.v2.protected-aws-workspace-roots"
    );
    assert_eq!(
        decision["lifecycleBehavior"],
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
    );

    for point in ["0.162.0", "0.162.1", "0.163.0", "0.164.0"] {
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
    assert_eq!(
        codex_app_server_claim().assess(&version("0.155.2")),
        InterfaceCompatibilityAssessment::Incompatible
    );
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_exec_claim().assess(&version("0.164.1"))
    else {
        panic!("0.164.1 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.164.0");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_app_server_claim().assess(&version("0.164.1"))
    else {
        panic!("app-server 0.164.1 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.164.0");
}
