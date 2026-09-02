use serde_json::Value;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim, antigravity_release_binding,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.1.14/identity.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.1.14/protocol.json");

#[test]
fn identity_and_claim_qualify_1_1_14_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Antigravity 1.1.14 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Antigravity 1.1.14 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(
        identity["github_repo"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["github_latest"], true);
    assert_eq!(identity["not_gemini_cli"], true);
    assert_eq!(identity["host"]["version"], "1.1.9");
    assert_eq!(identity["official"]["version"], "1.1.14");
    assert_eq!(
        identity["official"]["github_commit"],
        "fbf22703a9c4bda0758b5bace0ab3142746780a9"
    );
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert!(is_sha256(
        identity["official"]["extracted_cli_sha256"]
            .as_str()
            .expect("official digest is text")
    ));
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["unpublished_1_1_15"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_catalogue_behavior"],
        "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(
        decision["reuse_headless_behavior"],
        "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "1.1.14");
    assert_eq!(decision["keep_baseline"], "1.1.9");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["map_input_format"], false);
    assert_eq!(decision["flatten_to_gemini_api_key"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_catalogue"], false);
    assert_eq!(decision["live_print_run"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--print",
        "--output-format",
        "--model",
        "--mode",
        "--sandbox",
        "--effort",
        "--json-schema",
        "--conversation",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_catalogue_command"], "models");
    assert_eq!(protocol["map_input_format"], false);
    assert_eq!(protocol["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_LATEST_QUALIFIED_VERSION, "1.1.24");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.9"
    );

    let catalogue = antigravity_catalogue_claim();
    let headless = antigravity_headless_claim();
    assert!(matches!(
        catalogue.assess(&version("1.1.9")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str()
                    == "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
    ));
    for candidate in ["1.1.10", "1.1.11", "1.1.12", "1.1.13", "1.1.14"] {
        assert!(matches!(
            catalogue.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
        ));
        assert!(matches!(
            headless.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
        ));
    }
    assert!(!catalogue.permits(&version("1.1.8")));
    assert!(!headless.permits(&version("1.1.8")));
    assert_eq!(
        antigravity_release_binding("1.1.14")
            .expect("version binds")
            .axis()
            .as_str(),
        ANTIGRAVITY_RELEASE_AXIS
    );
}

const IDENTITY_1_1_15: &str = include_str!("fixtures/antigravity-cli-1.1.15/identity.json");
const PROTOCOL_1_1_15: &str = include_str!("fixtures/antigravity-cli-1.1.15/protocol.json");

#[test]
fn identity_and_claim_qualify_1_1_15_as_compatible_extension() {
    let identity: Value = serde_json::from_str(IDENTITY_1_1_15)
        .expect("Antigravity 1.1.15 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_1_1_15)
        .expect("Antigravity 1.1.15 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(
        identity["github_repo"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["github_latest"], true);
    assert_eq!(identity["not_gemini_cli"], true);
    assert_eq!(identity["host"]["version"], "1.1.9");
    assert_eq!(identity["official"]["version"], "1.1.15");
    assert_eq!(
        identity["official"]["github_commit"],
        "76ff39c65b5d52482172b6408c27ded9b17c303d"
    );
    assert_eq!(identity["official"]["extracted_version"], "1.1.15");
    assert!(is_sha256(
        identity["host"]["executable_sha256"]
            .as_str()
            .expect("host digest is text")
    ));
    assert!(is_sha256(
        identity["official"]["extracted_cli_sha256"]
            .as_str()
            .expect("official digest is text")
    ));
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["unpublished_1_1_16"], true);
    assert_eq!(identity["public_git_1_1_14_to_1_1_15"], "changelog-only");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_catalogue_behavior"],
        "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(
        decision["reuse_headless_behavior"],
        "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "1.1.15");
    assert_eq!(decision["keep_baseline"], "1.1.9");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["map_input_format"], false);
    assert_eq!(decision["flatten_to_gemini_api_key"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_catalogue"], false);
    assert_eq!(decision["live_print_run"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--print",
        "--output-format",
        "--model",
        "--mode",
        "--sandbox",
        "--effort",
        "--json-schema",
        "--conversation",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_catalogue_command"], "models");
    assert_eq!(protocol["map_input_format"], false);
    assert_eq!(protocol["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(ANTIGRAVITY_LATEST_QUALIFIED_VERSION, "1.1.24");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.14"
    );

    let catalogue = antigravity_catalogue_claim();
    let headless = antigravity_headless_claim();
    for candidate in ["1.1.9", "1.1.14", "1.1.15"] {
        assert!(matches!(
            catalogue.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
        ));
        assert!(matches!(
            headless.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
        ));
    }
    assert!(!catalogue.permits(&version("1.1.8")));
    assert!(!headless.permits(&version("1.1.8")));
    assert!(matches!(
        catalogue.assess(&version("1.1.25")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        antigravity_release_binding("1.1.15")
            .expect("version binds")
            .axis()
            .as_str(),
        ANTIGRAVITY_RELEASE_AXIS
    );
}

const IDENTITY_1_1_17: &str = include_str!("fixtures/antigravity-cli-1.1.17/identity.json");
const PROTOCOL_1_1_17: &str = include_str!("fixtures/antigravity-cli-1.1.17/protocol.json");

#[test]
fn identity_and_claim_qualify_1_1_17_as_compatible_extension() {
    let identity: Value = serde_json::from_str(IDENTITY_1_1_17)
        .expect("Antigravity 1.1.17 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_1_1_17)
        .expect("Antigravity 1.1.17 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(
        identity["github_repo"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["github_latest"], true);
    assert_eq!(identity["not_gemini_cli"], true);
    assert_eq!(identity["not_antigravity_acp"], true);
    assert_eq!(identity["host"]["installed"], false);
    assert_eq!(identity["official"]["version"], "1.1.17");
    assert_eq!(
        identity["official"]["github_commit"],
        "efa16f096dc02fb654b7e86958d268195284d014"
    );
    assert_eq!(identity["official"]["extracted_version"], "1.1.17");
    assert!(is_sha256(
        identity["official"]["extracted_cli_sha256"]
            .as_str()
            .expect("official digest is text")
    ));
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["unpublished_1_1_18"], true);
    assert_eq!(identity["public_git_1_1_15_to_1_1_17"], "changelog-only");
    assert_eq!(identity["public_git_1_1_16_to_1_1_17"], "identical");
    assert_eq!(identity["shared_git_sha_does_not_unify_binaries"], true);
    assert_eq!(identity["flatten_to_antigravity_acp"], false);
    assert_eq!(
        identity["published_stables_from_previous_ceiling"],
        serde_json::json!(["1.1.16", "1.1.17"])
    );
    assert_eq!(
        identity["intermediate_1_1_16"]["extracted_version"],
        "1.1.16"
    );
    assert_ne!(
        identity["official"]["extracted_cli_sha256"],
        identity["intermediate_1_1_16"]["extracted_cli_sha256"]
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_catalogue_behavior"],
        "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(
        decision["reuse_headless_behavior"],
        "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "1.1.17");
    assert_eq!(decision["keep_baseline"], "1.1.9");
    assert_eq!(
        decision["qualify_intermediates"],
        serde_json::json!(["1.1.16"])
    );
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["map_input_format"], false);
    assert_eq!(decision["map_mcp_subcommand"], false);
    assert_eq!(decision["flatten_to_gemini_api_key"], false);
    assert_eq!(decision["flatten_to_antigravity_acp"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["live_catalogue"], false);
    assert_eq!(decision["live_print_run"], false);
    assert_eq!(decision["host_install_changed"], false);

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "--print",
        "--output-format",
        "--model",
        "--mode",
        "--sandbox",
        "--effort",
        "--json-schema",
        "--conversation",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_catalogue_command"], "models");
    assert_eq!(protocol["map_input_format"], false);
    assert_eq!(protocol["help_1_1_16_byte_identical_to_1_1_17"], true);
    assert_eq!(
        protocol["help_delta_from_1_1_15"],
        serde_json::json!(["mcp"])
    );
    assert_eq!(protocol["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(ANTIGRAVITY_LATEST_QUALIFIED_VERSION, "1.1.24");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.15"
    );

    let catalogue = antigravity_catalogue_claim();
    let headless = antigravity_headless_claim();
    for candidate in ["1.1.9", "1.1.15", "1.1.16", "1.1.17"] {
        assert!(matches!(
            catalogue.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
        ));
        assert!(matches!(
            headless.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
        ));
    }
    assert!(!catalogue.permits(&version("1.1.8")));
    assert!(!headless.permits(&version("1.1.8")));
    assert!(matches!(
        catalogue.assess(&version("1.1.25")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert_eq!(
        antigravity_release_binding("1.1.17")
            .expect("version binds")
            .axis()
            .as_str(),
        ANTIGRAVITY_RELEASE_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
