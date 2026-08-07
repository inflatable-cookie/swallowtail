#[test]
fn task_list_snapshots_are_typed_bounded_redacted_and_clearable() {
    let snapshot = TaskListSnapshot::new(
        [
            TaskListItem::new(
                OperationContent::new("inspect private source").unwrap(),
                TaskListItemStatus::Completed,
            )
            .with_priority(TaskListItemPriority::High),
            TaskListItem::new(
                OperationContent::new("run private tests").unwrap(),
                TaskListItemStatus::InProgress,
            ),
        ],
        4,
        64,
    )
    .unwrap();
    let observation = ActivityObservation::new(
        ActivityId::new("plan-1").unwrap(),
        run_id(),
        ActivityKind::Plan,
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
        None,
        ActivityDisclosure::ProviderDisplayContent,
    )
    .unwrap()
    .with_task_list(snapshot)
    .unwrap();

    let items = observation
        .task_list()
        .expect("snapshot is carried")
        .items()
        .collect::<Vec<_>>();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].status(), TaskListItemStatus::Completed);
    assert_eq!(items[0].priority(), Some(TaskListItemPriority::High));
    assert_eq!(items[1].content().as_str(), "run private tests");
    assert!(!format!("{observation:?}").contains("private"));

    let cleared = TaskListSnapshot::new([], 4, 64).expect("empty replacement clears the list");
    assert!(cleared.is_empty());
    assert!(
        TaskListSnapshot::new(
            [TaskListItem::new(
                OperationContent::new("too long").unwrap(),
                TaskListItemStatus::Pending,
            )],
            4,
            3,
        )
        .is_err()
    );
}

#[test]
fn task_list_snapshots_are_limited_to_plan_and_task_activity() {
    let result = ActivityObservation::new(
        ActivityId::new("message-list").unwrap(),
        run_id(),
        ActivityKind::AssistantMessage,
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
        Some(ActivityAssistantPhase::Intermediate),
        ActivityDisclosure::ProviderDisplayContent,
    )
    .unwrap()
    .with_task_list(TaskListSnapshot::new([], 1, 1).unwrap());

    assert!(result.is_err());
}

