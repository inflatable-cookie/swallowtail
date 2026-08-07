#[test]
fn completion_only_and_correlations_are_representable_without_raw_bodies() {
    let observation = ActivityObservation::new(
        ActivityId::new("tool-1").unwrap(),
        run_id(),
        ActivityKind::ConsumerOwnedTool,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::ProviderDisplayContent,
    )
    .unwrap()
    .with_provider_activity_ref(
        ProviderActivityRef::new("provider/private/item").expect("provider ref is valid"),
    )
    .with_correlation(ActivityCorrelation::Callback(
        CallbackId::new("private-callback").unwrap(),
    ))
    .with_label(ActivityLabel::new("Private tool label").unwrap())
    .unwrap()
    .with_content(ActivityContentUpdate::new(
        ActivityContentChangeKind::ReplacementSnapshot,
        ActivityContentStream::ProviderToolDisplay,
        ActivityContent::new(
            OperationContent::new("private provider display").unwrap(),
            64,
        )
        .unwrap(),
    ))
    .unwrap();

    assert_eq!(observation.phase(), ActivityLifecyclePhase::Completed);
    let rendered = format!("{observation:?}");
    assert!(!rendered.contains("provider/private/item"));
    assert!(!rendered.contains("private-callback"));
    assert!(!rendered.contains("private provider display"));
    assert!(!rendered.contains("Private tool label"));
    assert_eq!(
        observation.label().map(ActivityLabel::as_str),
        Some("Private tool label")
    );
    assert!(!observation.to_string().contains("private"));

    let request = ActivityCorrelation::ProviderRequest(
        ProviderRequestRef::new("private-request").expect("request ref is valid"),
    );
    let direct = ActivityCorrelation::DirectToolCall(
        DirectToolCallId::new("private-direct-tool").expect("direct tool id is valid"),
    );
    assert!(!format!("{request:?}").contains("private-request"));
    assert!(!format!("{direct:?}").contains("private-direct-tool"));
}

#[test]
fn reasoning_summary_and_assistant_streams_are_exact() {
    let reasoning = ActivityObservation::new(
        ActivityId::new("reasoning-1").unwrap(),
        run_id(),
        ActivityKind::ReasoningSummary,
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
        None,
        ActivityDisclosure::ProviderDisplayContent,
    )
    .unwrap()
    .with_content(ActivityContentUpdate::new(
        ActivityContentChangeKind::Delta,
        ActivityContentStream::ReasoningSummaryText,
        ActivityContent::new(OperationContent::new("readable summary").unwrap(), 64).unwrap(),
    ))
    .expect("reasoning summary stream matches");
    assert!(reasoning.content().is_some());

    let assistant = ActivityObservation::new(
        ActivityId::new("message-1").unwrap(),
        run_id(),
        ActivityKind::AssistantMessage,
        ActivityLifecyclePhase::Updated,
        ActivityStatus::InProgress,
        Some(ActivityAssistantPhase::Intermediate),
        ActivityDisclosure::ProviderDisplayContent,
    )
    .unwrap();
    let wrong_stream = assistant
        .with_content(ActivityContentUpdate::new(
            ActivityContentChangeKind::Delta,
            ActivityContentStream::FinalAnswerText,
            ActivityContent::new(OperationContent::new("not final").unwrap(), 64).unwrap(),
        ))
        .expect_err("intermediate message cannot claim final-answer content");
    assert_eq!(
        wrong_stream.diagnostic().code(),
        "swallowtail.activity_record_invalid"
    );
}

#[test]
fn provider_unspecified_assistant_phase_retains_identity_without_mislabeling_content() {
    let assistant = ActivityObservation::new(
        ActivityId::new("legacy-message").unwrap(),
        run_id(),
        ActivityKind::AssistantMessage,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        Some(ActivityAssistantPhase::ProviderUnspecified),
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .expect("provider-unspecified assistant identity is representable");

    assert_eq!(
        assistant.assistant_phase(),
        Some(ActivityAssistantPhase::ProviderUnspecified)
    );
}

