use futures_executor::block_on;
use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use swallowtail_adapter_pi::{
    PI_PACKAGE_AXIS, PI_PACKAGE_LATEST_QUALIFIED_VERSION, PiRpcDriver, pi_rpc_claim,
};
use swallowtail_core::{CredentialRef, DiscoveryStatus, ExecutionHostId, InterfaceVersionAxis};
use swallowtail_host_local::{LocalExecutableLaunch, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, MonotonicInstant, RequestId, ScopeId,
};

const MAXIMUM_SHEBANG_BYTES: u64 = 64;
const MAXIMUM_RUNTIME_NANOS: u64 = 5_000_000_000;

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_PI=1 and an installed Pi CLI"]
fn installed_pi_is_exactly_classified() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_PI").as_deref(),
        Ok("1"),
        "live Pi probe requires its explicit gate"
    );
    let selected_pi = installed_path("pi").expect("Pi is installed on PATH");
    let script = std::fs::canonicalize(selected_pi).expect("Pi launcher resolves exactly");
    assert_eq!(bounded_shebang(&script), "#!/usr/bin/env node");
    let node = std::fs::canonicalize(installed_path("node").expect("Node is installed on PATH"))
        .expect("Node interpreter resolves exactly");

    let execution_host_id = ExecutionHostId::new("live.pi.local-host").expect("host id is valid");
    let executable = ExecutableRef::new("live.pi.installed").expect("executable ref is valid");
    let launch = LocalExecutableLaunch::new(node)
        .with_prefix_arguments([OsString::from(script.as_os_str())]);
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(PI_PACKAGE_AXIS).expect("Pi axis is valid"),
            launch,
        );
    let local = builder.build_services(execution_host_id.clone());
    let now = local
        .services()
        .time()
        .expect("local host has time service")
        .now();
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("live-pi-installed-version").expect("request id is valid"),
        ScopeId::new("live-pi-installed-version").expect("scope id is valid"),
        execution_host_id.clone(),
        target,
        Deadline::at(MonotonicInstant::from_ticks(
            now.ticks().saturating_add(MAXIMUM_RUNTIME_NANOS),
        )),
        DiscoveryCancellation::new(),
    );
    let driver = PiRpcDriver::new(
        EnvironmentRef::new("live.pi.unused-environment").expect("environment ref is valid"),
        CredentialRef::new("live.pi.unused-credential").expect("credential ref is valid"),
    );
    let outcome = block_on(driver.discover_installed_executable(request, local.services().clone()))
        .expect("installed Pi discovery completes");

    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    let observation = outcome
        .installed_executable_observation()
        .expect("installed Pi produces one observation");
    assert_eq!(observation.execution_host_id(), &execution_host_id);
    assert_eq!(
        observation.version().version().as_str(),
        PI_PACKAGE_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(observation.claim_id(), pi_rpc_claim().id());
    assert!(observation.is_qualified());
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
        .expect("Pi script is readable")
        .take(MAXIMUM_SHEBANG_BYTES)
        .read_to_end(&mut bytes)
        .expect("Pi shebang remains bounded");
    let first_line = bytes
        .split(|byte| *byte == b'\n')
        .next()
        .expect("Pi script has a first line");
    std::str::from_utf8(first_line)
        .expect("Pi shebang is UTF-8")
        .to_owned()
}
