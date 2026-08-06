use crate::failure::AlibabaProtocolFailure;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt;

macro_rules! opaque_ref {
    ($name:ident, $label:literal) => {
        #[doc = concat!("Opaque, bounded Alibaba ", $label, ".")]
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates an Alibaba ", $label, " after validating its bounded form.")]
            pub fn new(value: impl Into<String>) -> Result<Self, AlibabaProtocolFailure> {
                let value = value.into();
                if value.trim().is_empty() || value.len() > 256 {
                    Err(AlibabaProtocolFailure::invalid($label))
                } else {
                    Ok(Self(value))
                }
            }

            pub(crate) fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<redacted>")
                    .finish()
            }
        }
    };
}

opaque_ref!(ConversationRef, "conversation reference");
opaque_ref!(ItemRef, "conversation item reference");
opaque_ref!(ResponseRef, "provider response reference");

#[derive(Clone, Debug, Eq, PartialEq)]
/// Validated provider conversation identity and creation time.
pub struct ConversationMetadata {
    conversation: ConversationRef,
    created_at: u64,
}

impl ConversationMetadata {
    #[must_use]
    /// Returns the opaque conversation reference.
    pub const fn conversation(&self) -> &ConversationRef {
        &self.conversation
    }

    #[must_use]
    /// Returns the provider creation timestamp.
    pub const fn created_at(&self) -> u64 {
        self.created_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Complete bounded inventory of items in one conversation.
pub struct ConversationInventory {
    items: Vec<ItemRef>,
}

impl ConversationInventory {
    /// Iterates exact item references in provider order.
    pub fn items(&self) -> impl ExactSizeIterator<Item = &ItemRef> {
        self.items.iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Provider resource kind acknowledged by a deletion response.
pub enum DeletionKind {
    /// One item inside a conversation.
    ConversationItem,
    /// The conversation container itself.
    Conversation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Validated provider acknowledgement of an exact resource deletion.
pub struct DeletionConfirmation {
    kind: DeletionKind,
}

impl DeletionConfirmation {
    #[must_use]
    /// Returns the deleted resource kind.
    pub const fn kind(&self) -> DeletionKind {
        self.kind
    }
}

/// Parses a newly created conversation reference.
pub fn parse_conversation(input: &[u8]) -> Result<ConversationRef, AlibabaProtocolFailure> {
    Ok(parse_conversation_metadata(input)?.conversation)
}

/// Parses retrieved metadata and verifies the expected conversation identity.
pub fn parse_conversation_retrieval(
    input: &[u8],
    expected: &ConversationRef,
) -> Result<ConversationMetadata, AlibabaProtocolFailure> {
    let metadata = parse_conversation_metadata(input)?;
    if metadata.conversation != *expected {
        return Err(AlibabaProtocolFailure::invalid(
            "conversation retrieval identity",
        ));
    }
    Ok(metadata)
}

fn parse_conversation_metadata(
    input: &[u8],
) -> Result<ConversationMetadata, AlibabaProtocolFailure> {
    let value = object(input, "conversation creation response")?;
    if text(&value, "/object")? != "conversation" {
        return Err(AlibabaProtocolFailure::invalid("conversation response"));
    }
    let created_at = value
        .get("created_at")
        .and_then(Value::as_u64)
        .ok_or_else(|| AlibabaProtocolFailure::invalid("conversation response"))?;
    Ok(ConversationMetadata {
        conversation: ConversationRef::new(text(&value, "/id")?)?,
        created_at,
    })
}

/// Parses a complete bounded item inventory for deletion.
pub fn parse_inventory(input: &[u8]) -> Result<ConversationInventory, AlibabaProtocolFailure> {
    let value = object(input, "conversation item inventory")?;
    let data = value
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| AlibabaProtocolFailure::invalid("conversation item inventory"))?;
    if text(&value, "/object")? != "list"
        || value.get("has_more").and_then(Value::as_bool) != Some(false)
        || data.len() > 100
    {
        return Err(AlibabaProtocolFailure::invalid(
            "complete bounded conversation item inventory",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut items = Vec::with_capacity(data.len());
    for item in data {
        if text(item, "/type")? != "message" || text(item, "/status")? != "completed" {
            return Err(AlibabaProtocolFailure::invalid("conversation item"));
        }
        let id = ItemRef::new(text(item, "/id")?)?;
        if !seen.insert(id.clone()) {
            return Err(AlibabaProtocolFailure::invalid(
                "conversation item identity",
            ));
        }
        items.push(id);
    }
    if !items.is_empty()
        && (value.get("first_id").and_then(Value::as_str) != items.first().map(ItemRef::as_str)
            || value.get("last_id").and_then(Value::as_str) != items.last().map(ItemRef::as_str))
    {
        return Err(AlibabaProtocolFailure::invalid(
            "conversation item inventory bounds",
        ));
    }
    Ok(ConversationInventory { items })
}

/// Parses and verifies deletion of one exact conversation resource.
pub fn parse_deletion(
    input: &[u8],
    expected_id: &str,
    kind: DeletionKind,
) -> Result<DeletionConfirmation, AlibabaProtocolFailure> {
    let value = object(input, "deletion confirmation")?;
    let expected_object = match kind {
        DeletionKind::ConversationItem => "conversation.item.deleted",
        DeletionKind::Conversation => "conversation.deleted",
    };
    if value.get("deleted").and_then(Value::as_bool) != Some(true)
        || value.get("id").and_then(Value::as_str) != Some(expected_id)
        || value.get("object").and_then(Value::as_str) != Some(expected_object)
    {
        return Err(AlibabaProtocolFailure::invalid("deletion confirmation"));
    }
    Ok(DeletionConfirmation { kind })
}

fn object(input: &[u8], subject: &'static str) -> Result<Value, AlibabaProtocolFailure> {
    if input.len() > 512 * 1024 {
        return Err(AlibabaProtocolFailure::invalid(subject));
    }
    serde_json::from_slice(input).map_err(|_| AlibabaProtocolFailure::invalid(subject))
}

fn text<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, AlibabaProtocolFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| AlibabaProtocolFailure::invalid("provider response field"))
}
