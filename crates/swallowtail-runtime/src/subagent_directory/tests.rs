use super::{
    SubagentDirectoryChangeKind, SubagentDirectoryFailureKind, SubagentDirectoryProjection,
};
use crate::{
    ActivityActor, ActivityDisclosure, ActivityId, ActivityKind, ActivityLifecyclePhase,
    ActivityObservation, ActivityOperationId, ActivityStatus, RuntimeEvent, RuntimeEventKind,
    RuntimeRunId, SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus,
};

fn operation(value: &str) -> ActivityOperationId {
    ActivityOperationId::Run(RuntimeRunId::new(value).unwrap())
}

fn activity(
    operation_id: ActivityOperationId,
    actor: ActivityActor,
    snapshots: impl IntoIterator<Item = SubagentSnapshot>,
) -> ActivityObservation {
    ActivityObservation::new(
        ActivityId::new("collaboration").unwrap(),
        operation_id,
        ActivityKind::SubagentOrCollaboration,
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
        None,
        ActivityDisclosure::AdapterNormalizedSummary,
    )
    .unwrap()
    .with_actor(actor)
    .with_subagents(snapshots)
    .unwrap()
}

#[test]
fn directory_rejects_zero_capacity() {
    let failure = SubagentDirectoryProjection::new(operation("run"), 0)
        .err()
        .expect("zero capacity rejects");

    assert_eq!(
        failure.kind(),
        SubagentDirectoryFailureKind::InvalidCapacity
    );
}

#[test]
fn directory_preserves_first_seen_order_and_replaces_snapshots() {
    let operation_id = operation("run");
    let first = SubagentId::new("first").unwrap();
    let second = SubagentId::new("second").unwrap();
    let mut directory = SubagentDirectoryProjection::new(operation_id.clone(), 4).unwrap();

    let added = directory
        .observe_activity(&activity(
            operation_id.clone(),
            ActivityActor::Primary,
            [
                SubagentSnapshot::new(
                    first.clone(),
                    SubagentParent::Operation,
                    SubagentStatus::Pending,
                ),
                SubagentSnapshot::new(
                    second.clone(),
                    SubagentParent::Operation,
                    SubagentStatus::Running,
                ),
            ],
        ))
        .unwrap();

    assert_eq!(added.actor(), &ActivityActor::Primary);
    assert_eq!(
        added
            .changes()
            .map(|change| change.kind())
            .collect::<Vec<_>>(),
        [
            SubagentDirectoryChangeKind::Added,
            SubagentDirectoryChangeKind::Added
        ]
    );

    let replaced = directory
        .observe_activity(&activity(
            operation_id,
            ActivityActor::Subagent(first.clone()),
            [SubagentSnapshot::new(
                first.clone(),
                SubagentParent::Operation,
                SubagentStatus::Completed,
            )],
        ))
        .unwrap();

    assert_eq!(
        replaced
            .changes()
            .map(|change| (change.id().clone(), change.kind()))
            .collect::<Vec<_>>(),
        [(first.clone(), SubagentDirectoryChangeKind::Replaced)]
    );
    assert_eq!(
        directory
            .subagents()
            .map(|snapshot| snapshot.id().clone())
            .collect::<Vec<_>>(),
        [first.clone(), second]
    );
    assert_eq!(
        directory.get(&first).unwrap().status(),
        SubagentStatus::Completed
    );
}

#[test]
fn identical_snapshots_emit_no_change() {
    let operation_id = operation("run");
    let child = SubagentId::new("child").unwrap();
    let snapshot = || {
        SubagentSnapshot::new(
            child.clone(),
            SubagentParent::Operation,
            SubagentStatus::Running,
        )
    };
    let mut directory = SubagentDirectoryProjection::new(operation_id.clone(), 2).unwrap();

    directory
        .observe_activity(&activity(
            operation_id.clone(),
            ActivityActor::Primary,
            [snapshot()],
        ))
        .unwrap();
    let unchanged = directory
        .observe_activity(&activity(
            operation_id,
            ActivityActor::Primary,
            [snapshot()],
        ))
        .unwrap();

    assert!(unchanged.is_unchanged());
}

#[test]
fn known_actor_and_parent_identities_create_unknown_placeholders() {
    let operation_id = operation("run");
    let parent = SubagentId::new("parent").unwrap();
    let child = SubagentId::new("child").unwrap();
    let actor = SubagentId::new("actor").unwrap();
    let mut directory = SubagentDirectoryProjection::new(operation_id.clone(), 4).unwrap();

    let delta = directory
        .observe_activity(&activity(
            operation_id,
            ActivityActor::Subagent(actor.clone()),
            [SubagentSnapshot::new(
                child.clone(),
                SubagentParent::Subagent(parent.clone()),
                SubagentStatus::Running,
            )],
        ))
        .unwrap();

    assert_eq!(delta.actor(), &ActivityActor::Subagent(actor.clone()));
    assert_eq!(directory.len(), 3);
    assert_eq!(
        directory.get(&parent).unwrap().parent(),
        &SubagentParent::Unknown
    );
    assert_eq!(
        directory.get(&actor).unwrap().status(),
        SubagentStatus::Unknown
    );
    assert_eq!(
        directory
            .children_of(&parent)
            .map(|snapshot| snapshot.id().clone())
            .collect::<Vec<_>>(),
        [child]
    );
    assert_eq!(
        directory
            .unknown_parent()
            .map(|snapshot| snapshot.id().clone())
            .collect::<Vec<_>>(),
        [parent, actor]
    );
}

#[test]
fn operation_mismatch_and_capacity_failure_do_not_mutate() {
    let operation_id = operation("run");
    let existing = SubagentId::new("existing").unwrap();
    let mut directory = SubagentDirectoryProjection::new(operation_id.clone(), 1).unwrap();
    directory
        .observe_activity(&activity(
            operation_id.clone(),
            ActivityActor::Primary,
            [SubagentSnapshot::new(
                existing.clone(),
                SubagentParent::Operation,
                SubagentStatus::Running,
            )],
        ))
        .unwrap();

    let mismatch = directory
        .observe_activity(&activity(operation("other"), ActivityActor::Primary, []))
        .unwrap_err();
    assert_eq!(
        mismatch.kind(),
        SubagentDirectoryFailureKind::OperationMismatch
    );

    let capacity = directory
        .observe_activity(&activity(
            operation_id,
            ActivityActor::Primary,
            [SubagentSnapshot::new(
                SubagentId::new("overflow").unwrap(),
                SubagentParent::Operation,
                SubagentStatus::Pending,
            )],
        ))
        .unwrap_err();
    assert_eq!(
        capacity.kind(),
        SubagentDirectoryFailureKind::CapacityExceeded
    );
    assert_eq!(directory.len(), 1);
    assert_eq!(directory.subagents().next().unwrap().id(), &existing);
}

#[test]
fn runtime_event_observation_routes_activity_and_ignores_other_events() {
    let operation_id = operation("run");
    let child = SubagentId::new("child").unwrap();
    let mut directory = SubagentDirectoryProjection::new(operation_id.clone(), 2).unwrap();
    let event = RuntimeEvent::new(
        4,
        RuntimeEventKind::Activity(activity(
            operation_id,
            ActivityActor::Subagent(child.clone()),
            [],
        )),
    );

    let delta = directory.observe_event(&event).unwrap().unwrap();
    assert_eq!(delta.actor(), &ActivityActor::Subagent(child));
    assert_eq!(
        delta.changes().next().unwrap().kind(),
        SubagentDirectoryChangeKind::Added
    );
    assert!(
        directory
            .observe_event(&RuntimeEvent::new(5, RuntimeEventKind::Progress))
            .unwrap()
            .is_none()
    );
}
