mod support;

use futures_executor::block_on;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use support::{FakeProcessService, host_services, host_services_with, plan_with, working_resource};
use swallowtail_adapter_codex::CodexExecDriver;
use swallowtail_core::{Capability, CapabilityConstraint, CapabilityRequirement, HostServiceKind};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, BoxFuture, Deadline, DeadlineObservation,
    EnvironmentRef, MonotonicInstant, OperationContent, RequestId, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus, TimeService,
};
use swallowtail_testkit::{RecordedHostCall, RecordingHostServices};

const COMPLETED_JSONL: &str = concat!(
    "{\"type\":\"thread.started\",\"thread_id\":\"private-thread\"}\n",
    "{\"type\":\"turn.started\"}\n",
    "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"finished\"}}\n",
    "{\"type\":\"turn.completed\"}\n"
);

#[test]
fn deadline_expiry_force_stops_releases_and_reports_timeout() {
    let (process, state) = FakeProcessService::held_open();
    let recording = RecordingHostServices::default();
    let attachment = AttachmentDescriptor::new(
        AttachmentRef::new("screenshot.timeout").expect("attachment reference is valid"),
        "image/png",
        AttachmentRole::Input,
    )
    .expect("attachment descriptor is valid")
    .with_known_length(512);
    let request = request("request-timeout")
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(10)))
        .with_attachments([attachment]);
    let attachment_capability = CapabilityRequirement::new(
        Capability::Attachments,
        [
            CapabilityConstraint::attachment_media_type("image/png").expect("media type is valid"),
            CapabilityConstraint::AttachmentMaximumBytes(1024),
            CapabilityConstraint::AttachmentMaximumCount(1),
        ],
    );
    let services = [HostServiceKind::Time, HostServiceKind::Attachment];
    let mut handle = block_on(driver().start_run(
        plan_with([attachment_capability], services),
        request,
        host_services_with(process, &recording, services),
    ))
    .expect("deadline-bound run starts");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert_eq!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    assert!(state.force_stopped());
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::TimeWaitUntil), 1);
    assert_eq!(recording.count(RecordedHostCall::AttachmentFileRelease), 1);
}

#[test]
fn process_completion_cancels_the_outstanding_deadline_wait() {
    let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
    let dropped = Arc::new(AtomicBool::new(false));
    let time = Arc::new(PendingTimeService {
        dropped: Arc::clone(&dropped),
    });
    let services = host_services(process).with_time(time);
    let request = request("request-before-deadline")
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)));
    let mut handle =
        block_on(driver().start_run(plan_with([], [HostServiceKind::Time]), request, services))
            .expect("deadline-bound run starts");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    assert!(state.waited());
    assert!(!state.force_stopped());
    assert!(dropped.load(Ordering::SeqCst));
}

fn request(id: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("wait if needed").expect("content is valid"),
        support::current_exec_policy(),
    )
    .with_working_resource(working_resource())
}

fn driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").expect("environment is valid"))
}

struct PendingTimeService {
    dropped: Arc<AtomicBool>,
}

impl TimeService for PendingTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(PendingDeadline {
            dropped: Arc::clone(&self.dropped),
        })
    }
}

struct PendingDeadline {
    dropped: Arc<AtomicBool>,
}

impl Future for PendingDeadline {
    type Output = DeadlineObservation;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for PendingDeadline {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::SeqCst);
    }
}
