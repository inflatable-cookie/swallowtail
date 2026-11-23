use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_codex::{codex_app_server_claim, codex_app_server_lifecycle_claim};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/codex-app-server-0.171.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/codex-app-server-0.171.0/protocol.json");
const SOURCE: &str = include_str!("fixtures/codex-app-server-0.171.0/source-inventory.json");

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
fn app_server_0_171_0_identity_and_reused_bwrap_provenance_are_frozen() {
    assert_eq!(
        sha256_hex(IDENTITY.as_bytes()),
        "ba5b38342ead282366060a8adab2bba15b5b1008fafc32c9129d228d5eda52fa"
    );

    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "codex.cli");
    assert_eq!(identity["package"], "@openai/codex");
    assert_eq!(identity["routes"], serde_json::json!(["codex.app-server"]));
    assert_eq!(
        identity["siblingRoutesAlreadyQualified"],
        serde_json::json!(["codex.exec"])
    );
    assert_eq!(identity["previousCeiling"], "0.164.0");
    assert_eq!(identity["officialLatest"], "0.171.0");
    assert_eq!(identity["channelsAgree"], true);
    assert_eq!(identity["prereleaseIgnored"], "0.173.0-alpha.4");
    assert_eq!(identity["downloadedArtifactsExecuted"], false);
    assert_eq!(identity["syntheticUnverifiedNewer"], "0.171.1");

    let decision = &identity["identityDecision"];
    assert_eq!(decision["appServerShape"], "compatible-extension");
    assert_eq!(decision["lifecycleShape"], "private-milestone");
    assert_eq!(decision["raiseLatestQualifiedTo"], "0.171.0");
    assert_eq!(decision["keepClaimIds"], true);
    assert_eq!(decision["newPublicOperation"], false);
    assert_eq!(decision["consumerVisibleNarrowing"], false);
    assert_eq!(
        decision["appServerBehavior"],
        "codex.app-server.v2.protected-aws-workspace-roots"
    );
    assert_eq!(
        decision["lifecycle0_140_to_0_165Behavior"],
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
    );
    assert_eq!(
        decision["lifecycle0_166_to_0_171Behavior"],
        "codex.app-server.lifecycle.v1.best-effort-descendant-restore"
    );

    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), EXPECTED_VERSIONS.len());
    for (hop, expected) in hops.iter().zip(EXPECTED_VERSIONS) {
        assert_eq!(hop["version"], expected);
        assert_eq!(hop["githubTag"], format!("rust-v{expected}"));
        assert_sha256(&hop["wrapper"]["sha256"]);
        assert_sha256(&hop["darwinArm64"]["cliSha256"]);
        assert_sha256(&hop["linuxX64"]["bwrapSha256"]);
    }
}

#[test]
fn app_server_and_lifecycle_claims_qualify_through_0_171_0_with_0_166_0_restore_milestone() {
    let app_claim = codex_app_server_claim();
    for v in EXPECTED_VERSIONS {
        let InterfaceCompatibilityAssessment::Qualified(matched) = app_claim.assess(&version(v))
        else {
            panic!("codex.app-server {v} must be qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "codex.app-server.v2.protected-aws-workspace-roots"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(app_newer) =
        app_claim.assess(&version("0.171.1"))
    else {
        panic!("0.171.1 must remain unverified newer for codex.app-server");
    };
    assert_eq!(app_newer.latest_qualified().as_str(), "0.171.0");
    assert_eq!(
        app_newer.behavior_revision().as_str(),
        "codex.app-server.v2.protected-aws-workspace-roots"
    );

    let lifecycle_claim = codex_app_server_lifecycle_claim();
    for v in ["0.164.0", "0.165.0"] {
        let InterfaceCompatibilityAssessment::Qualified(matched) =
            lifecycle_claim.assess(&version(v))
        else {
            panic!("codex.app-server lifecycle {v} must be qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }
    for v in [
        "0.166.0", "0.167.0", "0.167.1", "0.167.2", "0.168.0", "0.169.0", "0.169.1", "0.170.0",
        "0.171.0",
    ] {
        let InterfaceCompatibilityAssessment::Qualified(matched) =
            lifecycle_claim.assess(&version(v))
        else {
            panic!("codex.app-server lifecycle {v} must be qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "codex.app-server.lifecycle.v1.best-effort-descendant-restore"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(lifecycle_newer) =
        lifecycle_claim.assess(&version("0.171.1"))
    else {
        panic!("0.171.1 must remain unverified newer for codex.app-server lifecycle");
    };
    assert_eq!(lifecycle_newer.latest_qualified().as_str(), "0.171.0");
    assert_eq!(
        lifecycle_newer.behavior_revision().as_str(),
        "codex.app-server.lifecycle.v1.best-effort-descendant-restore"
    );

    for gap in [
        "0.82.0", "0.83.0", "0.108.0", "0.109.0", "0.149.2", "0.150.2", "0.151.1", "0.152.2",
        "0.154.1", "0.155.2",
    ] {
        assert_eq!(
            app_claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "retained app-server gap {gap} stays incompatible"
        );
        assert_eq!(
            lifecycle_claim.assess(&version(gap)),
            InterfaceCompatibilityAssessment::Incompatible,
            "retained lifecycle gap {gap} stays incompatible"
        );
    }
}

#[test]
fn app_server_protocol_and_source_inventory_are_mutation_sensitive() {
    assert_eq!(
        sha256_hex(PROTOCOL.as_bytes()),
        "a7094807089ddb6aaf94c0e4b609bb155a3753888474f2fcb9cd76e4b06c039f"
    );
    assert_eq!(
        sha256_hex(SOURCE.as_bytes()),
        "61066df650584f97cc7c39af8cbee271064d7792f5becbfebff6c108ef139305"
    );

    let protocol = json(PROTOCOL);
    assert_eq!(protocol["route"], "codex.app-server");
    assert_eq!(
        protocol["selectedInvocation"],
        serde_json::json!(["app-server", "--listen", "stdio://"])
    );
    assert_eq!(protocol["selectedMappedWireShapeUnchanged"], true);
    assert_eq!(protocol["noPrompt"], true);
    assert_eq!(protocol["noLiveSession"], true);
    assert_eq!(protocol["noHostMutation"], true);
    assert_eq!(protocol["downloadedBinariesExecuted"], false);

    let bwrap = &protocol["bwrapGateResolution"];
    assert_eq!(bwrap["research463GateClosed"], true);
    assert_eq!(bwrap["reusedFromResearch487"], true);
    assert_eq!(
        bwrap["protectedMetadataPathNames"],
        serde_json::json!([".git", ".agents", ".codex", ".aws"])
    );

    let attached = &protocol["attachedChildLifecycleResolution"];
    assert_eq!(attached["research467Baseline"], "0.165.0");
    assert_eq!(
        attached["threadDeleteProcessorByteIdenticalThrough0_171_0"],
        true
    );
    assert_eq!(
        attached["rolloutReferenceIndexByteIdenticalThrough0_171_0"],
        true
    );
    assert_eq!(attached["historyRevisionByteIdenticalThrough0_171_0"], true);
    assert_eq!(
        attached["deleteStoreAddsDiagnosticLogsFailClosedGuard"],
        true
    );
    assert_eq!(attached["ordinaryListKeepsExcludeAttachedThreads"], true);
    assert_eq!(attached["adapterSendsNoAttachedRelationFilter"], true);
    assert_eq!(attached["refusalMapsToFailedBeforeEffect"], true);
    assert_eq!(
        attached["deleteAffectedScopeRemainsProviderDefinedDescendants"],
        true
    );

    let restore = &protocol["bestEffortDescendantRestoreMilestone"];
    assert_eq!(restore["introducedIn"], "0.166.0");
    assert_eq!(
        restore["behaviorRevision"],
        "codex.app-server.lifecycle.v1.best-effort-descendant-restore"
    );
    assert_eq!(restore["restoreAffectedScopeRemainsTargetOnly"], true);
    assert_eq!(restore["wireRequestAndResponseUnchanged"], true);
    assert_eq!(restore["emitsThreadUnarchivedPerRestoredId"], true);

    let resume = &protocol["paginatedResumeAndAccountCheck"];
    assert_eq!(
        resume["resumeModelContextReuseEquivalentOnSelectedPath"],
        true
    );
    assert_eq!(
        resume["cliResumeAccountCheckClients"],
        serde_json::json!(["codex-tui", "codex_exec"])
    );
    assert_eq!(resume["swallowtailClientNameExempt"], true);

    let source = json(SOURCE);
    let feeding = source["selectedFeedingFiles"]
        .as_array()
        .expect("selectedFeedingFiles");
    assert_eq!(feeding.len(), 37);
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
    assert!(
        per_hop["0.165.0->0.166.0"]
            .as_array()
            .expect("0.165->0.166")
            .iter()
            .any(|p| p == "codex-rs/thread-store/src/local/unarchive_thread.rs")
    );
}
