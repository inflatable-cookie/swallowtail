fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("kimi-platform.admission.host").expect("host id is valid")
}

fn services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("kimi-platform").expect("family id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> swallowtail_core::AdmittedInstanceRecord {
    let descriptor = kimi_platform_chat_addable_route_descriptor(services);
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), descriptor_route_id(&catalog))
            .with_config_refs([(
                ConfigFieldId::new(KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("kimi-platform.work.endpoint").expect("config ref is valid"),
            )]),
    )
    .expect("admission succeeds")
}

fn descriptor_route_id(catalog: &AddableRouteCatalog) -> swallowtail_core::AddableRouteId {
    catalog
        .routes()
        .next()
        .expect("catalog has the kimi-platform route")
        .id()
        .clone()
}

fn access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("kimi-platform.work.access").expect("access id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new(KIMI_PLATFORM_ENDPOINT_AUDIENCE).expect("audience is valid"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

fn ready_evidence(profile: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        profile.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

fn prepared_services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    let thread = Arc::new(ThreadServices::new());
    HostServices::new(host_id())
        .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
        .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
        .with_time(thread as Arc<dyn TimeService>)
        .with_network(Arc::new(host.clone()) as Arc<dyn NetworkPolicyService>)
        .with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn ready_access_status(profile: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        profile.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn k3_model_selection() -> KimiPlatformModelSelection {
    KimiPlatformModelSelection::new(
        ModelRouteId::new("kimi-platform.lifecycle.k3").expect("route id is valid"),
        ModelRouteRevision::new("2026-07-21").expect("route revision is valid"),
        ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id is valid"),
    )
}

fn prepared_after_admission(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> (
    KimiPlatformPreparedIntegration,
    AccessProfile,
    PreparedAccessEvidence,
) {
    let admitted = admitted_record(services, store).with_credential_refs([(
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
            evidence.clone(),
        )
        .expect("admitted fields produce preparation input"),
        services,
    )
    .expect("admitted instance prepares");
    (prepared, profile, evidence)
}

fn kimi_catalogue_entry() -> ModelCatalogEntry {
    ModelCatalogEntry::new(
        ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id is valid"),
        ModelMetadata::default().with_default(true),
    )
    .with_provider_id(ProviderId::new(KIMI_PLATFORM_PROVIDER_ID).expect("provider id is valid"))
}

fn snapshot_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> ConfiguredProviderInstanceRecord {
    let (prepared, profile, evidence) = prepared_after_admission(services, store);
    let catalogue = prepared
        .prepare_catalogue(KimiPlatformCatalogueProfileInput::new(
            RequestId::new("lifecycle-catalogue").expect("request id is valid"),
        ))
        .expect("catalogue prepares");
    let attempt = prepared
        .prepare_inference_attempt(KimiPlatformInferenceAttemptInput::new(
            RequestId::new("lifecycle-attempt").expect("request id is valid"),
            k3_model_selection(),
            OperationContent::new("lifecycle fixture prompt").expect("content is valid"),
            ReasoningMode::new("high").expect("reasoning is valid"),
            NonZeroU64::new(128).expect("output bound is valid"),
        ))
        .expect("attempt prepares");
    ConfiguredProviderInstanceRecord::admit(
        ConfiguredProviderInstanceAdmission::new(
            kimi_platform_direct_descriptor(),
            prepared.instance().clone(),
            profile,
            evidence,
        )
        .with_prepared_routes([
            catalogue.evidence().operation().clone(),
            attempt.evidence().operation().clone(),
        ])
        .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
            catalogue.evidence().operation().clone(),
            [kimi_catalogue_entry()],
        )),
    )
    .expect("047 snapshot assembles")
}
