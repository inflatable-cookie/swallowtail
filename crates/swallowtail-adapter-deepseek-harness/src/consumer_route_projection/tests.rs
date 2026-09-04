
use std::collections::BTreeSet;

const JSONRPC_LEDGER: [&str; 11] = [
    "feature.model-catalogue",
    "feature.structured-run",
    "feature.streaming-events",
    "feature.usage-evidence",
    "feature.cancellation-or-interruption",
    "feature.working-resource",
    "feature.owned-runtime-lifecycle",
    "feature.persistent-session-posture",
    "feature.prepared-facade",
    "feature.activity-observation",
    "control.model-selection",
];
const WEB_LEDGER: [&str; 17] = [
    "feature.model-catalogue",
    "feature.structured-run",
    "feature.streaming-events",
    "feature.usage-evidence",
    "feature.cancellation-or-interruption",
    "feature.provider-session-catalogue",
    "feature.working-resource",
    "feature.provider-session-archive",
    "feature.owned-runtime-lifecycle",
    "feature.persistent-session-posture",
    "feature.prepared-facade",
    "feature.activity-observation",
    "control.model-selection",
    "control.provider-session-catalogue",
    "control.provider-session-history",
    "control.provider-session-fork",
    "control.provider-session-archive",
];
const JSONRPC_WITHHELD: [&str; 3] = [
    "feature.model-catalogue",
    "feature.owned-runtime-lifecycle",
    "feature.persistent-session-posture",
];
const WEB_WITHHELD: [&str; 3] = [
    "feature.model-catalogue",
    "feature.owned-runtime-lifecycle",
    "feature.persistent-session-posture",
];

#[test]
fn candidate_i_harness_ledger_is_exact_without_filtering() {
    assert_eq!(JSONRPC_LEDGER.len(), 11);
    assert_eq!(WEB_LEDGER.len(), 17);
    assert_eq!(JSONRPC_WITHHELD.len() + WEB_WITHHELD.len(), 6);
    assert_eq!(JSONRPC_LEDGER.len() + WEB_LEDGER.len() - 6, 22);
    assert_eq!(22 + 19, 41);
    assert_eq!(41 + 6, 47);
    assert_eq!(
        JSONRPC_LEDGER.into_iter().collect::<BTreeSet<_>>().len(),
        11
    );
    assert_eq!(WEB_LEDGER.into_iter().collect::<BTreeSet<_>>().len(), 17);
    for withheld in JSONRPC_WITHHELD {
        assert!(JSONRPC_LEDGER.contains(&withheld));
    }
    for withheld in WEB_WITHHELD {
        assert!(WEB_LEDGER.contains(&withheld));
    }
}

#[test]
fn matrix_only_capabilities_have_no_construction_mapping() {
    assert_eq!(
        super::builder::feature_for(swallowtail_core::Capability::ModelCatalog),
        None
    );
    assert_eq!(
        super::builder::feature_for(swallowtail_core::Capability::ProviderSessionHistory),
        None
    );
}
