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
    for version in [
        "0.29.0", "0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1", "0.34.0", "0.36.1", "0.37.2",
        "0.38.0",
    ] {
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

#[test]
fn callback_bearing_sessions_reject_detachment_before_dispatch() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.callback-detachment");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services, "0.29.0");
    let error = prepared
        .prepare_session(session_input(
            "callback-detachment",
            KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Manual)
                .with_active_turn_detachment(),
        ))
        .expect_err("callback-bearing detachment rejects");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.detachment_unsupported"
    );
    assert!(
        server
            .requests()
            .iter()
            .all(|request| { !request.contains("/sessions/interactive-session/prompts") })
    );
}

#[test]
fn unverified_newer_session_cannot_opt_into_detachment() {
    let server =
        InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, "0.38.1");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.newer-detachment");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services, "0.38.1");
    let error = prepared
        .prepare_session(
            session_input(
                "newer-detachment",
                KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto)
                    .with_active_turn_detachment(),
            )
            .allow_unverified_newer(),
        )
        .expect_err("unverified newer detachment rejects");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.detachment_unsupported"
    );
}
