fn capture_header(line: &[u8], headers: &mut BTreeMap<String, String>) {
    let Ok(line) = std::str::from_utf8(line) else {
        return;
    };
    let Some((name, value)) = line.split_once(':') else {
        return;
    };
    let name = name.trim().to_ascii_lowercase();
    if name == "x-request-id" || name.starts_with("x-ratelimit-") {
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
        "swallowtail.openai.endpoint_invalid",
        "Host-approved OpenAI endpoint was not an eligible HTTP endpoint",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.openai.response_limit",
        "OpenAI response exceeded the bounded input limit",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.openai.sse_backpressure",
        "OpenAI SSE delivery exceeded its bounded capacity",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.openai.request_cancelled",
        "OpenAI HTTP request was cancelled",
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.openai.transport_failed",
        "OpenAI HTTP transport failed",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
