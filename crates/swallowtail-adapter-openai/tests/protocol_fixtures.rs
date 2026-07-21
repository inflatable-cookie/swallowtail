mod support;

use serde_json::Value;
use support::{FixtureServer, exchange};

const CREATE: &str = include_str!("fixtures/openai-responses-2026-07-21/create-request.json");

#[test]
fn loopback_endpoint_separates_one_inference_attempt_from_management_requests() {
    let server = FixtureServer::start();

    let created = exchange(
        server.endpoint(),
        &request("POST", "/v1/responses", Some(CREATE)),
    );
    assert_eq!(created.status, 200);
    assert_eq!(created.headers["content-type"], "text/event-stream");
    assert!(
        String::from_utf8(created.body)
            .expect("SSE is UTF-8")
            .contains("response.created")
    );

    let retrieved = exchange(
        server.endpoint(),
        &request("GET", "/v1/responses/resp_fixture_123", None),
    );
    assert_eq!(retrieved.status, 200);
    let snapshot: Value = serde_json::from_slice(&retrieved.body).expect("snapshot is JSON");
    assert_eq!(snapshot["status"], "in_progress");

    let reattached = exchange(
        server.endpoint(),
        &request(
            "GET",
            "/v1/responses/resp_fixture_123?stream=true&starting_after=3",
            None,
        ),
    );
    assert_eq!(reattached.status, 200);
    assert!(
        String::from_utf8(reattached.body)
            .expect("SSE is UTF-8")
            .contains("\"sequence_number\":4")
    );

    let cancelled = exchange(
        server.endpoint(),
        &request("POST", "/v1/responses/resp_fixture_123/cancel", None),
    );
    assert_eq!(cancelled.status, 200);
    let snapshot: Value = serde_json::from_slice(&cancelled.body).expect("cancel is JSON");
    assert_eq!(snapshot["status"], "cancelled");

    assert_eq!(server.inference_attempts(), 1);
    assert_eq!(server.requests().len(), 4);
}

#[test]
fn loopback_endpoint_rejects_a_second_create_attempt_and_wrong_access() {
    let server = FixtureServer::start();
    assert_eq!(
        exchange(
            server.endpoint(),
            &request("POST", "/v1/responses", Some(CREATE))
        )
        .status,
        200
    );
    assert_eq!(
        exchange(
            server.endpoint(),
            &request("POST", "/v1/responses", Some(CREATE))
        )
        .status,
        409
    );
    assert_eq!(server.inference_attempts(), 2);

    let unauthorised = b"GET /v1/responses/resp_fixture_123 HTTP/1.1\r\nHost: fixture\r\nConnection: close\r\n\r\n";
    assert_eq!(exchange(server.endpoint(), unauthorised).status, 401);
}

fn request(method: &str, target: &str, body: Option<&str>) -> Vec<u8> {
    let body = body.unwrap_or("").trim();
    format!(
        "{method} {target} HTTP/1.1\r\nHost: fixture\r\nAuthorization: Bearer fixture-secret\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}
