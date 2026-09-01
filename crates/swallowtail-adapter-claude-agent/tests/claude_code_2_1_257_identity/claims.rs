use super::support::{
    FROZEN_2_1_252_HELP_SHA256, IDENTITY, OFFICIAL_2_1_257_HELP_SHA256, RESPONSE_ONLY,
    assert_sha256, json, strings, version,
};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_BASELINE_VERSION, CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
    CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
    claude_code_response_only_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

#[test]
fn watcher_authorization_stays_on_exact_2_1_251() {
    let identity = json(IDENTITY);
    let watcher = &identity["watcher_audit"];
    assert_sha256(&watcher["frozen_help_sha256"], FROZEN_2_1_252_HELP_SHA256);
    assert_sha256(
        &watcher["official_2_1_257_help_sha256"],
        OFFICIAL_2_1_257_HELP_SHA256,
    );
    assert_eq!(watcher["help_digest_unchanged"], false);
    assert_eq!(watcher["exact_watcher_version_remains"], "2.1.251");
    assert_eq!(watcher["widen_watcher_help_authorization"], false);
    assert_eq!(watcher["widen_watcher_digest_authorization"], false);
    assert_eq!(watcher["widen_watcher_live_authorization"], false);
    assert_eq!(watcher["copy_watcher_isolation_fixture"], false);
    assert_eq!(watcher["copy_watcher_tool_admission_fixture"], false);
    assert_eq!(watcher["mechanism_change_gate_unchanged"], true);
    assert_eq!(
        watcher["official_2_1_257_rejected_at_watcher_admission"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["watcher_exact_version"],
        "2.1.251"
    );
    assert_eq!(
        identity["identity_decision"]["widen_watcher_authorization"],
        false
    );
    assert_eq!(identity["identity_decision"]["map_watcher_flags"], false);
}

#[test]
fn unpublished_gaps_and_later_2_1_258_stay_classified() {
    let identity = json(IDENTITY);
    let response_only = json(RESPONSE_ONLY);
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        ["2.1.257"]
    );
    assert_eq!(identity["unpublished_2_1_244"], true);
    assert_eq!(identity["unpublished_2_1_249"], true);
    assert_eq!(identity["unpublished_2_1_253"], true);
    assert_eq!(identity["unpublished_2_1_254"], true);
    assert_eq!(identity["unpublished_2_1_255"], true);
    assert_eq!(identity["unpublished_2_1_256"], true);
    assert_eq!(identity["unpublished_2_1_258"], true);
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_244_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_249_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_253_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_254_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_255_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["keep_unpublished_2_1_256_incompatible"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_after_qualification"],
        "2.1.258"
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_published"],
        false
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_after_qualification"],
        "2.1.258"
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_published"],
        false
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &["2.1.244", "2.1.249"]
    );

    let headless = claude_code_headless_claim();
    assert!(!headless.permits(&version("2.1.244")));
    assert!(!headless.permits(&version("2.1.249")));
    assert!(matches!(
        headless.assess(&version("2.1.253")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        headless.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let response = claude_code_response_only_claim();
    assert!(!response.permits(&version("2.1.244")));
    assert!(!response.permits(&version("2.1.249")));
    assert!(matches!(
        response.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

#[test]
fn identity_names_compatible_extension_without_raising_the_claim() {
    let identity = json(IDENTITY);
    let response_only = json(RESPONSE_ONLY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.257");
    assert_eq!(decision["keep_baseline"], "2.1.220");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["map_system_prompt_snapshot"], false);
    assert_eq!(
        response_only["identity_decision"]["reuse_behavior_revision"],
        "claude-code.response-only.stream-json.v1"
    );
    assert_eq!(
        identity["claim_at_observation"]["deny_list"],
        serde_json::json!(["2.1.244", "2.1.249"])
    );
    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "2.1.252"
    );
    assert_eq!(
        response_only["claim_at_observation"]["latest_qualified"],
        "2.1.252"
    );
    assert_eq!(CLAUDE_CODE_HEADLESS_BASELINE_VERSION, "2.1.220");
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.252");
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, "2.1.227");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.252"
    );

    let headless = claude_code_headless_claim();
    assert!(matches!(
        headless.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        headless.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let response = claude_code_response_only_claim();
    assert!(matches!(
        response.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        response.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
