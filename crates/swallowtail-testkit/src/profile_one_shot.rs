use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, SyntheticProfile, assert_common_contract, poll_immediate,
};
use swallowtail_runtime::{Deadline, ExecutableRef, MonotonicInstant, ProcessRequest, ScopeId};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::OneShotStructuredCli;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    fixture.preflight().expect("one-shot preflight succeeds");
    let recording = RecordingHostServices::default();
    let services = recording.services();
    let deadline = Deadline::at(MonotonicInstant::from_ticks(25));
    poll_immediate(
        services
            .time()
            .expect("time service is available")
            .wait_until(deadline),
    );
    let request = ProcessRequest::new(
        ExecutableRef::new("fixture-one-shot-executable").expect("reference is valid"),
    );
    let process = poll_immediate(
        services
            .process()
            .expect("process service is available")
            .start(
                ScopeId::new("one-shot-scope").expect("scope is valid"),
                request,
            ),
    )
    .expect("process start succeeds");
    poll_immediate(process.request_stop()).expect("graceful stop succeeds");
    poll_immediate(process.force_stop()).expect("authorized force stop succeeds");
    poll_immediate(process.wait()).expect("process wait succeeds");

    for call in [
        RecordedHostCall::ProcessStart,
        RecordedHostCall::ProcessGracefulStop,
        RecordedHostCall::ProcessForceStop,
        RecordedHostCall::ProcessWait,
    ] {
        assert_eq!(recording.count(call), 1, "missing {call:?}");
    }
    report.record(ConformanceAssertion::ProcessLifecycle);
    report
}
