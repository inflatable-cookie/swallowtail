use super::*;
use std::sync::Arc;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, DeadlineObservation, MonotonicInstant,
    ProviderSessionOperationFailureStage, TimeService,
};
use swallowtail_testkit::{ExecutionTopologyFixture, assert_provider_session_import_contract};

#[test]
fn codex_acceptance_includes_the_provider_neutral_contract() {
    assert_provider_session_import_contract();
}

#[test]
fn catalogue_and_import_preserve_local_and_remote_authoritative_hosts() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let recording = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            swallowtail_testkit::RecordingOutcome::Succeed,
        );
        let prepared_app = super::super::prepared_on_host(
            CodexPreparedDriver::AppServer,
            "0.146.0",
            &recording,
            true,
            topology.execution_host_id().clone(),
        );
        let catalogue = prepared_app
            .prepare_session_catalogue(catalogue_input(topology.execution_host_id().as_str()))
            .expect("catalogue prepares on its authoritative host");
        let (process, list_state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
            ThreadCatalogueMode::Available,
        ));
        let page = block_on(catalogue.list_sessions(host_services_with_for(
            topology.execution_host_id().clone(),
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect("catalogue executes on its authoritative host");
        let candidate = page
            .candidates()
            .next()
            .expect("catalogue returns one importable candidate")
            .clone();
        let import = prepared_app
            .prepare_read_only_session_import(
                &catalogue,
                candidate,
                session_input(&format!("import-{}", topology.execution_host_id().as_str())),
            )
            .expect("import prepares on the same host");
        let (process, read_state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
            ThreadCatalogueMode::Available,
        ));
        let outcome = block_on(import.import_session(host_services_with_for(
            topology.execution_host_id().clone(),
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect("import executes on the same host");

        assert_eq!(
            import.plan().preflight().execution_host_id(),
            topology.execution_host_id()
        );
        assert_eq!(
            outcome.binding().execution_host_id(),
            topology.execution_host_id()
        );
        assert_eq!(
            list_state.request().working_resource.as_deref(),
            Some(working_resource().as_host_value())
        );
        assert_eq!(
            read_state.request().working_resource.as_deref(),
            Some(working_resource().as_host_value())
        );
        assert!(list_state.waited());
        assert!(read_state.waited());
    }
}

#[test]
fn catalogue_cancellation_deadline_disconnect_and_cleanup_are_joined() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);

    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("cancel-after-dispatch"))
        .expect("catalogue prepares");
    let cancellation = Arc::clone(catalogue.request().cancellation());
    let (process, state) =
        ScriptedAppServer::new(AppServerMode::ThreadCatalogue(ThreadCatalogueMode::Hold));
    let execution = std::thread::spawn({
        let future = catalogue.list_sessions(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        ));
        move || block_on(future)
    });
    while !state.methods().contains(&"thread/list".to_owned()) {
        std::thread::yield_now();
    }
    block_on(cancellation.request()).expect("catalogue cancellation records");
    let failure = execution
        .join()
        .expect("catalogue execution thread joins")
        .expect_err("cancelled catalogue fails");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::Cancelled
    );
    assert!(state.waited());

    let deadline = Deadline::at(MonotonicInstant::from_ticks(100));
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("deadline").with_deadline(deadline))
        .expect("deadline catalogue prepares");
    let (process, state) =
        ScriptedAppServer::new(AppServerMode::ThreadCatalogue(ThreadCatalogueMode::Hold));
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource])
        .with_time(Arc::new(ElapsedTime));
    let failure = block_on(catalogue.list_sessions(services))
        .expect_err("deadline-bound catalogue times out");
    assert_eq!(
        failure.stage(),
        ProviderSessionOperationFailureStage::TimedOut
    );
    assert!(state.waited());

    for (mode, expected_stage) in [
        (
            ThreadCatalogueMode::Disconnect,
            ProviderSessionOperationFailureStage::CatalogueDispatch,
        ),
        (
            ThreadCatalogueMode::CleanupFailure,
            ProviderSessionOperationFailureStage::Cleanup,
        ),
    ] {
        let catalogue = prepared_app
            .prepare_session_catalogue(catalogue_input("failure-boundary"))
            .expect("catalogue prepares");
        let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(mode));
        let failure = block_on(catalogue.list_sessions(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect_err("catalogue lifecycle failure is explicit");
        assert_eq!(failure.stage(), expected_stage);
        assert!(state.waited());
    }
}

#[test]
fn read_disconnect_and_cleanup_issue_no_import_binding() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("read-failures"))
        .expect("catalogue prepares");
    let candidate = catalogue_candidate(&catalogue, &recording);

    for (mode, expected_stage) in [
        (
            ThreadCatalogueMode::Disconnect,
            ProviderSessionOperationFailureStage::ImportRevalidation,
        ),
        (
            ThreadCatalogueMode::CleanupFailure,
            ProviderSessionOperationFailureStage::Cleanup,
        ),
    ] {
        let import = prepared_app
            .prepare_read_only_session_import(
                &catalogue,
                candidate.clone(),
                session_input("read-failure-import"),
            )
            .expect("import prepares");
        let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(mode));
        let failure = block_on(import.import_session(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect_err("failed read issues no binding");
        assert_eq!(failure.stage(), expected_stage);
        assert!(state.waited());
    }
}

struct ElapsedTime;

impl TimeService for ElapsedTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(
            async move { DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(100)) },
        )
    }
}
