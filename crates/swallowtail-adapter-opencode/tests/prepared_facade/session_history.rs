use super::fixture::PreparedFixture;
use futures_executor::block_on;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_opencode::{OpenCodeSessionHistoryInput, OpenCodeSessionProfileInput};
use swallowtail_runtime::{
    OperationContent, ProviderSessionHistoryBounds, ProviderSessionHistoryId,
    ProviderSessionHistoryTotal, RequestId, SessionResumeBinding,
};

fn history_bounds(page_items: u32, snapshot_items: u32) -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        NonZeroU32::new(page_items).unwrap(),
        NonZeroU64::new(4096).unwrap(),
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(snapshot_items).unwrap(),
    )
}

fn history_binding(
    fixture: &PreparedFixture,
    session: &swallowtail_adapter_opencode::OpenCodePreparedSession,
) -> SessionResumeBinding {
    let plan = session.plan();
    SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        session.request().access_policy().clone(),
    )
}

#[test]
fn prepared_history_pages_newest_first_with_older_continuation() {
    let fixture = PreparedFixture::new("opencode.history.prepared", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("history-session-plan").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .unwrap();
    let binding = history_binding(&fixture, &session);
    let history = prepared
        .prepare_session_history(OpenCodeSessionHistoryInput::new(
            RequestId::new("prepared-history-first").unwrap(),
            ProviderSessionHistoryId::new("opencode-prepared-history").unwrap(),
            fixture.model(),
            binding,
            history_bounds(2, 8),
        ))
        .expect("history prepares");

    let first = block_on(history.page_history(fixture.services())).expect("first page succeeds");
    assert_eq!(first.fetched_count(), 2);
    assert!(first.has_older());
    assert_eq!(first.total(), ProviderSessionHistoryTotal::Exact(4));
    assert_eq!(
        first
            .items()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        ["Later question.", "Later answer."]
    );

    let older_request = history
        .older_page_request(
            RequestId::new("prepared-history-older").unwrap(),
            first.older_cursor().expect("older cursor").clone(),
        )
        .expect("older request prepares");
    let older =
        block_on(history.page(older_request, fixture.services())).expect("older page succeeds");
    assert_eq!(older.fetched_count(), 2);
    assert!(!older.has_older());
    assert_eq!(
        older
            .items()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        ["Earlier question.", "Earlier answer."]
    );

    let requests = fixture.server.requests();
    assert!(!requests.iter().any(|request| request.starts_with("POST ")));
    assert!(
        !requests
            .iter()
            .any(|request| request.starts_with("DELETE "))
    );
}

#[test]
fn prepared_history_snapshot_overflow_fails_closed() {
    let fixture = PreparedFixture::new("opencode.history.overflow", "1.18.10");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("overflow-session-plan").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .unwrap();
    let binding = history_binding(&fixture, &session);
    let history = prepared
        .prepare_session_history(OpenCodeSessionHistoryInput::new(
            RequestId::new("prepared-history-overflow").unwrap(),
            ProviderSessionHistoryId::new("opencode-prepared-overflow").unwrap(),
            fixture.model(),
            binding,
            history_bounds(2, 2),
        ))
        .expect("history prepares");

    let failure = block_on(history.page_history(fixture.services()))
        .expect_err("snapshot overflow fails closed");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_history.snapshot_limit_exceeded"
    );
}

#[test]
fn unverified_newer_server_does_not_offer_history_preparation() {
    let fixture = PreparedFixture::new("opencode.history.newer", "1.18.11");
    let prepared = fixture.prepared();
    let session = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("newer-history-session-plan").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .unwrap();
    let binding = history_binding(&fixture, &session);
    let error = prepared
        .prepare_session_history(OpenCodeSessionHistoryInput::new(
            RequestId::new("newer-history").unwrap(),
            ProviderSessionHistoryId::new("opencode-newer-history").unwrap(),
            fixture.model(),
            binding,
            history_bounds(2, 8),
        ))
        .expect_err("unverified newer history is unavailable");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.session_history_version_unsupported"
    );
}
