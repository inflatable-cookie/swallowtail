//! Bounded replay collection and projection for sidecar session load.
//!
//! Replay items arrive as wire `replay_item` events between the
//! `session_replay` command and its response. The collector enforces the
//! strictly increasing wire sequence, the message count cap, and the total
//! content byte bound, and projects each typed message onto the runtime's
//! `SessionReplayItem` surface with distinct semantic kinds.

use super::failure::failure;
use super::wire::replay::{PiSdkReplayItem, PiSdkReplayPart};
use swallowtail_core::SessionRef;
use swallowtail_runtime::{OperationContent, RuntimeFailure, SessionReplayItem, SessionReplayKind};

pub(crate) const MAXIMUM_REPLAY_ITEMS: usize = 1024;
pub(crate) const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct ReplayCollector {
    session: SessionRef,
    expected_sequence: u64,
    messages: usize,
    bytes: usize,
    items: Vec<SessionReplayItem>,
}

impl ReplayCollector {
    pub(crate) const fn new(session: SessionRef) -> Self {
        Self {
            session,
            expected_sequence: 0,
            messages: 0,
            bytes: 0,
            items: Vec::new(),
        }
    }

    pub(crate) fn push(
        &mut self,
        sequence: u64,
        item: PiSdkReplayItem,
    ) -> Result<(), RuntimeFailure> {
        if sequence != self.expected_sequence {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.replay_sequence_gap",
                "Pi SDK sidecar replay sequence was not strictly increasing from zero",
            ));
        }
        self.expected_sequence += 1;
        self.messages += 1;
        if self.messages > MAXIMUM_REPLAY_ITEMS {
            return Err(replay_overflow());
        }
        match item {
            PiSdkReplayItem::User { text, .. } => {
                self.push_content(SessionReplayKind::UserMessage, text)?;
            }
            PiSdkReplayItem::Assistant { parts, .. } => {
                if parts.is_empty() {
                    self.items.push(SessionReplayItem::new(
                        self.session.clone(),
                        self.next_sequence(),
                        SessionReplayKind::AgentMessage,
                    ));
                }
                for part in parts {
                    match part {
                        PiSdkReplayPart::Text(text) => {
                            self.push_content(SessionReplayKind::AgentMessage, text)?;
                        }
                        PiSdkReplayPart::Reasoning(thinking) => {
                            self.push_content(SessionReplayKind::AgentReasoning, thinking)?;
                        }
                        PiSdkReplayPart::ToolCall { .. } => {
                            self.items.push(SessionReplayItem::new(
                                self.session.clone(),
                                self.next_sequence(),
                                SessionReplayKind::ToolCall,
                            ));
                        }
                    }
                }
            }
            PiSdkReplayItem::ToolResult { text, .. } => {
                self.push_content(SessionReplayKind::ToolCallUpdate, text)?;
            }
        }
        Ok(())
    }

    pub(crate) fn finish(
        self,
        reported_items: u64,
        complete: bool,
    ) -> Result<Vec<SessionReplayItem>, RuntimeFailure> {
        if !complete || reported_items != self.messages as u64 {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.replay_incomplete",
                "Pi SDK sidecar replay response did not match the transported replay",
            ));
        }
        Ok(self.items)
    }

    fn push_content(
        &mut self,
        kind: SessionReplayKind,
        text: String,
    ) -> Result<(), RuntimeFailure> {
        self.bytes = self.bytes.saturating_add(text.len());
        if self.bytes > MAXIMUM_REPLAY_BYTES {
            return Err(replay_overflow());
        }
        let sequence = self.next_sequence();
        let item = if text.is_empty() {
            SessionReplayItem::new(self.session.clone(), sequence, kind)
        } else {
            SessionReplayItem::with_content(
                self.session.clone(),
                sequence,
                kind,
                OperationContent::new(text).map_err(|_| {
                    failure(
                        "swallowtail.pi.sdk-sidecar.replay_malformed",
                        "Pi SDK sidecar replay content was invalid",
                    )
                })?,
            )
        };
        self.items.push(item);
        Ok(())
    }

    fn next_sequence(&self) -> u64 {
        u64::try_from(self.items.len()).unwrap_or(u64::MAX)
    }
}

fn replay_overflow() -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.replay_overflow",
        "Pi SDK sidecar replay exceeded the adapter bounds",
    )
}

#[cfg(test)]
mod tests {
    use super::{MAXIMUM_REPLAY_ITEMS, ReplayCollector};
    use crate::sidecar::wire::replay::{PiSdkReplayItem, PiSdkReplayPart};
    use swallowtail_core::SessionRef;
    use swallowtail_runtime::SessionReplayKind;

    fn session() -> SessionRef {
        SessionRef::new("fixture-session").expect("valid session ref")
    }

    #[test]
    fn projects_ordered_typed_items_with_distinct_kinds() {
        let mut collector = ReplayCollector::new(session());
        collector
            .push(
                0,
                PiSdkReplayItem::User {
                    text: "first question".to_owned(),
                    images: 0,
                },
            )
            .unwrap();
        collector
            .push(
                1,
                PiSdkReplayItem::Assistant {
                    parts: vec![
                        PiSdkReplayPart::Reasoning("thinking".to_owned()),
                        PiSdkReplayPart::Text("answer".to_owned()),
                        PiSdkReplayPart::ToolCall {
                            name: "read".to_owned(),
                            arguments: serde_json::json!({}),
                        },
                    ],
                    stop_reason: "stop".to_owned(),
                    usage: None,
                },
            )
            .unwrap();
        collector
            .push(
                2,
                PiSdkReplayItem::ToolResult {
                    name: "read".to_owned(),
                    failed: false,
                    text: "file body".to_owned(),
                },
            )
            .unwrap();
        let items = collector.finish(3, true).expect("replay completes");
        let kinds: Vec<_> = items.iter().map(|item| item.kind()).collect();
        assert_eq!(
            kinds,
            [
                SessionReplayKind::UserMessage,
                SessionReplayKind::AgentReasoning,
                SessionReplayKind::AgentMessage,
                SessionReplayKind::ToolCall,
                SessionReplayKind::ToolCallUpdate,
            ]
        );
        let sequences: Vec<_> = items.iter().map(|item| item.sequence()).collect();
        assert_eq!(sequences, [0, 1, 2, 3, 4]);
        assert_eq!(
            items[1].content().map(|content| content.as_str()),
            Some("thinking")
        );
    }

    #[test]
    fn rejects_gaps_overflow_and_incomplete_replay() {
        let mut collector = ReplayCollector::new(session());
        collector
            .push(
                0,
                PiSdkReplayItem::User {
                    text: "a".to_owned(),
                    images: 0,
                },
            )
            .unwrap();
        let error = collector
            .push(
                2,
                PiSdkReplayItem::User {
                    text: "b".to_owned(),
                    images: 0,
                },
            )
            .expect_err("sequence gap fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.replay_sequence_gap"
        );

        let mut collector = ReplayCollector::new(session());
        for sequence in 0..MAXIMUM_REPLAY_ITEMS as u64 {
            collector
                .push(
                    sequence,
                    PiSdkReplayItem::User {
                        text: String::new(),
                        images: 0,
                    },
                )
                .unwrap();
        }
        let error = collector
            .push(
                MAXIMUM_REPLAY_ITEMS as u64,
                PiSdkReplayItem::User {
                    text: String::new(),
                    images: 0,
                },
            )
            .expect_err("overflow fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.replay_overflow"
        );

        let collector = ReplayCollector::new(session());
        let error = collector.finish(1, true).expect_err("count mismatch fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.replay_incomplete"
        );
        let collector = ReplayCollector::new(session());
        assert!(collector.finish(0, false).is_err());
    }
}
