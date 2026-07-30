use super::*;
use serde_json::{Value, json};
use swallowtail_core::ActivityKindClass;

#[test]
fn exact_kimi_lazy_tools_and_plan_replacements_are_projected() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("kimi-activity-turn").unwrap());
    let updates = [
        json!({"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"Answer"}}),
        json!({"sessionUpdate":"agent_thought_chunk","content":{"type":"text","text":"Inspecting"}}),
        json!({"sessionUpdate":"tool_call","toolCallId":"7:tool-1","title":"Read","kind":"read","status":"pending","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"7:tool-1","status":"in_progress","rawInput":{"secret":"excluded"},"content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"7:tool-1","status":"completed","rawOutput":{"secret":"excluded"},"content":[{"type":"content","content":{"type":"text","text":"Done"}}]}),
        json!({"sessionUpdate":"plan","entries":[{"content":"Inspect","priority":"medium","status":"completed"}]}),
        json!({"sessionUpdate":"future_kimi_activity","private":"excluded"}),
    ];
    let observations = project_all(&mut projection, &updates);
    assert_eq!(
        observations
            .iter()
            .map(|observation| observation.kind().class())
            .collect::<Vec<_>>(),
        [
            ActivityKindClass::AssistantMessage,
            ActivityKindClass::ReasoningSummary,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::Plan,
            ActivityKindClass::Unknown,
        ]
    );
    assert_eq!(observations[2].phase(), ActivityLifecyclePhase::Started);
    assert_eq!(observations[4].phase(), ActivityLifecyclePhase::Completed);
    let task = observations[5]
        .task_list()
        .expect("ACP plan carries a typed task-list replacement")
        .items()
        .next()
        .unwrap();
    assert_eq!(task.content().as_str(), "Inspect");
    assert_eq!(task.status(), TaskListItemStatus::Completed);
    assert_eq!(task.priority(), Some(TaskListItemPriority::Medium));
    assert_eq!(
        projection
            .complete(&TerminalStatus::Completed)
            .unwrap()
            .len(),
        3
    );
    assert!(!format!("{observations:?}").contains("excluded"));
}

#[test]
fn translation_mismatch_keeps_first_portable_identity() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("kimi-activity-turn").unwrap());
    let original = projection
        .open_or_insert(
            "translation-mismatch",
            None,
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityStatus::InProgress,
        )
        .unwrap();
    let reconciled = projection
        .open_or_insert(
            "translation-mismatch",
            None,
            ActivityKind::AssistantMessage,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::IdentityAndLifecycleOnly,
            ActivityStatus::InProgress,
        )
        .unwrap();

    assert_eq!(reconciled.id, original.id);
    assert_eq!(reconciled.kind, ActivityKind::ProviderOwnedTool);
    assert_eq!(reconciled.assistant_phase, None);
    assert_eq!(
        reconciled.disclosure,
        ActivityDisclosure::ProviderDisplayContent
    );
}

#[test]
fn noncanonical_tool_lifecycles_degrade_without_failure() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("kimi-tool-lifecycle-turn").unwrap());
    let updates = [
        json!({"sessionUpdate":"tool_call","toolCallId":"direct","title":"Finished","kind":"other","status":"completed","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"orphan","title":"Suppressed","status":"completed","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"dropped","status":"in_progress"}),
        json!({"sessionUpdate":"tool_call","toolCallId":"repeated","title":"Placeholder","kind":"other","status":"pending","content":[]}),
        json!({"sessionUpdate":"tool_call","toolCallId":"repeated","title":"Read","kind":"read","status":"in_progress","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"repeated","status":"completed","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"repeated","status":"completed","content":[]}),
    ];

    let observations = project_all(&mut projection, &updates);
    assert_eq!(
        observations
            .iter()
            .map(ActivityObservation::phase)
            .collect::<Vec<_>>(),
        [
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Completed,
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Completed,
            ActivityLifecyclePhase::Started,
            ActivityLifecyclePhase::Updated,
            ActivityLifecyclePhase::Completed,
        ]
    );
    assert_eq!(observations[0].status(), ActivityStatus::InProgress);
    assert_eq!(observations[1].status(), ActivityStatus::Completed);
    assert_eq!(observations[0].activity_id(), observations[1].activity_id());
    assert_eq!(observations[2].activity_id(), observations[3].activity_id());
    assert_eq!(observations[4].activity_id(), observations[5].activity_id());
    assert_eq!(observations[5].activity_id(), observations[6].activity_id());
}

fn project_all(
    projection: &mut AcpActivityProjection,
    updates: &[Value],
) -> Vec<ActivityObservation> {
    updates
        .iter()
        .flat_map(|update| {
            let decoded = swallowtail_protocol_acp::decode_session_update(
                &json!({"sessionId":"fixture-session","update":update}),
            )
            .unwrap();
            projection.project(&decoded.update).unwrap()
        })
        .collect()
}
