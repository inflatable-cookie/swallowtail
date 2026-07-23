use super::*;
use crate::support::ServerScenario;

#[test]
fn provider_failure_and_disconnect_are_safe_and_terminal() {
    for (scenario, expected) in [
        (
            ServerScenario::ProviderError,
            "swallowtail.deepseek.account_concurrency",
        ),
        (
            ServerScenario::DisconnectAfterTool,
            "swallowtail.deepseek.stream_incomplete",
        ),
    ] {
        let fixture = Fixture::with_scenario(scenario);
        let mut session = open(&fixture, "failure-session");
        let mut turn = block_on(session.start_direct_continuation_turn(
            turn_request("failure-turn", FIRST_PROMPT, 5_000),
            fixture.services(),
        ))
        .expect("turn starts");
        if matches!(scenario, ServerScenario::DisconnectAfterTool) {
            submit_fixture_result(&mut turn);
        }
        let (_events, outcome) = complete(&mut turn);
        let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
            panic!("expected provider failure: {:?}", outcome.status());
        };
        assert_eq!(diagnostic.code(), expected);
        assert!(!diagnostic.to_string().contains("raw secret detail"));
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases(), 1);
    }
}

#[test]
fn active_stream_cancellation_joins_before_session_credential_release() {
    let fixture = Fixture::with_scenario(ServerScenario::WaitAfterTool);
    let mut session = open(&fixture, "cancel-session");
    let mut turn = block_on(session.start_direct_continuation_turn(
        turn_request("cancel-turn", FIRST_PROMPT, 5_000),
        fixture.services(),
    ))
    .expect("turn starts");
    submit_fixture_result(&mut turn);
    for _ in 0..100 {
        if fixture.server.attempts() == 2 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(turn.cancellation().request()).expect("cancellation accepted");
    let (_events, outcome) = complete(&mut turn);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 0);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);
    assert_eq!(fixture.release_after_blocking(), [2]);
}

#[test]
fn tool_wait_deadline_stops_without_authorizing_an_extra_provider_attempt() {
    let fixture = Fixture::new();
    let mut session = open(&fixture, "deadline-session");
    let mut turn = block_on(session.start_direct_continuation_turn(
        turn_request("deadline-turn", FIRST_PROMPT, 20),
        fixture.services(),
    ))
    .expect("turn starts");
    let mut exchange = turn.take_direct_tool_exchange().expect("tool exchange");
    let mut calls = exchange.take_calls().expect("tool calls");
    block_on(calls.next())
        .expect("tool call exists")
        .expect("tool call valid");
    let (_events, outcome) = complete(&mut turn);
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(fixture.server.attempts(), 1);
    let late =
        block_on(exchange.submitter().submit(Vec::new())).expect_err("late tool result rejects");
    assert_eq!(
        late.diagnostic().code(),
        "swallowtail.deepseek.tool_exchange_invalid"
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);
}
