use crate::fixtures::{prepared, profile_input};
use crate::provider_session_import::catalogue_input;
use crate::support::{CleanupEvent, FixtureHost, Scenario};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, Deadline, DeadlineObservation, MonotonicInstant,
    ProviderSessionOperationFailureStage, RequestId, SessionOptions, TimeService,
};
use swallowtail_testkit::{ExecutionTopologyFixture, assert_provider_session_import_contract};

#[test]
fn kimi_acceptance_includes_the_provider_neutral_contract() {
    assert_provider_session_import_contract();
}

#[test]
fn catalogue_and_import_preserve_local_and_remote_authoritative_hosts() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let host_id = topology.execution_host_id().clone();
        let preparation_host = FixtureHost::new(Scenario::Complete);
        let prepared = prepared(&preparation_host, host_id.clone(), "0.31.1");
        let catalogue = prepared
            .prepare_session_catalogue(catalogue_input(host_id.as_str()))
            .expect("catalogue prepares on the selected host");
        let list_host = FixtureHost::new(Scenario::ReasoningEffort311Success);
        let page = block_on(catalogue.list_sessions(list_host.services(host_id.clone())))
            .expect("catalogue executes on the selected host");
        let candidate = page.candidates().next().unwrap().clone();
        let import = prepared
            .prepare_session_import(
                &catalogue,
                candidate,
                profile_input("topology-import", SessionOptions::default()),
            )
            .expect("import prepares on the same host");
        let import_host = FixtureHost::new(Scenario::ReasoningEffort311Success);
        let outcome = block_on(import.import_session(import_host.services(host_id.clone())))
            .expect("import executes on the selected host");
        assert_eq!(outcome.binding().execution_host_id(), &host_id);
        assert_eq!(list_host.cleanup_counts(), (1, 1));
        assert_eq!(import_host.cleanup_counts(), (1, 1));
    }
}

#[test]
fn pagination_retains_cursor_scope_and_candidate_identity() {
    let host_id = ExecutionHostId::new("fixture.kimi.pagination").unwrap();
    let preparation_host = FixtureHost::new(Scenario::Complete);
    let prepared = prepared(&preparation_host, host_id.clone(), "0.28.1");
    let catalogue = prepared
        .prepare_session_catalogue(catalogue_input("pagination"))
        .expect("catalogue prepares");
    let first_host = FixtureHost::new(Scenario::CataloguePaginated);
    let first = block_on(catalogue.list_sessions(first_host.services(host_id.clone())))
        .expect("first page lists");
    let cursor = first.next_cursor().expect("next cursor").clone();
    assert_eq!(
        first.candidates().next().unwrap().candidate_id().as_str(),
        "kimi-acp-session-candidate-0"
    );
    let next = catalogue
        .next_page_request(RequestId::new("kimi-page-two").unwrap(), cursor)
        .expect("cursor remains bound to its catalogue");
    let second_host = FixtureHost::new(Scenario::CataloguePaginated);
    let second = block_on(catalogue.list_page(next, second_host.services(host_id)))
        .expect("second page lists");
    assert_eq!(
        second.candidates().next().unwrap().candidate_id().as_str(),
        "kimi-acp-session-candidate-1"
    );
    assert!(second.next_cursor().is_none());
}

#[test]
fn cancellation_deadline_disconnect_and_cleanup_are_joined() {
    let host_id = ExecutionHostId::new("fixture.kimi.lifecycle").unwrap();
    let preparation_host = FixtureHost::new(Scenario::Complete);
    let prepared = prepared(&preparation_host, host_id.clone(), "0.28.1");

    let catalogue = prepared
        .prepare_session_catalogue(catalogue_input("cancelled"))
        .expect("catalogue prepares");
    let cancellation = Arc::clone(catalogue.request().cancellation());
    let host = FixtureHost::new(Scenario::CatalogueHold);
    let execution = std::thread::spawn({
        let future = catalogue.list_sessions(host.services(host_id.clone()));
        move || block_on(future)
    });
    while !host.wire_methods().contains(&"session/list".to_owned()) {
        std::thread::yield_now();
    }
    block_on(cancellation.request()).expect("cancellation records");
    let failure = execution
        .join()
        .unwrap()
        .expect_err("cancelled catalogue fails");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::Cancelled
    );
    assert_eq!(host.cleanup_counts(), (1, 1));
    assert!(host.cleanup_events().contains(&CleanupEvent::ProcessWait));

    let catalogue = prepared
        .prepare_session_catalogue(
            catalogue_input("deadline")
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(100))),
        )
        .expect("deadline catalogue prepares");
    let deadline_host = FixtureHost::new(Scenario::CatalogueHold);
    let services = deadline_host
        .services(host_id.clone())
        .with_time(Arc::new(ElapsedTime));
    let failure =
        block_on(catalogue.list_sessions(services)).expect_err("deadline-bound catalogue fails");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::TimedOut
    );
    assert_eq!(deadline_host.cleanup_counts(), (1, 1));

    for (scenario, expected) in [
        (
            Scenario::CatalogueDisconnect,
            ProviderSessionOperationFailureStage::CatalogueDispatch,
        ),
        (
            Scenario::CleanupFailure,
            ProviderSessionOperationFailureStage::Cleanup,
        ),
    ] {
        let catalogue = prepared
            .prepare_session_catalogue(catalogue_input("failure"))
            .expect("catalogue prepares");
        let failure_host = FixtureHost::new(scenario);
        let failure = block_on(catalogue.list_sessions(failure_host.services(host_id.clone())))
            .expect_err("lifecycle failure is explicit");
        assert_eq!(failure.stage(), expected);
        assert!(
            failure_host
                .cleanup_events()
                .contains(&CleanupEvent::ProcessWait)
        );
    }
}

#[test]
fn unsupported_capability_and_import_disconnect_issue_no_binding() {
    let host_id = ExecutionHostId::new("fixture.kimi.unsupported").unwrap();
    let preparation_host = FixtureHost::new(Scenario::Complete);
    let prepared = prepared(&preparation_host, host_id.clone(), "0.28.1");
    let catalogue = prepared
        .prepare_session_catalogue(catalogue_input("unsupported"))
        .expect("catalogue prepares from exact version evidence");
    let unsupported = FixtureHost::new(Scenario::CatalogueUnsupported);
    let failure = block_on(catalogue.list_sessions(unsupported.services(host_id.clone())))
        .expect_err("agent without list capability fails");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::BeforeDispatch
    );
    assert!(
        !unsupported
            .wire_methods()
            .contains(&"session/list".to_owned())
    );

    let source_host = FixtureHost::new(Scenario::Complete);
    let page = block_on(catalogue.list_sessions(source_host.services(host_id.clone())))
        .expect("source candidate lists");
    let import = prepared
        .prepare_session_import(
            &catalogue,
            page.candidates().next().unwrap().clone(),
            profile_input("disconnect-import", SessionOptions::default()),
        )
        .expect("import prepares");
    let disconnected = FixtureHost::new(Scenario::CatalogueDisconnect);
    let failure = block_on(import.import_session(disconnected.services(host_id)))
        .expect_err("disconnected revalidation issues no binding");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::ImportRevalidation
    );
}

struct ElapsedTime;

impl TimeService for ElapsedTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}
