use std::num::{NonZeroU32, NonZeroU64};

fn history_bounds(page_items: u32, snapshot_items: u32) -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        NonZeroU32::new(page_items).unwrap(),
        NonZeroU64::new(4096).unwrap(),
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(snapshot_items).unwrap(),
    )
}

#[test]
fn newest_first_history_pages_over_session_messages_without_control_dispatch() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.session-history", "1.18.10");
    let plan = fixture.plan(DriverRole::ProviderSessionHistory);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionHistoryPlan::new(
        plan,
        ProviderSessionHistoryAgreement::new(
            ProviderSessionHistoryId::new("opencode-history-first").unwrap(),
            binding,
            history_bounds(2, 8),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-first-page").unwrap(),
        &plan,
        None,
    )
    .unwrap();

    let first = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
        plan.clone(),
        request,
        fixture.services(),
    ))
    .expect("first history page succeeds");

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

    let older_cursor = first.older_cursor().expect("older cursor").clone();
    let older_request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-older-page").unwrap(),
        &plan,
        Some(older_cursor),
    )
    .unwrap();
    let older = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
        plan,
        older_request,
        fixture.services(),
    ))
    .expect("older history page succeeds");

    assert_eq!(older.fetched_count(), 2);
    assert!(!older.has_older());
    assert!(older.older_cursor().is_none());
    assert_eq!(
        older
            .items()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        ["Earlier question.", "Earlier answer."]
    );

    let requests = server.requests();
    assert!(requests.iter().any(|request| request.starts_with("GET /global/health")));
    assert!(requests.iter().any(|request| request.contains("/session/ses_fixture")));
    assert!(requests.iter().any(|request| request.contains("/session/") && request.contains("/message")));
    assert!(!requests.iter().any(|request| request.starts_with("POST ")));
    assert!(!requests.iter().any(|request| request.starts_with("DELETE ")));
}

#[test]
fn history_snapshot_overflow_fails_closed() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.history-overflow", "1.18.10");
    let plan = fixture.plan(DriverRole::ProviderSessionHistory);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionHistoryPlan::new(
        plan,
        ProviderSessionHistoryAgreement::new(
            ProviderSessionHistoryId::new("opencode-history-overflow").unwrap(),
            binding,
            history_bounds(2, 2),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-overflow").unwrap(),
        &plan,
        None,
    )
    .unwrap();

    let failure = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
        plan,
        request,
        fixture.services(),
    ))
    .expect_err("snapshot overflow fails closed");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_history.snapshot_limit_exceeded"
    );
    let requests = server.requests();
    assert!(!requests.iter().any(|request| request.starts_with("POST ")));
    assert!(!requests.iter().any(|request| request.starts_with("DELETE ")));
}
