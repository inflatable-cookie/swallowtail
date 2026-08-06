#![deny(missing_docs)]

use crate::activity::{ActivityLifecycleTracker, ActivityTransitionFailure};
use crate::{EventDelivery, RuntimeEvent, RuntimeEventKind};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

/// Stable reason an ordered event buffer rejected an event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventBufferFailureKind {
    /// Buffer capacity must be positive.
    ZeroCapacity,
    /// A semantic event arrived before the operation start event.
    MissingStart,
    /// More than one operation start event was observed.
    DuplicateStart,
    /// An event sequence did not increase monotonically.
    NonMonotonicSequence,
    /// Capacity was exhausted without a replaceable coalescible event.
    SemanticOverflow,
    /// An event arrived after terminal state.
    LateEvent,
    /// An activity event also carried incompatible legacy content.
    ActivityEnvelopeInvalid,
    /// One activity key was reused with conflicting identity dimensions.
    ActivityIdentityConflict,
    /// An activity start arrived after a later phase.
    ActivityPhaseRegression,
    /// An activity status regressed.
    ActivityStatusRegression,
    /// One activity completed more than once.
    DuplicateActivityCompletion,
    /// An activity observation arrived after completion.
    ActivityAfterCompletion,
}

/// Safe event-buffer rejection with a stable classification.
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
    /// Returns the stable buffer-failure classification.
    pub const fn kind(&self) -> EventBufferFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the bounded, redacted diagnostic.
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
    activities: ActivityLifecycleTracker,
    quarantined_late_events: Vec<RuntimeEvent>,
}

impl OrderedEventBuffer {
    /// Creates an ordered event buffer with a positive fixed capacity.
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
            activities: ActivityLifecycleTracker::default(),
            quarantined_late_events: Vec::new(),
        })
    }

    /// Validates and appends an event, coalescing only declared replaceable events.
    pub fn push(&mut self, event: RuntimeEvent) -> Result<(), EventBufferFailure> {
        if self.terminal {
            self.quarantined_late_events.push(event);
            return Err(EventBufferFailure::new(
                EventBufferFailureKind::LateEvent,
                "Operation event arrived after the terminal outcome",
            ));
        }
        self.validate_order(&event)?;
        let replacement = self.coalescible_replacement(&event)?;
        self.validate_activity(&event)?;

        if let Some(index) = replacement {
            self.events.remove(index);
        }
        if matches!(event.kind(), RuntimeEventKind::Started) {
            self.started = true;
        }
        self.last_sequence = Some(event.sequence());
        self.events.push_back(event);
        Ok(())
    }

    #[must_use]
    /// Removes and returns the oldest retained event.
    pub fn pop_front(&mut self) -> Option<RuntimeEvent> {
        self.events.pop_front()
    }

    #[must_use]
    /// Returns the number of currently retained events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    #[must_use]
    /// Reports whether no event is currently retained.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Marks the operation terminal so later events are quarantined and rejected.
    pub fn mark_terminal(&mut self) {
        self.terminal = true;
    }

    /// Iterates events quarantined after terminal state.
    pub fn quarantined_late_events(&self) -> impl ExactSizeIterator<Item = &RuntimeEvent> {
        self.quarantined_late_events.iter()
    }

    fn validate_order(&self, event: &RuntimeEvent) -> Result<(), EventBufferFailure> {
        match (event.kind(), self.started) {
            (RuntimeEventKind::Started, true) => {
                return Err(EventBufferFailure::new(
                    EventBufferFailureKind::DuplicateStart,
                    "Operation emitted more than one start event",
                ));
            }
            (RuntimeEventKind::Started, false) => {}
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

    fn coalescible_replacement(
        &self,
        event: &RuntimeEvent,
    ) -> Result<Option<usize>, EventBufferFailure> {
        if self.events.len() < self.capacity {
            return Ok(None);
        }
        if event.delivery() != EventDelivery::Coalescible {
            return Err(Self::semantic_overflow());
        }
        self.events
            .iter()
            .rposition(|buffered| buffered.delivery() == EventDelivery::Coalescible)
            .map(Some)
            .ok_or_else(Self::semantic_overflow)
    }

    fn semantic_overflow() -> EventBufferFailure {
        EventBufferFailure::new(
            EventBufferFailureKind::SemanticOverflow,
            "Event buffer cannot discard a semantic event",
        )
    }

    fn validate_activity(&mut self, event: &RuntimeEvent) -> Result<(), EventBufferFailure> {
        let RuntimeEventKind::Activity(observation) = event.kind() else {
            return Ok(());
        };
        if event.content().is_some() {
            return Err(EventBufferFailure::new(
                EventBufferFailureKind::ActivityEnvelopeInvalid,
                "Activity event cannot carry legacy operation content",
            ));
        }

        self.activities
            .observe(observation)
            .map_err(activity_transition_failure)
    }
}

fn activity_transition_failure(failure: ActivityTransitionFailure) -> EventBufferFailure {
    let (kind, message) = match failure {
        ActivityTransitionFailure::IdentityConflict => (
            EventBufferFailureKind::ActivityIdentityConflict,
            "Activity identity changed within one operation",
        ),
        ActivityTransitionFailure::PhaseRegression => (
            EventBufferFailureKind::ActivityPhaseRegression,
            "Activity start arrived after an earlier observation",
        ),
        ActivityTransitionFailure::StatusRegression => (
            EventBufferFailureKind::ActivityStatusRegression,
            "Activity status regressed",
        ),
        ActivityTransitionFailure::DuplicateCompletion => (
            EventBufferFailureKind::DuplicateActivityCompletion,
            "Activity emitted more than one completion",
        ),
        ActivityTransitionFailure::AfterCompletion => (
            EventBufferFailureKind::ActivityAfterCompletion,
            "Activity emitted an observation after completion",
        ),
    };
    EventBufferFailure::new(kind, message)
}

#[cfg(test)]
#[path = "event_buffer/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "event_buffer/activity_tests.rs"]
mod activity_tests;
