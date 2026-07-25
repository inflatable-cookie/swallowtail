use super::{WEBSOCKET_DISCONNECT, WEBSOCKET_SUCCESS, body_method, kind, parse_records};

#[test]
fn websocket_retains_upgrade_cookie_and_preserves_full_duplex_order() {
    let records = parse_records(WEBSOCKET_SUCCESS).expect("WebSocket transcript parses");
    assert_eq!(records[0]["headers"]["upgrade"], "websocket");
    assert_eq!(records[1]["status"], 101);
    assert_eq!(
        records[1]["headers"]["acp-connection-id"],
        "connection-private"
    );
    assert!(records[1]["headers"]["set-cookie"].is_string());
    assert_eq!(records[2]["scope"], "connection");
    assert_eq!(records[2]["retained"], true);

    let text = records
        .iter()
        .filter(|record| matches!(kind(record), "client_text" | "server_text"))
        .collect::<Vec<_>>();
    assert_eq!(kind(text[0]), "client_text");
    assert_eq!(body_method(text[0]), Some("initialize"));
    assert_eq!(text[1]["body"]["result"]["protocolVersion"], 1);
    assert_eq!(body_method(text[2]), Some("session/new"));
    assert_eq!(body_method(text[4]), Some("session/prompt"));
    assert_eq!(body_method(text[5]), Some("session/update"));

    let callback = text
        .iter()
        .find(|record| body_method(record) == Some("session/request_permission"))
        .expect("callback request exists");
    let response = text
        .iter()
        .find(|record| {
            kind(record) == "client_text"
                && record["body"]["id"] == callback["body"]["id"]
                && record["body"].get("result").is_some()
        })
        .expect("callback response is correlated");
    assert_eq!(
        response["body"]["result"]["outcome"]["outcome"],
        "cancelled"
    );
    assert_eq!(body_method(text[9]), Some("session/cancel"));
    assert_eq!(records[13]["kind"], "client_close");
    assert_eq!(records[14]["cookie_state_discarded"], true);
    assert_eq!(records[14]["owned_work_joined"], true);
}

#[test]
fn disconnect_invalidates_one_connection_without_recovery_or_deletion_claims() {
    let records =
        parse_records(WEBSOCKET_DISCONNECT).expect("WebSocket disconnect transcript parses");
    let disconnect = records.last().expect("disconnect exists");
    assert_eq!(disconnect["connection_invalidated"], true);
    assert_eq!(disconnect["session_deletion_claimed"], false);
    for field in [
        "retry",
        "reconnect",
        "replay",
        "resumption",
        "transport_fallback",
    ] {
        assert_eq!(disconnect[field], false);
    }
    assert_eq!(disconnect["owned_work_must_join"], true);
}
