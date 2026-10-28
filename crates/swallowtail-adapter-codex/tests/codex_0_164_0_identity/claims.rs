use super::support::{IDENTITY, json, version};
use swallowtail_adapter_codex::{
    codex_app_server_claim, codex_app_server_lifecycle_claim, codex_exec_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn decision_extends_both_claims_without_flattening() {
    let identity = json(IDENTITY);
    let before = &identity["claimAtObservation"];
    assert_eq!(before["exec"]["id"], "codex.exec.cli-window-2");
    assert_eq!(before["appServer"]["id"], "codex.app-server.cli-window-2");
    assert_eq!(before["exec"]["latest"], "0.161.0");
    assert_eq!(before["appServer"]["latest"], "0.161.0");
    assert_ne!(before["exec"]["behavior"], before["appServer"]["behavior"]);

    let decision = &identity["identityDecision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["raiseLatestQualifiedTo"], "0.164.0");
    assert_eq!(decision["flattenClaims"], false);
    assert_eq!(decision["newMilestone"], false);
    assert_eq!(decision["newPublicOperation"], false);
    assert_eq!(decision["keepClaimIds"], true);
    assert_eq!(decision["keepBaselines"], true);
    assert_eq!(decision["keepOlderSegments"], true);
    assert_eq!(decision["execBehavior"], "codex.exec.jsonl-v1");
    assert_eq!(
        decision["appServerBehavior"],
        "codex.app-server.v2.protected-aws-workspace-roots"
    );
    assert_eq!(
        decision["lifecycleBehavior"],
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
    );

    for point in ["0.162.0", "0.162.1", "0.163.0", "0.164.0", "0.165.0"] {
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
        codex_exec_claim().assess(&version("0.165.1"))
    else {
        panic!("0.165.1 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.165.0");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_app_server_claim().assess(&version("0.165.1"))
    else {
        panic!("app-server 0.165.1 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.165.0");
    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
        codex_app_server_lifecycle_claim().assess(&version("0.165.1"))
    else {
        panic!("lifecycle 0.165.1 stays unverified newer");
    };
    assert_eq!(newer.latest_qualified().as_str(), "0.165.0");
}
