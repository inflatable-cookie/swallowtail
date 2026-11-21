use crate::config::{TransportConfig, validate_frame_bound};
use crate::cookies::BoundedCookieStore;
use crate::correlation::CorrelationState;
use crate::error::{
    RemoteAcpError, RemoteAcpErrorKind, capacity_error, protocol_error, transport_error,
};
use crate::wire;
use crate::worker::{
    WorkerCommand, WorkerEvent, cancellation_error, race_close_grace, race_deadline,
};
use async_tungstenite::tungstenite::Message as WebSocketMessage;
use async_tungstenite::tungstenite::protocol::WebSocketConfig;
use futures_channel::{mpsc, oneshot};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use swallowtail_protocol_acp::Message;
use swallowtail_runtime::{Deadline, TimeService};

pub(crate) async fn run(
    config: TransportConfig,
    mut commands: mpsc::Receiver<WorkerCommand>,
    mut events: mpsc::Sender<WorkerEvent>,
    ready: oneshot::Sender<Result<(), RemoteAcpError>>,
    deadline: Option<Deadline>,
    time: Option<Arc<dyn TimeService>>,
) -> Result<(), RemoteAcpError> {
    let maximum_frame_bytes = validate_frame_bound(config.bounds.maximum_frame_bytes().get())?;
    let mut correlation = CorrelationState::new(
        usize::try_from(config.bounds.maximum_pending_requests().get())
            .map_err(|_| capacity_error())?,
        usize::try_from(config.bounds.maximum_pending_callbacks().get())
            .map_err(|_| capacity_error())?,
    );
    let mut cookies =
        BoundedCookieStore::new(config.maximum_cookie_count, config.maximum_cookie_bytes)?;
    let cookie_endpoint = cookie_endpoint(&config.endpoint)?;
    let ws_config = WebSocketConfig::default()
        .max_message_size(Some(maximum_frame_bytes))
        .max_frame_size(Some(maximum_frame_bytes));
    // The initial connect is raced against the deadline so a hanging
    // handshake is interruptible.
    let connection = match race_deadline(
        deadline,
        time.as_deref(),
        async_tungstenite::tokio::connect_async_with_config(
            config.endpoint.as_str(),
            Some(ws_config),
        ),
    )
    .await
    {
        Ok(Ok(connection)) => Ok(connection),
        Ok(Err(_)) => Err(transport_error()),
        Err(error) => Err(error),
    };
    let (mut socket, response) = match connection {
        Ok(connection) => connection,
        Err(error) => {
            let _ = ready.send(Err(error.clone()));
            return Err(error);
        }
    };
    if let Err(error) = validate_connection_id(response.headers(), maximum_frame_bytes)
        .and_then(|()| cookies.store_response(response.headers(), &cookie_endpoint))
    {
        let _ = ready.send(Err(error.clone()));
        return Err(error);
    }
    let _ = ready.send(Ok(()));
    let mut initialized = false;

    let result = loop {
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
                            break Err(protocol_error());
                        }
                        if let Err(error) = correlation.outbound(&message) {
                            break Err(error);
                        }
                        let encoded = match wire::encode(&message, maximum_frame_bytes) {
                            Ok(encoded) => encoded,
                            Err(error) => break Err(error),
                        };
                        match race_deadline(
                            deadline,
                            time.as_deref(),
                            socket.send(WebSocketMessage::Text(encoded.into())),
                        )
                        .await
                        {
                            Ok(Ok(())) => {}
                            Ok(Err(_)) => break Err(transport_error()),
                            Err(error) => break Err(error),
                        }
                    }
                    Some(WorkerCommand::Cancel) => {
                        let _ = race_deadline(deadline, time.as_deref(), socket.close(None)).await;
                        break Err(cancellation_error(false));
                    }
                    Some(WorkerCommand::Deadline) => {
                        break Err(cancellation_error(true));
                    }
                    Some(WorkerCommand::Close) | None => {
                        match race_deadline(deadline, time.as_deref(), socket.close(None)).await {
                            Ok(Ok(())) => break Ok(()),
                            Ok(Err(_)) => break Err(transport_error()),
                            Err(error) => break Err(error),
                        }
                    }
                }
            }
            incoming = race_deadline(deadline, time.as_deref(), socket.next()) => {
                let incoming = match incoming {
                    Ok(incoming) => incoming,
                    Err(error) => break Err(error),
                };
                match incoming {
                    Some(Ok(WebSocketMessage::Text(text))) => {
                        let message = match wire::decode(text.as_str(), maximum_frame_bytes) {
                            Ok(message) => message,
                            Err(error) => break Err(error),
                        };
                        let metadata = match correlation.inbound(&message) {
                            Ok(metadata) => metadata,
                            Err(error) => break Err(error),
                        };
                        if metadata.completed_method.as_deref() == Some("initialize") {
                            if let Err(error) = validate_initialize_response(&message) {
                                break Err(error);
                            }
                            initialized = true;
                        }
                        if events.send(WorkerEvent::Message(message)).await.is_err() {
                            break Err(transport_error());
                        }
                    }
                    Some(Ok(WebSocketMessage::Ping(payload))) => {
                        match race_deadline(
                            deadline,
                            time.as_deref(),
                            socket.send(WebSocketMessage::Pong(payload)),
                        )
                        .await
                        {
                            Ok(Ok(())) => {}
                            Ok(Err(_)) => break Err(transport_error()),
                            Err(error) => break Err(error),
                        }
                    }
                    Some(Ok(WebSocketMessage::Pong(_))) => {}
                    Some(Ok(WebSocketMessage::Close(_))) | Some(Err(_)) | None => {
                        break Err(transport_error());
                    }
                    Some(Ok(WebSocketMessage::Binary(_) | WebSocketMessage::Frame(_))) => {
                        break Err(protocol_error());
                    }
                }
            }
        }
    };
    if matches!(&result, Err(error) if error.kind() == RemoteAcpErrorKind::DeadlineExceeded) {
        let _ = race_close_grace(time.as_deref(), socket.close(None)).await;
    }
    result
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
