use crate::fixture::{id, prepare, session_input};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::FixtureHost;
use swallowtail_adapter_kimi::{
    KimiLocalServerPermissionMode, KimiLocalServerSessionConfiguration,
};
use swallowtail_core::{ExecutionHostId, ReasoningMode};
use swallowtail_runtime::ConsumerRouteProjectionSourceId;

#[test]
fn local_session_optional_rows_follow_the_prepared_profile() {
    let server =
        InteractiveFixtureServer::start_with_version(InteractiveScenario::Complete, "0.29.2");
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.local.projection");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services, "0.29.2");

    let minimal = prepared
        .prepare_session(session_input(
            "local-projection-minimal",
            KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto),
        ))
        .expect("minimal session prepares")
        .consumer_route_projection_contribution(source("kimi.local.minimal"))
        .expect("minimal projection is admitted");
    assert!(!has_namespace(&minimal, "feature.permission-exchange"));
    assert!(!has_namespace(&minimal, "feature.question-exchange"));
    assert!(!has_namespace(&minimal, "control.active-turn-detachment"));

    let manual = prepared
        .prepare_session(
            session_input(
                "local-projection-manual",
                KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Manual),
            )
            .with_reasoning(id(ReasoningMode::new, "high")),
        )
        .expect("manual session prepares")
        .consumer_route_projection_contribution(source("kimi.local.manual"))
        .expect("manual projection is admitted");
    assert!(has_namespace(&manual, "feature.permission-exchange"));
    assert!(has_namespace(&manual, "feature.question-exchange"));

    let detached = prepared
        .prepare_session(session_input(
            "local-projection-detached",
            KimiLocalServerSessionConfiguration::new(KimiLocalServerPermissionMode::Auto)
                .with_active_turn_detachment(),
        ))
        .expect("detached session prepares")
        .consumer_route_projection_contribution(source("kimi.local.detached"))
        .expect("detached projection is admitted");
    assert!(has_namespace(&detached, "control.active-turn-detachment"));
}

fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source")
}

fn has_namespace(
    contribution: &swallowtail_runtime::ConsumerRouteProjectionContribution,
    semantic: &str,
) -> bool {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .any(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| extension.semantic_id() == semantic)
        })
}
