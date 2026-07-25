use super::ServerEvidence;
use crate::transport_tests::{HTTP_CORPUS, corpus_messages};
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::combinators::UnsyncBoxBody;
use http_body_util::{BodyExt, Empty, Full, channel::Channel};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use std::collections::VecDeque;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use swallowtail_protocol_acp::Message;
use tokio::net::TcpListener;

struct FixtureState {
    initialize: Message,
    sse: VecDeque<Message>,
    connection_stream: Option<http_body_util::channel::Sender<Bytes, Infallible>>,
    session_stream: Option<http_body_util::channel::Sender<Bytes, Infallible>>,
}

type FixtureBody = UnsyncBoxBody<Bytes, Infallible>;

pub(super) fn spawn(
    listener: TcpListener,
    evidence: Arc<Mutex<ServerEvidence>>,
) -> tokio::task::JoinHandle<()> {
    let state = Arc::new(tokio::sync::Mutex::new(FixtureState {
        initialize: corpus_messages(HTTP_CORPUS, "server_response")
            .into_iter()
            .next()
            .expect("raw corpus has initialize response"),
        sse: corpus_messages(HTTP_CORPUS, "sse_event").into(),
        connection_stream: None,
        session_stream: None,
    }));
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let service = service_fn(move |request| {
            serve_request(request, Arc::clone(&state), Arc::clone(&evidence))
        });
        let _ = hyper::server::conn::http2::Builder::new(TokioExecutor::new())
            .serve_connection(TokioIo::new(stream), service)
            .await;
    })
}

async fn serve_request(
    request: Request<Incoming>,
    state: Arc<tokio::sync::Mutex<FixtureState>>,
    evidence: Arc<Mutex<ServerEvidence>>,
) -> Result<Response<FixtureBody>, Infallible> {
    let method = request.method().clone();
    let headers = request.headers().clone();
    let session = headers.get("acp-session-id").is_some();
    if headers
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("affinity=opaque-cookie"))
    {
        evidence.lock().unwrap().cookie_seen = true;
    }
    if method == http::Method::GET {
        return Ok(open_stream(state, session).await);
    }
    if method == http::Method::DELETE {
        evidence.lock().unwrap().delete_seen = true;
        return Ok(empty_response());
    }
    let body = request.into_body().collect().await.unwrap().to_bytes();
    let message = swallowtail_protocol_acp::decode_message(&body).unwrap();
    let mut state = state.lock().await;
    let response = match &message {
        Message::Request { method, .. } if method == "initialize" => {
            initialize_response(&state.initialize)
        }
        Message::Request { method, .. } if method == "session/new" => {
            let event = state.sse.pop_front().expect("session response exists");
            send_sse(
                state
                    .connection_stream
                    .as_mut()
                    .expect("connection SSE is open"),
                &event,
            )
            .await;
            empty_response()
        }
        Message::Request { method, .. } if method == "session/prompt" => {
            let events = state.sse.drain(..3).collect::<Vec<_>>();
            let sender = state.session_stream.as_mut().expect("session SSE is open");
            for event in events {
                send_sse(sender, &event).await;
            }
            empty_response()
        }
        Message::Response { id, .. } if id == &serde_json::json!(99) => {
            evidence.lock().unwrap().callback_session_seen = headers
                .get("acp-session-id")
                .and_then(|value| value.to_str().ok())
                == Some("session-private");
            empty_response()
        }
        Message::Notification { method, .. } if method == "session/cancel" => {
            evidence.lock().unwrap().cancel_seen = true;
            empty_response()
        }
        _ => panic!("unexpected raw-corpus client message"),
    };
    Ok(response)
}

async fn open_stream(
    state: Arc<tokio::sync::Mutex<FixtureState>>,
    session: bool,
) -> Response<FixtureBody> {
    let (sender, body) = Channel::new(8);
    let mut state = state.lock().await;
    if session {
        state.session_stream = Some(sender);
    } else {
        state.connection_stream = Some(sender);
    }
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/event-stream")
        .body(body.boxed_unsync())
        .unwrap()
}

fn initialize_response(message: &Message) -> Response<FixtureBody> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .header("acp-connection-id", "connection-private")
        .header("set-cookie", "affinity=opaque-cookie; HttpOnly; Path=/")
        .body(
            Full::new(Bytes::from(
                crate::wire::encode(message, 64 * 1024).unwrap(),
            ))
            .boxed_unsync(),
        )
        .unwrap()
}

fn empty_response() -> Response<FixtureBody> {
    Response::builder()
        .status(StatusCode::ACCEPTED)
        .body(Empty::new().boxed_unsync())
        .unwrap()
}

async fn send_sse(
    sender: &mut http_body_util::channel::Sender<Bytes, Infallible>,
    message: &Message,
) {
    let encoded = crate::wire::encode(message, 64 * 1024).unwrap();
    sender
        .send_data(Bytes::from(format!("data: {encoded}\n\n")))
        .await
        .unwrap();
}
