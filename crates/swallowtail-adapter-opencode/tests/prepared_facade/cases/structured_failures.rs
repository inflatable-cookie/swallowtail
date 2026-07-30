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

