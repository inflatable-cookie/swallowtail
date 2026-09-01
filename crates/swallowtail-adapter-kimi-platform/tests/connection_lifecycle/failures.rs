#[test]
fn preparation_fails_closed_on_admission_drift() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);

    let wrong_route = swallowtail_core::AdmittedInstanceRecord::new(
        instance_id(),
        family(),
        swallowtail_core::AddableRouteId::new("kimi-platform.other").expect("route id is valid"),
        swallowtail_adapter_kimi_platform::kimi_platform_direct_descriptor()
            .identity()
            .clone(),
        swallowtail_core::RouteTopology::Hosted,
    );
    let route_mismatch = KimiPlatformPreparationInput::from_admitted(
        &wrong_route,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        evidence.clone(),
    )
    .expect_err("another route fails closed");
    assert_eq!(
        route_mismatch.stage(),
        swallowtail_runtime::PreparationStage::TargetSelection
    );

    let no_refs = swallowtail_core::AdmittedInstanceRecord::new(
        instance_id(),
        family(),
        swallowtail_core::AddableRouteId::new("kimi-platform.chat").expect("route id is valid"),
        swallowtail_adapter_kimi_platform::kimi_platform_direct_descriptor()
            .identity()
            .clone(),
        swallowtail_core::RouteTopology::Hosted,
    );
    let endpoint_missing = KimiPlatformPreparationInput::from_admitted(
        &no_refs,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        evidence.clone(),
    )
    .expect_err("missing endpoint ref fails closed");
    assert_eq!(
        endpoint_missing.stage(),
        swallowtail_runtime::PreparationStage::TargetSelection
    );

    let missing_refs = KimiPlatformPreparationInput::from_admitted(
        &admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        evidence.clone(),
    )
    .expect_err("missing credential ref fails closed");
    assert_eq!(
        missing_refs.stage(),
        swallowtail_runtime::PreparationStage::AccessEvidence
    );

    let mismatched_credential = KimiPlatformPreparationInput::from_admitted(
        &admitted.with_credential_refs([(
            swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
                .expect("credential id is valid"),
            CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
        )]),
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        access_profile(CredentialRef::new("kimi-platform.work.other-key").expect("ref is valid")),
        evidence,
    )
    .expect_err("mismatched credential ref fails closed");
    assert_eq!(
        mismatched_credential.stage(),
        swallowtail_runtime::PreparationStage::AccessEvidence
    );
}
