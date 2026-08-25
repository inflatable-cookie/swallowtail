use super::support::{
    assert_status, execute, local_topology, prepared, prepared_with_version, profile,
};
use swallowtail_runtime::{OperationContent, ProcessExit, RuntimeEventKind, TerminalStatus};

const V2_COMPLETE: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/headless-complete.jsonl");
const V2_TOOLS: &str = include_str!("fixtures/kimi-code-0.38.0-headless-v2/headless-tools.jsonl");
const V2_RETRY: &str = include_str!("fixtures/kimi-code-0.38.0-headless-v2/headless-retry.jsonl");
const V2_MALFORMED: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/headless-malformed.jsonl");
const V2_UNKNOWN: &str =
    include_str!("fixtures/kimi-code-0.38.0-headless-v2/headless-unknown.jsonl");
const V1_COMPLETE: &str = include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-complete.jsonl");

#[test]
fn v2_complete_corpus_requires_matching_version_preamble() {
    let topology = local_topology();
    let prepared = prepared_with_version(topology.execution_host_id().clone(), "0.38.0");
    assert_eq!(
        prepared.instance().protocol_facade_id().as_str(),
        "kimi-headless-stream-json-v1"
    );
    let evidence = execute(
        &profile(
            &prepared,
            topology.working_resource().clone(),
            "v2-complete",
        ),
        topology.execution_host_id().clone(),
        V2_COMPLETE,
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        evidence.outcome.output().map(OperationContent::as_str),
        Some("fixture result")
    );
    assert!(
        !evidence.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(
                    activity.kind(),
                    swallowtail_runtime::ActivityKind::Unknown(namespace)
                        if namespace.as_str() == "kimi-code.headless.system.version"
                )
        ))
    );
    assert!(evidence.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if activity.kind() == &swallowtail_runtime::ActivityKind::AssistantMessage
    )));
}

#[test]
fn v2_tools_and_retry_corpus_decode_under_v2_revision() {
    let topology = local_topology();
    let prepared = prepared_with_version(topology.execution_host_id().clone(), "0.38.0");

    let tools = execute(
        &profile(&prepared, topology.working_resource().clone(), "v2-tools"),
        topology.execution_host_id().clone(),
        V2_TOOLS,
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(tools.outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        tools.outcome.output().map(OperationContent::as_str),
        Some("checkingdone")
    );
    assert!(tools.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if activity.kind() == &swallowtail_runtime::ActivityKind::ProviderOwnedTool
    )));

    let retry = execute(
        &profile(&prepared, topology.working_resource().clone(), "v2-retry"),
        topology.execution_host_id().clone(),
        V2_RETRY,
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(retry.outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        retry.outcome.output().map(OperationContent::as_str),
        Some("final answer")
    );
    assert!(!format!("{:?}", retry.events).contains("fixture retry"));
}

#[test]
fn v2_malformed_unknown_and_revision_mismatches_fail_before_provider_work() {
    let topology = local_topology();
    let prepared_v2 = prepared_with_version(topology.execution_host_id().clone(), "0.38.0");
    let prepared_v1 = prepared(topology.execution_host_id().clone());

    let malformed = execute(
        &profile(
            &prepared_v2,
            topology.working_resource().clone(),
            "v2-malformed",
        ),
        topology.execution_host_id().clone(),
        V2_MALFORMED,
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &malformed.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );

    let unknown = execute(
        &profile(
            &prepared_v2,
            topology.working_resource().clone(),
            "v2-unknown",
        ),
        topology.execution_host_id().clone(),
        V2_UNKNOWN,
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(unknown.outcome.status(), &TerminalStatus::Completed);

    let v1_on_v2 = execute(
        &profile(
            &prepared_v1,
            topology.working_resource().clone(),
            "v1-on-v2",
        ),
        topology.execution_host_id().clone(),
        V2_COMPLETE,
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &v1_on_v2.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );

    let v2_without_preamble = execute(
        &profile(
            &prepared_v2,
            topology.working_resource().clone(),
            "v2-no-preamble",
        ),
        topology.execution_host_id().clone(),
        V1_COMPLETE,
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &v2_without_preamble.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );

    let wrong_version = execute(
        &profile(
            &prepared_v2,
            topology.working_resource().clone(),
            "v2-wrong-version",
        ),
        topology.execution_host_id().clone(),
        "{\"role\":\"meta\",\"type\":\"system.version\",\"version\":\"0.37.2\"}\n{\"role\":\"assistant\",\"content\":\"fixture\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &wrong_version.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );

    let duplicate = execute(
        &profile(
            &prepared_v2,
            topology.working_resource().clone(),
            "v2-duplicate",
        ),
        topology.execution_host_id().clone(),
        "{\"role\":\"meta\",\"type\":\"system.version\",\"version\":\"0.38.0\"}\n{\"role\":\"meta\",\"type\":\"system.version\",\"version\":\"0.38.0\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &duplicate.outcome,
        "swallowtail.kimi.headless.malformed_stream",
        false,
    );
}

#[test]
fn v2_incomplete_and_interrupted_streams_stay_distinct() {
    let topology = local_topology();
    let prepared = prepared_with_version(topology.execution_host_id().clone(), "0.38.0");

    let incomplete = execute(
        &profile(
            &prepared,
            topology.working_resource().clone(),
            "v2-incomplete",
        ),
        topology.execution_host_id().clone(),
        "{\"role\":\"meta\",\"type\":\"system.version\",\"version\":\"0.38.0\"}\n{\"role\":\"assistant\",\"content\":\"no terminal\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &incomplete.outcome,
        "swallowtail.kimi.headless.incomplete_stream",
        false,
    );

    let interrupted = execute(
        &profile(
            &prepared,
            topology.working_resource().clone(),
            "v2-interrupted",
        ),
        topology.execution_host_id().clone(),
        V2_COMPLETE,
        ProcessExit::new(false, Some(130)),
    );
    assert_status(
        &interrupted.outcome,
        "swallowtail.kimi.headless.process_interrupted",
        true,
    );
}
