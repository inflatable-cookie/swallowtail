fn capture_header(line: &[u8], headers: &mut BTreeMap<String, String>) {
    let Ok(line) = std::str::from_utf8(line) else {
        return;
    };
    let Some((name, value)) = line.split_once(':') else {
        return;
    };
    let name = name.trim().to_ascii_lowercase();
    let _ = (name, value, headers);
}

fn parse_status(line: &[u8]) -> Option<u32> {
    std::str::from_utf8(line)
        .ok()?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.endpoint_invalid",
        "Host-approved Kimi Platform endpoint was not an eligible HTTP endpoint",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.response_limit",
        "Kimi Platform response exceeded the bounded input limit",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.sse_backpressure",
        "Kimi Platform SSE delivery exceeded its bounded capacity",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.request_cancelled",
        "Kimi Platform HTTP request was cancelled",
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.transport_failed",
        "Kimi Platform HTTP transport failed",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
