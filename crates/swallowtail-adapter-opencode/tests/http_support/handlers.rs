fn handle(mut stream: TcpStream, fixture: StreamFixture, state: HandleState, server_version: &str) {
    let HandleState {
        requests,
        aborted,
        callback_replies,
        health_requests,
        stop,
    } = state;
    stream
        .set_nonblocking(false)
        .expect("accepted fixture stream is blocking");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout sets");
    let Some((target, body)) = read_request(&mut stream) else {
        return;
    };
    let recorded = if body.is_empty() {
        target.clone()
    } else {
        format!("{target}\n{body}")
    };
    requests
        .lock()
        .expect("fixture request lock poisoned")
        .push(recorded);
    let path = target
        .split_once(' ')
        .map_or(target.as_str(), |(_, target)| target);
    if path.starts_with("/global/health") {
        let health_request = health_requests.fetch_add(1, Ordering::SeqCst);
        let observed_version =
            if matches!(fixture, StreamFixture::DeleteHealthDrift) && health_request >= 2 {
                "1.18.5"
            } else {
                server_version
            };
        respond_json(
            &mut stream,
            200,
            &serde_json::json!({"healthy": true, "version": observed_version}).to_string(),
        );
    } else if path.starts_with("/provider") {
        let fixture: serde_json::Value =
            serde_json::from_str(HTTP_SUCCESS).expect("fixture parses");
        respond_json(
            &mut stream,
            200,
            &fixture[1]["response"]["body"].to_string(),
        );
    } else if path.starts_with("/session?") {
        let fixture: serde_json::Value =
            serde_json::from_str(HTTP_SUCCESS).expect("fixture parses");
        let mut body = fixture[2]["response"]["body"].clone();
        body["version"] = serde_json::Value::String(server_version.to_owned());
        respond_json(&mut stream, 200, &body.to_string());
    } else if target.starts_with("GET ") && path.starts_with("/session/ses_fixture/message?") {
        if path.contains("before=") {
            respond_json(
                &mut stream,
                200,
                &message_page(&[("msg_1", "Earlier question."), ("msg_2", "Earlier answer.")])
                    .to_string(),
            );
        } else {
            respond_json_with_cursor(
                &mut stream,
                &message_page(&[("msg_3", "Later question."), ("msg_4", "Later answer.")])
                    .to_string(),
                "cursor-older",
            );
        }
    } else if target.starts_with("GET ") && path.starts_with("/session/ses_fixture?") {
        respond_json(
            &mut stream,
            200,
            &serde_json::json!({"id":"ses_fixture","version":server_version}).to_string(),
        );
    } else if target.starts_with("DELETE ") && path.starts_with("/session/") {
        match fixture {
            StreamFixture::DeleteMissing => respond_json(
                &mut stream,
                404,
                r#"{"error":"private missing-target detail"}"#,
            ),
            StreamFixture::DeleteUnauthorized => respond_json(
                &mut stream,
                401,
                r#"{"error":"private authorization detail"}"#,
            ),
            StreamFixture::DeleteServerError => {
                respond_json(&mut stream, 500, r#"{"error":"private server detail"}"#)
            }
            StreamFixture::DeleteMalformedSuccess => respond_json(&mut stream, 200, "false"),
            StreamFixture::DeleteDisconnect => {}
            StreamFixture::DeleteDelayed => {
                thread::sleep(Duration::from_millis(100));
                respond_json(&mut stream, 200, "true");
            }
            StreamFixture::DeleteHealthDrift => respond_json(&mut stream, 200, "true"),
            _ => respond_json(&mut stream, 200, "true"),
        }
    } else if path.contains("/prompt_async?") {
        respond_empty(&mut stream, 204);
    } else if path.starts_with("/permission/per_fixture/reply?")
        || path.starts_with("/question/que_fixture/reply?")
        || path.starts_with("/question/que_fixture/reject?")
    {
        callback_replies.fetch_add(1, Ordering::SeqCst);
        respond_json(&mut stream, 200, "true");
    } else if path.contains("/abort?") {
        aborted.store(true, Ordering::SeqCst);
        respond_json(&mut stream, 200, "true");
    } else if path.starts_with("/event?") {
        respond_sse(&mut stream, fixture, &aborted, &callback_replies, &stop);
    } else {
        respond_json(&mut stream, 404, r#"{"error":"private fixture payload"}"#);
    }
}

