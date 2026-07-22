mod realtime_support;

use futures_executor::block_on;
use realtime_support::{
    Call, RealtimeFixture, RealtimeScenario, TimeMode, assert_ordered_evidence, complete, config,
    input_chunk, open, turn_id,
};
use swallowtail_adapter_openai::openai_realtime_descriptor;
use swallowtail_core::DriverRole;
use swallowtail_runtime::{CleanupOutcome, MediaInputCommit, MediaStreamId, TerminalStatus};

#[test]
fn descriptor_and_two_serial_turns_preserve_exact_route_events_and_cleanup() {
    let descriptor = openai_realtime_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.openai.realtime"
    );
    assert_eq!(descriptor.transport_family().as_str(), "realtime-websocket");
    assert!(descriptor.supports_role(DriverRole::RealtimeMediaSession));
    assert!(!descriptor.supports_role(DriverRole::StructuredRun));

    for host in ["host.local", "host.remote-authoritative"] {
        let fixture =
            RealtimeFixture::for_host(RealtimeScenario::TwoTurns, TimeMode::Pending, host);
        assert_exact_plan(&fixture, host);
        let mut session = open(&fixture, "serial", None);
        let session_id = session.session_id().clone();
        for turn in 1..=2 {
            let stream_id = MediaStreamId::new(format!("input-{turn}")).expect("stream is valid");
            block_on(session.append_input(
                input_chunk(&session_id, stream_id.clone(), 1),
                fixture.services(),
            ))
            .expect("input appends");
            let response = block_on(session.commit_input(
                MediaInputCommit::new(turn_id(turn), stream_id),
                fixture.services(),
            ))
            .expect("input commits");
            let (response, events, outcome) = complete(response);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_ordered_evidence(&events);
            assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
        }
        assert_turn_bound(&fixture, &mut session, &session_id);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(
            fixture.server.handshake(),
            Some((
                "/v1/realtime?model=gpt-realtime-2.1".to_owned(),
                Some("Bearer fixture-secret".to_owned())
            ))
        );
        assert_cleanup_order(&fixture);
    }
}

fn assert_exact_plan(fixture: &RealtimeFixture, host: &str) {
    let plan = fixture.plan();
    assert_eq!(plan.execution_host_id().as_str(), host);
    assert_eq!(plan.endpoint_audience().as_str(), "api.openai.com");
    assert_eq!(plan.access_profile_id().as_str(), "access.openai.realtime");
    assert_eq!(
        plan.model_route_id().expect("route is bound").as_str(),
        "openai-gpt-realtime-2-1"
    );
    assert_eq!(
        plan.model_id().expect("model is bound").as_str(),
        "gpt-realtime-2.1"
    );
    assert_eq!(
        plan.requirements()
            .realtime_media()
            .expect("media requirements are bound")
            .config(),
        &config()
    );
}

fn assert_turn_bound(
    fixture: &RealtimeFixture,
    session: &mut Box<dyn swallowtail_runtime::RealtimeMediaSessionHandle>,
    session_id: &swallowtail_runtime::RuntimeSessionId,
) {
    let before = fixture.server.frames().len();
    let error = block_on(session.append_input(
        input_chunk(
            session_id,
            MediaStreamId::new("input-3").expect("stream is valid"),
            1,
        ),
        fixture.services(),
    ))
    .expect_err("third turn input is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.realtime_media_rejected"
    );
    assert_eq!(fixture.server.frames().len(), before);
}

fn assert_cleanup_order(fixture: &RealtimeFixture) {
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 1);
    assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
    let calls = fixture.calls.calls();
    let release = calls
        .iter()
        .rposition(|call| *call == Call::CredentialRelease)
        .expect("credential release recorded");
    for joined in [Call::TaskJoin, Call::BlockingJoin] {
        assert!(
            calls
                .iter()
                .rposition(|call| *call == joined)
                .expect("owned work joined")
                < release
        );
    }
}
