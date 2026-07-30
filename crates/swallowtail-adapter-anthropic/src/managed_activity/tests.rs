use super::*;

#[test]
fn persisted_activity_projects_provider_tools_but_not_custom_callbacks() {
    let events = crate::managed::parse_stream(include_str!(concat!(
        "../../tests/fixtures/managed-agents-2026-04-01/",
        "activity.sse"
    )))
    .expect("managed activity fixture parses");
    let mut projection = ManagedActivityProjection::new(
        RuntimeRunId::new("managed-activity-fixture").expect("run id"),
    );
    let mut observations = Vec::new();
    for event in &events {
        observations.extend(projection.project(event).expect("activity projects"));
    }
    assert_eq!(
        observations
            .iter()
            .filter(|item| item.kind() == &ActivityKind::ProviderOwnedTool)
            .count(),
        2
    );
    assert!(
        observations
            .iter()
            .filter(|item| { item.kind() == &ActivityKind::ProviderOwnedTool })
            .all(|item| item.label().is_some() && item.content().is_none())
    );
    assert!(
        !observations
            .iter()
            .any(|item| item.kind() == &ActivityKind::ConsumerOwnedTool)
    );
    assert!(observations.iter().any(|item| {
        item.kind() == &ActivityKind::ReasoningSummary
            && item.disclosure() == ActivityDisclosure::IdentityAndLifecycleOnly
    }));
    let rendered = format!("{observations:?}");
    assert!(!rendered.contains("fixture-private-result"));
    assert!(!rendered.contains("fixture-private-input"));
}
