#[test]
fn overlay_marks_kimi_catalogue_rows_without_changing_readiness() {
    let services = prepared_services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = snapshot_record(&services, &store);

    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    assert_eq!(record.instance_id(), &instance_id());
    assert_eq!(
        record.driver_identity(),
        kimi_platform_direct_descriptor().identity()
    );
    assert_eq!(
        record.protocol_facade_id().as_str(),
        KIMI_PLATFORM_FACADE_REVISION
    );
    assert_eq!(
        record.instance_policy_id().as_str(),
        "public-platform-api-key"
    );
    assert_eq!(record.routes().len(), 2);
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("sk-"));
    assert!(!snapshot_debug.contains("fixture-secret"));

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new(KIMI_PLATFORM_PROVIDER_ID).expect("provider id is valid"),
                ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id is valid"),
            )
            .with_favourite(true)
            .with_ordinal(Some(0)),
        )
        .expect("overlay marker stores");

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the kimi catalogue");

    assert_eq!(overlay.selection_readiness(), record.selection_readiness());
    assert_eq!(overlay.instance_id(), &instance_id());
    let entries: Vec<_> = overlay.entries().collect();
    assert_eq!(entries.len(), 1);
    let entry = entries[0];
    assert_eq!(entry.model_id().as_str(), KIMI_PLATFORM_MODEL_ID);
    assert!(entry.favourite());
    assert_eq!(entry.ordinal(), Some(0));
    assert!(entry.provider_default());
    assert!(!entry.consumer_default());
    assert!(!entry.hidden());
}
