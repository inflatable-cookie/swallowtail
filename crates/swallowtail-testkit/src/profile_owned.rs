use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, RecordingOutcome, SyntheticProfile, assert_common_contract,
    poll_immediate,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use swallowtail_core::{InstanceOwnership, SafeDiagnostic};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, BoxFuture, CleanupOutcome,
    OwnedServingHandle, ServingInstanceId,
};

struct SyntheticOwnedHandle {
    id: ServingInstanceId,
    ownership: InstanceOwnership,
    stops: Arc<AtomicUsize>,
}

impl OwnedServingHandle for SyntheticOwnedHandle {
    fn serving_instance_id(&self) -> &ServingInstanceId {
        &self.id
    }

    fn ownership(&self) -> InstanceOwnership {
        self.ownership
    }

    fn stop(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        self.stops.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::OwnedSelfHosted;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture.preflight().expect("owned preflight succeeds");
    assert_eq!(plan.ownership(), InstanceOwnership::HostOwnedEphemeral);
    assert!(plan.model_route_id().is_some());

    let stops = Arc::new(AtomicUsize::new(0));
    for (suffix, ownership) in [
        ("ephemeral", InstanceOwnership::HostOwnedEphemeral),
        ("persistent", InstanceOwnership::HostOwnedPersistent),
    ] {
        let handle: Box<dyn OwnedServingHandle> = Box::new(SyntheticOwnedHandle {
            id: ServingInstanceId::new(format!("owned-{suffix}")).expect("serving id is valid"),
            ownership,
            stops: Arc::clone(&stops),
        });
        assert_eq!(handle.ownership(), ownership);
        assert_eq!(poll_immediate(handle.stop()), CleanupOutcome::Clean);
    }
    assert_eq!(stops.load(Ordering::SeqCst), 2);

    let failing = RecordingHostServices::new(RecordingOutcome::Fail(SafeDiagnostic::new(
        "fixture.resource_unavailable",
        "Resource unavailable",
    )));
    let result = poll_immediate(
        failing
            .services()
            .attachment()
            .expect("attachment service is available")
            .materialize_file(
                swallowtail_runtime::ScopeId::new("owned-artifact-scope").expect("scope is valid"),
                AttachmentDescriptor::new(
                    AttachmentRef::new("owned-artifact").expect("reference is valid"),
                    "application/octet-stream",
                    AttachmentRole::Input,
                )
                .expect("descriptor is valid"),
            ),
    );
    assert!(result.is_err());
    assert_eq!(
        failing.count(RecordedHostCall::AttachmentMaterializeFile),
        1
    );

    report.record(ConformanceAssertion::OwnedServiceStops);
    report
}
