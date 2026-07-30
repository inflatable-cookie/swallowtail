#[test]
fn tool_progress_provider_failure_and_malformed_stream_remain_distinct() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "outcomes",
        None,
    );

    let tools = execute(
        &profile,
        topology.execution_host_id().clone(),
        &fixture("headless-tools.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(tools.outcome.status(), &TerminalStatus::Completed);
    assert!(tools.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if activity.kind() == &swallowtail_runtime::ActivityKind::ProviderOwnedTool
    )));
    assert!(!format!("{:?}", tools.events).contains("private fixture file content"));

    let provider = execute(
        &profile,
        topology.execution_host_id().clone(),
        &fixture("headless-provider-failure.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &provider.outcome,
        "swallowtail.claude_code.headless.provider_failed",
        true,
    );

    let malformed = execute(
        &profile,
        topology.execution_host_id().clone(),
        "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"wrong\",\"permissionMode\":\"plan\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &malformed.outcome,
        "swallowtail.claude_code.headless.malformed_stream",
        false,
    );

    for invalid_pre_init in [
        concat!(
            "{\"type\":\"system\",\"subtype\":\"hook_started\",",
            "\"session_id\":\"first-session\"}\n",
            "{\"type\":\"system\",\"subtype\":\"init\",",
            "\"session_id\":\"second-session\",\"model\":\"claude-opus-5\",",
            "\"permissionMode\":\"plan\"}\n",
        ),
        "{\"type\":\"rate_limit_event\",\"session_id\":\"fixture-session\"}\n",
    ] {
        let rejected = execute(
            &profile,
            topology.execution_host_id().clone(),
            invalid_pre_init,
            ProcessExit::new(true, Some(0)),
        );
        assert_status(
            &rejected.outcome,
            "swallowtail.claude_code.headless.malformed_stream",
            false,
        );
    }

    let incomplete = execute(
        &profile,
        topology.execution_host_id().clone(),
        "",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &incomplete.outcome,
        "swallowtail.claude_code.headless.incomplete_stream",
        false,
    );

    let failed = execute(
        &profile,
        topology.execution_host_id().clone(),
        "",
        ProcessExit::new(false, Some(1)),
    );
    assert_status(
        &failed.outcome,
        "swallowtail.claude_code.headless.process_failed",
        true,
    );
}

