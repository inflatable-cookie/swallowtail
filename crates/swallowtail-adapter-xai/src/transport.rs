use crate::RESPONSES_WEBSOCKET_PATH;
use crate::failure::failure;
use crate::protocol::{MAX_FRAME_BYTES, Request, TurnState, TurnUpdate};
use futures_channel::mpsc;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_runtime::RuntimeFailure;
use tungstenite::client::{IntoClientRequest, connect_with_config};
use tungstenite::http::{HeaderValue, header::AUTHORIZATION};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

#[derive(Clone)]
pub(crate) struct Connection {
    socket: Arc<Mutex<Option<Socket>>>,
    closer: SocketCloser,
}

#[derive(Clone)]
pub(crate) struct SocketCloser {
    stream: Arc<Mutex<Option<TcpStream>>>,
}

impl SocketCloser {
    pub(crate) fn shutdown(&self) {
        if let Some(stream) = self
            .stream
            .lock()
            .expect("socket closer lock poisoned")
            .take()
        {
            let _ = stream.shutdown(Shutdown::Both);
        }
    }
}

impl Connection {
    pub(crate) fn open(endpoint: &str, secret: &[u8]) -> Result<Self, RuntimeFailure> {
        let mut request = endpoint
            .into_client_request()
            .map_err(|_| endpoint_failure())?;
        if request.uri().path() != RESPONSES_WEBSOCKET_PATH || request.uri().query().is_some() {
            return Err(endpoint_failure());
        }
        let mut bearer = SecretCopy(b"Bearer ".to_vec());
        bearer.0.extend_from_slice(secret);
        request.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(&bearer.0).map_err(|_| {
                failure(
                    "swallowtail.xai.credential_invalid",
                    "xAI API-key credential could not form a bearer header",
                )
            })?,
        );
        let config = WebSocketConfig::default()
            .read_buffer_size(MAX_FRAME_BYTES)
            .write_buffer_size(0)
            .max_write_buffer_size(MAX_FRAME_BYTES * 2)
            .max_message_size(Some(MAX_FRAME_BYTES))
            .max_frame_size(Some(MAX_FRAME_BYTES));
        let (socket, _) = connect_with_config(request, Some(config), 0).map_err(|_| {
            failure(
                "swallowtail.xai.websocket_connect_failed",
                "xAI WebSocket connection could not be established",
            )
        })?;
        let stream = clone_tcp(socket.get_ref())?;
        Ok(Self {
            socket: Arc::new(Mutex::new(Some(socket))),
            closer: SocketCloser {
                stream: Arc::new(Mutex::new(Some(stream))),
            },
        })
    }

    pub(crate) fn closer(&self) -> SocketCloser {
        self.closer.clone()
    }

    pub(crate) fn is_open(&self) -> bool {
        self.socket.lock().expect("socket lock poisoned").is_some()
    }

    pub(crate) fn run_turn(
        &self,
        model: &str,
        input: &str,
        continuation: &Arc<Mutex<Option<String>>>,
        chain_valid: &Arc<AtomicBool>,
        mut updates: mpsc::Sender<TurnUpdate>,
    ) -> Result<(), RuntimeFailure> {
        if !chain_valid.load(Ordering::SeqCst) {
            return Err(chain_failure());
        }
        let request = {
            let continuation = continuation.lock().expect("continuation lock poisoned");
            Request::turn(model, input, continuation.as_deref())?
        };
        let mut guard = self.socket.lock().expect("socket lock poisoned");
        let socket = guard.as_mut().ok_or_else(connection_closed)?;
        if let Err(error) = socket.send(Message::Text(request.into())) {
            invalidate(&mut guard, chain_valid, &self.closer);
            return Err(transport_failure(error));
        }

        let mut state = TurnState::default();
        loop {
            let message = match socket.read() {
                Ok(message) => message,
                Err(error) => {
                    invalidate(&mut guard, chain_valid, &self.closer);
                    return Err(transport_failure(error));
                }
            };
            let update = match message {
                Message::Text(frame) => state.apply(&frame),
                Message::Ping(_) | Message::Pong(_) => {
                    let _ = socket.flush();
                    continue;
                }
                Message::Close(_) => {
                    invalidate(&mut guard, chain_valid, &self.closer);
                    return Err(connection_closed());
                }
                Message::Binary(_) | Message::Frame(_) => Err(failure(
                    "swallowtail.xai.frame_type_rejected",
                    "xAI WebSocket returned an unsupported frame type",
                )),
            };
            let update = match update {
                Ok(update) => update,
                Err(error) => {
                    invalidate(&mut guard, chain_valid, &self.closer);
                    return Err(error);
                }
            };
            let terminal = matches!(
                &update,
                TurnUpdate::Complete { .. } | TurnUpdate::ProviderFailed(_)
            );
            let provider_failed = matches!(&update, TurnUpdate::ProviderFailed(_));
            if let TurnUpdate::Complete {
                continuation: ref next,
                ..
            } = update
            {
                *continuation.lock().expect("continuation lock poisoned") = Some(next.clone());
            }
            if updates.try_send(update).is_err() {
                invalidate(&mut guard, chain_valid, &self.closer);
                return Err(failure(
                    "swallowtail.xai.ingress_overflow",
                    "xAI WebSocket event ingress exceeded its bounded capacity",
                ));
            }
            if terminal {
                if provider_failed {
                    invalidate(&mut guard, chain_valid, &self.closer);
                }
                break;
            }
        }
        Ok(())
    }

    pub(crate) fn close(&self) -> Result<(), RuntimeFailure> {
        let socket = self.socket.lock().expect("socket lock poisoned").take();
        if let Some(mut socket) = socket {
            let _ = socket.close(None);
        }
        self.closer.shutdown();
        Ok(())
    }
}

fn invalidate(socket: &mut Option<Socket>, chain_valid: &AtomicBool, closer: &SocketCloser) {
    chain_valid.store(false, Ordering::SeqCst);
    closer.shutdown();
    let _ = socket.take();
}

fn clone_tcp(stream: &MaybeTlsStream<TcpStream>) -> Result<TcpStream, RuntimeFailure> {
    match stream {
        MaybeTlsStream::Plain(stream) => stream.try_clone(),
        MaybeTlsStream::Rustls(stream) => stream.sock.try_clone(),
        _ => return Err(endpoint_failure()),
    }
    .map_err(|_| {
        failure(
            "swallowtail.xai.socket_control_failed",
            "xAI WebSocket cancellation control could not be established",
        )
    })
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.endpoint_rejected",
        "xAI WebSocket endpoint did not match the approved Responses route",
    )
}

fn connection_closed() -> RuntimeFailure {
    failure(
        "swallowtail.xai.websocket_disconnected",
        "xAI WebSocket disconnected before turn completion",
    )
}

fn chain_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.continuation_invalid",
        "xAI WebSocket continuation is no longer valid",
    )
}

fn transport_failure(_error: tungstenite::Error) -> RuntimeFailure {
    connection_closed()
}
