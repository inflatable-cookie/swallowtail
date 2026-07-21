fn respond(
    stream: &mut TcpStream,
    request: &FixtureRequest,
    attempts: &AtomicUsize,
    fixture: StreamFixture,
) {
    if !authorized(request) {
        return respond_json(
            stream,
            400,
            r#"{"type":"error","error":{"type":"invalid_request_error","message":"fixture rejected headers"}}"#,
        );
    }
    match (request.method.as_str(), request.target.as_str()) {
        ("GET", "/v1/models?limit=2") => respond_json(stream, 200, PAGE_1),
        ("GET", "/v1/models?limit=2&after_id=claude-fixture-secondary") => {
            respond_json(stream, 200, PAGE_2)
        }
        ("POST", "/v1/messages") if attempts.fetch_add(1, Ordering::SeqCst) == 0 => match fixture {
            StreamFixture::WaitForCancel => respond_wait_for_cancel(stream),
            _ => respond_sse(stream, stream_body(fixture)),
        },
        ("POST", "/v1/messages") => respond_json(
            stream,
            409,
            r#"{"type":"error","error":{"type":"conflict_error","message":"fixture allows one inference attempt"}}"#,
        ),
        _ => respond_json(
            stream,
            404,
            r#"{"type":"error","error":{"type":"not_found_error","message":"fixture route not found"}}"#,
        ),
    }
}

fn stream_body(fixture: StreamFixture) -> &'static str {
    match fixture {
        StreamFixture::Success => SUCCESS,
        StreamFixture::MidstreamError => MIDSTREAM_ERROR,
        StreamFixture::Unknown => UNKNOWN,
        StreamFixture::Disconnect => DISCONNECT,
        StreamFixture::WaitForCancel => unreachable!(),
    }
}

fn authorized(request: &FixtureRequest) -> bool {
    request.headers.get("x-api-key").map(String::as_str) == Some("fixture-secret")
        && request.headers.get("anthropic-version").map(String::as_str) == Some("2023-06-01")
        && !request.headers.contains_key("anthropic-beta")
        && (request.method != "POST"
            || request.headers.get("content-type").map(String::as_str) == Some("application/json"))
}

fn respond_json(stream: &mut TcpStream, status: u16, body: &str) {
    respond_with(stream, status, "application/json", body);
}

fn respond_sse(stream: &mut TcpStream, body: &str) {
    respond_with(stream, 200, "text/event-stream", body);
}

fn respond_wait_for_cancel(stream: &mut TcpStream) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nrequest-id: req_fixture_cancel\r\nConnection: close\r\n\r\nevent: message_start\ndata: {{\"type\":\"message_start\",\"message\":{{\"usage\":{{\"input_tokens\":1,\"output_tokens\":1}}}}}}\n\n"
    )
    .expect("waiting stream starts");
    for _ in 0..2_000 {
        thread::sleep(Duration::from_millis(1));
        if stream
            .write_all(b"event: ping\ndata: {\"type\":\"ping\"}\n\n")
            .is_err()
        {
            break;
        }
        let _ = stream.flush();
    }
}

fn respond_with(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let reason = if status == 200 { "OK" } else { "Fixture Failure" };
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nrequest-id: req_fixture_success\r\nanthropic-ratelimit-requests-limit: 100\r\nanthropic-ratelimit-requests-remaining: 99\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("fixture response writes");
}

fn parse_response(bytes: &[u8]) -> FixtureResponse {
    let end = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("fixture response has headers");
    let head = std::str::from_utf8(&bytes[..end]).expect("response headers are utf-8");
    let mut lines = head.lines();
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse().ok())
        .expect("fixture status parses");
    let headers = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
        .collect();
    FixtureResponse {
        status,
        headers,
        body: bytes[end + 4..].to_vec(),
    }
}
