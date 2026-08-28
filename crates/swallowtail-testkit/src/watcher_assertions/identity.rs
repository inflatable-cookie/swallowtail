use super::WATCHER_RULE;
use swallowtail_core::{
    MAX_WATCHER_ID_BYTES, MAX_WATCHER_OPERATION_DATA_BYTES, MAX_WATCHER_OWNING_TURN_BYTES,
    MAX_WATCHER_SUMMARY_BYTES, WatcherId, WatcherOperationData, WatcherOwningTurn,
    WatcherRequester, WatcherSummary,
};
use swallowtail_runtime::{WatcherFailureKind, WatcherRegistry};

/// Proves watcher ids and summaries stay redacted in default formatting.
pub fn assert_watcher_identity_redaction(watcher_id: &WatcherId, summary: &WatcherSummary) {
    let id_value = watcher_id.as_str();
    let summary_value = summary.as_str();
    assert!(
        !format!("{watcher_id:?}").contains(id_value),
        "{WATCHER_RULE}: WatcherId debug exposed its value"
    );
    assert!(
        !format!("{watcher_id}").contains(id_value),
        "{WATCHER_RULE}: WatcherId display exposed its value"
    );
    assert!(
        !format!("{summary:?}").contains(summary_value),
        "{WATCHER_RULE}: WatcherSummary debug exposed its value"
    );
    assert!(
        !format!("{summary}").contains(summary_value),
        "{WATCHER_RULE}: WatcherSummary display exposed its value"
    );
}

/// Proves exact and overflow UTF-8 byte bounds for watcher public identities.
pub fn assert_watcher_byte_bounds() {
    let exact_id = "a".repeat(MAX_WATCHER_ID_BYTES);
    WatcherId::new(exact_id.clone()).expect("{WATCHER_RULE}: exact id bound must accept");
    assert!(
        WatcherId::new(format!("{exact_id}a")).is_err(),
        "{WATCHER_RULE}: id overflow must reject"
    );

    let exact_turn = "t".repeat(MAX_WATCHER_OWNING_TURN_BYTES);
    WatcherOwningTurn::new(exact_turn.clone())
        .expect("{WATCHER_RULE}: exact owning-turn bound must accept");
    assert!(
        WatcherOwningTurn::new(format!("{exact_turn}x")).is_err(),
        "{WATCHER_RULE}: owning-turn overflow must reject"
    );

    let exact_summary = "s".repeat(MAX_WATCHER_SUMMARY_BYTES);
    WatcherSummary::new(exact_summary.clone())
        .expect("{WATCHER_RULE}: exact summary bound must accept");
    assert!(
        WatcherSummary::new(format!("{exact_summary}!")).is_err(),
        "{WATCHER_RULE}: summary overflow must reject"
    );

    let exact_operation = "o".repeat(MAX_WATCHER_OPERATION_DATA_BYTES);
    WatcherOperationData::new(exact_operation.clone())
        .expect("{WATCHER_RULE}: exact operation-data bound must accept");
    assert!(
        WatcherOperationData::new(format!("{exact_operation}!")).is_err(),
        "{WATCHER_RULE}: operation-data overflow must reject"
    );

    // Multi-byte UTF-8: two bytes each, so length-by-chars can fit while bytes overflow.
    let utf8_pair = "é"; // 2 bytes
    assert_eq!(utf8_pair.len(), 2);
    let utf8_id = utf8_pair.repeat(MAX_WATCHER_ID_BYTES / 2);
    assert_eq!(utf8_id.len(), MAX_WATCHER_ID_BYTES);
    WatcherId::new(utf8_id.clone()).expect("{WATCHER_RULE}: exact UTF-8 byte bound must accept");
    assert!(
        WatcherId::new(format!("{utf8_id}é")).is_err(),
        "{WATCHER_RULE}: UTF-8 byte overflow must reject"
    );

    let utf8_operation = utf8_pair.repeat(MAX_WATCHER_OPERATION_DATA_BYTES / 2);
    WatcherOperationData::new(utf8_operation.clone())
        .expect("{WATCHER_RULE}: exact UTF-8 operation-data bound must accept");
    assert!(
        WatcherOperationData::new(format!("{utf8_operation}é")).is_err(),
        "{WATCHER_RULE}: UTF-8 operation-data overflow must reject"
    );
}

/// Proves operation data stays redacted and cannot be mistaken for a summary.
pub fn assert_watcher_operation_data_redaction() {
    let data = WatcherOperationData::new("private-operation-data").expect("operation data");
    assert!(
        !format!("{data:?}").contains(data.as_str()),
        "{WATCHER_RULE}: operation data debug exposed its value"
    );
    assert!(
        !format!("{data}").contains(data.as_str()),
        "{WATCHER_RULE}: operation data display exposed its value"
    );
}

/// Proves foreign and unknown watcher identities fail closed.
pub fn assert_watcher_ownership_rejection(registry: &WatcherRegistry) {
    let foreign = WatcherOwningTurn::new("foreign-turn").expect("foreign turn is valid");
    let unknown = WatcherId::new("missing-watcher").expect("watcher id is valid");
    let failure = registry
        .inspect(&foreign, &unknown)
        .expect_err("{WATCHER_RULE}: foreign turn must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::ForeignIdentity);

    let owned = registry.owning_turn().clone();
    let failure = registry
        .inspect(&owned, &unknown)
        .expect_err("{WATCHER_RULE}: unknown watcher must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::UnknownWatcher);
}

/// Proves capacity bounds reject additional accepted starts.
pub fn assert_watcher_capacity_bound(mut registry: WatcherRegistry) {
    let capacity = registry.maximum_watchers();
    for _ in 0..capacity {
        registry
            .accept_start(
                WatcherRequester::Model,
                WatcherOperationData::new("capacity-operation").expect("operation data"),
            )
            .expect("{WATCHER_RULE}: accepted start within bound");
    }
    let failure = registry
        .accept_start(
            WatcherRequester::Operator,
            WatcherOperationData::new("overflow-operation").expect("operation data"),
        )
        .expect_err("{WATCHER_RULE}: over-capacity start must fail");
    assert_eq!(failure.kind(), WatcherFailureKind::CapacityExceeded);
}
