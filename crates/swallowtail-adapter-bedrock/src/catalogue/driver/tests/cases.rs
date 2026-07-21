#[test]
fn descriptor_and_registration_expose_only_the_catalogue_role() {
    let fixture = Fixture::new("host.registration");
    let descriptor = bedrock_catalogue_descriptor();
    assert!(descriptor.supports_role(DriverRole::ModelCatalog));
    assert!(!descriptor.supports_role(DriverRole::StructuredRun));
    assert_eq!(descriptor.transport_family().as_str(), "rust-sdk-control-plane");
    let registration = swallowtail_runtime::DriverRegistration::new(descriptor)
        .with_model_catalog(Arc::new(BedrockCatalogueDriver::new(fixture.binding())))
        .expect("catalogue role registers");
    assert!(registration.model_catalog().is_some());
    assert!(registration.structured_run().is_none());
}

#[test]
fn local_and_remote_authoritative_hosts_use_the_exact_control_plane() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::new(host);
        let fake = Arc::new(FakeExecutor::new(FakeMode::Success));
        let driver = BedrockCatalogueDriver::with_executor(fixture.binding(), fake.clone());
        let plan = fixture.plan();
        assert_eq!(plan.model_id(), None);
        assert_eq!(plan.provider_id(), None);
        let models = block_on(driver.list_models(
            plan,
            fixture.request("catalogue"),
            fixture.services(),
        ))
        .expect("catalogue succeeds");
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id().as_str(), "provider.model-v1");
        assert_eq!(models[0].provider_id(), None);
        assert_eq!(fake.calls.load(Ordering::SeqCst), 1);
        assert_eq!(fake.completions.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
        assert_eq!(
            fake.endpoints
                .lock()
                .expect("endpoint lock is available")
                .as_slice(),
            ["https://bedrock.eu-west-2.amazonaws.com"]
        );
    }
}

#[test]
fn deadline_signals_and_joins_private_executor_before_release() {
    let fixture = Fixture::new("host.deadline");
    let fake = Arc::new(FakeExecutor::new(FakeMode::WaitForCancellation));
    let driver = BedrockCatalogueDriver::with_executor(fixture.binding(), fake.clone());
    let request = fixture.request("deadline").with_deadline(Deadline::at(
        swallowtail_runtime::MonotonicInstant::from_ticks(100),
    ));
    let error = block_on(driver.list_models(fixture.plan(), request, fixture.services()))
        .expect_err("deadline fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.bedrock.catalogue_timed_out"
    );
    assert_eq!(fake.calls.load(Ordering::SeqCst), 1);
    assert_eq!(fake.cancellations.load(Ordering::SeqCst), 1);
    assert_eq!(fake.completions.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
}

#[test]
fn provider_failure_is_redacted_and_releases_credentials() {
    let fixture = Fixture::new("host.failure");
    let fake = Arc::new(FakeExecutor::new(FakeMode::ProviderFailure));
    let driver = BedrockCatalogueDriver::with_executor(fixture.binding(), fake);
    let error = block_on(driver.list_models(
        fixture.plan(),
        fixture.request("failure"),
        fixture.services(),
    ))
    .expect_err("provider failure is returned");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.bedrock.catalogue_unavailable"
    );
    let diagnostic = format!("{error:?}");
    assert!(!diagnostic.contains("fixture-secret-key"));
    assert!(!diagnostic.contains("private-resource"));
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
}

#[test]
fn binding_drift_fails_before_host_or_sdk_work() {
    let fixture = Fixture::new("host.drift");
    let fake = Arc::new(FakeExecutor::new(FakeMode::Success));
    let driver = BedrockCatalogueDriver::with_executor(
        fixture.binding_with_access("access.aws.runtime"),
        fake.clone(),
    );
    let error = block_on(driver.list_models(
        fixture.plan(),
        fixture.request("drift"),
        fixture.services(),
    ))
    .expect_err("binding drift fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.bedrock.catalogue_binding_mismatch"
    );
    assert_eq!(fake.calls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 0);
}
