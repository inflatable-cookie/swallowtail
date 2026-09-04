mod process;
mod task;
mod time;
pub mod watcher_proof;

#[allow(unused_imports)]
pub use process::{FakeProcessService, ObservedProcessRequest, ProcessCompleter};
pub use task::{FailingTaskService, TaskState, ThreadTaskService};
pub use time::{ControllableTimeService, ImmediateTimeService, PendingTimeService};

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_RESPONSE_ONLY_AXIS, ClaudeCodePreparationInput,
    ClaudeCodePreparationProbe, ClaudeCodeResponsePreparationInput,
    ClaudeCodeResponsePreparationProbe,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, HostServices,
    InstalledExecutableTarget, MonotonicInstant, PreparedAccessEvidence, ProcessRequest,
    ProcessService, RequestId, ScopeId, TimeService,
};

pub fn host_services(
    host: ExecutionHostId,
    process: Arc<dyn ProcessService>,
    time: Arc<dyn TimeService>,
) -> (HostServices, Arc<TaskState>) {
    let task = Arc::new(TaskState::default());
    let services = HostServices::new(host)
        .with_task(Arc::new(task::ThreadTaskService::new(Arc::clone(&task))))
        .with_process(process)
        .with_time(time);
    (services, task)
}

#[allow(dead_code)]
pub fn local_watcher_host(host: ExecutionHostId) -> LocalHostServices {
    static TEMPORARY_ROOT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    let temporary_root_sequence = TEMPORARY_ROOT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temporary_root = std::env::temp_dir().join(format!(
        "swallowtail-claude-code-watcher-fixture-{}-{temporary_root_sequence}",
        std::process::id()
    ));
    let sleep = ExecutableRef::new("watcher.sleep").expect("executable is valid");
    let complete = ExecutableRef::new("watcher.complete").expect("executable is valid");
    let sleep_operation =
        swallowtail_core::WatcherOperationData::new("sleep-operation").expect("operation is valid");
    let complete_operation = swallowtail_core::WatcherOperationData::new("exit-zero-operation")
        .expect("operation is valid");
    LocalProcessHost::builder(LocalProcessLimits::default())
        .with_temporary_root(temporary_root)
        .approve_executable(sleep.clone(), "/bin/sleep")
        .approve_executable(complete.clone(), "/usr/bin/true")
        .approve_watcher_operation(
            sleep_operation,
            ProcessRequest::new(sleep).with_arguments(["30".to_owned()]),
        )
        .approve_watcher_operation(complete_operation, ProcessRequest::new(complete))
        .build_services(host)
}

#[allow(dead_code)]
pub fn watcher_host_services(
    host: ExecutionHostId,
    process: Arc<dyn ProcessService>,
    time: Arc<dyn TimeService>,
    local: &LocalHostServices,
) -> (HostServices, Arc<TaskState>) {
    let task = Arc::new(TaskState::default());
    let services = HostServices::new(host)
        .with_task(Arc::new(task::ThreadTaskService::new(Arc::clone(&task))))
        .with_process(process)
        .with_time(time)
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
    (services, task)
}

pub fn preparation_input(host: ExecutionHostId) -> ClaudeCodePreparationInput {
    let access = access_profile();
    ClaudeCodePreparationInput::new(
        ConfiguredInstanceId::new("claude-code.headless.fixture").expect("instance is valid"),
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude.fixture.executable").expect("executable is valid"),
            InterfaceVersionAxis::new(CLAUDE_CODE_HEADLESS_AXIS).expect("axis is valid"),
        ),
        EnvironmentRef::new("claude.fixture.local-subscription-environment")
            .expect("environment is valid"),
        access.clone(),
        PreparedAccessEvidence::caller_asserted(access_status(&access)),
    )
}

pub fn preparation_probe() -> ClaudeCodePreparationProbe {
    ClaudeCodePreparationProbe::new(
        RequestId::new("claude-code-preparation").expect("request is valid"),
        ScopeId::new("claude-code-preparation").expect("scope is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

#[allow(dead_code)]
pub fn response_preparation_input(host: ExecutionHostId) -> ClaudeCodeResponsePreparationInput {
    let access = access_profile();
    ClaudeCodeResponsePreparationInput::new(
        ConfiguredInstanceId::new("claude-code.response-only.fixture").expect("instance is valid"),
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude.fixture.executable").expect("executable is valid"),
            InterfaceVersionAxis::new(CLAUDE_CODE_RESPONSE_ONLY_AXIS).expect("axis is valid"),
        ),
        EnvironmentRef::new("claude.fixture.local-subscription-environment")
            .expect("environment is valid"),
        access.clone(),
        PreparedAccessEvidence::caller_asserted(access_status(&access)),
    )
}

#[allow(dead_code)]
pub fn response_preparation_probe() -> ClaudeCodeResponsePreparationProbe {
    ClaudeCodeResponsePreparationProbe::new(
        RequestId::new("claude-code-response-preparation").expect("request is valid"),
        ScopeId::new("claude-code-response-preparation").expect("scope is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

pub fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("claude-code.local-subscription").expect("access id is valid"),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("anthropic-claude-code").expect("audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

fn access_status(access: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        access.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

pub fn fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/claude-code-2.1.220")
        .join(name);
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
}

#[allow(dead_code)]
pub fn response_fixture(name: &str) -> String {
    response_fixture_at("2.1.228", name)
}

#[allow(dead_code)]
pub fn response_fixture_at(version: &str, name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/claude-code-{version}"))
        .join(name);
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
}
