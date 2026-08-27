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

#[test]
fn unexpected_fixture_handler_panic_is_observable_on_drop() {
    use std::io::Write;
    use std::net::TcpStream;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::thread;
    use std::time::Duration;

    let result = catch_unwind(AssertUnwindSafe(|| {
        let server = FixtureServer::start(StreamFixture::PanicOnEvent);
        let endpoint = server.endpoint().trim_start_matches("http://").to_owned();
        let mut client = TcpStream::connect(&endpoint).expect("fixture client connects");
        write!(
            client,
            "GET /event? HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        )
        .expect("event request writes");
        thread::sleep(Duration::from_millis(50));
        drop(server);
    }));
    let payload = result.expect_err("unexpected fixture panic must surface on drop");
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
