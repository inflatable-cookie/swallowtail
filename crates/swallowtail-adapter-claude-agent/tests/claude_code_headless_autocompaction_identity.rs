use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const AUTOCOMPACTION: &str =
    include_str!("fixtures/claude-code-2.1.241/headless-autocompaction.json");

#[test]
fn autocompaction_evidence_fixture_is_secret_free_and_records_an_empty_deliver_now_set() {
    let evidence: Value = serde_json::from_str(AUTOCOMPACTION)
        .expect("Claude Code autocompaction corpus is valid JSON");

    assert_eq!(evidence["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(evidence["provider_prompt_sent"], false);
    assert_eq!(evidence["credentials_used"], false);
    assert_eq!(evidence["host_install_changed"], false);
    assert_eq!(evidence["ambient_configuration_mutated"], false);

    // Native meaning is a compact window, never context size or output limit.
    assert_eq!(evidence["native_meaning"]["is_context_size"], false);
    assert_eq!(evidence["native_meaning"]["is_output_token_limit"], false);
    assert_eq!(evidence["native_meaning"]["is_session_continuity"], false);
    assert_eq!(evidence["native_meaning"]["is_enablement"], false);
    assert_eq!(evidence["native_meaning"]["token_floor"], 100_000);
    assert_eq!(evidence["native_meaning"]["token_ceiling"], 1_000_000);

    // Membership starts at 2.1.221; 2.1.220 rejects the flag as unknown.
    assert_eq!(
        evidence["option_declaration"]["first_published_version"],
        "2.1.221"
    );
    assert_eq!(evidence["option_declaration"]["absent_at"], "2.1.220");
    assert_eq!(evidence["option_declaration"]["hidden"], false);

    let accepted = evidence["parser_domain"]["accepted"]
        .as_array()
        .expect("accepted values are an array");
    for value in ["auto", "500k", "200", "1m", "1e5"] {
        assert!(accepted.iter().any(|entry| entry == value));
    }
    let rejected = evidence["parser_domain"]["rejected"]
        .as_array()
        .expect("rejected values are an array");
    for value in ["99k", "bogus", "0", "Infinity", "off"] {
        assert!(rejected.iter().any(|entry| entry == value));
    }

    // Ambient environment defeats operation-private argv precedence.
    assert_eq!(evidence["precedence"]["environment_overrides_flag"], true);
    assert_eq!(
        evidence["precedence"]["operation_private_precedence"],
        false
    );
    assert_eq!(
        evidence["enablement_gates"]["can_nullify_selected_window"],
        true
    );
    assert_eq!(
        evidence["effective_state"]["prompt_free_stream_field_observed"],
        false
    );
    assert_eq!(
        evidence["effective_state"]["compaction_observation_requires_provider_prompt"],
        true
    );

    // Omission stays exact and claim-free.
    assert_eq!(evidence["omission"]["preserves_exact_prior_argv"], true);
    assert_eq!(evidence["omission"]["claims_default_window"], false);
    assert_eq!(evidence["omission"]["claims_context_size"], false);

    let probed = evidence["probed_versions"]
        .as_object()
        .expect("probed versions are an object");
    assert_eq!(probed.len(), 21);
    assert!(!probed.contains_key("2.1.230"));
    assert!(probed.contains_key(CLAUDE_CODE_HEADLESS_BASELINE_VERSION));
    assert!(probed.contains_key("2.1.241"));
    assert!(!probed.contains_key(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION));

    let claim = claude_code_headless_claim();
    for (probed_version, row) in probed {
        assert!(matches!(
            claim.assess(&version(probed_version)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
        assert_eq!(
            row["darwin_arm64_binary_sha256"]
                .as_str()
                .expect("digest is a string")
                .len(),
            64
        );
        if probed_version == "2.1.220" {
            assert_eq!(row["option_present"], false);
            assert_eq!(row["doctor_unknown_option_on_autocompact"], true);
            assert_eq!(row["env_overrides_flag_in_source"], false);
        } else {
            assert_eq!(row["option_present"], true);
            assert_eq!(row["help_advertises_autocompact"], true);
            assert_eq!(row["doctor_accepts_auto"], true);
            assert_eq!(row["doctor_accepts_500k"], true);
            assert_eq!(row["doctor_rejects_bogus"], true);
            assert_eq!(row["env_overrides_flag_in_source"], true);
        }
    }

    assert_eq!(
        evidence["unpublished_in_range_versions"],
        serde_json::json!(["2.1.230"])
    );
    assert_eq!(
        evidence["disposition"]["deliver_now_rows"]
            .as_array()
            .expect("deliver-now rows are an array")
            .len(),
        0
    );
    assert_eq!(evidence["disposition"]["deliver_now_count"], 0);
    assert_eq!(
        evidence["disposition"]["production_binding"],
        "not authorized"
    );
    assert_eq!(evidence["disposition"]["existing_route"], "unchanged");
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
