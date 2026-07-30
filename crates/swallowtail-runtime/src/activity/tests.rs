use super::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentStream,
    ActivityContentUpdate, ActivityCorrelation, ActivityDisclosure, ActivityId, ActivityKind,
    ActivityLabel, ActivityLifecyclePhase, ActivityNamespace, ActivityObservation,
    ActivityOperationId, ActivityStatus,
};
use crate::{
    CallbackId, DirectToolCallId, EventDelivery, OperationContent, RuntimeEventKind, RuntimeRunId,
};
use swallowtail_core::{ProviderActivityRef, ProviderRequestRef};

fn run_id() -> ActivityOperationId {
    ActivityOperationId::Run(RuntimeRunId::new("private-run").expect("run id is valid"))
}

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

#[test]
fn malformed_phase_disclosure_and_assistant_claims_fail() {
    assert!(
        ActivityObservation::new(
            ActivityId::new("bad-status").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .is_err()
    );
    assert!(
        ActivityObservation::new(
            ActivityId::new("bad-assistant").unwrap(),
            run_id(),
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
            ActivityDisclosure::ProviderDisplayContent,
        )
        .is_err()
    );
    assert!(
        ActivityObservation::new(
            ActivityId::new("unavailable").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::Unavailable,
        )
        .is_err()
    );
    assert!(
        ActivityObservation::new(
            ActivityId::new("identity-only-label").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .unwrap()
        .with_label(ActivityLabel::new("Not disclosed").unwrap())
        .is_err()
    );
}

#[test]
fn activity_events_are_always_semantic() {
    let observation = ActivityObservation::new(
        ActivityId::new("task-1").unwrap(),
        run_id(),
        ActivityKind::Task,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .unwrap();

    assert_eq!(
        RuntimeEventKind::Activity(observation).delivery(),
        EventDelivery::Semantic
    );
}
