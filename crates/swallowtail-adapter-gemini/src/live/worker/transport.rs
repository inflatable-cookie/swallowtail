use crate::failure::failure;
use crate::live::LIVE_PATH;
use std::net::TcpStream;
use std::time::Duration;
use swallowtail_runtime::RuntimeFailure;
use tungstenite::WebSocket;
use tungstenite::client::{IntoClientRequest, connect_with_config};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;
use zeroize::Zeroize;

const MAX_FRAME_BYTES: usize = 1_048_576;
pub(super) type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

pub(super) fn open_socket(
    endpoint: &str,
    secret: &[u8],
) -> Result<(Socket, TcpStream), RuntimeFailure> {
    let mut secret = SecretText::new(secret)?;
    let url = live_url(endpoint, &secret.0)?;
    secret.0.zeroize();
    let request = url
        .as_str()
        .into_client_request()
        .map_err(|_| endpoint_rejected())?;
    let config = WebSocketConfig::default()
        .read_buffer_size(MAX_FRAME_BYTES)
        .write_buffer_size(0)
        .max_write_buffer_size(MAX_FRAME_BYTES * 2)
        .max_message_size(Some(MAX_FRAME_BYTES))
        .max_frame_size(Some(MAX_FRAME_BYTES));
    let (socket, _) =
        connect_with_config(request, Some(config), 0).map_err(|_| connection_failed())?;
    set_read_timeout(socket.get_ref())?;
    let control = clone_tcp(socket.get_ref())?;
    Ok((socket, control))
}

fn live_url(endpoint: &str, secret: &str) -> Result<url::Url, RuntimeFailure> {
    let mut url = url::Url::parse(endpoint).map_err(|_| endpoint_rejected())?;
    if !matches!(url.scheme(), "ws" | "wss")
        || url.path() != LIVE_PATH
        || url.query().is_some()
        || url.fragment().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(endpoint_rejected());
    }
    url.query_pairs_mut().append_pair("key", secret);
    Ok(url)
}

fn set_read_timeout(stream: &MaybeTlsStream<TcpStream>) -> Result<(), RuntimeFailure> {
    let timeout = Some(Duration::from_millis(10));
    match stream {
        MaybeTlsStream::Plain(stream) => stream.set_read_timeout(timeout),
        MaybeTlsStream::Rustls(stream) => stream.sock.set_read_timeout(timeout),
        _ => return Err(endpoint_rejected()),
    }
    .map_err(|_| connection_failed())
}

fn clone_tcp(stream: &MaybeTlsStream<TcpStream>) -> Result<TcpStream, RuntimeFailure> {
    match stream {
        MaybeTlsStream::Plain(stream) => stream.try_clone(),
        MaybeTlsStream::Rustls(stream) => stream.sock.try_clone(),
        _ => return Err(endpoint_rejected()),
    }
    .map_err(|_| connection_failed())
}

struct SecretText(String);

impl SecretText {
    fn new(bytes: &[u8]) -> Result<Self, RuntimeFailure> {
        String::from_utf8(bytes.to_vec())
            .map(Self)
            .map_err(|_| credential_rejected())
    }
}

impl Drop for SecretText {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

fn endpoint_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_endpoint_rejected",
        "Gemini Live endpoint did not match the approved v1beta route",
    )
}

fn credential_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_credential_invalid",
        "Gemini Live credential could not form the authenticated request",
    )
}

fn connection_failed() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_connect_failed",
        "Gemini Live WebSocket connection could not be established",
    )
}
