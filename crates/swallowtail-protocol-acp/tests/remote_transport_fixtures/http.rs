use super::{HTTP_DISCONNECT, HTTP_INVALID, HTTP_SUCCESS, body_method, kind, parse_records};
use serde_json::Value;

#[test]
fn streamable_http_uses_two_sse_scopes_cookie_affinity_and_correlated_202_responses() {
    let records = parse_records(HTTP_SUCCESS).expect("HTTP transcript parses");
    assert_eq!(records.len(), 20);
    assert_eq!(records[0]["http_version"], "2");
    assert_eq!(records[0]["method"], "POST");
    assert_eq!(body_method(&records[0]), Some("initialize"));
    assert_eq!(records[1]["status"], 200);
    assert_eq!(
        records[1]["headers"]["acp-connection-id"],
        records[1]["body"]["result"]["connectionId"]
    );
    assert_eq!(records[1]["body"]["result"]["protocolVersion"], 1);

    for record in records
        .iter()
        .skip(2)
        .filter(|record| kind(record) == "client_request")
    {
        assert_eq!(record["headers"]["cookie"], "affinity=opaque-cookie");
        assert_eq!(record["headers"]["acp-connection-id"], "connection-private");
    }

    let stream_scopes = records
        .iter()
        .filter(|record| kind(record) == "server_stream_open")
        .map(|record| record["scope"].as_str().expect("scope is text"))
        .collect::<Vec<_>>();
    assert_eq!(stream_scopes, ["connection", "session"]);

    let accepted = records
        .iter()
        .filter(|record| kind(record) == "server_response" && record["status"] == 202)
        .count();
    assert_eq!(accepted, 5);

    let session_new = records
        .iter()
        .find(|record| body_method(record) == Some("session/new"))
        .expect("session/new exists");
    assert!(session_new["headers"].get("acp-session-id").is_none());
    let session_created = records
        .iter()
        .find(|record| record["scope"] == "connection" && record["body"]["id"] == 2)
        .expect("session response uses connection stream");
    assert_eq!(
        session_created["body"]["result"]["sessionId"],
        "session-private"
    );

    for method in ["session/prompt", "session/cancel"] {
        let record = records
            .iter()
            .find(|record| body_method(record) == Some(method))
            .expect("session request exists");
        assert_eq!(record["headers"]["acp-session-id"], "session-private");
    }
    let callback = records
        .iter()
        .find(|record| record["body"]["method"] == "session/request_permission")
        .expect("callback request exists");
    let callback_response = records
        .iter()
        .find(|record| {
            kind(record) == "client_request"
                && record["body"]["id"] == callback["body"]["id"]
                && record["body"].get("result").is_some()
        })
        .expect("callback response is correlated");
    assert_eq!(
        callback_response["headers"]["acp-session-id"],
        "session-private"
    );

    assert_eq!(records[18]["method"], "DELETE");
    assert_eq!(records[19]["connection_closed"], true);
    assert_eq!(records[19]["streams_closed"], true);
}

#[test]
fn invalid_http_headers_and_batching_fail_with_exact_transport_status() {
    let records = parse_records(HTTP_INVALID).expect("invalid HTTP transcript parses");
    let expected = [
        ("missing-connection", 400),
        ("unknown-connection", 404),
        ("missing-session", 400),
        ("wrong-content-type", 415),
        ("missing-sse-accept", 406),
        ("batch", 501),
    ];
    for (case, status) in expected {
        let response = records
            .iter()
            .find(|record| kind(record) == "server_response" && record["case"] == case)
            .expect("case response exists");
        assert_eq!(response["status"], status);
    }
}

#[test]
fn incomplete_sse_invalidates_the_connection_without_a_deletion_claim() {
    let payload = HTTP_DISCONNECT
        .strip_prefix("data: ")
        .expect("SSE data prefix exists");
    assert!(serde_json::from_str::<Value>(payload).is_err());
}
