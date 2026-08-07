#[test]
fn identities_and_content_are_bounded_and_redacted() {
    let id = ActivityId::new("private-activity").expect("activity id is valid");
    let namespace = ActivityNamespace::new("provider.private-kind").expect("namespace is valid");
    let label = ActivityLabel::new("Read File").expect("label is valid");
    let content = ActivityContent::new(
        OperationContent::new("private visible task content").expect("content is valid"),
        64,
    )
    .expect("content fits");

    assert_eq!(id.as_str(), "private-activity");
    assert_eq!(namespace.as_str(), "provider.private-kind");
    assert_eq!(content.as_str(), "private visible task content");
    assert_eq!(label.as_str(), "Read File");
    assert!(!format!("{id:?}").contains(id.as_str()));
    assert!(!id.to_string().contains(id.as_str()));
    assert!(!format!("{content:?}").contains(content.as_str()));
    assert!(!content.to_string().contains(content.as_str()));
    assert!(!format!("{label:?}").contains(label.as_str()));
    assert!(!label.to_string().contains(label.as_str()));
    assert!(ActivityId::new("x".repeat(257)).is_err());
    assert!(ActivityNamespace::new("x".repeat(129)).is_err());
    assert!(ActivityLabel::new("x".repeat(513)).is_err());
    assert!(ActivityContent::new(OperationContent::new("private").unwrap(), 3).is_err());
}

#[test]
fn activity_key_isolates_reused_local_and_provider_ids_across_operations() {
    let activity_id = ActivityId::new("provider-message-reused").unwrap();
    let provider_ref = ProviderActivityRef::new("provider/message/reused").unwrap();
    let observation = |run: &str| {
        ActivityObservation::new(
            activity_id.clone(),
            ActivityOperationId::Run(RuntimeRunId::new(run).unwrap()),
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .unwrap()
        .with_provider_activity_ref(provider_ref.clone())
    };
    let first = observation("consumer-operation-a");
    let second = observation("consumer-operation-b");

    assert_eq!(first.activity_id(), second.activity_id());
    assert_eq!(
        first.provider_activity_ref(),
        second.provider_activity_ref()
    );
    assert_ne!(first.key(), second.key());
    assert_eq!(first.key().activity_id(), first.activity_id());
    assert_eq!(first.key().operation_id(), first.operation_id());
    assert_eq!(
        std::collections::BTreeSet::<ActivityKey>::from([first.key(), second.key()]).len(),
        2
    );
    assert!(!format!("{:?}", first.key()).contains("consumer-operation-a"));
    assert!(!format!("{:?}", first.key()).contains("provider-message-reused"));
}

