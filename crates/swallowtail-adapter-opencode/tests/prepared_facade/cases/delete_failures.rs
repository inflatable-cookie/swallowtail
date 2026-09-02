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
    assert_eq!(block_on(fixture.close_session(handle)), CleanupOutcome::Clean);
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
        assert_eq!(block_on(fixture.close_session(handle)), CleanupOutcome::Clean);
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
    assert_eq!(block_on(fixture.close_session(handle)), CleanupOutcome::Clean);
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
