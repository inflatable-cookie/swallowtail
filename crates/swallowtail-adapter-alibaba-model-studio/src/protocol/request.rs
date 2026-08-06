use super::{ConversationRef, ItemRef};
use crate::failure::AlibabaProtocolFailure;
use crate::selection::EXACT_MODEL_ID;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::OperationContent;
use url::form_urlencoded::byte_serialize;

mod options;

pub use options::TurnOptions;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// HTTP method used by a bounded workspace request.
pub enum Method {
    /// Retrieves a provider resource.
    Get,
    /// Creates a resource or submits inference.
    Post,
    /// Deletes a provider resource.
    Delete,
}

#[derive(Clone, Eq, PartialEq)]
/// Redacted, validated HTTP request for the qualified Model Studio subset.
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
    pub(crate) fn deployable_models(page: u32, source: &str) -> Self {
        Self {
            method: Method::Get,
            path: format!(
                "/api/v1/deployments/models?page_no={page}&page_size=100&version=v1.0&model_source={source}"
            ),
            body: None,
            session_cache: false,
        }
    }

    #[must_use]
    /// Builds a conversation-creation request.
    pub fn create_conversation() -> Self {
        Self {
            method: Method::Post,
            path: "/compatible-mode/v1/conversations".to_owned(),
            body: Some(json!({})),
            session_cache: false,
        }
    }

    #[must_use]
    /// Builds an exact conversation-retrieval request.
    pub fn retrieve_conversation(conversation: &ConversationRef) -> Self {
        Self {
            method: Method::Get,
            path: format!(
                "/compatible-mode/v1/conversations/{}",
                conversation.as_str()
            ),
            body: None,
            session_cache: false,
        }
    }

    /// Builds a streamed turn inside an existing conversation.
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

    /// Builds one unstored, conversation-free structured response.
    pub fn structured_response(input: &OperationContent) -> Result<Self, AlibabaProtocolFailure> {
        Ok(Self {
            method: Method::Post,
            path: "/compatible-mode/v1/responses".to_owned(),
            body: Some(json!({
                "model": EXACT_MODEL_ID,
                "input": input.as_str(),
                "stream": true,
                "store": false,
                "reasoning": {"effort": "none"}
            })),
            session_cache: false,
        })
    }

    #[must_use]
    /// Builds a first page request for complete ordered conversation items.
    pub fn list_items(conversation: &ConversationRef) -> Self {
        Self::list_items_after(conversation, None)
    }

    #[must_use]
    /// Builds an ordered item-page request after an optional cursor.
    pub fn list_items_after(conversation: &ConversationRef, after: Option<&ItemRef>) -> Self {
        let mut path = format!(
            "/compatible-mode/v1/conversations/{}/items?limit=100&order=asc",
            conversation.as_str()
        );
        if let Some(after) = after {
            path.push_str("&after=");
            path.extend(byte_serialize(after.as_str().as_bytes()));
        }
        Self {
            method: Method::Get,
            path,
            body: None,
            session_cache: false,
        }
    }

    #[must_use]
    /// Builds deletion of one exact conversation item.
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
    /// Builds deletion of one exact conversation.
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
    /// Returns the HTTP method.
    pub const fn method(&self) -> Method {
        self.method
    }

    #[must_use]
    /// Returns the provider-relative request path.
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    /// Returns the optional JSON request body.
    pub const fn body(&self) -> Option<&Value> {
        self.body.as_ref()
    }

    #[must_use]
    /// Returns whether provider session caching was requested.
    pub const fn session_cache_enabled(&self) -> bool {
        self.session_cache
    }
}
