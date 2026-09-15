fn model(route: &str) -> OhMyPiModelSelection {
    OhMyPiModelSelection::new(
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
fn seventeen_and_eighteen_segments_are_qualified_and_later_stable_remains_unverified() {
    let host_id = ExecutionHostId::new("fixture.pi.prepared.newer").expect("valid host");
    let discovery = FixtureHost::version_probe("17.2.9");
    let prepared = block_on(prepare_oh_my_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("latest OhMyPi prepares");
    let InstalledExecutableCompatibility::Qualified(assessment) =
        prepared.observation().compatibility()
    else {
        panic!("latest OhMyPi is qualified");
    };
    assert_eq!(
        assessment.behavior_revision().as_str(),
        "oh-my-pi.rpc-v2-v17.2.9"
    );
    let run = prepared
        .prepare_run(OhMyPiRunProfileInput::new(
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
        "17.2.9"
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
        "oh-my-pi.rpc-v2-v17.2.9"
    );
}

#[test]
fn later_17_x_and_every_18_x_point_are_qualified_on_their_own_behavior() {
    for (version, behavior) in [
        ("17.4.1", "oh-my-pi.rpc-v2-v17.2.9"),
        ("17.4.2", "oh-my-pi.rpc-v2-v17.2.9"),
        ("18.0.0", "oh-my-pi.rpc-v2-v18.0.0"),
        ("18.1.16", "oh-my-pi.rpc-v2-v18.0.0"),
        ("18.1.22", "oh-my-pi.rpc-v2-v18.0.0"),
    ] {
        let host_id =
            ExecutionHostId::new(format!("fixture.pi.prepared.{version}")).expect("valid host");
        let discovery = FixtureHost::version_probe(version);
        let prepared = block_on(prepare_oh_my_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id),
        ))
        .expect("qualified OhMyPi prepares");
        let InstalledExecutableCompatibility::Qualified(assessment) =
            prepared.observation().compatibility()
        else {
            panic!("{version} is qualified");
        };
        assert_eq!(
            assessment.behavior_revision().as_str(),
            behavior,
            "{version} uses its segment behavior"
        );
    }
}

#[test]
fn passed_major_boundary_and_unpublished_gaps_remain_unexecutable() {
    for version in ["17.4.3", "17.4.4", "18.0.2", "18.1.7"] {
        let host_id =
            ExecutionHostId::new(format!("fixture.pi.prepared.gap.{version}")).expect("valid host");
        let discovery = FixtureHost::version_probe(version);
        let outcome = block_on(prepare_oh_my_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id),
        ));
        assert!(outcome.is_err(), "{version} must fail preparation");
    }
}

#[test]
fn later_stable_above_official_stays_unverified_newer() {
    let host_id = ExecutionHostId::new("fixture.pi.prepared.unverified").expect("valid host");
    let discovery = FixtureHost::version_probe("18.1.23");
    let prepared = block_on(prepare_oh_my_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("later OhMyPi remains executable");
    let InstalledExecutableCompatibility::UnverifiedNewer(assessment) =
        prepared.observation().compatibility()
    else {
        panic!("later OhMyPi is unverified newer");
    };
    assert_eq!(
        assessment.behavior_revision().as_str(),
        "oh-my-pi.rpc-v2-v18.0.0"
    );
}

fn preparation_input(host: ExecutionHostId) -> OhMyPiPreparationInput {
    OhMyPiPreparationInput::new(
        ConfiguredInstanceId::new("pi.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("pi.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(OH_MY_PI_PACKAGE_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("pi.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("pi.prepared.access").expect("valid access"),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::Unknown,
            EndpointAudience::new("oh-my-pi-harness").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("pi.prepared.access").expect("valid access"),
        CredentialState::NotRequired,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> OhMyPiPreparationProbe {
    OhMyPiPreparationProbe::new(
        RequestId::new("pi-prepared-probe").expect("valid request"),
        ScopeId::new("pi-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn prepared_catalogue(
    host_id: ExecutionHostId,
    deadline: Option<Deadline>,
) -> swallowtail_adapter_oh_my_pi::OhMyPiPreparedCatalogue {
    let discovery = FixtureHost::version_probe("17.2.9");
    let prepared = block_on(prepare_oh_my_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("OhMyPi prepares");
    let input = OhMyPiCatalogueProfileInput::new(
        RequestId::new("pi-prepared-catalogue-failure").expect("valid request"),
    );
    let input = match deadline {
        Some(deadline) => input.with_deadline(deadline),
        None => input,
    };
    prepared
        .prepare_catalogue(input)
        .expect("OhMyPi catalogue profile prepares")
}
