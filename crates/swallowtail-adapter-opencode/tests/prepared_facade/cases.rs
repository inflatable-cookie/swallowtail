use super::fixture::PreparedFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use std::sync::atomic::Ordering;
use swallowtail_adapter_opencode::{
    OpenCodeCatalogueProfileInput, OpenCodeRunProfileInput, OpenCodeSessionManagementInput,
    OpenCodeSessionProfileInput, prepare_opencode_attached,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, ExecutionHostId, HarnessConfigurationPosture,
    HarnessIsolation, InstanceOwnership, InterfaceCompatibilityAssessment, OwnedRemoteResourceKind,
    ProviderSessionDeletionStrength, ProviderSessionEffectTruth, ReasoningMode,
    StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, DiscoveryCancellation, HostServices, OperationContent,
    PreparationStage, ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId,
    SchemaDocument, StructuredOutputDescriptor, StructuredRunDriver, TerminalStatus,
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
fn prepared_structured_run_is_private_and_deletes_its_session_on_both_host_topologies() {
    for host_id in ["opencode.run.local", "opencode.run.remote-authoritative"] {
        let fixture = PreparedFixture::new(host_id, "1.18.4");
        let prepared = fixture.prepared();
        let run = prepared
            .prepare_run(OpenCodeRunProfileInput::new(
                RequestId::new("prepared-run").unwrap(),
                fixture.model(),
                OperationContent::new("fixture private prompt").unwrap(),
                fixture.resource.clone(),
            ))
            .expect("structured run prepares");
        assert_eq!(
            run.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());

        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        assert!(handle.provider_run_ref().is_none());
        assert_eq!(
            handle.cancellation().scope(),
            swallowtail_core::CancellationScope::StructuredRun
        );
        let mut events = handle.take_events().expect("events are available");
        let terminal = handle
            .take_terminal_outcome()
            .expect("terminal outcome is available");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("runtime event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(RemoteResourceDeletionOutcome::Confirmed)
        );
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 2);

        let requests = fixture.server.requests();
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("POST /session?directory="))
                .count(),
            1
        );
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.contains("/prompt_async?directory="))
                .count(),
            1
        );
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("DELETE /session/ses_fixture?directory="))
                .count(),
            1
        );
        let prompt = requests
            .iter()
            .position(|request| request.contains("/prompt_async?directory="))
            .expect("prompt request observed");
        let delete = requests
            .iter()
            .position(|request| request.starts_with("DELETE /session/ses_fixture?directory="))
            .expect("delete request observed");
        assert!(prompt < delete);
    }
}

#[test]
fn prepared_generation_controls_use_exact_catalogue_evidence_and_zero_retry_dispatch() {
    for (host_id, version) in [
        ("opencode.controls.local", "1.18.4"),
        ("opencode.controls.remote-authoritative", "1.18.5"),
    ] {
        let fixture = PreparedFixture::new(host_id, version);
        let prepared = fixture.prepared();
        let mut models = block_on(
            prepared
                .prepare_catalogue(OpenCodeCatalogueProfileInput::new(
                    RequestId::new(format!("controls-catalogue-{host_id}"))
                        .expect("request id is valid"),
                ))
                .expect("catalogue prepares")
                .list_models(fixture.services()),
        )
        .expect("catalogue succeeds");
        let reasoning = ReasoningMode::new("high").expect("reasoning is valid");
        let run = prepared
            .prepare_run(
                OpenCodeRunProfileInput::new(
                    RequestId::new(format!("controls-run-{host_id}")).expect("request id is valid"),
                    fixture.model().with_catalogue_entry(models.remove(0)),
                    OperationContent::new("Return one fixture result").expect("content is valid"),
                    fixture.resource.clone(),
                )
                .with_reasoning_mode(reasoning.clone())
                .with_structured_output(schema()),
            )
            .expect("generation controls prepare");
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::ReasoningSelection
                && requirement
                    .constraints()
                    .eq([&CapabilityConstraint::ReasoningMode(reasoning.clone())])
        }));
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::StructuredOutput
                && requirement.constraints().any(|constraint| {
                    constraint
                        == &CapabilityConstraint::StructuredOutputEnforcement(
                            StructuredOutputEnforcement::HarnessValidated,
                        )
                })
        }));
        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        let outcome = block_on(
            handle
                .take_terminal_outcome()
                .expect("terminal outcome exists"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

        let request_count = fixture.server.requests().len();
        let error = prepared
            .prepare_run(
                OpenCodeRunProfileInput::new(
                    RequestId::new(format!("controls-missing-{host_id}"))
                        .expect("request id is valid"),
                    fixture.model(),
                    OperationContent::new("No catalogue evidence").expect("content is valid"),
                    fixture.resource.clone(),
                )
                .with_reasoning_mode(reasoning.clone()),
            )
            .expect_err("missing catalogue evidence fails");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.opencode.preparation.catalogue_evidence_missing"
        );
        assert_eq!(fixture.server.requests().len(), request_count);
    }
}

#[test]
fn structured_run_disconnect_and_delete_failure_remain_separate() {
    for (suffix, stream_fixture, terminal_code, deletion, cleanup_code) in [
        (
            "disconnect",
            crate::http_support::StreamFixture::Disconnect,
            Some("swallowtail.opencode.sse_disconnected"),
            RemoteResourceDeletionOutcome::Confirmed,
            None,
        ),
        (
            "delete-unconfirmed",
            crate::http_support::StreamFixture::DeleteMalformedSuccess,
            None,
            RemoteResourceDeletionOutcome::Unconfirmed,
            Some("swallowtail.opencode.run_delete_unconfirmed"),
        ),
    ] {
        let fixture = PreparedFixture::new_with_fixture(
            &format!("opencode.run.{suffix}"),
            "1.18.4",
            stream_fixture,
        );
        let prepared = fixture.prepared();
        let run = prepared
            .prepare_run(OpenCodeRunProfileInput::new(
                RequestId::new(format!("run-{suffix}")).unwrap(),
                fixture.model(),
                OperationContent::new("fixture private prompt").unwrap(),
                fixture.resource.clone(),
            ))
            .expect("structured run prepares");
        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        let outcome = block_on(
            handle
                .take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        match terminal_code {
            Some(code) => match outcome.status() {
                TerminalStatus::RuntimeFailed(diagnostic) => assert_eq!(diagnostic.code(), code),
                status => panic!("expected runtime failure, got {status:?}"),
            },
            None => assert_eq!(outcome.status(), &TerminalStatus::Completed),
        }
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(deletion)
        );
        match cleanup_code {
            Some(code) => match outcome.cleanup() {
                CleanupOutcome::Failed(diagnostic) => assert_eq!(diagnostic.code(), code),
                cleanup => panic!("expected failed cleanup, got {cleanup:?}"),
            },
            None => assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean),
        }
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn unsupported_structured_input_stops_before_opencode_network_effects() {
    let fixture = PreparedFixture::new("opencode.run.unsupported", "1.18.4");
    let prepared = fixture.prepared();
    let run = prepared
        .prepare_run(OpenCodeRunProfileInput::new(
            RequestId::new("run-unsupported").unwrap(),
            fixture.model(),
            OperationContent::new("fixture private prompt").unwrap(),
            fixture.resource.clone(),
        ))
        .expect("structured run prepares");
    let request_count = fixture.server.requests().len();
    let (_, plan, request) = run.into_parts();
    let request =
        request.with_maximum_output_tokens(NonZeroU64::new(8).expect("non-zero token limit"));
    let error = block_on(
        swallowtail_adapter_opencode::OpenCodeHttpDriver::new().start_run(
            plan,
            request,
            fixture.services(),
        ),
    )
    .err()
    .expect("unsupported run fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.unsupported"
    );
    assert_eq!(fixture.server.requests().len(), request_count);
}

fn schema() -> StructuredOutputDescriptor {
    StructuredOutputDescriptor::new(
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"result":{"type":"string"}},"required":["result"],"additionalProperties":false}"#,
            4096,
        )
        .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("schema descriptor is valid")
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
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

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
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
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

    let newer = PreparedFixture::new("opencode.prepared.delete.newer", "1.18.5");
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
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
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

#[test]
fn post_dispatch_cancellation_is_joined_and_unconfirmed() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.prepared.delete.cancel",
        "1.18.4",
        crate::http_support::StreamFixture::DeleteDelayed,
    );
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("delete-cancel-session").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let handle = block_on(session.open_session(fixture.services())).expect("session opens");
    let binding = handle.management_binding().unwrap().clone();
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let delete = prepared
        .prepare_delete_session(OpenCodeSessionManagementInput::new(
            RequestId::new("delete-cancel").unwrap(),
            binding,
        ))
        .expect("delete prepares");
    let cancellation = std::sync::Arc::clone(delete.request().cancellation());
    let requests = fixture.server.request_log();
    let canceller = std::thread::spawn(move || {
        while !requests
            .lock()
            .expect("request lock")
            .iter()
            .any(|request| request.starts_with("DELETE "))
        {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        block_on(cancellation.request()).expect("cancellation requests");
    });
    let outcome = block_on(delete.execute(fixture.services())).expect("delete resolves");
    canceller.join().expect("canceller joins");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
}

#[test]
fn provider_rejection_and_server_failure_preserve_effect_truth() {
    for (suffix, stream_fixture, expected, code) in [
        (
            "missing",
            crate::http_support::StreamFixture::DeleteMissing,
            ProviderSessionEffectTruth::FailedBeforeEffect,
            "swallowtail.opencode.lifecycle.delete_rejected",
        ),
        (
            "server",
            crate::http_support::StreamFixture::DeleteServerError,
            ProviderSessionEffectTruth::UnconfirmedAfterEffect,
            "swallowtail.opencode.lifecycle.delete_unconfirmed",
        ),
    ] {
        let fixture = PreparedFixture::new_with_fixture(
            &format!("opencode.prepared.delete.{suffix}"),
            "1.18.4",
            stream_fixture,
        );
        let prepared = fixture.prepared();
        let session = prepared
            .prepare_session(OpenCodeSessionProfileInput::new(
                RequestId::new(format!("delete-{suffix}-session")).unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            ))
            .expect("session prepares");
        let handle = block_on(session.open_session(fixture.services())).expect("session opens");
        let binding = handle.management_binding().unwrap().clone();
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        let delete = prepared
            .prepare_delete_session(OpenCodeSessionManagementInput::new(
                RequestId::new(format!("delete-{suffix}")).unwrap(),
                binding,
            ))
            .expect("delete prepares");
        let outcome = block_on(delete.execute(fixture.services())).expect("delete resolves");
        assert_eq!(outcome.effect().truth(), expected);
        assert_eq!(outcome.diagnostic().expect("diagnostic").code(), code);
        let debug = format!("{outcome:?}");
        assert!(!debug.contains("private missing-target detail"));
        assert!(!debug.contains("private server detail"));
    }
}

#[test]
fn exact_server_version_drift_stops_before_delete_dispatch() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.prepared.delete.health-drift",
        "1.18.4",
        crate::http_support::StreamFixture::DeleteHealthDrift,
    );
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("delete-health-drift-session").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("session prepares");
    let handle = block_on(session.open_session(fixture.services())).expect("session opens");
    let binding = handle.management_binding().unwrap().clone();
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let delete = prepared
        .prepare_delete_session(OpenCodeSessionManagementInput::new(
            RequestId::new("delete-health-drift").unwrap(),
            binding,
        ))
        .expect("delete prepares");
    let error = block_on(delete.execute(fixture.services())).expect_err("version drift rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.version_mismatch"
    );
    assert!(
        !fixture
            .server
            .requests()
            .iter()
            .any(|request| request.starts_with("DELETE "))
    );
}
