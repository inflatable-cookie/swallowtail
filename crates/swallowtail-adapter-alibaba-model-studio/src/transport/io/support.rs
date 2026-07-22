fn capture_header(line: &[u8], headers: &mut BTreeMap<String, String>) {
    let Ok(line) = std::str::from_utf8(line) else { return; };
    let Some((name, value)) = line.split_once(':') else { return; };
    headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
}

fn parse_status(line: &[u8]) -> Option<u32> {
    std::str::from_utf8(line).ok()?.split_whitespace().nth(1)?.parse().ok()
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.endpoint_invalid",
        "Alibaba Model Studio authorized endpoint was invalid",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.http_protocol_invalid",
        "Alibaba Model Studio HTTP response was invalid",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.response_limit_exceeded",
        "Alibaba Model Studio response exceeded its byte limit",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.stream_backpressure",
        "Alibaba Model Studio response exceeded its event buffer",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.connection_cancelled",
        "Alibaba Model Studio local connection was cancelled",
    )
}

fn curl_failure(_error: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.http_failed",
        "Alibaba Model Studio HTTP work failed",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) { self.0.fill(0); }
}
