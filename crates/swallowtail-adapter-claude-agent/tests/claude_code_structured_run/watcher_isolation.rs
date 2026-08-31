/// Configured Claude authentication a watcher command still admits, as frozen
/// by exact `2.1.251` help in `fixtures/claude-code-2.1.251/watcher-isolation.json`.
#[derive(Debug, Eq, PartialEq)]
enum FixtureAuthenticationAdmission {
    /// No mode flag narrows credentials, so the host's configured path applies.
    HostConfigured,
    /// `--bare` never reads OAuth or keychain state.
    ApiKeyOrHelperOnly,
}

/// Ambient user, project, and local authority a watcher command still admits.
#[derive(Debug, Eq, PartialEq)]
enum FixtureAmbientAdmission {
    Excluded,
    Admitted(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
struct FixtureIsolation {
    authentication: FixtureAuthenticationAdmission,
    setting_sources: FixtureAmbientAdmission,
    mcp_servers: FixtureAmbientAdmission,
}

fn fixture_isolation(arguments: &[String]) -> FixtureIsolation {
    let names = |flag: &str| arguments.iter().any(|argument| argument == flag);
    let authentication = if names("--bare") {
        FixtureAuthenticationAdmission::ApiKeyOrHelperOnly
    } else {
        FixtureAuthenticationAdmission::HostConfigured
    };
    let setting_sources = if names("--bare") || names("--restricted") {
        FixtureAmbientAdmission::Excluded
    } else {
        let selected = arguments
            .iter()
            .position(|argument| argument == "--setting-sources")
            .and_then(|index| arguments.get(index + 1))
            .map(|value| {
                value
                    .split(',')
                    .filter(|source| !source.is_empty())
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if selected.is_empty() {
            FixtureAmbientAdmission::Excluded
        } else {
            FixtureAmbientAdmission::Admitted(selected)
        }
    };
    let mcp_servers = if names("--strict-mcp-config") {
        FixtureAmbientAdmission::Excluded
    } else {
        FixtureAmbientAdmission::Admitted(Vec::new())
    };
    FixtureIsolation {
        authentication,
        setting_sources,
        mcp_servers,
    }
}

fn replacing(arguments: &[String], flag: &str, replacement: &[&str]) -> Vec<String> {
    let index = arguments
        .iter()
        .position(|argument| argument == flag)
        .unwrap_or_else(|| panic!("{flag} is present"));
    let mut variant = arguments[..index].to_vec();
    variant.extend(replacement.iter().map(|argument| (*argument).to_owned()));
    variant.extend_from_slice(&arguments[index + 1..]);
    variant
}

fn started_watcher_arguments(
    prepared: &ClaudeCodePreparedIntegration,
    local: &swallowtail_host_local::LocalHostServices,
    topology: &ExecutionTopologyFixture,
    id: &str,
) -> (Vec<String>, String, String, String) {
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
    let settings_path = argument_after(&arguments, "--settings").to_owned();
    let add_dir = argument_after(&arguments, "--add-dir").to_owned();
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
    assert!(!Path::new(&settings_path).exists());
    (arguments, mcp_path, settings_path, add_dir)
}

#[test]
fn watcher_opt_in_preserves_configured_authentication_without_reopening_ambient_authority() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let (arguments, _, settings_path, add_dir) =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-current");

    assert_eq!(
        fixture_isolation(&arguments),
        FixtureIsolation {
            authentication: FixtureAuthenticationAdmission::HostConfigured,
            setting_sources: FixtureAmbientAdmission::Excluded,
            mcp_servers: FixtureAmbientAdmission::Excluded,
        }
    );
    assert!(arguments.iter().any(|argument| argument == "--restricted"));
    for forbidden in ["--bare", "--safe-mode", "--setting-sources"] {
        assert!(
            !arguments.iter().any(|argument| argument == forbidden),
            "{forbidden}"
        );
    }
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--permission-mode", "plan"])
    );
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--tools", "Read,Glob,Grep"])
    );
    assert!(Path::new(&settings_path).is_absolute());
    assert!(Path::new(&add_dir).is_absolute());
    secret_free(&arguments);
}

#[test]
fn the_ambient_reopening_counterexample_is_rejected_against_the_prepared_command() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let (arguments, ..) =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-ambient");

    let reopened = replacing(
        &arguments,
        "--restricted",
        &["--setting-sources", "user,project,local"],
    );
    assert_eq!(
        fixture_isolation(&reopened),
        FixtureIsolation {
            authentication: FixtureAuthenticationAdmission::HostConfigured,
            setting_sources: FixtureAmbientAdmission::Admitted(vec![
                "user".to_owned(),
                "project".to_owned(),
                "local".to_owned(),
            ]),
            mcp_servers: FixtureAmbientAdmission::Excluded,
        }
    );
    assert_ne!(fixture_isolation(&reopened), fixture_isolation(&arguments));
    assert_ne!(reopened, arguments);
}

#[test]
fn the_authentication_removing_counterexample_is_rejected_against_the_prepared_command() {
    let topology = ExecutionTopologyFixture::local();
    let (prepared, local) =
        prepared_at_with_watchers(topology.execution_host_id().clone(), "2.1.251");
    let (arguments, ..) =
        started_watcher_arguments(&prepared, &local, &topology, "watcher-isolation-bare");

    let bare = replacing(&arguments, "--restricted", &["--bare"]);
    assert_eq!(
        fixture_isolation(&bare),
        FixtureIsolation {
            authentication: FixtureAuthenticationAdmission::ApiKeyOrHelperOnly,
            setting_sources: FixtureAmbientAdmission::Excluded,
            mcp_servers: FixtureAmbientAdmission::Excluded,
        }
    );
    assert_ne!(fixture_isolation(&bare), fixture_isolation(&arguments));
    assert_eq!(
        fixture_isolation(&arguments).authentication,
        FixtureAuthenticationAdmission::HostConfigured
    );
}

#[test]
fn the_watcher_repair_leaves_the_omitted_command_byte_identical() {
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
    assert_eq!(
        fixture_isolation(&arguments),
        FixtureIsolation {
            authentication: FixtureAuthenticationAdmission::HostConfigured,
            setting_sources: FixtureAmbientAdmission::Admitted(vec![
                "user".to_owned(),
                "project".to_owned(),
                "local".to_owned(),
            ]),
            mcp_servers: FixtureAmbientAdmission::Excluded,
        }
    );
    for forbidden in ["--restricted", "--bare", "--settings", "--add-dir"] {
        assert!(
            !arguments.iter().any(|argument| argument == forbidden),
            "{forbidden}"
        );
    }
}
