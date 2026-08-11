use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
use swallowtail_adapter_command_code::{
    COMMAND_CODE_EXECUTABLE_NAME, COMMAND_CODE_RELEASE_AXIS, COMMAND_CODE_RELEASE_VERSION,
    CommandCodeHeadlessDriver, CommandCodeHeadlessModelSelection, CommandCodePreparationInput,
    CommandCodePreparationProbe, CommandCodeRunProfileInput, CommandCodeSessionProfileInput,
    command_code_headless_claim, command_code_local_account_access_profile,
    prepare_command_code_headless,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, DiscoveryStatus,
    EndpointAuthorization, EntitlementState, ExecutionHostId, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_host_local::{
    LocalExecutableLaunch, LocalHostServices, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{
    DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, OperationContent, PreparedAccessEvidence, RequestId,
    RuntimeTurnId, ScopeId, TerminalStatus, TurnRequest, WorkingResourceRef,
};

const MAXIMUM_SHEBANG_BYTES: u64 = 64;

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_COMMAND_CODE=1 and the exact installed command-code payload"]
fn installed_command_code_payload_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_COMMAND_CODE").as_deref(),
        Ok("1"),
        "installed Command Code probe requires its explicit gate"
    );
    let (local, target, _, _, execution_host_id) = live_host();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-command-code-installed-version").expect("request id"),
        ScopeId::new("live-command-code-installed-version").expect("scope id"),
        execution_host_id.clone(),
        target,
        local.deadline_after(Duration::from_secs(5)),
        DiscoveryCancellation::new(),
    );
    let driver = CommandCodeHeadlessDriver::new(
        EnvironmentRef::new("live.command-code.unused-environment").expect("environment"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed Command Code discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed payload produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        COMMAND_CODE_RELEASE_VERSION
    );
    assert_eq!(observation.claim_id(), command_code_headless_claim().id());
    assert!(observation.is_qualified());
}

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_COMMAND_CODE_PROMPT=1, local account state, one subscription-backed turn, and SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL"]
fn configured_command_code_completes_one_turn_through_the_prepared_facade() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_COMMAND_CODE_PROMPT").as_deref(),
        Ok("1"),
        "authenticated Command Code probe requires its explicit gate"
    );
    let model_id =
        std::env::var("SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL").expect("explicit live model id");
    let (local, target, environment, working_resource, execution_host_id) = live_host();
    let access_id = AccessProfileId::new("live.command-code.local-account").expect("access id");
    let prepared = block_on(prepare_command_code_headless(
        CommandCodePreparationInput::new(
            ConfiguredInstanceId::new("live.command-code.instance").expect("instance id"),
            InstanceRevision::new(COMMAND_CODE_RELEASE_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            command_code_local_account_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        CommandCodePreparationProbe::new(
            RequestId::new("live-command-code-prepare").expect("request id"),
            ScopeId::new("live-command-code-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed authenticated Command Code prepares");
    let run = prepared
        .prepare_run(CommandCodeRunProfileInput::new(
            RequestId::new("live-command-code-turn").expect("request id"),
            CommandCodeHeadlessModelSelection::new(
                ModelRouteId::new("live.command-code.route").expect("route id"),
                ModelRouteRevision::new(COMMAND_CODE_RELEASE_VERSION).expect("route revision"),
                ProviderId::new("command-code").expect("provider id"),
                ModelId::new(model_id).expect("model id"),
            ),
            OperationContent::new("Reply exactly COMMAND_CODE_LIVE_OK. Do not use tools.")
                .expect("prompt"),
            working_resource,
            local.deadline_after(Duration::from_secs(90)),
        ))
        .expect("Command Code run prepares");

    let mut handle = block_on(run.start_run(local.services().clone()))
        .expect("authenticated Command Code run starts");
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live Command Code event remains valid");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome
            .output()
            .map(OperationContent::as_str)
            .map(str::trim),
        Some("COMMAND_CODE_LIVE_OK")
    );
    assert!(matches!(
        outcome.cleanup(),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(matches!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_COMMAND_CODE_PROMPT=1, local account state, two subscription-backed turns, and SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL"]
fn configured_command_code_completes_two_interactive_turns_with_exact_resume() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_COMMAND_CODE_PROMPT").as_deref(),
        Ok("1"),
        "authenticated Command Code interactive probe requires its explicit gate"
    );
    let model_id =
        std::env::var("SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL").expect("explicit live model id");
    let (local, target, environment, working_resource, execution_host_id) = live_host();
    let access_id = AccessProfileId::new("live.command-code.local-account").expect("access id");
    let prepared = block_on(prepare_command_code_headless(
        CommandCodePreparationInput::new(
            ConfiguredInstanceId::new("live.command-code.instance").expect("instance id"),
            InstanceRevision::new(COMMAND_CODE_RELEASE_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            command_code_local_account_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        CommandCodePreparationProbe::new(
            RequestId::new("live-command-code-interactive-prepare").expect("request id"),
            ScopeId::new("live-command-code-interactive-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed authenticated Command Code prepares");
    let session = prepared
        .prepare_session(
            CommandCodeSessionProfileInput::new(
                RequestId::new("live-command-code-interactive").expect("request id"),
                CommandCodeHeadlessModelSelection::new(
                    ModelRouteId::new("live.command-code.route").expect("route id"),
                    ModelRouteRevision::new(COMMAND_CODE_RELEASE_VERSION).expect("route revision"),
                    ProviderId::new("command-code").expect("provider id"),
                    ModelId::new(model_id).expect("model id"),
                ),
                working_resource,
            )
            .with_deadline(local.deadline_after(Duration::from_secs(90))),
        )
        .expect("Command Code interactive session prepares");
    let mut handle = block_on(session.open_session(local.services().clone()))
        .expect("interactive session opens");
    for (turn, expected) in [
        ("1", "COMMAND_CODE_LIVE_TURN_1"),
        ("2", "COMMAND_CODE_LIVE_TURN_2"),
    ] {
        let mut turn_handle = block_on(
            handle.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("live-command-code-turn-{turn}")).expect("turn id"),
                    OperationContent::new(format!("Reply exactly {expected}. Do not use tools."))
                        .expect("prompt"),
                )
                .with_deadline(local.deadline_after(Duration::from_secs(90))),
                local.services().clone(),
            ),
        )
        .expect("interactive turn starts");
        let mut events = turn_handle.take_events().expect("turn events");
        let terminal = turn_handle.take_terminal_outcome().expect("turn terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("live interactive event remains valid");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome
                .output()
                .map(OperationContent::as_str)
                .map(str::trim),
            Some(expected)
        );
        assert!(matches!(
            outcome.cleanup(),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
        assert!(matches!(
            block_on(turn_handle.close()),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
    }
    assert!(matches!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

fn live_host() -> (
    LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let selected = installed_path(COMMAND_CODE_EXECUTABLE_NAME)
        .expect("exact Command Code launcher is installed on PATH");
    let script = std::fs::canonicalize(selected).expect("Command Code launcher resolves exactly");
    assert_eq!(bounded_shebang(&script), "#!/usr/bin/env node");
    let node = std::fs::canonicalize(installed_path("node").expect("Node is installed on PATH"))
        .expect("Node interpreter resolves exactly");
    let environment = EnvironmentRef::new("live.command-code.local-account-environment")
        .expect("environment is valid");
    let working_resource = WorkingResourceRef::new("live.command-code.read-only-workspace")
        .expect("resource is valid");
    let execution_host_id = ExecutionHostId::new("live.command-code.local-host").expect("host id");
    let executable =
        ExecutableRef::new("live.command-code.installed").expect("executable ref is valid");
    let launch = LocalExecutableLaunch::interpreted_script(node, script);
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(COMMAND_CODE_RELEASE_AXIS).expect("release axis"),
            launch,
        );
    let home = std::env::var_os("HOME").expect("local Command Code auth requires HOME");
    let local = builder
        .approve_environment(environment.clone(), [(OsString::from("HOME"), home)])
        .approve_working_resource(
            working_resource.clone(),
            std::env::current_dir().expect("probe working directory"),
        )
        .build_services(execution_host_id.clone());
    (
        local,
        target,
        environment,
        working_resource,
        execution_host_id,
    )
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
}

fn bounded_shebang(path: &PathBuf) -> String {
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .expect("Command Code script is readable")
        .take(MAXIMUM_SHEBANG_BYTES)
        .read_to_end(&mut bytes)
        .expect("Command Code shebang remains bounded");
    let first_line = bytes
        .split(|byte| *byte == b'\n')
        .next()
        .expect("Command Code script has a first line");
    std::str::from_utf8(first_line)
        .expect("Command Code shebang is UTF-8")
        .to_owned()
}
