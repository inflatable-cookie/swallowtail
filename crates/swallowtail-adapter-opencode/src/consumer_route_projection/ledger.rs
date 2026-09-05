use std::collections::BTreeSet;

struct Entry {
    shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const RUN: &[&str] = &["OpenCodePreparedRun"];
const SESSION: &[&str] = &["OpenCodePreparedSession"];
const RUN_SESSION: &[&str] = &["OpenCodePreparedRun", "OpenCodePreparedSession"];
const CALLBACK_RUN_SESSION: &[&str] = &[
    "OpenCodePreparedRun[with_provider_callbacks]",
    "OpenCodePreparedSession[with_provider_callbacks]",
];

const LEDGER: &[Entry] = &[
    Entry {
        shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &["OpenCodePreparedCatalogue"],
        withheld_because: "",
    },
    Entry {
        shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.structured-output",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.attachments",
        emitted_by: &[
            "OpenCodePreparedRun[with_attachments]",
            "OpenCodePreparedSession[with_image_attachments]",
        ],
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.permission-exchange",
        emitted_by: CALLBACK_RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.question-exchange",
        emitted_by: CALLBACK_RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "session-lifecycle",
        semantic_id: "feature.load-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "session-lifecycle",
        semantic_id: "feature.resume-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "session-lifecycle",
        semantic_id: "feature.provider-session-catalogue",
        emitted_by: &["OpenCodePreparedSessionCatalogue"],
        withheld_because: "",
    },
    Entry {
        shape: "session-lifecycle",
        semantic_id: "feature.provider-session-import",
        emitted_by: &["OpenCodePreparedSessionImport"],
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "session-lifecycle",
        semantic_id: "feature.provider-session-delete",
        emitted_by: &["OpenCodePreparedDelete"],
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.owned-remote-resource-cleanup",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[
            "OpenCodePreparedCatalogue",
            "OpenCodePreparedRun",
            "OpenCodePreparedSession",
            "OpenCodePreparedSessionCatalogue",
            "OpenCodePreparedSessionImport",
            "OpenCodePreparedDelete",
        ],
        withheld_because: "",
    },
    Entry {
        shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: RUN_SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "structured-run",
        semantic_id: "control.reasoning-selection",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "interactive-session",
        semantic_id: "control.reasoning-selection",
        emitted_by: &[],
        withheld_because: "matrix-descriptor-only; no retained interactive-session owner",
    },
    Entry {
        shape: "structured-run",
        semantic_id: "control.structured-output",
        emitted_by: RUN,
        withheld_because: "",
    },
    Entry {
        shape: "structured-run",
        semantic_id: "control.attachments",
        emitted_by: &["OpenCodePreparedRun[with_attachments]"],
        withheld_because: "",
    },
    Entry {
        shape: "interactive-session",
        semantic_id: "control.attachments",
        emitted_by: &["OpenCodePreparedSession[with_image_attachments]"],
        withheld_because: "",
    },
    Entry {
        shape: "structured-run",
        semantic_id: "control.provider-callbacks",
        emitted_by: &["OpenCodePreparedRun[with_provider_callbacks]"],
        withheld_because: "",
    },
    Entry {
        shape: "interactive-session",
        semantic_id: "control.provider-callbacks",
        emitted_by: &["OpenCodePreparedSession[with_provider_callbacks]"],
        withheld_because: "",
    },
    Entry {
        shape: "interactive-session",
        semantic_id: "control.active-turn-detachment",
        emitted_by: &["OpenCodePreparedSession[with_active_turn_detachment]"],
        withheld_because: "",
    },
    Entry {
        shape: "session-management",
        semantic_id: "control.load-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "session-management",
        semantic_id: "control.resume-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    Entry {
        shape: "session-management",
        semantic_id: "control.provider-session-catalogue",
        emitted_by: &["OpenCodePreparedSessionCatalogue"],
        withheld_because: "",
    },
    Entry {
        shape: "per-turn",
        semantic_id: "control.provider-turn-reference",
        emitted_by: &[],
        withheld_because: "matrix-descriptor-only; reconciliation rejects provider turn references",
    },
];

#[test]
fn opencode_ledger_is_exact_and_has_no_exception_list() {
    assert_eq!(LEDGER.len(), 35);
    assert_eq!(
        LEDGER
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        33
    );
    assert_eq!(
        LEDGER
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        2
    );
    let identities = LEDGER
        .iter()
        .map(|entry| (entry.shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(identities.len(), LEDGER.len());
    assert!(
        LEDGER
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .all(|entry| { entry.withheld_because.contains("matrix-descriptor-only") })
    );
}

#[test]
fn opencode_ledger_records_all_per_turn_authority_as_consumer_mediated() {
    let per_turn = LEDGER
        .iter()
        .filter(|entry| entry.shape == "per-turn")
        .collect::<Vec<_>>();
    assert_eq!(per_turn.len(), 1);
    assert!(per_turn[0].emitted_by.is_empty());
    assert!(
        per_turn[0]
            .withheld_because
            .contains("matrix-descriptor-only")
    );
    assert!(
        LEDGER
            .iter()
            .any(|entry| entry.semantic_id == "control.attachments"
                && entry.shape == "interactive-session")
    );
    assert!(
        LEDGER
            .iter()
            .any(|entry| entry.semantic_id == "control.provider-callbacks"
                && entry.shape == "structured-run")
    );
    assert!(
        LEDGER
            .iter()
            .any(|entry| entry.semantic_id == "control.provider-callbacks"
                && entry.shape == "interactive-session")
    );
}
