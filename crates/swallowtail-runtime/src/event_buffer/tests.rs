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
