fn started_watcher_arguments(
    prepared: &ClaudeCodePreparedIntegration,
    local: &swallowtail_host_local::LocalHostServices,
    topology: &ExecutionTopologyFixture,
    id: &str,
) -> Vec<String> {
    let profile = watcher_profile(prepared, topology.working_resource().clone(), id, true)
        .expect("watcher opt-in prepares");
    let (process, state, completer) = FakeProcessService::controllable();
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        local,
    );
    let mut run = block_on(profile.start_run(services)).expect("watcher run starts");
    let started = std::time::Instant::now();
    while !state.started() {
        assert!(started.elapsed() < Duration::from_secs(2), "process starts");
        std::thread::yield_now();
    }
    let arguments = state.request().arguments;
    let mcp_path = argument_after(&arguments, "--mcp-config").to_owned();
    completer.complete(
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    let _ = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(task.joined());
    assert!(!Path::new(&mcp_path).exists());
    secret_free(&arguments);
    arguments
}

#[test]
fn the_unrepaired_watcher_command_still_fails_the_review_oracle_on_two_axes() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let arguments =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-current");
    let isolation = fixture_isolation(&arguments);

    assert!(arguments.iter().any(|argument| argument == "--bare"));
    assert_eq!(
        isolation.authentication,
        FixtureAuthenticationAdmission::ApiKeyOrHelperOnly
    );
    assert_eq!(isolation.ambient.skills, FixtureAmbientAdmission::Admitted);
    assert_eq!(isolation.composition.stop_hook, FixtureCompositionAdmission::Unstated);
    assert!(!isolation.satisfies_the_review_oracle());
}

#[test]
fn no_compared_isolation_candidate_satisfies_the_review_oracle() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let arguments =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-candidates");

    let candidates = [
        ("current-bare", replacing(&arguments, "--bare", &["--bare"])),
        (
            "watcher-only-restricted",
            replacing(&arguments, "--bare", &["--restricted"]),
        ),
        (
            "empty-setting-sources",
            replacing(&arguments, "--bare", &["--setting-sources", ""]),
        ),
        ("safe-mode", replacing(&arguments, "--bare", &["--safe-mode"])),
    ];
    for (name, candidate) in &candidates {
        assert!(
            !fixture_isolation(candidate).satisfies_the_review_oracle(),
            "{name} must not be admitted without exact ambient-exclusion evidence"
        );
    }
}

#[test]
fn the_ambient_authority_counterexample_names_the_axis_each_candidate_reopens() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let arguments =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-ambient");

    let restricted = fixture_isolation(&replacing(&arguments, "--bare", &["--restricted"]));
    assert_eq!(
        restricted.authentication,
        FixtureAuthenticationAdmission::HostConfigured
    );
    assert!(restricted.composition.is_wholly_preserved());
    assert_eq!(restricted.ambient.settings, FixtureAmbientAdmission::Excluded);
    for reopened in [
        &restricted.ambient.skills,
        &restricted.ambient.memory_claude_md,
        &restricted.ambient.plugins,
    ] {
        assert_eq!(*reopened, FixtureAmbientAdmission::Admitted);
    }
    assert!(!restricted.ambient.excludes_every_named_axis());
    assert!(!restricted.satisfies_the_review_oracle());

    let reopened_sources = fixture_isolation(&replacing(
        &arguments,
        "--bare",
        &["--restricted", "--setting-sources", "user,project,local"],
    ));
    assert_eq!(
        reopened_sources.ambient.settings,
        FixtureAmbientAdmission::Admitted
    );
}

#[test]
fn the_only_candidate_excluding_every_ambient_axis_disables_the_private_composition() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let arguments =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-safe-mode");

    let safe_mode = fixture_isolation(&replacing(&arguments, "--bare", &["--safe-mode"]));
    assert!(safe_mode.ambient.excludes_every_named_axis());
    assert_eq!(
        safe_mode.authentication,
        FixtureAuthenticationAdmission::HostConfigured
    );
    assert_eq!(
        safe_mode.composition,
        FixturePrivateComposition {
            private_mcp: FixtureCompositionAdmission::Disabled,
            stop_hook: FixtureCompositionAdmission::Disabled,
            injected_skill: FixtureCompositionAdmission::Disabled,
        }
    );
    assert!(!safe_mode.satisfies_the_review_oracle());

    let no_slash_commands = fixture_isolation(&replacing(
        &arguments,
        "--bare",
        &["--restricted", "--disable-slash-commands"],
    ));
    assert_eq!(
        no_slash_commands.ambient.skills,
        FixtureAmbientAdmission::Excluded
    );
    assert_eq!(
        no_slash_commands.composition.injected_skill,
        FixtureCompositionAdmission::Disabled
    );
    assert!(!no_slash_commands.satisfies_the_review_oracle());
}

#[test]
fn the_evidence_stop_leaves_the_omitted_command_byte_identical() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let profile = watcher_profile(
        &prepared,
        topology.working_resource().clone(),
        "watcher-isolation-omitted",
        false,
    )
    .expect("omission prepares");
    let (process, state) = FakeProcessService::with_exit(
        &fixture("headless-complete.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    let (services, task) = watcher_host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
        &local,
    );
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let _ = block_on(run.take_events().expect("events").collect::<Vec<_>>());
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());

    let arguments = state.request().arguments;
    assert_eq!(
        arguments,
        [
            "-p",
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "claude-opus-5",
            "--permission-mode",
            "plan",
            "--tools",
            "Read,Glob,Grep",
            "--setting-sources",
            "user,project,local",
            r#"--mcp-config"#,
            r#"{"mcpServers":{}}"#,
            "--strict-mcp-config",
        ]
    );
    for forbidden in ["--restricted", "--safe-mode", "--bare", "--settings", "--add-dir"] {
        assert!(
            !arguments.iter().any(|argument| argument == forbidden),
            "{forbidden}"
        );
    }
}
