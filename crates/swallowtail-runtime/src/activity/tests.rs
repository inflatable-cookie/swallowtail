use super::{
    ActivityActor, ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind,
    ActivityContentStream, ActivityContentUpdate, ActivityCorrelation, ActivityDisclosure,
    ActivityId, ActivityKey, ActivityKind, ActivityLabel, ActivityLifecyclePhase,
    ActivityNamespace, ActivityObservation, ActivityOperationId, ActivityStatus,
    SubagentControlActionKind, SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus,
    TaskListItem, TaskListItemPriority, TaskListItemStatus, TaskListSnapshot,
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
