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
