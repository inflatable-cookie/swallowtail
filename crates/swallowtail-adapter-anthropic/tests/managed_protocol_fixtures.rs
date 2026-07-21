mod support;

use serde_json::Value;
use support::{ManagedFixtureServer, exchange};

const ROOT: &str = "tests/fixtures/managed-agents-2026-04-01";

fn fixture(name: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(ROOT)
            .join(name),
    )
    .unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
}

fn request(method: &str, target: &str, body: Option<&str>) -> Vec<u8> {
    let body = body.unwrap_or_default();
    let content = if method == "POST" {
        "content-type: application/json\r\n"
    } else {
        ""
    };
    format!(
        "{method} {target} HTTP/1.1\r\nHost: fixture\r\nx-api-key: managed-fixture-secret\r\nanthropic-version: 2023-06-01\r\nanthropic-beta: managed-agents-2026-04-01\r\n{content}content-length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

#[test]
fn loopback_transcript_enforces_one_session_reconciliation_callbacks_and_ordered_cleanup() {
    let server = ManagedFixtureServer::start();

    assert_eq!(
        exchange(
            server.endpoint(),
            &request("GET", "/v1/agents/agent_fixture?version=7", None),
        )
        .status,
        200
    );
    assert_eq!(
        exchange(
            server.endpoint(),
            &request(
                "POST",
                "/v1/environments",
                Some(&fixture("environment-create.json")),
            ),
        )
        .status,
        200
    );
    assert_eq!(
        exchange(
            server.endpoint(),
            &request(
                "POST",
                "/v1/sessions",
                Some(&fixture("session-create.json")),
            ),
        )
        .status,
        200
    );
    assert_eq!(
        exchange(
            server.endpoint(),
            &request(
                "POST",
                "/v1/sessions/session_fixture/events",
                Some(&fixture("user-message.json")),
            ),
        )
        .status,
        200
    );

    let stream = exchange(
        server.endpoint(),
        &request("GET", "/v1/sessions/session_fixture/events/stream", None),
    );
    assert_eq!(stream.status, 200);
    assert_eq!(stream.headers["content-type"], "text/event-stream");
    assert_eq!(stream.body, fixture("success.sse").as_bytes());

    let history = exchange(
        server.endpoint(),
        &request(
            "GET",
            "/v1/sessions/session_fixture/events?limit=1000&order=asc",
            None,
        ),
    );
    let history: Value = serde_json::from_slice(&history.body).expect("history is JSON");
    assert_eq!(history["next_page"], Value::Null);

    assert_eq!(
        exchange(
            server.endpoint(),
            &request(
                "POST",
                "/v1/sessions/session_fixture/events",
                Some(&fixture("custom-tool-result.json")),
            ),
        )
        .status,
        200
    );
    assert_eq!(
        exchange(
            server.endpoint(),
            &request(
                "POST",
                "/v1/sessions/session_fixture/events",
                Some(&fixture("interrupt.json")),
            ),
        )
        .status,
        200
    );

    let premature = exchange(
        server.endpoint(),
        &request("DELETE", "/v1/environments/env_fixture", None),
    );
    assert_eq!(premature.status, 409);
    assert!(String::from_utf8_lossy(&premature.body).contains("fixture-secret-never-log"));

    assert_eq!(
        exchange(
            server.endpoint(),
            &request("DELETE", "/v1/sessions/session_fixture", None),
        )
        .status,
        200
    );
    assert_eq!(
        exchange(
            server.endpoint(),
            &request("DELETE", "/v1/environments/env_fixture", None),
        )
        .status,
        200
    );

    let state = server.state();
    assert!(state.environment_created);
    assert_eq!(state.session_creations, 1);
    assert_eq!(state.stream_attachments, 1);
    assert!(state.session_deleted);
    assert!(state.environment_deleted);
    assert_eq!(server.requests().len(), 11);
}

#[test]
fn loopback_rejects_missing_beta_authority_before_any_resource_effect() {
    let server = ManagedFixtureServer::start();
    let raw = b"POST /v1/environments HTTP/1.1\r\nHost: fixture\r\nx-api-key: managed-fixture-secret\r\nanthropic-version: 2023-06-01\r\ncontent-type: application/json\r\ncontent-length: 2\r\nConnection: close\r\n\r\n{}";
    assert_eq!(exchange(server.endpoint(), raw).status, 400);
    assert_eq!(server.state(), Default::default());
}
