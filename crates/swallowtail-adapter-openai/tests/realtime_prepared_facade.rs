mod realtime_support;

use futures_executor::block_on;
use realtime_support::{
    Call, RealtimeFixture, RealtimeScenario, TimeMode, complete, config, start_turn,
};
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{
    OpenAiRealtimeSessionProfileInput, openai_realtime_media_config, prepare_openai_realtime,
};
use swallowtail_core::{PlannedConnectionRolloverPolicy, RealtimeMediaConfig};
use swallowtail_runtime::{CleanupOutcome, RequestId, TerminalStatus};

#[test]
fn prepared_openai_realtime_preserves_two_turn_media_and_cleanup_on_both_hosts() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture =
            RealtimeFixture::for_host(RealtimeScenario::TwoTurns, TimeMode::Pending, host);
        let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI Realtime integration prepares");
        let operation = prepared
            .prepare_realtime_session(OpenAiRealtimeSessionProfileInput::manual_pcm_two_turns(
                RequestId::new(format!("prepared-{host}")).expect("request id is valid"),
                None,
            ))
            .expect("OpenAI Realtime session prepares");
        assert_eq!(operation.plan().execution_host_id().as_str(), host);
        assert_eq!(
            operation.request().config(),
            &openai_realtime_media_config()
        );
        assert_eq!(
            operation.request().planned_connection_rollover(),
            PlannedConnectionRolloverPolicy::Disabled
        );

        let mut session =
            block_on(operation.open_session(fixture.services())).expect("session opens");
        for turn in 1..=2 {
            let response = start_turn(
                &mut session,
                &fixture,
                &format!("prepared-stream-{turn}"),
                turn,
            );
            let (response, _, outcome) = complete(response);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
        }
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
    }
}

#[test]
fn openai_realtime_config_and_rollover_drift_fail_before_access() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let prepared = prepare_openai_realtime(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI Realtime integration prepares");
    let base = config();
    let wrong = RealtimeMediaConfig::new(
        base.input_format(),
        base.output_format(),
        NonZeroU64::new(16_384).expect("bound is non-zero"),
        base.maximum_turns(),
    );
    for input in [
        OpenAiRealtimeSessionProfileInput::new(
            RequestId::new("wrong-config").expect("request id is valid"),
            wrong,
            None,
            PlannedConnectionRolloverPolicy::Disabled,
        ),
        OpenAiRealtimeSessionProfileInput::new(
            RequestId::new("wrong-rollover").expect("request id is valid"),
            openai_realtime_media_config(),
            None,
            PlannedConnectionRolloverPolicy::Bounded(
                std::num::NonZeroU32::new(1).expect("bound is non-zero"),
            ),
        ),
    ] {
        assert!(prepared.prepare_realtime_session(input).is_err());
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
