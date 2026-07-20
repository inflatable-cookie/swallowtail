use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, SyntheticProfile, assert_common_contract, poll_immediate,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use swallowtail_core::InstanceOwnership;
use swallowtail_runtime::{AttachedServingHandle, BoxFuture, CleanupOutcome, ServingInstanceId};

struct SyntheticAttachedHandle {
    id: ServingInstanceId,
    closes: Arc<AtomicUsize>,
}

impl AttachedServingHandle for SyntheticAttachedHandle {
    fn serving_instance_id(&self) -> &ServingInstanceId {
        &self.id
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        self.closes.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::AttachedSelfHosted;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture.preflight().expect("attached preflight succeeds");
    assert_eq!(plan.ownership(), InstanceOwnership::ExternalAttached);
    assert!(plan.model_route_id().is_some());

    let closes = Arc::new(AtomicUsize::new(0));
    let handle: Box<dyn AttachedServingHandle> = Box::new(SyntheticAttachedHandle {
        id: ServingInstanceId::new("attached-serving").expect("serving id is valid"),
        closes: Arc::clone(&closes),
    });
    assert_eq!(
        poll_immediate(handle.close()),
        CleanupOutcome::NotApplicable
    );
    assert_eq!(closes.load(Ordering::SeqCst), 1);

    let recording = RecordingHostServices::default();
    assert_eq!(recording.count(RecordedHostCall::ProcessGracefulStop), 0);
    assert_eq!(recording.count(RecordedHostCall::ProcessForceStop), 0);
    report.record(ConformanceAssertion::AttachedServiceNeverStopped);
    report
}
