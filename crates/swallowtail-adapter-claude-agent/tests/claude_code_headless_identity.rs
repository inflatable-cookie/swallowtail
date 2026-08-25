use serde_json::Value;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_HEADLESS_BASELINE_VERSION,
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_binding,
    claude_code_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.241/identity.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.241/protocol.json");
const STRUCTURED_OUTPUT: &str =
    include_str!("fixtures/claude-code-2.1.238/headless-structured-output.json");
const ULTRACODE: &str = include_str!("fixtures/claude-code-2.1.241/headless-ultracode.json");

#[test]
fn identity_and_claim_qualify_2_1_241_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Claude Code 2.1.241 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Code 2.1.241 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(identity["version"], "2.1.241");
    assert_eq!(identity["npm_package"], "@anthropic-ai/claude-code");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-S7DWEmJJAsI5taAUjhKm6soXcFJYIVeTH6Lg9kmp3yntFllCP612hGwZ7thOGh8r7YaRUH9+1jCX5A9QGazsxg=="
    );
    assert_eq!(identity["host"]["not_installed"], true);
    assert_eq!(identity["official_help_byte_identical_to_2_1_238"], true);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["2.1.239", "2.1.240", "2.1.241"])
    );
    assert_eq!(identity["unpublished_2_1_242"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.headless.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.241");
    assert_eq!(decision["keep_baseline"], "2.1.220");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["mix_response_only_axis"], false);
    assert_eq!(decision["flatten_to_claude_agent_acp"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "2.1.242");

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "-p",
        "--output-format",
        "--no-session-persistence",
        "--permission-mode",
        "--tools",
        "--mcp-config",
        "--strict-mcp-config",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_permission_mode"], "plan");
    assert_eq!(protocol["include_partial_messages_selected"], false);
    assert_eq!(protocol["decoder_corpus"], "claude-code-2.1.220");
    assert_eq!(protocol["official_help_byte_identical_to_2_1_238"], true);
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CLAUDE_CODE_HEADLESS_BASELINE_VERSION, "2.1.220");
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.241");
    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "2.1.238"
    );

    let claim = claude_code_headless_claim();
    assert!(claim.supports(&version("2.1.220")));
    assert!(claim.supports(&version("2.1.221")));
    assert!(claim.supports(&version("2.1.238")));
    assert!(claim.supports(&version("2.1.239")));
    assert!(claim.supports(&version("2.1.240")));
    assert!(matches!(
        claim.assess(&version("2.1.241")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        claim.assess(&version("2.1.242")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        claude_code_headless_binding("2.1.241")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_CODE_HEADLESS_AXIS
    );
}

#[test]
fn structured_output_evidence_fixture_is_secret_free_and_fail_closed() {
    let evidence: Value = serde_json::from_str(STRUCTURED_OUTPUT)
        .expect("Claude Code 2.1.238 structured-output corpus is valid JSON");

    assert_eq!(evidence["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(evidence["version"], "2.1.238");
    assert_eq!(evidence["provider_prompt_sent"], false);
    assert_eq!(evidence["credentials_used"], false);
    assert_eq!(
        evidence["help"]["json_schema_flag"],
        "--json-schema <schema>"
    );
    assert_eq!(evidence["help"]["max_turns_flag_present"], false);

    let parse_cases = evidence["schema_parse_specimens"]
        .as_array()
        .expect("schema parse specimens are an array");
    assert_eq!(
        parse_cases[0]["stderr"],
        "Error: --json-schema is not valid JSON: JSON Parse error: Expected '}'"
    );
    assert_eq!(
        parse_cases[1]["stderr"],
        "Error: --json-schema is not a valid JSON Schema: data/required must be array"
    );
    assert_eq!(
        parse_cases[2]["stderr"],
        "Error: --json-schema is not a valid JSON Schema: strict mode: unknown keyword: \"x-unsupported\""
    );
    assert_eq!(parse_cases[3]["case"], "declared-draft-07");
    assert_eq!(parse_cases[3]["local_validation"], "accepted");
    assert_eq!(
        parse_cases[3]["no_authentication_result"]["subtype"],
        "success"
    );
    assert_eq!(parse_cases[3]["no_authentication_result"]["is_error"], true);
    assert_eq!(
        parse_cases[4]["stderr"],
        "Error: --json-schema is not a valid JSON Schema: no schema with key or ref \"https://json-schema.org/draft/2019-09/schema\""
    );
    assert_eq!(
        parse_cases[5]["stderr"],
        "Error: --json-schema is not a valid JSON Schema: no schema with key or ref \"https://json-schema.org/draft/2020-12/schema\""
    );

    let arguments = evidence["selected_headless_command"]["arguments"]
        .as_array()
        .expect("selected command arguments are an array");
    for required in [
        "--input-format",
        "--output-format",
        "--no-session-persistence",
        "--permission-mode",
        "--tools",
        "--mcp-config",
        "--strict-mcp-config",
        "--json-schema",
    ] {
        assert!(arguments.iter().any(|argument| argument == required));
    }

    assert_eq!(
        evidence["stream_without_structured_output"]["init"]["tools"],
        serde_json::json!(["Glob", "Grep", "Read", "StructuredOutput"])
    );
    assert_eq!(
        evidence["stream_without_structured_output"]["result"]["subtype"],
        "success"
    );
    assert_eq!(
        evidence["stream_without_structured_output"]["result"]["structured_output_field"],
        "absent"
    );
    assert_eq!(
        evidence["implementation_signals"]["contract_040_enforcement_classification"],
        "HarnessValidated"
    );
    assert_eq!(
        evidence["schema_dialect"]["official_sdk_dialect"],
        "draft-07"
    );
    assert_eq!(
        evidence["schema_dialect"]["exact_cli_declared_draft"],
        "draft-07 at local validation boundary"
    );
    assert_eq!(
        evidence["schema_dialect"]["exact_keyword_subset"],
        "unresolved"
    );
    assert!(evidence["implementation_signals"]["exact_retry_count"].is_null());
    assert!(evidence["implementation_signals"]["preflight_bindable_retry_bound"].is_null());
    assert_eq!(
        evidence["disposition"]["deliver_now_rows"],
        serde_json::json!([])
    );
    assert_eq!(evidence["disposition"]["schema_absent_path"], "unchanged");
}

#[test]
fn ultracode_evidence_fixture_is_secret_free_and_fail_closed() {
    let evidence: Value = serde_json::from_str(ULTRACODE)
        .expect("Claude Code 2.1.241 ultracode corpus is valid JSON");

    assert_eq!(evidence["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(evidence["version"], "2.1.241");
    assert_eq!(evidence["provider_prompt_sent"], false);
    assert_eq!(evidence["credentials_used"], false);
    assert_eq!(evidence["help"]["ultracode_advertised_in_help"], false);
    assert_eq!(
        evidence["help"]["advertised_effort_choices"],
        serde_json::json!(["low", "medium", "high", "xhigh", "max"])
    );
    assert_eq!(
        evidence["probed_versions"]["2.1.202"]["effort_ultracode_parser"],
        "rejected"
    );
    assert_eq!(
        evidence["probed_versions"]["2.1.203"]["effort_ultracode_parser"],
        "accepted"
    );
    assert_eq!(
        evidence["probed_versions"]["2.1.241"]["effort_ultracode_parser"],
        "accepted"
    );
    assert_eq!(
        evidence["portable_reasoning"]["seventh_reasoning_mode"],
        "rejected"
    );
    assert_eq!(evidence["portable_reasoning"]["xhigh_alias"], "rejected");
    assert_eq!(
        evidence["portable_reasoning"]["adapter_local_opt_in"],
        "withheld"
    );
    assert_eq!(
        evidence["disposition"]["deliver_now_rows"],
        serde_json::json!([])
    );
    assert_eq!(evidence["disposition"]["cards_182_183"], "blocked");
    assert_eq!(evidence["disposition"]["existing_route"], "unchanged");
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
