use crate::config::TransportConfig;
use crate::cookies::BoundedCookieStore;
use crate::correlation::CorrelationState;
use crate::error::{RemoteAcpError, capacity_error, protocol_error, transport_error};
use crate::wire;
use crate::worker::{WorkerCommand, WorkerEvent, cancellation_error};
use async_tungstenite::tungstenite::Message as WebSocketMessage;
use futures_channel::{mpsc, oneshot};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use swallowtail_protocol_acp::Message;

pub(crate) async fn run(
    config: TransportConfig,
    mut commands: mpsc::Receiver<WorkerCommand>,
    mut events: mpsc::Sender<WorkerEvent>,
    ready: oneshot::Sender<Result<(), RemoteAcpError>>,
) -> Result<(), RemoteAcpError> {
    let maximum_frame_bytes =
        usize::try_from(config.bounds.maximum_frame_bytes().get()).map_err(|_| capacity_error())?;
    let mut correlation = CorrelationState::new(
        usize::try_from(config.bounds.maximum_pending_requests().get())
            .map_err(|_| capacity_error())?,
        usize::try_from(config.bounds.maximum_pending_callbacks().get())
            .map_err(|_| capacity_error())?,
    );
    let mut cookies =
        BoundedCookieStore::new(config.maximum_cookie_count, config.maximum_cookie_bytes)?;
    let cookie_endpoint = cookie_endpoint(&config.endpoint)?;
    let connection = async_tungstenite::tokio::connect_async(config.endpoint.as_str()).await;
    let (mut socket, response) = match connection {
        Ok(connection) => connection,
        Err(_) => {
            let error = transport_error();
            let _ = ready.send(Err(error.clone()));
            return Err(error);
        }
    };
    validate_connection_id(response.headers(), maximum_frame_bytes)?;
    cookies.store_response(response.headers(), &cookie_endpoint)?;
    let _ = ready.send(Ok(()));
    let mut initialized = false;

    loop {
        tokio::select! {
            command = commands.next() => {
                match command {
                    Some(WorkerCommand::Send(message)) => {
                        if !initialized
                            && !matches!(
                                &message,
                                Message::Request { method, .. } if method == "initialize"
                            )
                        {
                            return Err(protocol_error());
                        }
                        correlation.outbound(&message)?;
                        let encoded = wire::encode(&message, maximum_frame_bytes)?;
                        socket.send(WebSocketMessage::Text(encoded.into()))
                            .await
                            .map_err(|_| transport_error())?;
                    }
                    Some(WorkerCommand::Cancel) => {
                        let _ = socket.close(None).await;
                        return Err(cancellation_error(false));
                    }
                    Some(WorkerCommand::Deadline) => {
                        let _ = socket.close(None).await;
                        return Err(cancellation_error(true));
                    }
                    Some(WorkerCommand::Close) | None => {
                        socket.close(None).await.map_err(|_| transport_error())?;
                        return Ok(());
                    }
                }
            }
            incoming = socket.next() => {
                match incoming {
                    Some(Ok(WebSocketMessage::Text(text))) => {
                        let message = wire::decode(text.as_str(), maximum_frame_bytes)?;
                        let metadata = correlation.inbound(&message)?;
                        if metadata.completed_method.as_deref() == Some("initialize") {
                            validate_initialize_response(&message)?;
                            initialized = true;
                        }
                        events.send(WorkerEvent::Message(message))
                            .await
                            .map_err(|_| transport_error())?;
                    }
                    Some(Ok(WebSocketMessage::Ping(payload))) => {
                        socket.send(WebSocketMessage::Pong(payload))
                            .await
                            .map_err(|_| transport_error())?;
                    }
                    Some(Ok(WebSocketMessage::Pong(_))) => {}
                    Some(Ok(WebSocketMessage::Close(_))) | Some(Err(_)) | None => {
                        return Err(transport_error());
                    }
                    Some(Ok(WebSocketMessage::Binary(_) | WebSocketMessage::Frame(_))) => {
                        return Err(protocol_error());
                    }
                }
            }
        }
    }
}

fn validate_initialize_response(message: &Message) -> Result<(), RemoteAcpError> {
    match message {
        Message::Response {
            result: Ok(result), ..
        } if result.get("protocolVersion").and_then(Value::as_u64)
            == Some(u64::from(swallowtail_core::REMOTE_ACP_WIRE_VERSION)) =>
        {
            Ok(())
        }
        _ => Err(protocol_error()),
    }
}

fn validate_connection_id(
    headers: &reqwest::header::HeaderMap,
    maximum_bytes: usize,
) -> Result<(), RemoteAcpError> {
    let id = headers
        .get("acp-connection-id")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty() && value.len() <= maximum_bytes)
        .ok_or_else(protocol_error)?;
    let _ = id;
    Ok(())
}

fn cookie_endpoint(endpoint: &url::Url) -> Result<url::Url, RemoteAcpError> {
    let mut endpoint = endpoint.clone();
    let scheme = match endpoint.scheme() {
        "ws" => "http",
        "wss" => "https",
        _ => return Err(protocol_error()),
    };
    endpoint.set_scheme(scheme).map_err(|_| protocol_error())?;
    Ok(endpoint)
}
