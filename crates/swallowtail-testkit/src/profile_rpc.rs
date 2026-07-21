use crate::{
    CallbackExchangeFixture, ConformanceAssertion, ConformanceReport, ProfilePreflightFixture,
    RecordedHostCall, RecordingHostServices, RecordingOutcome, SyntheticProfile,
    assert_common_contract, poll_immediate, successful_callback_response,
};
use swallowtail_core::{CancellationScope, SessionRef};
use swallowtail_runtime::{
    CallbackAbandonment, CallbackId, CallbackPayload, CallbackRequest, CallbackWaitState,
    CancellationControl, Deadline, ImmediateCancellation, MonotonicInstant, RuntimeEvent,
    RuntimeEventKind, RuntimeSessionId, RuntimeTurnId, ScopeId, SessionResumeBinding,
    runtime_event_channel,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::LongLivedRpcHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let plan = ProfilePreflightFixture::new(profile)
        .preflight()
        .expect("RPC preflight succeeds");
    let session_id = RuntimeSessionId::new("fixture-session").expect("session id is valid");
    let turn_id = RuntimeTurnId::new("fixture-turn").expect("turn id is valid");
    let callback_id = CallbackId::new("fixture-callback").expect("callback id is valid");
    assert_ne!(session_id.as_str(), turn_id.as_str());

    let callback_request = CallbackRequest::tool_call(
        callback_id.clone(),
        turn_id.clone(),
        2,
        Some(Deadline::at(MonotonicInstant::from_ticks(50))),
        "fixture_tool",
        CallbackPayload::new(b"{}".to_vec(), 16).expect("payload is bounded"),
    )
    .expect("callback request is valid");
    let mut callback = CallbackExchangeFixture::new(callback_request);
    assert_eq!(callback.event().sequence(), 2);
    assert_eq!(
        callback.deadline(),
        Some(Deadline::at(MonotonicInstant::from_ticks(50)))
    );
    callback
        .respond(successful_callback_response(callback.request()))
        .expect("callback response is correlated");
    assert_eq!(callback.state(), CallbackWaitState::Responded);

    let timeout_request = CallbackRequest::tool_call(
        CallbackId::new("fixture-timeout-callback").expect("callback id is valid"),
        turn_id.clone(),
        3,
        Some(Deadline::at(MonotonicInstant::from_ticks(60))),
        "fixture_tool",
        CallbackPayload::new(b"{}".to_vec(), 16).expect("payload is bounded"),
    )
    .expect("callback request is valid");
    let mut timed_out = CallbackExchangeFixture::new(timeout_request);
    timed_out
        .abandon(CallbackAbandonment::TimedOut)
        .expect("waiting callback times out");
    assert_eq!(
        timed_out.state(),
        CallbackWaitState::Abandoned(CallbackAbandonment::TimedOut)
    );

    let (events, _stream) = runtime_event_channel(3).expect("capacity is valid");
    events
        .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .expect("start is accepted");
    events
        .send(RuntimeEvent::new(
            2,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))
        .expect("callback is accepted");
    events.mark_terminal();

    let cancellation = ImmediateCancellation::new(CancellationScope::ActiveTurn);
    poll_immediate(cancellation.request()).expect("active turn cancellation succeeds");
    assert_eq!(cancellation.scope(), CancellationScope::ActiveTurn);
    let provider_session =
        SessionRef::new("provider/session/private").expect("session reference is valid");
    assert!(!format!("{provider_session:?}").contains(provider_session.as_provider_value()));
    let resume_binding = SessionResumeBinding::new(
        provider_session,
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id()
            .expect("RPC plan has a model route")
            .clone(),
        plan.model_id().expect("RPC plan has a model").clone(),
        swallowtail_runtime::WorkingResourceRef::new("rpc-resource")
            .expect("resource ref is valid"),
        swallowtail_runtime::SessionAccessPolicy::ambient_harness(
            swallowtail_runtime::ResourceAccess::Read,
        ),
    );
    assert!(resume_binding.matches_plan(&plan));

    let recording = RecordingHostServices::for_host(
        plan.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    recording
        .services()
        .require_execution_host(plan.execution_host_id())
        .expect("RPC services belong to the preflight-bound host");
    let task = recording
        .services()
        .task()
        .expect("task service is available")
        .spawn(
            ScopeId::new("rpc-session-scope").expect("scope is valid"),
            Box::pin(async {}),
        )
        .expect("session task starts");
    poll_immediate(task.join()).expect("session task joins on close");
    assert_eq!(recording.count(RecordedHostCall::TaskJoin), 1);

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::CallbackExchange);
    report
}
