use super::{
    HTTP_CORPUS, HTTP_DISCONNECT_CORPUS, assert_initialize, channels, config, corpus_messages,
    initialize, message, send,
};
use crate::RemoteAcpErrorKind;
use crate::worker::WorkerCommand;
use bytes::{Bytes, BytesMut};
use futures_util::SinkExt;
use h2::server::SendResponse;
use http::{Request, Response, StatusCode};
use std::sync::{Arc, Mutex};
use swallowtail_core::RemoteAcpTransport;
use swallowtail_protocol_acp::Message;
use tokio::net::TcpListener;
use url::Url;

mod server;

#[derive(Default)]
pub(super) struct ServerEvidence {
    pub(super) cookie_seen: bool,
    pub(super) callback_session_seen: bool,
    pub(super) cancel_seen: bool,
    pub(super) delete_seen: bool,
}

pub(super) async fn http2_sse_runs_raw_corpus_affinity_callback_cancel_and_close() {
    let (endpoint, server, evidence) = portability_server().await;
    let (mut commands, command_rx, event_tx, mut events, ready_tx, ready_rx) = channels().await;
    let worker = tokio::spawn(crate::http::run(
        config(endpoint, RemoteAcpTransport::StreamableHttpSse),
        command_rx,
        event_tx,
        ready_tx,
        None,
        None,
    ));
    ready_rx.await.unwrap().unwrap();

    let client = corpus_messages(HTTP_CORPUS, "client_request");
    send(&mut commands, initialize()).await;
    assert_initialize(&message(&mut events, "HTTP initialize response arrives").await);
    send(&mut commands, client[1].clone()).await;
    let _session = message(&mut events, "HTTP session response arrives").await;
    send(&mut commands, client[2].clone()).await;
    let _update = message(&mut events, "HTTP update arrives").await;
    let callback = message(&mut events, "HTTP callback arrives").await;
    assert!(matches!(callback, Message::Request { .. }));
    send(&mut commands, client[3].clone()).await;
    let _prompt_response = message(&mut events, "HTTP prompt response arrives").await;
    send(&mut commands, client[4].clone()).await;
    commands.send(WorkerCommand::Close).await.unwrap();

    let joined = tokio::time::timeout(std::time::Duration::from_secs(10), worker).await;
    if joined.is_err() {
        panic!(
            "HTTP transport worker joins after DELETE observed={}",
            evidence.lock().unwrap().delete_seen
        );
    }
    joined.unwrap().unwrap().unwrap();
    server.abort();
    let _ = server.await;
    let evidence = evidence.lock().unwrap();
    assert!(evidence.cookie_seen);
    assert!(evidence.callback_session_seen);
    assert!(evidence.cancel_seen);
    assert!(evidence.delete_seen);
}

pub(super) async fn portability_server()
-> (Url, tokio::task::JoinHandle<()>, Arc<Mutex<ServerEvidence>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let evidence = Arc::new(Mutex::new(ServerEvidence::default()));
    let server = server::spawn(listener, Arc::clone(&evidence));
    let endpoint = Url::parse(&format!("http://{address}/acp")).unwrap();
    (endpoint, server, evidence)
}

pub(super) async fn http2_incomplete_sse_disconnect_invalidates_without_recovery() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut connection = h2::server::handshake(stream).await.unwrap();
        let init = corpus_messages(HTTP_CORPUS, "server_response")
            .into_iter()
            .next()
            .unwrap();
        while let Some(request) = connection.accept().await {
            let (request, mut respond) = request.unwrap();
            if request.method() == http::Method::POST {
                let _body = read_body(request).await;
                respond_json(&mut respond, &init, true);
            } else {
                let mut stream = open_sse(&mut respond);
                stream
                    .send_data(Bytes::from_static(HTTP_DISCONNECT_CORPUS.as_bytes()), true)
                    .unwrap();
                connection.graceful_shutdown();
            }
        }
    });

    let endpoint = Url::parse(&format!("http://{address}/acp")).unwrap();
    let (mut commands, command_rx, event_tx, mut events, ready_tx, ready_rx) = channels().await;
    let worker = tokio::spawn(crate::http::run(
        config(endpoint, RemoteAcpTransport::StreamableHttpSse),
        command_rx,
        event_tx,
        ready_tx,
        None,
        None,
    ));
    ready_rx.await.unwrap().unwrap();
    send(&mut commands, initialize()).await;
    assert_initialize(&message(&mut events, "HTTP disconnect initialize arrives").await);
    let error = worker
        .await
        .unwrap()
        .expect_err("incomplete SSE fails transport");
    assert_eq!(error.kind(), RemoteAcpErrorKind::TransportFailed);
    server.await.unwrap();
}

pub(super) async fn http_non_responding_peer_fails_within_deadline() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        // Accept the connection, hold it open, and never answer the HTTP/2
        // preface.
        let (stream, _) = listener.accept().await.unwrap();
        let _open = stream;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    });
    let time = Arc::new(super::host::ElapsedTimeService::new());
    let deadline = time.deadline(std::time::Duration::from_millis(500));
    let endpoint = Url::parse(&format!("http://{address}/acp")).unwrap();
    let (mut commands, command_rx, event_tx, _events, ready_tx, _ready_rx) = channels().await;
    let worker = tokio::spawn(crate::http::run(
        config(endpoint, RemoteAcpTransport::StreamableHttpSse),
        command_rx,
        event_tx,
        ready_tx,
        Some(deadline),
        Some(time),
    ));
    send(&mut commands, initialize()).await;
    let error = tokio::time::timeout(std::time::Duration::from_secs(5), worker)
        .await
        .expect("non-responding HTTP peer fails within the deadline")
        .unwrap()
        .expect_err("non-responding HTTP peer fails transport");
    assert_eq!(error.kind(), RemoteAcpErrorKind::DeadlineExceeded);
    server.await.unwrap();
}

async fn read_body(request: Request<h2::RecvStream>) -> Vec<u8> {
    let mut body = request.into_body();
    let mut bytes = BytesMut::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk.unwrap();
        bytes.extend_from_slice(&chunk);
        body.flow_control()
            .release_capacity(chunk.len())
            .expect("fixture releases capacity");
    }
    bytes.to_vec()
}

fn open_sse(respond: &mut SendResponse<Bytes>) -> h2::SendStream<Bytes> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/event-stream")
        .body(())
        .unwrap();
    respond.send_response(response, false).unwrap()
}

fn respond_json(respond: &mut SendResponse<Bytes>, message: &Message, initialize: bool) {
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json");
    if initialize {
        response = response
            .header("acp-connection-id", "connection-private")
            .header("set-cookie", "affinity=opaque-cookie; HttpOnly; Path=/");
    }
    let response = response.body(()).unwrap();
    let mut stream = respond.send_response(response, false).unwrap();
    let body = crate::wire::encode(message, 64 * 1024).unwrap();
    stream.send_data(Bytes::from(body), true).unwrap();
}
