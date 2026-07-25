use super::StreamItem;
use crate::error::{RemoteAcpError, capacity_error, protocol_error, transport_error};
use crate::sse::SseDecoder;
use futures_util::StreamExt;
use serde_json::Value;
use swallowtail_protocol_acp::Message;

pub(super) async fn read_stream(
    response: reqwest::Response,
    session: bool,
    maximum_frame_bytes: usize,
    sender: tokio::sync::mpsc::Sender<StreamItem>,
) {
    let mut bytes = response.bytes_stream();
    let mut decoder = SseDecoder::new(maximum_frame_bytes);
    while let Some(chunk) = bytes.next().await {
        let Ok(chunk) = chunk else {
            let _ = sender.send(StreamItem::Failed).await;
            return;
        };
        let Ok(events) = decoder.push(&chunk) else {
            let _ = sender.send(StreamItem::Failed).await;
            return;
        };
        for text in events {
            if sender
                .send(StreamItem::Message { session, text })
                .await
                .is_err()
            {
                return;
            }
        }
    }
    let _ = sender.send(StreamItem::Failed).await;
}

pub(super) async fn bounded_body(
    response: reqwest::Response,
    maximum_bytes: usize,
) -> Result<Vec<u8>, RemoteAcpError> {
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| transport_error())?;
        if body.len().saturating_add(chunk.len()) > maximum_bytes {
            return Err(capacity_error());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

pub(super) fn require_http2(response: &reqwest::Response) -> Result<(), RemoteAcpError> {
    if response.version() == reqwest::Version::HTTP_2 {
        Ok(())
    } else {
        Err(protocol_error())
    }
}

pub(super) fn require_http2_owned(
    response: reqwest::Response,
) -> Result<reqwest::Response, RemoteAcpError> {
    require_http2(&response)?;
    Ok(response)
}

pub(super) fn bounded_header(
    headers: &reqwest::header::HeaderMap,
    name: &'static str,
    maximum_bytes: usize,
) -> Result<String, RemoteAcpError> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty() && value.len() <= maximum_bytes)
        .map(str::to_owned)
        .ok_or_else(protocol_error)
}

pub(super) fn response_connection_id(message: &Message) -> Result<&str, RemoteAcpError> {
    match message {
        Message::Response {
            result: Ok(result), ..
        } => result
            .get("connectionId")
            .and_then(Value::as_str)
            .ok_or_else(protocol_error),
        _ => Err(protocol_error()),
    }
}

pub(super) fn validate_initialize_response(
    message: &Message,
    completed_method: Option<&str>,
) -> Result<(), RemoteAcpError> {
    if completed_method != Some("initialize") {
        return Err(protocol_error());
    }
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

pub(super) fn message_session(message: &Message) -> Option<String> {
    match message {
        Message::Request { params, .. } | Message::Notification { params, .. } => params
            .get("sessionId")
            .and_then(Value::as_str)
            .map(str::to_owned),
        Message::Response { .. } => None,
    }
}
