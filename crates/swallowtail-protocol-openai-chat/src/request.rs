use crate::{CodecLimits, ProtocolError, ProtocolErrorKind};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChatMessage {
    role: String,
    content: Option<String>,
    extensions: BTreeMap<String, Value>,
}

impl ChatMessage {
    #[must_use]
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: Some(content.into()),
            extensions: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn without_content(role: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: None,
            extensions: BTreeMap::new(),
        }
    }

    pub fn insert_extension(
        &mut self,
        name: impl Into<String>,
        value: Value,
    ) -> Result<(), ProtocolError> {
        let name = name.into();
        if name.is_empty()
            || matches!(name.as_str(), "role" | "content")
            || self.extensions.contains_key(&name)
        {
            return Err(ProtocolError::new(ProtocolErrorKind::InvalidStructure));
        }
        self.extensions.insert(name, value);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    include_usage: bool,
    extensions: BTreeMap<String, Value>,
}

impl ChatRequest {
    #[must_use]
    pub fn new(
        model: impl Into<String>,
        messages: Vec<ChatMessage>,
        stream: bool,
        include_usage: bool,
    ) -> Self {
        Self {
            model: model.into(),
            messages,
            stream,
            include_usage,
            extensions: BTreeMap::new(),
        }
    }

    pub fn insert_extension(
        &mut self,
        name: impl Into<String>,
        value: Value,
    ) -> Result<(), ProtocolError> {
        let name = name.into();
        if name.is_empty() || known_field(&name) || self.extensions.contains_key(&name) {
            return Err(ProtocolError::new(ProtocolErrorKind::InvalidStructure));
        }
        self.extensions.insert(name, value);
        Ok(())
    }
}

pub fn encode_request(
    request: &ChatRequest,
    limits: CodecLimits,
) -> Result<Vec<u8>, ProtocolError> {
    check_string(&request.model, limits)?;
    if request.messages.is_empty() || request.messages.len() > limits.maximum_messages() {
        return Err(ProtocolError::new(ProtocolErrorKind::MessageLimitExceeded));
    }
    if request.extensions.len().saturating_add(4) > limits.maximum_fields() {
        return Err(ProtocolError::new(ProtocolErrorKind::FieldLimitExceeded));
    }
    let mut messages = Vec::with_capacity(request.messages.len());
    for message in &request.messages {
        check_string(&message.role, limits)?;
        if let Some(content) = &message.content {
            check_string(content, limits)?;
        }
        if message.extensions.len().saturating_add(2) > limits.maximum_fields() {
            return Err(ProtocolError::new(ProtocolErrorKind::FieldLimitExceeded));
        }
        let mut encoded = Map::new();
        encoded.insert("role".to_owned(), Value::String(message.role.clone()));
        encoded.insert(
            "content".to_owned(),
            message.content.clone().map_or(Value::Null, Value::String),
        );
        for (name, value) in &message.extensions {
            check_string(name, limits)?;
            encoded.insert(name.clone(), value.clone());
        }
        messages.push(Value::Object(encoded));
    }
    let mut object = Map::new();
    object.insert("model".to_owned(), Value::String(request.model.clone()));
    object.insert("messages".to_owned(), Value::Array(messages));
    object.insert("stream".to_owned(), Value::Bool(request.stream));
    if request.include_usage {
        object.insert(
            "stream_options".to_owned(),
            serde_json::json!({"include_usage": true}),
        );
    }
    for (name, value) in &request.extensions {
        check_string(name, limits)?;
        object.insert(name.clone(), value.clone());
    }
    let encoded = serde_json::to_vec(&Value::Object(object))
        .map_err(|_| ProtocolError::new(ProtocolErrorKind::SerializationFailed))?;
    if encoded.len() > limits.maximum_wire_bytes() {
        return Err(ProtocolError::new(ProtocolErrorKind::WireLimitExceeded));
    }
    Ok(encoded)
}

fn known_field(name: &str) -> bool {
    matches!(name, "model" | "messages" | "stream" | "stream_options")
}

fn check_string(value: &str, limits: CodecLimits) -> Result<(), ProtocolError> {
    if value.is_empty() {
        return Err(ProtocolError::new(ProtocolErrorKind::InvalidStructure));
    }
    if value.len() > limits.maximum_string_bytes() {
        return Err(ProtocolError::new(ProtocolErrorKind::StringLimitExceeded));
    }
    Ok(())
}
