mod support;

use futures_executor::block_on;
use std::num::NonZeroU32;
use support::{DriverFixture, ServerScenario};
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaModelStudioDriver, AlibabaSessionHistoryInput, EXACT_MODEL_ID,
    MAXIMUM_REPLAY_PAGE_BYTES, MODEL_ROUTE_ID, prepare_alibaba_model_studio,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, SessionRef};
use swallowtail_runtime::{
    CleanupOutcome, ProviderSessionHistoryBounds, ProviderSessionHistoryDriver,
    ProviderSessionHistoryId, ProviderSessionHistoryRequest, ProviderSessionHistoryTotal,
    RequestId, SessionAccessPolicy, SessionResumeBinding,
};

#[test]
fn retained_history_pages_newest_first_with_older_continuation() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let binding = binding(&fixture.retained_plan());
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let history = prepared
        .prepare_session_history(history_input("first", binding, 1, 8))
        .expect("history prepares");
    let first = block_on(history.page_history(fixture.services())).expect("first page succeeds");

    assert_eq!(first.fetched_count(), 1);
    assert!(first.has_older());
    assert_eq!(first.total(), ProviderSessionHistoryTotal::Exact(4));
    assert_eq!(
        first
            .items()
            .filter_map(|item| item.content().map(|content| content.as_str().to_owned()))
            .collect::<Vec<_>>(),
        ["Second output."]
    );

    let older_request = history
        .older_page_request(
            RequestId::new("history-older").expect("request id"),
            first.older_cursor().expect("older cursor").clone(),
        )
        .expect("older request prepares");
    let older =
        block_on(history.page(older_request, fixture.services())).expect("older page succeeds");

    assert_eq!(older.fetched_count(), 1);
    assert!(older.has_older());
    assert_eq!(
        older
            .items()
            .filter_map(|item| item.content().map(|content| content.as_str().to_owned()))
            .collect::<Vec<_>>(),
        ["Second fixture input"]
    );

    let requests = fixture.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(
        requests[0].target,
        "/compatible-mode/v1/conversations/conv_fixture_01"
    );
    assert!(requests.iter().all(|request| request.method != "DELETE"));
    assert!(requests.iter().all(|request| request.method != "POST"));
    assert_eq!(fixture.releases(), 2);
}

#[test]
fn retained_history_issues_no_live_handle_or_delete_side_effects() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let plan = fixture.history_plan("history-readonly", binding(&fixture.retained_plan()), 2, 8);
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-readonly").expect("request id"),
        &plan,
        None,
    )
    .expect("history request");
    let page = block_on(
        AlibabaModelStudioDriver::new().page_provider_session_history(
            plan,
            request,
            fixture.services(),
        ),
    )
    .expect("history page succeeds");

    assert_eq!(page.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(page.fetched_count(), 2);
    let requests = fixture.requests();
    assert!(requests.iter().all(|request| request.method != "DELETE"));
    assert!(
        requests
            .iter()
            .all(|request| request.target != "/compatible-mode/v1/responses")
    );
}

#[test]
fn retained_history_snapshot_overflow_fails_closed() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let history = prepared
        .prepare_session_history(history_input(
            "overflow",
            binding(&fixture.retained_plan()),
            1,
            1,
        ))
        .expect("history prepares");
    let failure = block_on(history.page_history(fixture.services())).expect_err("overflow fails");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_history.snapshot_limit_exceeded"
    );
    let requests = fixture.requests();
    assert!(requests.iter().all(|request| request.method != "DELETE"));
    assert_eq!(fixture.releases(), 1);
}

#[test]
fn retained_empty_history_is_exact_zero_without_cursor() {
    let fixture = DriverFixture::new(ServerScenario::RetainedEmptyHistory);
    let prepared = prepare_alibaba_model_studio(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let history = prepared
        .prepare_session_history(history_input(
            "empty",
            binding(&fixture.retained_plan()),
            2,
            8,
        ))
        .expect("history prepares");
    let page = block_on(history.page_history(fixture.services())).expect("empty page succeeds");

    assert_eq!(page.fetched_count(), 0);
    assert!(!page.has_older());
    assert!(page.older_cursor().is_none());
    assert_eq!(page.total(), ProviderSessionHistoryTotal::Exact(0));
    assert_eq!(page.cleanup(), &CleanupOutcome::Clean);
}

fn history_input(
    suffix: &str,
    binding: SessionResumeBinding,
    page_items: u32,
    snapshot_items: u32,
) -> AlibabaSessionHistoryInput {
    AlibabaSessionHistoryInput::new(
        RequestId::new(format!("history-{suffix}")).expect("request id"),
        ProviderSessionHistoryId::new(format!("alibaba-history-{suffix}")).expect("history id"),
        ModelRouteId::new(MODEL_ROUTE_ID).expect("route id"),
        ModelRouteRevision::new("fixture-1").expect("route revision"),
        ModelId::new(EXACT_MODEL_ID).expect("model id"),
        binding,
        history_bounds(page_items, snapshot_items),
    )
}

fn history_bounds(page_items: u32, snapshot_items: u32) -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        NonZeroU32::new(page_items).expect("page items"),
        std::num::NonZeroU64::new(MAXIMUM_REPLAY_PAGE_BYTES as u64).expect("page bytes"),
        NonZeroU32::new(64).expect("cursor bytes"),
        NonZeroU32::new(snapshot_items).expect("snapshot items"),
    )
}

fn binding(plan: &swallowtail_core::PreflightPlan) -> SessionResumeBinding {
    SessionResumeBinding::resource_free(
        SessionRef::new("conv_fixture_01").expect("session ref"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route").clone(),
        plan.model_id().expect("model").clone(),
        SessionAccessPolicy::resource_free(),
    )
}
