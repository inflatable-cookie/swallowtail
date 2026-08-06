use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;
use swallowtail_adapter_muse::{
    MUSE_CODE_PAYLOAD_BASENAME, MUSE_CODE_RELEASE_AXIS, MUSE_CODE_RELEASE_REVISION,
    MUSE_META_PROVIDER_ID, MUSE_SPARK_MODEL_ID, MuseHeadlessDriver, MuseHeadlessModelSelection,
    MusePreparationInput, MusePreparationProbe, MuseRunProfileInput, muse_headless_claim,
    muse_local_meta_account_access_profile, prepare_muse_headless,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, DiscoveryStatus,
    EndpointAuthorization, EntitlementState, ExecutionHostId, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, OperationContent, PreparedAccessEvidence, RequestId,
    ScopeId, TerminalStatus, WorkingResourceRef,
};

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_MUSE=1 and the exact installed Muse Code payload"]
fn installed_muse_payload_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_MUSE").as_deref(),
        Ok("1"),
        "installed Muse Code probe requires its explicit gate"
    );
    let (local, target, _, _, execution_host_id) = live_host();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-muse-installed-version").expect("request id"),
        ScopeId::new("live-muse-installed-version").expect("scope id"),
        execution_host_id.clone(),
        target,
        local.deadline_after(Duration::from_secs(5)),
        DiscoveryCancellation::new(),
    );
    let driver = MuseHeadlessDriver::new(
        EnvironmentRef::new("live.muse.unused-environment").expect("environment"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed Muse Code discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed payload produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        MUSE_CODE_RELEASE_REVISION
    );
    assert_eq!(observation.claim_id(), muse_headless_claim().id());
    assert!(observation.is_qualified());
}

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_MUSE_PROMPT=1, local Meta account state, and one subscription-backed turn"]
fn configured_muse_completes_spark_low_through_the_prepared_facade() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_MUSE_PROMPT").as_deref(),
        Ok("1"),
        "authenticated Muse Code probe requires its explicit gate"
    );
    let (local, target, environment, working_resource, execution_host_id) = live_host();
    let access_id = AccessProfileId::new("live.muse.local-meta-account").expect("access id");
    let prepared = block_on(prepare_muse_headless(
        MusePreparationInput::new(
            ConfiguredInstanceId::new("live.muse.instance").expect("instance id"),
            InstanceRevision::new(MUSE_CODE_RELEASE_REVISION).expect("instance revision"),
            execution_host_id,
            target,
            environment,
            muse_local_meta_account_access_profile(access_id.clone()),
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        MusePreparationProbe::new(
            RequestId::new("live-muse-prepare").expect("request id"),
            ScopeId::new("live-muse-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed authenticated Muse Code prepares");
    let run = prepared
        .prepare_run(MuseRunProfileInput::new(
            RequestId::new("live-muse-spark-low").expect("request id"),
            MuseHeadlessModelSelection::new(
                ModelRouteId::new("live.muse.meta.spark").expect("route id"),
                ModelRouteRevision::new(MUSE_CODE_RELEASE_REVISION).expect("route revision"),
                ProviderId::new(MUSE_META_PROVIDER_ID).expect("provider id"),
                ModelId::new(MUSE_SPARK_MODEL_ID).expect("model id"),
            ),
            OperationContent::new("Reply exactly MUSE_LIVE_OK. Do not use tools.").expect("prompt"),
            ReasoningMode::new("low").expect("reasoning"),
            working_resource,
            local.deadline_after(Duration::from_secs(90)),
        ))
        .expect("Muse Spark low run prepares");

    let mut handle = block_on(run.start_run(local.services().clone()))
        .expect("authenticated Muse Spark run starts");
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live Muse event remains valid");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome
            .output()
            .map(OperationContent::as_str)
            .map(str::trim),
        Some("MUSE_LIVE_OK")
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
    let payload = installed_payload().expect("exact Muse Code payload is installed");
    assert_eq!(
        payload.file_name().and_then(std::ffi::OsStr::to_str),
        Some(MUSE_CODE_PAYLOAD_BASENAME)
    );
    let environment =
        EnvironmentRef::new("live.muse.local-account-environment").expect("environment is valid");
    let working_resource =
        WorkingResourceRef::new("live.muse.read-only-workspace").expect("resource is valid");
    let execution_host_id = ExecutionHostId::new("live.muse.local-host").expect("host id");
    let executable = ExecutableRef::new(payload.to_string_lossy()).expect("executable ref");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(
            executable,
            InterfaceVersionAxis::new(MUSE_CODE_RELEASE_AXIS).expect("release axis"),
            payload,
        );
    let home = std::env::var_os("HOME").expect("local Muse auth requires HOME");
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

fn installed_payload() -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(MUSE_CODE_PAYLOAD_BASENAME))
        .find(|candidate| candidate.is_file())
        .and_then(|candidate| std::fs::canonicalize(candidate).ok())
}
