use crate::config::TransportConfig;
use crate::worker::{WorkerCommand, WorkerEvent};
use futures_channel::{mpsc, oneshot};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::future::Future;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{REMOTE_ACP_WIRE_VERSION, RemoteAcpConnectionBounds, RemoteAcpTransport};
use swallowtail_protocol_acp::Message;
use url::Url;

mod host;
mod http;
mod lifecycle;
mod portability;
mod sdk_server;
mod websocket;

const HTTP_CORPUS: &str = include_str!(
    "../../swallowtail-protocol-acp/tests/fixtures/acp-v1-remote-transport-2.0.0/http-sse-success.jsonl"
);
const HTTP_DISCONNECT_CORPUS: &str = include_str!(
    "../../swallowtail-protocol-acp/tests/fixtures/acp-v1-remote-transport-2.0.0/http-disconnect.sse"
);
const WEBSOCKET_CORPUS: &str = include_str!(
    "../../swallowtail-protocol-acp/tests/fixtures/acp-v1-remote-transport-2.0.0/websocket-success.jsonl"
);
const WEBSOCKET_DISCONNECT_CORPUS: &str = include_str!(
    "../../swallowtail-protocol-acp/tests/fixtures/acp-v1-remote-transport-2.0.0/websocket-disconnect.jsonl"
);

#[test]
fn remote_transport_lifecycles_run_sequentially() {
    run_scenario(host::host_blocking_runtime_is_owned_and_joined());
    run_scenario(http::http2_incomplete_sse_disconnect_invalidates_without_recovery());
    run_scenario(http::http2_sse_runs_raw_corpus_affinity_callback_cancel_and_close());
    run_scenario(portability::public_profile_runs_both_transports_and_topologies());
    run_scenario(websocket::websocket_cancel_and_deadline_close_owned_connection());
    run_scenario(websocket::websocket_disconnect_invalidates_without_recovery());
    run_scenario(websocket::websocket_runs_raw_corpus_callback_cancel_and_close());
    run_scenario(http::http_non_responding_peer_fails_within_deadline());
    run_scenario(websocket::websocket_hanging_connect_fails_within_deadline());
    run_scenario(websocket::websocket_silent_peer_fails_within_deadline());
}

fn run_scenario(scenario: impl Future<Output = ()>) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(scenario);
}

fn config(endpoint: Url, transport: RemoteAcpTransport) -> TransportConfig {
    TransportConfig {
        endpoint,
        transport,
        bounds: RemoteAcpConnectionBounds::new(
            NonZeroU64::new(64 * 1024).unwrap(),
            NonZeroU32::new(16).unwrap(),
            NonZeroU32::new(8).unwrap(),
            NonZeroU32::new(128).unwrap(),
            NonZeroU32::new(128).unwrap(),
        ),
        maximum_cookie_count: NonZeroU32::new(16).unwrap(),
        maximum_cookie_bytes: NonZeroU64::new(16 * 1024).unwrap(),
    }
}

fn corpus_messages(corpus: &str, kind: &str) -> Vec<Message> {
    corpus
        .lines()
        .filter_map(|line| {
            let record: Value = serde_json::from_str(line).expect("fixture record is JSON");
            (record.get("kind").and_then(Value::as_str) == Some(kind))
                .then(|| record.get("body").cloned())
                .flatten()
        })
        .map(|body| {
            let bytes = serde_json::to_vec(&body).expect("fixture body encodes");
            swallowtail_protocol_acp::decode_message(&bytes).expect("fixture body is ACP")
        })
        .collect()
}

fn initialize() -> Message {
    Message::Request {
        id: json!(1),
        method: "initialize".to_owned(),
        params: json!({"protocolVersion": REMOTE_ACP_WIRE_VERSION}),
    }
}

fn assert_initialize(message: &Message) {
    assert!(matches!(
        message,
        Message::Response {
            id,
            result: Ok(result),
        } if id == &json!(1)
            && result.get("protocolVersion").and_then(Value::as_u64)
                == Some(u64::from(REMOTE_ACP_WIRE_VERSION))
    ));
}

async fn channels() -> (
    mpsc::Sender<WorkerCommand>,
    mpsc::Receiver<WorkerCommand>,
    mpsc::Sender<WorkerEvent>,
    mpsc::Receiver<WorkerEvent>,
    oneshot::Sender<Result<(), crate::RemoteAcpError>>,
    oneshot::Receiver<Result<(), crate::RemoteAcpError>>,
) {
    let (commands, command_rx) = mpsc::channel(16);
    let (event_tx, events) = mpsc::channel(32);
    let (ready_tx, ready_rx) = oneshot::channel();
    (commands, command_rx, event_tx, events, ready_tx, ready_rx)
}

async fn send(commands: &mut mpsc::Sender<WorkerCommand>, message: Message) {
    commands
        .send(WorkerCommand::Send(message))
        .await
        .expect("transport command remains open");
}

async fn message(events: &mut mpsc::Receiver<WorkerEvent>, stage: &'static str) -> Message {
    let event = tokio::time::timeout(std::time::Duration::from_secs(10), events.next())
        .await
        .expect(stage)
        .expect("transport event exists");
    match event {
        WorkerEvent::Message(message) => message,
        WorkerEvent::Failed(error) => panic!("transport failed: {error}"),
    }
}
