use super::support::{IDENTITY, PUBLISHED_HOPS, RESPONSE_ONLY, json, strings, version};
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
    assert_eq!(watcher["exact_watcher_version_remains"], "2.1.251");
    assert_eq!(watcher["widen_watcher_help_authorization"], false);
    assert_eq!(watcher["widen_watcher_digest_authorization"], false);
    assert_eq!(watcher["widen_watcher_live_authorization"], false);
    assert_eq!(watcher["copy_watcher_isolation_fixture"], false);
    assert_eq!(watcher["copy_watcher_tool_admission_fixture"], false);
    assert_eq!(watcher["mechanism_change_gate_unchanged"], true);
    assert_eq!(
        identity["claim_at_observation"]["watcher_exact_version"],
        "2.1.251"
    );
    assert_eq!(
        identity["identity_decision"]["widen_watcher_authorization"],
        false
    );
    assert_eq!(identity["identity_decision"]["map_watcher_flags"], false);
    assert_eq!(identity["identity_decision"]["widen_maximum_turns"], false);
}

#[test]
fn unpublished_gaps_and_later_2_1_271_stay_classified() {
    let identity = json(IDENTITY);
    let response_only = json(RESPONSE_ONLY);
    assert_eq!(
        strings(&identity["published_stables_from_previous_ceiling"]),
        PUBLISHED_HOPS
    );
    for (field, value) in [
        ("unpublished_2_1_244", true),
        ("unpublished_2_1_249", true),
        ("unpublished_2_1_253", true),
        ("unpublished_2_1_254", true),
        ("unpublished_2_1_255", true),
        ("unpublished_2_1_256", true),
        ("unpublished_2_1_262", true),
        ("unpublished_2_1_264", true),
    ] {
        assert_eq!(identity[field], value, "{field}");
    }
    assert_eq!(identity["first_unpublished_after_official"], "2.1.271");
    for field in [
        "keep_unpublished_2_1_244_incompatible",
        "keep_unpublished_2_1_249_incompatible",
        "keep_unpublished_2_1_253_incompatible",
        "keep_unpublished_2_1_254_incompatible",
        "keep_unpublished_2_1_255_incompatible",
        "keep_unpublished_2_1_256_incompatible",
        "add_unpublished_2_1_262_gap",
        "add_unpublished_2_1_264_gap",
    ] {
        assert_eq!(identity["identity_decision"][field], true, "{field}");
    }
    assert_eq!(
        identity["identity_decision"]["later_unverified_after_qualification"],
        "2.1.271"
    );
    assert_eq!(
        identity["identity_decision"]["later_unverified_published"],
        false
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_after_qualification"],
        "2.1.271"
    );
    assert_eq!(
        response_only["identity_decision"]["later_unverified_published"],
        false
    );
    assert_eq!(
        identity["claim_at_observation"]["denied_versions"],
        serde_json::json!([
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256",
        ])
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &[
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256",
        ]
    );
}

#[test]
fn identity_names_compatible_extension_without_raising_the_claim() {
    let identity = json(IDENTITY);
    let response_only = json(RESPONSE_ONLY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_headless_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(
        decision["reuse_response_only_behavior_revision"],
        "claude-code.response-only.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.270");
    assert_eq!(decision["keep_headless_baseline"], "2.1.220");
    assert_eq!(decision["keep_response_only_baseline"], "2.1.227");
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_headless_and_response_only"], false);
    assert_eq!(decision["flatten_to_claude_agent_acp"], false);
    assert_eq!(decision["map_restricted"], false);
    assert_eq!(decision["map_watcher_flags"], false);
    assert_eq!(
        strings(&decision["qualified_published_intermediates"]),
        PUBLISHED_HOPS
    );
    assert_eq!(
        response_only["identity_decision"]["reuse_behavior_revision"],
        "claude-code.response-only.stream-json.v1"
    );

    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "2.1.257"
    );
    assert_eq!(
        response_only["claim_at_observation"]["latest_qualified"],
        "2.1.257"
    );
    assert_eq!(CLAUDE_CODE_HEADLESS_BASELINE_VERSION, "2.1.220");
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.257");
    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, "2.1.227");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.257"
    );

    let headless = claude_code_headless_claim();
    assert!(matches!(
        headless.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    for candidate in PUBLISHED_HOPS.iter().chain(["2.1.271"].iter()).copied() {
        assert!(
            matches!(
                headless.assess(&version(candidate)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ),
            "{candidate}"
        );
    }
    let response = claude_code_response_only_claim();
    assert!(matches!(
        response.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        response.assess(&version("2.1.270")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(matches!(
        response.assess(&version("2.1.271")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
