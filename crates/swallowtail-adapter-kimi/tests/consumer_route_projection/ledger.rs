use std::collections::BTreeSet;
use swallowtail_runtime::{ConsumerRouteProjectionContribution, ConsumerRouteRowIdentity};

pub(super) fn observed_tuples(
    route: &str,
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<(String, String, String)> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| {
            let semantic = row.identity().namespaced_extension().map_or_else(
                || match row.identity() {
                    ConsumerRouteRowIdentity::Feature(feature) => {
                        let name = format!("{feature:?}");
                        if name == "ActiveSessionReasoningAcknowledgement" {
                            "feature.active-session-reasoning-and-plan-ack".to_owned()
                        } else {
                            format!("feature.{}", kebab(&name))
                        }
                    }
                    ConsumerRouteRowIdentity::Control(control) => {
                        format!("control.{}", kebab(&format!("{control:?}")))
                    }
                },
                |extension| extension.semantic_id().to_owned(),
            );
            let shape = match semantic.as_str() {
                "feature.model-catalogue" => "model-catalogue",
                "feature.structured-run" => "structured-run",
                "feature.interactive-session"
                | "feature.active-session-reasoning-and-plan-ack"
                | "feature.negotiated-model-options-observation" => "interactive-session",
                "feature.streaming-events"
                | "feature.usage-evidence"
                | "feature.activity-observation" => "route-observation",
                "feature.load-session"
                | "feature.resume-session"
                | "feature.provider-session-catalogue"
                | "feature.provider-session-import"
                | "feature.provider-managed-recovery"
                | "feature.persistent-session-posture"
                | "feature.stream-reattachment"
                | "feature.provider-session-archive"
                | "feature.provider-session-restore" => "session-lifecycle",
                "control.load-session"
                | "control.resume-session"
                | "control.provider-session-catalogue"
                | "control.provider-session-import" => "session-management",
                value if value.starts_with("control.") => match contribution
                    .applicability()
                    .operation_shape()
                {
                    swallowtail_core::OperationShape::StructuredRun => "structured-run",
                    swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                    other => panic!("unexpected Kimi control shape {other:?}"),
                },
                value if value.starts_with("feature.") => "route-capability",
                _ => panic!("unexpected Kimi semantic {semantic}"),
            };
            (route.to_owned(), shape.to_owned(), semantic)
        })
        .collect()
}

fn kebab(value: &str) -> String {
    let mut result = String::new();
    for (index, character) in value.chars().enumerate() {
        if character.is_uppercase() && index != 0 {
            result.push('-');
        }
        result.extend(character.to_lowercase());
    }
    result
}

pub(super) type Row = (&'static str, &'static str, bool);

pub(super) const ACP: [Row; 25] = [
    ("model-catalogue", "feature.model-catalogue", false),
    ("structured-run", "feature.structured-run", false),
    ("interactive-session", "feature.interactive-session", true),
    ("route-observation", "feature.streaming-events", true),
    ("route-capability", "feature.reasoning-selection", true),
    (
        "route-capability",
        "feature.cancellation-or-interruption",
        true,
    ),
    ("session-lifecycle", "feature.load-session", true),
    ("session-lifecycle", "feature.resume-session", true),
    (
        "session-lifecycle",
        "feature.provider-session-catalogue",
        true,
    ),
    ("session-lifecycle", "feature.provider-session-import", true),
    ("route-capability", "feature.working-resource", true),
    (
        "route-capability",
        "feature.bounded-workspace-text-write",
        true,
    ),
    (
        "session-lifecycle",
        "feature.provider-managed-recovery",
        false,
    ),
    (
        "session-lifecycle",
        "feature.persistent-session-posture",
        true,
    ),
    ("route-capability", "feature.prepared-facade", true),
    ("route-observation", "feature.activity-observation", true),
    (
        "interactive-session",
        "feature.active-session-reasoning-and-plan-ack",
        true,
    ),
    (
        "interactive-session",
        "feature.negotiated-model-options-observation",
        true,
    ),
    ("interactive-session", "control.model-selection", true),
    ("interactive-session", "control.reasoning-selection", true),
    ("interactive-session", "control.session-options", true),
    ("session-management", "control.load-session", true),
    ("session-management", "control.resume-session", true),
    (
        "session-management",
        "control.provider-session-catalogue",
        true,
    ),
    (
        "session-management",
        "control.provider-session-import",
        true,
    ),
];

pub(super) const HEADLESS: [Row; 20] = [
    ("model-catalogue", "feature.model-catalogue", false),
    ("structured-run", "feature.structured-run", true),
    ("interactive-session", "feature.interactive-session", false),
    ("route-observation", "feature.streaming-events", true),
    ("route-capability", "feature.reasoning-selection", false),
    (
        "route-capability",
        "feature.cancellation-or-interruption",
        true,
    ),
    ("session-lifecycle", "feature.load-session", false),
    ("session-lifecycle", "feature.resume-session", false),
    (
        "session-lifecycle",
        "feature.provider-session-catalogue",
        false,
    ),
    (
        "session-lifecycle",
        "feature.provider-session-import",
        false,
    ),
    ("route-capability", "feature.working-resource", true),
    (
        "route-capability",
        "feature.bounded-workspace-text-write",
        false,
    ),
    (
        "session-lifecycle",
        "feature.provider-managed-recovery",
        true,
    ),
    (
        "session-lifecycle",
        "feature.persistent-session-posture",
        true,
    ),
    ("route-capability", "feature.prepared-facade", true),
    ("route-observation", "feature.activity-observation", true),
    ("structured-run", "control.model-selection", true),
    ("session-management", "control.load-session", false),
    ("session-management", "control.resume-session", false),
    ("structured-run", "control.provider-managed-recovery", true),
];

pub(super) const LOCAL: [Row; 31] = [
    ("model-catalogue", "feature.model-catalogue", true),
    ("structured-run", "feature.structured-run", true),
    ("interactive-session", "feature.interactive-session", true),
    ("route-observation", "feature.streaming-events", true),
    ("route-capability", "feature.reasoning-selection", true),
    ("route-capability", "feature.permission-exchange", true),
    ("route-capability", "feature.question-exchange", true),
    (
        "route-capability",
        "feature.cancellation-or-interruption",
        true,
    ),
    ("session-lifecycle", "feature.resume-session", true),
    ("route-capability", "feature.working-resource", true),
    ("session-lifecycle", "feature.stream-reattachment", true),
    (
        "session-lifecycle",
        "feature.provider-managed-recovery",
        true,
    ),
    (
        "session-lifecycle",
        "feature.provider-session-archive",
        true,
    ),
    (
        "session-lifecycle",
        "feature.provider-session-restore",
        true,
    ),
    ("route-capability", "feature.owned-runtime-lifecycle", true),
    (
        "session-lifecycle",
        "feature.persistent-session-posture",
        true,
    ),
    ("route-capability", "feature.prepared-facade", true),
    ("route-observation", "feature.activity-observation", true),
    ("structured-run", "control.model-selection", true),
    ("interactive-session", "control.model-selection", true),
    ("structured-run", "control.reasoning-selection", true),
    ("interactive-session", "control.reasoning-selection", true),
    ("structured-run", "control.managed-recovery", true),
    ("structured-run", "control.stream-reattachment", true),
    ("structured-run", "control.permission-mode", true),
    ("interactive-session", "control.permission-mode", true),
    ("structured-run", "control.provider-profile", true),
    ("interactive-session", "control.provider-profile", true),
    ("structured-run", "control.disabled-tools", true),
    ("interactive-session", "control.disabled-tools", true),
    (
        "interactive-session",
        "control.active-turn-detachment",
        true,
    ),
];

const PLATFORM: [Row; 13] = [
    ("model-catalogue", "feature.model-catalogue", true),
    ("structured-run", "feature.structured-run", true),
    ("route-observation", "feature.streaming-events", true),
    ("route-observation", "feature.usage-evidence", true),
    ("route-capability", "feature.output-token-limit", true),
    ("route-capability", "feature.reasoning-selection", true),
    (
        "route-capability",
        "feature.cancellation-or-interruption",
        false,
    ),
    ("route-capability", "feature.prepared-facade", true),
    ("route-observation", "feature.activity-observation", true),
    ("structured-run", "control.model-selection", true),
    ("structured-run", "control.reasoning-selection", true),
    ("structured-run", "control.maximum-output-tokens", true),
    (
        "structured-run",
        "control.reasoning-and-output-required",
        true,
    ),
];

#[test]
fn four_route_ledgers_are_duplicate_free_and_reconcile_to_75_of_89() {
    let routes = [
        ("kimi-code.acp", ACP.as_slice(), 22),
        ("kimi-code.headless", HEADLESS.as_slice(), 10),
        ("kimi-code.local-server", LOCAL.as_slice(), 31),
        ("kimi-platform.chat", PLATFORM.as_slice(), 12),
    ];
    let mut tuples = BTreeSet::new();
    let mut emitted = 0;
    for (route, rows, expected) in routes {
        assert_eq!(rows.iter().filter(|row| row.2).count(), expected);
        for (operation, semantic, disposition) in rows {
            assert!(tuples.insert((route, operation, semantic)));
            emitted += usize::from(*disposition);
        }
    }
    assert_eq!(tuples.len(), 89);
    assert_eq!(emitted, 75);
    assert_eq!(tuples.len() - emitted, 14);
}
