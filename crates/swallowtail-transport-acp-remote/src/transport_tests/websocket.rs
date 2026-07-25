use super::{
    WEBSOCKET_CORPUS, WEBSOCKET_DISCONNECT_CORPUS, assert_initialize, channels, config,
    corpus_messages, initialize, message, send,
};
use crate::RemoteAcpErrorKind;
use crate::worker::WorkerCommand;
use async_tungstenite::tungstenite::Message as WebSocketMessage;
use async_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use swallowtail_core::RemoteAcpTransport;
use swallowtail_protocol_acp::Message;
use tokio::net::TcpListener;
use url::Url;

#[derive(Default)]
struct ServerEvidence {
    upgrade_cookie_set: bool,
    callback_received: bool,
    cancel_received: bool,
    close_received: bool,
}

struct UpgradeCallback(Arc<Mutex<ServerEvidence>>);

impl Callback for UpgradeCallback {
    fn on_request(
        self,
        _request: &Request,
        mut response: Response,
    ) -> Result<Response, ErrorResponse> {
        response
            .headers_mut()
            .insert("acp-connection-id", "connection-private".parse().unwrap());
        response.headers_mut().insert(
            "set-cookie",
            "affinity=opaque-cookie; HttpOnly; Path=/".parse().unwrap(),
        );
        self.0.lock().unwrap().upgrade_cookie_set = true;
        Ok(response)
    }
}

pub(super) async fn websocket_runs_raw_corpus_callback_cancel_and_close() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let evidence = Arc::new(Mutex::new(ServerEvidence::default()));
    let server_evidence = Arc::clone(&evidence);
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = async_tungstenite::tokio::accept_hdr_async(
            stream,
            UpgradeCallback(Arc::clone(&server_evidence)),
        )
        .await
        .unwrap();
        let server_messages = corpus_messages(WEBSOCKET_CORPUS, "server_text");
        let mut replies = server_messages.into_iter();
        while let Some(frame) = socket.next().await {
            match frame.unwrap() {
                WebSocketMessage::Text(text) => {
                    let client = crate::wire::decode(text.as_str(), 64 * 1024).unwrap();
                    match &client {
                        Message::Response { id, .. } if id == &serde_json::json!(99) => {
                            server_evidence.lock().unwrap().callback_received = true;
                            continue;
                        }
                        Message::Notification { method, .. } if method == "session/cancel" => {
                            server_evidence.lock().unwrap().cancel_received = true;
                            continue;
                        }
                        _ => {}
                    }
                    let response = replies.next().expect("raw corpus has a response");
                    socket
                        .send(WebSocketMessage::Text(
                            crate::wire::encode(&response, 64 * 1024).unwrap().into(),
                        ))
                        .await
                        .unwrap();
                    if matches!(&client, Message::Request { method, .. } if method == "session/prompt")
                    {
                        for response in replies.by_ref().take(2) {
                            socket
                                .send(WebSocketMessage::Text(
                                    crate::wire::encode(&response, 64 * 1024).unwrap().into(),
                                ))
                                .await
                                .unwrap();
                        }
                    }
                }
                WebSocketMessage::Close(_) => {
                    server_evidence.lock().unwrap().close_received = true;
                    let _ = socket.close(None).await;
                    return;
                }
                _ => {}
            }
        }
    });

    let endpoint = Url::parse(&format!("ws://{address}/acp")).unwrap();
    let (mut commands, command_rx, event_tx, mut events, ready_tx, ready_rx) = channels().await;
    let worker = tokio::spawn(crate::websocket::run(
        config(endpoint, RemoteAcpTransport::WebSocket),
        command_rx,
        event_tx,
        ready_tx,
    ));
    ready_rx.await.unwrap().unwrap();

    let client_messages = corpus_messages(WEBSOCKET_CORPUS, "client_text");
    send(&mut commands, initialize()).await;
    assert_initialize(&message(&mut events, "WebSocket initialize response arrives").await);
    send(&mut commands, client_messages[1].clone()).await;
    let _session = message(&mut events, "WebSocket session response arrives").await;
    send(&mut commands, client_messages[2].clone()).await;
    let _update = message(&mut events, "WebSocket update arrives").await;
    let callback = message(&mut events, "WebSocket callback arrives").await;
    send(&mut commands, client_messages[3].clone()).await;
    assert!(matches!(callback, Message::Request { .. }));
    let _prompt_response = message(&mut events, "WebSocket prompt response arrives").await;
    send(&mut commands, client_messages[4].clone()).await;
    commands.send(WorkerCommand::Close).await.unwrap();

    worker.await.unwrap().unwrap();
    server.await.unwrap();
    let evidence = evidence.lock().unwrap();
    assert!(evidence.upgrade_cookie_set);
    assert!(evidence.callback_received);
    assert!(evidence.cancel_received);
    assert!(evidence.close_received);
}

pub(super) async fn websocket_disconnect_invalidates_without_recovery() {
    let (endpoint, server) = termination_server(true).await;
    let (mut commands, command_rx, event_tx, mut events, ready_tx, ready_rx) = channels().await;
    let worker = tokio::spawn(crate::websocket::run(
        config(endpoint, RemoteAcpTransport::WebSocket),
        command_rx,
        event_tx,
        ready_tx,
    ));
    ready_rx.await.unwrap().unwrap();
    send(&mut commands, initialize()).await;
    assert_initialize(&message(&mut events, "WebSocket disconnect initialize arrives").await);
    let error = worker
        .await
        .unwrap()
        .expect_err("disconnect fails transport");
    assert_eq!(error.kind(), RemoteAcpErrorKind::TransportFailed);
    server.await.unwrap();
}

pub(super) async fn websocket_cancel_and_deadline_close_owned_connection() {
    for (command, expected) in [
        (WorkerCommand::Cancel, RemoteAcpErrorKind::Cancelled),
        (
            WorkerCommand::Deadline,
            RemoteAcpErrorKind::DeadlineExceeded,
        ),
    ] {
        let (endpoint, server) = termination_server(false).await;
        let (mut commands, command_rx, event_tx, _events, ready_tx, ready_rx) = channels().await;
        let worker = tokio::spawn(crate::websocket::run(
            config(endpoint, RemoteAcpTransport::WebSocket),
            command_rx,
            event_tx,
            ready_tx,
        ));
        ready_rx.await.unwrap().unwrap();
        commands.send(command).await.unwrap();
        let error = worker
            .await
            .unwrap()
            .expect_err("termination fails active operation");
        assert_eq!(error.kind(), expected);
        assert!(server.await.unwrap(), "fixture peer observed close");
    }
}

pub(super) async fn termination_server(
    disconnect_after_initialize: bool,
) -> (Url, tokio::task::JoinHandle<bool>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let evidence = Arc::new(Mutex::new(ServerEvidence::default()));
        let mut socket =
            async_tungstenite::tokio::accept_hdr_async(stream, UpgradeCallback(evidence))
                .await
                .unwrap();
        if disconnect_after_initialize {
            let _initialize = socket.next().await.unwrap().unwrap();
            let response = corpus_messages(WEBSOCKET_DISCONNECT_CORPUS, "server_text")
                .into_iter()
                .next()
                .unwrap();
            socket
                .send(WebSocketMessage::Text(
                    crate::wire::encode(&response, 64 * 1024).unwrap().into(),
                ))
                .await
                .unwrap();
            return false;
        }
        while let Some(frame) = socket.next().await {
            if matches!(frame.unwrap(), WebSocketMessage::Close(_)) {
                let _ = socket.close(None).await;
                return true;
            }
        }
        false
    });
    (Url::parse(&format!("ws://{address}/acp")).unwrap(), server)
}
