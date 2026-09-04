use std::collections::BTreeSet;

type Row = (&'static str, &'static str, bool);

const ACP: [Row; 25] = [
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

const HEADLESS: [Row; 20] = [
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

const LOCAL: [Row; 31] = [
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
