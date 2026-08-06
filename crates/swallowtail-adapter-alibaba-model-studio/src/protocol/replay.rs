use super::{ConversationRef, ItemRef};
use crate::failure::AlibabaProtocolFailure;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::SessionRef;
use swallowtail_runtime::{OperationContent, SessionReplayItem, SessionReplayKind};

/// Maximum replay items accepted from one provider page.
pub const MAXIMUM_REPLAY_PAGE_ITEMS: usize = 100;
/// Maximum encoded bytes accepted from one replay page.
pub const MAXIMUM_REPLAY_PAGE_BYTES: usize = 512 * 1024;
/// Maximum provider pages loaded before session readiness.
pub const MAXIMUM_REPLAY_PAGES: usize = 10;
/// Maximum replay items accepted across all pages.
pub const MAXIMUM_REPLAY_ITEMS: usize = MAXIMUM_REPLAY_PAGE_ITEMS * MAXIMUM_REPLAY_PAGES;
/// Maximum decoded content bytes accepted across all pages.
pub const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
/// One validated ordered page of retained conversation replay.
pub struct ConversationReplayPage {
    replay: Vec<SessionReplayItem>,
    item_ids: Vec<ItemRef>,
    next_after: Option<ItemRef>,
    content_bytes: usize,
}

impl ConversationReplayPage {
    /// Iterates projected replay items in provider order.
    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.replay.iter()
    }

    /// Iterates the provider item identities in page order.
    pub fn item_ids(&self) -> impl ExactSizeIterator<Item = &ItemRef> {
        self.item_ids.iter()
    }

    #[must_use]
    /// Returns the cursor for the next page, when the provider reports more.
    pub const fn next_after(&self) -> Option<&ItemRef> {
        self.next_after.as_ref()
    }

    #[must_use]
    /// Returns the decoded content bytes in this page.
    pub const fn content_bytes(&self) -> usize {
        self.content_bytes
    }

    #[must_use]
    /// Consumes the page and returns its ordered replay items.
    pub fn into_replay(self) -> Vec<SessionReplayItem> {
        self.replay
    }
}

/// Parses and bounds one ascending retained-conversation replay page.
pub fn parse_replay_page(
    input: &[u8],
    conversation: &ConversationRef,
    first_sequence: u64,
) -> Result<ConversationReplayPage, AlibabaProtocolFailure> {
    if input.len() > MAXIMUM_REPLAY_PAGE_BYTES {
        return Err(invalid("conversation replay page"));
    }
    let value: Value =
        serde_json::from_slice(input).map_err(|_| invalid("conversation replay page"))?;
    let data = value
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("conversation replay page"))?;
    let has_more = value
        .get("has_more")
        .and_then(Value::as_bool)
        .ok_or_else(|| invalid("conversation replay page"))?;
    if value.get("object").and_then(Value::as_str) != Some("list")
        || data.len() > MAXIMUM_REPLAY_PAGE_ITEMS
        || (has_more && data.is_empty())
    {
        return Err(invalid("bounded conversation replay page"));
    }

    let session =
        SessionRef::new(conversation.as_str()).map_err(|_| invalid("conversation identity"))?;
    let mut seen = BTreeSet::new();
    let mut item_ids = Vec::with_capacity(data.len());
    let mut replay = Vec::with_capacity(data.len());
    let mut content_bytes = 0usize;
    for (offset, item) in data.iter().enumerate() {
        if item.get("type").and_then(Value::as_str) != Some("message")
            || item.get("status").and_then(Value::as_str) != Some("completed")
        {
            return Err(invalid("conversation replay item"));
        }
        let id = item
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("conversation replay item identity"))?;
        let id = ItemRef::new(id)?;
        if !seen.insert(id.clone()) {
            return Err(invalid("conversation replay item identity"));
        }
        let role = item
            .get("role")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("conversation replay role"))?;
        let (kind, content_type) = match role {
            "user" => (SessionReplayKind::UserMessage, "input_text"),
            "assistant" => (SessionReplayKind::AgentMessage, "output_text"),
            _ => return Err(invalid("conversation replay role")),
        };
        let content = item
            .get("content")
            .and_then(Value::as_array)
            .filter(|content| !content.is_empty())
            .ok_or_else(|| invalid("conversation replay content"))?;
        let mut text = Vec::with_capacity(content.len());
        for part in content {
            if part.get("type").and_then(Value::as_str) != Some(content_type) {
                return Err(invalid("conversation replay content"));
            }
            text.push(
                part.get("text")
                    .and_then(Value::as_str)
                    .filter(|text| !text.trim().is_empty())
                    .ok_or_else(|| invalid("conversation replay content"))?,
            );
        }
        let text = text.join("\n");
        content_bytes = content_bytes
            .checked_add(text.len())
            .ok_or_else(|| invalid("conversation replay byte bound"))?;
        let sequence = first_sequence
            .checked_add(
                u64::try_from(offset).map_err(|_| invalid("conversation replay sequence"))?,
            )
            .ok_or_else(|| invalid("conversation replay sequence"))?;
        replay.push(SessionReplayItem::with_content(
            session.clone(),
            sequence,
            kind,
            OperationContent::new(text).map_err(|_| invalid("conversation replay content"))?,
        ));
        item_ids.push(id);
    }

    let first = item_ids.first().map(ItemRef::as_str);
    let last = item_ids.last().map(ItemRef::as_str);
    if value.get("first_id").and_then(Value::as_str) != first
        || value.get("last_id").and_then(Value::as_str) != last
    {
        return Err(invalid("conversation replay page bounds"));
    }
    let next_after = has_more.then(|| item_ids.last().expect("nonempty page checked").clone());
    Ok(ConversationReplayPage {
        replay,
        item_ids,
        next_after,
        content_bytes,
    })
}

fn invalid(subject: &'static str) -> AlibabaProtocolFailure {
    AlibabaProtocolFailure::invalid(subject)
}
