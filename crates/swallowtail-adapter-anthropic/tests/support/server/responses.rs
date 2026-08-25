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
        ("POST", "/v1/messages") => {
            let attempt = attempts.fetch_add(1, Ordering::SeqCst);
            match (fixture, attempt) {
                (StreamFixture::WaitForCancel, 0) => respond_wait_for_cancel(stream),
                (StreamFixture::ToolContinuation, 0) => {
                    respond_sse(stream, &terminate_sse(TOOL_USE))
                }
                (StreamFixture::ThinkingToolContinuation, 0) => {
                    respond_sse(stream, &terminate_sse(THINKING_TOOL_USE))
                }
                (StreamFixture::RedactedToolContinuation, 0) => {
                    respond_sse(stream, &terminate_sse(REDACTED_TOOL_USE))
                }
                (StreamFixture::ConsecutiveThinkingToolContinuation, 0) => {
                    respond_sse(stream, &terminate_sse(CONSECUTIVE_THINKING_TOOL_USE))
                }
                (StreamFixture::LateThinkingAfterTool, 0) => {
                    respond_sse(stream, &terminate_sse(LATE_THINKING))
                }
                (StreamFixture::LateRedactedAfterTool, 0) => {
                    respond_sse(stream, &terminate_sse(LATE_REDACTED))
                }
                (StreamFixture::DuplicateThinkingSignature, 0) => {
                    respond_sse(stream, &terminate_sse(DUPLICATE_SIGNATURE))
                }
                (StreamFixture::OversizedThinkingSignature, 0) => {
                    respond_sse(stream, &oversized_thinking_signature())
                }
                (
                    StreamFixture::ToolContinuation
                    | StreamFixture::ThinkingToolContinuation
                    | StreamFixture::RedactedToolContinuation
                    | StreamFixture::ConsecutiveThinkingToolContinuation,
                    1 | 2,
                ) => respond_sse(stream, &terminate_sse(SUCCESS)),
                (StreamFixture::WebSearch, 0) => {
                    respond_sse(stream, &terminate_sse(WEB_SEARCH))
                }
                (StreamFixture::Disconnect, 0) => respond_sse(stream, DISCONNECT),
                (_, 0) => respond_sse(stream, &terminate_sse(stream_body(fixture))),
                _ => respond_json(
                    stream,
                    409,
                    r#"{"type":"error","error":{"type":"conflict_error","message":"fixture allows one inference attempt"}}"#,
                ),
            }
        }
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
        StreamFixture::ThinkingThenText => THINKING_THEN_TEXT,
        StreamFixture::ThinkingDelta => THINKING_DELTA,
        StreamFixture::ThinkingUnsigned => THINKING_UNSIGNED,
        StreamFixture::ThinkingAfterText => THINKING_AFTER_TEXT,
        StreamFixture::WaitForCancel
        | StreamFixture::Disconnect
        | StreamFixture::ToolContinuation
        | StreamFixture::WebSearch
        | StreamFixture::ThinkingToolContinuation
        | StreamFixture::RedactedToolContinuation
        | StreamFixture::ConsecutiveThinkingToolContinuation
        | StreamFixture::LateThinkingAfterTool
        | StreamFixture::LateRedactedAfterTool
        | StreamFixture::DuplicateThinkingSignature
        | StreamFixture::OversizedThinkingSignature => unreachable!(),
    }
}

fn terminate_sse(body: &str) -> String {
    if body.ends_with("\n\n") {
        body.to_owned()
    } else if body.ends_with('\n') {
        format!("{body}\n")
    } else {
        format!("{body}\n\n")
    }
}

fn oversized_thinking_signature() -> String {
    let signature = "x".repeat(262_145);
    terminate_sse(&format!(
        "event: message_start\ndata: {{\"type\":\"message_start\",\"message\":{{\"id\":\"msg_fixture_overflow\",\"type\":\"message\",\"role\":\"assistant\",\"content\":[],\"model\":\"claude-opus-4-7\",\"stop_reason\":null,\"usage\":{{\"input_tokens\":12,\"output_tokens\":1}}}}}}\n\nevent: content_block_start\ndata: {{\"type\":\"content_block_start\",\"index\":0,\"content_block\":{{\"type\":\"thinking\",\"thinking\":\"\",\"signature\":\"\"}}}}\n\nevent: content_block_delta\ndata: {{\"type\":\"content_block_delta\",\"index\":0,\"delta\":{{\"type\":\"signature_delta\",\"signature\":\"{signature}\"}}}}\n\nevent: content_block_stop\ndata: {{\"type\":\"content_block_stop\",\"index\":0}}\n\nevent: content_block_start\ndata: {{\"type\":\"content_block_start\",\"index\":1,\"content_block\":{{\"type\":\"tool_use\",\"id\":\"toolu_fixture_1\",\"name\":\"lookup_customer\",\"input\":{{}}}}}}\n\nevent: content_block_delta\ndata: {{\"type\":\"content_block_delta\",\"index\":1,\"delta\":{{\"type\":\"input_json_delta\",\"partial_json\":\"{{\\\"customer_id\\\":\\\"customer-fixture\\\"}}\"}}}}\n\nevent: content_block_stop\ndata: {{\"type\":\"content_block_stop\",\"index\":1}}\n\nevent: message_delta\ndata: {{\"type\":\"message_delta\",\"delta\":{{\"stop_reason\":\"tool_use\",\"stop_sequence\":null}},\"usage\":{{\"input_tokens\":12,\"output_tokens\":8}}}}\n\nevent: message_stop\ndata: {{\"type\":\"message_stop\"}}\n"
    ))
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
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nrequest-id: req_fixture_cancel\r\nConnection: close\r\n\r\nevent: message_start\ndata: {{\"type\":\"message_start\",\"message\":{{\"id\":\"msg_fixture_cancel\",\"usage\":{{\"input_tokens\":1,\"output_tokens\":1}}}}}}\n\n"
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
