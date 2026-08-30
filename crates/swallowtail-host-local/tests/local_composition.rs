use futures_executor::block_on;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{ExecutionHostId, HostServiceKind, InterfaceVersionAxis};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits, LocalScopedTaskService};
use swallowtail_runtime::{
    CancellationControl, DiscoveryCancellation, ExecutableRef, ScopeId, ScopedTaskService,
};

#[test]
fn composition_exposes_one_exact_host_and_owned_service_set() {
    let host_id = host_id("fixture.host.local-composition");
    let local =
        LocalProcessHost::builder(LocalProcessLimits::default()).build_services(host_id.clone());

    assert_eq!(local.services().execution_host_id(), &host_id);
    assert_eq!(local.task_service().execution_host_id(), &host_id);
    assert_eq!(
        local.services().available_kinds(),
        BTreeSet::from([
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
            HostServiceKind::Network,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
            HostServiceKind::Attachment,
            HostServiceKind::ModelArtifact,
            HostServiceKind::ServingEndpoint,
            HostServiceKind::Schema,
            HostServiceKind::Watcher,
            HostServiceKind::WatcherBridge,
        ])
    );
}

#[test]
fn scoped_tasks_complete_cancel_reach_deadlines_and_join() {
    let host_id = host_id("fixture.host.joined-task");
    let local =
        LocalProcessHost::builder(LocalProcessLimits::default()).build_services(host_id.clone());
    let task_service = local.task_service().clone();

    let completed = Arc::new(AtomicBool::new(false));
    let completed_by_task = Arc::clone(&completed);
    let task = task_service
        .spawn(
            scope("complete"),
            Box::pin(async move {
                completed_by_task.store(true, Ordering::SeqCst);
            }),
        )
        .expect("task starts");
    block_on(task.join()).expect("task joins");
    assert!(completed.load(Ordering::SeqCst));

    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_by_task = Arc::clone(&cancelled);
    let task = task_service
        .spawn(
            scope("cancel"),
            Box::pin(async move {
                task_cancellation.wait_requested().await;
                cancelled_by_task.store(true, Ordering::SeqCst);
            }),
        )
        .expect("cancellable task starts");
    block_on(cancellation.request()).expect("cancellation request succeeds");
    block_on(task.join()).expect("cancelled task joins");
    assert!(cancelled.load(Ordering::SeqCst));

    let time = local
        .services()
        .time()
        .expect("composition includes time")
        .clone();
    let deadline = local.deadline_after(std::time::Duration::from_millis(2));
    let reached = Arc::new(AtomicBool::new(false));
    let reached_by_task = Arc::clone(&reached);
    let task = task_service
        .spawn(
            scope("deadline"),
            Box::pin(async move {
                let observation = time.wait_until(deadline).await;
                assert!(observation.observed_at() >= deadline.instant());
                reached_by_task.store(true, Ordering::SeqCst);
            }),
        )
        .expect("deadline task starts");
    block_on(task.join()).expect("deadline task joins");
    assert!(reached.load(Ordering::SeqCst));
}

#[test]
fn dropped_handles_join_and_panics_become_safe_cleanup_failures() {
    let service = LocalScopedTaskService::new(host_id("fixture.host.drop-join"));
    let completed = Arc::new(AtomicBool::new(false));
    let completed_by_task = Arc::clone(&completed);
    let task = service
        .spawn(
            scope("drop"),
            Box::pin(async move {
                completed_by_task.store(true, Ordering::SeqCst);
            }),
        )
        .expect("task starts");
    drop(task);
    assert!(completed.load(Ordering::SeqCst));

    let task = service
        .spawn(
            scope("panic"),
            Box::pin(async move {
                panic!("fixture task panic");
            }),
        )
        .expect("task starts");
    let failure = block_on(task.join()).expect_err("task panic is visible at join");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.panicked"
    );
    assert!(!format!("{failure:?}").contains("fixture task panic"));
}

#[test]
fn exact_target_approval_returns_only_an_opaque_discovery_target() {
    let executable = ExecutableRef::new("fixture.executable").expect("executable ref is valid");
    let axis = InterfaceVersionAxis::new("fixture.cli").expect("version axis is valid");
    let raw_path = "/private/fixture/bin/provider";
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(executable.clone(), axis.clone(), raw_path);
    let local = builder.build_services(host_id("fixture.host.target"));

    assert_eq!(target.executable(), &executable);
    assert_eq!(target.version_axis(), &axis);
    assert!(local.services().process().is_some());
    assert!(!format!("{target:?}").contains(raw_path));
}

#[test]
fn a_local_composition_cannot_substitute_for_remote_authority() {
    let local_id = host_id("fixture.host.local");
    let remote_id = host_id("fixture.host.remote-authoritative");
    let local =
        LocalProcessHost::builder(LocalProcessLimits::default()).build_services(local_id.clone());

    local
        .services()
        .require_execution_host(&local_id)
        .expect("matching local authority succeeds");
    let failure = local
        .services()
        .require_execution_host(&remote_id)
        .expect_err("remote authority cannot be substituted");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!format!("{failure:?}").contains(remote_id.as_str()));
}

fn host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("execution host id is valid")
}

fn scope(value: &str) -> ScopeId {
    ScopeId::new(value).expect("scope id is valid")
}
