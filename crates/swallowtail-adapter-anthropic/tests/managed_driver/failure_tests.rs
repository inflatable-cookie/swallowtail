#[test]
fn provider_failure_is_safe_and_still_deletes_owned_resources() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::ProviderFailure);
    let (run, _events, outcome) = complete(&fixture, fixture.request("managed-provider-failure"));

    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
    assert!(!format!("{:?}", outcome.status()).contains("fixture-secret-never-log"));
    assert!(fixture.server.state().session_deleted);
    assert!(fixture.server.state().environment_deleted);
    assert_eq!(fixture.credential_releases(), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn deadline_interrupts_remote_work_before_ordered_deletion() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::WaitForInterrupt);
    let request = fixture
        .request("managed-deadline")
        .with_deadline(fixture.deadline_after(100));
    let (run, _events, outcome) = complete(&fixture, request);

    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(fixture.server.state().interrupts, 1);
    assert!(fixture.server.state().session_deleted);
    assert!(fixture.server.state().environment_deleted);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn unconfirmed_session_deletion_degrades_cleanup_and_does_not_claim_environment_removal() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::SessionDeleteFailure);
    let (run, _events, outcome) = complete(&fixture, fixture.request("managed-delete-failure"));

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert!(matches!(outcome.cleanup(), CleanupOutcome::Degraded(_)));
    assert_eq!(
        outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Unconfirmed)
    );
    assert_eq!(
        outcome.remote_resource_deletion(OwnedRemoteResourceKind::Environment),
        Some(RemoteResourceDeletionOutcome::Unconfirmed)
    );
    assert_eq!(fixture.credential_releases(), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}
