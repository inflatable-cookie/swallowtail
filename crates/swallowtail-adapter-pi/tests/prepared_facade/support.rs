fn model(route: &str) -> PiModelSelection {
    PiModelSelection::new(
        ModelRouteId::new(route).expect("valid route"),
        ModelRouteRevision::new("1").expect("valid route revision"),
        ProviderId::new("fixture-provider").expect("valid provider"),
        ModelId::new("fixture-model").expect("valid model"),
    )
}

fn image(reference: &str) -> AttachmentDescriptor {
    AttachmentDescriptor::new(
        AttachmentRef::new(reference).expect("valid attachment"),
        "image/png",
        AttachmentRole::Input,
    )
    .expect("valid descriptor")
    .with_known_length(8)
}

fn assert_prompt_image(host: &FixtureHost) {
    let prompt = host
        .inputs()
        .into_iter()
        .find(|input| input["type"] == "prompt")
        .expect("prompt was dispatched");
    assert_eq!(prompt["images"][0]["type"], "image");
    assert_eq!(prompt["images"][0]["mimeType"], "image/png");
    assert_eq!(prompt["images"][0]["data"], "iVBORw0KGgo=");
    assert!(!prompt.to_string().contains("/tmp/"));
}

#[test]
fn latest_pi_is_qualified_and_later_stable_remains_unverified() {
    let host_id = ExecutionHostId::new("fixture.pi.prepared.newer").expect("valid host");
    let discovery = FixtureHost::version_probe("0.84.3");
    let prepared = block_on(prepare_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("latest Pi prepares");
    let InstalledExecutableCompatibility::Qualified(assessment) =
        prepared.observation().compatibility()
    else {
        panic!("latest Pi is qualified");
    };
    assert_eq!(
        assessment.behavior_revision().as_str(),
        "pi.rpc.strict-lf-v0.84.0-message-update-delta"
    );
    let run = prepared
        .prepare_run(PiRunProfileInput::new(
            RequestId::new("pi-latest-run").expect("valid request"),
            model("pi.latest.route"),
            OperationContent::new("latest private prompt").expect("valid content"),
            WorkingResourceRef::new("pi.latest.workspace").expect("valid resource"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
        ))
        .expect("latest run profile prepares");
    assert_eq!(
        run.plan()
            .interface_versions()
            .next()
            .expect("exact version is planned")
            .version()
            .as_str(),
        "0.84.3"
    );
    let basis = run
        .evidence()
        .operation()
        .observable_activity()
        .interface_basis()
        .next()
        .expect("activity basis is available");
    assert_eq!(
        basis.behavior_revision().as_str(),
        "pi.rpc.strict-lf-v0.84.0-message-update-delta"
    );

    let host_id = ExecutionHostId::new("fixture.pi.prepared.later").expect("valid host");
    let discovery = FixtureHost::version_probe("0.84.4");
    let prepared = block_on(prepare_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("later Pi remains executable");
    assert!(matches!(
        prepared.observation().compatibility(),
        InstalledExecutableCompatibility::UnverifiedNewer(_)
    ));
}

fn preparation_input(host: ExecutionHostId) -> PiPreparationInput {
    PiPreparationInput::new(
        ConfiguredInstanceId::new("pi.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("pi.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(PI_PACKAGE_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("pi.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("pi.prepared.access").expect("valid access"),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("pi/delegated-harness-auth").expect("valid namespace"),
            ),
            EntitlementMetering::Unknown,
            EndpointAudience::new("pi-harness").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(
            CredentialRef::new("pi.prepared.credential").expect("valid credential"),
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("pi.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> PiPreparationProbe {
    PiPreparationProbe::new(
        RequestId::new("pi-prepared-probe").expect("valid request"),
        ScopeId::new("pi-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn prepared_catalogue(
    host_id: ExecutionHostId,
    deadline: Option<Deadline>,
) -> swallowtail_adapter_pi::PiPreparedCatalogue {
    let discovery = FixtureHost::version_probe("0.80.10");
    let prepared = block_on(prepare_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("Pi prepares");
    let input = PiCatalogueProfileInput::new(
        RequestId::new("pi-prepared-catalogue-failure").expect("valid request"),
    );
    let input = match deadline {
        Some(deadline) => input.with_deadline(deadline),
        None => input,
    };
    prepared
        .prepare_catalogue(input)
        .expect("Pi catalogue profile prepares")
}
