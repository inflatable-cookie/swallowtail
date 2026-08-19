use super::failure::{
    backpressure, credential_failure, disconnected, endpoint_failure, protocol_failure,
    resync_failure,
};
use super::{Command, FRAME_LIMIT, Update};
use crate::local_server::protocol::{WsFrame, decode_ws_frame, encode_pong};
use futures_channel::{mpsc, oneshot};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc as sync_mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use swallowtail_runtime::RuntimeFailure;
use tungstenite::client::{IntoClientRequest, connect_with_config};
use tungstenite::http::{HeaderValue, header::AUTHORIZATION};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Error, Message, WebSocket};

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

#[allow(clippy::too_many_arguments)]
pub(super) fn run(
    endpoint: String,
    mut secret: Vec<u8>,
    session_id: String,
    cursor_seq: u64,
    cursor_epoch: Option<String>,
    mut updates: mpsc::Sender<Result<Update, RuntimeFailure>>,
    commands: sync_mpsc::Receiver<Command>,
    cancelled: Arc<AtomicBool>,
    control: Arc<Mutex<Option<TcpStream>>>,
) -> Result<(), RuntimeFailure> {
    let result = run_connected(
        &endpoint,
        &secret,
        &session_id,
        cursor_seq,
        cursor_epoch,
        &mut updates,
        &commands,
        &cancelled,
        &control,
    );
    secret.fill(0);
    result
}

#[allow(clippy::too_many_arguments)]
fn run_connected(
    endpoint: &str,
    secret: &[u8],
    session_id: &str,
    cursor_seq: u64,
    cursor_epoch: Option<String>,
    updates: &mut mpsc::Sender<Result<Update, RuntimeFailure>>,
    commands: &sync_mpsc::Receiver<Command>,
    cancelled: &AtomicBool,
    control: &Mutex<Option<TcpStream>>,
) -> Result<(), RuntimeFailure> {
    let mut socket = connect(endpoint, secret)?;
    let tcp = clone_tcp(socket.get_ref())?;
    tcp.set_read_timeout(Some(Duration::from_millis(100)))
        .map_err(|_| disconnected())?;
    *control.lock().expect("control lock poisoned") = Some(tcp);
    require_hello(&mut socket)?;
    let (current_seq, current_epoch) =
        subscribe(&mut socket, session_id, cursor_seq, cursor_epoch.as_deref())?;
    updates
        .try_send(Ok(Update::Ready {
            current_seq,
            current_epoch,
        }))
        .map_err(|_| backpressure())?;
    let mut pending_abort = None;
    loop {
        match commands.try_recv() {
            Ok(Command::Abort { frame, id, result }) => {
                socket
                    .send(Message::Text(frame.into()))
                    .map_err(|_| disconnected())?;
                pending_abort = Some((id, result));
            }
            Ok(Command::Close) | Err(sync_mpsc::TryRecvError::Disconnected) => {
                let _ = socket.close(None);
                return Ok(());
            }
            Err(sync_mpsc::TryRecvError::Empty) => {}
        }
        if cancelled.load(Ordering::SeqCst) && pending_abort.is_none() {
            return Ok(());
        }
        match socket.read() {
            Ok(Message::Text(frame)) => {
                if complete_abort_ack(&frame, &mut pending_abort)? {
                    continue;
                }
                if let Ok(WsFrame::Ping { nonce }) = decode_ws_frame(frame.as_bytes()) {
                    socket
                        .send(Message::Text(encode_pong(&nonce).into()))
                        .map_err(|_| disconnected())?;
                    continue;
                }
                if updates
                    .try_send(Ok(Update::Event(frame.as_bytes().to_vec())))
                    .is_err()
                {
                    return Err(backpressure());
                }
            }
            Ok(Message::Ping(bytes)) => {
                socket
                    .send(Message::Pong(bytes))
                    .map_err(|_| disconnected())?;
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => return Err(disconnected()),
            Ok(Message::Binary(_) | Message::Frame(_)) => return Err(protocol_failure()),
            Err(Error::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(Error::ConnectionClosed | Error::AlreadyClosed) => return Err(disconnected()),
            Err(_) => return Err(disconnected()),
        }
    }
}

fn connect(endpoint: &str, secret: &[u8]) -> Result<Socket, RuntimeFailure> {
    let mut url = crate::local_server::transport::require_loopback_endpoint(endpoint)?;
    url.set_scheme("ws").map_err(|_| endpoint_failure())?;
    url.set_path("/api/v1/ws");
    let mut request = url
        .as_str()
        .into_client_request()
        .map_err(|_| endpoint_failure())?;
    let mut bearer = b"Bearer ".to_vec();
    bearer.extend_from_slice(secret);
    request.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_bytes(&bearer).map_err(|_| credential_failure())?,
    );
    bearer.fill(0);
    let config = WebSocketConfig::default()
        .read_buffer_size(FRAME_LIMIT)
        .max_message_size(Some(FRAME_LIMIT))
        .max_frame_size(Some(FRAME_LIMIT));
    connect_with_config(request, Some(config), 0)
        .map(|(socket, _)| socket)
        .map_err(|_| disconnected())
}

fn require_hello(socket: &mut Socket) -> Result<(), RuntimeFailure> {
    let Message::Text(frame) = socket.read().map_err(|_| disconnected())? else {
        return Err(protocol_failure());
    };
    match decode_ws_frame(frame.as_bytes())? {
        WsFrame::ServerHello {
            protocol_version: 2,
        } => Ok(()),
        _ => Err(protocol_failure()),
    }
}

fn subscribe(
    socket: &mut Socket,
    session_id: &str,
    seq: u64,
    epoch: Option<&str>,
) -> Result<(u64, Option<String>), RuntimeFailure> {
    let cursor = match epoch {
        Some(epoch) => serde_json::json!({"seq":seq,"epoch":epoch}),
        None => serde_json::json!({"seq":seq}),
    };
    let frame = serde_json::json!({
        "type":"subscribe",
        "id":"swallowtail-subscribe",
        "payload":{"session_ids":[session_id],"cursors":{(session_id):cursor}}
    });
    socket
        .send(Message::Text(frame.to_string().into()))
        .map_err(|_| disconnected())?;
    let Message::Text(frame) = socket.read().map_err(|_| disconnected())? else {
        return Err(protocol_failure());
    };
    match decode_ws_frame(frame.as_bytes())? {
        WsFrame::Ack {
            code: 0,
            accepted_count: 1,
            resync_count: 0,
            cursors,
        } => {
            let [cursor] = cursors.as_slice() else {
                return Err(resync_failure());
            };
            if cursor.session_id != session_id
                || cursor.seq < seq
                || cursor_epoch_mismatch(epoch, cursor.epoch.as_deref())
            {
                return Err(resync_failure());
            }
            Ok((cursor.seq, cursor.epoch.clone()))
        }
        _ => Err(resync_failure()),
    }
}

fn cursor_epoch_mismatch(requested: Option<&str>, accepted: Option<&str>) -> bool {
    requested.is_some_and(|requested| accepted != Some(requested))
}

fn complete_abort_ack(
    frame: &str,
    pending: &mut Option<(String, oneshot::Sender<Result<(), RuntimeFailure>>)>,
) -> Result<bool, RuntimeFailure> {
    let Some((expected, _)) = pending.as_ref() else {
        return Ok(false);
    };
    let value: serde_json::Value = serde_json::from_str(frame).map_err(|_| protocol_failure())?;
    if value.get("type").and_then(serde_json::Value::as_str) != Some("ack")
        || value.get("id").and_then(serde_json::Value::as_str) != Some(expected)
    {
        return Ok(false);
    }
    let (_, sender) = pending.take().expect("pending abort remains");
    let result = match decode_ws_frame(frame.as_bytes())? {
        WsFrame::Ack { code: 0, .. } => Ok(()),
        _ => Err(protocol_failure()),
    };
    let _ = sender.send(result);
    Ok(true)
}

fn clone_tcp(stream: &MaybeTlsStream<TcpStream>) -> Result<TcpStream, RuntimeFailure> {
    match stream {
        MaybeTlsStream::Plain(stream) => stream.try_clone().map_err(|_| disconnected()),
        _ => Err(endpoint_failure()),
    }
}
