use super::{complete, open, open_request, turn_request};
use crate::support::{DriverFixture, ServerScenario};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_alibaba_model_studio::AlibabaModelStudioDriver;
use swallowtail_core::{OwnedRemoteResourceKind, ReasoningMode, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, InteractiveSessionDriver, MonotonicInstant, OpenSessionRequest,
    ProviderCancellationOutcome, RemoteResourceDeletionOutcome, RequestId, SessionOptions,
    TerminalStatus,
};

#[test]
fn provider_failure_disconnect_and_cleanup_failure_remain_distinct() {
    for (scenario, expected_cleanup) in [
        (ServerScenario::ProviderError, "degraded"),
        (ServerScenario::Disconnect, "degraded"),
        (ServerScenario::CleanupFailure, "failed"),
    ] {
        let fixture = DriverFixture::new(scenario);
        let mut session = open(&fixture, "failure-session");
        let mut turn = block_on(
            session.start_turn(turn_request("failure-turn", "private"), fixture.services()),
        )
        .expect("turn starts");
        let (_, outcome) = complete(&mut turn);
        if scenario == ServerScenario::CleanupFailure {
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
        } else {
            assert!(matches!(
                outcome.status(),
                TerminalStatus::ProviderFailed(_)
            ));
            assert_eq!(
                outcome.remote_resource_deletion(OwnedRemoteResourceKind::ConversationItems),
                Some(RemoteResourceDeletionOutcome::Unconfirmed)
            );
        }
        let rendered = format!("{outcome:?}");
        for private in [
            "fixture-secret",
            "conv_fixture_01",
            "req_fixture_01",
            "raw private detail",
            fixture.server.endpoint(),
        ] {
            assert!(!rendered.contains(private));
        }
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        let cleanup = block_on(session.close());
        assert_eq!(
            matches!(cleanup, CleanupOutcome::Degraded(_)),
            expected_cleanup == "degraded"
        );
        assert_eq!(
            matches!(cleanup, CleanupOutcome::Failed(_)),
            expected_cleanup == "failed"
        );
        assert_eq!(fixture.releases(), 1);
        assert!(!format!("{cleanup:?}").contains("workspace_fixture"));
    }
}

#[test]
fn cancellation_and_deadline_end_session_with_unconfirmed_remote_truth() {
    for timed in [false, true] {
        let fixture = DriverFixture::new(ServerScenario::WaitForCancel);
        let mut session = open(
            &fixture,
            if timed {
                "deadline-session"
            } else {
                "cancel-session"
            },
        );
        let request = if timed {
            turn_request("timed-turn", "private").with_deadline(fixture.deadline_after(20))
        } else {
            turn_request("cancelled-turn", "private")
        };
        let mut turn =
            block_on(session.start_turn(request, fixture.services())).expect("turn starts");
        let mut events = turn.take_events().expect("events exist");
        let terminal = turn.take_terminal_outcome().expect("terminal exists");
        let parallel = block_on(session.start_turn(
            turn_request("parallel-turn", "must reject"),
            fixture.services(),
        ))
        .err()
        .expect("parallel turn rejects");
        assert_eq!(
            parallel.diagnostic().code(),
            "swallowtail.alibaba_model_studio.turn_active"
        );
        if !timed {
            for _ in 0..1_000 {
                if fixture.server.response_attempts() != 0 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            assert_eq!(fixture.server.response_attempts(), 1);
            block_on(turn.cancellation().request()).expect("cancellation accepted");
        }
        let outcome = block_on(async {
            while events.next().await.is_some() {}
            terminal.await
        });
        assert_eq!(
            outcome.status(),
            if timed {
                &TerminalStatus::TimedOut
            } else {
                &TerminalStatus::Cancelled
            }
        );
        assert_eq!(
            outcome.provider_cancellation(),
            Some(ProviderCancellationOutcome::Unconfirmed)
        );
        for resource in [
            OwnedRemoteResourceKind::ConversationItems,
            OwnedRemoteResourceKind::Conversation,
        ] {
            assert_eq!(
                outcome.remote_resource_deletion(resource),
                Some(RemoteResourceDeletionOutcome::Unconfirmed)
            );
        }
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert!(matches!(
            block_on(session.close()),
            CleanupOutcome::Degraded(_)
        ));
        assert_eq!(fixture.releases(), 1);
    }
}

#[test]
fn unsupported_session_options_and_elapsed_deadline_fail_before_effects() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let options =
        SessionOptions::default().with_reasoning_mode(ReasoningMode::new("low").expect("mode"));
    let request = open_request("options-reject").with_options(options);
    let error = block_on(AlibabaModelStudioDriver::new().open_session(
        fixture.plan(),
        request,
        fixture.services(),
    ))
    .err()
    .expect("options reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.alibaba_model_studio.unsupported_input"
    );

    let expired = OpenSessionRequest::resource_free(
        RequestId::new("deadline-reject").expect("request id"),
        Some(Deadline::at(MonotonicInstant::from_ticks(0))),
    )
    .with_provider_state_policy(SessionProviderStatePolicy::DurableConversationDeleteOnClose);
    let error = block_on(AlibabaModelStudioDriver::new().open_session(
        fixture.plan(),
        expired,
        fixture.services(),
    ))
    .err()
    .expect("deadline rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.alibaba_model_studio.deadline_elapsed"
    );
    assert!(fixture.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}
