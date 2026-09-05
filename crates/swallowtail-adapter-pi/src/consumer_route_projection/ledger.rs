use std::collections::BTreeSet;

struct Entry {
    route_id: &'static str,
    shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
    withheld_because: &'static str,
}

const SIDECAR: &[&str] = &["pi.sdk-sidecar prepared session"];

const PI_RPC_LEDGER: &[Entry] = &[
    Entry {
        route_id: "pi.rpc",
        shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &["PiPreparedCatalogue"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &["PiPreparedRun"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &["PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &["PiPreparedRun", "PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &["PiPreparedRun", "PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-capability",
        semantic_id: "feature.attachments",
        emitted_by: &[
            "PiPreparedRun[with_attachments]",
            "PiPreparedSession[with_image_attachments]",
        ],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-capability",
        semantic_id: "feature.question-exchange",
        emitted_by: &["PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &["PiPreparedRun", "PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &["PiPreparedRun", "PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &["PiPreparedCatalogue", "PiPreparedRun", "PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &["PiPreparedRun", "PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: &["PiPreparedRun"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: &["PiPreparedSession"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "structured-run",
        semantic_id: "control.attachments",
        emitted_by: &["PiPreparedRun[with_attachments]"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.rpc",
        shape: "interactive-session",
        semantic_id: "control.attachments",
        emitted_by: &["PiPreparedSession[with_image_attachments]"],
        withheld_because: "",
    },
];

const PI_SIDECAR_LEDGER: &[Entry] = &[
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: &["PiSdkSidecarPreparedSession[with_reasoning]"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-capability",
        semantic_id: "feature.attachments",
        emitted_by: &["PiSdkSidecarPreparedSession[with_image_attachments]"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "session-lifecycle",
        semantic_id: "feature.load-session",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "session-lifecycle",
        semantic_id: "feature.resume-session",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "interactive-session",
        semantic_id: "control.reasoning-selection",
        emitted_by: &["PiSdkSidecarPreparedSession[with_reasoning]"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "interactive-session",
        semantic_id: "control.attachments",
        emitted_by: &["PiSdkSidecarPreparedSession[with_image_attachments]"],
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "interactive-session",
        semantic_id: "control.session-options",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "session-management",
        semantic_id: "control.load-session",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
    Entry {
        route_id: "pi.sdk-sidecar",
        shape: "session-management",
        semantic_id: "control.resume-session",
        emitted_by: SIDECAR,
        withheld_because: "",
    },
];

fn assert_ledger(route_id: &str, ledger: &[Entry], emitted: usize, withheld: usize) {
    assert_eq!(ledger.len(), emitted + withheld);
    assert_eq!(
        ledger
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        emitted
    );
    assert_eq!(
        ledger
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        withheld
    );
    assert!(ledger.iter().all(|entry| entry.route_id == route_id));
    let identities = ledger
        .iter()
        .map(|entry| (entry.route_id, entry.shape, entry.semantic_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(identities.len(), ledger.len());
    assert!(
        ledger
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .all(|entry| { !entry.withheld_because.is_empty() })
    );
}

#[test]
fn pi_rpc_ledger_is_exact() {
    assert_ledger("pi.rpc", PI_RPC_LEDGER, 15, 0);
}

#[test]
fn pi_sdk_sidecar_ledger_is_exact() {
    assert_ledger("pi.sdk-sidecar", PI_SIDECAR_LEDGER, 19, 0);
    assert!(PI_SIDECAR_LEDGER.iter().any(|entry| {
        entry.semantic_id == "control.attachments" && entry.shape == "interactive-session"
    }));
}
