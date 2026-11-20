use super::{
    DiscoveryCancellation, InstalledExecutableDiscoveryRequest, InstalledExecutableTarget,
    validate_installed_executable_discovery_services,
};
use crate::{
    CancellationControl, Deadline, ExecutableRef, HostServices, MonotonicInstant, RequestId,
    ScopeId,
};
use swallowtail_core::{CancellationScope, ExecutionHostId, InterfaceVersionAxis};

#[test]
fn request_retains_exact_target_and_redacts_host_values() {
    let request = request("fixture.host.remote");
    assert_eq!(
        request.target().version_axis().as_str(),
        "fixture.harness.package"
    );
    assert_eq!(request.deadline().instant().ticks(), 100);
    let debug = format!("{request:?}");
    assert!(!debug.contains("/private/provider/bin"));
    assert!(!debug.contains("request-private"));
    assert!(!debug.contains("scope-private"));
}

#[test]
fn cancellation_is_shared_idempotent_and_probe_scoped() {
    let cancellation = DiscoveryCancellation::new();
    let shared = cancellation.clone();
    assert_eq!(cancellation.scope(), CancellationScope::DiscoveryProbe);
    assert!(!shared.is_requested());
    let first = poll_immediate(cancellation.request()).expect("request succeeds");
    assert_eq!(first, crate::CancellationAcknowledgement::Requested);
    let second = poll_immediate(cancellation.request()).expect("repeat succeeds");
    assert_eq!(second, crate::CancellationAcknowledgement::AlreadyRequested);
    assert!(shared.is_requested());
    poll_immediate(shared.wait_requested());
}

#[test]
fn validation_rejects_host_substitution_and_missing_services() {
    let request = request("fixture.host.remote");
    let wrong =
        HostServices::new(ExecutionHostId::new("fixture.host.local").expect("host id is valid"));
    let mismatch = validate_installed_executable_discovery_services(&request, &wrong)
        .expect_err("different host must fail");
    assert_eq!(
        mismatch.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );

    let matching = HostServices::new(request.execution_host_id().clone());
    let missing = validate_installed_executable_discovery_services(&request, &matching)
        .expect_err("required services must be explicit");
    assert_eq!(
        missing.diagnostic().code(),
        "swallowtail.installed_executable.host_services_missing"
    );
}

fn request(host: &str) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("request-private").expect("request id is valid"),
        ScopeId::new("scope-private").expect("scope id is valid"),
        ExecutionHostId::new(host).expect("host id is valid"),
        InstalledExecutableTarget::new(
            ExecutableRef::new("/private/provider/bin").expect("reference is valid"),
            InterfaceVersionAxis::new("fixture.harness.package").expect("axis is valid"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn poll_immediate<T>(future: impl std::future::Future<Output = T>) -> T {
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());
    match Pin::as_mut(&mut future).poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("fixture future must be immediately ready"),
    }
}

#[test]
fn discovery_cancellation_wakes_outside_lock_and_cleans_up_dropped_waiters() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Wake, Waker};

    let cancellation = DiscoveryCancellation::new();
    let mut dropped_future = cancellation.wait_requested();
    let mut noop_context = Context::from_waker(Waker::noop());
    assert_eq!(
        dropped_future.as_mut().poll(&mut noop_context),
        Poll::Pending
    );
    assert_eq!(cancellation.inner().pending_waiter_count(), 1);
    drop(dropped_future);
    assert_eq!(
        cancellation.inner().pending_waiter_count(),
        0,
        "dropped DiscoveryCancellation wait future must release its waker"
    );

    struct ReentrantWake {
        cancellation: DiscoveryCancellation,
        lock_was_free: AtomicBool,
        wake_count: AtomicUsize,
        pending: Mutex<Option<crate::BoxFuture<'static, ()>>>,
    }

    impl Wake for ReentrantWake {
        fn wake(self: Arc<Self>) {
            self.wake_count.fetch_add(1, Ordering::SeqCst);
            self.lock_was_free
                .store(self.cancellation.inner().try_lock_free(), Ordering::SeqCst);
            // Re-entering DiscoveryCancellation from inside wake() must never deadlock.
            if let Some(mut future) = self.pending.lock().expect("pending lock").take() {
                let mut context = Context::from_waker(Waker::noop());
                assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
            }
        }
    }

    let probe = Arc::new(ReentrantWake {
        cancellation: cancellation.clone(),
        lock_was_free: AtomicBool::new(false),
        wake_count: AtomicUsize::new(0),
        pending: Mutex::new(None),
    });
    let waker = Waker::from(Arc::clone(&probe));
    let mut context = Context::from_waker(&waker);
    let mut waiter = cancellation.wait_requested();
    assert_eq!(waiter.as_mut().poll(&mut context), Poll::Pending);
    *probe.pending.lock().expect("pending lock") = Some(waiter);

    assert_eq!(
        poll_immediate(cancellation.request()).expect("request succeeds"),
        crate::CancellationAcknowledgement::Requested
    );
    assert_eq!(probe.wake_count.load(Ordering::SeqCst), 1);
    assert!(
        probe.lock_was_free.load(Ordering::SeqCst),
        "DiscoveryCancellation must drop its waiter lock before calling wake()"
    );
}
