use super::*;

#[test]
fn control_reader_rejects_cumulative_output_overflow() {
    let mut reader = ControlReader::default();
    let line = format!("{{\"value\":\"{}\"}}\n", "x".repeat(1024));
    let mut accepted = 0;

    while reader.observed_bytes + line.len() <= MAXIMUM_OUTPUT_BYTES {
        reader
            .push(line.as_bytes())
            .expect("bounded control record");
        accepted += 1;
    }

    assert!(accepted > 1);
    let error = reader
        .push(line.as_bytes())
        .expect_err("cumulative output must be bounded");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.reasoning_control_invalid"
    );
}

#[test]
fn control_reader_rejects_too_many_records() {
    let mut reader = ControlReader::default();

    for _ in 0..MAXIMUM_CONTROL_RECORDS {
        reader.push(b"{}\n").expect("bounded control record");
    }

    let error = reader
        .push(b"{}\n")
        .expect_err("control record count must be bounded");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.reasoning_control_invalid"
    );
}

#[test]
fn control_reader_rejects_an_unexpected_response_id() {
    let mut reader = ControlReader::default();
    let response = serde_json::to_vec(&serde_json::json!({
        "type": "control_response",
        "response": {
            "request_id": "unexpected-request",
            "subtype": "success",
            "response": {"subtype": "initialize"}
        }
    }))
    .expect("control response serializes");
    let mut line = response;
    line.push(b'\n');
    reader.push(&line).expect("control response parses");

    let error = reader
        .take_response("expected-request", "initialize")
        .expect_err("unexpected response must fail closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.reasoning_control_unexpected_response"
    );
}
