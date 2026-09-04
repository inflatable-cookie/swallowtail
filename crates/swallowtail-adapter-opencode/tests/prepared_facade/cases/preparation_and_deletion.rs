#[test]
fn preparation_preserves_unverified_newer_and_rejects_binding_drift() {
    let fixture = PreparedFixture::new("opencode.prepared.newer", "1.18.29");
    let prepared = fixture.prepared();
    assert!(matches!(
        prepared.server().compatibility(),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let error = prepared
        .validate_execution_binding(
            &ExecutionHostId::new("opencode.prepared.other").unwrap(),
            &fixture.target,
        )
        .expect_err("host drift is rejected");
    assert_eq!(error.stage(), PreparationStage::TargetSelection);
    assert_eq!(fixture.server.requests().len(), 1);
}

#[test]
fn cancelled_or_mismatched_preparation_stops_before_endpoint_work() {
    let fixture = PreparedFixture::new("opencode.prepared.cancelled", "1.18.4");
    let cancellation = DiscoveryCancellation::new();
    block_on(swallowtail_runtime::CancellationControl::request(
        &cancellation,
    ))
    .expect("cancellation request succeeds");
    let error = block_on(prepare_opencode_attached(
        fixture.preparation_input(),
        fixture.probe(cancellation),
        fixture.services(),
    ))
    .expect_err("cancelled preparation fails");
    assert_eq!(error.stage(), PreparationStage::BoundedOutput);
    assert!(fixture.server.requests().is_empty());

    let wrong_services = HostServices::new(ExecutionHostId::new("wrong.host").unwrap());
    let error = block_on(prepare_opencode_attached(
        fixture.preparation_input(),
        fixture.probe(DiscoveryCancellation::new()),
        wrong_services,
    ))
    .expect_err("host mismatch fails");
    assert_eq!(error.stage(), PreparationStage::TargetSelection);
    assert!(fixture.server.requests().is_empty());
}

#[test]
fn incompatible_health_is_classified_and_credential_cleanup_remains_joined() {
    let fixture = PreparedFixture::new("opencode.prepared.incompatible", "1.18.11-rc.1");
    let error = block_on(prepare_opencode_attached(
        fixture.preparation_input(),
        fixture.probe(DiscoveryCancellation::new()),
        fixture.services(),
    ))
    .expect_err("prerelease is incompatible");
    assert_eq!(error.stage(), PreparationStage::CompatibilityClassification);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
}

#[test]
fn preparation_cleanup_failure_stays_visible_after_successful_health() {
    let fixture = PreparedFixture::new("opencode.prepared.cleanup", "1.18.4");
    let error = block_on(prepare_opencode_attached(
        fixture.preparation_input(),
        fixture.probe(DiscoveryCancellation::new()),
        fixture.services_with_release_failure(true),
    ))
    .expect_err("cleanup failure prevents successful preparation");
    assert_eq!(error.stage(), PreparationStage::Cleanup);
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.cleanup_failed"
    );
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
}

#[test]
fn prepared_session_promotes_one_exact_inactive_delete_binding() {
    let fixture = PreparedFixture::new("opencode.prepared.delete", "1.18.4");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("delete-session").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let handle = block_on(session.open_session(fixture.services())).expect("session opens");
    let binding = handle
        .management_binding()
        .expect("prepared session exposes management binding")
        .clone();
    assert_eq!(binding.working_resource(), Some(&fixture.resource));
    assert_eq!(block_on(fixture.close_session(handle)), CleanupOutcome::Clean);

    let delete = prepared
        .prepare_delete_session(OpenCodeSessionManagementInput::new(
            RequestId::new("delete-session-operation").unwrap(),
            binding,
        ))
        .expect("delete prepares");
    assert_eq!(
        delete.plan().preflight().requirements().driver_role(),
        DriverRole::ProviderSessionManagement
    );
    assert_prepared_operation_evidence_matches_plan(
        delete.evidence().operation(),
        delete.plan().preflight(),
    );
    let outcome = block_on(delete.execute(fixture.services())).expect("delete executes");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert_eq!(
        outcome.effect().confirmed_deletion_strength(),
        Some(ProviderSessionDeletionStrength::ProviderDataDeleted)
    );
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);
    let requests = fixture.server.requests();
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.starts_with("DELETE /session/ses_fixture?directory="))
            .count(),
        1
    );
}

#[test]
fn prepared_delete_rejects_route_drift_and_unverified_newer_by_default() {
    let first = PreparedFixture::new("opencode.prepared.delete.first", "1.18.4");
    let second = PreparedFixture::new("opencode.prepared.delete.second", "1.18.4");
    let first_prepared = first.prepared();
    let second_prepared = second.prepared();
    let session = first_prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("delete-drift-session").unwrap(),
            first.model(),
            first.resource.clone(),
        ))
        .expect("session prepares");
    let handle = block_on(session.open_session(first.services())).expect("session opens");
    let binding = handle.management_binding().unwrap().clone();
    assert_eq!(block_on(first.close_session(handle)), CleanupOutcome::Clean);
    let error = second_prepared
        .prepare_delete_session(OpenCodeSessionManagementInput::new(
            RequestId::new("delete-drift").unwrap(),
            binding,
        ))
        .expect_err("foreign binding rejects");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.lifecycle_binding_mismatch"
    );
    assert!(
        !second
            .server
            .requests()
            .iter()
            .any(|request| request.starts_with("DELETE "))
    );

    let newer = PreparedFixture::new("opencode.prepared.delete.newer", "1.18.29");
    let newer_prepared = newer.prepared();
    let session = newer_prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("delete-newer-session").unwrap(),
            newer.model(),
            newer.resource.clone(),
        ))
        .expect("newer session prepares");
    let handle = block_on(session.open_session(newer.services())).expect("newer session opens");
    let binding = handle.management_binding().unwrap().clone();
    assert_eq!(block_on(newer.close_session(handle)), CleanupOutcome::Clean);
    let error = newer_prepared
        .prepare_delete_session(OpenCodeSessionManagementInput::new(
            RequestId::new("delete-newer").unwrap(),
            binding.clone(),
        ))
        .expect_err("unverified newer deletion needs acceptance");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.lifecycle_unverified_newer"
    );
    newer_prepared
        .prepare_delete_session(
            OpenCodeSessionManagementInput::new(
                RequestId::new("delete-newer-accepted").unwrap(),
                binding,
            )
            .allow_unverified_newer(),
        )
        .expect("explicit acceptance prepares");
}
