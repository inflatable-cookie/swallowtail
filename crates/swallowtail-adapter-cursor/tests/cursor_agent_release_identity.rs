use serde_json::Value;
use swallowtail_adapter_cursor::{
    CURSOR_AGENT_AUGUST_04_BUILD_REVISION, CURSOR_AGENT_AUGUST_04_VERSION,
    CURSOR_AGENT_BASELINE_VERSION, CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION,
    CURSOR_AGENT_LATEST_QUALIFIED_VERSION, CURSOR_AGENT_RELEASE_AXIS, cursor_acp_claim,
    cursor_agent_release_binding, cursor_catalogue_claim, cursor_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str =
    include_str!("fixtures/cursor-agent-2026.08.04-2026.08.11/identity.json");
const PROTOCOL: &str =
    include_str!("fixtures/cursor-agent-2026.08.04-2026.08.11/protocol.json");

#[test]
fn identity_adds_exact_august_milestones_without_inferring_the_gap() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("Cursor 2026.08 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Cursor 2026.08 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], CURSOR_AGENT_RELEASE_AXIS);
    assert_eq!(identity["host"]["version"], "2026.08.04-aaa8809");
    assert_eq!(identity["official"]["version"], "2026.08.11-e8db854");
    assert_eq!(identity["host"]["release_date"], CURSOR_AGENT_AUGUST_04_VERSION);
    assert_eq!(
        identity["host"]["build_revision"],
        CURSOR_AGENT_AUGUST_04_BUILD_REVISION
    );
    assert_eq!(
        identity["official"]["release_date"],
        CURSOR_AGENT_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(
        identity["official"]["build_revision"],
        CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION
    );
    assert!(is_sha256(
        identity["host"]["runtime_index_sha256"]
            .as_str()
            .expect("host runtime digest is text")
    ));
    assert!(is_sha256(
        identity["official"]["artifact_sha256"]
            .as_str()
            .expect("official archive digest is text")
    ));

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "exact-milestones");
    assert_eq!(decision["infer_calendar_gap"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["mix_npm_cursor_agent_axis"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["raise_latest_qualified_to"], "2026.08.11-e8db854");
    assert_eq!(decision["add_host_milestone"], "2026.08.04-aaa8809");
    assert_eq!(decision["keep_baseline"], "2026.07.01-41b2de7");

    let flags = protocol["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in ["--print", "--output-format", "--model", "--trust", "--mode"] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(protocol["selected_catalogue_command"], "models");
    assert_eq!(protocol["selected_acp_command"], "acp");
    assert_eq!(protocol["acp_initialize"]["protocol_version"], 1);
    assert_eq!(
        protocol["acp_initialize"]["auth_methods"],
        serde_json::json!(["cursor_login"])
    );
    assert_eq!(protocol["acp_initialize"]["stderr_bytes"], 0);
    assert_eq!(protocol["continuation_recovery"], "blocked");
    assert_eq!(protocol["provider_prompt_sent"], false);

    assert_eq!(CURSOR_AGENT_BASELINE_VERSION, "2026-07-01");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "2026-07-23"
    );

    for claim in [
        cursor_catalogue_claim(),
        cursor_acp_claim(),
        cursor_headless_claim(),
    ] {
        assert!(matches!(
            claim.assess(&version("2026-08-04")),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
        ));
        assert!(matches!(
            claim.assess(&version("2026-08-11")),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
        ));
        assert!(!claim.permits(&version("2026-08-05")));
        assert!(matches!(
            claim.assess(&version("2026-08-12")),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
    assert_eq!(
        cursor_agent_release_binding("2026.08.11-e8db854")
            .expect("version binds")
            .axis()
            .as_str(),
        CURSOR_AGENT_RELEASE_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
