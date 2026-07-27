use super::support::{StagedDeadline, lifecycle_binding};
use super::*;

#[test]
fn pre_dispatch_cancellation_starts_no_app_server() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let operation = prepared_app
        .prepare_archive_session(CodexSessionManagementInput::new(
            RequestId::new("cancelled-archive").unwrap(),
            lifecycle_binding(&prepared_app, "0.145.0"),
        ))
        .expect("archive prepares");
    block_on(operation.request().cancellation().request()).expect("cancellation records");
    let (process, state) = ScriptedAppServer::new(AppServerMode::LifecycleSuccess);
    let outcome = block_on(operation.execute(crate::support::host_services(process)))
        .expect("cancelled operation resolves");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert!(!state.started());
}

#[test]
fn after_dispatch_cancellation_and_deadline_remain_unconfirmed() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let operation = prepared_app
        .prepare_archive_session(CodexSessionManagementInput::new(
            RequestId::new("cancel-after-dispatch").unwrap(),
            lifecycle_binding(&prepared_app, "0.145.0"),
        ))
        .expect("archive prepares");
    let cancellation = Arc::clone(operation.request().cancellation());
    let (process, state) = ScriptedAppServer::new(AppServerMode::LifecycleHold);
    let execution = std::thread::spawn({
        let future = operation.execute(crate::support::host_services(process));
        move || block_on(future)
    });
    while !state.methods().contains(&"thread/archive".to_owned()) {
        std::thread::yield_now();
    }
    block_on(cancellation.request()).expect("cancellation wakes the operation");
    let outcome = execution
        .join()
        .expect("execution thread joins")
        .expect("cancelled operation resolves");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert!(state.forced());
    assert!(state.waited());

    let operation = prepared_app
        .prepare_archive_session(
            CodexSessionManagementInput::new(
                RequestId::new("deadline-after-dispatch").unwrap(),
                lifecycle_binding(&prepared_app, "0.145.0"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(100))),
        )
        .expect("deadline-bound archive prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::LifecycleHold);
    let services =
        crate::support::host_services(process).with_time(Arc::new(StagedDeadline::default()));
    let outcome = block_on(operation.execute(services)).expect("deadline resolves");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert!(state.methods().contains(&"thread/archive".to_owned()));
    assert!(state.forced());
    assert!(state.waited());
}
