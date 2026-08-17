use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;
use swallowtail_adapter_deepseek_harness::{
    DEEPSEEK_HARNESS_EXECUTABLE_BASENAME, DEEPSEEK_HARNESS_RELEASE_AXIS,
    DEEPSEEK_HARNESS_RELEASE_VERSION, DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME,
    DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, DEEPSEEK_HARNESS_WEB_RELEASE_VERSION,
    DeepSeekHarnessModelSelection, DeepSeekHarnessPreparationInput,
    DeepSeekHarnessPreparationProbe, DeepSeekHarnessRunProfileInput,
    DeepSeekHarnessWebModelSelection, DeepSeekHarnessWebPreparationInput,
    DeepSeekHarnessWebPreparationProbe, DeepSeekHarnessWebRunProfileInput,
    deepseek_harness_access_profile, deepseek_harness_jsonrpc_claim, deepseek_harness_web_claim,
    prepare_deepseek_harness_jsonrpc, prepare_deepseek_harness_web,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, DiscoveryStatus,
    EndpointAudience, EndpointAuthorization, EntitlementState, ExecutionHostId, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    DiscoveryCancellation, DiscoveryDriver, EndpointRef, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, OperationContent, PreparedAccessEvidence, RequestId,
    ScopeId, TerminalStatus, WorkingResourceRef,
};

#[test]
#[ignore = "requires explicit live gate, exact executable, Cordis config, and cwd"]
fn installed_deepseek_harness_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS").as_deref(),
        Ok("1"),
        "installed DeepSeek Harness probe requires its explicit gate"
    );
    let (local, target, _, _, execution_host_id) = live_host();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-deepseek-harness-installed-version").expect("request id"),
        ScopeId::new("live-deepseek-harness-installed-version").expect("scope id"),
        execution_host_id.clone(),
        target,
        local.deadline_after(Duration::from_secs(5)),
        DiscoveryCancellation::new(),
    );
    let driver = swallowtail_adapter_deepseek_harness::DeepSeekHarnessJsonRpcDriver::new(
        EnvironmentRef::new("live.deepseek-harness.cordis").expect("environment"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed DeepSeek Harness discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed payload produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        DEEPSEEK_HARNESS_RELEASE_VERSION
    );
    assert_eq!(
        observation.claim_id(),
        deepseek_harness_jsonrpc_claim().id()
    );
    assert!(observation.is_qualified());
}

#[test]
#[ignore = "requires explicit live gate, exact executable, Cordis config, cwd, provider, and model"]
fn configured_deepseek_harness_completes_one_prompt_through_prepared_facade() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS_PROMPT").as_deref(),
        Ok("1"),
        "configured DeepSeek Harness probe requires its explicit gate"
    );
    let provider =
        std::env::var("SWALLOWTAIL_DEEPSEEK_HARNESS_PROVIDER").expect("explicit live provider");
    let model = std::env::var("SWALLOWTAIL_DEEPSEEK_HARNESS_MODEL").expect("explicit live model");
    let (local, target, environment, working_resource, execution_host_id) = live_host();
    let access_id = AccessProfileId::new("live.deepseek-harness.host-config").expect("access id");
    let prepared = block_on(prepare_deepseek_harness_jsonrpc(
        DeepSeekHarnessPreparationInput::new(
            ConfiguredInstanceId::new("live.deepseek-harness.instance").expect("instance id"),
            InstanceRevision::new(DEEPSEEK_HARNESS_RELEASE_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            deepseek_harness_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        DeepSeekHarnessPreparationProbe::new(
            RequestId::new("live-deepseek-harness-prepare").expect("request id"),
            ScopeId::new("live-deepseek-harness-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed DeepSeek Harness prepares");
    let run = prepared
        .prepare_run(DeepSeekHarnessRunProfileInput::new(
            RequestId::new("live-deepseek-harness-prompt").expect("request id"),
            DeepSeekHarnessModelSelection::new(
                ModelRouteId::new("live.deepseek-harness.route").expect("route id"),
                ModelRouteRevision::new(DEEPSEEK_HARNESS_RELEASE_VERSION).expect("route revision"),
                ProviderId::new(provider).expect("provider id"),
                ModelId::new(model).expect("model id"),
            ),
            OperationContent::new("Reply exactly DEEPSEEK_HARNESS_LIVE_OK.").expect("prompt"),
            working_resource,
            local.deadline_after(Duration::from_secs(180)),
        ))
        .expect("DeepSeek Harness run prepares");

    let mut handle =
        block_on(run.start_run(local.services().clone())).expect("DeepSeek Harness run starts");
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live DeepSeek Harness event remains valid");
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

#[test]
#[ignore = "requires explicit live gate, exact dsh executable, Cordis config, and cwd"]
fn installed_deepseek_harness_web_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS_WEB").as_deref(),
        Ok("1"),
        "installed DeepSeek Harness Web probe requires its explicit gate"
    );
    let (local, target, _, _, execution_host_id) = live_web_host();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-deepseek-harness-web-installed-version").expect("request id"),
        ScopeId::new("live-deepseek-harness-web-installed-version").expect("scope id"),
        execution_host_id.clone(),
        target,
        local.deadline_after(Duration::from_secs(5)),
        DiscoveryCancellation::new(),
    );
    let driver = swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebDriver::new(
        EnvironmentRef::new("live.deepseek-harness.cordis").expect("environment"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed DeepSeek Harness Web discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed payload produces one Web observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        DEEPSEEK_HARNESS_WEB_RELEASE_VERSION
    );
    assert_eq!(
        observation.version().axis().as_str(),
        DEEPSEEK_HARNESS_WEB_RELEASE_AXIS
    );
    assert_eq!(observation.claim_id(), deepseek_harness_web_claim().id());
    assert!(observation.is_qualified());
}

#[test]
#[ignore = "requires explicit live gate, exact dsh executable, Cordis config, cwd, provider, and model"]
fn configured_deepseek_harness_web_completes_one_prompt_through_prepared_facade() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS_WEB_PROMPT").as_deref(),
        Ok("1"),
        "configured DeepSeek Harness Web probe requires its explicit gate"
    );
    let provider =
        std::env::var("SWALLOWTAIL_DEEPSEEK_HARNESS_PROVIDER").expect("explicit live provider");
    let model = std::env::var("SWALLOWTAIL_DEEPSEEK_HARNESS_MODEL").expect("explicit live model");
    let (local, target, environment, working_resource, execution_host_id) = live_web_host();
    let access_id = AccessProfileId::new("live.deepseek-harness.host-config").expect("access id");
    let prepared = block_on(prepare_deepseek_harness_web(
        DeepSeekHarnessWebPreparationInput::new(
            ConfiguredInstanceId::new("live.deepseek-harness.web.instance").expect("instance id"),
            InstanceRevision::new(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            deepseek_harness_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        DeepSeekHarnessWebPreparationProbe::new(
            RequestId::new("live-deepseek-harness-web-prepare").expect("request id"),
            ScopeId::new("live-deepseek-harness-web-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed DeepSeek Harness Web prepares");
    let run = prepared
        .prepare_run(DeepSeekHarnessWebRunProfileInput::new(
            RequestId::new("live-deepseek-harness-web-prompt").expect("request id"),
            DeepSeekHarnessWebModelSelection::new(
                ModelRouteId::new("live.deepseek-harness.web.route").expect("route id"),
                ModelRouteRevision::new(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION)
                    .expect("route revision"),
                ProviderId::new(provider).expect("provider id"),
                ModelId::new(model).expect("model id"),
            ),
            OperationContent::new("Reply exactly DEEPSEEK_HARNESS_WEB_LIVE_OK.").expect("prompt"),
            working_resource,
            local.deadline_after(Duration::from_secs(180)),
        ))
        .expect("DeepSeek Harness Web run prepares");

    let mut handle =
        block_on(run.start_run(local.services().clone())).expect("DeepSeek Harness Web run starts");
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live DeepSeek Harness Web event remains valid");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert!(
        outcome.output().is_some(),
        "live Web run returns assistant text"
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
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    live_host_for(
        DEEPSEEK_HARNESS_EXECUTABLE_BASENAME,
        DEEPSEEK_HARNESS_RELEASE_AXIS,
        false,
    )
}

fn live_web_host() -> (
    LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    live_host_for(
        DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME,
        DEEPSEEK_HARNESS_WEB_RELEASE_AXIS,
        true,
    )
}

fn live_host_for(
    expected_basename: &str,
    release_axis: &str,
    approve_web_endpoint: bool,
) -> (
    LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    WorkingResourceRef,
    ExecutionHostId,
) {
    let executable_path = PathBuf::from(
        std::env::var_os("SWALLOWTAIL_DEEPSEEK_HARNESS_EXECUTABLE")
            .expect("exact DeepSeek Harness executable path"),
    );
    assert_eq!(
        executable_path.file_name().and_then(|value| value.to_str()),
        Some(expected_basename),
        "the live executable must be the exact packaged runtime basename"
    );
    let cordis_path = std::env::var_os("SWALLOWTAIL_DEEPSEEK_HARNESS_CORDIS")
        .expect("host-approved Cordis config path");
    let cwd = PathBuf::from(
        std::env::var_os("SWALLOWTAIL_DEEPSEEK_HARNESS_CWD")
            .expect("host-approved DeepSeek Harness cwd"),
    );
    assert!(
        cwd.is_dir(),
        "host-approved DeepSeek Harness cwd is a directory"
    );

    let environment =
        EnvironmentRef::new("live.deepseek-harness.cordis").expect("environment is valid");
    let working_resource = WorkingResourceRef::new(cwd.to_string_lossy().to_string())
        .expect("working resource is valid");
    let execution_host_id =
        ExecutionHostId::new("live.deepseek-harness.local-host").expect("host id");
    let executable = ExecutableRef::new(executable_path.to_string_lossy().to_string())
        .expect("executable ref is valid");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(
            executable,
            InterfaceVersionAxis::new(release_axis).expect("release axis"),
            executable_path,
        );
    let home = std::env::var_os("HOME").expect("DeepSeek Harness live probe requires HOME");
    let mut environment_values = vec![
        (OsString::from("HOME"), home),
        (OsString::from("DSH_CORDIS_CONFIG"), cordis_path),
        (OsString::from("DSH_CWD"), cwd.clone().into_os_string()),
    ];
    if let Some(provider_key) = std::env::var_os("OLLAMA_API_KEY") {
        environment_values.push((OsString::from("OLLAMA_API_KEY"), provider_key));
    }
    let builder = if approve_web_endpoint {
        builder.approve_endpoint(
            EndpointRef::new(target.executable().as_host_value()).expect("endpoint reference"),
            EndpointAudience::new("deepseek-harness.host-config").expect("endpoint audience"),
            "http://127.0.0.1:3080",
        )
    } else {
        builder
    };
    let local = builder
        .approve_environment(environment.clone(), environment_values)
        .approve_working_resource(working_resource.clone(), cwd)
        .build_services(execution_host_id.clone());
    (
        local,
        target,
        environment,
        working_resource,
        execution_host_id,
    )
}
