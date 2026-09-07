//! Contract 063 conformance against the real mounted local composition.

use crate::host::LocalProcessHost;
use crate::limits::LocalProcessLimits;
use std::sync::Arc;
use std::time::Duration;
use swallowtail_runtime::HostServices;
use swallowtail_testkit::{
    RegisteredToolHostSpec, ScriptedRegisteredToolDispatcher,
    UncooperativeRegisteredToolDispatcher, assert_registered_tool_conformance, conformance_host_id,
    drive_fixture, poll_fixture_once,
};

fn compose(spec: RegisteredToolHostSpec) -> HostServices {
    LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(spec.dispatcher)
        .with_registered_tool_cleanup_budget(spec.cleanup_budget)
        .with_registered_tool_clock(spec.clock)
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
        .with_registered_tool_clock(Arc::new(swallowtail_testkit::FakeClock::default()))
        .build_services(conformance_host_id());
    let harness = swallowtail_testkit::RegisteredToolHarness::with_clock(
        services.services().clone(),
        Arc::new(swallowtail_testkit::FakeClock::default()),
    );
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
        .with_registered_tool_clock(Arc::new(swallowtail_testkit::FakeClock::default()))
        .build_services(conformance_host_id());
    let harness = swallowtail_testkit::RegisteredToolHarness::with_clock(
        services.services().clone(),
        Arc::new(swallowtail_testkit::FakeClock::default()),
    );
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
fn a_rejected_direct_open_creates_no_lease() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(Arc::new(ScriptedRegisteredToolDispatcher::echoing()))
        .with_registered_tool_clock(Arc::new(swallowtail_testkit::FakeClock::default()))
        .build_services(conformance_host_id());
    let port = services
        .services()
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();
    let host = conformance_host_id();
    let mut input = swallowtail_testkit::fixture_snapshot_input(&host);
    input.required_services = [swallowtail_core::HostServiceKind::DeviceCodeDisplay]
        .into_iter()
        .collect();
    let snapshot = Arc::new(
        swallowtail_runtime::RegisteredToolSnapshot::new(input).expect("snapshot is valid"),
    );

    let error = drive_fixture(
        port.open(swallowtail_runtime::RegisteredToolOpenRequest::new(
            host,
            swallowtail_testkit::conformance_instance(),
            swallowtail_testkit::conformance_scope(),
            swallowtail_testkit::conformance_turn("turn-direct-reject"),
            swallowtail_testkit::fixture_selection(snapshot),
            swallowtail_testkit::fixture_admission(Arc::new(
                swallowtail_testkit::ScriptedAdmissionPort::current(),
            )),
            swallowtail_testkit::conformance_deadline(),
        )),
    )
    .expect_err("the mounted port applies the typed readiness gate");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.registered_tool.missing_host_service"
    );
    assert_eq!(services.registered_tool_lease_count(), 0);
    assert_eq!(services.operation_bridge_lease_count(), 0);
}

#[test]
fn both_profiles_share_one_lease_owner_and_generation_space() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(Arc::new(ScriptedRegisteredToolDispatcher::echoing()))
        .with_registered_tool_clock(Arc::new(swallowtail_testkit::FakeClock::default()))
        .build_services(conformance_host_id());
    let registry = services.services().clone();
    let watcher_port = registry.watcher_bridge().expect("watcher port").clone();
    let registered_port = registry
        .registered_tool_bridge()
        .expect("registered port")
        .clone();
    let harness = swallowtail_testkit::RegisteredToolHarness::with_clock(
        registry,
        Arc::new(swallowtail_testkit::FakeClock::default()),
    );

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
        !registered_lease.transport().binds_listener(),
        "the registered profile binds no second listener"
    );
    assert_eq!(services.registered_tool_lease_count(), 1);
    assert_eq!(
        services.operation_bridge_lease_count(),
        2,
        "one shared registry owns both profile leases"
    );
    assert_eq!(
        services.operation_bridge_generations(&swallowtail_testkit::conformance_turn(
            "turn-both-profiles"
        )),
        vec![("watcher", 1), ("registered-tool", 2)],
        "both profiles draw from one monotonic generation space"
    );

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
    assert_eq!(services.operation_bridge_lease_count(), 0);
}

#[test]
fn one_joined_teardown_closes_every_selected_profile() {
    let services = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(Arc::new(ScriptedRegisteredToolDispatcher::echoing()))
        .with_registered_tool_clock(Arc::new(swallowtail_testkit::FakeClock::default()))
        .build_services(conformance_host_id());
    let registry = services.services().clone();
    let watcher_port = registry.watcher_bridge().expect("watcher port").clone();
    let harness = swallowtail_testkit::RegisteredToolHarness::with_clock(
        registry,
        Arc::new(swallowtail_testkit::FakeClock::default()),
    );
    let turn = swallowtail_testkit::conformance_turn("turn-joined");
    let watcher_lease = drive_fixture(watcher_port.open(
        swallowtail_runtime::WatcherBridgeOpenRequest::new(
            swallowtail_testkit::conformance_scope(),
            turn.clone(),
        ),
    ))
    .expect("the watcher profile opens");
    let registered_lease = harness.open("turn-joined");

    assert_eq!(services.operation_bridge_lease_count(), 2);
    let cleanup = services
        .close_operation_bridges(&turn, crate::OperationBridgeCleanupCause::Completion)
        .expect("one joined teardown closes both profiles");

    assert_eq!(cleanup, swallowtail_runtime::CleanupOutcome::Clean);
    assert_eq!(
        services.operation_bridge_lease_count(),
        0,
        "one lifecycle owner released every profile lease"
    );
    drop(watcher_lease);
    drop(registered_lease);
}
