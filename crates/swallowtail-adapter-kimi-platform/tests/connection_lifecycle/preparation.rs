#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store).with_credential_refs([(
        swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
            .expect("credential id is valid"),
        CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
    )]);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);

    let prepared = prepare_kimi_platform_direct(
        KimiPlatformPreparationInput::from_admitted(
            &admitted,
            InstanceRevision::new("1").expect("revision is valid"),
            host_id(),
            profile.clone(),
            evidence,
        )
        .expect("admitted fields produce preparation input"),
        &services,
    )
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
}
