//! Contract 063 conformance against the real mounted local composition.

use crate::host::LocalProcessHost;
use crate::limits::LocalProcessLimits;
use std::sync::Arc;
use std::time::Duration;
use swallowtail_runtime::{HostServices, RegisteredToolDispatcher};
use swallowtail_testkit::{
    ScriptedRegisteredToolDispatcher, UncooperativeRegisteredToolDispatcher,
    assert_registered_tool_conformance, conformance_host_id, drive_fixture, poll_fixture_once,
};

fn compose(
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    cleanup_budget: Duration,
) -> HostServices {
    LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(dispatcher)
        .with_registered_tool_cleanup_budget(cleanup_budget)
        .build_services(conformance_host_id())
        .services()
        .clone()
}

#[test]
fn local_composition_passes_the_registered_tool_oracle() {
    assert_registered_tool_conformance(&compose);
}

#[test]
fn omitting_the_dispatcher_registers_no_registered_tool_port() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(conformance_host_id());

    assert!(services.services().registered_tool_bridge().is_none());
    assert_eq!(services.registered_tool_lease_count(), 0);
    assert!(services.services().watcher_bridge().is_some());
}

#[test]
fn a_clean_close_releases_the_lease_the_host_owned() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(Arc::new(ScriptedRegisteredToolDispatcher::echoing()))
        .build_services(conformance_host_id());
    let harness = swallowtail_testkit::RegisteredToolHarness::new(services.services().clone());
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();

    let lease = harness.open("turn-local-clean");
    let owned_while_open = services.registered_tool_lease_count();
    let cleanup = drive_fixture(port.close(
        lease,
        swallowtail_runtime::RegisteredToolCleanupCause::Completion,
    ))
    .expect("close joins");

    assert_eq!(owned_while_open, 1);
    assert_eq!(cleanup, swallowtail_runtime::CleanupOutcome::Clean);
    assert_eq!(services.registered_tool_lease_count(), 0);
}

#[test]
fn a_failed_cleanup_keeps_the_lease_under_host_ownership() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(Arc::new(UncooperativeRegisteredToolDispatcher::default()))
        .with_registered_tool_cleanup_budget(Duration::from_millis(20))
        .build_services(conformance_host_id());
    let harness = swallowtail_testkit::RegisteredToolHarness::new(services.services().clone());
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();
    let lease = harness.open("turn-local-uncooperative");
    let mut pending = lease.call(swallowtail_runtime::RegisteredToolCallRequest::new(
        swallowtail_runtime::RegisteredToolCallId::new("call-local").expect("call id"),
        swallowtail_testkit::fixture_tool_id(swallowtail_testkit::FIXTURE_NATIVE_TOOL),
        swallowtail_testkit::fixture_payload(8, 1024),
        swallowtail_testkit::conformance_deadline(),
    ));
    assert!(matches!(
        poll_fixture_once(&mut pending),
        std::task::Poll::Pending
    ));
    drop(pending);

    let cleanup = drive_fixture(port.close(
        lease,
        swallowtail_runtime::RegisteredToolCleanupCause::Deadline,
    ))
    .expect("close reports its exact truth");

    assert!(matches!(
        cleanup,
        swallowtail_runtime::CleanupOutcome::Failed(_)
    ));
    assert_eq!(
        services.registered_tool_lease_count(),
        1,
        "the failed lease stays under the existing host ownership"
    );
}

#[test]
fn both_profiles_share_one_kernel_with_exactly_one_listener_owner() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(Arc::new(ScriptedRegisteredToolDispatcher::echoing()))
        .build_services(conformance_host_id());
    let registry = services.services().clone();
    let watcher_port = registry.watcher_bridge().expect("watcher port").clone();
    let registered_port = registry
        .registered_tool_bridge()
        .expect("registered port")
        .clone();
    let harness = swallowtail_testkit::RegisteredToolHarness::new(registry);

    let watcher_lease = drive_fixture(watcher_port.open(
        swallowtail_runtime::WatcherBridgeOpenRequest::new(
            swallowtail_testkit::conformance_scope(),
            swallowtail_testkit::conformance_turn("turn-both-profiles"),
        ),
    ))
    .expect("the watcher profile opens its own lease");
    let registered_lease = harness.open("turn-both-profiles");

    assert!(
        !watcher_lease.endpoint().expose().is_empty(),
        "the watcher profile still owns the one loopback listener"
    );
    assert!(
        registered_lease.endpoint().is_none(),
        "the registered profile binds no second listener"
    );
    assert_eq!(services.registered_tool_lease_count(), 1);

    let registered_cleanup = drive_fixture(registered_port.close(
        registered_lease,
        swallowtail_runtime::RegisteredToolCleanupCause::Completion,
    ))
    .expect("registered close joins");
    let watcher_cleanup = drive_fixture(watcher_port.close(
        watcher_lease,
        swallowtail_core::WatcherCleanupCause::Stopped,
    ))
    .expect("watcher close joins");

    assert_eq!(
        registered_cleanup,
        swallowtail_runtime::CleanupOutcome::Clean
    );
    assert!(matches!(
        watcher_cleanup,
        swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::NotApplicable
    ));
    assert_eq!(services.registered_tool_lease_count(), 0);
}
