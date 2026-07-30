use super::*;
use serde_json::{Value, json};
use swallowtail_core::ActivityKindClass;

#[test]
fn exact_acp_activity_maps_without_raw_payloads() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("claude-activity-turn").unwrap());
    let updates = [
        json!({"sessionUpdate":"agent_message_chunk","messageId":"message-1","content":{"type":"text","text":"Answer"}}),
        json!({"sessionUpdate":"agent_thought_chunk","messageId":"thought-1","content":{"type":"text","text":"Inspecting"}}),
        json!({"sessionUpdate":"plan","entries":[{"content":"Inspect","priority":"high","status":"in_progress"}]}),
        json!({"sessionUpdate":"tool_call","toolCallId":"tool-1","title":"Tool","kind":"other","status":"in_progress","rawInput":{"secret":"excluded"}}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"tool-1","title":"Read","kind":"read","status":"completed","rawOutput":{"secret":"excluded"}}),
        json!({"sessionUpdate":"future_claude_activity","private":"excluded"}),
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
            ActivityKindClass::Plan,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::Unknown,
        ]
    );
    assert_eq!(observations[3].phase(), ActivityLifecyclePhase::Started);
    assert_eq!(observations[4].phase(), ActivityLifecyclePhase::Completed);
    let task = observations[2]
        .task_list()
        .expect("ACP plan carries a typed task-list replacement")
        .items()
        .next()
        .unwrap();
    assert_eq!(task.content().as_str(), "Inspect");
    assert_eq!(task.status(), TaskListItemStatus::InProgress);
    assert_eq!(task.priority(), Some(TaskListItemPriority::High));
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
        AcpActivityProjection::new(RuntimeTurnId::new("claude-activity-turn").unwrap());
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
        AcpActivityProjection::new(RuntimeTurnId::new("claude-tool-lifecycle-turn").unwrap());
    let updates = [
        json!({"sessionUpdate":"tool_call","toolCallId":"direct","title":"memory_recall","kind":"other","status":"completed","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"orphan","title":"TodoWrite","status":"completed","content":[]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"dropped","status":"in_progress"}),
        json!({"sessionUpdate":"tool_call","toolCallId":"repeated","title":"Permission check","kind":"other","status":"pending","content":[]}),
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

#[test]
fn oversized_tool_display_is_truncated_without_failing_projection() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("claude-large-tool-turn").unwrap());
    let file_content = "é".repeat(50_000);
    let updates = [
        json!({"sessionUpdate":"tool_call","toolCallId":"large-read","title":"Read","kind":"read","status":"in_progress","content":[]}),
        json!({
            "sessionUpdate": "tool_call_update",
            "toolCallId": "large-read",
            "title": "Read",
            "status": "completed",
            "content": [{
                "type": "content",
                "content": {"type": "text", "text": file_content}
            }],
            "rawOutput": file_content
        }),
    ];

    let observations = project_all(&mut projection, &updates);
    let projected = observations[1]
        .content()
        .expect("completed read retains bounded display content")
        .content();
    assert_eq!(projected.byte_len(), 64 * 1024);
}

#[test]
fn tool_title_refines_and_survives_payload_only_completion() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("claude-tool-label-turn").unwrap());
    let updates = [
        json!({
            "sessionUpdate": "tool_call",
            "toolCallId": "read-1",
            "title": "Read File",
            "kind": "read",
            "status": "in_progress",
            "content": []
        }),
        json!({
            "sessionUpdate": "tool_call_update",
            "toolCallId": "read-1",
            "title": "Read screens/PaperCourseOverview.svelte",
            "status": "in_progress"
        }),
        json!({
            "sessionUpdate": "tool_call_update",
            "toolCallId": "read-1",
            "status": "completed",
            "content": [{
                "type": "content",
                "content": {"type": "text", "text": "```\n1\\t<script>\n```"}
            }]
        }),
    ];

    let observations = project_all(&mut projection, &updates);
    assert_eq!(observations.len(), 3);
    assert_eq!(
        observations
            .iter()
            .map(|observation| observation.label().map(ActivityLabel::as_str))
            .collect::<Vec<_>>(),
        [
            Some("Read File"),
            Some("Read screens/PaperCourseOverview.svelte"),
            Some("Read screens/PaperCourseOverview.svelte"),
        ]
    );
    assert!(observations[0].content().is_none());
    assert!(observations[1].content().is_none());
    assert_eq!(
        observations[2]
            .content()
            .expect("completion carries tool payload")
            .content()
            .as_str(),
        "```\n1\\t<script>\n```"
    );
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
