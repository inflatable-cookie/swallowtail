use super::fixture::{id, prepare, session_input};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::FixtureHost;
use swallowtail_adapter_kimi::{
    KimiLocalServerPermissionMode, KimiLocalServerSessionConfiguration,
};
use swallowtail_core::ExecutionHostId;
use swallowtail_core::ReasoningMode;

#[test]
fn revision_specific_options_require_the_qualified_milestone() {
    let server =
        InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, "0.28.1");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.baseline");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services, "0.28.1");
    let configuration =
        KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto)
            .with_profile("default")
            .expect("profile value is valid");
    let error = prepared
        .prepare_session(session_input("baseline-profile", configuration))
        .expect_err("baseline rejects the newer prompt option");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.revision_option_unsupported"
    );

    prepared
        .prepare_session(
            session_input(
                "model-declared-effort",
                KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto),
            )
            .with_reasoning(id(ReasoningMode::new, "xhigh")),
        )
        .expect("model-declared reasoning effort remains explicit and permitted");
}

#[test]
fn profile_and_tool_options_remain_available_across_later_milestones() {
    for version in ["0.29.0", "0.29.1", "0.29.2"] {
        let server =
            InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, version);
        let host = FixtureHost::for_endpoint(server.endpoint());
        let execution_host = id(ExecutionHostId::new, &format!("fixture.kimi.{version}"));
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare(execution_host, services, version);
        let configuration =
            KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto)
                .with_profile("default")
                .expect("profile value is valid")
                .with_disabled_tools(["Bash".to_owned()])
                .expect("tool value is valid");
        prepared
            .prepare_session(session_input("qualified-profile-tools", configuration))
            .expect("later qualified revisions preserve profile and tool options");
    }
}
