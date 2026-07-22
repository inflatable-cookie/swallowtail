use crate::failure::failure;
use crate::realtime::{MODEL_ID, REALTIME_PATH};
use std::net::TcpStream;
use std::time::Duration;
use swallowtail_core::ProviderRequestRef;
use swallowtail_runtime::RuntimeFailure;
use tungstenite::WebSocket;
use tungstenite::client::{IntoClientRequest, connect_with_config};
use tungstenite::http::{HeaderValue, header::AUTHORIZATION};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;

const MAX_FRAME_BYTES: usize = 1_048_576;
pub(super) type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

pub(super) fn open_socket(
    endpoint: &str,
    secret: &[u8],
) -> Result<(Socket, ProviderRequestRef, TcpStream), RuntimeFailure> {
    let url = realtime_url(endpoint)?;
    let mut request = url
        .as_str()
        .into_client_request()
        .map_err(|_| endpoint_rejected())?;
    let mut bearer = SecretCopy(b"Bearer ".to_vec());
    bearer.0.extend_from_slice(secret);
    request.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_bytes(&bearer.0).map_err(|_| credential_rejected())?,
    );
    let config = WebSocketConfig::default()
        .read_buffer_size(MAX_FRAME_BYTES)
        .write_buffer_size(0)
        .max_write_buffer_size(MAX_FRAME_BYTES * 2)
        .max_message_size(Some(MAX_FRAME_BYTES))
        .max_frame_size(Some(MAX_FRAME_BYTES));
    let (socket, response) =
        connect_with_config(request, Some(config), 0).map_err(|_| connection_failed())?;
    set_read_timeout(socket.get_ref())?;
    let control = clone_tcp(socket.get_ref())?;
    let request_ref = response
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| ProviderRequestRef::new(value.to_owned()).ok())
        .ok_or_else(|| {
            failure(
                "swallowtail.openai.realtime_request_id_missing",
                "OpenAI Realtime handshake omitted safe request correlation",
            )
        })?;
    Ok((socket, request_ref, control))
}

fn realtime_url(endpoint: &str) -> Result<url::Url, RuntimeFailure> {
    let mut url = url::Url::parse(endpoint).map_err(|_| endpoint_rejected())?;
    if !matches!(url.scheme(), "ws" | "wss")
        || url.path() != REALTIME_PATH
        || url.query().is_some()
        || url.fragment().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(endpoint_rejected());
    }
    url.query_pairs_mut().append_pair("model", MODEL_ID);
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

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

fn endpoint_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_endpoint_rejected",
        "OpenAI Realtime endpoint did not match the approved public route",
    )
}

fn credential_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_credential_invalid",
        "OpenAI Realtime credential could not form a bearer header",
    )
}

fn connection_failed() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_connect_failed",
        "OpenAI Realtime WebSocket connection could not be established",
    )
}
