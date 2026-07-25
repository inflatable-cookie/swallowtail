use crate::error::{RemoteAcpError, capacity_error, protocol_error};
use serde_json::Value;
use std::collections::BTreeMap;
use swallowtail_protocol_acp::Message;

pub(crate) struct CorrelationState {
    maximum_pending_requests: usize,
    maximum_pending_callbacks: usize,
    requests: BTreeMap<String, String>,
    callbacks: BTreeMap<String, Option<String>>,
}

pub(crate) struct InboundMetadata {
    pub(crate) opened_session: Option<String>,
    pub(crate) completed_method: Option<String>,
}

impl CorrelationState {
    pub(crate) fn new(maximum_pending_requests: usize, maximum_pending_callbacks: usize) -> Self {
        Self {
            maximum_pending_requests,
            maximum_pending_callbacks,
            requests: BTreeMap::new(),
            callbacks: BTreeMap::new(),
        }
    }

    pub(crate) fn outbound(&mut self, message: &Message) -> Result<(), RemoteAcpError> {
        match message {
            Message::Request { id, method, .. } => {
                if self.requests.len() == self.maximum_pending_requests {
                    return Err(capacity_error());
                }
                let key = key(id)?;
                if self.requests.insert(key, method.clone()).is_some() {
                    return Err(protocol_error());
                }
            }
            Message::Response { id, .. } => {
                if self.callbacks.remove(&key(id)?).is_none() {
                    return Err(protocol_error());
                }
            }
            Message::Notification { .. } => {}
        }
        Ok(())
    }

    pub(crate) fn inbound(&mut self, message: &Message) -> Result<InboundMetadata, RemoteAcpError> {
        let mut opened_session = None;
        let mut completed_method = None;
        match message {
            Message::Response { id, result } => {
                let method = self.requests.remove(&key(id)?).ok_or_else(protocol_error)?;
                if method == "session/new" {
                    opened_session = result
                        .as_ref()
                        .ok()
                        .and_then(|value| value.get("sessionId"))
                        .and_then(Value::as_str)
                        .map(str::to_owned);
                    if opened_session.is_none() {
                        return Err(protocol_error());
                    }
                }
                completed_method = Some(method);
            }
            Message::Request { id, params, .. } => {
                if self.callbacks.len() == self.maximum_pending_callbacks {
                    return Err(capacity_error());
                }
                let session = params
                    .get("sessionId")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                if self.callbacks.insert(key(id)?, session).is_some() {
                    return Err(protocol_error());
                }
            }
            Message::Notification { .. } => {}
        }
        Ok(InboundMetadata {
            opened_session,
            completed_method,
        })
    }

    pub(crate) fn callback_session(&self, id: &Value) -> Result<Option<&str>, RemoteAcpError> {
        self.callbacks
            .get(&key(id)?)
            .map(|value| value.as_deref())
            .ok_or_else(protocol_error)
    }
}

fn key(id: &Value) -> Result<String, RemoteAcpError> {
    match id {
        Value::String(_) | Value::Number(_) => {
            serde_json::to_string(id).map_err(|_| protocol_error())
        }
        _ => Err(protocol_error()),
    }
}

#[cfg(test)]
mod tests {
    use super::CorrelationState;
    use serde_json::json;
    use swallowtail_protocol_acp::Message;

    #[test]
    fn requests_and_callbacks_use_separate_bounded_maps() {
        let mut state = CorrelationState::new(1, 1);
        state
            .outbound(&Message::Request {
                id: json!(1),
                method: "session/new".to_owned(),
                params: json!({}),
            })
            .unwrap();
        assert!(
            state
                .outbound(&Message::Request {
                    id: json!(2),
                    method: "session/prompt".to_owned(),
                    params: json!({}),
                })
                .is_err()
        );
        let metadata = state
            .inbound(&Message::Response {
                id: json!(1),
                result: Ok(json!({"sessionId": "session-private"})),
            })
            .unwrap();
        assert_eq!(metadata.opened_session.as_deref(), Some("session-private"));
    }
}
