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
