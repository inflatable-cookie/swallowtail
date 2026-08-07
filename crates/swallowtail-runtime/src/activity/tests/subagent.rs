#[test]
fn subagent_graph_metadata_is_bounded_redacted_and_separate_from_control_authority() {
    let child = SubagentId::new("provider-child-private").unwrap();
    let snapshot = SubagentSnapshot::new(
        child.clone(),
        SubagentParent::Operation,
        SubagentStatus::Running,
    )
    .with_label(ActivityLabel::new("Explore tests").unwrap())
    .with_description(
        ActivityContent::new(
            OperationContent::new("Inspect the private test layout").unwrap(),
            64,
        )
        .unwrap(),
    )
    .with_model(swallowtail_core::ModelId::new("private-model").unwrap())
    .with_reasoning(swallowtail_core::ReasoningMode::new("high").unwrap())
    .with_background(true);
    let observation = ActivityObservation::new(
        ActivityId::new("collaboration-1").unwrap(),
        run_id(),
        ActivityKind::SubagentOrCollaboration,
        ActivityLifecyclePhase::Started,
        ActivityStatus::InProgress,
        None,
        ActivityDisclosure::AdapterNormalizedSummary,
    )
    .unwrap()
    .with_actor(ActivityActor::Subagent(child.clone()))
    .with_subagents([snapshot])
    .unwrap()
    .with_subagent_control(SubagentControlActionKind::Spawn)
    .unwrap();

    assert_eq!(observation.actor(), &ActivityActor::Subagent(child));
    let child = observation.subagents().next().unwrap();
    assert_eq!(child.status(), SubagentStatus::Running);
    assert_eq!(child.parent(), &SubagentParent::Operation);
    assert_eq!(
        child.label().map(ActivityLabel::as_str),
        Some("Explore tests")
    );
    assert_eq!(child.background(), Some(true));
    assert_eq!(
        observation.subagent_control(),
        Some(SubagentControlActionKind::Spawn)
    );
    let rendered = format!("{observation:?}");
    assert!(!rendered.contains("provider-child-private"));
    assert!(!rendered.contains("Inspect the private test layout"));
    assert!(!rendered.contains("private-model"));

    assert!(
        ActivityObservation::new(
            ActivityId::new("not-subagent").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .unwrap()
        .with_subagents([SubagentSnapshot::new(
            SubagentId::new("child").unwrap(),
            SubagentParent::Unknown,
            SubagentStatus::Unknown,
        )])
        .is_err()
    );

    let collaboration = || {
        ActivityObservation::new(
            ActivityId::new("bounded-subagents").unwrap(),
            run_id(),
            ActivityKind::SubagentOrCollaboration,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .unwrap()
    };
    let duplicate = SubagentSnapshot::new(
        SubagentId::new("duplicate").unwrap(),
        SubagentParent::Operation,
        SubagentStatus::Pending,
    );
    assert!(
        collaboration()
            .with_subagents([duplicate.clone(), duplicate])
            .is_err()
    );
    assert!(
        collaboration()
            .with_subagents((0..65).map(|index| {
                SubagentSnapshot::new(
                    SubagentId::new(format!("child-{index}")).unwrap(),
                    SubagentParent::Operation,
                    SubagentStatus::Pending,
                )
            }))
            .is_err()
    );
}

