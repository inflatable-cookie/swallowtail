use async_tungstenite::tungstenite::Message as WebSocketMessage;
use async_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};
use futures_util::StreamExt;
use serde_json::json;
use std::sync::{Arc, Mutex};
use swallowtail_protocol_acp::{Message, decode_message, encode_message};
use tokio::net::TcpListener;

#[derive(Default)]
struct State {
    connections: usize,
    methods: Vec<String>,
}

#[derive(Clone)]
pub struct Evidence(Arc<Mutex<State>>);

impl Evidence {
    pub fn connections(&self) -> usize {
        self.0.lock().expect("evidence lock poisoned").connections
    }

    pub fn methods(&self) -> Vec<String> {
        self.0
            .lock()
            .expect("evidence lock poisoned")
            .methods
            .clone()
    }
}

struct Upgrade;

impl Callback for Upgrade {
    fn on_request(
        self,
        _request: &Request,
        mut response: Response,
    ) -> Result<Response, ErrorResponse> {
        response
            .headers_mut()
            .insert("acp-connection-id", "opaque-connection".parse().unwrap());
        Ok(response)
    }
}

pub async fn spawn(disconnect_on_delete: bool) -> (String, tokio::task::JoinHandle<()>, Evidence) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let evidence = Evidence(Arc::new(Mutex::new(State::default())));
    let server_evidence = evidence.clone();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        server_evidence
            .0
            .lock()
            .expect("evidence lock poisoned")
            .connections += 1;
        let mut socket = async_tungstenite::tokio::accept_hdr_async(stream, Upgrade)
            .await
            .unwrap();
        while let Some(frame) = socket.next().await {
            match frame.unwrap() {
                WebSocketMessage::Text(text) => {
                    let message = decode_message(text.as_bytes()).unwrap();
                    let Message::Request { id, method, .. } = message else {
                        panic!("fixture accepts requests only");
                    };
                    server_evidence
                        .0
                        .lock()
                        .expect("evidence lock poisoned")
                        .methods
                        .push(method.clone());
                    if method == "session/delete" && disconnect_on_delete {
                        let _ = socket.close(None).await;
                        return;
                    }
                    let result = if method == "initialize" {
                        json!({
                            "protocolVersion": 1,
                            "agentInfo": {
                                "name": "@agentclientprotocol/claude-agent-acp",
                                "version": "0.61.0"
                            },
                            "agentCapabilities": {
                                "sessionCapabilities": {"close": {}, "delete": {}}
                            },
                            "authMethods": []
                        })
                    } else if method == "session/delete" {
                        json!({})
                    } else {
                        panic!("unexpected method");
                    };
                    let mut encoded = encode_message(&Message::Response {
                        id,
                        result: Ok(result),
                    })
                    .unwrap();
                    encoded.pop();
                    socket
                        .send(WebSocketMessage::Text(
                            String::from_utf8(encoded).unwrap().into(),
                        ))
                        .await
                        .unwrap();
                }
                WebSocketMessage::Close(_) => {
                    let _ = socket.close(None).await;
                    return;
                }
                _ => {}
            }
        }
    });
    (format!("ws://{address}/acp"), server, evidence)
}
