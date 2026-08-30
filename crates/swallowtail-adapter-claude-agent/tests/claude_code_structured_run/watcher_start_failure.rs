fn retry_profile(
    prepared: &ClaudeCodePreparedIntegration,
    resource: WorkingResourceRef,
    id: &str,
) -> ClaudeCodePreparedRun {
    watcher_profile(prepared, resource, id, true).expect("opt-in prepares")
}

fn retry_succeeds(
    host: swallowtail_core::ExecutionHostId,
    profile: &ClaudeCodePreparedRun,
    local: &swallowtail_host_local::LocalHostServices,
) {
    let (process, state, completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        host,
        process,
        Arc::new(PendingTimeService),
        local,
    );
    let mut run = block_on(profile.start_run(services)).expect("same-turn retry starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }
    completer.complete(&fixture("headless-complete.jsonl"), ProcessExit::new(true, Some(0)));
    let _ = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let _ = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
}

#[test]
fn binding_failure_releases_the_feed_so_same_turn_retry_succeeds() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = retry_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-binding-fail",
    );
    let (process, _state) = FakeProcessService::held_open();
    let task = Arc::new(TaskState::default());
    let services = HostServices::new(topology.execution_host_id().clone())
        .with_task(Arc::new(ThreadTaskService::new(Arc::clone(&task))))
        .with_process(process)
        .with_time(Arc::new(PendingTimeService))
        .with_working_resource(
            local
                .services()
                .working_resource()
                .expect("working resource")
                .clone(),
        )
        .with_working_resource_io(
            local
                .services()
                .working_resource_io()
                .expect("working-resource I/O")
                .clone(),
        )
        .with_watcher(local.services().watcher().expect("watcher").clone());
    let error = match block_on(profile.start_run(services)) {
        Ok(_) => panic!("binding should fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_code.headless.host_service_missing"
    );
    retry_succeeds(topology.execution_host_id().clone(), &profile, &local);
}

#[test]
fn provider_start_failure_releases_the_feed_so_same_turn_retry_succeeds() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = retry_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-start-fail",
    );
    let process = FakeProcessService::fail_start();
    let (services, _task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let error = match block_on(profile.start_run(services)) {
        Ok(_) => panic!("start should fail"),
        Err(error) => error,
    };
    assert_eq!(error.diagnostic().code(), "fixture.process.start_failed");
    retry_succeeds(topology.execution_host_id().clone(), &profile, &local);
}

#[test]
fn prompt_write_failure_releases_the_feed_so_same_turn_retry_succeeds() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = retry_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-stdin-fail",
    );
    let (process, _state) = FakeProcessService::fail_stdin();
    let (services, _task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let error = match block_on(profile.start_run(services)) {
        Ok(_) => panic!("stdin should fail"),
        Err(error) => error,
    };
    assert_eq!(error.diagnostic().code(), "fixture.process.stdin_failed");
    retry_succeeds(topology.execution_host_id().clone(), &profile, &local);
}

#[test]
fn task_spawn_failure_releases_the_feed_so_same_turn_retry_succeeds() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = retry_profile(
        &prepared,
        topology.working_resource().clone(),
        "watchers-spawn-fail",
    );
    let (process, _state) = FakeProcessService::held_open();
    let services = HostServices::new(topology.execution_host_id().clone())
        .with_task(Arc::new(FailingTaskService))
        .with_process(process)
        .with_time(Arc::new(PendingTimeService))
        .with_working_resource(
            local
                .services()
                .working_resource()
                .expect("working resource")
                .clone(),
        )
        .with_working_resource_io(
            local
                .services()
                .working_resource_io()
                .expect("working-resource I/O")
                .clone(),
        )
        .with_watcher(local.services().watcher().expect("watcher").clone())
        .with_watcher_bridge(
            local
                .services()
                .watcher_bridge()
                .expect("watcher bridge")
                .clone(),
        );
    let error = match block_on(profile.start_run(services)) {
        Ok(_) => panic!("spawn should fail"),
        Err(error) => error,
    };
    assert_eq!(error.diagnostic().code(), "fixture.task.spawn_failed");
    retry_succeeds(topology.execution_host_id().clone(), &profile, &local);
}
