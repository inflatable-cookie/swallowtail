#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services(), &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "kimi-platform");
    assert_eq!(record.route_id().as_str(), "kimi-platform.chat");
    assert_eq!(record.topology(), swallowtail_core::RouteTopology::Hosted);
    assert_eq!(record.credential_refs().len(), 0);
    assert!(
        store
            .get_instance(&instance_id())
            .expect("store read succeeds")
            .is_some()
    );
}

#[test]
fn api_key_collection_completes_without_browser_ports() {
    let services = services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let descriptor = kimi_platform_chat_addable_route_descriptor(&services);
    let fields: Vec<_> = descriptor.credential_fields().cloned().collect();

    let mut session = start_sign_in(
        &services,
        SignInStartRequest::new(
            ScopeId::new("kimi-platform.admission.sign-in:work").expect("scope is valid"),
            admitted.id().clone(),
            family(),
            admitted.route_id().clone(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                EndpointAudience::new(KIMI_PLATFORM_ENDPOINT_AUDIENCE).expect("audience is valid"),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::ApiKeyCollection { fields },
        ),
    )
    .expect("API-key collection starts without browser ports");
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("poll succeeds"),
        SignInStatus::InProgress
    );

    let status = submit_sign_in_credential_field(
        &mut session,
        swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
            .expect("field id is valid"),
        CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
    )
    .expect("field submit succeeds");
    assert_eq!(status, SignInStatus::ReadyToComplete);

    let outcome =
        complete_sign_in(session, &services, Some(&store)).expect("sign-in loop completes");
    assert_eq!(outcome.instance_id(), &instance_id());
    let refs: Vec<_> = outcome.credential_refs().collect();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].1.as_host_value(), CREDENTIAL_REF);

    let stored = store
        .get_instance(&instance_id())
        .expect("store read succeeds")
        .expect("instance is stored");
    let stored_refs: Vec<_> = stored.credential_refs().collect();
    assert_eq!(stored_refs.len(), 1);
    assert_eq!(stored_refs[0].1.as_host_value(), CREDENTIAL_REF);
    let debug = format!("{stored:?}");
    assert!(debug.contains("CredentialRef(\"<opaque>\")"));
    assert!(!debug.contains("sk-"));
}
