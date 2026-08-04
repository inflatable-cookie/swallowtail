fn message_page(messages: &[(&str, &str)]) -> serde_json::Value {
    serde_json::Value::Array(
        messages
            .iter()
            .enumerate()
            .map(|(index, (id, text))| {
                let role = if index % 2 == 0 { "user" } else { "assistant" };
                serde_json::json!({
                    "info":{"id":id,"sessionID":"ses_fixture","role":role},
                    "parts":[{
                        "id":format!("prt_{id}"),
                        "messageID":id,
                        "sessionID":"ses_fixture",
                        "type":"text",
                        "text":text
                    }]
                })
            })
            .collect(),
    )
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String)> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let count = stream.read(&mut chunk).ok()?;
        if count == 0 {
            return None;
        }
        bytes.extend_from_slice(&chunk[..count]);
        let Some(header_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let headers = std::str::from_utf8(&bytes[..header_end]).ok()?;
        let length = headers
            .lines()
            .find_map(|line| {
                line.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .and_then(|value| value.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        if bytes.len() < header_end + 4 + length {
            continue;
        }
        let target = headers.lines().next()?.to_owned();
        let body_start = header_end + 4;
        let body = String::from_utf8_lossy(&bytes[body_start..body_start + length]).into_owned();
        return Some((target, body));
    }
}

fn respond_json(stream: &mut TcpStream, status: u16, body: &str) {
    let reason = if status == 200 { "OK" } else { "Not Found" };
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("fixture response writes");
}

fn respond_json_with_cursor(stream: &mut TcpStream, body: &str, cursor: &str) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Next-Cursor: {cursor}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .expect("fixture response writes");
}

fn respond_empty(stream: &mut TcpStream, status: u16) {
    write!(
        stream,
        "HTTP/1.1 {status} No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    )
    .expect("fixture response writes");
}

fn respond_sse(
    stream: &mut TcpStream,
    fixture: StreamFixture,
    aborted: &AtomicBool,
    callback_replies: &AtomicUsize,
    stop: &AtomicBool,
) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n"
    )
    .expect("SSE headers write");
    match fixture {
        StreamFixture::Success
        | StreamFixture::ReconciliationActive
        | StreamFixture::DeleteMissing
        | StreamFixture::DeleteUnauthorized
        | StreamFixture::DeleteServerError
        | StreamFixture::DeleteMalformedSuccess
        | StreamFixture::DeleteDisconnect
        | StreamFixture::DeleteDelayed
        | StreamFixture::DeleteHealthDrift
        | StreamFixture::ImportTitleDrift
        | StreamFixture::ImportDelayed => {
            stream.write_all(SUCCESS.as_bytes()).expect("SSE writes")
        }
        StreamFixture::ProviderError => stream
            .write_all(PROVIDER_ERROR.as_bytes())
            .expect("SSE writes"),
        StreamFixture::Unknown => stream.write_all(UNKNOWN.as_bytes()).expect("SSE writes"),
        StreamFixture::Disconnect => stream.write_all(DISCONNECT.as_bytes()).expect("SSE writes"),
        StreamFixture::DuplicateUsage => stream
            .write_all(DUPLICATE_USAGE.as_bytes())
            .expect("SSE writes"),
        StreamFixture::MissingUsage => stream
            .write_all(MISSING_USAGE.as_bytes())
            .expect("SSE writes"),
        StreamFixture::Compaction => stream
            .write_all(COMPACTION.as_bytes())
            .expect("SSE writes"),
        StreamFixture::InputCallbacks => {
            for (event, expected_replies) in [
                (
                    r#"{"id":"evt_permission_1","type":"permission.asked","properties":{"id":"per_fixture","sessionID":"ses_fixture","permission":"edit","patterns":["src/approved.rs"],"metadata":{},"always":["src/**"]}}"#,
                    1,
                ),
                (
                    r#"{"id":"evt_question_1","type":"question.asked","properties":{"id":"que_fixture","sessionID":"ses_fixture","questions":[{"question":"Choose a bounded mode.","header":"Mode","options":[{"label":"Safe","description":"Keep the operation read-only."},{"label":"Stop","description":"Reject the request."}],"multiple":false}]}}"#,
                    2,
                ),
            ] {
                if aborted.load(Ordering::SeqCst) || stop.load(Ordering::SeqCst) {
                    break;
                }
                writeln!(stream, "data: {event}\n").expect("callback SSE writes");
                stream.flush().expect("callback SSE flushes");
                while callback_replies.load(Ordering::SeqCst) < expected_replies
                    && !aborted.load(Ordering::SeqCst)
                    && !stop.load(Ordering::SeqCst)
                {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            if !aborted.load(Ordering::SeqCst) && !stop.load(Ordering::SeqCst) {
                stream.write_all(SUCCESS.as_bytes()).expect("SSE writes");
            }
        }
        StreamFixture::WaitForAbort => {
            stream
                .write_all(
                    b"data: {\"id\":\"evt_1\",\"type\":\"server.connected\",\"properties\":{}}\n\n",
                )
                .expect("connected event writes");
            stream.flush().expect("connected event flushes");
            while !aborted.load(Ordering::SeqCst) && !stop.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(1));
            }
            if aborted.load(Ordering::SeqCst) {
                stream
                    .write_all(ABORTED.as_bytes())
                    .expect("abort SSE writes");
            }
        }
    }
}
