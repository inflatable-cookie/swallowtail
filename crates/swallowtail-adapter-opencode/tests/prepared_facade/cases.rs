use super::fixture::PreparedFixture;
use futures_executor::block_on;
use std::sync::atomic::Ordering;
use swallowtail_adapter_opencode::{
    OpenCodeCatalogueProfileInput, OpenCodeSessionProfileInput, prepare_opencode_attached,
};
use swallowtail_core::{
    DriverRole, ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation, InstanceOwnership,
    InterfaceCompatibilityAssessment,
};
use swallowtail_runtime::{
    CleanupOutcome, DiscoveryCancellation, HostServices, PreparationStage, RequestId,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_catalogue_and_session_stay_separate_on_both_host_topologies() {
    for host_id in [
        "opencode.prepared.local",
        "opencode.prepared.remote-authoritative",
    ] {
        let fixture = PreparedFixture::new(host_id, "1.18.4");
        let prepared = fixture.prepared();
        assert_eq!(
            prepared.instance().ownership(),
            InstanceOwnership::ExternalAttached
        );
        assert_eq!(
            prepared.instance().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(prepared.server().binding().version().as_str(), "1.18.4");
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);

        let catalogue = prepared
            .prepare_catalogue(OpenCodeCatalogueProfileInput::new(
                RequestId::new("prepared-catalogue").unwrap(),
            ))
            .expect("catalogue prepares");
        assert_eq!(
            catalogue.plan().requirements().driver_role(),
            DriverRole::ModelCatalog
        );
        assert!(catalogue.plan().model_route_id().is_none());
        assert!(catalogue.plan().provider_id().is_none());
        assert_prepared_operation_evidence_matches_plan(
            catalogue.evidence().operation(),
            catalogue.plan(),
        );
        let models =
            block_on(catalogue.list_models(fixture.services())).expect("catalogue succeeds");
        assert_eq!(models.len(), 1);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 2);

        let session = prepared
            .prepare_session(OpenCodeSessionProfileInput::new(
                RequestId::new("prepared-session").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            ))
            .expect("session prepares");
        assert_eq!(
            session.plan().requirements().driver_role(),
            DriverRole::InteractiveSession
        );
        assert_eq!(
            session.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(session.plan().provider_id().unwrap().as_str(), "anthropic");
        assert_eq!(
            session.request().working_resource(),
            Some(&fixture.resource)
        );
        assert_prepared_operation_evidence_matches_plan(
            session.evidence().operation(),
            session.plan(),
        );
        let handle = block_on(session.open_session(fixture.services())).expect("session opens");
        assert_eq!(
            handle.provider_session_ref().unwrap().as_provider_value(),
            "ses_fixture"
        );
        assert!(handle.resume_binding().is_some());
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);

        let requests = fixture.server.requests();
        assert!(!requests.iter().any(|request| {
            request.contains("/dispose")
                || request.contains("/delete")
                || request.contains("/share")
                || request.contains("/config")
        }));
    }
}

#[test]
fn preparation_preserves_unverified_newer_and_rejects_binding_drift() {
    let fixture = PreparedFixture::new("opencode.prepared.newer", "1.18.5");
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
    let fixture = PreparedFixture::new("opencode.prepared.incompatible", "1.18.4-rc.1");
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
