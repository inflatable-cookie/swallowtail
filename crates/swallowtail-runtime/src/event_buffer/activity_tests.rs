use super::{EventBufferFailureKind, OrderedEventBuffer};
use crate::{
    ActivityCorrelation, ActivityDisclosure, ActivityId, ActivityKind, ActivityLifecyclePhase,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeEvent,
    RuntimeEventKind, RuntimeRunId, RuntimeTurnId,
};

fn activity(
    id: &str,
    operation_id: ActivityOperationId,
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
) -> ActivityObservation {
    ActivityObservation::new(
        ActivityId::new(id).expect("activity id is valid"),
        operation_id,
        ActivityKind::Task,
        phase,
        status,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .expect("activity observation is valid")
}

fn run_owner() -> ActivityOperationId {
    ActivityOperationId::Run(RuntimeRunId::new("run-1").expect("run id is valid"))
}

#[test]
fn complete_and_completion_only_activity_lifecycles_are_accepted() {
    let mut buffer = OrderedEventBuffer::new(8).expect("capacity is valid");
    buffer
        .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            2,
            RuntimeEventKind::Activity(activity(
                "full",
                run_owner(),
                ActivityLifecyclePhase::Started,
                ActivityStatus::Pending,
            )),
        ))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            3,
            RuntimeEventKind::Activity(activity(
                "full",
                run_owner(),
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
            )),
        ))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            4,
            RuntimeEventKind::Activity(activity(
                "full",
                run_owner(),
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
            )),
        ))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            5,
            RuntimeEventKind::Activity(activity(
                "completion-only",
                run_owner(),
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
            )),
        ))
        .expect("completion-only activity is valid");
}

#[test]
fn activity_status_cannot_regress() {
    let mut buffer = OrderedEventBuffer::new(4).expect("capacity is valid");
    buffer
        .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            2,
            RuntimeEventKind::Activity(activity(
                "task",
                run_owner(),
                ActivityLifecyclePhase::Started,
                ActivityStatus::InProgress,
            )),
        ))
        .unwrap();
    let failure = buffer
        .push(RuntimeEvent::new(
            3,
            RuntimeEventKind::Activity(activity(
                "task",
                run_owner(),
                ActivityLifecyclePhase::Updated,
                ActivityStatus::Pending,
            )),
        ))
        .expect_err("status regression must fail");

    assert_eq!(
        failure.kind(),
        EventBufferFailureKind::ActivityStatusRegression
    );
    assert!(!failure.to_string().contains("task"));
}

#[test]
fn activity_completion_is_final() {
    let mut buffer = OrderedEventBuffer::new(5).expect("capacity is valid");
    buffer
        .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            2,
            RuntimeEventKind::Activity(activity(
                "task",
                run_owner(),
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
            )),
        ))
        .unwrap();
    let duplicate = buffer
        .push(RuntimeEvent::new(
            3,
            RuntimeEventKind::Activity(activity(
                "task",
                run_owner(),
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
            )),
        ))
        .expect_err("duplicate completion must fail");
    assert_eq!(
        duplicate.kind(),
        EventBufferFailureKind::DuplicateActivityCompletion
    );

    let after = buffer
        .push(RuntimeEvent::new(
            4,
            RuntimeEventKind::Activity(activity(
                "task",
                run_owner(),
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
            )),
        ))
        .expect_err("post-completion update must fail");
    assert_eq!(
        after.kind(),
        EventBufferFailureKind::ActivityAfterCompletion
    );
}

#[test]
fn activity_owner_and_envelope_cannot_change() {
    let mut buffer = OrderedEventBuffer::new(5).expect("capacity is valid");
    buffer
        .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(
            2,
            RuntimeEventKind::Activity(activity(
                "task",
                run_owner(),
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
            )),
        ))
        .unwrap();
    let foreign_owner =
        ActivityOperationId::Turn(RuntimeTurnId::new("turn-1").expect("turn id is valid"));
    let failure = buffer
        .push(RuntimeEvent::new(
            3,
            RuntimeEventKind::Activity(activity(
                "task",
                foreign_owner,
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
            )),
        ))
        .expect_err("foreign owner must fail");
    assert_eq!(
        failure.kind(),
        EventBufferFailureKind::ActivityIdentityConflict
    );

    let invalid_envelope = RuntimeEvent::with_content(
        4,
        RuntimeEventKind::Activity(activity(
            "other",
            run_owner(),
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
        )),
        OperationContent::new("legacy duplicate content").unwrap(),
    );
    let failure = buffer
        .push(invalid_envelope)
        .expect_err("activity cannot use legacy event content");
    assert_eq!(
        failure.kind(),
        EventBufferFailureKind::ActivityEnvelopeInvalid
    );
    assert!(!failure.to_string().contains("legacy duplicate content"));
}

#[test]
fn correlation_is_adopted_once_then_fixed() {
    let mut buffer = OrderedEventBuffer::new(8).expect("capacity is valid");
    buffer
        .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .unwrap();
    // Codex 0.147 emits item/started before item/tool/call, so the first
    // observation of a consumer tool has no correlation yet.
    buffer
        .push(RuntimeEvent::new(
            2,
            RuntimeEventKind::Activity(activity(
                "tool",
                run_owner(),
                ActivityLifecyclePhase::Started,
                ActivityStatus::InProgress,
            )),
        ))
        .unwrap();
    let correlated = activity(
        "tool",
        run_owner(),
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
    )
    .with_correlation(ActivityCorrelation::Callback(
        crate::CallbackId::new("callback-1").expect("callback id is valid"),
    ));
    buffer
        .push(RuntimeEvent::new(3, RuntimeEventKind::Activity(correlated)))
        .expect("learning the correlation later is not an identity change");
    let conflicting = activity(
        "tool",
        run_owner(),
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
    )
    .with_correlation(ActivityCorrelation::Callback(
        crate::CallbackId::new("callback-2").expect("callback id is valid"),
    ));
    let failure = buffer
        .push(RuntimeEvent::new(
            4,
            RuntimeEventKind::Activity(conflicting),
        ))
        .expect_err("an established correlation cannot change");
    assert_eq!(
        failure.kind(),
        EventBufferFailureKind::ActivityIdentityConflict
    );
    assert!(failure.to_string().contains("tool"));
}

#[test]
fn rejected_overflow_does_not_commit_activity_lifecycle() {
    let mut buffer = OrderedEventBuffer::new(2).expect("capacity is valid");
    buffer
        .push(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .unwrap();
    buffer
        .push(RuntimeEvent::new(2, RuntimeEventKind::OutputAvailable))
        .unwrap();
    let completed = RuntimeEvent::new(
        3,
        RuntimeEventKind::Activity(activity(
            "task",
            run_owner(),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
        )),
    );

    let failure = buffer
        .push(completed.clone())
        .expect_err("semantic overflow must fail before lifecycle commit");
    assert_eq!(failure.kind(), EventBufferFailureKind::SemanticOverflow);

    let _ = buffer.pop_front();
    buffer
        .push(completed)
        .expect("rejected activity must remain eligible for admission");
}
