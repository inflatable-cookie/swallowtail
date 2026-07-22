use super::{ConversationRef, ItemRef};
use crate::failure::AlibabaProtocolFailure;
use crate::selection::EXACT_MODEL_ID;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::OperationContent;

mod options;

pub use options::TurnOptions;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WireRequest {
    method: Method,
    path: String,
    body: Option<Value>,
    session_cache: bool,
}

impl fmt::Debug for WireRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WireRequest")
            .field("method", &self.method)
            .field("path", &"<redacted>")
            .field("body", &self.body.as_ref().map(|_| "<redacted>"))
            .field("session_cache", &self.session_cache)
            .finish()
    }
}

impl WireRequest {
    #[must_use]
    pub fn create_conversation() -> Self {
        Self {
            method: Method::Post,
            path: "/compatible-mode/v1/conversations".to_owned(),
            body: Some(json!({})),
            session_cache: false,
        }
    }

    pub fn response(
        conversation: &ConversationRef,
        input: &OperationContent,
        options: &TurnOptions,
    ) -> Result<Self, AlibabaProtocolFailure> {
        options.validate()?;
        Ok(Self {
            method: Method::Post,
            path: "/compatible-mode/v1/responses".to_owned(),
            body: Some(json!({
                "model": EXACT_MODEL_ID,
                "input": input.as_str(),
                "conversation": conversation.as_str(),
                "stream": true,
                "store": false,
                "reasoning": {"effort": "none"}
            })),
            session_cache: false,
        })
    }

    #[must_use]
    pub fn list_items(conversation: &ConversationRef) -> Self {
        Self {
            method: Method::Get,
            path: format!(
                "/compatible-mode/v1/conversations/{}/items?limit=100&order=asc",
                conversation.as_str()
            ),
            body: None,
            session_cache: false,
        }
    }

    #[must_use]
    pub fn delete_item(conversation: &ConversationRef, item: &ItemRef) -> Self {
        Self {
            method: Method::Delete,
            path: format!(
                "/compatible-mode/v1/conversations/{}/items/{}",
                conversation.as_str(),
                item.as_str()
            ),
            body: None,
            session_cache: false,
        }
    }

    #[must_use]
    pub fn delete_conversation(conversation: &ConversationRef) -> Self {
        Self {
            method: Method::Delete,
            path: format!(
                "/compatible-mode/v1/conversations/{}",
                conversation.as_str()
            ),
            body: None,
            session_cache: false,
        }
    }

    #[must_use]
    pub const fn method(&self) -> Method {
        self.method
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub const fn body(&self) -> Option<&Value> {
        self.body.as_ref()
    }

    #[must_use]
    pub const fn session_cache_enabled(&self) -> bool {
        self.session_cache
    }
}
