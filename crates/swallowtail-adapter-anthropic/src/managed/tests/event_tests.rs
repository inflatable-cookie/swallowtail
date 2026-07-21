use super::*;

#[test]
fn authoritative_events_cover_success_callbacks_recovery_and_safe_failure() {
    let success = parse_stream(SUCCESS).expect("success stream is valid");
    assert_eq!(success.len(), 6);
    assert_eq!(success[3].event_type(), "agent.message");
    assert!(matches!(
        success[5].kind(),
        ManagedEventKind::Idle(IdleReason::EndTurn)
    ));

    let callbacks = parse_stream(REQUIRES_ACTION).expect("callback stream is valid");
    assert!(matches!(
        callbacks[1].kind(),
        ManagedEventKind::CustomToolUse { name, input }
            if name == "lookup_fixture" && input["key"] == "alpha"
    ));
    assert!(matches!(
        callbacks[2].kind(),
        ManagedEventKind::Idle(IdleReason::RequiresAction(ids)) if ids == &["event_tool"]
    ));

    let rescheduling = parse_stream(RESCHEDULING).expect("rescheduling stream is valid");
    assert!(matches!(
        rescheduling[1].kind(),
        ManagedEventKind::Rescheduled
    ));

    let provider_failure = parse_stream(PROVIDER_FAILURE).expect("failure event is valid");
    assert!(matches!(
        provider_failure[0].kind(),
        ManagedEventKind::ProviderError
    ));
    assert!(!format!("{:?}", provider_failure[0]).contains("fixture-secret-never-log"));
}

#[test]
fn previews_disconnects_and_schema_drift_fail_closed() {
    assert!(parse_stream(DISCONNECT).is_err());
    assert!(parse_stream(PREVIEW).is_err());
    assert!(parse_stream(SCHEMA_DRIFT).is_err());
}

#[test]
fn persisted_history_reconciles_exact_duplicates_without_a_second_session() {
    let history = parse_history(HISTORY).expect("history is valid");
    let live = parse_stream(SUCCESS).expect("live stream is valid");
    let reconciled = reconcile(history, live).expect("exact duplicates reconcile");
    assert_eq!(reconciled.len(), 6);
    assert_eq!(reconciled[0].id(), "event_1");
    assert_eq!(reconciled[5].id(), "event_6");

    let contradictory = parse_stream(
        "event: session.status_running\n\
         data: {\"id\":\"event_2\",\"type\":\"session.status_running\",\"processed_at\":\"2026-07-21T10:00:00Z\"}\n\n",
    )
    .expect("contradictory fixture is framed");
    assert!(reconcile(parse_history(HISTORY).expect("history"), contradictory).is_err());
}
