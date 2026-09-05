use super::fixture::PreparedFixture;
use crate::http_support::StreamFixture;
use futures_executor::block_on;
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Arc;
use std::time::Duration;
use swallowtail_adapter_opencode::{
    OpenCodeSessionCatalogueInput, OpenCodeSessionProfileInput, OpenCodeSessionReconciliationInput,
};
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionCatalogueBounds,
    ProviderSessionImportAvailability, ProviderSessionImportUnavailableReason,
};
use swallowtail_runtime::{CancellationControl, ProviderSessionOperationFailureStage};
use swallowtail_runtime::{
    ProviderSessionCatalogueId, ProviderSessionReconciliationBounds, RequestId, RuntimeTurnId,
    SessionResumeBinding, WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};

fn bounds() -> ProviderSessionCatalogueBounds {
    ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(8).unwrap(),
        NonZeroU32::new(32).unwrap(),
        NonZeroU32::new(1024).unwrap(),
        NonZeroU32::new(256).unwrap(),
    )
    .unwrap()
}

#[test]
fn prepared_reconciliation_preserves_the_original_turn_without_attaching_it() {
    let fixture = PreparedFixture::new("opencode.reconciliation.prepared", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("reconciliation-session-plan").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .unwrap();
    let plan = session.plan();
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        session.request().access_policy().clone(),
    );
    let interrupted = RuntimeTurnId::new("consumer-turn-before-restart").unwrap();
    let restoration = prepared
        .prepare_working_state_restoration(OpenCodeSessionReconciliationInput::new(
            RequestId::new("reconcile-prepared-session").unwrap(),
            fixture.model(),
            binding,
            interrupted.clone(),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(4).unwrap(),
                NonZeroU64::new(1024).unwrap(),
            ),
        ))
        .expect("restoration prepares");
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    );

    let restored =
        block_on(restoration.restore(fixture.services())).expect("prepared restoration succeeds");
    let WorkingStateRestorationOutcome::SessionReconciled(outcome) = restored else {
        panic!("OpenCode must preserve session reconciliation truth");
    };
    assert_eq!(outcome.interrupted_turn_id(), &interrupted);
    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
    );
    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ProviderSession
    );
}

#[test]
fn opencode_acceptance_includes_the_provider_neutral_contract() {
    swallowtail_testkit::assert_provider_session_import_contract();
}

#[test]
fn catalogue_import_and_existing_load_share_one_exact_continuation_path() {
    for host in [
        "opencode.import.local",
        "opencode.import.remote-authoritative",
    ] {
        let fixture = PreparedFixture::new(host, "1.18.10");
        let prepared = fixture.prepared();
        let catalogue = prepared
            .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
                RequestId::new("opencode-import-catalogue").unwrap(),
                ProviderSessionCatalogueId::new("opencode-import-catalogue").unwrap(),
                fixture.resource.clone(),
                bounds(),
            ))
            .expect("catalogue prepares");
        let first =
            block_on(catalogue.list_sessions(fixture.services())).expect("first page lists");
        let candidates = first.candidates().cloned().collect::<Vec<_>>();
        assert_eq!(candidates.len(), 2);
        assert_eq!(
            candidates[0].activity(),
            ProviderSessionActivityState::Inactive
        );
        assert_eq!(
            candidates[0].import_availability(),
            ProviderSessionImportAvailability::Available
        );
        assert_eq!(
            candidates[1].import_availability(),
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::ProviderReportedUnavailable
            )
        );
        let next = catalogue
            .next_page_request(
                RequestId::new("opencode-import-page-two").unwrap(),
                first.next_cursor().expect("full page has cursor").clone(),
            )
            .expect("continuation request prepares");
        let second =
            block_on(catalogue.list_page(next, fixture.services())).expect("second page lists");
        let busy = second.candidates().next().expect("busy candidate");
        assert_eq!(busy.activity(), ProviderSessionActivityState::Active);
        assert_eq!(
            busy.import_availability(),
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::Active
            )
        );

        let imported = prepared
            .prepare_session_import(
                &catalogue,
                candidates[0].clone(),
                OpenCodeSessionProfileInput::new(
                    RequestId::new("opencode-import").unwrap(),
                    fixture.model(),
                    fixture.resource.clone(),
                ),
            )
            .expect("import prepares");
        let outcome =
            block_on(imported.import_session(fixture.services())).expect("import revalidates");
        assert_eq!(
            outcome.revalidation().activity(),
            ProviderSessionActivityState::Inactive
        );

        let session = prepared
            .prepare_session(OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-import-load-plan").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            ))
            .expect("ordinary session prepares");
        let loaded = block_on(
            session
                .load_session(
                    RequestId::new("opencode-import-load").unwrap(),
                    outcome.binding().clone(),
                    fixture.services(),
                )
                .expect("load request derives"),
        )
        .expect("imported session loads through existing replay");
        assert_eq!(loaded.replay().count(), 4);
        let (_, handle) = loaded.into_parts();
        assert!(matches!(
            block_on(fixture.close_session(handle)),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
    }
}

#[test]
fn unverified_newer_server_does_not_offer_catalogue_preparation() {
    let fixture = PreparedFixture::new("opencode.import.newer", "1.18.29");
    let prepared = fixture.prepared();
    let error = prepared
        .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
            RequestId::new("opencode-import-newer").unwrap(),
            ProviderSessionCatalogueId::new("opencode-import-newer").unwrap(),
            fixture.resource.clone(),
            bounds(),
        ))
        .expect_err("unverified newer catalogue is unavailable");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.session_catalogue_version_unsupported"
    );
}

#[test]
fn changed_candidate_issues_no_import_binding() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.import.stale",
        "1.18.10",
        StreamFixture::ImportTitleDrift,
    );
    let prepared = fixture.prepared();
    let catalogue = prepared
        .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
            RequestId::new("opencode-stale-catalogue").unwrap(),
            ProviderSessionCatalogueId::new("opencode-stale-catalogue").unwrap(),
            fixture.resource.clone(),
            bounds(),
        ))
        .unwrap();
    let listed = block_on(catalogue.list_sessions(fixture.services())).unwrap();
    let candidate = listed.candidates().next().unwrap().clone();
    let import = prepared
        .prepare_session_import(
            &catalogue,
            candidate,
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-stale-import").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            ),
        )
        .unwrap();
    let failure = block_on(import.import_session(fixture.services()))
        .expect_err("stale candidate issues no binding");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::ImportRevalidation
    );
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.opencode.session_import.candidate_changed"
    );
}

#[test]
fn cancellation_deadline_and_cleanup_release_leases_without_owning_the_server() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.import.lifecycle",
        "1.18.10",
        StreamFixture::ImportGated,
    );
    let prepared = fixture.prepared();
    let catalogue = prepared
        .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
            RequestId::new("opencode-lifecycle-cancel").unwrap(),
            ProviderSessionCatalogueId::new("opencode-lifecycle-cancel").unwrap(),
            fixture.resource.clone(),
            bounds(),
        ))
        .unwrap();
    let cancellation = Arc::clone(catalogue.request().cancellation());
    let response_gate = fixture.server.delete_response_gate();
    let execution = std::thread::spawn({
        let future = catalogue.list_sessions(fixture.services());
        move || block_on(future)
    });
    response_gate.wait_for_dispatch();
    block_on(cancellation.request()).unwrap();
    let failure = execution.join();
    response_gate.release();
    let failure = failure.unwrap().expect_err("cancelled list fails");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::Cancelled
    );
    assert_eq!(
        fixture.releases.load(std::sync::atomic::Ordering::SeqCst),
        2
    );
    assert!(
        fixture.prepared().server().is_qualified(),
        "attached server survives cancellation"
    );

    let deadline_fixture = PreparedFixture::new_with_fixture(
        "opencode.import.deadline",
        "1.18.10",
        StreamFixture::ImportGated,
    );
    let deadline_prepared = deadline_fixture.prepared();
    let deadline = deadline_prepared
        .prepare_session_catalogue(
            OpenCodeSessionCatalogueInput::new(
                RequestId::new("opencode-lifecycle-deadline").unwrap(),
                ProviderSessionCatalogueId::new("opencode-lifecycle-deadline").unwrap(),
                deadline_fixture.resource.clone(),
                bounds(),
            )
            .with_deadline(deadline_fixture.deadline_after(Duration::from_millis(10))),
        )
        .unwrap();
    let deadline_trigger = deadline_fixture.arm_manual_deadline();
    let response_gate = deadline_fixture.server.delete_response_gate();
    let execution = std::thread::spawn({
        let future = deadline.list_sessions(deadline_fixture.services());
        move || block_on(future)
    });
    response_gate.wait_for_dispatch();
    deadline_trigger.fire_and_wait_for_observation();
    let failure = execution.join();
    response_gate.release();
    let failure = failure.unwrap().expect_err("deadline list fails");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::TimedOut
    );

    let clean_fixture = PreparedFixture::new("opencode.import.cleanup", "1.18.10");
    let clean_prepared = clean_fixture.prepared();
    let catalogue = clean_prepared
        .prepare_session_catalogue(OpenCodeSessionCatalogueInput::new(
            RequestId::new("opencode-cleanup-failure").unwrap(),
            ProviderSessionCatalogueId::new("opencode-cleanup-failure").unwrap(),
            clean_fixture.resource.clone(),
            bounds(),
        ))
        .unwrap();
    let failure =
        block_on(catalogue.list_sessions(clean_fixture.services_with_release_failure(true)))
            .expect_err("credential cleanup failure is explicit");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::Cleanup
    );
    assert!(
        clean_fixture.prepared().server().is_qualified(),
        "attached server survives cleanup failure"
    );
}
