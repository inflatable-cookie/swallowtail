use crate::live_support;

use futures_executor::block_on;
use live_support::{Call, LiveFixture, LiveScenario, TimeMode, complete, config, start_turn};
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::{
    GeminiLiveSessionProfileInput, gemini_live_media_config, gemini_live_rollover_policy,
    prepare_gemini_live,
};
use swallowtail_core::{PlannedConnectionRolloverPolicy, RealtimeMediaConfig};
use swallowtail_runtime::{CleanupOutcome, RequestId, TerminalStatus};
use swallowtail_testkit::assert_observable_activity_not_applicable;

#[test]
fn prepared_gemini_live_preserves_rollover_and_cleanup_on_both_hosts() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture =
            LiveFixture::for_host(LiveScenario::TwoTurnsRollover, TimeMode::Pending, host);
        let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
            .expect("Gemini Live integration prepares");
        let operation = prepared
            .prepare_live_session(GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new(format!("prepared-{host}")).expect("request id is valid"),
                None,
            ))
            .expect("Gemini Live session prepares");
        assert_eq!(operation.plan().execution_host_id().as_str(), host);
        assert_eq!(operation.request().config(), &gemini_live_media_config());
        assert_eq!(
            operation.request().planned_connection_rollover(),
            gemini_live_rollover_policy()
        );
        assert_observable_activity_not_applicable(operation.evidence().operation());

        let mut session =
            block_on(operation.open_session(fixture.services())).expect("session opens");
        for turn in 1..=2 {
            let response = start_turn(&mut session, &fixture, turn);
            let (response, _, outcome) = complete(response);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
        }
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.handshakes().len(), 2);
        assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
    }
}

#[test]
fn gemini_live_config_and_rollover_drift_fail_before_access() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let base = config();
    let wrong = RealtimeMediaConfig::new(
        base.input_format(),
        base.output_format(),
        NonZeroU64::new(16_384).expect("bound is non-zero"),
        base.maximum_turns(),
    );
    for input in [
        GeminiLiveSessionProfileInput::new(
            RequestId::new("wrong-config").expect("request id is valid"),
            wrong,
            None,
            gemini_live_rollover_policy(),
        ),
        GeminiLiveSessionProfileInput::new(
            RequestId::new("wrong-rollover").expect("request id is valid"),
            gemini_live_media_config(),
            None,
            PlannedConnectionRolloverPolicy::Disabled,
        ),
    ] {
        assert!(prepared.prepare_live_session(input).is_err());
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
