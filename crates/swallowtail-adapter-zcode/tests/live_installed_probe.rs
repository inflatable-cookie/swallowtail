use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;
use swallowtail_adapter_zcode::{
    ZCODE_EXECUTABLE_BASENAME, ZCODE_RELEASE_AXIS, ZCODE_RELEASE_VERSION, ZcodeAppServerDriver,
    ZcodeAppServerMode, ZcodeModelSelection, ZcodePreparationInput, ZcodePreparationProbe,
    ZcodeRunProfileInput, prepare_zcode_app_server, zcode_access_profile, zcode_app_server_claim,
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
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, OperationContent,
    PreparedAccessEvidence, RequestId, ScopeId, TerminalStatus, WorkingResourceRef,
};

#[test]
#[ignore = "requires explicit live gate, exact zcode.cjs payload, host settings, and cwd"]
fn installed_zcode_app_server_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_ZCODE").as_deref(),
        Ok("1"),
        "installed ZCode probe requires its explicit gate"
    );
    let (local, target, _, _, _, execution_host_id) = live_host();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-zcode-installed-version").expect("request id"),
        ScopeId::new("live-zcode-installed-version").expect("scope id"),
        execution_host_id.clone(),
        target,
        local.deadline_after(Duration::from_secs(5)),
        DiscoveryCancellation::new(),
    );
    let driver = ZcodeAppServerDriver::new(
        EnvironmentRef::new("live.zcode.host-config").expect("environment"),
        ZcodeAppServerMode::plan(),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed ZCode discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed payload produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        ZCODE_RELEASE_VERSION
    );
    assert_eq!(observation.version().axis().as_str(), ZCODE_RELEASE_AXIS);
    assert_eq!(observation.claim_id(), zcode_app_server_claim().id());
    assert!(observation.is_qualified());
}

#[test]
#[ignore = "requires explicit live gate, exact payload, host settings, cwd, mode, provider, and model"]
fn configured_zcode_app_server_completes_one_prompt_through_prepared_facade() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_ZCODE_PROMPT").as_deref(),
        Ok("1"),
        "configured ZCode probe requires its explicit gate"
    );
    let provider = std::env::var("SWALLOWTAIL_ZCODE_PROVIDER").expect("explicit live provider");
    let model = std::env::var("SWALLOWTAIL_ZCODE_MODEL").expect("explicit live model");
    let mode = ZcodeAppServerMode::new(
        &std::env::var("SWALLOWTAIL_ZCODE_MODE").expect("explicit live mode"),
    )
    .expect("live mode is host-supplied plan or build");
    let (local, target, interpreter, environment, working_resource, execution_host_id) =
        live_host();
    let access_id = AccessProfileId::new("live.zcode.host-config").expect("access id");
    let prepared = block_on(prepare_zcode_app_server(
        ZcodePreparationInput::new(
            ConfiguredInstanceId::new("live.zcode.instance").expect("instance id"),
            InstanceRevision::new(ZCODE_RELEASE_VERSION).expect("instance revision"),
            execution_host_id,
            interpreter,
            target,
            environment,
            zcode_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        ZcodePreparationProbe::new(
            RequestId::new("live-zcode-prepare").expect("request id"),
            ScopeId::new("live-zcode-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed ZCode prepares");
    let run = prepared
        .prepare_run(ZcodeRunProfileInput::new(
            RequestId::new("live-zcode-prompt").expect("request id"),
            ZcodeModelSelection::new(
                ModelRouteId::new("live.zcode.route").expect("route id"),
                ModelRouteRevision::new(ZCODE_RELEASE_VERSION).expect("route revision"),
                ProviderId::new(provider).expect("provider id"),
                ModelId::new(model).expect("model id"),
            ),
            mode,
            OperationContent::new("Reply exactly ZCODE_LIVE_OK.").expect("prompt"),
            working_resource,
            local.deadline_after(Duration::from_secs(180)),
        ))
        .expect("ZCode run prepares");

    let mut handle = block_on(run.start_run(local.services().clone())).expect("ZCode run starts");
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live ZCode event remains valid");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert!(
        outcome.output().is_some(),
        "live run returns assistant text"
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

fn live_host() -> (
    LocalHostServices,
    InstalledExecutableTarget,
    ExecutableRef,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let executable_path = PathBuf::from(
        std::env::var_os("SWALLOWTAIL_ZCODE_EXECUTABLE").expect("exact zcode.cjs payload path"),
    );
    assert_eq!(
        executable_path.file_name().and_then(|value| value.to_str()),
        Some(ZCODE_EXECUTABLE_BASENAME),
        "the live executable must be the exact packaged runtime basename"
    );
    let settings_path = PathBuf::from(
        std::env::var_os("SWALLOWTAIL_ZCODE_SETTINGS").expect("host-approved settings path"),
    );
    assert!(
        settings_path.is_file(),
        "host-approved settings path is a file"
    );
    let cwd =
        PathBuf::from(std::env::var_os("SWALLOWTAIL_ZCODE_CWD").expect("host-approved ZCode cwd"));
    assert!(cwd.is_dir(), "host-approved ZCode cwd is a directory");

    let node = std::env::var_os("SWALLOWTAIL_ZCODE_NODE")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("PATH")
                .into_iter()
                .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
                .map(|directory| directory.join("node"))
                .find(|candidate| candidate.is_file())
        })
        .expect("ZCode live probe requires a host-approved Node interpreter");
    let node = std::fs::canonicalize(node).expect("Node interpreter resolves exactly");
    let interpreter =
        ExecutableRef::new(node.to_string_lossy().to_string()).expect("interpreter ref is valid");
    let environment = EnvironmentRef::new(settings_path.to_string_lossy().into_owned())
        .expect("settings path is a valid environment reference");
    let working_resource = WorkingResourceRef::new(cwd.to_string_lossy().to_string())
        .expect("working resource is valid");
    let execution_host_id = ExecutionHostId::new("live.zcode.local-host").expect("host id");
    let executable = ExecutableRef::new(executable_path.to_string_lossy().to_string())
        .expect("executable ref is valid");
    let axis = InterfaceVersionAxis::new(ZCODE_RELEASE_AXIS).expect("release axis");
    let launch = LocalExecutableLaunch::interpreted_script(node, executable_path);
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(executable, axis, launch);
    let isolated_home = settings_path
        .parent()
        .expect("settings file has a parent directory")
        .join("zcode-live-home");
    if isolated_home.exists() {
        std::fs::remove_dir_all(&isolated_home).expect("previous isolated HOME is removable");
    }
    let config_dir = isolated_home.join(".zcode").join("cli");
    std::fs::create_dir_all(&config_dir).expect("isolated HOME config directory is writable");
    std::fs::copy(&settings_path, config_dir.join("config.json"))
        .expect("host-approved settings are placed at the payload HOME path");
    let mut environment_values = vec![(OsString::from("HOME"), isolated_home.into_os_string())];
    if let Some(path) = std::env::var_os("PATH") {
        environment_values.push((OsString::from("PATH"), path));
    }
    let local = builder
        .approve_environment(environment.clone(), environment_values)
        .approve_working_resource(working_resource.clone(), cwd)
        .build_services(execution_host_id.clone());
    (
        local,
        target,
        interpreter,
        environment,
        working_resource,
        execution_host_id,
    )
}
