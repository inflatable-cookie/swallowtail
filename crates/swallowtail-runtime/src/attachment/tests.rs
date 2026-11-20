use super::{AttachmentReadFailure, read_bounded_attachment};

#[cfg(unix)]
#[test]
fn a_read_of_an_unbounded_stream_stops_at_the_size_bound() {
    // `/dev/zero` never ends, so an unbounded read cannot return at all. A
    // bounded read must reject after the extra byte instead of draining an
    // arbitrary stream into memory.
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let outcome = read_bounded_attachment("/dev/zero", 8);
        let _ = sender.send(outcome);
    });
    let outcome = receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("a bounded attachment read returns without draining the stream");
    assert_eq!(outcome, Err(AttachmentReadFailure::TooLarge));
}

#[test]
fn a_missing_file_is_reported_as_unreadable() {
    assert_eq!(
        read_bounded_attachment("/nonexistent/swallowtail-attachment", 8),
        Err(AttachmentReadFailure::Unreadable)
    );
}

#[cfg(unix)]
#[test]
fn a_file_exactly_at_the_bound_is_accepted() {
    use std::io::Write;

    let fixture = std::env::temp_dir().join(format!(
        "swallowtail-attachment-read-{}-{}.bin",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock is after the epoch")
            .as_nanos()
    ));
    let mut file = std::fs::File::create(&fixture).expect("fixture file is created");
    file.write_all(b"12345678")
        .expect("fixture bytes are written");
    drop(file);

    let bytes = read_bounded_attachment(fixture.to_str().expect("fixture path is UTF-8"), 8)
        .expect("a file exactly at the bound is accepted");
    assert_eq!(bytes, b"12345678");
    std::fs::remove_file(&fixture).expect("the fixture removes only its own file");
}
