use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const MAXIMUM_TURNS: &str =
    include_str!("fixtures/claude-code-2.1.241/headless-maximum-turns.json");

#[test]
fn maximum_turns_evidence_fixture_is_secret_free_and_covers_every_published_version() {
    let evidence: Value = serde_json::from_str(MAXIMUM_TURNS)
        .expect("Claude Code maximum-turns corpus is valid JSON");

    assert_eq!(evidence["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(evidence["provider_prompt_sent"], false);
    assert_eq!(evidence["credentials_used"], false);
    assert_eq!(evidence["host_install_changed"], false);
    assert_eq!(evidence["ambient_configuration_mutated"], false);

    // Help omission is a `hideHelp()` call, not missing support.
    assert_eq!(evidence["help"]["max_turns_advertised_in_help"], false);
    assert_eq!(evidence["option_declaration"]["hidden"], true);
    assert_eq!(evidence["option_declaration"]["alias"], Value::Null);
    assert_eq!(evidence["option_declaration"]["short_form"], Value::Null);

    // The native parser is far wider than the documented positive domain.
    let accepted = evidence["parser_domain"]["accepted"]
        .as_array()
        .expect("accepted values are an array");
    for value in ["0", "-1", "3.5", "1e3", "0x3", "", "Infinity"] {
        assert!(accepted.iter().any(|entry| entry == value));
    }
    let rejected = evidence["parser_domain"]["rejected"]
        .as_array()
        .expect("rejected values are an array");
    for value in ["abc", "NaN"] {
        assert!(rejected.iter().any(|entry| entry == value));
    }

    // Explicit argv precedence needs no environment inspection or mutation.
    assert_eq!(
        evidence["environment"]["argv_precedence_holds_for_argv_values_the_env_would_reject"],
        true
    );
    assert_eq!(evidence["environment"]["competing_settings_key"], false);
    assert_eq!(
        evidence["environment"]["omission_passes_ambient_value_through"],
        true
    );
    assert_eq!(
        evidence["environment"]["environment_inspected_or_mutated_by_swallowtail"],
        false
    );

    // A counted turn is one tool-use round trip, not a portable budget unit.
    assert_eq!(evidence["counted_turn"]["unit"], "one tool-use round trip");
    for portable in [
        "counts_output_tokens",
        "counts_tool_calls",
        "counts_provider_requests",
        "counts_retries",
        "counts_wall_time_or_cost",
    ] {
        assert_eq!(evidence["counted_turn"][portable], false);
    }

    // Enforcement is a truthiness guard, so zero must never be selectable.
    assert_eq!(evidence["enforcement"]["guard_is_truthiness_test"], true);
    assert_eq!(evidence["enforcement"]["zero_disables_enforcement"], true);
    assert_eq!(evidence["enforcement"]["clamped_or_rounded"], false);
    assert_eq!(
        evidence["enforcement"]["overridable_by_model_tools_hooks_settings_resume_or_provider"],
        false
    );

    // Native bound reached is a provider failure with no output, never success.
    assert_eq!(evidence["terminal"]["result_subtype"], "error_max_turns");
    assert_eq!(evidence["terminal"]["result_field_present"], false);
    assert_eq!(evidence["terminal"]["process_exit_code"], 1);
    assert_eq!(
        evidence["current_driver_mapping"]["terminal_status"],
        "ProviderFailed"
    );
    assert_eq!(
        evidence["current_driver_mapping"]["mapped_to_completion"],
        false
    );
    assert_eq!(
        evidence["current_driver_mapping"]["new_diagnostic_admitted"],
        false
    );

    // The fixture's probed set is the deliver-now version set. The route's own
    // qualified window is deliberately wider: it spans a semantic range that
    // contains the never-published `2.1.230` and permits later stable points.
    assert_eq!(
        evidence["binding"]["version_gate"],
        "exact Research 226 probed set only; UnverifiedNewer and unpublished in-range points reject before process work"
    );

    // Every published point in the qualified window was probed and agreed.
    let probed = evidence["probed_versions"]
        .as_object()
        .expect("probed versions are an object");
    assert_eq!(probed.len(), 21);
    assert!(!probed.contains_key("2.1.230"));
    for boundary in [
        CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
        CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
    ] {
        assert!(probed.contains_key(boundary));
    }
    let claim = claude_code_headless_claim();
    for (probed_version, row) in probed {
        assert!(matches!(
            claim.assess(&version(probed_version)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
        assert_eq!(row["help_advertises_max_turns"], false);
        for required in [
            "hidden_option_declaration",
            "argv_precedence_short_circuit",
            "loop_guard_truthiness",
            "error_max_turns_result",
            "exit_1_on_error_result",
            "parser_accepts_positive",
            "parser_accepts_zero_negative_fractional",
            "parser_rejects_non_numeric",
        ] {
            assert_eq!(row[required], true);
        }
        assert_eq!(
            row["darwin_arm64_binary_sha256"]
                .as_str()
                .expect("digest is a string")
                .len(),
            64
        );
    }

    // The binding stays closed, adapter-local, and version-gated.
    assert_eq!(evidence["binding"]["domain"], "positive 32-bit integer");
    assert_eq!(evidence["binding"]["public_low_level_setter"], false);
    assert_eq!(
        evidence["binding"]["low_level_dispatch_revalidates_plan_version"],
        true
    );
    assert_eq!(
        evidence["probed_versions_are_the_admitted_version_set"],
        true
    );
    assert_eq!(
        evidence["unpublished_in_range_versions"],
        serde_json::json!(["2.1.230"])
    );
    assert_eq!(evidence["binding"]["raw_escape_hatch"], false);
    assert_eq!(evidence["binding"]["portable_capability"], false);
    assert_eq!(
        evidence["binding"]["omission_preserves_exact_prior_argv"],
        true
    );
    assert_eq!(
        evidence["binding"]["omission_claims_unlimited_execution"],
        false
    );
    assert_eq!(
        evidence["disposition"]["deliver_now_rows"]
            .as_array()
            .expect("deliver-now rows are an array")
            .len(),
        2
    );
    assert_eq!(evidence["disposition"]["cards_220_221"], "unblocked");
    assert_eq!(evidence["disposition"]["existing_route"], "unchanged");
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
