#[test]
fn prepared_interrupt_deletes_owned_resources_before_credential_release() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::WaitForInterrupt);
    let prepared =
        prepare_anthropic_managed_agent(fixture.preparation_input(), &fixture.services())
            .expect("managed integration prepares");
    let run = prepared
        .prepare_managed_run(fixture.prepared_run_input("prepared-interrupt", []))
        .expect("managed run prepares");
    let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    for _ in 0..200 {
        if fixture.server.state().stream_attachments == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(handle.cancellation().request()).expect("cancellation is accepted");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(fixture.server.state().interrupts, 1);
    assert_delete_order(&fixture);
    assert_eq!(fixture.credential_releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}
