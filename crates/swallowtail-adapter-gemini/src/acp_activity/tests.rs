use super::*;
use serde_json::{Value, json};
use swallowtail_core::ActivityKindClass;

#[test]
fn exact_gemini_thought_classification_and_tools_remain_distinct() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("gemini-activity-turn").unwrap());
    let updates = [
        json!({"sessionUpdate":"agent_thought_chunk","content":{"type":"text","text":"Inspecting"}}),
        json!({"sessionUpdate":"agent_thought_chunk","messageId":"warning-1","content":{"type":"text","text":"Warning: file access failed"}}),
        json!({"sessionUpdate":"agent_message_chunk","messageId":"mode-1","content":{"type":"text","text":"[MODE_UPDATE] plan"}}),
        json!({"sessionUpdate":"tool_call","toolCallId":"tool-1","title":"Read","kind":"read","status":"in_progress","locations":[{"path":"/fixture/src/lib.rs"}]}),
        json!({"sessionUpdate":"tool_call_update","toolCallId":"tool-1","status":"completed","content":[{"type":"content","content":{"type":"text","text":"Done"}}]}),
        json!({"sessionUpdate":"future_gemini_activity","private":"excluded"}),
    ];
    let observations = project_all(&mut projection, &updates);
    assert_eq!(
        observations
            .iter()
            .map(|observation| observation.kind().class())
            .collect::<Vec<_>>(),
        [
            ActivityKindClass::ReasoningSummary,
            ActivityKindClass::WarningOrError,
            ActivityKindClass::AssistantMessage,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::Unknown,
        ]
    );
    assert_eq!(
        observations[2].assistant_phase(),
        Some(ActivityAssistantPhase::ProviderUnspecified)
    );
    assert!(observations[1].content().is_none());
    assert_eq!(
        projection
            .complete(&TerminalStatus::Completed)
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn translation_mismatch_keeps_first_portable_identity() {
    let mut projection =
        AcpActivityProjection::new(RuntimeTurnId::new("gemini-activity-turn").unwrap());
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
