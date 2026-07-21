fn capture_header(line: &[u8], headers: &mut BTreeMap<String, String>) {
    let Ok(line) = std::str::from_utf8(line) else {
        return;
    };
    let Some((name, value)) = line.split_once(':') else {
        return;
    };
    let name = name.trim().to_ascii_lowercase();
    if name == "request-id" || name.starts_with("anthropic-ratelimit-") {
        headers.insert(name, value.trim().to_owned());
    }
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
        "swallowtail.anthropic.endpoint_invalid",
        "Host-approved Anthropic endpoint was not an eligible HTTP endpoint",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.response_limit",
        "Anthropic response exceeded the bounded input limit",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.sse_backpressure",
        "Anthropic SSE delivery exceeded its bounded capacity",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.request_cancelled",
        "Anthropic HTTP request was cancelled",
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.transport_failed",
        "Anthropic HTTP transport failed",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
