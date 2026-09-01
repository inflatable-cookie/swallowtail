#[path = "watcher_service/feed.rs"]
mod feed;
#[path = "watcher_service/lifecycle.rs"]
mod lifecycle;
#[path = "watcher_service/policy.rs"]
mod policy;
#[path = "watcher_service/scoped_task.rs"]
mod scoped_task;
#[allow(dead_code)]
#[path = "local_process/support.rs"]
mod support;
#[path = "watcher_service/wait.rs"]
mod wait;

use swallowtail_core::{ExecutionHostId, WatcherOperationData, WatcherOwningTurn};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::ProcessRequest;

fn watcher_host(mode: &str, capacity: usize) -> LocalHostServices {
    watcher_host_with_limits(mode, capacity, LocalProcessLimits::default())
}

fn watcher_host_with_limits(
    mode: &str,
    capacity: usize,
    limits: LocalProcessLimits,
) -> LocalHostServices {
    let executable = support::executable_ref();
    let environment = support::environment_ref();
    let operation = operation_data(&format!("{mode}-operation"));
    let request = ProcessRequest::new(executable.clone())
        .with_arguments(support::fixture_arguments())
        .with_environment([environment.clone()]);
    LocalProcessHost::builder(limits)
        .with_watcher_capacity(capacity)
        .approve_executable(
            executable,
            std::env::current_exe().expect("watcher fixture executable"),
        )
        .approve_environment(environment, support::fixture_environment(mode))
        .approve_watcher_operation(operation, request)
        .build_services(
            ExecutionHostId::new(format!("fixture.host.watcher.{mode}"))
                .expect("watcher host id is valid"),
        )
}

fn default_watcher_host(capacity: usize) -> LocalHostServices {
    LocalProcessHost::builder(LocalProcessLimits::default())
        .with_watcher_capacity(capacity)
        .build_services(
            ExecutionHostId::new("fixture.host.watcher.default").expect("watcher host id is valid"),
        )
}

fn operation_data(value: &str) -> WatcherOperationData {
    WatcherOperationData::new(value).expect("watcher operation data is valid")
}

fn runtime_turn(value: &str) -> swallowtail_runtime::RuntimeTurnId {
    swallowtail_runtime::RuntimeTurnId::new(value).expect("runtime turn is valid")
}

fn watcher_owning_turn(value: &str) -> WatcherOwningTurn {
    WatcherOwningTurn::new(value).expect("watcher owning turn is valid")
}
