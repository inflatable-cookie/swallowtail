use crate::failure::AlibabaProtocolFailure;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt;

macro_rules! opaque_ref {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
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
pub struct ConversationInventory {
    items: Vec<ItemRef>,
}

impl ConversationInventory {
    pub fn items(&self) -> impl ExactSizeIterator<Item = &ItemRef> {
        self.items.iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeletionKind {
    ConversationItem,
    Conversation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeletionConfirmation {
    kind: DeletionKind,
}

impl DeletionConfirmation {
    #[must_use]
    pub const fn kind(&self) -> DeletionKind {
        self.kind
    }
}

pub fn parse_conversation(input: &[u8]) -> Result<ConversationRef, AlibabaProtocolFailure> {
    let value = object(input, "conversation creation response")?;
    if text(&value, "/object")? != "conversation"
        || value.get("created_at").and_then(Value::as_u64).is_none()
    {
        return Err(AlibabaProtocolFailure::invalid(
            "conversation creation response",
        ));
    }
    ConversationRef::new(text(&value, "/id")?)
}

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
