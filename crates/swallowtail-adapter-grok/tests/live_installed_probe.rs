use futures_executor::block_on;
use std::path::PathBuf;
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_AXIS, GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, GrokAcpDriver,
    grok_build_acp_claim,
};
use swallowtail_core::{CredentialRef, DiscoveryStatus, ExecutionHostId, InterfaceVersionAxis};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, MonotonicInstant, RequestId, ScopeId,
};

const MAXIMUM_RUNTIME_NANOS: u64 = 5_000_000_000;

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_GROK=1 and an installed Grok Build CLI"]
fn installed_grok_build_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_GROK").as_deref(),
        Ok("1"),
        "live Grok probe requires its explicit gate"
    );
    let installed =
        std::fs::canonicalize(installed_path("grok").expect("Grok Build is installed on PATH"))
            .expect("Grok Build executable resolves exactly");

    let execution_host_id = ExecutionHostId::new("live.grok.local-host").expect("host id is valid");
    let executable = ExecutableRef::new("live.grok.installed").expect("executable ref is valid");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(
            executable,
            InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("Grok axis is valid"),
            installed,
        );
    let local = builder.build_services(execution_host_id.clone());
    let now = local
        .services()
        .time()
        .expect("local host has time service")
        .now();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-grok-installed-version").expect("request id is valid"),
        ScopeId::new("live-grok-installed-version").expect("scope id is valid"),
        execution_host_id.clone(),
        target,
        Deadline::at(MonotonicInstant::from_ticks(
            now.ticks().saturating_add(MAXIMUM_RUNTIME_NANOS),
        )),
        DiscoveryCancellation::new(),
    );
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("live.grok.ambient-state").expect("environment ref is valid"),
        CredentialRef::new("live.grok.unused-credential").expect("credential ref is valid"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed Grok Build discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed Grok Build produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(observation.claim_id(), grok_build_acp_claim().id());
    assert!(observation.is_qualified());
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
}
