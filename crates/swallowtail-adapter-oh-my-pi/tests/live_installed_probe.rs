use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
use swallowtail_adapter_oh_my_pi::{
    OH_MY_PI_PACKAGE_AXIS, OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, OhMyPiCatalogueProfileInput,
    OhMyPiModelSelection, OhMyPiPreparationInput, OhMyPiPreparationProbe, OhMyPiRpcDriver,
    OhMyPiRunProfileInput, oh_my_pi_rpc_claim, prepare_oh_my_pi_rpc,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DiscoveryStatus, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_host_local::{
    LocalExecutableLaunch, LocalHostServices, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, MonotonicInstant, OperationContent,
    PreparedAccessEvidence, RequestId, ScopeId, TerminalStatus, WorkingResourceRef,
};

const MAXIMUM_SHEBANG_BYTES: u64 = 64;
const MAXIMUM_RUNTIME_NANOS: u64 = 5_000_000_000;
const LIVE_PROVIDER: &str = "openai-codex";
const LIVE_MODEL: &str = "gpt-5.6-luna";
const LIVE_REASONING: &str = "low";

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_OMP=1 and an installed OMP CLI"]
fn installed_omp_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_OMP").as_deref(),
        Ok("1"),
        "live OhMyPi probe requires its explicit gate"
    );
    let selected_omp = installed_path("omp").expect("OhMyPi is installed on PATH");
    let script = std::fs::canonicalize(selected_omp).expect("OhMyPi launcher resolves exactly");
    assert_eq!(bounded_shebang(&script), "#!/usr/bin/env bun");
    let bun = std::fs::canonicalize(installed_path("bun").expect("Bun is installed on PATH"))
        .expect("Bun interpreter resolves exactly");

    let execution_host_id = ExecutionHostId::new("live.omp.local-host").expect("host id is valid");
    let executable = ExecutableRef::new("live.omp.installed").expect("executable ref is valid");
    let launch =
        LocalExecutableLaunch::new(bun).with_prefix_arguments([OsString::from(script.as_os_str())]);
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(OH_MY_PI_PACKAGE_AXIS).expect("OhMyPi axis is valid"),
            launch,
        );
    let local = builder.build_services(execution_host_id.clone());
    let now = local
        .services()
        .time()
        .expect("local host has time service")
        .now();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-omp-installed-version").expect("request id is valid"),
        ScopeId::new("live-omp-installed-version").expect("scope id is valid"),
        execution_host_id.clone(),
        target,
        Deadline::at(MonotonicInstant::from_ticks(
            now.ticks().saturating_add(MAXIMUM_RUNTIME_NANOS),
        )),
        DiscoveryCancellation::new(),
    );
    let driver = OhMyPiRpcDriver::new(
        EnvironmentRef::new("live.omp.unused-environment").expect("environment ref is valid"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed OhMyPi discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed OhMyPi produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(observation.claim_id(), oh_my_pi_rpc_claim().id());
    assert!(observation.is_qualified());
}

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_OMP_PROMPT=1, local OMP auth, and one paid/subscription-backed model turn"]
fn configured_omp_completes_luna_low_through_the_prepared_facade() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_OMP_PROMPT").as_deref(),
        Ok("1"),
        "authenticated OhMyPi probe requires its explicit gate"
    );
    let (local, target, environment, working_resource, execution_host_id) = live_host();
    let access_profile_id =
        AccessProfileId::new("live.omp.local-auth").expect("access id is valid");
    let access_profile = AccessProfile::new(
        access_profile_id.clone(),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("oh-my-pi-harness").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access_profile_id,
        CredentialState::NotRequired,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    ));
    let prepared = block_on(prepare_oh_my_pi_rpc(
        OhMyPiPreparationInput::new(
            ConfiguredInstanceId::new("live.omp.instance").expect("instance id is valid"),
            InstanceRevision::new("17.2.9-live").expect("instance revision is valid"),
            execution_host_id,
            target,
            environment,
            access_profile,
            access_evidence,
        ),
        OhMyPiPreparationProbe::new(
            RequestId::new("live-omp-prepare").expect("request id is valid"),
            ScopeId::new("live-omp-prepare").expect("scope id is valid"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed authenticated OhMyPi prepares");

    let catalogue = prepared
        .prepare_catalogue(
            OhMyPiCatalogueProfileInput::new(
                RequestId::new("live-omp-catalogue").expect("request id is valid"),
            )
            .with_deadline(local.deadline_after(Duration::from_secs(30))),
        )
        .expect("authenticated OhMyPi catalogue prepares");
    let models = block_on(catalogue.list_models(local.services().clone()))
        .expect("authenticated OhMyPi catalogue loads");
    let selected = models
        .iter()
        .find(|model| {
            model.id().as_str() == LIVE_MODEL
                && model.provider_id().map(ProviderId::as_str) == Some(LIVE_PROVIDER)
        })
        .expect("authenticated OhMyPi catalogue exposes the selected Luna model");
    assert_eq!(
        selected
            .metadata()
            .catalog_observations()
            .and_then(|observations| observations.reasoning_supported()),
        Some(true)
    );

    let run = prepared
        .prepare_run(
            OhMyPiRunProfileInput::new(
                RequestId::new("live-omp-luna-low").expect("request id is valid"),
                OhMyPiModelSelection::new(
                    ModelRouteId::new("live.omp.openai-codex.luna")
                        .expect("model route id is valid"),
                    ModelRouteRevision::new("17.2.9-live").expect("model route revision is valid"),
                    ProviderId::new(LIVE_PROVIDER).expect("provider id is valid"),
                    ModelId::new(LIVE_MODEL).expect("model id is valid"),
                ),
                OperationContent::new("Reply exactly OMP_LIVE_OK. Do not use tools.")
                    .expect("prompt is valid"),
                working_resource,
                local.deadline_after(Duration::from_secs(90)),
            )
            .with_reasoning_mode(
                ReasoningMode::new(LIVE_REASONING).expect("reasoning mode is valid"),
            ),
        )
        .expect("Luna low run prepares");
    assert_eq!(
        run.plan().provider_id().map(ProviderId::as_str),
        Some(LIVE_PROVIDER)
    );
    assert_eq!(run.plan().model_id().map(ModelId::as_str), Some(LIVE_MODEL));
    assert_eq!(
        run.request()
            .policy()
            .reasoning_mode()
            .map(ReasoningMode::as_str),
        Some(LIVE_REASONING)
    );

    let mut handle = block_on(run.start_run(local.services().clone()))
        .expect("authenticated Luna low run starts");
    let mut events = handle.take_events().expect("run exposes events");
    let terminal = handle
        .take_terminal_outcome()
        .expect("run exposes one terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live Luna event remains valid");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome
            .output()
            .map(OperationContent::as_str)
            .map(str::trim),
        Some("OMP_LIVE_OK")
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
    let selected_omp = installed_path("omp").expect("OhMyPi is installed on PATH");
    let script = std::fs::canonicalize(selected_omp).expect("OhMyPi launcher resolves exactly");
    assert_eq!(bounded_shebang(&script), "#!/usr/bin/env bun");
    let bun = std::fs::canonicalize(installed_path("bun").expect("Bun is installed on PATH"))
        .expect("Bun interpreter resolves exactly");
    let environment =
        EnvironmentRef::new("live.omp.local-auth-environment").expect("environment is valid");
    let working_resource =
        WorkingResourceRef::new("live.omp.read-only-workspace").expect("resource is valid");
    let execution_host_id = ExecutionHostId::new("live.omp.local-host").expect("host id is valid");
    let executable = ExecutableRef::new("live.omp.installed").expect("executable ref is valid");
    let launch =
        LocalExecutableLaunch::new(bun).with_prefix_arguments([OsString::from(script.as_os_str())]);
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(OH_MY_PI_PACKAGE_AXIS).expect("OhMyPi axis is valid"),
            launch,
        );
    let home = std::env::var_os("HOME").expect("local OMP auth requires HOME");
    let local = builder
        .approve_environment(environment.clone(), [(OsString::from("HOME"), home)])
        .approve_working_resource(
            working_resource.clone(),
            std::env::current_dir().expect("live probe has a working directory"),
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
        .expect("OhMyPi script is readable")
        .take(MAXIMUM_SHEBANG_BYTES)
        .read_to_end(&mut bytes)
        .expect("OhMyPi shebang remains bounded");
    let first_line = bytes
        .split(|byte| *byte == b'\n')
        .next()
        .expect("OhMyPi script has a first line");
    std::str::from_utf8(first_line)
        .expect("OhMyPi shebang is UTF-8")
        .to_owned()
}
