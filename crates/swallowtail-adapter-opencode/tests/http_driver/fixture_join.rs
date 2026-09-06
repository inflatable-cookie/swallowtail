#[test]
fn cancelled_client_disconnect_does_not_panic_fixture_drop() {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    let server = FixtureServer::start(StreamFixture::Success);
    let endpoint = server.endpoint().trim_start_matches("http://").to_owned();
    let mut client = TcpStream::connect(&endpoint).expect("fixture client connects");
    write!(
        client,
        "GET /event? HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
    )
    .expect("event request writes");
    let mut probe = [0_u8; 1];
    let _ = client.read(&mut probe);
    drop(client);
    thread::sleep(Duration::from_millis(20));
    drop(server);
}

fn panicking_fixture_server() -> FixtureServer {
    use std::io::Write;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    let server = FixtureServer::start(StreamFixture::PanicOnEvent);
    let endpoint = server.endpoint().trim_start_matches("http://").to_owned();
    let mut client = TcpStream::connect(&endpoint).expect("fixture client connects");
    write!(
        client,
        "GET /event? HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
    )
    .expect("event request writes");
    thread::sleep(Duration::from_millis(50));
    server
}

#[test]
fn unexpected_fixture_handler_panic_is_observable_on_shutdown() {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut server = panicking_fixture_server();
        server.shutdown();
    }));
    let payload = result.expect_err("unexpected fixture panic must surface on shutdown");
    let message = payload
        .downcast_ref::<&str>()
        .copied()
        .map(str::to_owned)
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    assert!(
        message.contains("unexpected fixture failure"),
        "observed panic payload: {message:?}"
    );
}

/// `Drop` records the fixture-thread panic and reports it, but never raises
/// one. It runs during test unwinding, where a panic is non-unwinding and
/// aborts the binary instead of failing the test.
#[test]
fn unexpected_fixture_handler_panic_never_panics_from_drop() {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    let result = catch_unwind(AssertUnwindSafe(|| {
        drop(panicking_fixture_server());
    }));
    assert!(result.is_ok(), "drop must not raise the recorded panic");
}

/// The abort shape from the cancellation-cleanup flake: a test panic unwinds
/// through `FixtureServer::drop` while the fixture thread has also panicked.
#[test]
fn fixture_drop_during_unwind_preserves_the_primary_panic() {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    let result = catch_unwind(AssertUnwindSafe(|| {
        let _server = panicking_fixture_server();
        panic!("primary test failure");
    }));
    let payload = result.expect_err("primary panic must remain observable");
    let message = payload
        .downcast_ref::<&str>()
        .copied()
        .map(str::to_owned)
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    assert!(
        message.contains("primary test failure"),
        "observed panic payload: {message:?}"
    );
}

#[test]
fn fixture_join_suppresses_secondary_panic_during_existing_unwind() {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::thread;

    let result = catch_unwind(AssertUnwindSafe(|| {
        struct JoinOnDrop(Option<std::thread::JoinHandle<()>>);
        impl Drop for JoinOnDrop {
            fn drop(&mut self) {
                if let Some(handle) = self.0.take() {
                    crate::http_support::join_fixture_thread_for_test(handle);
                }
            }
        }

        let secondary = thread::spawn(|| panic!("secondary fixture panic"));
        let _guard = JoinOnDrop(Some(secondary));
        panic!("primary test failure");
    }));
    assert!(result.is_err(), "primary panic must remain observable");
}
