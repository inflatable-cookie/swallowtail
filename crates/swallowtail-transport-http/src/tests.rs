use crate::stream::result_channel;
use crate::{
    BodyDecoder, HttpRequest, HttpTransport, Method, SecretHeader, SseDecodeError,
    SseDecodeErrorKind, SseDecoder, SseFrame, SseFramer, StreamError, StreamItem, Subscription,
    TransportError,
};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use std::thread;
use std::time::Duration;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, HostServices, RuntimeFailure, ScopeId,
};

struct ThreadedBlocking;

impl BlockingWorkService for ThreadedBlocking {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let (sender, receiver) = result_channel();
        thread::spawn(move || {
            let _ = sender.send(job());
        });
        Box::pin(async move {
            receiver.recv().await.ok_or_else(|| {
                RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.blocking_panicked",
                    "Fixture blocking work panicked",
                ))
            })?
        })
    }
}

fn services() -> HostServices {
    HostServices::new(ExecutionHostId::new("transport-fixture").expect("fixture host id is valid"))
        .with_blocking_work(Arc::new(ThreadedBlocking))
}

fn scope() -> ScopeId {
    ScopeId::new("transport-fixture-scope").expect("fixture scope is valid")
}

fn cancelled() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

fn serve(responder: impl FnOnce(TcpStream) + Send + 'static) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
    let endpoint = format!(
        "http://{}",
        listener.local_addr().expect("fixture address is available")
    );
    thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            responder(stream);
        }
    });
    endpoint
}

fn read_request(stream: &mut TcpStream) {
    let mut request = Vec::new();
    let mut chunk = [0u8; 1024];
    while !request.windows(4).any(|window| window == b"\r\n\r\n") {
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => request.extend_from_slice(&chunk[..read]),
        }
    }
}

fn write_all(stream: &mut TcpStream, bytes: &[u8]) {
    let _ = stream.write_all(bytes);
    let _ = stream.flush();
}

fn read_request_line_and_body(stream: &mut TcpStream) -> (String, Vec<u8>) {
    let mut request = Vec::new();
    let mut chunk = [0u8; 1024];
    while !request.windows(4).any(|window| window == b"\r\n\r\n") {
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => request.extend_from_slice(&chunk[..read]),
        }
    }
    let body_start = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map_or(request.len(), |end| end + 4);
    let head = String::from_utf8_lossy(&request[..body_start]).into_owned();
    let declared = head
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    let mut body = request[body_start..].to_vec();
    while body.len() < declared {
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => body.extend_from_slice(&chunk[..read]),
        }
    }
    let request_line = head
        .lines()
        .next()
        .unwrap_or_default()
        .trim_end()
        .to_owned();
    (request_line, body)
}

#[test]
fn unary_exchange_returns_the_bounded_response() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        write_all(
            &mut stream,
            b"HTTP/1.1 200 OK\r\nX-Request-Id: fixture-id\r\nContent-Length: 2\r\n\r\nok",
        );
    });
    let response = futures_executor::block_on(HttpTransport::new().execute(
        scope(),
        &endpoint,
        HttpRequest::new(Method::Get, "/route").capture_headers(|name| name == "x-request-id"),
        &services(),
        cancelled(),
    ))
    .expect("fixture response is returned");
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"ok");
    assert_eq!(
        response.headers.get("x-request-id").map(String::as_str),
        Some("fixture-id")
    );
}

#[test]
fn write_verbs_attach_their_request_bodies() {
    for (method, verb) in [(Method::Put, "PUT"), (Method::Patch, "PATCH")] {
        let endpoint = serve(move |mut stream| {
            let (request_line, body) = read_request_line_and_body(&mut stream);
            let response_body = format!("{request_line}\n{}", String::from_utf8_lossy(&body));
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                response_body.len()
            );
            write_all(&mut stream, header.as_bytes());
            write_all(&mut stream, response_body.as_bytes());
        });
        let response = futures_executor::block_on(HttpTransport::new().execute(
            scope(),
            &endpoint,
            HttpRequest::new(method, "/route").with_body(b"payload".to_vec()),
            &services(),
            cancelled(),
        ))
        .expect("fixture response is returned");
        assert_eq!(
            String::from_utf8_lossy(&response.body),
            format!("{verb} /route HTTP/1.1\npayload"),
            "{verb} must carry its body and its own request line"
        );
    }
}

#[test]
fn response_limit_aborts_instead_of_truncating() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        let body = vec![b'x'; 65];
        let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
        write_all(&mut stream, header.as_bytes());
        write_all(&mut stream, &body);
    });
    let error = futures_executor::block_on(HttpTransport::new().execute(
        scope(),
        &endpoint,
        HttpRequest::new(Method::Get, "/overflow").with_response_limit(64),
        &services(),
        cancelled(),
    ))
    .expect_err("an oversized response aborts");
    assert_eq!(error, TransportError::ResponseLimit);
}

#[test]
fn finite_exchanges_honour_an_explicit_transfer_timeout() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        thread::sleep(Duration::from_secs(2));
    });
    let error = futures_executor::block_on(HttpTransport::new().execute(
        scope(),
        &endpoint,
        HttpRequest::new(Method::Get, "/slow").with_transfer_timeout(Duration::from_millis(200)),
        &services(),
        cancelled(),
    ))
    .expect_err("a wedged peer times out");
    assert_eq!(error, TransportError::Transport);
}

#[test]
fn streaming_backpressure_delivers_every_frame_beyond_the_channel_capacity() {
    const FRAMES: usize = 200;
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        write_all(&mut stream, b"HTTP/1.1 200 OK\r\n\r\n");
        let mut body = Vec::new();
        for index in 0..FRAMES {
            body.extend_from_slice(format!("event: delta\ndata: {index}\n\n").as_bytes());
        }
        write_all(&mut stream, &body);
    });
    let mut subscription = HttpTransport::new()
        .subscribe(
            scope(),
            &endpoint,
            HttpRequest::new(Method::Get, "/events").stream_response_headers(),
            SseDecoder::new(),
            &services(),
            cancelled(),
        )
        .expect("fixture subscription starts");
    let items = futures_executor::block_on(async {
        let mut items = Vec::new();
        loop {
            let next =
                futures_util::future::poll_fn(|context| match subscription.poll_next(context) {
                    Poll::Ready(item) => Poll::Ready(item),
                    Poll::Pending => Poll::Pending,
                })
                .await;
            match next {
                Some(Ok(item)) => items.push(item),
                Some(Err(error)) => panic!("stream failed: {error:?}"),
                None => break,
            }
        }
        items
    });
    let frames = items
        .iter()
        .filter(|item| matches!(item, StreamItem::Body(_)))
        .count();
    assert_eq!(
        frames, FRAMES,
        "backpressure must not abort a stream that outruns the channel capacity"
    );
}

#[test]
fn non_success_stream_reports_the_bounded_error_body() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        write_all(
            &mut stream,
            b"HTTP/1.1 429 Too Many Requests\r\nContent-Length: 7\r\n\r\nlimited",
        );
    });
    let mut subscription = HttpTransport::new()
        .subscribe(
            scope(),
            &endpoint,
            HttpRequest::new(Method::Get, "/events"),
            SseDecoder::new(),
            &services(),
            cancelled(),
        )
        .expect("fixture subscription starts");
    let error = futures_executor::block_on(async {
        futures_util::future::poll_fn(|context| match subscription.poll_next(context) {
            Poll::Ready(item) => Poll::Ready(item),
            Poll::Pending => Poll::Pending,
        })
        .await
    })
    .expect("the rejection is surfaced")
    .expect_err("a rejection is a stream error");
    match error {
        StreamError::Status { status, body, .. } => {
            assert_eq!(status, 429);
            assert_eq!(body, b"limited");
        }
        other => panic!("expected a status failure, got {other:?}"),
    }
}

#[test]
fn a_terminal_stream_failure_is_polled_once_and_then_fused() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        write_all(
            &mut stream,
            b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 4\r\n\r\nfail",
        );
    });
    let mut subscription = HttpTransport::new()
        .subscribe(
            scope(),
            &endpoint,
            HttpRequest::new(Method::Get, "/events"),
            SseDecoder::new(),
            &services(),
            cancelled(),
        )
        .expect("fixture subscription starts");
    let poll = |subscription: &mut Subscription<SseFrame, SseDecodeError>| {
        futures_executor::block_on(futures_util::future::poll_fn(|context| match subscription
            .poll_next(context)
        {
            Poll::Ready(item) => Poll::Ready(item),
            Poll::Pending => Poll::Pending,
        }))
    };
    let failure = poll(&mut subscription).expect("the rejection is surfaced");
    assert!(failure.is_err(), "a rejection is a stream error");
    assert!(
        poll(&mut subscription).is_none(),
        "a terminal outcome must fuse the subscription instead of repeating the failure"
    );
}

#[test]
fn dropping_a_subscription_cancels_its_worker() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        write_all(&mut stream, b"HTTP/1.1 200 OK\r\n\r\n");
        write_all(&mut stream, b"event: delta\ndata: held\n\n");
        thread::sleep(Duration::from_secs(2));
    });
    let subscription = HttpTransport::new()
        .subscribe(
            scope(),
            &endpoint,
            HttpRequest::new(Method::Get, "/events"),
            SseDecoder::new(),
            &services(),
            cancelled(),
        )
        .expect("fixture subscription starts");
    let cancellation = subscription.cancellation();
    assert!(!cancellation.load(Ordering::SeqCst));
    drop(subscription);
    assert!(
        cancellation.load(Ordering::SeqCst),
        "dropping a subscription must cancel its blocking worker"
    );
}

#[test]
fn secret_headers_reject_header_injection_and_non_utf8_credentials() {
    assert_eq!(
        SecretHeader::bearer(b"secret\r\nx-injected: 1".to_vec()).expect_err("line breaks fail"),
        TransportError::CredentialInvalid
    );
    assert_eq!(
        SecretHeader::bearer(vec![0xff]).expect_err("non-UTF-8 fails"),
        TransportError::CredentialInvalid
    );
    assert_eq!(
        SecretHeader::bearer(Vec::new()).expect_err("an empty credential fails"),
        TransportError::CredentialInvalid
    );
    assert!(SecretHeader::bearer(b"secret".to_vec()).is_ok());
}

#[test]
fn endpoints_reject_credentials_queries_and_fragments() {
    let request = HttpRequest::new(Method::Get, "/v1/models").with_query("limit", "2");
    let url = request
        .resolve("https://provider.example/base/")
        .expect("an eligible endpoint resolves");
    assert_eq!(
        url.as_str(),
        "https://provider.example/base/v1/models?limit=2"
    );
    for rejected in [
        "ftp://provider.example",
        "https://user@provider.example",
        "https://provider.example?token=secret",
        "https://provider.example#fragment",
    ] {
        assert_eq!(
            request.resolve(rejected).expect_err("ineligible endpoint"),
            TransportError::EndpointInvalid
        );
    }
}

#[test]
fn sse_decoder_frames_names_payloads_and_keep_alives() {
    let mut decoder = SseDecoder::new();
    let frames = decoder
        .push(b": keep-alive\n\nevent: delta\r\ndata: first\r\ndata: second\r\n\r\n")
        .expect("frames decode");
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].name(), Some("delta"));
    assert_eq!(frames[0].data(), b"first\nsecond");
    decoder.finish().expect("drained stream finishes");
}

#[test]
fn sse_framer_applies_the_event_limit_to_each_event_in_a_chunk() {
    let mut framer = SseFramer::new().with_event_limit(b"data: one\n\n".len());
    let frames = framer
        .push(b"data: one\n\ndata: two\n\n")
        .expect("each complete event fits the event bound");
    assert_eq!(frames.len(), 2, "one chunk may carry several events");
    framer
        .push(b"data: three")
        .expect("a retained partial event fits the bound");
    let error = framer
        .push(b"data: four-oversized")
        .expect_err("an oversized retained event fails");
    assert_eq!(error.kind(), SseDecodeErrorKind::LimitExceeded);
}

#[test]
fn sse_framer_rejects_oversized_complete_frames() {
    let event_limit = b"data: 1234\n\n".len();
    let mut framer = SseFramer::new().with_event_limit(event_limit);
    let error = framer
        .push(b"data: ok\n\ndata: 12345\n\n")
        .expect_err("an oversized complete event fails even after a bounded event");
    assert_eq!(error.kind(), SseDecodeErrorKind::LimitExceeded);

    let mut framer = SseFramer::new().with_event_limit(event_limit);
    framer
        .push(b"data: 1234\r\n")
        .expect("a retained partial event fits the bound");
    let error = framer
        .push(b"\r\n")
        .expect_err("the complete event exceeds the bound before CRLF normalization");
    assert_eq!(error.kind(), SseDecodeErrorKind::LimitExceeded);
}

#[test]
fn sse_decoder_bounds_events_and_rejects_partial_streams() {
    let mut decoder = SseDecoder::with_framer(SseFramer::new().with_event_limit(8));
    let error = decoder
        .push(b"data: oversized")
        .expect_err("an oversized retained event fails");
    assert_eq!(error.kind(), SseDecodeErrorKind::LimitExceeded);

    let mut decoder = SseDecoder::new();
    decoder
        .push(b"event: delta\ndata: partial")
        .expect("prefix decodes");
    let error = decoder.finish().expect_err("a partial stream fails");
    assert_eq!(error.kind(), SseDecodeErrorKind::IncompleteFrame);

    let mut decoder = SseDecoder::new();
    let error = decoder
        .push(b"\xff\xfe\n\n")
        .expect_err("invalid UTF-8 fails");
    assert_eq!(error.kind(), SseDecodeErrorKind::InvalidUtf8);
}

#[test]
fn raw_sse_frames_are_delimited_and_normalised() {
    let mut framer = SseFramer::new();
    let frames = framer
        .push(b"event: first\r\ndata: one\r\n\r\nevent: second\ndata: two\n\n")
        .expect("frames decode");
    assert_eq!(
        frames,
        vec![
            b"event: first\ndata: one\n\n".to_vec(),
            b"event: second\ndata: two\n\n".to_vec(),
        ]
    );
}

#[test]
fn sse_frame_diagnostics_redact_the_payload() {
    let mut decoder = SseDecoder::new();
    let frames = decoder
        .push(b"event: delta\ndata: private-payload\n\n")
        .expect("frame decodes");
    let dump = format!("{frames:?}");
    assert!(!dump.contains("private-payload"));
    let error = SseDecodeError::new(SseDecodeErrorKind::IncompleteFrame);
    assert!(!format!("{error} {error:?}").contains("private-payload"));
}

#[test]
fn body_decoder_trait_is_implemented_for_the_shared_decoder() {
    let mut decoder = SseDecoder::new();
    let frames = BodyDecoder::push(&mut decoder, b"data: item\n\n").expect("frame decodes");
    assert_eq!(frames.len(), 1);
    BodyDecoder::finish(decoder).expect("drained decoder finishes");
}

#[test]
fn routes_cannot_escape_the_endpoint_base_path() {
    let endpoint = "https://provider.example/base/v1";
    for escaping in [
        "../../other",
        "../../../secret",
        "/../../other",
        "%2e%2e/%2e%2e/other",
    ] {
        assert_eq!(
            HttpRequest::new(Method::Get, escaping)
                .resolve(endpoint)
                .expect_err("a route that climbs above the base is rejected"),
            TransportError::EndpointInvalid,
            "route {escaping}"
        );
    }
    let inside = HttpRequest::new(Method::Get, "a/../models")
        .resolve(endpoint)
        .expect("a dot segment that stays under the base resolves");
    assert_eq!(inside.as_str(), "https://provider.example/base/v1/models");
}

#[test]
fn non_secret_headers_reject_invalid_names_and_line_breaks() {
    let endpoint = "http://127.0.0.1:1";
    for (name, value) in [
        ("", "value"),
        ("bad header", "value"),
        ("x-name:injected", "value"),
        ("x-valid", "ok\r\nx-injected: 1"),
        ("x-valid", "ok\nline"),
    ] {
        let request = HttpRequest::new(Method::Get, "/route").with_header(name, value);
        assert_eq!(
            futures_executor::block_on(HttpTransport::new().execute(
                scope(),
                endpoint,
                request.clone(),
                &services(),
                cancelled(),
            ))
            .expect_err("invalid header must fail execute"),
            TransportError::Transport,
            "execute rejected ({name:?}, {value:?})"
        );
        assert_eq!(
            HttpTransport::new()
                .execute_blocking(endpoint, request.clone(), &cancelled())
                .expect_err("invalid header must fail execute_blocking"),
            TransportError::Transport,
            "execute_blocking rejected ({name:?}, {value:?})"
        );
        assert_eq!(
            HttpTransport::new()
                .subscribe(
                    scope(),
                    endpoint,
                    request,
                    SseDecoder::new(),
                    &services(),
                    cancelled(),
                )
                .err()
                .expect("invalid header must fail subscribe"),
            TransportError::Transport,
            "subscribe rejected ({name:?}, {value:?})"
        );
    }
}

#[test]
fn streaming_response_clears_interim_headers_and_emits_headers_for_empty_body() {
    let endpoint = serve(|mut stream| {
        read_request(&mut stream);
        write_all(
            &mut stream,
            b"HTTP/1.1 100 Continue\r\nX-Interim: drop-me\r\n\r\nHTTP/1.1 200 OK\r\nX-Request-Id: final-id\r\nContent-Length: 0\r\n\r\n",
        );
    });
    let mut subscription = HttpTransport::new()
        .subscribe(
            scope(),
            &endpoint,
            HttpRequest::new(Method::Get, "/empty")
                .capture_headers(|name| name == "x-request-id" || name == "x-interim")
                .stream_response_headers(),
            SseDecoder::new(),
            &services(),
            cancelled(),
        )
        .expect("fixture subscription starts");
    let items = futures_executor::block_on(async {
        let mut items = Vec::new();
        loop {
            let next =
                futures_util::future::poll_fn(|context| match subscription.poll_next(context) {
                    Poll::Ready(item) => Poll::Ready(item),
                    Poll::Pending => Poll::Pending,
                })
                .await;
            match next {
                Some(Ok(item)) => items.push(item),
                Some(Err(error)) => panic!("stream failed: {error:?}"),
                None => break,
            }
        }
        items
    });
    assert_eq!(items.len(), 1, "empty-body 2xx must still emit headers");
    match &items[0] {
        StreamItem::Headers(headers) => {
            assert_eq!(
                headers.get("x-request-id").map(String::as_str),
                Some("final-id")
            );
            assert!(
                !headers.contains_key("x-interim"),
                "headers from 100 Continue must be cleared on the final status line"
            );
        }
        other => panic!("expected StreamItem::Headers, got {other:?}"),
    }
}

#[test]
fn sse_decoder_strips_at_most_one_leading_space_in_data() {
    let mut decoder = SseDecoder::new();
    let frames = decoder
        .push(b"data:  two spaces\n\ndata:\ttab\n\ndata:nospace\n\n")
        .expect("frames decode");
    assert_eq!(frames.len(), 3);
    assert_eq!(frames[0].data(), b" two spaces");
    assert_eq!(frames[1].data(), b"\ttab");
    assert_eq!(frames[2].data(), b"nospace");
    decoder.finish().expect("drained stream finishes");
}

#[test]
fn sse_framer_rejects_oversized_boundary_free_chunk_without_growing_pending() {
    let mut framer = SseFramer::new().with_event_limit(16);
    let oversized = vec![b'x'; 1024];
    let error = framer
        .push(&oversized)
        .expect_err("oversized boundary-free chunk must fail");
    assert_eq!(error.kind(), SseDecodeErrorKind::LimitExceeded);
    // Since pending was not grown, finish() sees an empty buffer (no incomplete frame).
    framer
        .finish()
        .expect("pending was not grown by the rejected chunk");
}

#[test]
fn secret_header_line_retains_bytes_until_zeroize_and_transmits_header() {
    use zeroize::Zeroize;

    let secret = SecretHeader::bearer(b"top-secret-token".to_vec()).expect("valid secret");
    let mut line = crate::client::format_secret_header_line(&secret);
    assert_eq!(
        line.as_str(),
        "authorization: Bearer top-secret-token",
        "formatted secret line must remain intact until Zeroizing::drop / zeroize"
    );
    line.zeroize();
    assert!(
        line.is_empty(),
        "zeroize() must wipe the still-owned secret header line"
    );

    let endpoint = serve(|mut stream| {
        let mut request = Vec::new();
        let mut chunk = [0u8; 1024];
        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            match stream.read(&mut chunk) {
                Ok(0) | Err(_) => break,
                Ok(read) => request.extend_from_slice(&chunk[..read]),
            }
        }
        let head = String::from_utf8_lossy(&request).to_ascii_lowercase();
        let body = if head.contains("authorization: bearer top-secret-token") {
            b"ok".as_slice()
        } else {
            b"missing".as_slice()
        };
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
        write_all(&mut stream, response.as_bytes());
        write_all(&mut stream, body);
    });
    let response = futures_executor::block_on(HttpTransport::new().execute(
        scope(),
        &endpoint,
        HttpRequest::new(Method::Get, "/auth").with_secret_header(secret),
        &services(),
        cancelled(),
    ))
    .expect("secret header exchange succeeds");
    assert_eq!(response.body, b"ok");
}
