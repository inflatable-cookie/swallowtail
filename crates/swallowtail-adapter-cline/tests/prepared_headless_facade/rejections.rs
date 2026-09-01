#[test]
fn preparation_rejects_access_axis_and_package_drift_before_json_work() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.reject").expect("host");
    let prepared = prepare(host_id.clone());
    let wrong_host = ExecutionHostId::new("fixture.prepared.headless.other").expect("host");
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );

    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let wrong_access = AccessProfile::new(
        AccessProfileId::new("cline.fixture.local-account").expect("access"),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("wrong.audience").expect("audience"),
        SupportAuthority::ProviderSupported,
    );
    let error = block_on(prepare_cline_headless(
        ClineHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            target(),
            EnvironmentRef::new("cline.fixture.isolated").expect("environment"),
            wrong_access,
            evidence(),
        ),
        probe(),
        discovery.services(host_id.clone()),
    ))
    .expect_err("access drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cline.headless.preparation.access_profile_rejected"
    );
    assert!(discovery.observed_process().is_none());

    let axis_host = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let error = block_on(prepare_cline_headless(
        ClineHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!("/fixture/bin/{CLINE_EXECUTABLE_NAME}"))
                    .expect("executable"),
                InterfaceVersionAxis::new("cline.acp").expect("axis"),
            ),
            EnvironmentRef::new("cline.fixture.isolated").expect("environment"),
            cline_local_account_access_profile(
                AccessProfileId::new("cline.fixture.local-account").expect("access"),
            ),
            evidence(),
        ),
        probe(),
        axis_host.services(host_id.clone()),
    ))
    .expect_err("ACP axis is not this route");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cline.headless.preparation.target_axis_mismatch"
    );
    assert!(axis_host.observed_process().is_none());

    let newer_host = ExecutionHostId::new("fixture.prepared.headless.newer").expect("host");
    let newer = DiscoveryHost::new("3.0.56");
    let error = block_on(prepare_cline_headless(
        preparation_input(newer_host.clone()),
        probe(),
        newer.services(newer_host),
    ))
    .expect_err("unqualified package fails");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::VersionParse
    );
    assert_eq!(
        newer.observed_process().expect("probe ran").arguments,
        ["--version"]
    );
}

#[test]
fn run_prepare_fails_closed_without_working_resource_authority() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.no-resource").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let prepared = block_on(prepare_cline_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("discovery does not require a working resource");
    let error = prepared
        .prepare_run(run_input("missing-resource"))
        .expect_err("run preflight requires working-resource authority");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
}
