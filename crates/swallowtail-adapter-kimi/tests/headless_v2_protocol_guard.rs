use super::support::{assert_status, execute, local_topology, prepared_with_version, profile};
use swallowtail_runtime::{ProcessExit, RuntimeEventKind};

const V2_PROVIDER_FAILURE: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/headless-provider-failure.jsonl");

#[test]
fn v2_rejects_late_version_preamble_after_unknown_meta_or_role() {
    let topology = local_topology();
    let prepared = prepared_with_version(topology.execution_host_id().clone(), "0.38.0");

    let late_meta_preamble = execute(
        &profile(
            &prepared,
            topology.working_resource().clone(),
            "v2-late-meta-preamble",
        ),
        topology.execution_host_id().clone(),
        "{\"role\":\"meta\",\"type\":\"future.activity\",\"content\":\"fixture\"}\n{\"role\":\"meta\",\"type\":\"system.version\",\"version\":\"0.38.0\"}\n{\"role\":\"assistant\",\"content\":\"fixture\"}\n{\"role\":\"meta\",\"type\":\"session.resume_hint\",\"session_id\":\"fixture-session\",\"command\":\"kimi -r fixture-session\",\"content\":\"To resume this session: kimi -r fixture-session\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &late_meta_preamble.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );

    let late_role_preamble = execute(
        &profile(
            &prepared,
            topology.working_resource().clone(),
            "v2-late-role-preamble",
        ),
        topology.execution_host_id().clone(),
        "{\"role\":\"error\",\"type\":\"provider_error\",\"message\":\"fixture\"}\n{\"role\":\"meta\",\"type\":\"system.version\",\"version\":\"0.38.0\"}\n{\"role\":\"assistant\",\"content\":\"fixture\"}\n{\"role\":\"meta\",\"type\":\"session.resume_hint\",\"session_id\":\"fixture-session\",\"command\":\"kimi -r fixture-session\",\"content\":\"To resume this session: kimi -r fixture-session\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &late_role_preamble.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );
}

#[test]
fn v2_provider_failure_role_maps_to_unknown_activity_without_terminal() {
    let topology = local_topology();
    let prepared = prepared_with_version(topology.execution_host_id().clone(), "0.38.0");
    let evidence = execute(
        &profile(
            &prepared,
            topology.working_resource().clone(),
            "v2-provider-failure",
        ),
        topology.execution_host_id().clone(),
        V2_PROVIDER_FAILURE,
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &evidence.outcome,
        "swallowtail.kimi.headless.incomplete_stream",
        false,
    );
    assert!(evidence.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if matches!(
                activity.kind(),
                swallowtail_runtime::ActivityKind::Unknown(namespace)
                    if namespace.as_str() == "kimi-code.headless.role.error"
            )
    )));
    assert!(!format!("{:?}", evidence.events).contains("fixture-private-provider-failure"));
}
