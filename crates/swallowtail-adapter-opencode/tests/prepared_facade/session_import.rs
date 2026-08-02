use super::fixture::PreparedFixture;
use crate::http_support::StreamFixture;
use futures_executor::block_on;
use std::num::NonZeroU32;
use swallowtail_adapter_opencode::{OpenCodeSessionCatalogueInput, OpenCodeSessionProfileInput};
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionCatalogueBounds,
    ProviderSessionImportAvailability, ProviderSessionImportUnavailableReason,
};
use swallowtail_runtime::ProviderSessionOperationFailureStage;
use swallowtail_runtime::{ProviderSessionCatalogueId, RequestId};

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
            block_on(handle.close()),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
    }
}

#[test]
fn unverified_newer_server_does_not_offer_catalogue_preparation() {
    let fixture = PreparedFixture::new("opencode.import.newer", "1.18.11");
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
