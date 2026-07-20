use crate::{EventDelivery, RuntimeEvent, RuntimeEventKind};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventBufferFailureKind {
    ZeroCapacity,
    MissingStart,
    DuplicateStart,
    NonMonotonicSequence,
    SemanticOverflow,
    LateEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventBufferFailure {
    kind: EventBufferFailureKind,
    diagnostic: SafeDiagnostic,
}

impl EventBufferFailure {
    fn new(kind: EventBufferFailureKind, message: &'static str) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new("swallowtail.event_buffer_rejected", message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> EventBufferFailureKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for EventBufferFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for EventBufferFailure {}

/// Deterministic bounded ingress buffer enforcing the common event contract.
#[derive(Debug)]
pub struct OrderedEventBuffer {
    capacity: usize,
    events: VecDeque<RuntimeEvent>,
    last_sequence: Option<u64>,
    started: bool,
    terminal: bool,
    quarantined_late_events: Vec<RuntimeEvent>,
}

impl OrderedEventBuffer {
    pub fn new(capacity: usize) -> Result<Self, EventBufferFailure> {
        if capacity == 0 {
            return Err(EventBufferFailure::new(
                EventBufferFailureKind::ZeroCapacity,
                "Event buffer capacity must be greater than zero",
            ));
        }
        Ok(Self {
            capacity,
            events: VecDeque::with_capacity(capacity),
            last_sequence: None,
            started: false,
            terminal: false,
            quarantined_late_events: Vec::new(),
        })
    }

    pub fn push(&mut self, event: RuntimeEvent) -> Result<(), EventBufferFailure> {
        if self.terminal {
            self.quarantined_late_events.push(event);
            return Err(EventBufferFailure::new(
                EventBufferFailureKind::LateEvent,
                "Operation event arrived after the terminal outcome",
            ));
        }
        self.validate_order(&event)?;

        if self.events.len() == self.capacity {
            if event.delivery() == EventDelivery::Coalescible {
                if let Some(index) = self
                    .events
                    .iter()
                    .rposition(|buffered| buffered.delivery() == EventDelivery::Coalescible)
                {
                    self.events.remove(index);
                } else {
                    return Err(Self::semantic_overflow());
                }
            } else {
                return Err(Self::semantic_overflow());
            }
        }

        self.last_sequence = Some(event.sequence());
        self.events.push_back(event);
        Ok(())
    }

    #[must_use]
    pub fn pop_front(&mut self) -> Option<RuntimeEvent> {
        self.events.pop_front()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn mark_terminal(&mut self) {
        self.terminal = true;
    }

    pub fn quarantined_late_events(&self) -> impl ExactSizeIterator<Item = &RuntimeEvent> {
        self.quarantined_late_events.iter()
    }

    fn validate_order(&mut self, event: &RuntimeEvent) -> Result<(), EventBufferFailure> {
        match (event.kind(), self.started) {
            (RuntimeEventKind::Started, true) => {
                return Err(EventBufferFailure::new(
                    EventBufferFailureKind::DuplicateStart,
                    "Operation emitted more than one start event",
                ));
            }
            (RuntimeEventKind::Started, false) => self.started = true,
            (_, false) => {
                return Err(EventBufferFailure::new(
                    EventBufferFailureKind::MissingStart,
                    "Operation event arrived before the start event",
                ));
            }
            (_, true) => {}
        }
        if self
            .last_sequence
            .is_some_and(|previous| event.sequence() <= previous)
        {
            return Err(EventBufferFailure::new(
                EventBufferFailureKind::NonMonotonicSequence,
                "Operation event sequence must increase monotonically",
            ));
        }
        Ok(())
    }

    fn semantic_overflow() -> EventBufferFailure {
        EventBufferFailure::new(
            EventBufferFailureKind::SemanticOverflow,
            "Event buffer cannot discard a semantic event",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{EventBufferFailureKind, OrderedEventBuffer};
    use crate::{RuntimeEvent, RuntimeEventKind};

    #[test]
    fn start_and_sequence_order_are_enforced() {
        let mut buffer = OrderedEventBuffer::new(3).expect("capacity is valid");
        let missing_start = buffer
            .push(RuntimeEvent::new(1, RuntimeEventKind::Progress))
            .expect_err("progress before start must fail");
        assert_eq!(missing_start.kind(), EventBufferFailureKind::MissingStart);

        buffer
            .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is valid");
        let duplicate = buffer
            .push(RuntimeEvent::new(2, RuntimeEventKind::Started))
            .expect_err("duplicate start must fail");
        assert_eq!(duplicate.kind(), EventBufferFailureKind::DuplicateStart);
    }

    #[test]
    fn only_coalescible_events_can_be_replaced() {
        let mut buffer = OrderedEventBuffer::new(2).expect("capacity is valid");
        buffer
            .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is valid");
        buffer
            .push(RuntimeEvent::new(2, RuntimeEventKind::ProgressSnapshot))
            .expect("snapshot is valid");
        buffer
            .push(RuntimeEvent::new(3, RuntimeEventKind::ProgressSnapshot))
            .expect("new snapshot replaces the old snapshot");

        assert_eq!(buffer.len(), 2);
        assert_eq!(
            buffer.pop_front().expect("start remains").kind(),
            &RuntimeEventKind::Started
        );
        assert_eq!(
            buffer
                .pop_front()
                .expect("latest snapshot remains")
                .sequence(),
            3
        );
    }

    #[test]
    fn semantic_overflow_fails_instead_of_dropping() {
        let mut buffer = OrderedEventBuffer::new(1).expect("capacity is valid");
        buffer
            .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is valid");
        let failure = buffer
            .push(RuntimeEvent::new(2, RuntimeEventKind::OutputAvailable))
            .expect_err("semantic overflow must fail");

        assert_eq!(failure.kind(), EventBufferFailureKind::SemanticOverflow);
        assert_eq!(buffer.len(), 1);
    }

    #[test]
    fn late_events_are_quarantined() {
        let mut buffer = OrderedEventBuffer::new(2).expect("capacity is valid");
        buffer
            .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is valid");
        buffer.mark_terminal();
        let failure = buffer
            .push(RuntimeEvent::new(2, RuntimeEventKind::OutputAvailable))
            .expect_err("late event must fail");

        assert_eq!(failure.kind(), EventBufferFailureKind::LateEvent);
        assert_eq!(buffer.quarantined_late_events().count(), 1);
    }
}
