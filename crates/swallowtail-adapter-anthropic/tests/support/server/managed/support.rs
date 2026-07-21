fn respond_managed_wait(stream: &mut TcpStream) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nrequest-id: req_fixture_wait\r\nConnection: close\r\n\r\nevent: session.status_running\ndata: {{\"id\":\"event_wait_running\",\"type\":\"session.status_running\",\"processed_at\":\"2026-07-21T09:00:01Z\"}}\n\n"
    )
    .expect("managed waiting stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream.write_all(b": keepalive\n\n").is_err() {
            break;
        }
        let _ = stream.flush();
    }
}

fn managed_authorized(request: &FixtureRequest) -> bool {
    request.headers.get("x-api-key").map(String::as_str) == Some("managed-fixture-secret")
        && request.headers.get("anthropic-version").map(String::as_str) == Some("2023-06-01")
        && request.headers.get("anthropic-beta").map(String::as_str)
            == Some("managed-agents-2026-04-01")
        && (request.method != "POST"
            || request.headers.get("content-type").map(String::as_str) == Some("application/json"))
}

fn body_matches(request: &FixtureRequest, fixture: &str) -> bool {
    serde_json::from_slice::<serde_json::Value>(&request.body).ok()
        == serde_json::from_str::<serde_json::Value>(fixture).ok()
}

fn managed_session_matches(request: &FixtureRequest) -> bool {
    let Ok(actual) = serde_json::from_slice::<serde_json::Value>(&request.body) else {
        return false;
    };
    let Ok(mut expected) = serde_json::from_str::<serde_json::Value>(MANAGED_SESSION_CREATE) else {
        return false;
    };
    let Some(tools) = actual
        .pointer("/agent/tools")
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    if tools.is_empty() {
        expected["agent"]["tools"] = serde_json::json!([]);
    }
    actual == expected
}

fn session_request_has_tools(request: &FixtureRequest) -> bool {
    serde_json::from_slice::<serde_json::Value>(&request.body)
        .ok()
        .and_then(|value| {
            value
                .pointer("/agent/tools")
                .and_then(|tools| tools.as_array())
                .cloned()
        })
        .is_some_and(|tools| !tools.is_empty())
}
